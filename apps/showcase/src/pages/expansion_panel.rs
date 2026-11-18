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

//! The ExpansionPanel page: an accordion, independent sections and change
//! reports.

use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::expandable::{expansion_panel, set_expanded};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("expansion_panel.rs");

/// The width of the example panels.
const WIDTH: f32 = 480.0;

// #region data
/// The questions of the examples and their answers.
const QUESTIONS: [(&str, &str); 3] = [
    (
        "What is oito2_iced?",
        "The OITO2 design system for Iced applications.",
    ),
    (
        "Which platforms does it support?",
        "Linux desktops, Windows and macOS.",
    ),
    (
        "Is it free software?",
        "Yes, under the GNU GPL version 3 or later.",
    ),
];
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    accordion: Vec<bool>,
    independent: Vec<bool>,
    reported: Vec<bool>,
    log: Vec<String>,
    compact: Vec<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            accordion: vec![true, false, false],
            independent: vec![false; QUESTIONS.len()],
            reported: vec![false; QUESTIONS.len()],
            log: Vec::new(),
            compact: vec![false, false],
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A section of the accordion asked to open or close.
    Accordion(usize, bool),
    /// A section of the independent panel asked to open or close.
    Independent(usize, bool),
    /// A section of the reporting panel asked to open or close.
    Reported(usize, bool),
    /// A section of the compact panel asked to open or close.
    Compact(usize, bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Accordion(index, expanded) => {
            set_expanded(&mut state.accordion, index, expanded, true);
        }
        // #endregion
        // #region independent-update
        Message::Independent(index, expanded) => {
            set_expanded(&mut state.independent, index, expanded, false);
        }
        // #endregion
        // #region callback-update
        Message::Reported(index, expanded) => {
            for (index, open) in set_expanded(&mut state.reported, index, expanded, true) {
                state
                    .log
                    .push(format!("{} {}", index + 1, if open { '+' } else { '−' }));
            }
        }
        // #endregion
        Message::Compact(index, expanded) => {
            set_expanded(&mut state.compact, index, expanded, false);
        }
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["data", "preview", "update"],
        container(
            // #region preview
            QUESTIONS
                .iter()
                .fold(
                    expansion_panel(&state.accordion),
                    |panel, (question, answer)| panel.section(*question, text(*answer)),
                )
                .on_toggle(Message::Accordion),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let independent = doc::example(
        ctx,
        SOURCE,
        &["independent", "independent-update"],
        container(
            // #region independent
            QUESTIONS
                .iter()
                .fold(
                    expansion_panel(&state.independent),
                    |panel, (question, answer)| panel.section(*question, text(*answer)),
                )
                .on_toggle(Message::Independent),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let changes = state.log.iter().rev().take(6).cloned().collect::<Vec<_>>();
    let callback = doc::example(
        ctx,
        SOURCE,
        &["callback", "callback-update"],
        column![
            // #region callback
            QUESTIONS
                .iter()
                .fold(
                    expansion_panel(&state.reported),
                    |panel, (question, answer)| { panel.section(*question, text(*answer)) }
                )
                .on_toggle(Message::Reported),
            // #endregion
            text(format!("Changes: {}", changes.join(", "))),
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(WIDTH)),
    );

    let compact = doc::example(
        ctx,
        SOURCE,
        &["compact"],
        container(
            // #region compact
            expansion_panel(&state.compact)
                .section("Without dividers", text("Sections sit closer together."))
                .section("Smaller padding", text("4 pixels around each section."))
                .dividers(false)
                .padding(spacing::XS)
                .animated(false)
                .on_toggle(Message::Compact),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "ExpansionPanel",
        "A bordered stack of sections whose headers show or hide their content, as an \
         accordion.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `expansion_panel` the open state of each section, then add the \
                 sections with `section` (a header text over its content) or `section_with` \
                 (any header element). Section `i` is open while `expanded[i]` is true; \
                 sections past the end of the slice are closed. Each section is an \
                 `expandable`.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::expandable::{expansion_panel, set_expanded};",
            ),
            doc::prose(
                "The open state lives in the application. `on_toggle` publishes the index \
                 of the activated section and the state it asks for, and `set_expanded` \
                 applies that request. With `collapse_on_expand` set to `true`, opening a \
                 section closes the others, so at most one is open. Without `on_toggle` \
                 the headers are disabled.",
            ),
            doc::heading("Examples"),
            doc::subheading("Independent sections"),
            doc::prose(
                "With `collapse_on_expand` set to `false`, each section opens and closes on \
                 its own.",
            ),
            independent,
            doc::subheading("Reacting to changes"),
            doc::prose(
                "`set_expanded` returns every section that changed with its new state: the \
                 sections that `collapse_on_expand` closed first, then the one that was \
                 activated.",
            ),
            callback,
            doc::subheading("Dividers, padding and motion"),
            doc::prose(
                "`dividers(false)` removes the 1-pixel lines between the sections, \
                 `padding` sets the space around each section (8 pixels vertically and 12 \
                 horizontally by default), and `animated(false)` shows and hides the \
                 content at once, for reduced motion.",
            ),
            compact,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next header."],
                    vec!["`Enter` / `Space`", "Opens or closes the focused section."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::expandable::expansion_panel",
                    "oito2_iced::widget::expandable::ExpansionPanel",
                    "oito2_iced::widget::expandable::set_expanded",
                    "oito2_iced::widget::expandable::expandable",
                ],
            ),
        ],
    )
}
