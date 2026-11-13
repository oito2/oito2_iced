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

//! Sheets: panels that slide in from an edge of the window.
//!
//! [`host`] wraps the application view and shows the [`Sheet`] it is given
//! against the reading-order start or end edge, or the bottom edge. The
//! application owns whether the sheet is open, rendering `None` to close it,
//! and, for a bottom sheet with [resting heights](Sheet::snap_points), the
//! height it rests at.
//!
//! A modal sheet dims the view below over a scrim and keeps the input: Tab
//! and Shift+Tab cycle the focus inside it and a press on the scrim
//! publishes the dismiss message when [dismissible](Sheet::dismissible). A
//! non-modal sheet leaves the view below usable beside it. Either publishes
//! the dismiss message on Escape (a non-modal one while the focus is inside
//! it) and, when dismissible, when dragged toward its edge.
//!
//! A side sheet is 360 pixels wide unless [sized](Sheet::size), and always
//! leaves 48 pixels of the window uncovered. A bottom sheet is as wide as
//! the window up to 640 pixels and as tall as its content, up to 90 % of the
//! window, unless resting heights are given: then its drag handle moves it
//! between them with a drag, the arrow keys, or a click, Enter or Space,
//! which cycle to the next height.
//!
//! A sheet that opens after the host was created slides in from its edge;
//! one shown from the start is drawn in place.
//!
//! [`content`] lays out the usual sheet content: a header with a title,
//! actions and a close button above the body.
//!
//! ```no_run
//! use oito2_iced::widget::sheet;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Close,
//! }
//!
//! # fn view<'a>(page: oito2_iced::Element<'a, Message>, open: bool) -> oito2_iced::Element<'a, Message> {
//! sheet::host(
//!     page,
//!     open.then(|| {
//!         sheet::sheet(
//!             sheet::content(iced::widget::text("Details"))
//!                 .title("Properties")
//!                 .on_close(Message::Close),
//!         )
//!         .side(sheet::Side::End)
//!         .on_dismiss(Message::Close)
//!     }),
//! )
//! # }
//! ```

use std::rc::Rc;
use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::widget::text::{Fragment, IntoFragment};
use iced::widget::{Row, Space, column, container};
use iced::{
    Alignment, Background, Border, Color, Event, Length, Padding, Point, Rectangle, Size, mouse,
    touch,
};

use super::dialog::layer::{self, DRAG_SLOP, Edge, Layer, Placement, Surface};
use super::styled_text;
use crate::animation::{duration, motion_duration};
use crate::l10n::Localizations;
use crate::theme::container::elevation;
use crate::tokens::{radius, sizes, spacing, typography};
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// The window edge a sheet slides in from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Side {
    /// The reading-order start edge.
    Start,
    /// The reading-order end edge.
    #[default]
    End,
    /// The bottom edge.
    Bottom,
}

/// Default width of a side sheet.
pub const SIDE_WIDTH: f32 = 360.0;

/// Width of the window a side sheet always leaves uncovered.
pub const SIDE_MARGIN: f32 = 48.0;

/// Largest width of a bottom sheet.
pub const BOTTOM_MAX_WIDTH: f32 = 640.0;

/// Largest height of a bottom sheet without resting heights, as a fraction
/// of the window height.
pub const BOTTOM_MAX_HEIGHT: f32 = 0.9;

/// Opacity of the scrim color behind a modal sheet.
pub const SCRIM_OPACITY: f32 = 0.32;

/// Duration of the slide in.
pub const ENTRANCE: Duration = Duration::from_millis(250);

/// Elevation of the sheet surface.
const ELEVATION: f32 = 8.0;

/// Size of the drag handle pill.
const PILL: Size = Size::new(36.0, 4.0);

/// Produces the message that moves a bottom sheet to another resting
/// height.
type OnSnap<'a, Message> = Rc<dyn Fn(usize) -> Message + 'a>;

/// A sheet shown by [`host`]. Built by [`sheet`].
pub struct Sheet<'a, Message> {
    content: Element<'a, Message>,
    side: Side,
    modal: bool,
    dismissible: bool,
    size: Option<f32>,
    snaps: Option<(Vec<f32>, usize, OnSnap<'a, Message>)>,
    on_dismiss: Option<Message>,
    direction: ReadingDirection,
    reduce_motion: bool,
}

/// A modal, dismissible sheet showing `content` at the end edge.
pub fn sheet<'a, Message>(content: impl Into<Element<'a, Message>>) -> Sheet<'a, Message> {
    Sheet {
        content: content.into(),
        side: Side::End,
        modal: true,
        dismissible: true,
        size: None,
        snaps: None,
        on_dismiss: None,
        direction: ReadingDirection::LeftToRight,
        reduce_motion: false,
    }
}

impl<'a, Message> Sheet<'a, Message> {
    /// Sets the edge the sheet slides in from.
    #[must_use]
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// Sets whether the sheet dims the view below and keeps the input
    /// (`true` by default).
    #[must_use]
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// Sets whether a press on the scrim and a drag toward the edge dismiss
    /// the sheet (`true` by default). Escape dismisses it either way.
    #[must_use]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets the width of a side sheet ([`SIDE_WIDTH`] by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Gives a bottom sheet the heights it rests at, as fractions of the
    /// window height in ascending order, rests it at the one at `current`,
    /// and publishes the message of `on_snap` with the index of the height a
    /// drag, a key or a click on the handle moves it to.
    ///
    /// Ignored for side sheets and when `points` is empty; `current` is
    /// clamped to the points.
    #[must_use]
    pub fn snap_points(
        mut self,
        points: impl Into<Vec<f32>>,
        current: usize,
        on_snap: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        let points = points.into();

        self.snaps = (!points.is_empty()).then(|| {
            let current = current.min(points.len() - 1);
            let on_snap: OnSnap<'a, Message> = Rc::new(on_snap);

            (points, current, on_snap)
        });
        self
    }

    /// Sets the message published on Escape, on a press on the scrim and on
    /// a drag that dismisses the sheet.
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }

    /// Sets the reading direction that maps the start and end sides to the
    /// left and right edges.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Skips the slide in and the settling animations.
    #[must_use]
    pub fn reduce_motion(mut self, reduce: bool) -> Self {
        self.reduce_motion = reduce;
        self
    }
}

/// The physical edge of `side` in `direction`.
fn edge(side: Side, direction: ReadingDirection) -> Edge {
    match (side, direction) {
        (Side::Bottom, _) => Edge::Bottom,
        (Side::Start, ReadingDirection::LeftToRight)
        | (Side::End, ReadingDirection::RightToLeft) => Edge::Left,
        (Side::End, ReadingDirection::LeftToRight)
        | (Side::Start, ReadingDirection::RightToLeft) => Edge::Right,
    }
}

/// The surface style of a sheet against `edge`: the dialog chrome tone with
/// the window radius on the corners away from the edge, a thin
/// `outline_variant` border and a shadow.
fn surface_style(theme: &Theme, edge: Edge) -> container::Style {
    let roles = theme.roles();
    let corners = match edge {
        Edge::Left => iced::border::right(radius::WINDOW),
        Edge::Right => iced::border::left(radius::WINDOW),
        Edge::Bottom => iced::border::top(radius::WINDOW),
    };

    container::Style {
        background: Some(Background::Color(roles.chrome.dialog)),
        text_color: Some(roles.on_surface),
        border: Border::default()
            .color(roles.outline_variant)
            .width(1.0)
            .rounded(corners),
        shadow: elevation(roles.shadow, ELEVATION),
        ..container::Style::default()
    }
}

/// `base` with `sheet`, when given, against its edge above it.
///
/// Wrap the whole application view, inside [`focus::navigation`] so Tab
/// also reaches a non-modal sheet: `focus::navigation(sheet::host(view,
/// sheet))`.
///
/// [`focus::navigation`]: crate::focus::navigation
pub fn host<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    sheet: Option<Sheet<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let surface = sheet.map(|sheet| {
        let edge = edge(sheet.side, sheet.direction);
        let snaps = sheet.snaps.filter(|_| edge == Edge::Bottom);

        let content: Element<'a, Message> = match (edge, &snaps) {
            (Edge::Bottom, snaps) => {
                let handle = Handle {
                    snaps: snaps
                        .as_ref()
                        .filter(|(points, _, _)| points.len() >= 2)
                        .map(|(points, current, on_snap)| {
                            (*current, points.len(), on_snap.clone())
                        }),
                };
                let fill = snaps.is_some();

                container(
                    column![handle, sheet.content]
                        .width(Length::Fill)
                        .height(if fill { Length::Fill } else { Length::Shrink }),
                )
                .width(Length::Fill)
                .height(if fill { Length::Fill } else { Length::Shrink })
                .style(move |theme: &Theme| surface_style(theme, edge))
                .clip(true)
                .into()
            }
            _ => container(sheet.content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |theme: &Theme| surface_style(theme, edge))
                .clip(true)
                .into(),
        };

        let placement = match edge {
            Edge::Bottom => Placement::Bottom {
                max_width: BOTTOM_MAX_WIDTH,
                max_height: BOTTOM_MAX_HEIGHT,
            },
            Edge::Left | Edge::Right => Placement::Side {
                edge,
                width: sheet.size.unwrap_or(SIDE_WIDTH),
                margin: SIDE_MARGIN,
            },
        };

        Surface {
            content,
            placement,
            modal: sheet.modal,
            scrim: SCRIM_OPACITY,
            on_dismiss: sheet.on_dismiss,
            dismissible: sheet.dismissible,
            draggable: true,
            snaps: snaps.map(|(points, current, on_snap)| layer::Snaps {
                points,
                current,
                on_snap: Box::new(move |index| on_snap(index)),
            }),
            entrance: motion_duration(ENTRANCE, sheet.reduce_motion),
            settle: motion_duration(duration::MEDIUM, sheet.reduce_motion),
        }
    });

    Layer::new(base, surface).into()
}

/// The content of a sheet: a header with a title, actions and a close
/// button above the body, with [`spacing::LG`] around them. Built by
/// [`content`].
pub struct Content<'a, Message> {
    body: Element<'a, Message>,
    title: Option<Fragment<'a>>,
    actions: Vec<Element<'a, Message>>,
    on_close: Option<Message>,
    close_label: &'static str,
    padding: Padding,
}

/// Sheet content showing `body` below a header.
pub fn content<'a, Message>(body: impl Into<Element<'a, Message>>) -> Content<'a, Message> {
    Content {
        body: body.into(),
        title: None,
        actions: Vec::new(),
        on_close: None,
        close_label: Localizations::en().close,
        padding: Padding::new(spacing::LG),
    }
}

impl<'a, Message> Content<'a, Message> {
    /// Shows `title` in the header, in the title large style.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Adds `action`, usually an icon button, at the end of the header.
    #[must_use]
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Shows a close button at the end of the header that publishes
    /// `message`.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Takes the label of the close button from `localizations`; English by
    /// default.
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.close_label = localizations.close;
        self
    }

    /// Sets the space around the header and the body.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<'a, Message> From<Content<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(content: Content<'a, Message>) -> Self {
        let Content {
            body,
            title,
            actions,
            on_close,
            close_label,
            padding,
        } = content;

        let has_header = title.is_some() || !actions.is_empty() || on_close.is_some();
        let mut children: Vec<Element<'a, Message>> = Vec::new();

        if has_header {
            let mut header: Vec<Element<'a, Message>> = vec![match title {
                Some(title) => container(styled_text(title, typography::TITLE_LARGE))
                    .width(Length::Fill)
                    .into(),
                None => Space::new().width(Length::Fill).into(),
            }];

            header.extend(actions);

            if let Some(message) = on_close {
                header.push(close(message, close_label));
            }

            children.push(
                Row::with_children(header)
                    .spacing(spacing::SM)
                    .align_y(Alignment::Center)
                    .into(),
            );
        }

        children.push(body);

        container(
            iced::widget::Column::with_children(children)
                .spacing(spacing::MD)
                .width(Length::Fill),
        )
        .padding(padding)
        .width(Length::Fill)
        .into()
    }
}

/// The close button of the header: an "x" icon button with the close
/// tooltip, or a text button without icons.
fn close<'a, Message: Clone + 'a>(message: Message, label: &'static str) -> Element<'a, Message> {
    #[cfg(feature = "svg")]
    {
        super::button::icon_button(crate::icon::lucide::X)
            .tooltip(label)
            .on_press(message)
            .into()
    }
    #[cfg(not(feature = "svg"))]
    {
        super::button::text(label).on_press(message).into()
    }
}

/// The drag handle of a bottom sheet: a pill, and with two or more resting
/// heights a focusable control that moves between them.
struct Handle<'a, Message> {
    /// The current height, the number of heights and the message of a move.
    snaps: Option<(usize, usize, OnSnap<'a, Message>)>,
}

/// The tree state of a [`Handle`].
#[derive(Debug, Default)]
struct HandleState {
    focused: bool,
    pressed: Option<Point>,
}

impl operation::Focusable for HandleState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<Message> Handle<'_, Message> {
    /// The height of the handle row.
    fn height(&self) -> f32 {
        if self.snaps.is_some() {
            spacing::XL
        } else {
            spacing::SM + PILL.height
        }
    }

    /// The bounds of the pill inside the handle `bounds`.
    fn pill(&self, bounds: Rectangle) -> Rectangle {
        let y = if self.snaps.is_some() {
            bounds.y + (bounds.height - PILL.height) / 2.0
        } else {
            bounds.y + spacing::SM
        };

        Rectangle::new(Point::new(bounds.center_x() - PILL.width / 2.0, y), PILL)
    }
}

/// The index a handle key or click moves to from `current` of `count`
/// heights: up and down by one, stopping at the ends, or the next one,
/// wrapping, on Enter, Space and clicks.
fn handle_target(current: usize, count: usize, key: Option<key::Named>) -> Option<usize> {
    match key {
        Some(key::Named::ArrowUp) => (current + 1 < count).then_some(current + 1),
        Some(key::Named::ArrowDown) => current.checked_sub(1),
        Some(key::Named::Enter | key::Named::Space) | None => {
            (count > 0).then(|| (current + 1) % count)
        }
        Some(_) => None,
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Handle<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<HandleState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(HandleState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height()))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, self.height())
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<HandleState>();

        if self.snaps.is_some() {
            operation.focusable(None, layout.bounds(), state);
        } else {
            state.focused = false;
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Some((current, count, on_snap)) = self.snaps.as_ref() else {
            return;
        };

        let state = tree.state.downcast_mut::<HandleState>();
        let bounds = layout.bounds();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.pressed = cursor.position_over(bounds);

                if state.pressed.is_none() && state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                state.pressed = bounds.contains(*position).then_some(*position);
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                let released = match event {
                    Event::Touch(touch::Event::FingerLifted { position, .. }) => Some(*position),
                    _ => cursor.position(),
                };

                if let (Some(origin), Some(released)) = (state.pressed.take(), released)
                    && bounds.contains(released)
                    && origin.distance(released) <= DRAG_SLOP
                    && let Some(target) = handle_target(*current, *count, None)
                {
                    shell.publish(on_snap(target));
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if state.focused && modifiers.is_empty() && !shell.is_event_captured() => {
                if matches!(
                    named,
                    key::Named::ArrowUp
                        | key::Named::ArrowDown
                        | key::Named::Enter
                        | key::Named::Space
                ) {
                    if let Some(target) = handle_target(*current, *count, Some(*named)) {
                        shell.publish(on_snap(target));
                    }
                    shell.capture_event();
                }
            }
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
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let roles = theme.roles();
        let pill = self.pill(layout.bounds());

        renderer.fill_quad(
            renderer::Quad {
                bounds: pill,
                border: Border::default().rounded(PILL.height / 2.0),
                ..renderer::Quad::default()
            },
            roles.on_surface_variant.scale_alpha(0.5),
        );

        if self.snaps.is_some() && tree.state.downcast_ref::<HandleState>().focused {
            let width = sizes::FOCUS_BORDER_WIDTH;

            renderer.fill_quad(
                renderer::Quad {
                    bounds: pill.expand(spacing::XXS + width),
                    border: Border::default()
                        .color(roles.primary)
                        .width(width)
                        .rounded(radius::SMALL),
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.snaps.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message> From<Handle<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(handle: Handle<'a, Message>) -> Self {
        Element::new(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_end_follow_the_reading_direction() {
        let ltr = ReadingDirection::LeftToRight;
        let rtl = ReadingDirection::RightToLeft;

        assert_eq!(edge(Side::Start, ltr), Edge::Left);
        assert_eq!(edge(Side::End, ltr), Edge::Right);
        assert_eq!(edge(Side::Start, rtl), Edge::Right);
        assert_eq!(edge(Side::End, rtl), Edge::Left);
        assert_eq!(edge(Side::Bottom, rtl), Edge::Bottom);
    }

    #[test]
    fn handle_keys_step_and_clicks_cycle() {
        assert_eq!(handle_target(0, 2, Some(key::Named::ArrowUp)), Some(1));
        assert_eq!(handle_target(1, 2, Some(key::Named::ArrowUp)), None);
        assert_eq!(handle_target(1, 2, Some(key::Named::ArrowDown)), Some(0));
        assert_eq!(handle_target(0, 2, Some(key::Named::ArrowDown)), None);
        assert_eq!(handle_target(1, 2, Some(key::Named::Enter)), Some(0));
        assert_eq!(handle_target(0, 3, None), Some(1));
        assert_eq!(handle_target(0, 2, Some(key::Named::Tab)), None);
    }
}
