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

//! Rating: a row of stars showing a value from 0 to the number of stars,
//! which the user changes with a click or the keyboard.
//!
//! With [`Rating::allow_half`] the value moves in halves: the start half of
//! a star gives the half value. While the pointer is over the stars they
//! preview the value under it. While focused, the arrow keys change the
//! value by one step (left and right follow the
//! [reading direction](Rating::direction)), Home and End go to 0 and to the
//! number of stars, and the digit keys 0 to 9 pick that whole value, up to
//! the number of stars. A click focuses the stars, so the keys work next.
//! A star that fills pops in, and a half star fills from its start; a
//! rating shows its final state the first time it is drawn.
//!
//! The stars are drawn as geometry, not icons, so a half star fills
//! exactly half of its outline.
//!
//! The value lives in the application: the rating shows `value` and
//! publishes new values through [`Rating::on_change`]; without it the
//! rating is read-only and skipped by Tab. [`value_text`] gives the
//! localized description of a value, such as "3.5 of 5 stars".
//!
//! ```no_run
//! use oito2_iced::widget::rating;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Rated(f32),
//! }
//!
//! # fn view<'a>(stars: f32) -> oito2_iced::Element<'a, Message> {
//! rating::rating(stars)
//!     .allow_half(true)
//!     .on_change(Message::Rated)
//!     .into()
//! # }
//! ```

use std::f32::consts::{FRAC_PI_2, PI, TAU};
use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::{
    Color, Element as IcedElement, Event, Length, Point, Rectangle, Size, mouse, touch, window,
};

use super::helpers::ReadingDirection;
use crate::l10n::Localizations;
use crate::{Element, Theme};

/// The default side of a star.
pub const STAR_SIZE: f32 = 24.0;

/// The default number of stars.
pub const DEFAULT_MAX: u32 = 5;

/// The length of the fill animation of a star.
pub const FILL_DURATION: Duration = Duration::from_millis(700);

/// The corner radius of the focus ring (a literal in the OITO2 design).
const RING_RADIUS: f32 = 4.0;

/// The side of the grid the star geometry is defined on.
const GRID: f32 = 24.0;

/// The center of the star on the grid.
const STAR_CENTER: (f32, f32) = (12.0, 12.8);

/// The radius of the star tips and of its inner corners on the grid.
const STAR_RADII: (f32, f32) = (10.0, 4.4);

/// The width of the star outline on the grid.
const OUTLINE_WIDTH: f32 = 2.0;

/// A row of stars showing a value. See [`rating`].
pub struct Rating<'a, Message> {
    value: f32,
    max: u32,
    allow_half: bool,
    size: f32,
    color: Option<Color>,
    on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    direction: ReadingDirection,
    animated: bool,
    id: Option<Id>,
}

/// A rating showing `value` out of [`DEFAULT_MAX`] stars unless
/// [`Rating::max`] says otherwise. Without [`Rating::on_change`] it is
/// read-only.
pub fn rating<'a, Message>(value: f32) -> Rating<'a, Message> {
    Rating {
        value,
        max: DEFAULT_MAX,
        allow_half: false,
        size: STAR_SIZE,
        color: None,
        on_change: None,
        direction: ReadingDirection::LeftToRight,
        animated: true,
        id: None,
    }
}

impl<'a, Message> Rating<'a, Message> {
    /// Sets the number of stars, at least 1.
    #[must_use]
    pub fn max(mut self, max: u32) -> Self {
        self.max = max.max(1);
        self
    }

    /// Sets whether half values can be picked (false by default).
    #[must_use]
    pub fn allow_half(mut self, allow_half: bool) -> Self {
        self.allow_half = allow_half;
        self
    }

    /// Sets the side of a star ([`STAR_SIZE`] by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    /// Sets the color of the filled stars; the theme's warning color by
    /// default.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the function that turns a new value into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or makes the rating read-only with
    /// `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(f32) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(f32) -> Message + 'a>);
        self
    }

    /// Sets the reading direction: right to left mirrors the stars, the
    /// pointer and the left and right arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether a star animates when it fills (true by default); false
    /// shows the final state at once, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the widget [`Id`], used to focus the rating programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// The localized description of `value` out of `max` stars, such as
/// "3.5 of 5 stars" or "3,5 de 5 estrelas": whole values without
/// decimals, other values with one.
pub fn value_text(value: f32, max: u32, localizations: &Localizations) -> String {
    let number = if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    };
    let number = if localizations.locale_name == "pt_BR" {
        number.replace('.', ",")
    } else {
        number
    };

    localizations.rating_value(&number, i64::from(max))
}

/// How a star is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The outline only.
    Empty,
    /// The outline with its start half filled.
    Half,
    /// A filled star.
    Full,
}

/// The drawing of a star and when its animation started.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Star {
    kind: Kind,
    /// Whether the star shows without animating: while previewing, or
    /// empty.
    still: bool,
    started: Option<Instant>,
}

/// The tree state of a [`Rating`].
#[derive(Debug, Clone, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the ring.
    keyboard: bool,
    /// Whether the left button was pressed over the stars.
    pressed: bool,
    stars: Vec<Star>,
    now: Option<Instant>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<Message> Rating<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_change.is_some()
    }

    fn step(&self) -> f32 {
        if self.allow_half { 0.5 } else { 1.0 }
    }

    /// `value` aligned to the step, so 3.7 reads 3.5 or 4.
    fn aligned(&self, value: f32) -> f32 {
        (value / self.step()).round() * self.step()
    }

    /// `value` aligned to the step and clamped between 0 and the number of
    /// stars.
    fn clamped(&self, value: f32) -> f32 {
        self.aligned(value).clamp(0.0, self.max as f32)
    }

    /// The value a step after (or `steps` before, when negative) the
    /// aligned value, so a value between steps moves to the next one.
    fn stepped(&self, steps: f32) -> f32 {
        self.clamped(self.aligned(self.value) + steps * self.step())
    }

    /// The value under the point `x` of a row with `bounds`.
    fn value_at(&self, bounds: Rectangle, x: f32) -> f32 {
        let width = self.size * self.max as f32;
        let dx = x - bounds.x;
        let position = match self.direction {
            ReadingDirection::LeftToRight => dx,
            ReadingDirection::RightToLeft => width - dx,
        }
        .clamp(0.0, (width - 0.001).max(0.0));
        let star = if self.size > 0.0 {
            position / self.size
        } else {
            0.0
        };
        let whole = star.floor();
        let half = self.allow_half && star - whole < 0.5;

        whole + if half { 0.5 } else { 1.0 }
    }

    /// The value the stars show: the one under the pointer while it is
    /// over an enabled rating, otherwise the value.
    fn shown(&self, bounds: Rectangle, cursor: mouse::Cursor) -> (f32, bool) {
        match cursor.position_over(bounds) {
            Some(position) if self.enabled() => (self.value_at(bounds, position.x), true),
            _ => (self.value, false),
        }
    }

    /// The drawing of each star when the stars show `shown`.
    fn stars(&self, shown: f32, hovering: bool) -> Vec<Star> {
        (0..self.max)
            .map(|index| {
                let fill = (shown - index as f32).clamp(0.0, 1.0);
                let kind = if fill >= 1.0 {
                    Kind::Full
                } else if fill >= 0.5 {
                    Kind::Half
                } else {
                    Kind::Empty
                };

                Star {
                    kind,
                    still: hovering || kind == Kind::Empty,
                    started: None,
                }
            })
            .collect()
    }

    /// The value a key sets, if `key` is one the rating takes.
    fn value_for(&self, key: &keyboard::Key) -> Option<f32> {
        let (back, forward) = self.direction.arrows();

        match key {
            keyboard::Key::Named(named) if *named == forward => Some(self.stepped(1.0)),
            keyboard::Key::Named(named) if *named == back => Some(self.stepped(-1.0)),
            keyboard::Key::Named(key::Named::ArrowUp) => Some(self.stepped(1.0)),
            keyboard::Key::Named(key::Named::ArrowDown) => Some(self.stepped(-1.0)),
            keyboard::Key::Named(key::Named::Home) => Some(0.0),
            keyboard::Key::Named(key::Named::End) => Some(self.max as f32),
            keyboard::Key::Character(text) => {
                let digit = text.parse::<u32>().ok().filter(|digit| *digit <= 9)?;

                (digit <= self.max).then_some(digit as f32)
            }
            _ => None,
        }
    }

    /// Publishes `value` when it differs from the current value.
    fn set(&self, shell: &mut Shell<'_, Message>, value: f32) {
        let value = self.clamped(value);

        if value != self.value
            && let Some(on_change) = &self.on_change
        {
            shell.publish(on_change(value));
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Rating<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            stars: self.stars(self.value, false),
            ..State::default()
        })
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.size * self.max as f32),
            Length::Fixed(self.size),
        )
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = Widget::<Message, Theme, iced::Renderer>::size(self);

        layout::atomic(limits, size.width, size.height)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), bounds, state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), bounds);
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
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let redraw_at = match event {
            Event::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };

        let (shown, hovering) = self.shown(bounds, cursor);
        let stars = self.stars(shown, hovering);

        if stars.len() == state.stars.len() {
            let mut changed = false;

            for (star, old) in stars.iter().zip(&mut state.stars) {
                if star.kind != old.kind || star.still != old.still {
                    let started = (!star.still && self.animated)
                        .then(|| redraw_at.unwrap_or_else(Instant::now));

                    *old = Star { started, ..*star };
                    changed = true;
                }
            }

            if changed {
                shell.request_redraw();
            }
        } else {
            state.stars = stars;
        }

        if let Some(now) = redraw_at {
            state.now = Some(now);

            let animating = state.stars.iter().any(|star| {
                star.started
                    .is_some_and(|started| now.saturating_duration_since(started) < FILL_DURATION)
            });

            if animating {
                shell.request_redraw();
            }
        }

        if !self.enabled() {
            state.focused = false;
            state.pressed = false;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    state.pressed = true;
                    shell.capture_event();
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if state.pressed {
                    state.pressed = false;

                    if let Some(position) = cursor.position_over(bounds) {
                        state.focused = true;
                        state.keyboard = false;
                        self.set(shell, self.value_at(bounds, position.x));
                        shell.capture_event();
                    }
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if cursor.is_over(bounds) {
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = false;
                shell.request_redraw();
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused && !shell.is_event_captured() && modifiers.is_empty() =>
            {
                if let Some(value) = self.value_for(key) {
                    self.set(shell, value);
                    state.keyboard = true;
                    shell.capture_event();
                    shell.request_redraw();
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
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let bounds = layout.bounds();
        let filled = self.color.unwrap_or(roles.semantic.warning);
        let (shown, hovering) = self.shown(bounds, cursor);
        let stars = self.stars(shown, hovering);
        let mirrored = self.direction == ReadingDirection::RightToLeft;

        let mut frame = Frame::new(renderer, bounds.size());

        for (index, star) in stars.iter().enumerate() {
            let progress = state
                .stars
                .get(index)
                .filter(|known| known.kind == star.kind && known.still == star.still)
                .and_then(|known| known.started.zip(state.now))
                .map_or(1.0, |(started, now)| {
                    now.saturating_duration_since(started).as_secs_f32()
                        / FILL_DURATION.as_secs_f32()
                })
                .clamp(0.0, 1.0);
            let progress = if star.still { 1.0 } else { progress };

            let slot = if mirrored {
                self.max - 1 - index as u32
            } else {
                index as u32
            };
            let origin = Point::new(slot as f32 * self.size, 0.0);
            let scale = self.size / GRID;
            let color = if star.kind == Kind::Empty {
                roles.outline
            } else {
                filled
            };

            draw_star(
                &mut frame, origin, scale, star.kind, progress, mirrored, color,
            );
        }

        draw_frame(renderer, frame, bounds.position());

        if state.focused && state.keyboard && self.enabled() {
            crate::focus::draw_ring(renderer, theme, bounds, RING_RADIUS);
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
        if self.enabled() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: 'a> From<Rating<'a, Message>> for Element<'a, Message> {
    fn from(rating: Rating<'a, Message>) -> Self {
        IcedElement::new(rating)
    }
}

/// The corners of the star on the grid, from the top tip clockwise.
fn star_points() -> [Point; 10] {
    std::array::from_fn(|index| {
        let radius = if index % 2 == 0 {
            STAR_RADII.0
        } else {
            STAR_RADII.1
        };
        let angle = -FRAC_PI_2 + index as f32 * PI / 5.0;

        Point::new(
            STAR_CENTER.0 + radius * angle.cos(),
            STAR_CENTER.1 + radius * angle.sin(),
        )
    })
}

/// A closed path through `points`, mapped from the grid to a star at
/// `origin` scaled by `scale`.
fn polygon(points: &[Point], origin: Point, scale: f32) -> Path {
    Path::new(|builder| {
        for (index, point) in points.iter().enumerate() {
            let point = Point::new(origin.x + point.x * scale, origin.y + point.y * scale);

            if index == 0 {
                builder.move_to(point);
            } else {
                builder.line_to(point);
            }
        }
        builder.close();
    })
}

/// Draws a star of `kind` at `origin`, scaled from the grid by `scale`, at
/// the animation `progress` (1 when settled). A full star turns into place
/// and pops in; a half star fills its start half, the right half when
/// `mirrored`.
fn draw_star(
    frame: &mut Frame<iced::Renderer>,
    origin: Point,
    scale: f32,
    kind: Kind,
    progress: f32,
    mirrored: bool,
    color: Color,
) {
    let points = star_points();
    let outline = || {
        Stroke::default()
            .with_color(color)
            .with_width(OUTLINE_WIDTH * scale)
            .with_line_join(LineJoin::Round)
    };

    match kind {
        Kind::Empty => frame.stroke(&polygon(&points, origin, scale), outline()),
        Kind::Full => {
            let angle = (1.0 - ease_out(progress)) * -TAU / 5.0;
            let grow = ease_out_back(progress);
            let (sin, cos) = angle.sin_cos();
            let (cx, cy) = STAR_CENTER;
            let center = GRID / 2.0;

            let turned = points.map(|point| {
                let (dx, dy) = (point.x - cx, point.y - cy);
                let rotated = Point::new(cx + dx * cos - dy * sin, cy + dx * sin + dy * cos);

                Point::new(
                    center + (rotated.x - center) * grow,
                    center + (rotated.y - center) * grow,
                )
            });

            frame.fill(&polygon(&turned, origin, scale), color);
        }
        Kind::Half => {
            frame.stroke(&polygon(&points, origin, scale), outline());

            let width = GRID / 2.0 * ease_out(progress);
            let half = if mirrored {
                clip(&points, |x| x >= GRID - width, GRID - width)
            } else {
                clip(&points, |x| x <= width, width)
            };

            if half.len() > 2 {
                frame.fill(&polygon(&half, origin, scale), color);
            }
        }
    }
}

/// The part of the polygon `points` on the side of the vertical line at
/// `edge` where `inside` holds.
fn clip(points: &[Point], inside: impl Fn(f32) -> bool, edge: f32) -> Vec<Point> {
    let mut clipped = Vec::with_capacity(points.len() + 2);

    for (index, &current) in points.iter().enumerate() {
        let previous = points[(index + points.len() - 1) % points.len()];
        let crossing = || {
            let t = (edge - previous.x) / (current.x - previous.x);

            Point::new(edge, previous.y + (current.y - previous.y) * t)
        };

        match (inside(previous.x), inside(current.x)) {
            (true, true) => clipped.push(current),
            (true, false) => clipped.push(crossing()),
            (false, true) => {
                clipped.push(crossing());
                clipped.push(current);
            }
            (false, false) => {}
        }
    }

    clipped
}

/// The ease-out curve: control points (0, 0) and (0.58, 1).
fn ease_out(t: f32) -> f32 {
    cubic((0.0, 0.0), (0.58, 1.0), t)
}

/// The ease-out curve that overshoots and settles back: control points
/// (0.175, 0.885) and (0.32, 1.275).
fn ease_out_back(t: f32) -> f32 {
    cubic((0.175, 0.885), (0.32, 1.275), t)
}

/// A cubic Bézier from (0, 0) to (1, 1) with the control points `a` and
/// `b`, evaluated at x = `t` by bisection.
fn cubic(a: (f32, f32), b: (f32, f32), t: f32) -> f32 {
    let at = |p1: f32, p2: f32, m: f32| {
        3.0 * p1 * (1.0 - m) * (1.0 - m) * m + 3.0 * p2 * (1.0 - m) * m * m + m * m * m
    };

    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let (mut low, mut high) = (0.0_f32, 1.0_f32);

    for _ in 0..24 {
        let middle = (low + high) / 2.0;

        if at(a.0, b.0, middle) < t {
            low = middle;
        } else {
            high = middle;
        }
    }

    at(a.1, b.1, (low + high) / 2.0)
}

/// Draws the geometry of `frame` with its origin at `origin`.
fn draw_frame<Renderer>(renderer: &mut Renderer, frame: Frame<Renderer>, origin: Point)
where
    Renderer: geometry::Renderer,
{
    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_start_and_end_in_place() {
        assert_eq!(ease_out(0.0), 0.0);
        assert_eq!(ease_out(1.0), 1.0);
        assert!(ease_out(0.5) > 0.5);
        assert!((0..=20).any(|step| ease_out_back(step as f32 / 20.0) > 1.0));
    }

    #[test]
    fn a_half_clip_keeps_the_start_of_the_star() {
        let half = clip(&star_points(), |x| x <= 12.0, 12.0);

        assert!(half.iter().all(|point| point.x <= 12.0 + 1e-4));
        assert!(half.iter().any(|point| (point.x - 12.0).abs() < 1e-4));
        assert!(half.len() >= 6);
    }

    #[test]
    fn values_read_in_the_locale() {
        assert_eq!(value_text(3.5, 5, &Localizations::en()), "3.5 of 5 stars");
        assert_eq!(value_text(4.0, 5, &Localizations::en()), "4 of 5 stars");
        assert_eq!(
            value_text(3.5, 5, &Localizations::pt_br()),
            "3,5 de 5 estrelas"
        );
    }
}
