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

//! The Steps page: linear steps with buttons, vertical and free steps with
//! an error, and display-only steps.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::steps::{self, Axis, Step, step};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("steps.rs");

// #region steps
/// The steps of the sign-up flow.
fn sign_up<'a>() -> [Step<'a, Message>; 3] {
    [
        step("Account")
            .description("E-mail and password")
            .content(text("Create the account.")),
        step("Profile")
            .description("Name and photo")
            .content(text("Tell us about yourself.")),
        step("Review").content(text("Check everything.")),
    ]
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    current: usize,
    free: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current: 1,
            free: 0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A step of the preview was chosen.
    Current(usize),
    /// A step of the free example was chosen.
    Free(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Current(current) => state.current = current,
        Message::Free(free) => state.free = free,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let current = state.current;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["steps", "preview"],
        container(
            // #region preview
            column![
                steps::steps(sign_up(), current).on_select(Message::Current),
                row![
                    button::outlined("Back")
                        .on_press_maybe((current > 0).then(|| Message::Current(current - 1))),
                    button::filled("Next")
                        .on_press_maybe((current < 3).then(|| Message::Current(current + 1))),
                ]
                .spacing(spacing::SM),
            ]
            .spacing(spacing::MD),
            // #endregion
        )
        .width(Length::Fixed(560.0)),
    );

    let vertical = doc::example(
        ctx,
        SOURCE,
        &["vertical"],
        container(
            // #region vertical
            steps::steps(
                sign_up().into_iter().take(2).chain([step("Payment")
                    .description("Card declined")
                    .error(true)
                    .content(text("Try another card."))]),
                state.free,
            )
            .axis(Axis::Vertical)
            .linear(false)
            .on_select(Message::Free),
            // #endregion
        )
        .width(Length::Fixed(360.0)),
    );

    let display = doc::example(
        ctx,
        SOURCE,
        &["display"],
        container(
            // #region display
            steps::steps(sign_up(), 2),
            // #endregion
        )
        .width(Length::Fixed(560.0)),
    );

    let state_text = steps::description(
        "Profile",
        1,
        3,
        steps::state_of(1, current, false),
        &ctx.l10n,
    );

    doc::page(
        "Steps",
        "Shows the progress through a sequence of steps, such as a sign-up form, with the \
         content of the current step.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the steps with `step`, each with a title and an optional `description` \
                 and `content`. `steps` is controlled: pass the current index, and change it \
                 from your own buttons or in the message of `on_select`. The steps before the \
                 current one show as completed with a check mark; pass the number of steps \
                 when all are done.",
            ),
            doc::code(ctx, "use oito2_iced::widget::steps::{self, step};"),
            doc::prose(
                "With `linear`, the default, only the completed steps and the current one can \
                 be clicked, so the user can go back but not skip ahead. In a row, each step \
                 header takes at most an equal share of the width, up to 160 pixels, and long \
                 titles wrap.",
            ),
            doc::heading("Examples"),
            doc::subheading("Vertical and free, with an error"),
            doc::prose(
                "With `axis(Axis::Vertical)` the steps run in a column and the content shows \
                 under the current step. With `linear(false)` any step can be clicked. \
                 `error(true)` marks a step with a cross and the error color.",
            ),
            vertical,
            doc::subheading("Display only"),
            doc::prose(
                "Without `on_select`, the steps only show the progress and can't be clicked or \
                 focused.",
            ),
            display,
            doc::subheading("Describing a step"),
            doc::prose(
                "`description` gives the localized text of a step, with its position and its \
                 `state_of`; for the second step of the preview:",
            ),
            doc::caption(state_text),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to the next step that can be reached.",
                    ],
                    vec!["Enter, Space", "Moves to the focused step."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::steps::steps",
                    "oito2_iced::widget::steps::Steps",
                    "oito2_iced::widget::steps::step",
                    "oito2_iced::widget::steps::Step",
                    "oito2_iced::widget::steps::Axis",
                    "oito2_iced::widget::steps::StepState",
                    "oito2_iced::widget::steps::state_of",
                    "oito2_iced::widget::steps::description",
                ],
            ),
        ],
    )
}
