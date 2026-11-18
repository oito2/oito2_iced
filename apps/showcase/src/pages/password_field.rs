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

//! The PasswordField page: signing in, a new password checked as it is
//! typed, an error and a disabled field.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::password::password_field;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("password_field.rs");

/// The width of the example fields.
const WIDTH: f32 = 300.0;

// #region min-length
/// The fewest characters a new password takes.
const MIN_LENGTH: usize = 8;
// #endregion

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    password: String,
    new_password: String,
    wrong: String,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The password changed.
    Password(String),
    /// The password was submitted.
    LogIn,
    /// The new password changed.
    NewPassword(String),
    /// The password of the error example changed.
    Wrong(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Password(password) => state.password = password,
        Message::LogIn => shell.toast("Signed in", InfoType::Success),
        // #endregion
        Message::NewPassword(password) => state.new_password = password,
        Message::Wrong(password) => state.wrong = password,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        container(
            // #region preview
            password_field(l10n, &state.password)
                .label("Password")
                .on_input(Message::Password)
                .on_submit(Message::LogIn),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let new_password = doc::example(
        ctx,
        SOURCE,
        &["min-length", "new-password"],
        container(
            // #region new-password
            password_field(l10n, &state.new_password)
                .label("New password")
                .placeholder("At least 8 characters")
                .error(
                    (!state.new_password.is_empty()
                        && state.new_password.chars().count() < MIN_LENGTH)
                        .then_some("Use at least 8 characters"),
                )
                .on_input(Message::NewPassword),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let error = doc::example(
        ctx,
        SOURCE,
        &["error"],
        container(
            // #region error
            password_field(l10n, &state.wrong)
                .label("Password")
                .error(Some("Wrong password"))
                .on_input(Message::Wrong),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        container(
            // #region disabled
            password_field::<Message>(l10n, "secret").label("Password"),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "PasswordField",
        "A text field for a password, with a button that shows or hides it and a warning while \
         Caps Lock is on.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `password_field` the strings of the current language and the text, and \
                 update the text from `on_input`; `on_submit` publishes a message on `Enter`. \
                 The field hides its text behind bullets until its eye button is pressed; \
                 whether the text shows lives in the field. The button's tooltip, \"Show \
                 password\" or \"Hide password\", comes from the strings of the language.",
            ),
            doc::code(ctx, "use oito2_iced::widget::password::password_field;"),
            doc::heading("Examples"),
            doc::subheading("New password"),
            doc::prose(
                "`error` shows a message below the field and outlines it in the error tone; \
                 `None` clears it. Here the field checks the length of a new password as it is \
                 typed.",
            ),
            new_password,
            doc::subheading("Caps Lock warning"),
            doc::prose(
                "While the field has the focus and Caps Lock is on, a warning shows below it, \
                 under the error when there is one. Turn on Caps Lock while typing in any \
                 field on this page to see it; `caps_lock_warning(false)` hides it.",
            ),
            error,
            doc::note(
                "Iced reports no lock keys, so the field follows Caps Lock from the keys it \
                 sees: a press of Caps Lock toggles it, starting from off, and every typed \
                 letter tells it again, as a capital typed without Shift means Caps Lock is \
                 on.",
            ),
            doc::subheading("Disabled"),
            doc::prose("Without `on_input`, the field and its eye button ignore input."),
            disabled,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::password::password_field",
                    "oito2_iced::widget::password::PasswordField",
                ],
            ),
        ],
    )
}
