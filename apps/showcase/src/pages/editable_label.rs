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
//! The EditableLabel page: renaming items in place, a heading and a
//! read-only label.

use iced::widget::{column, container};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::typography;
use oito2_iced::widget::editable_label::editable_label;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("editable_label.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    name: String,
    names: Vec<String>,
    title: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: "Holiday photos".to_owned(),
            names: vec![
                "Budget 2026".to_owned(),
                "Reading list".to_owned(),
                String::new(),
            ],
            title: "Quarterly report".to_owned(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview label was renamed.
    Name(String),
    /// A label of the list was renamed.
    Rename(usize, String),
    /// The heading was renamed.
    Title(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Name(name) => state.name = name,
        Message::Rename(index, name) => {
            if let Some(slot) = state.names.get_mut(index) {
                *slot = name;
            }
        }
        Message::Title(title) => state.title = title,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let width = Length::Fixed(320.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            editable_label(&state.name).on_submit(Message::Name),
            // #endregion
        )
        .width(width),
    );

    let placeholder = doc::example(
        ctx,
        SOURCE,
        &["placeholder"],
        container(column(state.names.iter().enumerate().map(
            |(index, name)| {
                // #region placeholder
                editable_label(name)
                    .placeholder("Untitled")
                    .on_submit(move |name| Message::Rename(index, name))
                    // #endregion
                    .into()
            },
        )))
        .width(width),
    );

    let heading = doc::example(
        ctx,
        SOURCE,
        &["heading"],
        container(
            // #region heading
            editable_label(&state.title)
                .style(typography::HEADLINE_SMALL)
                .on_submit(Message::Title),
            // #endregion
        )
        .width(Length::Fixed(400.0)),
    );

    let read_only = doc::example(
        ctx,
        SOURCE,
        &["read-only"],
        container(
            // #region read-only
            editable_label::<Message>("Shared folder"),
            // #endregion
        )
        .width(width),
    );

    doc::page(
        "EditableLabel",
        "A text label that turns into a field to rename something in place.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `editable_label` its text and replace it in `on_submit`. A double \
                 click, or Enter or F2 on the focused label, starts editing with the whole \
                 text selected. Enter or leaving the field keeps the change, and Escape \
                 restores the text. `on_submit` publishes the trimmed text only when it \
                 changed and is not empty.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::editable_label::editable_label;",
            ),
            doc::prose(
                "The text belongs to the application; whether the label is being edited \
                 and the text being typed live in the label, so the application only hears \
                 about the final text.",
            ),
            doc::heading("Examples"),
            doc::subheading("Placeholder"),
            doc::prose(
                "While the text is empty the label shows the `placeholder` in a muted \
                 color. Here the last item has no name yet.",
            ),
            placeholder,
            doc::subheading("As a heading"),
            doc::prose(
                "`style` sets the text style of both the label and the field, body large by \
                 default, so a heading keeps its size while edited.",
            ),
            heading,
            doc::subheading("Read-only"),
            doc::prose(
                "Without `on_submit` the label is read-only: it cannot be edited and Tab \
                 skips it.",
            ),
            read_only,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Enter` / `F2` on the label", "Starts editing."],
                    vec!["`Enter` in the field", "Keeps the change."],
                    vec!["`Escape` in the field", "Restores the text."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::editable_label::editable_label",
                    "oito2_iced::widget::editable_label::EditableLabel",
                ],
            ),
        ],
    )
}
