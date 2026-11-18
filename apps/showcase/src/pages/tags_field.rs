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

//! The TagsField page: suggestions, limits, validation and the subtle
//! style.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::collections::ListCollection;
use oito2_iced::widget::tags_field::{TagsFieldStyle, tags_field};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tags_field.rs");

/// The width of the examples.
const WIDTH: f32 = 420.0;

// #region suggestions-data
/// The languages the suggestions example offers; `None` when two repeat.
fn languages() -> Option<ListCollection<String>> {
    ListCollection::new(
        [
            "Dart",
            "Go",
            "Kotlin",
            "PHP",
            "Python",
            "Rust",
            "Svelte",
            "Swift",
            "TypeScript",
        ]
        .map(str::to_owned),
    )
    .ok()
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    fruits: Vec<String>,
    languages: Option<ListCollection<String>>,
    tags: Vec<String>,
    emails: Vec<String>,
    topics: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            fruits: vec!["Apple".to_owned(), "Mango".to_owned()],
            languages: languages(),
            tags: vec!["Rust".to_owned()],
            emails: Vec::new(),
            topics: ["Iced", "GTK", "Qt"].map(str::to_owned).to_vec(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The fruits changed.
    Fruits(Vec<String>),
    /// The languages changed.
    Tags(Vec<String>),
    /// The e-mail addresses changed.
    Emails(Vec<String>),
    /// The frameworks changed.
    Topics(Vec<String>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Fruits(tags) => state.fruits = tags,
        Message::Tags(tags) => state.tags = tags,
        Message::Emails(tags) => state.emails = tags,
        Message::Topics(tags) => state.topics = tags,
    }

    Task::none()
}

/// `content` at the width of the examples.
fn sized<'a>(content: impl Into<Element<'a, Message, Theme>>) -> Element<'a, Message, Theme> {
    container(content).width(Length::Fixed(WIDTH)).into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        sized(
            // #region preview
            tags_field(&state.fruits)
                .label("Fruits")
                .placeholder("Add a fruit")
                .localizations(l10n)
                .on_change(Message::Fruits),
            // #endregion
        ),
    );

    let suggestions = state.languages.as_ref().map(|languages| {
        doc::example(
            ctx,
            SOURCE,
            &["suggestions-data", "suggestions"],
            sized(
                // #region suggestions
                tags_field(&state.tags)
                    .label("Languages")
                    .placeholder("Add a language")
                    .suggestions(languages)
                    .localizations(l10n)
                    .on_change(Message::Tags),
                // #endregion
            ),
        )
    });

    let limits = doc::example(
        ctx,
        SOURCE,
        &["limits"],
        sized(
            // #region limits
            tags_field(&state.emails)
                .max_tags(3)
                .delimiters([",", " ", ";"])
                .placeholder("name@example.com")
                .validator(|tag| tag.contains('@'))
                .localizations(l10n)
                .on_change(Message::Emails),
            // #endregion
        ),
    );

    let subtle = doc::example(
        ctx,
        SOURCE,
        &["subtle"],
        sized(
            // #region subtle
            tags_field(&state.topics)
                .label("Frameworks")
                .style(TagsFieldStyle::Subtle)
                .clearable(true)
                .localizations(l10n)
                .on_change(Message::Topics),
            // #endregion
        ),
    );

    doc::page(
        "TagsField",
        "Collects a list of short values as chips, typed, pasted or picked from suggestions.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The field is controlled: pass the tags to `tags_field` and replace them in \
                 the message of `on_change`, which receives the whole new list. Enter or a \
                 comma adds the typed text, trimmed, as a tag, and a pasted list is split on \
                 the commas. Without `on_change` the field is disabled. `localizations` gives \
                 the tooltips of the \"x\" of the chips and of the clear button.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::tags_field::{TagsFieldStyle, tags_field};",
            ),
            doc::heading("Examples"),
            doc::subheading("Suggestions"),
            doc::prose(
                "With `suggestions`, a `ListCollection`, the enabled items that contain the \
                 typed text, ignoring case and accents, and aren't tags yet show in a panel \
                 below the input; Up and Down move the highlight, Enter or a click adds it.",
            ),
        ]
        .into_iter()
        .chain(suggestions)
        .chain([
            doc::subheading("Limits and validation"),
            doc::prose(
                "`max_tags` caps the list, `delimiters` sets the strings that end a tag, and \
                 `validator` refuses tags that don't fit. A typed tag that is refused stays in \
                 the input to be fixed; repeated tags are refused unless `allow_duplicates` is \
                 set. Once the limit is reached the input takes no more text.",
            ),
            limits,
            doc::subheading("Subtle style and clear button"),
            doc::prose(
                "`TagsFieldStyle::Subtle` draws neutral chips that take the accent when \
                 focused. `clearable(true)` adds a button that removes every tag at once.",
            ),
            subtle,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Enter, a delimiter", "Adds the typed text as a tag."],
                    vec![
                        "Backspace or Left in the empty input",
                        "Moves the focus to the last chip.",
                    ],
                    vec![
                        "Left, Right on a chip",
                        "Move to the previous or next chip; past the last one, back to the \
                         input.",
                    ],
                    vec!["Backspace, Delete on a chip", "Removes the focused chip."],
                    vec!["Escape", "Closes the suggestions until the text changes."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tags_field::tags_field",
                    "oito2_iced::widget::tags_field::TagsField",
                    "oito2_iced::widget::tags_field::TagsFieldStyle",
                ],
            ),
        ]),
    )
}
