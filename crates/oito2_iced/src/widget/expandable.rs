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

//! Expandable sections and expansion panels.
//!
//! An [`expandable`] is a header that shows or hides its content: the
//! whole header row is one focus stop, toggled by a click, Enter or Space,
//! with a chevron that turns as the content opens. The content grows and
//! shrinks in an animation; with [`Expandable::animated`] turned off, for
//! reduced motion, it shows and hides at once.
//!
//! An [`expansion_panel`] stacks sections in a bordered box, as an
//! accordion.
//!
//! The expanded state lives in the application: the widgets show the state
//! they are given and publish the requested one through their `on_toggle`
//! closure. [`set_expanded`] applies a request to the sections of a panel.
//!
//! ```no_run
//! use oito2_iced::widget::expandable;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Advanced(bool),
//! }
//!
//! # fn view<'a>(expanded: bool) -> oito2_iced::Element<'a, Message> {
//! expandable::expandable("Advanced", iced::widget::text("Details"), expanded)
//!     .on_toggle(Message::Advanced)
//!     .into()
//! # }
//! ```

use std::f32::consts::PI;
use std::rc::Rc;
use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::time::Instant;
use iced::widget::text::IntoFragment;
use iced::widget::{Column, Row, Space, button, container, rule};
use iced::{
    Alignment, Border, Element as IcedElement, Event, Length, Padding, Point, Rectangle, Size,
    Vector, mouse, window,
};

use super::selectable::{HOVER_OVERLAY_OPACITY, blend};
use super::{containers, focusable, styled_text};
use crate::animation::{Animation, ease_in_out};
use crate::theme::{DISABLED_CONTENT_OPACITY, TRANSPARENT};
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme};

/// The time the content takes to grow or shrink and the chevron to turn.
pub const DURATION: Duration = Duration::from_millis(200);

/// Where an [`Expandable`] places its chevron.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonPosition {
    /// Before the header.
    Start,
    /// After the header.
    #[default]
    End,
}

/// A header that shows or hides content. See [`expandable`].
pub struct Expandable<'a, Message> {
    header: Element<'a, Message>,
    content: Element<'a, Message>,
    collapsed: Option<Element<'a, Message>>,
    expanded: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    button_position: ButtonPosition,
    gap: f32,
    id: Option<Id>,
    animated: bool,
}

/// An expandable section with `header` in the title-small style over
/// `content`, which shows while `expanded`.
///
/// The section fills the available width. Without
/// [`on_toggle`](Expandable::on_toggle) its header is disabled.
pub fn expandable<'a, Message: 'a>(
    header: impl IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
    expanded: bool,
) -> Expandable<'a, Message> {
    expandable_with(
        styled_text(header, typography::TITLE_SMALL),
        content,
        expanded,
    )
}

/// An expandable section with any `header` element over `content`, which
/// shows while `expanded`. Text in the header takes the on-surface color.
pub fn expandable_with<'a, Message>(
    header: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    expanded: bool,
) -> Expandable<'a, Message> {
    Expandable {
        header: header.into(),
        content: content.into(),
        collapsed: None,
        expanded,
        on_toggle: None,
        button_position: ButtonPosition::End,
        gap: spacing::XS,
        id: None,
        animated: true,
    }
}

impl<'a, Message> Expandable<'a, Message> {
    /// Sets the closure that turns the requested state into a message,
    /// published when the header is activated. Without it the header is
    /// disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Sets the toggle closure, or disables the header with `None`.
    #[must_use]
    pub fn on_toggle_maybe(mut self, on_toggle: Option<impl Fn(bool) -> Message + 'a>) -> Self {
        self.on_toggle = on_toggle.map(|f| Box::new(f) as Box<dyn Fn(bool) -> Message + 'a>);
        self
    }

    /// Sets the content shown while collapsed, such as a summary.
    #[must_use]
    pub fn collapsed(mut self, collapsed: impl Into<Element<'a, Message>>) -> Self {
        self.collapsed = Some(collapsed.into());
        self
    }

    /// Sets where the chevron sits (after the header by default).
    #[must_use]
    pub fn button_position(mut self, position: ButtonPosition) -> Self {
        self.button_position = position;
        self
    }

    /// Sets the space between the header and the content.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Sets the widget [`Id`] of the header, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets whether the content and the chevron animate (on by default);
    /// turned off for reduced motion, they change at once.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message: Clone + 'a> From<Expandable<'a, Message>> for Element<'a, Message> {
    fn from(expandable: Expandable<'a, Message>) -> Self {
        let Expandable {
            header,
            content,
            collapsed,
            expanded,
            on_toggle,
            button_position,
            gap,
            id,
            animated,
        } = expandable;

        let message = on_toggle.map(|on_toggle| on_toggle(!expanded));
        let enabled = message.is_some();
        let chevron = || Chevron {
            expanded,
            enabled,
            animated,
        };

        let mut row = Row::new().spacing(spacing::SM).align_y(Alignment::Center);

        if button_position == ButtonPosition::Start {
            row = row.push(chevron());
        }

        row = row.push(container(header).width(Length::Fill));

        if button_position == ButtonPosition::End {
            row = row.push(chevron());
        }

        let native = button(row)
            .padding(Padding::from([spacing::XS, 0.0]))
            .width(Length::Fill)
            .on_press_maybe(message.clone())
            .style(|theme: &Theme, status| {
                let roles = theme.roles();
                let (background, text_color) = match status {
                    button::Status::Hovered | button::Status::Pressed => (
                        blend(TRANSPARENT, roles.on_surface, HOVER_OVERLAY_OPACITY),
                        roles.on_surface,
                    ),
                    button::Status::Active => (TRANSPARENT, roles.on_surface),
                    button::Status::Disabled => (
                        TRANSPARENT,
                        roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
                    ),
                };

                button::Style {
                    background: Some(background.into()),
                    text_color,
                    border: Border::default().rounded(radius::MEDIUM),
                    ..button::Style::default()
                }
            });

        let mut header = focusable(native).radius(radius::MEDIUM).enabled(enabled);

        if let Some(message) = message {
            header = header.on_activate(message);
        }

        if let Some(id) = id {
            header = header.id(id);
        }

        let body: Element<'a, Message> = match (expanded, collapsed) {
            (true, _) => container(content)
                .padding(Padding::ZERO.top(gap))
                .width(Length::Fill)
                .into(),
            (false, Some(collapsed)) => container(collapsed)
                .padding(Padding::ZERO.top(gap))
                .width(Length::Fill)
                .into(),
            (false, None) => Space::new().width(Length::Fill).into(),
        };

        Column::new()
            .width(Length::Fill)
            .push(header)
            .push(Reveal {
                content: body,
                expanded,
                animated,
            })
            .into()
    }
}

/// A chevron pointing down that turns to point up while expanded.
struct Chevron {
    expanded: bool,
    enabled: bool,
    animated: bool,
}

/// The tree state of a [`Chevron`].
#[derive(Debug)]
struct ChevronState {
    turn: Animation<bool>,
    now: Option<Instant>,
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chevron
where
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ChevronState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ChevronState {
            turn: Animation::new(self.expanded).duration(DURATION),
            now: None,
        })
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(sizes::ICON_SIZE),
            Length::Fixed(sizes::ICON_SIZE),
        )
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, sizes::ICON_SIZE, sizes::ICON_SIZE)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<ChevronState>();
        let redraw_at = match event {
            Event::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };

        if state.turn.value() != self.expanded {
            let at = redraw_at.unwrap_or_else(Instant::now);
            state.turn.go_mut(self.expanded, at);
            state.now = Some(at);
            shell.request_redraw();
        }

        if let Some(now) = redraw_at {
            state.now = Some(now);

            if state.turn.is_animating(now) {
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<ChevronState>();
        let turn = match state.now {
            Some(now) if self.animated && state.turn.value() == self.expanded => {
                state.turn.interpolate(0.0, 1.0, now)
            }
            _ => f32::from(u8::from(self.expanded)),
        };

        let roles = theme.roles();
        let color = if self.enabled {
            roles.on_surface_variant
        } else {
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
        };

        let bounds = layout.bounds();
        let size = bounds.width;
        let unit = size / 24.0;
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let path = Path::new(|builder| {
            builder.move_to(Point::new(6.0 * unit, 9.0 * unit));
            builder.line_to(Point::new(12.0 * unit, 15.0 * unit));
            builder.line_to(Point::new(18.0 * unit, 9.0 * unit));
        });

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.rotate(turn * PI);
            frame.translate(Vector::new(-center.x, -center.y));
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(color)
                    .with_width(2.0 * unit)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            );
        });

        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message, Renderer> From<Chevron> for IcedElement<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer + 'a,
{
    fn from(chevron: Chevron) -> Self {
        IcedElement::new(chevron)
    }
}

/// A box that animates its height to the height of its content when
/// `expanded` changes, clipping the content while it animates.
struct Reveal<'a, Message> {
    content: Element<'a, Message>,
    expanded: bool,
    animated: bool,
}

/// The tree state of a [`Reveal`].
#[derive(Debug, Clone, Copy)]
struct RevealState {
    expanded: bool,
    /// The height the animation starts from.
    from: f32,
    /// The height laid out last.
    height: f32,
    /// When the running animation started.
    started: Option<Instant>,
}

impl RevealState {
    /// Records the `expanded` state wanted and returns the height to lay
    /// out at `now` for content `natural` pixels tall.
    fn advance(&mut self, expanded: bool, animated: bool, natural: f32, now: Instant) -> f32 {
        if self.expanded != expanded {
            self.expanded = expanded;
            self.from = self.height;
            self.started = animated.then_some(now);
        }

        let height = match self.started {
            Some(start) => {
                let t = now.saturating_duration_since(start).as_secs_f32() / DURATION.as_secs_f32();

                if t >= 1.0 {
                    self.started = None;
                    natural
                } else {
                    self.from + (natural - self.from) * ease_in_out(t)
                }
            }
            None => natural,
        };

        self.height = height;
        height
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Reveal<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<RevealState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(RevealState {
            expanded: self.expanded,
            from: 0.0,
            height: 0.0,
            started: None,
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.content.as_widget().size().width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let node = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let natural = node.size().height;
        let height = tree.state.downcast_mut::<RevealState>().advance(
            self.expanded,
            self.animated,
            natural,
            Instant::now(),
        );

        layout::Node::with_children(Size::new(node.size().width, height), vec![node])
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let state = tree.state.downcast_ref::<RevealState>();

        if matches!(event, Event::Window(window::Event::RedrawRequested(_)))
            && (state.started.is_some() || state.expanded != self.expanded)
        {
            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let Some(content_layout) = layout.children().next() else {
            return;
        };

        let bounds = layout.bounds();
        let draw = |renderer: &mut iced::Renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        };

        if content_layout.bounds().height > bounds.height + 0.5 {
            if bounds.height > 0.0 {
                renderer.with_layer(bounds, draw);
            }
        } else {
            draw(renderer);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        layout
            .children()
            .next()
            .map(|content_layout| {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let content_layout = layout.children().next()?;

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Reveal<'a, Message>> for Element<'a, Message> {
    fn from(reveal: Reveal<'a, Message>) -> Self {
        Element::new(reveal)
    }
}

/// A bordered stack of expandable sections. See [`expansion_panel`].
pub struct ExpansionPanel<'a, Message> {
    sections: Vec<(Element<'a, Message>, Element<'a, Message>)>,
    expanded: Vec<bool>,
    on_toggle: Option<Box<dyn Fn(usize, bool) -> Message + 'a>>,
    dividers: bool,
    padding: Padding,
    animated: bool,
}

/// A bordered stack of sections, each with a header that expands its
/// content, as an accordion. Section `i` is open while `expanded[i]` is
/// true; sections past the end of `expanded` are closed.
///
/// Add the sections with [`ExpansionPanel::section`]. The panel fills the
/// available width.
pub fn expansion_panel<'a, Message>(expanded: &[bool]) -> ExpansionPanel<'a, Message> {
    ExpansionPanel {
        sections: Vec::new(),
        expanded: expanded.to_vec(),
        on_toggle: None,
        dividers: true,
        padding: Padding::from([spacing::SM, spacing::MD]),
        animated: true,
    }
}

impl<'a, Message> ExpansionPanel<'a, Message> {
    /// Adds a section with `header` in the title-small style over
    /// `content`.
    #[must_use]
    pub fn section(
        self,
        header: impl IntoFragment<'a>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self
    where
        Message: 'a,
    {
        self.section_with(styled_text(header, typography::TITLE_SMALL), content)
    }

    /// Adds a section with any `header` element over `content`.
    #[must_use]
    pub fn section_with(
        mut self,
        header: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
    ) -> Self {
        self.sections.push((header.into(), content.into()));
        self
    }

    /// Sets the closure that turns the index of an activated section and
    /// its requested state into a message. Without it the headers are
    /// disabled. [`set_expanded`] applies the request.
    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl Fn(usize, bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Sets whether 1-pixel dividers separate the sections (on by default).
    #[must_use]
    pub fn dividers(mut self, dividers: bool) -> Self {
        self.dividers = dividers;
        self
    }

    /// Sets the space around each section (8 pixels vertically and 12
    /// horizontally by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets whether the sections animate (on by default).
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<'a, Message: Clone + 'a> From<ExpansionPanel<'a, Message>> for Element<'a, Message> {
    fn from(panel: ExpansionPanel<'a, Message>) -> Self {
        let ExpansionPanel {
            sections,
            expanded,
            on_toggle,
            dividers,
            padding,
            animated,
        } = panel;
        let on_toggle: Option<Rc<dyn Fn(usize, bool) -> Message + 'a>> = on_toggle.map(Rc::from);

        let mut column = Column::new().width(Length::Fill);

        for (index, (header, content)) in sections.into_iter().enumerate() {
            if index > 0 && dividers {
                column = column.push(rule::horizontal(1));
            }

            let open = expanded.get(index).copied().unwrap_or(false);
            let section = expandable_with(header, content, open)
                .animated(animated)
                .on_toggle_maybe(
                    on_toggle
                        .clone()
                        .map(|on_toggle| move |expanded: bool| on_toggle(index, expanded)),
                );

            column = column.push(container(section).padding(padding));
        }

        containers::border_container(column)
            .width(Length::Fill)
            .into()
    }
}

/// Applies the request of the section at `index` of a panel to be
/// `expanded` to the open state of the sections in `open`. With
/// `collapse_on_expand`, opening a section closes the others, so at most
/// one is open.
///
/// Returns the sections that changed with their new state: the sections
/// closed by `collapse_on_expand` first, then the section at `index`. An
/// `index` past the end of `open` changes nothing.
pub fn set_expanded(
    open: &mut [bool],
    index: usize,
    expanded: bool,
    collapse_on_expand: bool,
) -> Vec<(usize, bool)> {
    if index >= open.len() {
        return Vec::new();
    }

    let mut changes = Vec::new();

    if expanded && collapse_on_expand {
        for (other, state) in open.iter_mut().enumerate() {
            if other != index && *state {
                *state = false;
                changes.push((other, false));
            }
        }
    }

    if open[index] != expanded {
        open[index] = expanded;
        changes.push((index, expanded));
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_a_section_closes_the_others_first() {
        let mut open = [true, false, true];

        assert_eq!(
            set_expanded(&mut open, 1, true, true),
            vec![(0, false), (2, false), (1, true)]
        );
        assert_eq!(open, [false, true, false]);
    }

    #[test]
    fn without_collapse_sections_open_independently() {
        let mut open = [true, false];

        assert_eq!(set_expanded(&mut open, 1, true, false), vec![(1, true)]);
        assert_eq!(set_expanded(&mut open, 0, false, false), vec![(0, false)]);
        assert_eq!(open, [false, true]);
        assert!(set_expanded(&mut open, 5, true, true).is_empty());
    }

    #[test]
    fn the_height_animates_only_after_a_change() {
        let start = Instant::now();
        let mut state = RevealState {
            expanded: false,
            from: 0.0,
            height: 0.0,
            started: None,
        };

        assert_eq!(state.advance(false, true, 0.0, start), 0.0);

        let first = state.advance(true, true, 100.0, start);
        assert_eq!(first, 0.0);

        let middle = state.advance(true, true, 100.0, start + DURATION / 2);
        assert!(middle > 0.0 && middle < 100.0, "{middle}");

        assert_eq!(state.advance(true, true, 100.0, start + DURATION), 100.0);
        assert!(state.started.is_none());
    }

    #[test]
    fn without_animation_the_height_changes_at_once() {
        let now = Instant::now();
        let mut state = RevealState {
            expanded: true,
            from: 0.0,
            height: 100.0,
            started: None,
        };

        assert_eq!(state.advance(false, false, 20.0, now), 20.0);
        assert!(state.started.is_none());
    }
}
