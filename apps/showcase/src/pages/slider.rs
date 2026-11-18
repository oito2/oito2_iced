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

//! The Slider page: labels, steps, marks, ranges and a vertical track.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::widget::slider::{self, mark};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("slider.rs");

/// The width of the horizontal examples.
const WIDTH: f32 = 300.0;

/// The state of the page.
#[derive(Debug)]
pub struct State {
    value: f32,
    volume: f32,
    steps: f32,
    level: f32,
    price: (f32, f32),
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: 30.0,
            volume: 40.0,
            steps: 50.0,
            level: 60.0,
            price: (20.0, 80.0),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview slider moved.
    Value(f32),
    /// The volume slider moved.
    Volume(f32),
    /// The slider with marks moved.
    Steps(f32),
    /// The vertical slider moved.
    Level(f32),
    /// The price range changed.
    Price(f32, f32),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Value(value) => state.value = value,
        Message::Volume(value) => state.volume = value,
        Message::Steps(value) => state.steps = value,
        Message::Level(value) => state.level = value,
        Message::Price(start, end) => state.price = (start, end),
    }

    Task::none()
}

/// `content` at the width of the horizontal examples.
fn sized<'a>(content: impl Into<Element<'a, Message, Theme>>) -> Element<'a, Message, Theme> {
    container(content).width(Length::Fixed(WIDTH)).into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        sized(
            // #region preview
            slider::slider(state.value)
                .range(0.0, 100.0)
                .on_change(Message::Value),
            // #endregion
        ),
    );

    let label = doc::example(
        ctx,
        SOURCE,
        &["label"],
        sized(
            // #region label
            slider::slider(state.volume)
                .range(0.0, 100.0)
                .label("Volume")
                .value_text(|value| format!("{value:.0}%"))
                .show_value_indicator(true)
                .on_change(Message::Volume),
            // #endregion
        ),
    );

    let marks = doc::example(
        ctx,
        SOURCE,
        &["marks"],
        sized(
            // #region marks
            slider::slider(state.steps)
                .range(0.0, 100.0)
                .divisions(4)
                .marks(
                    [0.0, 25.0, 50.0, 75.0, 100.0]
                        .map(|value| mark(value).label(format!("{value:.0}%"))),
                )
                .on_change(Message::Steps),
            // #endregion
        ),
    );

    let range = doc::example(
        ctx,
        SOURCE,
        &["range"],
        sized(
            // #region range
            slider::range_slider(state.price.0, state.price.1)
                .range(0.0, 100.0)
                .label("Price")
                .value_text(|value| format!("${value:.0}"))
                .on_change(Message::Price),
            // #endregion
        ),
    );

    let vertical = doc::example(
        ctx,
        SOURCE,
        &["vertical"],
        // #region vertical
        slider::slider(state.level)
            .range(0.0, 100.0)
            .vertical(true)
            .length(160.0)
            .marks([
                mark(0.0).label("Low"),
                mark(50.0),
                mark(100.0).label("High"),
            ])
            .on_change(Message::Level),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        sized(
            // #region disabled
            slider::slider::<Message>(0.3),
            // #endregion
        ),
    );

    doc::page(
        "Slider",
        "Picks a value, or a range, by dragging a thumb along a track or with the keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`slider` is controlled: give it the value and update it in the message of \
                 `on_change`. Without `on_change` it is disabled. Values run from the minimum \
                 to the maximum of `range`, 0 to 1 by default. The track is inset by the \
                 thumb radius, so the thumb stays inside the slider at both ends.",
            ),
            doc::code(ctx, "use oito2_iced::widget::slider::{self, mark};"),
            doc::prose(
                "`on_change_start` and `on_change_end` publish the value when a drag or a \
                 key press starts and ends a change, for work that should run once per \
                 change rather than on every step.",
            ),
            doc::heading("Examples"),
            doc::subheading("Label and value"),
            doc::prose(
                "A `label` names the slider above the track, and `value_text` formats the \
                 value shown beside it. With `show_value_indicator(true)`, the formatted value \
                 also floats above the thumb while dragging.",
            ),
            label,
            doc::subheading("Steps and marks"),
            doc::prose(
                "`divisions` splits the range into equal steps that the thumb snaps to. \
                 `marks` put a dot, and an optional label, under chosen values.",
            ),
            marks,
            doc::subheading("Range"),
            doc::prose(
                "`range_slider` picks a start and an end with two thumbs, each a stop in the \
                 Tab order; neither passes the other. Its `value_text` formats each end, \
                 shown as \"start – end\".",
            ),
            range,
            doc::subheading("Vertical"),
            doc::prose(
                "With `vertical(true)` the track stands `length` pixels tall with its minimum \
                 at the bottom, and the mark labels sit beside it.",
            ),
            vertical,
            doc::subheading("Disabled"),
            doc::prose("Without `on_change` the slider ignores input and is dimmed."),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the thumb."],
                    vec![
                        "Right, Up",
                        "Raises the value by one division; without divisions, by 5 % of the \
                         range.",
                    ],
                    vec![
                        "Left, Down",
                        "Lowers the value by one division; without divisions, by 5 % of the \
                         range.",
                    ],
                    vec![
                        "Page Up, Page Down",
                        "Move the value by a tenth of the range.",
                    ],
                    vec!["Home, End", "Move the value to the minimum or the maximum."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::slider::slider",
                    "oito2_iced::widget::slider::Slider",
                    "oito2_iced::widget::slider::range_slider",
                    "oito2_iced::widget::slider::RangeSlider",
                    "oito2_iced::widget::slider::mark",
                    "oito2_iced::widget::slider::Mark",
                ],
            ),
        ],
    )
}
