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

//! The Collections page: lists, selections, trees and lists loaded page by
//! page, without user interface.

use std::collections::HashSet;
use std::time::Duration;

use iced::widget::{Column, column, container, text};
use iced::{Element, Length, Padding, Task};

use oito2_iced::Theme;
use oito2_iced::collections::{
    AsyncListController, AsyncListPage, ListCollection, LoadId, SelectionController, SelectionMode,
    SelectionOptions, TreeCollection,
};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::list_box::{SelectionChange, list_box};
use oito2_iced::widget::list_tile::list_tile;
use oito2_iced::widget::progress;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("collections.rs");

// #region list-data
/// A language: its value, its label and its group.
type Language = (&'static str, &'static str, &'static str);

/// The languages of the list.
const LANGUAGES: [Language; 7] = [
    ("dart", "Dart", "Compiled"),
    ("go", "Go", "Compiled"),
    ("rust", "Rust", "Compiled"),
    ("cobol", "COBOL (unavailable)", "Compiled"),
    ("python", "Python", "Interpreted"),
    ("ruby", "Ruby", "Interpreted"),
    ("lua", "Lua", "Interpreted"),
];

/// The selection over the languages, several at a time.
fn languages() -> Option<SelectionController<Language>> {
    let collection = ListCollection::with_value_of(LANGUAGES, |language| language.0.to_owned())
        .ok()?
        .with_label_of(|language| language.1.to_owned())
        .with_group_of(|language| Some(language.2.to_owned()))
        .with_disabled(|language| language.0 == "cobol");

    Some(SelectionController::new(
        collection,
        SelectionOptions {
            mode: SelectionMode::Multiple,
            ..SelectionOptions::default()
        },
    ))
}
// #endregion

// #region async-data
/// The page of numbers at `cursor`: twenty numbers, up to 100.
fn numbers(cursor: Option<u32>) -> AsyncListPage<u32, u32> {
    let start = cursor.unwrap_or(0);
    let next = start + 20;
    let items = (start + 1..=next).collect();

    if next < 100 {
        AsyncListPage::with_cursor(items, next)
    } else {
        AsyncListPage::last(items)
    }
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    languages: Option<SelectionController<Language>>,
    // #region tree-data
    tree: TreeCollection<()>,
    expanded: HashSet<String>,
    // #endregion
    numbers: AsyncListController<u32, u32>,
}

impl Default for State {
    fn default() -> Self {
        let mut loader = AsyncListController::new();
        let request = loader.reload();
        loader.finish(request.id, Ok(numbers(request.cursor)));

        Self {
            languages: languages(),
            tree: TreeCollection::from_paths(
                [
                    "src/main.rs",
                    "src/app.rs",
                    "src/pages/home.rs",
                    "src/pages/settings.rs",
                    "tests/app.rs",
                    "Cargo.toml",
                ],
                "/",
            ),
            expanded: HashSet::from(["src".to_owned()]),
            numbers: loader,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The list box changed the selection.
    Selection(SelectionChange),
    /// A branch of the tree opens or closes.
    Toggle(String),
    /// Loads the next page of numbers.
    LoadMore,
    /// A page of numbers arrived.
    Loaded(LoadId, AsyncListPage<u32, u32>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region async-update
        Message::LoadMore => {
            if let Some(request) = state.numbers.load_more() {
                return Task::perform(
                    async move {
                        std::thread::sleep(Duration::from_millis(400));
                        numbers(request.cursor)
                    },
                    move |page| Message::Loaded(request.id, page),
                );
            }
        }
        Message::Loaded(id, page) => {
            state.numbers.finish(id, Ok(page));
        }
        // #endregion
        Message::Selection(change) => {
            if let Some(languages) = &mut state.languages {
                change.apply(languages);
            }
        }
        Message::Toggle(value) => {
            if !state.expanded.remove(&value) {
                state.expanded.insert(value);
            }
        }
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let list: Element<'a, Message, Theme> = match &state.languages {
        Some(languages) => column![
            // #region list-ui
            list_box(languages)
                .on_change(Message::Selection)
                .max_height(280.0),
            text(format!(
                "Selected: {}",
                languages.selected_values().join(", ")
            )),
            // #endregion
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(320.0))
        .into(),
        None => text("The languages share a value.").into(),
    };
    let list = doc::example(ctx, SOURCE, &["list-data", "list-ui"], list);

    let tree =
        doc::example(
            ctx,
            SOURCE,
            &["tree-data", "tree-ui"],
            container(
                // #region tree-ui
                Column::with_children(state.tree.visible(&state.expanded).into_iter().map(
                    |entry| {
                        let node = entry.node;
                        let glyph = if node.is_branch() {
                            lucide::FOLDER
                        } else {
                            lucide::FILE
                        };

                        container(
                            list_tile()
                                .leading(icon(glyph))
                                .title(node.label())
                                .on_press_maybe(
                                    node.is_branch()
                                        .then(|| Message::Toggle(node.value.clone())),
                                ),
                        )
                        .padding(Padding {
                            left: entry.depth as f32 * 20.0,
                            ..Padding::ZERO
                        })
                        .into()
                    },
                )),
                // #endregion
            )
            .width(Length::Fixed(320.0)),
        );

    let loading = state.numbers.is_loading() || state.numbers.is_loading_more();
    let numbers = doc::example(
        ctx,
        SOURCE,
        &["async-data", "async-ui", "async-update"],
        // #region async-ui
        column![
            text(format!(
                "{} of 100 items loaded",
                state.numbers.items().len()
            )),
            if loading {
                Element::from(container(progress::linear_indeterminate()).width(200))
            } else {
                button::outlined("Load more")
                    .on_press_maybe(state.numbers.has_more().then_some(Message::LoadMore))
                    .into()
            },
        ]
        .spacing(spacing::SM)
        .align_x(iced::Alignment::Center),
        // #endregion
    );

    doc::page(
        "Collections",
        "Descriptions of lists and trees of items, with their selection and loading, that \
         the list, tree and autocomplete widgets share.",
        [
            doc::heading("Usage"),
            doc::prose(
                "A collection wraps your own items and says how to read them: the value that \
                 identifies each one, the label to show and search, an optional group and \
                 whether it is disabled. Widgets take the collection instead of a list of \
                 elements, so the same items feed several of them. The controllers are plain \
                 structs: the application keeps them in its model and calls their methods from \
                 `update`; methods that change state return whether something changed.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::collections::{ListCollection, SelectionController};",
            ),
            doc::table(
                &["Type", "Describes"],
                &[
                    vec!["`ListCollection`", "A flat list, optionally grouped"],
                    vec!["`TreeCollection`", "A tree of `TreeNode`s"],
                    vec![
                        "`AsyncListController`",
                        "A list loaded from a source page by page",
                    ],
                    vec![
                        "`SelectionController`",
                        "The selected values of a list, single or multiple",
                    ],
                    vec![
                        "`fold_text`",
                        "Text without case and accents, for searching",
                    ],
                ],
            ),
            doc::heading("Examples"),
            doc::subheading("List and selection"),
            doc::prose(
                "`ListCollection::with_value_of` takes the function that gives each item its \
                 unique value; `with_label_of`, `with_group_of` and `with_disabled` add the \
                 rest. A `SelectionController` keeps the selected values of a list, skipping \
                 disabled items. With `SelectionMode::Multiple`, Control or Command and a click \
                 toggle an item and Shift and a click select a range, as in `list_box`.",
            ),
            list,
            doc::subheading("Tree from paths"),
            doc::prose(
                "`TreeCollection::from_paths` builds folders from paths split by a separator. \
                 `visible` lists the nodes to show for a set of open branches, each with its \
                 depth, which is all a custom tree needs; `tree_view` adds the keyboard, \
                 selection and filtering.",
            ),
            tree,
            doc::subheading("Loading page by page"),
            doc::prose(
                "`AsyncListController` never loads anything itself. `reload` and `load_more` \
                 update its state and return an `AsyncListRequest` with the cursor to fetch; \
                 the application fetches the page, here in a `Task`, and hands it to `finish` \
                 with the id of the request. A newer request supersedes an older one, whose \
                 result `finish` ignores. A page without a cursor is the last one.",
            ),
            numbers,
            doc::caption("The collections have no widgets and can be tested without a view."),
            doc::api(
                ctx,
                &[
                    "oito2_iced::collections::ListCollection",
                    "oito2_iced::collections::SelectionController",
                    "oito2_iced::collections::SelectionMode",
                    "oito2_iced::collections::SelectionOptions",
                    "oito2_iced::collections::TreeCollection",
                    "oito2_iced::collections::TreeNode",
                    "oito2_iced::collections::AsyncListController",
                    "oito2_iced::collections::AsyncListPage",
                    "oito2_iced::collections::AsyncListRequest",
                    "oito2_iced::collections::fold_text",
                ],
            ),
        ],
    )
}
