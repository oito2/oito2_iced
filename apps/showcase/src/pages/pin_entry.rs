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

//! The PinEntry page: a one-time code checked on completion, a masked PIN,
//! letters and a disabled entry.

use iced::widget::column;
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::pin_entry::pin_entry;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("pin_entry.rs");

/// The code the verification example accepts.
const RIGHT_CODE: &str = "123456";

/// The state of the page.
#[derive(Debug)]
pub struct State {
    code: String,
    verify: String,
    status: &'static str,
    error: bool,
    pin: String,
    letters: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            code: String::new(),
            verify: String::new(),
            status: "Type or paste 6 digits",
            error: false,
            pin: String::new(),
            letters: String::new(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The code of the preview changed.
    Code(String),
    /// The code to verify changed.
    Verify(String),
    /// The code to verify was completed.
    Verified(String),
    /// The masked PIN changed.
    Pin(String),
    /// The letters changed.
    Letters(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Code(code) => state.code = code,
        // #region verify-update
        Message::Verify(code) => {
            state.verify = code;
            state.error = false;
        }
        Message::Verified(code) => {
            state.error = code != RIGHT_CODE;
            state.status = if state.error {
                "Wrong code"
            } else {
                "Verified"
            };
        }
        // #endregion
        Message::Pin(pin) => state.pin = pin,
        Message::Letters(letters) => state.letters = letters,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        pin_entry(&state.code)
            .localizations(l10n)
            .on_change(Message::Code),
        // #endregion
    );

    let verify = doc::example(
        ctx,
        SOURCE,
        &["verify", "verify-update"],
        column![
            // #region verify
            pin_entry(&state.verify)
                .error(state.error)
                .localizations(l10n)
                .on_change(Message::Verify)
                .on_complete(Message::Verified),
            // #endregion
            doc::caption(state.status),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let masked = doc::example(
        ctx,
        SOURCE,
        &["masked"],
        // #region masked
        pin_entry(&state.pin)
            .length(4)
            .obscure(true)
            .localizations(l10n)
            .on_change(Message::Pin),
        // #endregion
    );

    let letters = doc::example(
        ctx,
        SOURCE,
        &["letters"],
        // #region letters
        pin_entry(&state.letters)
            .length(5)
            .numeric(false)
            .localizations(l10n)
            .on_change(Message::Letters),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        pin_entry::<Message>("12").length(4).localizations(l10n),
        // #endregion
    );

    doc::page(
        "PinEntry",
        "Enters a short code, such as a PIN or a one-time code, in one cell per character.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `pin_entry` the code and update it from `on_change`. `length` sets the \
                 number of cells, 6 by default. One hidden text input sits under the cells and \
                 holds the whole code, so the entry is a single focus stop: typing fills the \
                 next empty cell, `Backspace` empties the last filled one and a pasted code \
                 fills the cells in order. When the last cell is filled the entry publishes \
                 `on_complete` and gives up the focus.",
            ),
            doc::code(ctx, "use oito2_iced::widget::pin_entry::pin_entry;"),
            doc::heading("Examples"),
            doc::subheading("Checking a code"),
            doc::prose(
                "Check the code in `on_complete` and set `error` to outline the cells in the \
                 error tone. Here 123456 is the right code; editing the code clears the error.",
            ),
            verify,
            doc::subheading("Masked PIN"),
            doc::prose("With `obscure`, each character shows as a bullet."),
            masked,
            doc::subheading("Letters and digits"),
            doc::prose(
                "By default only digits are accepted. With `numeric(false)` any character but \
                 white space is.",
            ),
            letters,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change` the entry ignores input and is skipped by focus traversal.",
            ),
            disabled,
            doc::note(
                "Iced has no accessibility tree: the localized description of the entry, such \
                 as \"Code of 6 characters\" (or `label`), and the character of each cell are \
                 reported as text to widget operations, where test selectors find them.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["A character", "Fills the next empty cell."],
                    vec!["`Backspace`", "Clears the last filled cell."],
                    vec![
                        "`Ctrl+V` / `Cmd+V`",
                        "Pastes a code into the cells in order.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::pin_entry::pin_entry",
                    "oito2_iced::widget::pin_entry::PinEntry",
                    "oito2_iced::widget::pin_entry::DEFAULT_LENGTH",
                ],
            ),
        ],
    )
}
