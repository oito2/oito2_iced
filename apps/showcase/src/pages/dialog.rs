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
//! The Dialog page: modal dialogs over a scrim, with a title, a title bar
//! and actions, shown by a host or drawn in place.

use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::{button, dialog};

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("dialog.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    confirm_open: bool,
    deleted: u32,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The example dialog of the window should open.
    OpenWindowDialog,
    /// The confirmation dialog of the local host should open.
    OpenConfirm,
    /// The confirmation dialog was dismissed or canceled.
    CloseConfirm,
    /// The file was deleted from the confirmation dialog.
    Delete,
    /// A button of a dialog drawn in place was pressed.
    Pressed,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::OpenWindowDialog => shell.request(Action::OpenDialog),
        Message::OpenConfirm => state.confirm_open = true,
        Message::CloseConfirm => state.confirm_open = false,
        Message::Delete => {
            state.confirm_open = false;
            state.deleted += 1;
        }
        Message::Pressed => {}
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            container(
                // #region preview
                dialog::dialog(text("Discard the changes made to this document?"))
                    .title("Discard changes?")
                    .action(button::text("Cancel").on_press(Message::Pressed))
                    .action(button::danger("Discard").on_press(Message::Pressed))
                    .on_dismiss(Message::Pressed),
                // #endregion
            )
            .width(Length::Fixed(400.0)),
            button::outlined("Open as a modal dialog").on_press(Message::OpenWindowDialog),
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center),
    );

    let deleted = match state.deleted {
        0 => "Nothing deleted yet.".to_owned(),
        1 => "Deleted once.".to_owned(),
        deleted => format!("Deleted {deleted} times."),
    };

    let window = border_container(
        container(
            column![
                text("notes.txt"),
                button::outlined("Delete…").on_press(Message::OpenConfirm),
                doc::caption(deleted),
            ]
            .spacing(spacing::MD)
            .align_x(Alignment::Center),
        )
        .center(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(260.0))
    .padding(spacing::LG);

    let local = doc::example(
        ctx,
        SOURCE,
        &["host"],
        container(
            // #region host
            dialog::host(
                window,
                state.confirm_open.then(|| {
                    dialog::dialog(text("The file will be deleted for good."))
                        .title("Delete notes.txt?")
                        .action(button::text("Cancel").on_press(Message::CloseConfirm))
                        .action(button::danger("Delete").on_press(Message::Delete))
                        .on_dismiss(Message::CloseConfirm)
                }),
            ),
            // #endregion
        )
        .width(Length::Fixed(520.0)),
    );

    let title_bar = doc::example(
        ctx,
        SOURCE,
        &["title-bar"],
        container(
            // #region title-bar
            dialog::dialog(text("Changes are saved automatically."))
                .title_bar(
                    dialog::title_bar()
                        .title("Preferences")
                        .leading(icon(lucide::SETTINGS)),
                )
                .action(button::filled("Done").on_press(Message::Pressed))
                .on_dismiss(Message::Pressed),
            // #endregion
        )
        .width(Length::Fixed(400.0)),
    );

    let width = doc::example(
        ctx,
        SOURCE,
        &["width"],
        // #region width
        dialog::dialog(text(
            "A dialog drawn in place fills the width it is given, up to its largest width.",
        ))
        .title("Narrow dialog")
        .max_width(320.0)
        .action(button::text("OK").on_press(Message::Pressed)),
        // #endregion
    );

    doc::page(
        "Dialog",
        "A surface centered over a scrim that keeps the input, with an optional title bar, \
         a title, the content and a row of actions.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`dialog::dialog` builds the dialog: the content, a `title` in the title \
                 large style, and `action`s, usually buttons, at the end of the last row. \
                 It is drawn on the dialog surface of the theme, with the window radius, a \
                 thin border and a shadow.",
            ),
            doc::prose(
                "`dialog::host` wraps the application view and shows the dialog it is given \
                 above it, centered over a scrim; the application owns whether the dialog \
                 is open and passes `None` to close it. Wrap the whole view, inside the \
                 focus navigation:",
            ),
            doc::code(
                ctx,
                "focus::navigation(dialog::host(\n    \
                     view,\n    \
                     self.open.then(|| {\n        \
                         dialog::dialog(text(\"Changes are saved automatically.\"))\n            \
                             .title(\"Settings\")\n            \
                             .action(button::filled(\"Close\").on_press(Message::Close))\n            \
                             .on_dismiss(Message::Close)\n    \
                     }),\n\
                 ))",
            ),
            doc::prose(
                "While the dialog is open, the view below receives no input, Tab and \
                 Shift+Tab cycle the focus inside the dialog, and Escape publishes the \
                 `on_dismiss` message. A press on the scrim publishes it too, unless \
                 `dismissible(false)`.",
            ),
            doc::heading("Examples"),
            doc::subheading("A host of its own"),
            doc::prose(
                "The host centers the dialog over its own bounds, so it can also cover only \
                 a part of the window, such as a pane. This one covers the area below.",
            ),
            local,
            doc::subheading("Title bar"),
            doc::prose(
                "`title_bar` puts a dialog title bar at the top: a title, an optional \
                 leading element and a close button that publishes the dismiss message. \
                 The DialogTitleBar page shows its options.",
            ),
            title_bar,
            doc::subheading("Width"),
            doc::prose(
                "A dialog shown by the host is as wide as its content and at least \
                 `MIN_WIDTH`, 280 pixels; one drawn in place fills the width it is given. \
                 `width` and `max_width` set them, and the host keeps `INSET` free between \
                 the dialog and the window edges.",
            ),
            width,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab` / `Shift+Tab`",
                        "Cycle the focus inside the open dialog.",
                    ],
                    vec!["`Escape`", "Publishes the dismiss message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::dialog::dialog",
                    "oito2_iced::widget::dialog::Dialog",
                    "oito2_iced::widget::dialog::host",
                    "oito2_iced::widget::dialog::title_bar",
                    "oito2_iced::widget::dialog::MIN_WIDTH",
                    "oito2_iced::widget::dialog::INSET",
                    "oito2_iced::widget::dialog::SCRIM_OPACITY",
                ],
            ),
        ],
    )
}
