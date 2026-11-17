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

//! Timers and the text that shows them.
//!
//! A [`Timer`] is a stopwatch, or a countdown when built with
//! [`Timer::countdown`]. It is a plain value the application owns: the
//! application starts, pauses and resets it from `update`, passing the
//! current [`Instant`]. Time is measured from those instants, so it stays
//! exact however late the frames arrive.
//!
//! [`timer_text`] shows the remaining time of a countdown or the elapsed
//! time of a stopwatch as `m:ss` (or `h:mm:ss` from an hour), with each
//! character in a cell of fixed width so the digits don't shift. While the
//! timer runs, the text redraws itself when the shown time changes; no
//! subscription is needed. It publishes [`TimerText::on_tick`] every
//! [`Timer::interval`] and [`TimerText::on_complete`] when a countdown
//! reaches zero, with the instant of the event; the application passes
//! that instant to [`Timer::tick`], which stops a finished countdown.
//!
//! ```no_run
//! use oito2_iced::widget::timer::{self, Timer};
//! use std::time::{Duration, Instant};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Start,
//!     Completed(Instant),
//! }
//!
//! struct App {
//!     timer: Timer,
//! }
//!
//! impl App {
//!     fn new() -> Self {
//!         Self { timer: Timer::countdown(Duration::from_secs(90)) }
//!     }
//!
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::Start => self.timer.start(Instant::now()),
//!             Message::Completed(at) => {
//!                 self.timer.tick(at);
//!             }
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         timer::timer_text(&self.timer)
//!             .on_complete(Message::Completed)
//!             .into()
//!     }
//! }
//! ```

use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Paragraph as _, Renderer as _};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::time::Instant;
use iced::{
    Color, Element as IcedElement, Event, Length, Pixels, Point, Rectangle, Size, mouse, window,
};

use crate::tokens::typography::{self, TextStyle};
use crate::{Element, Theme};

/// The default [`Timer::interval`]: one second.
pub const DEFAULT_INTERVAL: Duration = Duration::from_secs(1);

/// The shortest [`Timer::interval`].
const MIN_INTERVAL: Duration = Duration::from_millis(1);

/// The delay added after the instant the shown text changes, so the redraw
/// lands on the new text.
const REDRAW_MARGIN: Duration = Duration::from_millis(1);

/// A stopwatch, or a countdown that stops at zero.
///
/// Every method that depends on the clock takes the current [`Instant`]:
/// the application passes `Instant::now()` from `update`, or the instant a
/// [`TimerText`] message carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    countdown: Option<Duration>,
    interval: Duration,
    started_at: Option<Instant>,
    offset: Duration,
    completed: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self::stopwatch()
    }
}

impl Timer {
    /// A stopped stopwatch at zero.
    #[must_use]
    pub fn stopwatch() -> Self {
        Self {
            countdown: None,
            interval: DEFAULT_INTERVAL,
            started_at: None,
            offset: Duration::ZERO,
            completed: false,
        }
    }

    /// A stopped countdown of `length`.
    #[must_use]
    pub fn countdown(length: Duration) -> Self {
        Self {
            countdown: Some(length),
            ..Self::stopwatch()
        }
    }

    /// Sets how often a running [`TimerText`] publishes
    /// [`TimerText::on_tick`] ([`DEFAULT_INTERVAL`] by default); intervals
    /// below one millisecond become one millisecond.
    #[must_use]
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval.max(MIN_INTERVAL);
        self
    }

    /// The length of a countdown, or `None` for a stopwatch.
    #[must_use]
    pub fn length(&self) -> Option<Duration> {
        self.countdown
    }

    /// How often a running [`TimerText`] publishes [`TimerText::on_tick`].
    #[must_use]
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// The time run until `now`, not clamped to the countdown.
    fn raw(&self, now: Instant) -> Duration {
        self.offset
            + self.started_at.map_or(Duration::ZERO, |started| {
                now.saturating_duration_since(started)
            })
    }

    /// The time run until `now`, up to the length of a countdown.
    #[must_use]
    pub fn elapsed(&self, now: Instant) -> Duration {
        let raw = self.raw(now);

        match self.countdown {
            Some(length) => raw.min(length),
            None => raw,
        }
    }

    /// The time left of a countdown at `now`, or `None` for a stopwatch.
    #[must_use]
    pub fn remaining(&self, now: Instant) -> Option<Duration> {
        self.countdown
            .map(|length| length.saturating_sub(self.elapsed(now)))
    }

    /// Whether the timer is running.
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.started_at.is_some()
    }

    /// Whether a countdown has reached zero and was stopped by
    /// [`Timer::tick`].
    #[must_use]
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// The instant a running countdown reaches zero, or `None` for a
    /// stopwatch or a stopped timer.
    #[must_use]
    pub fn completes_at(&self) -> Option<Instant> {
        let started = self.started_at?;
        let length = self.countdown?;

        Some(started + length.saturating_sub(self.offset))
    }

    /// Starts or resumes the timer at `now`; does nothing while it runs or
    /// after a countdown completes (see [`Timer::restart`]).
    pub fn start(&mut self, now: Instant) {
        if self.is_running() || self.completed {
            return;
        }

        self.started_at = Some(now);
    }

    /// Pauses the timer at `now`, keeping the time run so far; does
    /// nothing when it isn't running.
    pub fn pause(&mut self, now: Instant) {
        if !self.is_running() {
            return;
        }

        self.offset = self.raw(now);
        self.started_at = None;
    }

    /// Stops the timer and sets it back to zero, clearing
    /// [`Timer::is_completed`].
    pub fn reset(&mut self) {
        self.started_at = None;
        self.offset = Duration::ZERO;
        self.completed = false;
    }

    /// Sets the timer back to zero and starts it at `now`.
    pub fn restart(&mut self, now: Instant) {
        self.reset();
        self.start(now);
    }

    /// Completes a running countdown that has reached zero at `now`:
    /// stops it at its length and returns true. Returns false otherwise.
    pub fn tick(&mut self, now: Instant) -> bool {
        match self.countdown {
            Some(length) if self.is_running() && self.raw(now) >= length => {
                self.started_at = None;
                self.offset = length;
                self.completed = true;
                true
            }
            _ => false,
        }
    }
}

/// Formats `duration` as `m:ss`, or `h:mm:ss` from an hour, with tenths of
/// a second (truncated) when `tenths` is set.
#[must_use]
pub fn format_duration(duration: Duration, tenths: bool) -> String {
    let total = duration.as_secs();
    let hours = total / 3600;
    let minutes = total / 60 % 60;
    let seconds = total % 60;
    let base = if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    };

    if tenths {
        format!("{base}.{}", duration.subsec_millis() / 100)
    } else {
        base
    }
}

/// The time a timer shows at `now` as text: the remaining time of a
/// countdown, rounded up to the whole second without `tenths`, or the
/// elapsed time of a stopwatch.
#[must_use]
pub fn timer_label(timer: &Timer, now: Instant, tenths: bool) -> String {
    let shown = match timer.remaining(now) {
        Some(remaining) if !tenths => {
            let whole = remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0);

            Duration::from_secs(whole)
        }
        Some(remaining) => remaining,
        None => timer.elapsed(now),
    };

    format_duration(shown, tenths)
}

/// The time of a [`Timer`] as text. See [`timer_text`].
pub struct TimerText<'a, Message> {
    timer: Timer,
    style: TextStyle,
    color: Option<Color>,
    tenths: bool,
    on_tick: Option<Box<dyn Fn(Instant) -> Message + 'a>>,
    on_complete: Option<Box<dyn Fn(Instant) -> Message + 'a>>,
}

/// The time of `timer` as text, in the body medium style and the text
/// color of the parent.
pub fn timer_text<'a, Message>(timer: &Timer) -> TimerText<'a, Message> {
    TimerText {
        timer: *timer,
        style: typography::BODY_MEDIUM,
        color: None,
        tenths: false,
        on_tick: None,
        on_complete: None,
    }
}

impl<'a, Message> TimerText<'a, Message> {
    /// Sets the style of the type scale ([`typography::BODY_MEDIUM`] by
    /// default).
    #[must_use]
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the text color, instead of the text color of the parent.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets whether tenths of a second show (false by default); without
    /// them a countdown rounds up to the whole second.
    #[must_use]
    pub fn tenths(mut self, tenths: bool) -> Self {
        self.tenths = tenths;
        self
    }

    /// Sets the message published every [`Timer::interval`] while the
    /// timer runs, and when a countdown reaches zero, with the instant of
    /// the tick.
    #[must_use]
    pub fn on_tick(mut self, on_tick: impl Fn(Instant) -> Message + 'a) -> Self {
        self.on_tick = Some(Box::new(on_tick));
        self
    }

    /// Sets the message published once when a running countdown reaches
    /// zero, with the instant it did; pass that instant to [`Timer::tick`].
    #[must_use]
    pub fn on_complete(mut self, on_complete: impl Fn(Instant) -> Message + 'a) -> Self {
        self.on_complete = Some(Box::new(on_complete));
        self
    }

    /// The text at `now`.
    fn label(&self, now: Instant) -> String {
        timer_label(&self.timer, now, self.tenths)
    }

    /// The time until the shown text changes after `now`, while the timer
    /// runs and a countdown has not reached zero.
    fn next_change(&self, now: Instant) -> Option<Duration> {
        if !self.timer.is_running() {
            return None;
        }

        let unit = if self.tenths {
            Duration::from_millis(100)
        } else {
            Duration::from_secs(1)
        };
        let unit_nanos = unit.as_nanos();

        let wait = match self.timer.remaining(now) {
            Some(remaining) if remaining.is_zero() => return None,
            Some(remaining) => {
                let rest = remaining.as_nanos() % unit_nanos;

                // Truncated tenths change just below a multiple of the unit,
                // rounded-up seconds change on it.
                if rest == 0 && !self.tenths {
                    unit_nanos
                } else {
                    rest
                }
            }
            None => unit_nanos - self.timer.raw(now).as_nanos() % unit_nanos,
        };

        Some(Duration::from_nanos(
            u64::try_from(wait).unwrap_or(u64::MAX),
        ))
    }
}

/// The widths of the character cells of a [`TimerText`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Cells {
    digit: f32,
    colon: f32,
    dot: f32,
}

impl Cells {
    /// The width of the cell of `character`.
    fn of(&self, character: char) -> f32 {
        match character {
            ':' => self.colon,
            '.' => self.dot,
            _ => self.digit,
        }
    }

    /// The width of `label`.
    fn width(&self, label: &str) -> f32 {
        label.chars().map(|character| self.of(character)).sum()
    }
}

/// The tree state of a [`TimerText`].
#[derive(Debug, Clone, Default)]
struct State {
    now: Option<Instant>,
    cells: Cells,
    /// The width the text was laid out with.
    width: f32,
    /// The number of whole intervals run at the last tick.
    ticks: Option<u128>,
    /// Whether the completion of the running countdown was published.
    completed: bool,
}

impl<Message> TimerText<'_, Message> {
    /// The text description of one character in the style of the widget.
    fn text<'b>(&self, content: &'b str, bounds: Size) -> text::Text<&'b str> {
        text::Text {
            content,
            bounds,
            size: Pixels(self.style.size),
            line_height: text::LineHeight::Absolute(Pixels(self.style.line_height)),
            font: crate::font::inter(self.style.weight),
            align_x: text::Alignment::Center,
            align_y: iced::alignment::Vertical::Center,
            shaping: text::Shaping::Advanced,
            wrapping: text::Wrapping::None,
        }
    }

    /// The width of `content` alone.
    fn measure<R: text::Renderer<Font = iced::Font>>(&self, content: &str) -> f32 {
        R::Paragraph::with_text(self.text(content, Size::INFINITE)).min_width()
    }

    /// Publishes the ticks and the completion due at `now`.
    fn publish(&self, state: &mut State, now: Instant, shell: &mut Shell<'_, Message>) {
        if !self.timer.is_running() {
            state.ticks = None;
            state.completed = false;
            return;
        }

        if let Some(at) = self.timer.completes_at().filter(|at| *at <= now) {
            if !state.completed {
                state.completed = true;

                if let Some(on_tick) = &self.on_tick {
                    shell.publish(on_tick(at));
                }
                if let Some(on_complete) = &self.on_complete {
                    shell.publish(on_complete(at));
                }
            }
            return;
        }

        state.completed = false;

        let count = self.timer.raw(now).as_nanos() / self.timer.interval.as_nanos();

        if let Some(last) = state.ticks
            && count > last
            && let Some(on_tick) = &self.on_tick
        {
            shell.publish(on_tick(now));
        }

        state.ticks = Some(count);
    }

    /// The time until the next tick or completion after `now`.
    fn next_event(&self, now: Instant) -> Option<Duration> {
        if !self.timer.is_running() || (self.on_tick.is_none() && self.on_complete.is_none()) {
            return None;
        }

        let interval = self.timer.interval.as_nanos();
        let tick = interval - self.timer.raw(now).as_nanos() % interval;
        let tick = Duration::from_nanos(u64::try_from(tick).unwrap_or(u64::MAX));

        match self.timer.completes_at() {
            Some(at) if at <= now => None,
            Some(at) => Some(tick.min(at - now)),
            None => Some(tick),
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for TimerText<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<State>();
        let digit = ('0'..='9')
            .map(|digit| self.measure::<iced::Renderer>(&digit.to_string()))
            .fold(0.0_f32, f32::max);

        state.cells = Cells {
            digit,
            colon: self.measure::<iced::Renderer>(":"),
            dot: self.measure::<iced::Renderer>("."),
        };

        let now = state.now.unwrap_or_else(Instant::now);
        let label = self.label(now);

        state.width = state.cells.width(&label);

        let size = limits.resolve(
            Length::Shrink,
            Length::Shrink,
            Size::new(state.width, self.style.line_height),
        );

        layout::Node::new(size)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let now = state.now.unwrap_or_else(Instant::now);

        operation.text(None, layout.bounds(), &self.label(now));
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Event::Window(window::Event::RedrawRequested(now)) = event else {
            return;
        };

        let state = tree.state.downcast_mut::<State>();
        let now = *now;

        state.now = Some(now);
        self.publish(state, now, shell);

        if state.cells.width(&self.label(now)) != state.width {
            shell.invalidate_layout();
        }

        let wait = [
            self.next_change(now).map(|wait| wait + REDRAW_MARGIN),
            self.next_event(now),
        ]
        .into_iter()
        .flatten()
        .min();

        if let Some(wait) = wait {
            shell.request_redraw_at(now + wait);
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let color = self.color.unwrap_or(style.text_color);
        let now = state.now.unwrap_or_else(Instant::now);
        let label = self.label(now);
        let mut x = bounds.x;

        for character in label.chars() {
            let width = state.cells.of(character);
            let cell = Rectangle {
                x,
                y: bounds.y,
                width,
                height: bounds.height,
            };
            let content = character.to_string();

            renderer.fill_text(
                self.text(content.as_str(), Size::new(width + 1.0, bounds.height))
                    .with_content(content.clone()),
                Point::new(cell.center_x(), cell.center_y()),
                color,
                *viewport,
            );

            x += width;
        }
    }
}

impl<'a, Message: 'a> From<TimerText<'a, Message>> for Element<'a, Message> {
    fn from(text: TimerText<'a, Message>) -> Self {
        IcedElement::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_format_like_a_clock() {
        assert_eq!(format_duration(Duration::from_secs(5), false), "0:05");
        assert_eq!(
            format_duration(Duration::from_secs(61 * 60 + 2), false),
            "1:01:02"
        );
        assert_eq!(format_duration(Duration::from_millis(1250), true), "0:01.2");
    }
}
