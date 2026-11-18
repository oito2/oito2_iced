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

//! The SearchField page: search fields, a filtered list and the search
//! title field.

use iced::widget::column;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::highlighted_text::{self, highlighted_text};
use oito2_iced::widget::search;
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("search_field.rs");

/// The width of the examples.
const WIDTH: f32 = 300.0;

/// The rows the filter example searches.
const RESULTS: [&str; 5] = [
    "Configurações de ação",
    "Show hidden files",
    "Network settings",
    "Quarterly report",
    "Invoices 2026",
];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    query: String,
    messages: String,
    submitted: String,
    filter: String,
    searching: bool,
    files: String,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview query changed.
    Query(String),
    /// The message search changed.
    Messages(String),
    /// The message search was submitted.
    Submit,
    /// The filter changed.
    Filter(String),
    /// The title search opened or closed.
    Searching(bool),
    /// The title search changed.
    Files(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Query(query) => state.query = query,
        Message::Messages(query) => state.messages = query,
        Message::Submit => state.submitted = state.messages.clone(),
        Message::Filter(filter) => state.filter = filter,
        Message::Searching(searching) => {
            state.searching = searching;
            if !searching {
                state.files.clear();
            }
        }
        Message::Files(query) => state.files = query,
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
        column![
            // #region preview
            search::search_field(l10n, &state.query).on_input(Message::Query),
            // #endregion
            doc::caption(format!("Query: \"{}\"", state.query)),
        ]
        .spacing(spacing::MD)
        .width(Length::Fixed(WIDTH)),
    );

    let submit = doc::example(
        ctx,
        SOURCE,
        &["submit"],
        column![
            // #region submit
            search::search_field(l10n, &state.messages)
                .placeholder("Search messages")
                .on_input(Message::Messages)
                .on_submit(Message::Submit),
            // #endregion
            doc::caption(format!("Submitted: \"{}\"", state.submitted)),
        ]
        .spacing(spacing::MD)
        .width(Length::Fixed(WIDTH)),
    );

    let filter = doc::example(
        ctx,
        SOURCE,
        &["filter"],
        // #region filter
        column![
            search::search_field(l10n, &state.filter).on_input(Message::Filter),
            column(
                RESULTS
                    .into_iter()
                    .filter(|result| {
                        state.filter.trim().is_empty()
                            || !highlighted_text::matches(
                                result,
                                &state.filter,
                                &[],
                                Default::default(),
                            )
                            .is_empty()
                    })
                    .map(|result| highlighted_text(result, &state.filter).into()),
            )
            .spacing(spacing::XS),
        ]
        .spacing(spacing::MD)
        .width(Length::Fixed(WIDTH)),
        // #endregion
    );

    let title = doc::example(
        ctx,
        SOURCE,
        &["title"],
        column![
            // #region title
            search::search_title_field(
                l10n,
                styled_text("Files", typography::TITLE_MEDIUM),
                &state.files,
                state.searching,
                Message::Searching,
            )
            .on_input(Message::Files),
            // #endregion
            doc::caption(format!("Query: \"{}\"", state.files)),
        ]
        .spacing(spacing::MD),
    );

    doc::page(
        "SearchField",
        "A text field for a search, with a search icon and a clear button, and a title that \
         turns into one.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`search_field` shows the value it is given and publishes each edit through \
                 `on_input`; without it the field is disabled. The clear button shows while \
                 there is text: it publishes the empty text to `on_input` and then the \
                 message of `on_clear`. The hint defaults to the localized \"Search\".",
            ),
            doc::code(ctx, "use oito2_iced::widget::search;"),
            doc::heading("Examples"),
            doc::subheading("Hint and submit"),
            doc::prose(
                "`placeholder` replaces the default hint, and `on_submit` publishes its \
                 message on Enter.",
            ),
            submit,
            doc::subheading("Filtering a list"),
            doc::prose(
                "With `highlighted_text`, the rows that match the query show the matched \
                 parts in bold; the match ignores case and accents.",
            ),
            filter,
            doc::subheading("Search in a title"),
            doc::prose(
                "`search_title_field` switches between a title and a search field with a \
                 toggle button, as in a header bar. It is controlled: `active` says whether \
                 the field shows, and the closure after it turns each request to open or \
                 close it into a message. It keeps the same size in both states, \
                 `TITLE_FIELD_WIDTH` wide unless set with `width`, so the layout around it \
                 doesn't move. Opening focuses the field; closing returns the focus to the \
                 toggle.",
            ),
            title,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Escape with text",
                        "Clears the field and keeps the focus in it.",
                    ],
                    vec![
                        "Escape in the empty field",
                        "Publishes `on_escape`; in `search_title_field`, closes the search.",
                    ],
                    vec!["Enter", "Publishes `on_submit`."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::search::search_field",
                    "oito2_iced::widget::search::SearchField",
                    "oito2_iced::widget::search::search_title_field",
                    "oito2_iced::widget::search::SearchTitleField",
                    "oito2_iced::widget::search::TITLE_FIELD_WIDTH",
                ],
            ),
        ],
    )
}
