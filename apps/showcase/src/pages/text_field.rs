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

//! The TextField page: labels, helper and error text, icons, secure
//! fields and the text input classes.

use iced::widget::{column, container, text_input};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::theme::text_input::Class as TextInputClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::text_field::text_field;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("text_field.rs");

/// The width of the examples.
const WIDTH: f32 = 360.0;

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    name: String,
    email: String,
    city: String,
    password: String,
    submitted: Option<String>,
    raw: String,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The name changed.
    Name(String),
    /// The e-mail address changed.
    Email(String),
    /// The city changed.
    City(String),
    /// The city was cleared.
    ClearCity,
    /// The password changed.
    Password(String),
    /// The name was submitted with Enter.
    Submit,
    /// The plain text input changed.
    Raw(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Name(name) => state.name = name,
        Message::Email(email) => state.email = email,
        Message::City(city) => state.city = city,
        Message::ClearCity => state.city.clear(),
        Message::Password(password) => state.password = password,
        Message::Submit => state.submitted = Some(state.name.clone()),
        Message::Raw(raw) => state.raw = raw,
    }

    Task::none()
}

/// `content` at the width of the examples.
fn sized<'a>(content: impl Into<Element<'a, Message, Theme>>) -> Element<'a, Message, Theme> {
    container(content).width(Length::Fixed(WIDTH)).into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let submitted = match &state.submitted {
        Some(name) => format!("Submitted: \"{name}\""),
        None => "Press Enter in the field to submit it.".to_owned(),
    };

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        sized(
            column![
                // #region preview
                text_field("Your name", &state.name)
                    .label("Name")
                    .helper("As shown to other people")
                    .on_input(Message::Name)
                    .on_submit(Message::Submit),
                // #endregion
                doc::caption(submitted),
            ]
            .spacing(spacing::MD),
        ),
    );

    // #region validation
    let email_error = (!state.email.is_empty() && !state.email.contains('@'))
        .then_some("An e-mail address has an @");
    // #endregion

    let error = doc::example(
        ctx,
        SOURCE,
        &["validation", "error"],
        sized(
            // #region error
            text_field("name@example.com", &state.email)
                .label("E-mail")
                .leading(lucide::MAIL)
                .error(email_error)
                .on_input(Message::Email),
            // #endregion
        ),
    );

    let trailing = doc::example(
        ctx,
        SOURCE,
        &["trailing"],
        sized(
            // #region trailing
            text_field("City", &state.city)
                .leading(lucide::SEARCH)
                .trailing(
                    button::icon_button(lucide::X)
                        .tooltip("Clear")
                        .on_press_maybe((!state.city.is_empty()).then_some(Message::ClearCity)),
                )
                .on_input(Message::City),
            // #endregion
        ),
    );

    let secure = doc::example(
        ctx,
        SOURCE,
        &["secure"],
        sized(
            // #region secure
            text_field("Password", &state.password)
                .label("Password")
                .secure(true)
                .on_input(Message::Password),
            // #endregion
        ),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        sized(
            // #region disabled
            text_field::<Message>("Read only", "Not editable").label("Disabled"),
            // #endregion
        ),
    );

    let classes = doc::example(
        ctx,
        SOURCE,
        &["classes"],
        sized(
            // #region classes
            column![
                text_input("Default class", &state.raw).on_input(Message::Raw),
                text_input("Error class", &state.raw)
                    .class(TextInputClass::Error)
                    .on_input(Message::Raw),
            ]
            .spacing(spacing::MD),
            // #endregion
        ),
    );

    doc::page(
        "TextField",
        "A single-line input with an optional label, icons, and helper or error text.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`text_field` takes the placeholder and the value; the value lives in the \
                 application and each edit reaches it through `on_input`. Without `on_input` \
                 the field is disabled and skipped by focus traversal. `label` shows a label \
                 above the field and `helper` a hint below it; `on_submit` publishes its \
                 message on Enter and `on_paste` takes pasted text apart from typing.",
            ),
            doc::code(ctx, "use oito2_iced::widget::text_field::text_field;"),
            doc::prose(
                "The field is as tall as the control height of the density, the height of \
                 the buttons, with or without icons. It is filled with the high surface \
                 container and outlined; the outline thickens in the text-safe primary tone \
                 while focused.",
            ),
            doc::note(
                "The search, password, number and copy fields of `oito2_iced` are built on \
                 the text field and share its look.",
            ),
            doc::heading("Examples"),
            doc::subheading("Leading icon and error"),
            doc::prose(
                "`leading` places an icon at the start of the field. `error` shows its text \
                 in place of the helper and outlines the field in the error tone; `None` \
                 clears it. Type a text without an @ to see it.",
            ),
            error,
            doc::subheading("Trailing element"),
            doc::prose(
                "`trailing` places any element, such as an icon button, in a square box at \
                 the end of the field.",
            ),
            trailing,
            doc::subheading("Secure"),
            doc::prose("`secure(true)` hides the text behind bullets."),
            secure,
            doc::subheading("Disabled"),
            doc::prose("Without `on_input` the field shows its value dimmed and can't be edited."),
            disabled,
            doc::subheading("Text input classes"),
            doc::prose(
                "A plain Iced `text_input` takes the same look from the OITO2 theme. Its \
                 class is `Class::Default`, or `Class::Error` for the error outline; \
                 `Class::Custom` takes a style closure.",
            ),
            classes,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the field, and out of it."],
                    vec!["Enter", "Publishes `on_submit`."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::text_field::text_field",
                    "oito2_iced::widget::text_field::TextField",
                    "oito2_iced::theme::text_input::Class",
                ],
            ),
        ],
    )
}
