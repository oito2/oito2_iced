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

//! Content that scrolls continuously when it is longer than its space.
//!
//! [`marquee`] lays its content out without a limit along its [`Axis`].
//! When the content is longer than the marquee, it moves toward the start
//! of the reading direction (up when vertical), or the other way with
//! [`Marquee::reverse`], at [`Marquee::speed`] pixels per second, followed
//! by a copy after [`Marquee::gap`]. Content that fits stays still, at the
//! start (the right in [right-to-left](Marquee::direction) text when
//! horizontal). The marquee stops while the pointer is over it or the
//! focus is inside it ([`Marquee::pause_on_hover`]), and stays at its start
//! when [`Marquee::animated`] is false, for reduced motion.
//!
//! The copy is the same content drawn a second time further along the
//! axis, so both always look the same. Only the copy that sits at or
//! before the start edge receives the pointer; plain content, such as text
//! or icons, fits best.
//!
//! [`Marquee::fade_edges`] fades both ends over [`Marquee::fade_extent`]
//! pixels by drawing gradients from [`Marquee::fade_color`] (the surface
//! role by default) to transparent over the content; give it the color of
//! the background behind the marquee.
//!
//! ```no_run
//! use oito2_iced::widget::marquee;
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! marquee::marquee(iced::widget::text(
//!     "oito2_ui 0.1.0 · Linux, Windows, macOS, Android, iOS and web",
//! ))
//! .width(360)
//! .fade_color(|theme| theme.roles().surface_container)
//! .into()
//! # }
//! ```

use std::f32::consts::{FRAC_PI_2, PI};

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::gradient::Linear;
use iced::time::Instant;
use iced::{
    Background, Color, Element as IcedElement, Event, Gradient, Length, Point, Rectangle, Size,
    Vector, mouse, window,
};

use super::helpers::ReadingDirection;
use crate::{Element, Theme};

/// The default speed, in pixels per second.
pub const SPEED: f32 = 40.0;

/// The default space between the content and its copy.
pub const GAP: f32 = 32.0;

/// The default length of the faded ends.
pub const FADE_EXTENT: f32 = 24.0;

/// A function that picks a color from the theme.
type ColorFn<'a> = Box<dyn Fn(&Theme) -> Color + 'a>;

/// The direction a [`Marquee`] scrolls along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Axis {
    /// Side to side.
    #[default]
    Horizontal,
    /// Up and down.
    Vertical,
}

/// Content that scrolls continuously when it is longer than its space.
/// See [`marquee`].
pub struct Marquee<'a, Message> {
    content: Element<'a, Message>,
    axis: Axis,
    reverse: bool,
    speed: f32,
    gap: f32,
    pause_on_hover: bool,
    fade_edges: bool,
    fade_extent: f32,
    fade_color: Option<ColorFn<'a>>,
    animated: bool,
    direction: ReadingDirection,
    width: Length,
    height: Option<Length>,
}

/// A horizontal marquee of `content`, as wide as its parent, that scrolls
/// at [`SPEED`] with a [`GAP`] before the copy and faded ends.
pub fn marquee<'a, Message>(content: impl Into<Element<'a, Message>>) -> Marquee<'a, Message> {
    Marquee {
        content: content.into(),
        axis: Axis::Horizontal,
        reverse: false,
        speed: SPEED,
        gap: GAP,
        pause_on_hover: true,
        fade_edges: true,
        fade_extent: FADE_EXTENT,
        fade_color: None,
        animated: true,
        direction: ReadingDirection::default(),
        width: Length::Fill,
        height: None,
    }
}

impl<'a, Message> Marquee<'a, Message> {
    /// Sets the direction of the scroll ([`Axis::Horizontal`] by default).
    /// A vertical marquee fills the height of its parent unless
    /// [`Marquee::height`] says otherwise.
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Sets whether the content moves toward the end of the reading
    /// direction (down when vertical) instead of the start (false by
    /// default).
    #[must_use]
    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    /// Sets the speed in pixels per second ([`SPEED`] by default); a speed
    /// that is not greater than zero keeps the content still.
    #[must_use]
    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the space between the content and its copy ([`GAP`] by
    /// default).
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Sets whether the pointer over the marquee or the focus inside it
    /// stops the scroll (true by default).
    #[must_use]
    pub fn pause_on_hover(mut self, pause_on_hover: bool) -> Self {
        self.pause_on_hover = pause_on_hover;
        self
    }

    /// Sets whether the content fades at both ends while it scrolls (true
    /// by default).
    #[must_use]
    pub fn fade_edges(mut self, fade_edges: bool) -> Self {
        self.fade_edges = fade_edges;
        self
    }

    /// Sets the length of the faded ends ([`FADE_EXTENT`] by default); each
    /// end takes at most half the marquee.
    #[must_use]
    pub fn fade_extent(mut self, fade_extent: f32) -> Self {
        self.fade_extent = fade_extent.max(0.0);
        self
    }

    /// Sets the color the ends fade into, which should be the background
    /// behind the marquee (the surface role by default).
    #[must_use]
    pub fn fade_color(mut self, color: impl Fn(&Theme) -> Color + 'a) -> Self {
        self.fade_color = Some(Box::new(color));
        self
    }

    /// Sets whether the content scrolls (true by default); false keeps it
    /// at its start, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the reading direction: right to left mirrors the horizontal
    /// scroll and puts content that fits at the right.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the width ([`Length::Fill`] by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height ([`Length::Shrink`] by default when horizontal,
    /// [`Length::Fill`] when vertical).
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }
}

impl<'a, Message: 'a> From<Marquee<'a, Message>> for Element<'a, Message> {
    fn from(marquee: Marquee<'a, Message>) -> Self {
        let height = marquee.height.unwrap_or(match marquee.axis {
            Axis::Horizontal => Length::Shrink,
            Axis::Vertical => Length::Fill,
        });

        IcedElement::new(Inner {
            content: marquee.content,
            axis: marquee.axis,
            reverse: marquee.reverse,
            speed: marquee.speed,
            gap: marquee.gap,
            pause_on_hover: marquee.pause_on_hover,
            fade_edges: marquee.fade_edges,
            fade_extent: marquee.fade_extent,
            fade_color: marquee.fade_color,
            animated: marquee.animated,
            direction: marquee.direction,
            width: marquee.width,
            height,
        })
    }
}

/// The marquee widget, with the content as its only child.
struct Inner<'a, Message> {
    content: Element<'a, Message>,
    axis: Axis,
    reverse: bool,
    speed: f32,
    gap: f32,
    pause_on_hover: bool,
    fade_edges: bool,
    fade_extent: f32,
    fade_color: Option<ColorFn<'a>>,
    animated: bool,
    direction: ReadingDirection,
    width: Length,
    height: Length,
}

/// The tree state of a marquee.
#[derive(Debug, Clone, Copy, Default)]
struct State {
    /// How far the content has travelled, in pixels.
    offset: f32,
    /// The previous frame while scrolling; `None` while still.
    last: Option<Instant>,
    hovered: bool,
    /// Whether a widget inside the marquee has the focus.
    focus_inside: bool,
}

/// The length of the content and of the marquee along the axis.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Extents {
    content: f32,
    viewport: f32,
}

impl Extents {
    /// Whether the content is longer than the marquee.
    fn overflows(self) -> bool {
        overflows(self.content, self.viewport)
    }
}

/// Whether content of length `content` is longer than a non-empty
/// viewport of length `viewport`.
fn overflows(content: f32, viewport: f32) -> bool {
    content > viewport && viewport > 0.0
}

/// Advances `offset` by `speed` pixels per second over `seconds`, wrapping
/// it at `period`.
fn advance(offset: f32, speed: f32, seconds: f32, period: f32) -> f32 {
    if period <= 0.0 {
        return 0.0;
    }

    (offset + speed * seconds).rem_euclid(period)
}

/// The position of the content along the axis, in `(-period, 0]`, for a
/// travelled `offset`: the content moves toward negative positions when
/// `toward_negative` and toward positive ones otherwise. The copy sits
/// one `period` further.
fn shift(offset: f32, period: f32, toward_negative: bool) -> f32 {
    if period <= 0.0 {
        return 0.0;
    }

    let offset = offset.rem_euclid(period);
    let shift = if toward_negative {
        -offset
    } else {
        offset - period
    };

    if shift <= -period { 0.0 } else { shift }
}

impl<Message> Inner<'_, Message> {
    /// The extents of the content laid out in `child` inside `bounds`.
    fn extents(&self, bounds: Rectangle, child: Rectangle) -> Extents {
        match self.axis {
            Axis::Horizontal => Extents {
                content: child.width,
                viewport: bounds.width,
            },
            Axis::Vertical => Extents {
                content: child.height,
                viewport: bounds.height,
            },
        }
    }

    /// One content length plus the gap.
    fn period(&self, extents: Extents) -> f32 {
        extents.content + self.gap
    }

    /// Whether the content moves toward negative coordinates: toward the
    /// reading start, or up when vertical, unless reversed.
    fn toward_negative(&self) -> bool {
        match self.axis {
            Axis::Horizontal => (self.direction == ReadingDirection::RightToLeft) == self.reverse,
            Axis::Vertical => !self.reverse,
        }
    }

    /// The translation of the content along the axis.
    fn vector(&self, along: f32) -> Vector {
        match self.axis {
            Axis::Horizontal => Vector::new(along, 0.0),
            Axis::Vertical => Vector::new(0.0, along),
        }
    }

    /// The translation of the content that receives input, for the
    /// travelled `offset`.
    fn translation(&self, offset: f32, extents: Extents) -> Vector {
        if !extents.overflows() || !self.animated {
            return Vector::ZERO;
        }

        self.vector(shift(offset, self.period(extents), self.toward_negative()))
    }

    /// Whether the content scrolls now.
    fn running(&self, state: &State, extents: Extents) -> bool {
        let paused = self.pause_on_hover && (state.hovered || state.focus_inside);

        self.animated && self.speed > 0.0 && extents.overflows() && !paused
    }

    /// The cursor of the content translated by `translation`.
    fn content_cursor(
        cursor: mouse::Cursor,
        bounds: Rectangle,
        translation: Vector,
    ) -> mouse::Cursor {
        if cursor.is_over(bounds) {
            cursor - translation
        } else {
            mouse::Cursor::Unavailable
        }
    }

    /// The faded end at the start (or the end) of `bounds`, with the angle
    /// of its gradient.
    fn fade_bounds(&self, bounds: Rectangle, at_start: bool) -> (Rectangle, f32) {
        match self.axis {
            Axis::Horizontal => {
                let width = self.fade_extent.min(bounds.width / 2.0);
                let x = if at_start {
                    bounds.x
                } else {
                    bounds.x + bounds.width - width
                };

                (Rectangle { x, width, ..bounds }, FRAC_PI_2)
            }
            Axis::Vertical => {
                let height = self.fade_extent.min(bounds.height / 2.0);
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
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Inner<'_, Message> {
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
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let max = limits.max();
        let content_limits = match self.axis {
            Axis::Horizontal => {
                layout::Limits::new(Size::ZERO, Size::new(f32::INFINITY, max.height))
            }
            Axis::Vertical => layout::Limits::new(Size::ZERO, Size::new(max.width, f32::INFINITY)),
        };
        let node =
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &content_limits);
        let content = node.size();
        let size = limits.resolve(self.width, self.height, content);

        let position = match self.axis {
            Axis::Horizontal => {
                let rtl = self.direction == ReadingDirection::RightToLeft;
                let x = if rtl && !overflows(content.width, size.width) {
                    size.width - content.width
                } else {
                    0.0
                };

                Point::new(x, (size.height - content.height) / 2.0)
            }
            Axis::Vertical => Point::new((size.width - content.width) / 2.0, 0.0),
        };

        layout::Node::with_children(size, vec![node.move_to(position)])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            if let Some(child) = layout.children().next() {
                self.content.as_widget_mut().operate(
                    &mut tree.children[0],
                    child,
                    renderer,
                    operation,
                );
            }
        });
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
        let Some(child) = layout.children().next() else {
            return;
        };
        let bounds = layout.bounds();
        let extents = self.extents(bounds, child.bounds());
        let translation = self.translation(tree.state.downcast_ref::<State>().offset, extents);

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            child,
            Self::content_cursor(cursor, bounds, translation),
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let focus_inside = self.pause_on_hover
            && matches!(
                event,
                Event::Keyboard(_)
                    | Event::Mouse(mouse::Event::ButtonPressed(_))
                    | Event::Touch(_)
                    | Event::Window(window::Event::RedrawRequested(_))
            )
            && crate::focus::run(
                &mut self.content,
                &mut tree.children[0],
                child,
                renderer,
                crate::focus::operations::has_focus(),
            )
            .unwrap_or(false);

        let state = tree.state.downcast_mut::<State>();

        if self.pause_on_hover {
            if matches!(
                event,
                Event::Keyboard(_)
                    | Event::Mouse(mouse::Event::ButtonPressed(_))
                    | Event::Touch(_)
                    | Event::Window(window::Event::RedrawRequested(_))
            ) {
                state.focus_inside = focus_inside;
            }

            state.hovered =
                !matches!(event, Event::Mouse(mouse::Event::CursorLeft)) && cursor.is_over(bounds);
        } else {
            state.hovered = false;
            state.focus_inside = false;
        }

        let running = self.running(state, extents);

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            if running {
                if let Some(last) = state.last {
                    let seconds = now.saturating_duration_since(last).as_secs_f32();

                    state.offset = advance(state.offset, self.speed, seconds, self.period(extents));
                }
                state.last = Some(*now);
                shell.request_redraw();
            } else {
                state.last = None;
            }
        } else if running && state.last.is_none() {
            shell.request_redraw();
        }

        if !self.animated || !extents.overflows() {
            state.offset = 0.0;
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
        let Some(child) = layout.children().next() else {
            return;
        };
        let bounds = layout.bounds();
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();
        let extents = self.extents(bounds, child.bounds());
        let translation = self.translation(state.offset, extents);
        let copy = extents
            .overflows()
            .then(|| translation + self.vector(self.period(extents)));

        renderer.with_layer(clip, |renderer| {
            for (translation, cursor) in [
                (
                    Some(translation),
                    Self::content_cursor(cursor, bounds, translation),
                ),
                (copy, mouse::Cursor::Unavailable),
            ] {
                let Some(translation) = translation else {
                    continue;
                };
                let content_viewport = clip - translation;

                if !child.bounds().intersects(&content_viewport) {
                    continue;
                }

                renderer.with_translation(translation, |renderer| {
                    self.content.as_widget().draw(
                        &tree.children[0],
                        renderer,
                        theme,
                        style,
                        child,
                        cursor,
                        &content_viewport,
                    );
                });
            }
        });

        if self.fade_edges && extents.overflows() && self.fade_extent > 0.0 {
            let color = self
                .fade_color
                .as_ref()
                .map_or(theme.roles().surface, |color| color(theme));
            // The same color without opacity keeps the blend free of the
            // gray a fade to transparent black gives.
            let clear = color.scale_alpha(0.0);

            // A layer of their own keeps the gradients above the text of the
            // content.
            renderer.with_layer(clip, |renderer| {
                for at_start in [true, false] {
                    let (quad, angle) = self.fade_bounds(bounds, at_start);
                    let (first, last) = if at_start {
                        (color, clear)
                    } else {
                        (clear, color)
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
            });
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
        let Some(child) = layout.children().next() else {
            return mouse::Interaction::None;
        };
        let bounds = layout.bounds();

        if !cursor.is_over(bounds) {
            return mouse::Interaction::None;
        }

        let extents = self.extents(bounds, child.bounds());
        let translation = self.translation(tree.state.downcast_ref::<State>().offset, extents);

        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            child,
            cursor - translation,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let child = layout.children().next()?;
        let extents = self.extents(layout.bounds(), child.bounds());
        let shift = self.translation(tree.state.downcast_ref::<State>().offset, extents);

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            child,
            renderer,
            viewport,
            translation + shift,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_longer_content_in_a_viewport_overflows() {
        assert!(overflows(120.0, 60.0));
        assert!(!overflows(60.0, 60.0));
        assert!(!overflows(30.0, 60.0));
        assert!(!overflows(30.0, 0.0));
    }

    #[test]
    fn the_offset_advances_and_wraps_at_the_period() {
        assert_eq!(advance(0.0, 40.0, 0.5, 100.0), 20.0);
        assert_eq!(advance(90.0, 40.0, 0.5, 100.0), 10.0);
        assert_eq!(advance(10.0, 40.0, 1.0, 0.0), 0.0);
    }

    #[test]
    fn the_shift_stays_within_one_period_before_the_start() {
        assert_eq!(shift(0.0, 100.0, true), 0.0);
        assert_eq!(shift(30.0, 100.0, true), -30.0);
        assert_eq!(shift(0.0, 100.0, false), 0.0);
        assert_eq!(shift(30.0, 100.0, false), -70.0);
        assert_eq!(shift(130.0, 100.0, true), -30.0);
        assert_eq!(shift(10.0, 0.0, true), 0.0);
    }
}
