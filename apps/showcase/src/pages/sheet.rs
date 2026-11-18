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

//! The Sheet page: the end and bottom sheets of the showcase, with the
//! code that builds them.

use iced::widget::row;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("sheet.rs");

/// The code that shows a sheet over the view of an application.
const HOST: &str = r#"fn view(&self) -> Element<'_, Message> {
    let page = self.page();
    let sheet = self.details_open.then(|| {
        let content = sheet::content(text("The properties of the file."))
            .title("Details")
            .on_close(Message::CloseDetails)
            .localizations(&self.localizations);

        sheet::sheet(content).on_dismiss(Message::CloseDetails)
    });

    focus::navigation(sheet::host(page, sheet)).into()
}"#;

/// The code of a bottom sheet with two resting heights.
const SNAP: &str = r#"sheet::sheet(content)
    .side(sheet::Side::Bottom)
    .snap_points([0.35, 0.8], self.snap, Message::Snap)
    .on_dismiss(Message::CloseSheet)"#;

/// The code of a non-modal sheet at the start edge.
const NON_MODAL: &str = r#"sheet::sheet(content)
    .side(sheet::Side::Start)
    .modal(false)
    .size(320.0)
    .on_dismiss(Message::CloseSheet)"#;

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A sheet should open: at the bottom when `true`, at the end
    /// otherwise.
    Open(bool),
}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Open(bottom) => shell.request(Action::OpenSheet(bottom)),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        button::outlined("Open sheet").on_press(Message::Open(false)),
        // #endregion
    );

    let sides = doc::example(
        ctx,
        SOURCE,
        &["sides"],
        // #region sides
        row![
            button::outlined("End").on_press(Message::Open(false)),
            button::outlined("Bottom, 35 % and 80 %").on_press(Message::Open(true)),
        ]
        .spacing(spacing::SM),
        // #endregion
    );

    doc::page(
        "Sheet",
        "A panel that slides in from an edge of the window, for details, filters or a \
         secondary task.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`sheet::host` wraps the whole view of the application and shows the sheet it \
                 is given against its edge; the application owns whether the sheet is open \
                 and passes `None` to close it. `sheet::content` lays out the usual content: a \
                 header with the `title`, optional actions and a close button, above the body. \
                 `on_dismiss` publishes the message that closes the sheet.",
            ),
            doc::code(ctx, "use oito2_iced::widget::sheet;"),
            doc::code(ctx, HOST),
            doc::prose(
                "Wrap the host in `focus::navigation`, so Tab also reaches a non-modal sheet. \
                 A sheet that opens after the host was created slides in from its edge; one \
                 shown from the start is drawn in place.",
            ),
            doc::note(
                "For a short question that needs an answer before going on, use a dialog; a \
                 sheet suits longer content kept beside the page.",
            ),
            doc::heading("Examples"),
            doc::subheading("Sides"),
            doc::prose(
                "`side` picks the edge: `Side::Start` and `Side::End` follow the reading \
                 direction, and `Side::Bottom` slides up. A side sheet is `size` wide, 360 \
                 pixels by default, and always leaves 48 pixels of the window uncovered. A \
                 bottom sheet is as tall as its content, up to 90 % of the window, and at \
                 most 640 pixels wide. The buttons open the sheets of this showcase.",
            ),
            sides,
            doc::subheading("Resting heights"),
            doc::prose(
                "`snap_points` gives a bottom sheet the heights it rests at, as ascending \
                 fractions of the window height, the index of the current one and the \
                 message that moves it to another. Its handle then moves between them by \
                 drag, by the arrow keys, or by a click, Enter or Space, which cycle to the \
                 next height. Side sheets ignore them.",
            ),
            doc::code(ctx, SNAP),
            doc::subheading("Non-modal"),
            doc::prose(
                "A modal sheet, the default, dims the view below over a scrim and keeps the \
                 input: Tab cycles the focus inside it. With `modal(false)` the view below \
                 stays usable beside the sheet, and Escape closes it while the focus is \
                 inside.",
            ),
            doc::code(ctx, NON_MODAL),
            doc::prose(
                "With `dismissible(false)`, a press on the scrim and a drag toward the edge no \
                 longer publish the dismiss message; Escape and the close button still do. \
                 `reduce_motion(true)` skips the slide in and the settling animations.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Escape", "Closes the sheet."],
                    vec!["Tab, Shift+Tab", "Cycle the focus inside a modal sheet."],
                    vec![
                        "Up, Down",
                        "On the handle of a bottom sheet with resting heights, move to the \
                         next taller or shorter height.",
                    ],
                    vec!["Enter, Space", "On that handle, cycle to the next height."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::sheet::host",
                    "oito2_iced::widget::sheet::sheet",
                    "oito2_iced::widget::sheet::Sheet",
                    "oito2_iced::widget::sheet::Side",
                    "oito2_iced::widget::sheet::content",
                    "oito2_iced::widget::sheet::Content",
                    "oito2_iced::widget::sheet::SIDE_WIDTH",
                    "oito2_iced::widget::sheet::BOTTOM_MAX_WIDTH",
                ],
            ),
        ],
    )
}
