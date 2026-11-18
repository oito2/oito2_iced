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

//! The SegmentedEntry page: an IPv4 address and a license key.

use iced::widget::column;
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::entries::{segment, segmented_entry};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("segmented_entry.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    address: Vec<String>,
    key: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            address: Vec::new(),
            key: vec!["ABCD".to_owned(), "1234".to_owned()],
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A part of the address changed.
    Address(Vec<String>),
    /// A part of the license key changed.
    Key(Vec<String>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Address(parts) => state.address = parts,
        Message::Key(parts) => state.key = parts,
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
            // #region preview
            segmented_entry(
                [
                    segment(3).hint("192"),
                    segment(3).hint("168"),
                    segment(3).hint("0"),
                    segment(3).hint("1"),
                ],
                &state.address,
            )
            .on_change(Message::Address),
            // #endregion
            doc::caption(format!("Address: {}", state.address.join("."))),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let license = doc::example(
        ctx,
        SOURCE,
        &["license"],
        column![
            // #region license
            segmented_entry(
                [
                    segment(4).numeric(false),
                    segment(4).numeric(false),
                    segment(4).numeric(false),
                    segment(4).numeric(false),
                ],
                &state.key,
            )
            .delimiter(" - ")
            .on_change(Message::Key),
            // #endregion
            doc::caption(format!("Key: {}", state.key.join("-"))),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    doc::page(
        "SegmentedEntry",
        "Enters a value made of short fixed parts, such as an IP address or a license key, \
         in a row of small fields.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Describe each part with a `segment`: its length in characters, an optional \
                 `hint` placeholder and whether it is `numeric`, the default, which accepts \
                 digits only. The values live in the application: `segmented_entry` shows \
                 them, missing ones empty, and `on_change` receives the text of every segment \
                 whenever one changes. Without `on_change` the entry is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::entries::{segment, segmented_entry};",
            ),
            doc::prose(
                "The focus moves to the next segment when one fills up or when the delimiter \
                 is typed, so a value can be typed in one go. Each field is as wide as its \
                 number of digits and as tall as the text fields.",
            ),
            doc::heading("Examples"),
            doc::subheading("Delimiter and letters"),
            doc::prose(
                "`delimiter` is the text drawn between the segments, `.` by default. Typing \
                 its first visible character, `-` for `\" - \"`, moves to the next segment, and \
                 a segment that isn't `numeric` refuses that character but accepts any other.",
            ),
            license,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["The delimiter", "Moves the focus to the next segment."],
                    vec![
                        "Backspace in an empty segment",
                        "Moves the focus to the end of the previous segment.",
                    ],
                    vec![
                        "Left at the start of a segment",
                        "Moves the focus to the end of the previous segment (Right in right to \
                         left).",
                    ],
                    vec![
                        "Right at the end of a segment",
                        "Moves the focus to the start of the next segment (Left in right to \
                         left).",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::entries::segmented_entry",
                    "oito2_iced::widget::entries::SegmentedEntry",
                    "oito2_iced::widget::entries::segment",
                    "oito2_iced::widget::entries::Segment",
                ],
            ),
        ],
    )
}
