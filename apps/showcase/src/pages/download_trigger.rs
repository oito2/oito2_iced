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

//! The DownloadTrigger page: a button that builds a file on demand and
//! saves it through the "Save as" dialog.

use std::time::Duration;

use iced::widget::{column, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::text_field::text_field;
use oito2_iced_files::download::{self, download_trigger};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("download_trigger.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    content: String,
    saving: bool,
    exporting: bool,
    status: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            content: "Hello from oito2_iced_files!".to_owned(),
            saving: false,
            exporting: false,
            status: String::new(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The content changed.
    Content(String),
    /// Saves the content.
    Download,
    /// Builds and saves the CSV report.
    Export,
    /// The CSV report is built.
    Built(Vec<u8>),
    /// Saving ended: saved, canceled or failed.
    Saved(Result<bool, download::Error>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region preview-update
        Message::Download => {
            state.saving = true;
            return download::save("hello.txt", state.content.clone()).map(Message::Saved);
        }
        Message::Saved(result) => {
            state.saving = false;
            state.exporting = false;
            state.status = match result {
                Ok(true) => "Saved.".to_owned(),
                Ok(false) => "Canceled.".to_owned(),
                Err(error) => error.to_string(),
            };
        }
        // #endregion
        // #region slow-update
        Message::Export => {
            state.exporting = true;
            return Task::perform(
                async {
                    std::thread::sleep(Duration::from_secs(2));
                    b"name,size\nalpha,1\nbeta,2\n".to_vec()
                },
                Message::Built,
            );
        }
        Message::Built(bytes) => {
            return download::save("report.csv", bytes).map(Message::Saved);
        }
        // #endregion
        Message::Content(content) => state.content = content,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "preview-update"],
        column![
            text_field("Content", &state.content)
                .label("Content")
                .on_input(Message::Content),
            // #region preview
            download_trigger(l10n)
                .busy(state.saving)
                .on_press(Message::Download),
            // #endregion
        ]
        .push((!state.status.is_empty()).then(|| text(&state.status).class(TextClass::Variant)))
        .spacing(spacing::SM)
        .align_x(Alignment::Center)
        .width(Length::Fixed(320.0)),
    );

    let slow = doc::example(
        ctx,
        SOURCE,
        &["slow", "slow-update"],
        // #region slow
        download_trigger(l10n)
            .label("Export CSV")
            .busy(state.exporting)
            .on_press(Message::Export),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        download_trigger::<Message>(l10n),
        // #endregion
    );

    let support = if download::is_supported() {
        "This platform can save files."
    } else {
        "Saving files is not available on this platform."
    };

    doc::page(
        "DownloadTrigger",
        "A button that builds a file on demand and saves it through the \"Save as\" dialog.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`download_trigger` is an outlined button with a download icon and the \
                 localized \"Download\" label. Its `on_press` message asks the application to \
                 save: `download::save` takes the suggested file name and the bytes, opens the \
                 \"Save as\" dialog and writes the file, as a `Task` the application returns \
                 from `update`. The task yields `Ok(true)` once the file is written, \
                 `Ok(false)` when the dialog is canceled, and an `Error` when saving is not \
                 supported or writing fails.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_files::download::{self, download_trigger};",
            ),
            doc::note(
                "Saving works on Linux and the other free Unix systems, Windows and macOS. \
                 `download::is_supported` tells which; where it is false the trigger is \
                 disabled and its tooltip says why. To save bytes without a button, return \
                 `download::save` from any update.",
            ),
            doc::caption(support),
            doc::heading("Examples"),
            doc::subheading("Slow data and a custom label"),
            doc::prose(
                "While the file is built or saved, `busy(true)` turns the icon into a spinner \
                 and the button ignores presses. `label` replaces the localized \"Download\" \
                 text.",
            ),
            slow,
            doc::subheading("Disabled"),
            doc::prose("Without `on_press` the button is disabled and skipped by focus traversal."),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the button."],
                    vec!["Enter, Space", "Publishes the `on_press` message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_files::download::download_trigger",
                    "oito2_iced_files::download::DownloadTrigger",
                    "oito2_iced_files::download::save",
                    "oito2_iced_files::download::is_supported",
                    "oito2_iced_files::download::Error",
                ],
            ),
        ],
    )
}
