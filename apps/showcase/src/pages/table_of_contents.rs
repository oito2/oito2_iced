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

//! The TableOfContents page: a table of contents following a document,
//! and one with a lower activation line and a pick message.

use iced::widget::{Space, column, container, row, scrollable, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::styled_text;
use oito2_iced::widget::table_of_contents::{self as toc, Entry};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("table_of_contents.rs");

// #region sections
/// The sections of the document, with their depth.
const SECTIONS: [(&str, u16); 7] = [
    ("Introduction", 0),
    ("Installation", 0),
    ("Linux", 1),
    ("Windows", 1),
    ("Usage", 0),
    ("Theming", 1),
    ("License", 0),
];

/// The id of the scrollable document of the preview.
const DOCUMENT: &str = "toc-document";

/// The entries of the sections, whose targets are the ids of their
/// headings in the document with the `prefix`.
fn entries(prefix: &str) -> Vec<Entry> {
    SECTIONS
        .iter()
        .enumerate()
        .map(|(index, (title, depth))| {
            toc::entry(*title, format!("{prefix}-{index}")).depth(*depth)
        })
        .collect()
}
// #endregion

/// The id of the scrollable document of the second example.
const PICKS_DOCUMENT: &str = "toc-picks-document";

/// The state of the page.
#[derive(Debug)]
pub struct State {
    entries: Vec<Entry>,
    section: usize,
    picks_entries: Vec<Entry>,
    picks_section: usize,
    picked: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            entries: entries(DOCUMENT),
            section: 0,
            picks_entries: entries(PICKS_DOCUMENT),
            picks_section: 0,
            picked: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview document scrolled to another section.
    Section(usize),
    /// An entry of the preview was picked.
    Pick(usize),
    /// The second document scrolled to another section.
    PicksSection(usize),
    /// An entry of the second example was picked.
    Picked(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Section(section) => state.section = section,
        Message::PicksSection(section) => state.picks_section = section,
        // #region pick
        Message::Pick(index) => {
            if let Some(entry) = state.entries.get(index) {
                return toc::scroll_to(DOCUMENT, entry.target.clone());
            }
        }
        // #endregion
        Message::Picked(index) => {
            state.picked = Some(index);
            if let Some(entry) = state.picks_entries.get(index) {
                return toc::scroll_to(PICKS_DOCUMENT, entry.target.clone());
            }
        }
    }

    Task::none()
}

/// The sample document: one heading per section, with the id of its entry
/// in `entries`, and a paragraph under it, in a scrollable with the `id`.
fn document<'a>(entries: &[Entry], id: &'static str) -> Element<'a, Message, Theme> {
    let sections = SECTIONS.iter().zip(entries).map(|((title, depth), entry)| {
        let style = if *depth == 0 {
            typography::TITLE_LARGE
        } else {
            typography::TITLE_MEDIUM
        };

        column![
            container(styled_text(*title, style)).id(entry.target.clone()),
            text(format!("Text of the {title} section. ").repeat(12)),
        ]
        .spacing(spacing::SM)
        .into()
    });

    scrollable(
        column(sections)
            .push(Space::new().height(240.0))
            .spacing(spacing::XL)
            .padding([0.0, spacing::MD]),
    )
    .id(id)
    .height(Length::Fill)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["sections", "preview", "preview-tracker", "pick"],
        row![
            container(
                // #region preview
                toc::table_of_contents(&state.entries, state.section).on_select(Message::Pick),
                // #endregion
            )
            .width(Length::Fixed(180.0)),
            // #region preview-tracker
            toc::tracker(
                document(&state.entries, DOCUMENT),
                &state.entries,
                state.section
            )
            .on_change(Message::Section),
            // #endregion
        ]
        .spacing(spacing::LG)
        .width(Length::Fixed(640.0))
        .height(Length::Fixed(360.0)),
    );

    let picked = match state.picked.and_then(|index| SECTIONS.get(index)) {
        Some((title, _)) => format!("Picked: {title}"),
        None => "Picked: none".to_owned(),
    };

    let picks = doc::example(
        ctx,
        SOURCE,
        &["picks", "picks-tracker"],
        row![
            column![
                // #region picks
                toc::table_of_contents(&state.picks_entries, state.picks_section)
                    .on_select(Message::Picked),
                // #endregion
                doc::caption(picked),
            ]
            .spacing(spacing::MD)
            .width(Length::Fixed(180.0)),
            // #region picks-tracker
            toc::tracker(
                document(&state.picks_entries, PICKS_DOCUMENT),
                &state.picks_entries,
                state.picks_section,
            )
            .activation_offset(120.0)
            .on_change(Message::PicksSection),
            // #endregion
        ]
        .spacing(spacing::LG)
        .width(Length::Fixed(640.0))
        .height(Length::Fixed(300.0)),
    );

    doc::page(
        "TableOfContents",
        "Lists the sections of a scrolling document, highlights the one in view and scrolls to \
         the one picked.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the sections as `Entry` values built with `entry`: a title, the id of the section \
                 heading inside the scrollable, such as a `container` with an `id`, and a \
                 `depth` that indents subsections. Three pieces work together, with the \
                 current section kept in the application:",
            ),
            doc::table(
                &["Piece", "What it does"],
                &[
                    vec![
                        "`table_of_contents`",
                        "Shows the entries with the current one highlighted and publishes a \
                         picked entry through `on_select`.",
                    ],
                    vec![
                        "`tracker`",
                        "Wraps the scrollable that holds the sections and publishes the \
                         current section through `on_change` as the content scrolls.",
                    ],
                    vec![
                        "`scroll_to`",
                        "The task that scrolls the scrollable to the heading of a picked \
                         section, returned from `update`.",
                    ],
                ],
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::table_of_contents::{self as toc, Entry};",
            ),
            doc::prose(
                "The current section is the last one whose heading has reached the activation \
                 line, 32 pixels below the top of the viewport by default, or the last section \
                 when the view is scrolled to the end.",
            ),
            doc::heading("Examples"),
            doc::subheading("Activation line and picks"),
            doc::prose(
                "A larger `activation_offset` makes a section current while its heading is \
                 still lower in the view. `on_select` publishes the index of each picked \
                 entry, which `update` answers with the scroll.",
            ),
            picks,
            doc::note(
                "Without `on_select` the entries can't be picked and are skipped by Tab. \
                 `direction` puts the line and the indent at the start of the entries for \
                 right to left.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next entry."],
                    vec!["Enter, Space", "Picks the focused entry."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::table_of_contents::table_of_contents",
                    "oito2_iced::widget::table_of_contents::TableOfContents",
                    "oito2_iced::widget::table_of_contents::entry",
                    "oito2_iced::widget::table_of_contents::Entry",
                    "oito2_iced::widget::table_of_contents::tracker",
                    "oito2_iced::widget::table_of_contents::Tracker",
                    "oito2_iced::widget::table_of_contents::scroll_to",
                    "oito2_iced::widget::table_of_contents::ACTIVATION_OFFSET",
                ],
            ),
        ],
    )
}
