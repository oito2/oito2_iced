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

//! Linear and circular progress indicators, determinate (a value from 0 to
//! 1) or indeterminate (an endless animation).
//!
//! Both take their colors from the [progress bar
//! classes](crate::theme::progress_bar::Class): the bar color for the
//! progress and the background for the track.
//!
//! An indeterminate indicator advances with the time of the redraw events
//! of the window: it starts at the first frame it is drawn in and asks for
//! the next frame while it is shown. Before its first frame, and when built
//! with [`animated(false)`](Linear::animated) for reduced motion, it stays at
//! the start of its cycle.
//!
//! ```no_run
//! use oito2_iced::widget::progress;
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! # let _: oito2_iced::Element<'a, ()> =
//! progress::linear(0.25).into();
//! progress::circular_indeterminate().into()
//! # }
//! ```

use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, Path, Stroke, path::Arc};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::time::Instant;
use iced::widget::progress_bar::Catalog;
use iced::{
    Background, Border, Color, Element, Event, Length, Point, Radians, Rectangle, Size, mouse,
    window,
};

use super::helpers::ReadingDirection;
use crate::Theme;
use crate::tokens::sizes;

pub use crate::theme::progress_bar::Class;

/// Width of a linear indicator laid out without a width limit.
pub const UNBOUNDED_WIDTH: f32 = 160.0;

/// Default diameter of a circular indicator.
pub const CIRCULAR_SIZE: f32 = 32.0;

/// Default ring thickness of a circular indicator.
pub const STROKE_WIDTH: f32 = 4.0;

/// Length of one sweep of the indeterminate linear indicator; it runs back
/// and forth.
pub const LINEAR_PERIOD: Duration = Duration::from_millis(1100);

/// Length of one turn of the indeterminate circular indicator.
pub const CIRCULAR_PERIOD: Duration = Duration::from_millis(1500);

/// Share of the track covered by the moving bar of the indeterminate
/// linear indicator.
const INDETERMINATE_SHARE: f32 = 0.25;

/// The time an indeterminate indicator has been shown, kept in the widget
/// tree.
#[derive(Debug, Clone, Copy, Default)]
struct Clock {
    start: Option<Instant>,
    now: Option<Instant>,
}

impl Clock {
    /// The time between the first frame and the latest one.
    fn elapsed(&self) -> Duration {
        match (self.start, self.now) {
            (Some(start), Some(now)) => now.saturating_duration_since(start),
            _ => Duration::ZERO,
        }
    }

    /// Advances to the time of a redraw and asks for the next frame.
    fn tick<Message>(&mut self, event: &Event, shell: &mut Shell<'_, Message>) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let _ = self.start.get_or_insert(*now);
            self.now = Some(*now);
            shell.request_redraw();
        }
    }
}

/// The fraction of `period` reached after `elapsed`, from 0 up to 1.
fn cycle(elapsed: Duration, period: Duration) -> f32 {
    let period = period.as_secs_f64();

    if period <= 0.0 {
        return 0.0;
    }

    (elapsed.as_secs_f64() % period / period) as f32
}

/// The standard ease-in-out curve (control points (0.42, 0) and (0.58, 1))
/// at progress `t`, solved by bisection.
fn ease_in_out(t: f32) -> f32 {
    fn bezier(p1: f32, p2: f32, m: f32) -> f32 {
        3.0 * p1 * (1.0 - m) * (1.0 - m) * m + 3.0 * p2 * (1.0 - m) * m * m + m * m * m
    }

    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let (mut low, mut high) = (0.0_f32, 1.0_f32);

    for _ in 0..32 {
        let middle = (low + high) / 2.0;

        if bezier(0.42, 0.58, middle) < t {
            low = middle;
        } else {
            high = middle;
        }
    }

    bezier(0.0, 1.0, (low + high) / 2.0)
}

/// The position of the moving bar of an indeterminate linear indicator
/// after `elapsed`, from 0 (start of the track) to 1 (end of the track):
/// eased, forth in one period and back in the next.
fn linear_phase(elapsed: Duration) -> f32 {
    let double = cycle(elapsed, LINEAR_PERIOD * 2) * 2.0;
    let forth = if double <= 1.0 { double } else { 2.0 - double };

    ease_in_out(forth)
}

/// The start angle and the sweep, in radians clockwise from the right, of
/// the arc of an indeterminate circular indicator after `elapsed`: one turn
/// per period while the arc grows and shrinks between a quarter and three
/// quarters of a turn.
fn circular_arc(elapsed: Duration) -> (f32, f32) {
    let phase = cycle(elapsed, CIRCULAR_PERIOD);
    let sweep = PI * (0.5 + (0.5 - 0.5 * (TAU * phase).cos()));

    (-FRAC_PI_2 + TAU * phase, sweep)
}

/// The left edge and the width of the progress bar of a linear indicator
/// of `size`: from the start of the reading direction for a value, at the
/// animation `phase` otherwise. `None` when nothing is drawn.
fn linear_bar(
    value: Option<f32>,
    phase: f32,
    size: Size,
    direction: ReadingDirection,
) -> Option<(f32, f32)> {
    let (start, width) = match value {
        Some(value) if value <= 0.0 => return None,
        Some(value) => (0.0, (size.width * value.min(1.0)).max(size.height)),
        None => {
            let width = size.width * INDETERMINATE_SHARE;

            ((size.width - width) * phase, width)
        }
    };

    let width = width.min(size.width);

    Some(match direction {
        ReadingDirection::LeftToRight => (start, width),
        ReadingDirection::RightToLeft => (size.width - start - width, width),
    })
}

/// The color of a solid background, or the first stop of a gradient.
fn solid(background: Background) -> Color {
    match background {
        Background::Color(color) => color,
        Background::Gradient(iced::Gradient::Linear(linear)) => linear
            .stops
            .iter()
            .flatten()
            .next()
            .map_or(Color::TRANSPARENT, |stop| stop.color),
    }
}

/// A linear progress indicator. See [`linear`] and
/// [`linear_indeterminate`].
pub struct Linear<'a> {
    value: Option<f32>,
    width: Length,
    height: f32,
    class: Class<'a>,
    direction: ReadingDirection,
    animated: bool,
}

/// A bar showing `value`, from 0 to 1, as a pill from the start of the
/// reading direction over a pill-shaped track. It fills the available
/// width (160 pixels when unbounded) and is 6 pixels tall; a value above
/// zero shows at least a circle.
pub fn linear<'a>(value: f32) -> Linear<'a> {
    Linear {
        value: Some(value.clamp(0.0, 1.0)),
        ..linear_indeterminate()
    }
}

/// A bar for progress of unknown length: a pill a quarter of the track
/// long sweeps back and forth, eased, every 1.1 seconds.
pub fn linear_indeterminate<'a>() -> Linear<'a> {
    Linear {
        value: None,
        width: Length::Fill,
        height: sizes::PROGRESS_HEIGHT,
        class: Class::Default,
        direction: ReadingDirection::LeftToRight,
        animated: true,
    }
}

impl<'a> Linear<'a> {
    /// Sets the width (fills the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the bar (6 pixels by default).
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// Sets the style class.
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the reading direction the progress starts from.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether indeterminate progress moves (true by default); false
    /// keeps it still, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Linear<'_>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Clock>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Clock::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(
            self.width,
            Length::Fixed(self.height),
            Size::new(UNBOUNDED_WIDTH, self.height),
        );
        let width = if size.width.is_finite() {
            size.width
        } else {
            UNBOUNDED_WIDTH
        };

        layout::Node::new(Size::new(width, size.height))
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
        if self.value.is_none() && self.animated {
            tree.state.downcast_mut::<Clock>().tick(event, shell);
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
        let bounds = layout.bounds();
        let style = Catalog::style(theme, &self.class);
        let radius = bounds.height / 2.0;

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border::default().rounded(radius),
                ..Quad::default()
            },
            style.background,
        );

        let phase = if self.animated {
            linear_phase(tree.state.downcast_ref::<Clock>().elapsed())
        } else {
            0.0
        };

        if let Some((left, width)) = linear_bar(self.value, phase, bounds.size(), self.direction) {
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds.x + left,
                        width,
                        ..bounds
                    },
                    border: Border::default().rounded(radius),
                    ..Quad::default()
                },
                style.bar,
            );
        }
    }
}

impl<'a, Message, Renderer> From<Linear<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer + 'a,
{
    fn from(linear: Linear<'a>) -> Self {
        Element::new(linear)
    }
}

/// A circular progress indicator. See [`circular`] and
/// [`circular_indeterminate`].
pub struct Circular<'a> {
    value: Option<f32>,
    size: f32,
    stroke_width: f32,
    class: Class<'a>,
    animated: bool,
}

/// A ring showing `value`, from 0 to 1, as an arc with round ends going
/// clockwise from the top over the full track ring. It is 32 pixels wide
/// with a 4-pixel ring.
pub fn circular<'a>(value: f32) -> Circular<'a> {
    Circular {
        value: Some(value.clamp(0.0, 1.0)),
        ..circular_indeterminate()
    }
}

/// A ring for progress of unknown length, without a track: an arc turns
/// once every 1.5 seconds while its length grows and shrinks between a
/// quarter and three quarters of a turn.
pub fn circular_indeterminate<'a>() -> Circular<'a> {
    Circular {
        value: None,
        size: CIRCULAR_SIZE,
        stroke_width: STROKE_WIDTH,
        class: Class::Default,
        animated: true,
    }
}

impl<'a> Circular<'a> {
    /// Sets the diameter (32 pixels by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    /// Sets the ring thickness (4 pixels by default).
    #[must_use]
    pub fn stroke_width(mut self, width: f32) -> Self {
        self.stroke_width = width.max(0.0);
        self
    }

    /// Sets the style class.
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets whether indeterminate progress moves (true by default); false
    /// keeps it still, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Circular<'_>
where
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Clock>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Clock::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = Length::Fixed(self.size);

        layout::Node::new(limits.resolve(side, side, Size::new(self.size, self.size)))
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
        if self.value.is_none() && self.animated {
            tree.state.downcast_mut::<Clock>().tick(event, shell);
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
        let bounds = layout.bounds();
        let style = Catalog::style(theme, &self.class);
        let stroke = self.stroke_width;
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = ((bounds.width.min(bounds.height) - stroke) / 2.0).max(0.0);

        let ring = |color: Color| {
            Stroke::default()
                .with_color(color)
                .with_width(stroke)
                .with_line_cap(LineCap::Round)
        };

        let (start, sweep) = match self.value {
            Some(value) => {
                frame.stroke(&Path::circle(center, radius), ring(solid(style.background)));

                (-FRAC_PI_2, TAU * value)
            }
            None if self.animated => circular_arc(tree.state.downcast_ref::<Clock>().elapsed()),
            None => circular_arc(Duration::ZERO),
        };

        let bar = ring(solid(style.bar));

        if sweep >= TAU {
            frame.stroke(&Path::circle(center, radius), bar);
        } else if sweep > 0.0 {
            let arc = Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: Radians(start),
                    end_angle: Radians(start + sweep),
                });
            });

            frame.stroke(&arc, bar);
        }

        let geometry = frame.into_geometry();

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message, Renderer> From<Circular<'a>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer + 'a,
{
    fn from(circular: Circular<'a>) -> Self {
        Element::new(circular)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-3
    }

    #[test]
    fn the_ease_in_out_curve_is_symmetric() {
        assert_eq!(ease_in_out(0.0), 0.0);
        assert_eq!(ease_in_out(1.0), 1.0);
        assert!(close(ease_in_out(0.5), 0.5));
        assert!(close(ease_in_out(0.25) + ease_in_out(0.75), 1.0));
        assert!(ease_in_out(0.1) < 0.1);
    }

    #[test]
    fn the_linear_phase_goes_forth_and_back() {
        assert_eq!(linear_phase(Duration::ZERO), 0.0);
        assert!(close(linear_phase(LINEAR_PERIOD), 1.0));
        assert!(close(linear_phase(LINEAR_PERIOD / 2), 0.5));
        assert!(close(linear_phase(LINEAR_PERIOD * 3 / 2), 0.5));
        assert!(close(linear_phase(LINEAR_PERIOD * 2), 0.0));
    }

    #[test]
    fn the_circular_arc_turns_and_breathes() {
        let (start, sweep) = circular_arc(Duration::ZERO);
        assert!(close(start, -FRAC_PI_2));
        assert!(close(sweep, PI / 2.0));

        let (start, sweep) = circular_arc(CIRCULAR_PERIOD / 2);
        assert!(close(start, FRAC_PI_2));
        assert!(close(sweep, PI * 1.5));
    }

    #[test]
    fn a_determinate_bar_starts_at_the_reading_start() {
        let size = Size::new(300.0, 6.0);

        assert_eq!(
            linear_bar(Some(0.25), 0.0, size, ReadingDirection::LeftToRight),
            Some((0.0, 75.0))
        );
        assert_eq!(
            linear_bar(Some(0.25), 0.0, size, ReadingDirection::RightToLeft),
            Some((225.0, 75.0))
        );
        assert_eq!(
            linear_bar(Some(0.0), 0.0, size, ReadingDirection::LeftToRight),
            None
        );
        assert_eq!(
            linear_bar(Some(0.001), 0.0, size, ReadingDirection::LeftToRight),
            Some((0.0, 6.0))
        );
    }

    #[test]
    fn an_indeterminate_bar_covers_a_quarter_of_the_track() {
        let size = Size::new(200.0, 6.0);

        assert_eq!(
            linear_bar(None, 0.0, size, ReadingDirection::LeftToRight),
            Some((0.0, 50.0))
        );
        assert_eq!(
            linear_bar(None, 1.0, size, ReadingDirection::LeftToRight),
            Some((150.0, 50.0))
        );
        assert_eq!(
            linear_bar(None, 0.0, size, ReadingDirection::RightToLeft),
            Some((150.0, 50.0))
        );
    }
}
