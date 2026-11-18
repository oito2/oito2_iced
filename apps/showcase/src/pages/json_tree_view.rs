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

//! The JsonTreeView page: syntax colors, a filter, the open depth and
//! node paths.

use iced::widget::{column, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::collections::TreeCollection;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::json_tree_view::{
    JsonEntry, JsonValue, PATH_ROOT_LABEL, expanded_to_depth, json_tree, json_tree_view,
};
use oito2_iced::widget::search::search_field;
use oito2_iced::widget::tree_view::{TreeEvent, TreeState};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("json_tree_view.rs");

/// The width of the example trees.
const WIDTH: f32 = 360.0;

// #region data
/// The details of a package, as decoded JSON.
fn sample() -> JsonValue {
    let string = |value: &str| JsonValue::String(value.to_owned());
    let package = |tests: i64, native: bool| {
        JsonValue::Object(vec![
            ("tests".to_owned(), JsonValue::Integer(tests)),
            ("native".to_owned(), JsonValue::Bool(native)),
        ])
    };

    JsonValue::Object(vec![
        ("name".to_owned(), string("oito2_iced")),
        ("version".to_owned(), string("0.1.0")),
        ("private".to_owned(), JsonValue::Bool(false)),
        ("license".to_owned(), string("GPL-3.0-or-later")),
        ("homepage".to_owned(), JsonValue::Null),
        ("coverage".to_owned(), JsonValue::Float(0.87)),
        (
            "platforms".to_owned(),
            JsonValue::Array(vec![string("linux"), string("windows"), string("macos")]),
        ),
        (
            "crates".to_owned(),
            JsonValue::Object(vec![
                ("oito2_iced".to_owned(), package(963, false)),
                ("oito2_iced_files".to_owned(), package(42, true)),
            ]),
        ),
    ])
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    tree: TreeCollection<JsonEntry>,
    preview: TreeState,
    filtered: TreeState,
    query: String,
    closed: TreeState,
    paths: TreeCollection<JsonEntry>,
    paths_state: TreeState,
    path: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        // #region preview-state
        let tree = json_tree(&sample(), "package");
        let preview = TreeState::default().with_expanded(expanded_to_depth(&tree, 2));
        // #endregion

        // #region paths-data
        let user = JsonValue::Object(vec![(
            "user".to_owned(),
            JsonValue::Object(vec![
                ("name".to_owned(), JsonValue::String("Ana".to_owned())),
                (
                    "roles".to_owned(),
                    JsonValue::Array(vec![
                        JsonValue::String("admin".to_owned()),
                        JsonValue::String("editor".to_owned()),
                    ]),
                ),
            ]),
        )]);
        let paths = json_tree(&user, PATH_ROOT_LABEL);
        let paths_state = TreeState::default().with_expanded(expanded_to_depth(&paths, 3));
        // #endregion

        Self {
            filtered: preview.clone(),
            closed: TreeState::default(),
            tree,
            preview,
            query: String::new(),
            paths,
            paths_state,
            path: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview tree changed.
    Preview(TreeEvent),
    /// The filtered tree changed.
    Filtered(TreeEvent),
    /// The filter changed.
    Query(String),
    /// The closed tree changed.
    Closed(TreeEvent),
    /// The paths tree changed.
    Paths(TreeEvent),
    /// A node of the paths tree was activated.
    Path(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Preview(event) => state.preview.apply(&state.tree, event),
        // #endregion
        Message::Filtered(event) => state.filtered.apply(&state.tree, event),
        Message::Query(query) => state.query = query,
        Message::Closed(event) => state.closed.apply(&state.tree, event),
        Message::Paths(event) => state.paths_state.apply(&state.paths, event),
        Message::Path(path) => state.path = Some(path),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["data", "preview-state", "preview", "update"],
        // #region preview
        json_tree_view(&state.tree, &state.preview)
            .on_event(Message::Preview)
            .localizations(l10n)
            .width(Length::Fixed(WIDTH))
            .height(Length::Fixed(360.0)),
        // #endregion
    );

    let filter = doc::example(
        ctx,
        SOURCE,
        &["filter"],
        column![
            // #region filter
            search_field(l10n, &state.query)
                .placeholder("Filter keys and values")
                .on_input(Message::Query),
            json_tree_view(&state.tree, &state.filtered)
                .query(&state.query)
                .on_event(Message::Filtered)
                .localizations(l10n)
                .height(Length::Fixed(300.0)),
            // #endregion
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(WIDTH)),
    );

    let closed = doc::example(
        ctx,
        SOURCE,
        &["closed"],
        // #region closed
        json_tree_view(&state.tree, &state.closed)
            .on_event(Message::Closed)
            .localizations(l10n)
            .width(Length::Fixed(WIDTH))
            .height(Length::Shrink),
        // #endregion
    );

    let paths = doc::example(
        ctx,
        SOURCE,
        &["paths-data", "paths"],
        column![
            // #region paths
            json_tree_view(&state.paths, &state.paths_state)
                .on_event(Message::Paths)
                .on_activate(Message::Path)
                .localizations(l10n)
                .height(Length::Fixed(200.0)),
            // #endregion
            text(format!("Path: {}", state.path.as_deref().unwrap_or("none"))),
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "JsonTreeView",
        "A tree view of decoded JSON, with keys and values in the theme's syntax colors and \
         the size of each object and array.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Turn the data into a tree with `json_tree`: it takes a `JsonValue` (null, \
                 booleans, integers, floats, strings, arrays and objects) and the label of \
                 the top node, `DEFAULT_ROOT_LABEL` (\"root\") in the OITO2 design. Objects \
                 and arrays become branches that show their size; other values become \
                 leaves.",
            ),
            doc::prose(
                "`json_tree_view` shows the tree with the open branches of a `TreeState` the \
                 application owns: `on_event` publishes each change and `TreeState::apply` \
                 applies it. `expanded_to_depth` lists the branches to open so the tree \
                 starts open to a depth; open them again when the data changes. The view is \
                 a `TreeView`, so it has its keyboard and builders.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::json_tree_view::{\n    JsonValue, expanded_to_depth, \
                 json_tree, json_tree_view,\n};\nuse oito2_iced::widget::tree_view::{TreeEvent, \
                 TreeState};",
            ),
            doc::heading("Examples"),
            doc::subheading("Filtering"),
            doc::prose(
                "`query` filters the nodes by key and value, showing the matches and their \
                 ancestors, all open.",
            ),
            filter,
            doc::subheading("Closed at start"),
            doc::prose(
                "A `TreeState` with no open branches, or `expanded_to_depth` with a depth of \
                 0, starts with every branch closed. A `Shrink` height shows every row
                 without scrolling.",
            ),
            closed,
            doc::subheading("Node paths"),
            doc::prose(
                "The node values of the tree are JSON paths, such as `$.user.roles[0]`, and \
                 each node holds its key and summary as a `JsonEntry`. `PATH_ROOT_LABEL` \
                 labels the top node `$`. With `on_activate`, a double click or `Enter` \
                 publishes the path of the node.",
            ),
            paths,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Up` / `Down`, `Home` / `End`", "Move the active row."],
                    vec![
                        "`Right` / `Left`",
                        "Open a branch or move to its first child; close it or move to the parent.",
                    ],
                    vec![
                        "`Enter`",
                        "Publishes the path of the active node, or opens or closes a branch.",
                    ],
                    vec!["Letters", "Move to the next label that starts with them."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::json_tree_view::json_tree_view",
                    "oito2_iced::widget::json_tree_view::json_tree",
                    "oito2_iced::widget::json_tree_view::expanded_to_depth",
                    "oito2_iced::widget::json_tree_view::JsonValue",
                    "oito2_iced::widget::json_tree_view::JsonEntry",
                    "oito2_iced::widget::json_tree_view::JsonSummary",
                    "oito2_iced::widget::json_tree_view::DEFAULT_ROOT_LABEL",
                    "oito2_iced::widget::json_tree_view::PATH_ROOT_LABEL",
                ],
            ),
        ],
    )
}
