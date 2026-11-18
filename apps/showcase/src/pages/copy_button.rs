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

//! The CopyButton page: copy buttons and copy fields.

use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::copy;
use oito2_iced::widget::info::InfoType;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("copy_button.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    copies: u32,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The name was copied.
    NameCopied,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::NameCopied => {
            state.copies += 1;
            shell.toast("Name copied", InfoType::Success);
        }
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
        row![
            text("git clone https://github.com/oito2/oito2_iced.git"),
            copy::copy_button(l10n, "git clone https://github.com/oito2/oito2_iced.git"),
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center),
        // #endregion
    );

    let callback = doc::example(
        ctx,
        SOURCE,
        &["callback"],
        // #region callback
        copy::copy_button(l10n, "oito2_iced").on_copied(Message::NameCopied),
        // #endregion
    );

    let field = doc::example(
        ctx,
        SOURCE,
        &["field"],
        container(
            // #region field
            copy::copy_field(l10n, "https://github.com/oito2/oito2_iced").label("Share link"),
            // #endregion
        )
        .width(Length::Fixed(360.0)),
    );

    let obscure = doc::example(
        ctx,
        SOURCE,
        &["obscure"],
        container(
            // #region obscure
            copy::copy_field(l10n, "not-a-real-token-123456")
                .label("API token")
                .obscure(true),
            // #endregion
        )
        .width(Length::Fixed(360.0)),
    );

    let copies = match state.copies {
        0 => "Not copied yet.".to_owned(),
        1 => "Copied once.".to_owned(),
        copies => format!("Copied {copies} times."),
    };

    doc::page(
        "CopyButton",
        "Copies a text to the clipboard and confirms it with a check mark.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `copy_button` the strings of the current language and the text to \
                 copy. After a copy, the icon turns into a check mark and the tooltip into \
                 \"Copied\" for the feedback duration, then both reset.",
            ),
            doc::code(ctx, "use oito2_iced::widget::copy;"),
            doc::heading("Examples"),
            doc::subheading("Reacting to a copy"),
            doc::prose(
                "`on_copied` publishes a message after each copy, for example to show a \
                 toast.",
            ),
            callback,
            doc::caption(copies),
            doc::subheading("Copy field"),
            doc::prose(
                "`copy_field` shows a read-only value, such as a link, with a copy button \
                 at its end. Its text can still be focused, selected and copied with the \
                 keyboard.",
            ),
            field,
            doc::subheading("Secret values"),
            doc::prose(
                "With `obscure`, the field shows bullets instead of the value, which the \
                 button still copies.",
            ),
            obscure,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the button or the field."],
                    vec!["Enter, Space", "Copies the text."],
                ],
            ),
            doc::note(
                "Iced writes to the clipboard without reporting whether the platform \
                 accepted the text, so the button always shows the copied state.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::copy::copy_button",
                    "oito2_iced::widget::copy::CopyButton",
                    "oito2_iced::widget::copy::copy_field",
                    "oito2_iced::widget::copy::CopyField",
                ],
            ),
        ],
    )
}
