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

//! The TimerText page: a stopwatch, a countdown and the text that shows
//! them.

use std::time::{Duration, Instant};

use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::timer::{self, Timer, timer_text};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("timer_text.rs");

/// Which timer of the page a message is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// The stopwatch of the preview.
    Stopwatch,
    /// The countdown example.
    Countdown,
}

/// What a button does to a timer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Control {
    /// Starts or resumes the timer.
    Start,
    /// Pauses the timer.
    Pause,
    /// Sets the timer back to zero.
    Reset,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    stopwatch: Timer,
    countdown: Timer,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // #region stopwatch-data
            stopwatch: Timer::stopwatch().with_interval(Duration::from_millis(100)),
            // #endregion
            // #region countdown-data
            countdown: Timer::countdown(Duration::from_secs(10)),
            // #endregion
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A button of a timer was pressed.
    Control(Which, Control),
    /// The countdown reached zero at the instant.
    Completed(Instant),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    // #region update
    match message {
        Message::Control(which, control) => {
            let timer = match which {
                Which::Stopwatch => &mut state.stopwatch,
                Which::Countdown => &mut state.countdown,
            };
            let now = Instant::now();

            match control {
                Control::Start => timer.start(now),
                Control::Pause => timer.pause(now),
                Control::Reset => timer.reset(),
            }
        }
        Message::Completed(at) => {
            if state.countdown.tick(at) {
                shell.toast("Time's up", InfoType::Information);
            }
        }
    }
    // #endregion

    Task::none()
}

// #region controls
/// The time of `timer`, with tenths of a second when `tenths` is set, and
/// its start, pause and reset buttons.
fn controls<'a>(timer: &Timer, which: Which, tenths: bool) -> Element<'a, Message, Theme> {
    let toggle = if timer.is_running() {
        button::filled("Pause").on_press(Message::Control(which, Control::Pause))
    } else {
        button::filled("Start").on_press_maybe(
            (!timer.is_completed()).then_some(Message::Control(which, Control::Start)),
        )
    };

    row![
        container(
            timer_text(timer)
                .style(typography::DISPLAY_SMALL)
                .tenths(tenths)
                .on_complete(Message::Completed),
        )
        .width(Length::Fixed(160.0)),
        toggle,
        button::outlined("Reset").on_press(Message::Control(which, Control::Reset)),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center)
    .into()
}
// #endregion

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["stopwatch-data", "controls", "preview"],
        // #region preview
        controls(&state.stopwatch, Which::Stopwatch, true),
        // #endregion
    );

    let countdown = doc::example(
        ctx,
        SOURCE,
        &["countdown-data", "update", "countdown"],
        // #region countdown
        controls(&state.countdown, Which::Countdown, false),
        // #endregion
    );

    let format = doc::example(
        ctx,
        SOURCE,
        &["format"],
        // #region format
        text(
            [
                timer::format_duration(Duration::from_secs(75), false),
                timer::format_duration(Duration::from_secs(3720), false),
                timer::format_duration(Duration::from_millis(9870), true),
            ]
            .join("  ·  "),
        ),
        // #endregion
    );

    doc::page(
        "TimerText",
        "A stopwatch or a countdown that keeps exact time, and a text that shows it.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`Timer` is a plain value the application owns: a stopwatch from \
                 `Timer::stopwatch`, a countdown from `Timer::countdown`. The application \
                 starts, pauses and resets it from `update`, passing the current `Instant`: \
                 `start` starts or resumes it, `pause` keeps the time run so far, `reset` sets \
                 it back to zero and `restart` resets and starts it. `elapsed`, `remaining`, \
                 `is_running` and `is_completed` describe its state.",
            ),
            doc::prose(
                "Time is measured from those instants, so it stays exact however late the \
                 frames arrive. `with_interval` sets how often the timer ticks, one second by \
                 default.",
            ),
            doc::prose(
                "`timer_text` shows the remaining time of a countdown or the elapsed time of \
                 a stopwatch, with each character in a cell of fixed width so the digits do \
                 not shift; `style` and `color` set its text style. While the timer runs, the \
                 text redraws itself when the shown time changes, so no subscription is \
                 needed. `on_tick` publishes a message with the instant of every tick.",
            ),
            doc::code(ctx, "use oito2_iced::widget::timer::{Timer, timer_text};"),
            doc::heading("Examples"),
            doc::subheading("Countdown"),
            doc::prose(
                "A countdown counts its length down. `on_complete` publishes the instant it \
                 reaches zero; pass it to `Timer::tick`, which stops the finished countdown \
                 and returns `true`. A completed countdown does not start again until `reset` \
                 or `restart`. Without `tenths`, the text rounds up to the whole second, so it \
                 shows 0:00 only at the end.",
            ),
            countdown,
            doc::subheading("Formatting a duration"),
            doc::prose(
                "`format_duration` formats a `Duration` as `m:ss`, or `h:mm:ss` from an hour, \
                 with truncated tenths of a second when `tenths` is set. `timer_label` gives \
                 the text a timer shows at an instant.",
            ),
            format,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::timer::Timer",
                    "oito2_iced::widget::timer::timer_text",
                    "oito2_iced::widget::timer::TimerText",
                    "oito2_iced::widget::timer::format_duration",
                    "oito2_iced::widget::timer::timer_label",
                    "oito2_iced::widget::timer::DEFAULT_INTERVAL",
                ],
            ),
        ],
    )
}
