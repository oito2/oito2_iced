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

//! The Rating page: whole and half values, more stars, a custom color
//! and read-only.

use iced::widget::column;
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::rating;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("rating.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    whole: f32,
    half: f32,
    score: f32,
    love: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            whole: 3.0,
            half: 2.5,
            score: 7.0,
            love: 2.0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview rating changed.
    Whole(f32),
    /// The half star rating changed.
    Half(f32),
    /// The ten star rating changed.
    Score(f32),
    /// The pink rating changed.
    Love(f32),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Whole(value) => state.whole = value,
        Message::Half(value) => state.half = value,
        Message::Score(value) => state.score = value,
        Message::Love(value) => state.love = value,
    }

    Task::none()
}

/// A rating over its localized value.
fn with_value<'a>(
    ctx: &Context,
    rating: impl Into<Element<'a, Message, Theme>>,
    value: f32,
    max: u32,
) -> Element<'a, Message, Theme> {
    column![
        rating.into(),
        doc::caption(rating::value_text(value, max, &ctx.l10n))
    ]
    .spacing(spacing::SM)
    .align_x(Alignment::Center)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        with_value(
            ctx,
            // #region preview
            rating::rating(state.whole).on_change(Message::Whole),
            // #endregion
            state.whole,
            rating::DEFAULT_MAX,
        ),
    );

    let half = doc::example(
        ctx,
        SOURCE,
        &["half"],
        with_value(
            ctx,
            // #region half
            rating::rating(state.half)
                .allow_half(true)
                .size(32.0)
                .on_change(Message::Half),
            // #endregion
            state.half,
            rating::DEFAULT_MAX,
        ),
    );

    let max = doc::example(
        ctx,
        SOURCE,
        &["max"],
        with_value(
            ctx,
            // #region max
            rating::rating(state.score)
                .max(10)
                .size(20.0)
                .on_change(Message::Score),
            // #endregion
            state.score,
            10,
        ),
    );

    let color = doc::example(
        ctx,
        SOURCE,
        &["color"],
        // #region color
        rating::rating(state.love)
            .color(Accent::Pink.color())
            .on_change(Message::Love),
        // #endregion
    );

    let readonly = doc::example(
        ctx,
        SOURCE,
        &["readonly"],
        // #region readonly
        rating::rating::<Message>(4.0),
        // #endregion
    );

    doc::page(
        "Rating",
        "A row of stars that shows a value and lets the user change it with a click or the \
         keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Pass the current value, from 0 to the number of stars, to `rating` and \
                 update it in the message of `on_change`. Hovering previews the value under \
                 the pointer, and a click picks it and focuses the stars, so the keys work \
                 next. Without `on_change` the rating is read-only.",
            ),
            doc::code(ctx, "use oito2_iced::widget::rating;"),
            doc::prose(
                "`value_text` gives the localized description of a value, such as \"3 of 5 \
                 stars\", shown under the examples.",
            ),
            doc::heading("Examples"),
            doc::subheading("Half stars"),
            doc::prose(
                "With `allow_half(true)`, the value moves in halves: the start half of a \
                 star gives the half value. `size` sets the star size, 24 pixels by default.",
            ),
            half,
            doc::subheading("Number of stars"),
            doc::prose("`max` sets the number of stars, 5 by default."),
            max,
            doc::subheading("Color"),
            doc::prose(
                "`color` sets the color of the filled stars, the theme's warning color by \
                 default.",
            ),
            color,
            doc::subheading("Read-only"),
            doc::prose(
                "Without `on_change`, the rating only shows its value: it takes no focus and \
                 ignores the pointer.",
            ),
            readonly,
            doc::note(
                "A star that fills pops in, and a half star fills from its start; \
                 `animated(false)` shows the final state at once, for reduced motion. \
                 `direction` mirrors the stars, the pointer and the arrow keys for right to \
                 left.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the stars, and out of them."],
                    vec![
                        "Right, Up",
                        "Increases the value by one step; in right to left, Left increases it.",
                    ],
                    vec![
                        "Left, Down",
                        "Decreases the value by one step; in right to left, Right decreases it.",
                    ],
                    vec![
                        "Home, End",
                        "Sets the value to 0 or to the number of stars.",
                    ],
                    vec![
                        "0 to 9",
                        "Sets that whole value, up to the number of stars.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::rating::rating",
                    "oito2_iced::widget::rating::Rating",
                    "oito2_iced::widget::rating::value_text",
                    "oito2_iced::widget::rating::STAR_SIZE",
                    "oito2_iced::widget::rating::DEFAULT_MAX",
                ],
            ),
        ],
    )
}
