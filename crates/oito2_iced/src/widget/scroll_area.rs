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

//! Scroll areas: a [`scrollable()`] with the OITO2 scrollbars on the axes it
//! scrolls in, and the undershoot, a soft shadow at the edges with more
//! content beyond them.
//!
//! [`scroll_area`] wraps any content; [`undershoot`] adds the edge shadows
//! to any scroll view, such as a [`scrollable()`] built by hand. The shadows
//! fade in and out as the content scrolls and show their final state on
//! the first frame.
//!
//! ```no_run
//! use oito2_iced::widget::scroll_area::{self, Direction};
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! scroll_area::scroll_area(iced::widget::text("A long text"))
//!     .direction(Direction::Vertical)
//!     .undershoot(true)
//!     .into()
//! # }
//! ```

use std::f32::consts::{FRAC_PI_2, PI};
use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::gradient::Linear;
use iced::time::Instant;
use iced::widget::scrollable::{self as native, Scrollbar, Viewport};
use iced::widget::{container, scrollable};
use iced::{
    Background, Color, Event, Gradient, Length, Padding, Rectangle, Size, Vector, mouse, window,
};

use crate::animation::Animation;
use crate::tokens::spacing;
use crate::{Element, Theme};

pub use crate::theme::scrollable::Class;

/// The time an undershoot shadow takes to fade in or out.
pub const UNDERSHOOT_FADE: Duration = Duration::from_millis(150);

/// The default depth of an undershoot shadow.
pub const UNDERSHOOT_EXTENT: f32 = spacing::MD;

/// The opacity of the shadow color at the edge of an undershoot.
const UNDERSHOOT_OPACITY: f32 = 0.12;

/// The width of a scrollbar, which takes the pointer.
pub const SCROLLBAR_WIDTH: f32 = 10.0;

/// The thickness of a scroller.
pub const SCROLLER_WIDTH: f32 = 6.0;

/// The distance one line of a mouse wheel scrolls.
const WHEEL_LINE: f32 = 60.0;

/// The directions a [`ScrollArea`] scrolls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    /// Up and down.
    #[default]
    Vertical,
    /// Left and right.
    Horizontal,
    /// Both ways, with a scrollbar on each axis.
    Both,
}

/// The axis an [`Undershoot`] follows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
    /// Shadows at the top and bottom edges.
    #[default]
    Vertical,
    /// Shadows at the left and right edges.
    Horizontal,
}

/// A scrollable box for content larger than it. See [`scroll_area`].
pub struct ScrollArea<'a, Message> {
    content: Element<'a, Message>,
    direction: Direction,
    undershoot: bool,
    padding: Padding,
    id: Option<Id>,
    width: Length,
    height: Length,
    on_scroll: Option<Box<dyn Fn(Viewport) -> Message + 'a>>,
    class: Class<'a>,
}

/// A scroll area around `content` that fills the available space and
/// scrolls vertically, with the OITO2 scrollbar.
///
/// The scrollbars show on the axes the area scrolls in while the content
/// overflows them. With [`ScrollArea::undershoot`], a soft shadow marks the
/// edges with more content beyond them; with [`Direction::Both`] the
/// shadows follow the vertical axis.
pub fn scroll_area<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> ScrollArea<'a, Message> {
    ScrollArea {
        content: content.into(),
        direction: Direction::Vertical,
        undershoot: false,
        padding: Padding::ZERO,
        id: None,
        width: Length::Fill,
        height: Length::Fill,
        on_scroll: None,
        class: Class::Default,
    }
}

impl<'a, Message> ScrollArea<'a, Message> {
    /// Sets the directions the content scrolls in.
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether a soft shadow marks the edges with more content beyond
    /// them.
    #[must_use]
    pub fn undershoot(mut self, undershoot: bool) -> Self {
        self.undershoot = undershoot;
        self
    }

    /// Sets the space around the content, inside the scrolled area.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the widget [`Id`] of the scrollable, used to scroll it with the
    /// operations of [`iced::widget::operation`].
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width (fills the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height (fills the available height by default).
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the message published with the new [`Viewport`] whenever the
    /// content scrolls.
    #[must_use]
    pub fn on_scroll(mut self, on_scroll: impl Fn(Viewport) -> Message + 'a) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    /// Sets the style class of the scrollbars.
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

/// The OITO2 scrollbar: a 10-pixel rail with a 6-pixel scroller.
pub fn scrollbar() -> Scrollbar {
    Scrollbar::new()
        .width(SCROLLBAR_WIDTH)
        .scroller_width(SCROLLER_WIDTH)
}

impl<'a, Message: 'a> From<ScrollArea<'a, Message>> for Element<'a, Message> {
    fn from(area: ScrollArea<'a, Message>) -> Self {
        let content = if area.padding == Padding::ZERO {
            area.content
        } else {
            container(area.content).padding(area.padding).into()
        };

        let direction = match area.direction {
            Direction::Vertical => native::Direction::Vertical(scrollbar()),
            Direction::Horizontal => native::Direction::Horizontal(scrollbar()),
            Direction::Both => native::Direction::Both {
                vertical: scrollbar(),
                horizontal: scrollbar(),
            },
        };

        let mut view = scrollable(content)
            .direction(direction)
            .width(area.width)
            .height(area.height)
            .class(area.class);

        if let Some(id) = area.id {
            view = view.id(id);
        }

        if let Some(on_scroll) = area.on_scroll {
            view = view.on_scroll(on_scroll);
        }

        if !area.undershoot {
            return view.into();
        }

        let axis = match area.direction {
            Direction::Horizontal => Axis::Horizontal,
            Direction::Vertical | Direction::Both => Axis::Vertical,
        };

        undershoot(view).axis(axis).into()
    }
}

/// Soft shadows over the start and end edges of a scroll view. See
/// [`undershoot`].
pub struct Undershoot<'a, Message> {
    content: Element<'a, Message>,
    axis: Axis,
    start: bool,
    end: bool,
    extent: f32,
    pointer_scrolling: bool,
    animated: bool,
}

/// Fades a soft shadow in at the start and end edges of the scroll view in
/// `content` while there is more content beyond them.
///
/// The shadows follow the first [`scrollable()`] found in `content`, not the
/// scroll views nested in it, along the vertical axis unless set with
/// [`Undershoot::axis`]. Along the horizontal axis, the vertical mouse
/// wheel scrolls the content sideways, unless turned off with
/// [`Undershoot::pointer_scrolling`].
pub fn undershoot<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> Undershoot<'a, Message> {
    Undershoot {
        content: content.into(),
        axis: Axis::Vertical,
        start: true,
        end: true,
        extent: UNDERSHOOT_EXTENT,
        pointer_scrolling: true,
        animated: true,
    }
}

impl<Message> Undershoot<'_, Message> {
    /// Sets the axis the shadows follow.
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Sets whether the shadow at the start edge (top or left) shows.
    #[must_use]
    pub fn start(mut self, start: bool) -> Self {
        self.start = start;
        self
    }

    /// Sets whether the shadow at the end edge (bottom or right) shows.
    #[must_use]
    pub fn end(mut self, end: bool) -> Self {
        self.end = end;
        self
    }

    /// Sets the depth of the shadows, in logical pixels.
    #[must_use]
    pub fn extent(mut self, extent: f32) -> Self {
        self.extent = extent.max(0.0);
        self
    }

    /// Sets whether the vertical mouse wheel scrolls a horizontal scroll
    /// view (on by default).
    #[must_use]
    pub fn pointer_scrolling(mut self, pointer_scrolling: bool) -> Self {
        self.pointer_scrolling = pointer_scrolling;
        self
    }

    /// Sets whether the shadows fade (on by default); turned off for
    /// reduced motion, they show and hide at once.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

/// Which edges of a scroll view have content beyond them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Edges {
    before: bool,
    after: bool,
}

/// The edges along `axis` of a scroll view of bounds `viewport` showing
/// `content` shifted by `translation` that have content beyond them.
fn edges(axis: Axis, viewport: Rectangle, content: Rectangle, translation: Vector) -> Edges {
    let (offset, visible, total) = match axis {
        Axis::Vertical => (translation.y, viewport.height, content.height),
        Axis::Horizontal => (translation.x, viewport.width, content.width),
    };

    Edges {
        before: offset > 0.5,
        after: total - visible - offset > 0.5,
    }
}

/// The tree state of an [`Undershoot`].
#[derive(Debug)]
struct State {
    edges: Option<Edges>,
    before: Animation<bool>,
    after: Animation<bool>,
    now: Option<Instant>,
}

impl State {
    /// Records `edges`: the first time without fading, afterwards fading
    /// from `at`. Returns whether they changed.
    fn record(&mut self, edges: Edges, at: Instant) -> bool {
        match self.edges {
            None => {
                self.before = fade(edges.before);
                self.after = fade(edges.after);
            }
            Some(current) if current == edges => return false,
            Some(_) => {
                self.before.go_mut(edges.before, at);
                self.after.go_mut(edges.after, at);
            }
        }

        self.edges = Some(edges);
        true
    }

    /// Whether a shadow is fading at `at`.
    fn is_animating(&self, at: Instant) -> bool {
        self.before.is_animating(at) || self.after.is_animating(at)
    }
}

/// An undershoot fade resting at `visible`.
fn fade(visible: bool) -> Animation<bool> {
    Animation::new(visible).duration(UNDERSHOOT_FADE)
}

/// An operation that finds the first scrollable, scrolls it by `delta`
/// when given and reports its bounds, content bounds and translation.
struct Probe {
    delta: Option<Vector>,
    found: Option<(Rectangle, Rectangle, Vector)>,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if self.found.is_none() {
            operate(self);
        }
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        if self.found.is_some() {
            return;
        }

        if let Some(delta) = self.delta {
            state.scroll_by(
                AbsoluteOffset {
                    x: delta.x,
                    y: delta.y,
                },
                bounds,
                content_bounds,
            );
        }

        self.found = Some((bounds, content_bounds, translation));
    }
}

impl<Message> Undershoot<'_, Message> {
    /// Scrolls the first scrollable of the content by `delta`, when given,
    /// and returns its translation before the scroll and its edges with
    /// content beyond them.
    fn probe(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        delta: Option<Vector>,
    ) -> Option<(Vector, Edges)> {
        let mut probe = Probe { delta, found: None };

        self.content
            .as_widget_mut()
            .operate(tree, layout, renderer, &mut probe);

        probe.found.map(|(bounds, content, translation)| {
            (translation, edges(self.axis, bounds, content, translation))
        })
    }

    /// The horizontal scroll a vertical mouse wheel `event` asks of a
    /// horizontal scroll view, when pointer scrolling applies to it.
    fn wheel_delta(&self, event: &Event, over: bool) -> Option<Vector> {
        let Event::Mouse(mouse::Event::WheelScrolled { delta }) = event else {
            return None;
        };

        if !self.pointer_scrolling || self.axis != Axis::Horizontal || !over {
            return None;
        }

        let (x, y) = match *delta {
            mouse::ScrollDelta::Lines { x, y } => (x * WHEEL_LINE, y * WHEEL_LINE),
            mouse::ScrollDelta::Pixels { x, y } => (x, y),
        };

        (y != 0.0 && x.abs() < y.abs()).then_some(Vector::new(-y, 0.0))
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Undershoot<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            edges: None,
            before: fade(false),
            after: fade(false),
            now: None,
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
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

        if let Some((_, edges)) =
            self.probe(&mut tree.children[0], Layout::new(&node), renderer, None)
        {
            let state = tree.state.downcast_mut::<State>();
            let _ = state.record(edges, Instant::now());
        }

        layout::Node::with_children(node.size(), vec![node])
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
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        let delta = self.wheel_delta(event, cursor.is_over(layout.bounds()));
        let before = delta
            .and_then(|_| self.probe(&mut tree.children[0], content_layout, renderer, None))
            .map(|(translation, _)| translation);

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

        let redraw_at = match event {
            Event::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };

        let Some((translation, mut edges)) =
            self.probe(&mut tree.children[0], content_layout, renderer, None)
        else {
            return;
        };

        // The scroll view leaves a vertical wheel it cannot use unscrolled;
        // the wheel then scrolls it sideways.
        if let Some(delta) = delta
            && before == Some(translation)
        {
            let _ = self.probe(&mut tree.children[0], content_layout, renderer, Some(delta));

            if let Some((_, after)) =
                self.probe(&mut tree.children[0], content_layout, renderer, None)
            {
                edges = after;
            }

            shell.capture_event();
            shell.request_redraw();
        }

        let state = tree.state.downcast_mut::<State>();

        if let Some(now) = redraw_at {
            state.now = Some(now);
        }

        let at = redraw_at.unwrap_or_else(Instant::now);

        if state.record(edges, at) {
            state.now = Some(at);
            shell.request_redraw();
        }

        if let Some(now) = redraw_at
            && state.is_animating(now)
        {
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

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );

        let state = tree.state.downcast_ref::<State>();
        let opacity = |animation: &Animation<bool>| match state.now {
            Some(now) if self.animated => animation.interpolate(0.0, 1.0, now),
            _ => f32::from(u8::from(animation.value())),
        };

        let bounds = layout.bounds();
        let shadow = theme.roles().shadow;

        for (shown, at_start, opacity) in [
            (self.start, true, opacity(&state.before)),
            (self.end, false, opacity(&state.after)),
        ] {
            if !shown || opacity <= 0.0 {
                continue;
            }

            let (quad, angle) = shadow_bounds(self.axis, bounds, self.extent, at_start);
            let edge = shadow.scale_alpha(UNDERSHOOT_OPACITY * opacity);
            let (first, last) = if at_start {
                (edge, Color::TRANSPARENT)
            } else {
                (Color::TRANSPARENT, edge)
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: quad,
                    ..renderer::Quad::default()
                },
                Background::Gradient(Gradient::Linear(
                    Linear::new(angle).add_stop(0.0, first).add_stop(1.0, last),
                )),
            );
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

/// The rectangle of the shadow at the start or end edge of `bounds` along
/// `axis`, `extent` deep, and the angle of its gradient, which runs from
/// the start edge to the end edge.
fn shadow_bounds(axis: Axis, bounds: Rectangle, extent: f32, at_start: bool) -> (Rectangle, f32) {
    match axis {
        Axis::Vertical => {
            let height = extent.min(bounds.height);
            let y = if at_start {
                bounds.y
            } else {
                bounds.y + bounds.height - height
            };

            (
                Rectangle {
                    y,
                    height,
                    ..bounds
                },
                PI,
            )
        }
        Axis::Horizontal => {
            let width = extent.min(bounds.width);
            let x = if at_start {
                bounds.x
            } else {
                bounds.x + bounds.width - width
            };

            (Rectangle { x, width, ..bounds }, FRAC_PI_2)
        }
    }
}

impl<'a, Message: 'a> From<Undershoot<'a, Message>> for Element<'a, Message> {
    fn from(undershoot: Undershoot<'a, Message>) -> Self {
        Element::new(undershoot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::Point;

    fn rectangle(width: f32, height: f32) -> Rectangle {
        Rectangle::new(Point::ORIGIN, Size::new(width, height))
    }

    #[test]
    fn edges_follow_the_scroll_offset() {
        let viewport = rectangle(100.0, 100.0);
        let content = rectangle(100.0, 300.0);
        let at = |y: f32| edges(Axis::Vertical, viewport, content, Vector::new(0.0, y));

        assert_eq!(
            at(0.0),
            Edges {
                before: false,
                after: true
            }
        );
        assert_eq!(
            at(100.0),
            Edges {
                before: true,
                after: true
            }
        );
        assert_eq!(
            at(200.0),
            Edges {
                before: true,
                after: false
            }
        );
    }

    #[test]
    fn content_that_fits_has_no_edges() {
        let edges = edges(
            Axis::Horizontal,
            rectangle(100.0, 100.0),
            rectangle(80.0, 300.0),
            Vector::ZERO,
        );

        assert_eq!(edges, Edges::default());
    }

    #[test]
    fn shadows_sit_at_the_edges_of_the_axis() {
        let bounds = rectangle(200.0, 100.0);

        let (top, _) = shadow_bounds(Axis::Vertical, bounds, 12.0, true);
        let (bottom, _) = shadow_bounds(Axis::Vertical, bounds, 12.0, false);
        let (right, _) = shadow_bounds(Axis::Horizontal, bounds, 12.0, false);

        assert_eq!(top, Rectangle::new(Point::ORIGIN, Size::new(200.0, 12.0)));
        assert_eq!(bottom.y, 88.0);
        assert_eq!(
            right,
            Rectangle::new(Point::new(188.0, 0.0), Size::new(12.0, 100.0))
        );
    }

    #[test]
    fn a_first_record_does_not_fade() {
        let mut state = State {
            edges: None,
            before: fade(false),
            after: fade(false),
            now: None,
        };
        let now = Instant::now();

        assert!(state.record(
            Edges {
                before: false,
                after: true
            },
            now
        ));
        assert!(!state.is_animating(now));
        assert!(state.after.value());

        assert!(state.record(
            Edges {
                before: true,
                after: true
            },
            now
        ));
        assert!(state.is_animating(now));
        assert!(!state.record(
            Edges {
                before: true,
                after: true
            },
            now
        ));
    }
}
