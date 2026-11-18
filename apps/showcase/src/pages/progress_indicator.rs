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

//! The ProgressIndicator page: linear and circular indicators,
//! determinate and indeterminate.

use iced::widget::{column, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::progress_bar::Class;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::{button, progress, slider};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("progress_indicator.rs");

/// The width of the examples with a linear indicator.
const WIDTH: f32 = 320.0;

/// The state of the page.
#[derive(Debug)]
pub struct State {
    value: f32,
    running: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: 0.4,
            running: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The value slider moved.
    Value(f32),
    /// The indeterminate indicators started or stopped.
    Running(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Value(value) => state.value = value,
        Message::Running(running) => state.running = running,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let value = state.value;
    let running = state.running;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            // #region preview
            progress::circular(0.65),
            progress::linear(0.65),
            // #endregion
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center)
        .width(Length::Fixed(WIDTH)),
    );

    let determinate = doc::example(
        ctx,
        SOURCE,
        &["determinate"],
        column![
            slider::slider(value)
                .label("Value")
                .value_text(|value| format!("{:.0}%", value * 100.0))
                .on_change(Message::Value),
            // #region determinate
            progress::circular(value),
            progress::linear(value),
            // #endregion
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center)
        .width(Length::Fixed(WIDTH)),
    );

    let indeterminate = doc::example(
        ctx,
        SOURCE,
        &["indeterminate"],
        column![
            button::tonal(if running { "Stop" } else { "Start" })
                .on_press(Message::Running(!running)),
            // #region indeterminate
            progress::circular_indeterminate().animated(running),
            progress::linear_indeterminate().animated(running),
            // #endregion
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center)
        .width(Length::Fixed(WIDTH)),
    );

    let style = doc::example(
        ctx,
        SOURCE,
        &["style"],
        column![
            // #region style
            row![
                progress::circular(0.7).size(16.0).stroke_width(2.0),
                progress::circular(0.7).size(48.0).stroke_width(6.0),
            ]
            .spacing(spacing::LG)
            .align_y(Alignment::Center),
            progress::linear(0.7).height(10.0),
            // #endregion
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center)
        .width(Length::Fixed(WIDTH)),
    );

    let classes = doc::example(
        ctx,
        SOURCE,
        &["classes"],
        column![
            // #region classes
            progress::linear(0.8).class(Class::Success),
            progress::linear(0.6).class(Class::Warning),
            progress::linear(0.3).class(Class::Error),
            progress::circular(0.6).class(Class::Success),
            // #endregion
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center)
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "ProgressIndicator",
        "Circular and linear indicators of how far a task has gone, or that it is still running.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`circular` and `linear` take a value from 0 to 1 for determinate \
                 progress; `circular_indeterminate` and `linear_indeterminate` show that a \
                 task is running while the amount is unknown.",
            ),
            doc::code(ctx, "use oito2_iced::widget::progress;"),
            doc::prose(
                "Without a value, the circular indicator spins an arc of changing length \
                 and the linear one sweeps a short pill back and forth. With \
                 `animated(false)` they stand still, for reduced motion.",
            ),
            doc::heading("Examples"),
            doc::subheading("Determinate"),
            doc::prose(
                "The circular ring fills clockwise from the top and the linear bar fills \
                 from the start of the reading direction, set with `direction`. The linear \
                 indicator fills the available width, or 160 pixels when the width is \
                 unbounded. Drag the slider to change the value.",
            ),
            determinate,
            doc::subheading("Indeterminate"),
            doc::prose(
                "An indeterminate indicator asks for the next frame while it is shown. \
                 Here `animated` follows the button, so the indicators move only after \
                 Start.",
            ),
            indeterminate,
            doc::subheading("Size and thickness"),
            doc::prose(
                "`size` sets the circular diameter, 32 pixels by default, and \
                 `stroke_width` the ring thickness; `height` sets the linear bar height, \
                 6 pixels by default, and `width` its width.",
            ),
            style,
            doc::subheading("Colors"),
            doc::prose(
                "The indicators take their colors from the progress bar classes: the \
                 accent by default, or the success, warning and error tones. \
                 `Class::Custom` takes a style closure.",
            ),
            classes,
            doc::note(
                "With the default class, the progress color is the primary color when it \
                 stands out 3:1 from the surfaces, otherwise the indicator tone, over the \
                 highest surface container tone.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::progress::circular",
                    "oito2_iced::widget::progress::circular_indeterminate",
                    "oito2_iced::widget::progress::Circular",
                    "oito2_iced::widget::progress::linear",
                    "oito2_iced::widget::progress::linear_indeterminate",
                    "oito2_iced::widget::progress::Linear",
                    "oito2_iced::theme::progress_bar::Class",
                ],
            ),
        ],
    )
}
