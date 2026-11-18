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

//! The ListTile page: rounded rows in a list, square rows in a bordered
//! list and a centered title.

use iced::widget::{column, container, rule};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{self, Icon, lucide};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::list_tile::list_tile;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("list_tile.rs");

// #region folders
/// The folders of the sidebar example, with their icons.
const FOLDERS: [(Icon, &str); 4] = [
    (lucide::HOUSE, "Home"),
    (lucide::FILE_TEXT, "Documents"),
    (lucide::DOWNLOAD, "Downloads"),
    (lucide::AUDIO_LINES, "Music"),
];
// #endregion

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    selected: usize,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A folder was chosen.
    Select(usize),
    /// A settings row was pressed.
    Open(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Select(index) => state.selected = index,
        Message::Open(name) => shell.toast(format!("Opened {name}"), InfoType::Information),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["folders", "preview"],
        column(FOLDERS.iter().enumerate().map(|(index, (glyph, name))| {
            // #region preview
            list_tile()
                .leading(icon::icon(*glyph))
                .title(*name)
                .selected(state.selected == index)
                .on_press(Message::Select(index))
                // #endregion
                .into()
        }))
        .width(Length::Fixed(260.0)),
    );

    let square = doc::example(
        ctx,
        SOURCE,
        &["square"],
        container(
            // #region square
            border_container(column![
                list_tile()
                    .square()
                    .leading(icon::icon(lucide::MONITOR))
                    .title("Displays")
                    .subtitle("Resolution, scale and night light")
                    .trailing(icon::icon(lucide::CHEVRON_RIGHT))
                    .on_press(Message::Open("Displays")),
                rule::horizontal(1),
                list_tile()
                    .square()
                    .leading(icon::icon(lucide::VOLUME_2))
                    .title("Sound")
                    .trailing(icon::icon(lucide::CHEVRON_RIGHT))
                    .on_press(Message::Open("Sound")),
                rule::horizontal(1),
                list_tile()
                    .square()
                    .leading(icon::icon(lucide::PRINTER))
                    .title("Printers")
                    .subtitle("No printers found")
                    .enabled(false),
            ])
            .width(Length::Fill)
            .clip(true),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let centered = doc::example(
        ctx,
        SOURCE,
        &["centered"],
        container(
            // #region centered
            border_container(
                list_tile()
                    .square()
                    .title("Add account")
                    .center_title(true)
                    .on_press(Message::Open("Add account")),
            )
            .width(Length::Fill)
            .clip(true),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    doc::page(
        "ListTile",
        "An interactive row with a title, an optional subtitle and elements on both sides, for \
         sidebars, lists and settings pages.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Build a row with `list_tile` and fill it with `title`, and optionally \
                 `subtitle`, `leading` and `trailing`. With `on_press` the row shows hover, \
                 takes the keyboard focus and publishes its message on a click, `Enter` or \
                 `Space`; without it the row is static. `selected` fills it with the \
                 secondary container color.",
            ),
            doc::code(ctx, "use oito2_iced::widget::list_tile::list_tile;"),
            doc::prose(
                "The row is at least 40 pixels tall, 52 with a subtitle, for a pointer, and 56 \
                 or 72 on a touch screen, following the density when the row is built.",
            ),
            doc::note(
                "For static settings rows whose controls handle their own input, `tile` is \
                 lighter. For a row that toggles a check box, radio or switch, use \
                 `checkbox_list_tile`, `radio_list_tile` or `switch_list_tile`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Square rows in a bordered list"),
            doc::prose(
                "`square` gives the row square corners, for rows inside a bordered list that \
                 already rounds the corners. `enabled(false)` dims a row and ignores presses.",
            ),
            square,
            doc::subheading("Centered title"),
            doc::prose(
                "`center_title` centers the title and subtitle, as in a row that acts as a \
                 single button. `padding` and `spacing` set the space around and between the \
                 parts, and `radius` the corners of the hover, selection and focus shapes.",
            ),
            centered,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next interactive row."],
                    vec!["`Enter` / `Space`", "Publishes the press message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::list_tile::list_tile",
                    "oito2_iced::widget::list_tile::ListTile",
                ],
            ),
        ],
    )
}
