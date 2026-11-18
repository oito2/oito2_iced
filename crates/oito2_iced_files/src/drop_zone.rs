// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! A dashed box that asks for the file chooser when activated and receives
//! files dropped on the window.
//!
//! A [`drop_zone`] publishes its choose message on a click, Enter or Space;
//! the application opens the chooser with [`pick`](crate::pick). Files
//! dragged from the desktop arrive as window events: the platform reports
//! one event per file and no drop position, so the zone can't tell where
//! the files are over the window. Every zone with a drop message is a
//! target for the whole window: while files are dragged over the window it
//! draws its border and icon in the accent color on an accent tint, and the
//! files dropped anywhere on the window go to its drop message, together,
//! once the drop ends. Give a drop message to one zone per window.
//!
//! The dropped paths are read when the drop ends: items that no longer
//! exist are skipped and directories are marked as such. Drops arrive on
//! Windows, macOS and X11; the Wayland backend of the windowing library
//! reports none, so there [`can_drop_files`] is false and the zone invites
//! a click only.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_files::drop_zone::drop_zone;
//! use oito2_iced_files::file::File;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Choose,
//!     Files(Vec<File>),
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! drop_zone(&Localizations::en())
//!     .hint("PNG or JPEG, up to 5 MB")
//!     .on_choose(Message::Choose)
//!     .on_drop(Message::Files)
//!     .into()
//! # }
//! ```

use std::borrow::Cow;
use std::path::PathBuf;

use iced::advanced::graphics::geometry::{Frame, LineDash, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Id, Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::alignment::Horizontal;
use iced::widget::text::IntoFragment;
use iced::widget::{Column, container};
use iced::{
    Alignment, Background, Border, Color, Event, Length, Padding, Point, Rectangle, Size, mouse,
    touch, window,
};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::button::{Class, MIN_WIDTH};
use oito2_iced::widget::{Minimum, focusable, min_size, styled_text};
use oito2_iced::{Element, Theme, density};

use crate::file::File;

/// The default smallest height of a drop zone, in logical pixels.
pub const MIN_HEIGHT: f32 = 160.0;

/// The opacity of the accent tint behind a zone while files are dragged.
pub const DRAG_TINT_OPACITY: f32 = 0.08;

/// The opacity of a disabled zone.
pub const DISABLED_OPACITY: f32 = 0.5;

/// The width of the dashed border.
const BORDER_WIDTH: f32 = 1.5;

/// The lengths of a dash and of the gap after it.
const DASHES: [f32; 2] = [6.0, 4.0];

/// Whether files dragged from the desktop reach the window on this
/// platform: true on Windows, macOS and the free Unix systems running the
/// X11 backend, false under Wayland (when `WAYLAND_DISPLAY` or
/// `WAYLAND_SOCKET` is set, the windowing library picks Wayland) and on
/// other platforms.
#[must_use]
pub fn can_drop_files() -> bool {
    if cfg!(any(target_os = "windows", target_os = "macos")) {
        return true;
    }

    if cfg!(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd"
    )) {
        let set = |name: &str| std::env::var_os(name).is_some_and(|value| !value.is_empty());

        return !(set("WAYLAND_DISPLAY") || set("WAYLAND_SOCKET"));
    }

    false
}

/// A drop zone. See [`drop_zone`].
pub struct DropZone<'a, Message> {
    choose_files: Cow<'a, str>,
    choose_file: Cow<'a, str>,
    drop_files_here: Cow<'a, str>,
    on_choose: Option<Message>,
    on_drop: Option<Box<dyn Fn(Vec<File>) -> Message + 'a>>,
    multiple: bool,
    enabled: bool,
    hint: Option<Cow<'a, str>>,
    min_height: f32,
    accepts_drops: bool,
    id: Option<Id>,
}

/// A dashed box, as wide as its parent and at least [`MIN_HEIGHT`] tall,
/// with an upload icon in a circle, a title and an optional hint centered
/// in it, on the low surface container with the large radius.
///
/// The title reads "Drop files here" with an outlined "Choose files" button
/// below the hint when the zone takes drops, or "Choose files" ("Choose a
/// file" when not [`multiple`](DropZone::multiple)) alone otherwise. The
/// button only shows where to click: the whole zone is the button.
pub fn drop_zone<'a, Message>(localizations: &Localizations) -> DropZone<'a, Message> {
    DropZone {
        choose_files: Cow::Borrowed(localizations.choose_files),
        choose_file: Cow::Borrowed(localizations.choose_file),
        drop_files_here: Cow::Borrowed(localizations.drop_files_here),
        on_choose: None,
        on_drop: None,
        multiple: true,
        enabled: true,
        hint: None,
        min_height: MIN_HEIGHT,
        accepts_drops: can_drop_files(),
        id: None,
    }
}

impl<'a, Message> DropZone<'a, Message> {
    /// Sets the message published on a click, Enter or Space, which should
    /// open the file chooser. Without it the zone ignores clicks and is
    /// skipped by focus traversal.
    #[must_use]
    pub fn on_choose(mut self, message: Message) -> Self {
        self.on_choose = Some(message);
        self
    }

    /// Sets the message that receives the files dropped on the window,
    /// unchecked. Without it the zone ignores drops.
    #[must_use]
    pub fn on_drop(mut self, on_drop: impl Fn(Vec<File>) -> Message + 'a) -> Self {
        self.on_drop = Some(Box::new(on_drop));
        self
    }

    /// Sets whether the chooser takes several files (true by default),
    /// which picks the "Choose files" or "Choose a file" label. Drops pass
    /// every dropped file either way.
    #[must_use]
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Sets whether the zone takes clicks and drops (true by default). A
    /// disabled zone is drawn at half opacity.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets a line under the title, such as the accepted types and sizes.
    #[must_use]
    pub fn hint(mut self, hint: impl IntoFragment<'a>) -> Self {
        self.hint = Some(hint.into_fragment());
        self
    }

    /// Sets the smallest height, in logical pixels ([`MIN_HEIGHT`] by
    /// default).
    #[must_use]
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Sets whether the zone takes the file events of the window and
    /// invites drops; [`can_drop_files`] by default.
    #[must_use]
    pub fn accepts_drops(mut self, accepts_drops: bool) -> Self {
        self.accepts_drops = accepts_drops;
        self
    }

    /// Sets the widget [`Id`], used to focus the zone programmatically and
    /// to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// Scales the alpha of `color` to the opacity of an enabled or disabled
/// zone.
fn faded(color: Color, enabled: bool) -> Color {
    if enabled {
        color
    } else {
        color.scale_alpha(DISABLED_OPACITY)
    }
}

impl<'a, Message: Clone + 'a> From<DropZone<'a, Message>> for Element<'a, Message> {
    fn from(zone: DropZone<'a, Message>) -> Self {
        let enabled = zone.enabled;
        let on_choose = zone.on_choose.filter(|_| enabled);
        let can_drop = zone.accepts_drops && zone.on_drop.is_some();
        let on_drop = zone.on_drop.filter(|_| enabled && zone.accepts_drops);
        let choose = if zone.multiple {
            zone.choose_files
        } else {
            zone.choose_file
        };
        let title = if can_drop {
            zone.drop_files_here.clone()
        } else {
            choose.clone()
        };

        let icon_size = density::icon_size();
        let circle =
            container(icon(lucide::UPLOAD))
                .padding(spacing::SM)
                .style(move |theme: &Theme| container::Style {
                    background: Some(Background::Color(faded(
                        theme.roles().surface_container_highest,
                        enabled,
                    ))),
                    border: Border::default().rounded(icon_size / 2.0 + spacing::SM),
                    ..container::Style::default()
                });

        let mut content = Column::new()
            .push(circle)
            .push(
                styled_text(title, typography::TITLE_SMALL)
                    .align_x(Horizontal::Center)
                    .style(move |theme: &Theme| iced::widget::text::Style {
                        color: Some(faded(theme.roles().on_surface, enabled)),
                    }),
            )
            .spacing(spacing::SM)
            .padding(spacing::LG)
            .align_x(Alignment::Center);

        if let Some(hint) = zone.hint {
            content = content.push(
                styled_text(hint, typography::BODY_SMALL)
                    .align_x(Horizontal::Center)
                    .style(move |theme: &Theme| iced::widget::text::Style {
                        color: Some(faded(theme.roles().on_surface_variant, enabled)),
                    }),
            );
        }

        if can_drop {
            let label = min_size(
                styled_text(choose, typography::LABEL_LARGE),
                Minimum::Fixed(MIN_WIDTH - 2.0 * spacing::LG),
                Minimum::Control(0.0),
            );

            content = content.push(
                iced::widget::Button::new(label)
                    .class(Class::Outlined)
                    .padding(Padding::from([0.0, spacing::LG]))
                    .on_press_maybe(on_choose.clone()),
            );
        }

        let clickable = on_choose.is_some();
        let body = Zone {
            content: content.into(),
            on_choose: on_choose.clone(),
            on_drop,
            enabled,
            min_height: zone.min_height,
        };

        let mut wrapper = focusable(Element::new(body))
            .radius(radius::LARGE)
            .enabled(clickable);

        if let Some(message) = on_choose {
            wrapper = wrapper.on_activate(message);
        }

        if let Some(id) = zone.id {
            wrapper = wrapper.id(id);
        }

        wrapper.into()
    }
}

/// The box of a drop zone: its background, dashed border, clicks and
/// window file events around the decorative `content`.
struct Zone<'a, Message> {
    content: Element<'a, Message>,
    on_choose: Option<Message>,
    on_drop: Option<Box<dyn Fn(Vec<File>) -> Message + 'a>>,
    enabled: bool,
    min_height: f32,
}

/// The state of a zone in the widget tree.
#[derive(Debug, Default)]
struct State {
    /// Whether files are dragged over the window.
    dragging: bool,
    /// Whether the left button was pressed on the zone.
    pressed: bool,
    /// The paths dropped since the last redraw.
    pending: Vec<PathBuf>,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Zone<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &limits.loose());
        let inner = content.size();
        let size = limits.resolve(
            Length::Fill,
            Length::Shrink,
            Size::new(inner.width, inner.height.max(self.min_height)),
        );
        let content = content.move_to(Point::new(
            ((size.width - inner.width) / 2.0).max(0.0),
            ((size.height - inner.height) / 2.0).max(0.0),
        ));

        layout::Node::with_children(size, vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        // The content only gets the redraws, without the cursor, so its
        // button keeps the resting look of its enabled or disabled state.
        if let Event::Window(window::Event::RedrawRequested(_)) = event
            && let Some(content_layout) = layout.children().next()
        {
            self.content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                content_layout,
                mouse::Cursor::Unavailable,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let state = tree.state.downcast_mut::<State>();
        let over = cursor.is_over(layout.bounds());

        match event {
            Event::Window(window::Event::FileHovered(_)) if self.on_drop.is_some() => {
                if !state.dragging {
                    state.dragging = true;
                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::FilesHoveredLeft) => {
                if state.dragging {
                    state.dragging = false;
                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::FileDropped(path)) if self.on_drop.is_some() => {
                state.dragging = false;
                state.pending.push(path.clone());
                shell.request_redraw();
            }
            Event::Window(window::Event::RedrawRequested(_)) if !state.pending.is_empty() => {
                let paths = std::mem::take(&mut state.pending);

                if let Some(on_drop) = &self.on_drop {
                    let files: Vec<File> = paths
                        .into_iter()
                        .filter_map(|path| File::from_path(path).ok())
                        .collect();

                    if !files.is_empty() {
                        shell.publish(on_drop(files));
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if self.on_choose.is_some() && over =>
            {
                state.pressed = true;
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
                if state.pressed =>
            {
                state.pressed = false;

                if over && let Some(message) = &self.on_choose {
                    shell.publish(message.clone());
                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => state.pressed = false,
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        use iced::advanced::graphics::geometry::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let enabled = self.enabled;
        let active = state.dragging && enabled && self.on_drop.is_some();
        let accent = roles.semantic.primary_text;
        let bounds = layout.bounds();

        let background = if active {
            accent.scale_alpha(DRAG_TINT_OPACITY)
        } else {
            roles.surface_container_low
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border::default().rounded(radius::LARGE),
                ..renderer::Quad::default()
            },
            faded(background, enabled),
        );

        if let Some(content_layout) = layout.children().next() {
            let foreground = if active {
                accent
            } else {
                roles.on_surface_variant
            };
            let style = renderer::Style {
                text_color: faded(foreground, enabled),
            };

            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                &style,
                content_layout,
                cursor,
                viewport,
            );
        }

        let border = if active { accent } else { roles.outline };
        let inset = BORDER_WIDTH / 2.0;
        let mut frame = Frame::new(renderer, bounds.size());
        let outline = Path::rounded_rectangle(
            Point::new(inset, inset),
            Size::new(
                (bounds.width - BORDER_WIDTH).max(0.0),
                (bounds.height - BORDER_WIDTH).max(0.0),
            ),
            radius::LARGE.into(),
        );

        frame.stroke(
            &outline,
            Stroke {
                line_dash: LineDash {
                    segments: &DASHES,
                    offset: 0,
                },
                ..Stroke::default()
                    .with_color(faded(border, enabled))
                    .with_width(BORDER_WIDTH)
            },
        );

        let geometry = frame.into_geometry();

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.on_choose.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_colors_are_drawn_at_half_opacity() {
        assert_eq!(faded(Color::BLACK, true), Color::BLACK);
        assert!((faded(Color::BLACK, false).a - DISABLED_OPACITY).abs() < f32::EPSILON);
    }
}
