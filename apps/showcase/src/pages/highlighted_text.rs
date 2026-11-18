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

//! The HighlightedText page: matching a query as it is typed, the matching
//! options and the style of the matches.

use iced::font::Weight;
use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::controls::check_button;
use oito2_iced::widget::highlighted_text::highlighted_text;
use oito2_iced::widget::search::search_field;
use oito2_iced::widget::toggles::checkbox;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("highlighted_text.rs");

// #region lines
/// The lines the search example matches.
const LINES: [&str; 4] = [
    "Configurações de ação",
    "Confirm the action before deleting",
    "Ações rápidas e acessibilidade",
    "Keyboard configuration",
];
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    query: String,
    ignore_case: bool,
    whole_words: bool,
    match_all: bool,
    split_words: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            query: "acao conf".to_owned(),
            ignore_case: true,
            whole_words: false,
            match_all: true,
            split_words: true,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The query changed.
    Query(String),
    /// The ignore case option changed.
    IgnoreCase(bool),
    /// The whole words option changed.
    WholeWords(bool),
    /// The every occurrence option changed.
    MatchAll(bool),
    /// The each word apart option changed.
    SplitWords(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Query(query) => state.query = query,
        Message::IgnoreCase(on) => state.ignore_case = on,
        Message::WholeWords(on) => state.whole_words = on,
        Message::MatchAll(on) => state.match_all = on,
        Message::SplitWords(on) => state.split_words = on,
    }

    Task::none()
}

/// A check button for a matching option.
fn option<'a>(
    value: bool,
    title: &'a str,
    on_change: fn(bool) -> Message,
) -> Element<'a, Message, Theme> {
    check_button(
        checkbox(value).on_change(move |value| on_change(value.unwrap_or_default())),
        title,
    )
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["lines", "preview"],
        column![
            // #region preview
            search_field(l10n, &state.query).on_input(Message::Query),
            column(
                LINES
                    .iter()
                    .map(|line| highlighted_text(*line, &state.query).into())
            )
            .spacing(spacing::SM),
            // #endregion
        ]
        .spacing(spacing::MD)
        .width(Length::Fixed(320.0)),
    );

    let options = doc::example(
        ctx,
        SOURCE,
        &["options"],
        column![
            row![
                option(
                    state.ignore_case,
                    "Ignore case and accents",
                    Message::IgnoreCase
                ),
                option(state.whole_words, "Whole words only", Message::WholeWords),
            ],
            row![
                option(state.match_all, "Every occurrence", Message::MatchAll),
                option(state.split_words, "Each word apart", Message::SplitWords),
            ],
            // #region options
            highlighted_text(
                "Ark UI builds UI kits; the ark sails past the UI kit shop.",
                "ark",
            )
            .queries(["ui", "kit"])
            .ignore_case(state.ignore_case)
            .whole_words(state.whole_words)
            .match_all(state.match_all)
            .split_words(state.split_words),
            // #endregion
        ]
        .spacing(spacing::MD)
        .align_x(iced::Alignment::Center)
        .width(Length::Fixed(440.0)),
    );

    let style = doc::example(
        ctx,
        SOURCE,
        &["style"],
        container(
            // #region style
            highlighted_text("Keyboard shortcuts and keyboard navigation", "keyboard")
                .style(typography::TITLE_MEDIUM)
                .class(TextClass::Variant)
                .highlight_weight(Weight::Bold)
                .max_lines(1),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    doc::page(
        "HighlightedText",
        "Text with the matches of a search query emphasized, such as search results that show \
         why each item matched.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `highlighted_text` the full text and the query. By default each word of \
                 the query matches on its own, everywhere in the text, ignoring case and \
                 accents: \"acao\" matches \"Ação\". Overlapping and touching matches merge. \
                 The matches are drawn bold in the text-safe primary tone, in the body medium \
                 style.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::highlighted_text::highlighted_text;",
            ),
            doc::prose(
                "`highlighted_text::matches` returns the byte ranges of a text that match, \
                 with the same `Options`, for drawing the highlight another way or for \
                 filtering a list before showing it.",
            ),
            doc::heading("Examples"),
            doc::subheading("Matching options"),
            doc::prose(
                "`queries` highlights more queries along with the query. `ignore_case(false)` \
                 makes matches keep their case and accents, `whole_words` matches only whole \
                 words, `match_all(false)` highlights only the first occurrence of each term, \
                 and `split_words(false)` matches each query as a whole phrase. `options` sets \
                 them all at once. Here the queries are \"ark\", \"ui\" and \"kit\".",
            ),
            options,
            doc::subheading("Highlight style"),
            doc::prose(
                "`style` sets the type scale style of the whole text and `class` the color of \
                 the text outside the matches. `highlight_weight` and `highlight_color` change \
                 how the matches are drawn. `max_lines` clips the text after a number of \
                 lines.",
            ),
            style,
            doc::note(
                "Iced text cannot end with an ellipsis, so `max_lines` clips the last line \
                 instead of eliding it.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::highlighted_text::highlighted_text",
                    "oito2_iced::widget::highlighted_text::HighlightedText",
                    "oito2_iced::widget::highlighted_text::matches",
                    "oito2_iced::widget::highlighted_text::Options",
                ],
            ),
        ],
    )
}
