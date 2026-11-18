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

//! The TreeView page: selection, renaming, filtering, check boxes and
//! lazy loading.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::collections::{SelectionMode, TreeCollection, TreeNode};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::search::search_field;
use oito2_iced::widget::tree_view::{TreeEvent, TreeState, TreeView, tree_view};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tree_view.rs");

// #region data
/// The files the trees show.
const PATHS: [&str; 9] = [
    "src/main.rs",
    "src/app.rs",
    "src/pages/home.rs",
    "src/pages/settings.rs",
    "src/widgets/avatar.rs",
    "tests/app.rs",
    "tests/widgets/avatar.rs",
    "Cargo.toml",
    "README.md",
];
// #endregion

/// Which tree of the page an event is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// The tree of the preview.
    Files,
    /// The filtered tree.
    Filtered,
    /// The checkable tree.
    Checks,
    /// The lazily loaded tree.
    Lazy,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    files: TreeCollection<()>,
    tree: TreeState,
    remote: TreeCollection<()>,
    lazy: TreeState,
    filtered: TreeState,
    checks: TreeState,
    query: String,
    activated: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // #region preview-data
            files: TreeCollection::from_paths(PATHS, "/"),
            tree: TreeState::new(SelectionMode::Multiple).with_expanded(["src"]),
            // #endregion
            // #region lazy-data
            remote: TreeCollection::new(vec![
                TreeNode::branch("server", Vec::new()).with_label("server (loads on open)"),
            ])
            .unwrap_or_else(|_| TreeCollection::from_paths(["server"], "/")),
            lazy: TreeState::default(),
            // #endregion
            filtered: TreeState::default(),
            checks: TreeState::default().with_expanded(["src", "tests"]),
            query: String::new(),
            activated: "None".to_owned(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A tree changed its open, selected or checked nodes.
    Tree(Which, TreeEvent),
    /// A node was activated.
    Activate(String),
    /// A node was renamed to the label.
    Rename(String, String),
    /// Opens every branch of the preview.
    ExpandAll,
    /// Closes every branch of the preview.
    CollapseAll,
    /// The filter changed.
    Query(String),
    /// A branch of the lazy tree asks for its children.
    Load(String),
    /// The children of the branch arrived.
    Loaded(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Tree(which, event) => match which {
            Which::Files => state.tree.apply(&state.files, event),
            Which::Filtered => state.filtered.apply(&state.files, event),
            Which::Checks => state.checks.apply(&state.files, event),
            Which::Lazy => state.lazy.apply(&state.remote, event),
        },
        Message::Activate(value) => state.activated = value,
        // #region rename
        Message::Rename(value, label) => {
            if let Some(node) = state.files.find(&value)
                && let Ok(files) = state.files.replace(&value, node.clone().with_label(label))
            {
                state.files = files;
            }
        }
        // #endregion
        // #region code-update
        Message::ExpandAll => state.tree.expand_all(&state.files),
        Message::CollapseAll => state.tree.collapse_all(),
        // #endregion
        // #region lazy-update
        Message::Load(value) => {
            state.lazy.start_loading(&value);

            return Task::perform(
                async move {
                    std::thread::sleep(Duration::from_secs(1));
                    value
                },
                Message::Loaded,
            );
        }
        Message::Loaded(value) => {
            let children = ["a.log", "b.log", "c.log"]
                .into_iter()
                .map(|name| TreeNode::leaf(format!("{value}/{name}")).with_label(name))
                .collect();
            let inserted = state.remote.insert(Some(&value), children, None);
            let loaded = inserted.is_ok();

            if let Ok(remote) = inserted {
                state.remote = remote;
            }
            state.lazy.finish_loading(&value, loaded);
        }
        // #endregion
        Message::Query(query) => state.query = query,
    }

    Task::none()
}

// #region icon
/// A folder icon, open or closed, for branches and a file icon for leaves.
fn file_icon<'a>(node: &TreeNode<()>, open: bool) -> Option<Element<'a, Message, Theme>> {
    let glyph = match (node.is_branch(), open) {
        (true, true) => lucide::FOLDER_OPEN,
        (true, false) => lucide::FOLDER,
        (false, _) => lucide::FILE,
    };

    Some(icon(glyph).into())
}
// #endregion

/// A bordered box around a tree view, 300 pixels wide and at most 280
/// tall; the tree takes the height of its rows up to that.
fn frame<'a>(tree: TreeView<'a, (), Message>) -> Element<'a, Message, Theme> {
    container(
        border_container(tree.height(Length::Shrink))
            .radius(radius::MEDIUM)
            .clip(true)
            .width(Length::Fixed(300.0)),
    )
    .max_height(280.0)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["data", "preview-data", "icon", "preview", "rename"],
        column![
            frame(
                // #region preview
                tree_view(&state.files, &state.tree)
                    .icon(file_icon)
                    .on_event(|event| Message::Tree(Which::Files, event))
                    .on_activate(Message::Activate)
                    .on_rename(Message::Rename)
                    .localizations(&ctx.l10n),
                // #endregion
            ),
            text(format!("Last opened: {}", state.activated)).class(TextClass::Variant),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    );

    let code = doc::example(
        ctx,
        SOURCE,
        &["code", "code-update"],
        // #region code
        row![
            button::outlined("Expand all").on_press(Message::ExpandAll),
            button::outlined("Collapse all").on_press(Message::CollapseAll),
        ]
        .spacing(spacing::SM),
        // #endregion
    );

    let filter = doc::example(
        ctx,
        SOURCE,
        &["filter"],
        column![
            container(
                search_field(&ctx.l10n, &state.query)
                    .placeholder("Filter")
                    .on_input(Message::Query)
            )
            .width(Length::Fixed(300.0)),
            frame(
                // #region filter
                tree_view(&state.files, &state.filtered)
                    .query(&state.query)
                    .icon(file_icon)
                    .on_event(|event| Message::Tree(Which::Filtered, event)),
                // #endregion
            ),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    );

    let checkable = doc::example(
        ctx,
        SOURCE,
        &["checkable"],
        frame(
            // #region checkable
            tree_view(&state.files, &state.checks)
                .checkable(true)
                .on_event(|event| Message::Tree(Which::Checks, event)),
            // #endregion
        ),
    );

    let lazy = doc::example(
        ctx,
        SOURCE,
        &["lazy-data", "lazy", "lazy-update"],
        frame(
            // #region lazy
            tree_view(&state.remote, &state.lazy)
                .icon(file_icon)
                .on_event(|event| Message::Tree(Which::Lazy, event))
                .on_load_children(Message::Load)
                .localizations(&ctx.l10n),
            // #endregion
        ),
    );

    doc::page(
        "TreeView",
        "A tree of nested items, such as files and folders, that the user opens, selects, \
         checks and renames with the mouse or the keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `tree_view` a `TreeCollection` of the nodes and a `TreeState`, which \
                 keeps the open, selected and checked node values. The state lives in the \
                 application: the tree publishes `TreeEvent`s through `on_event`, which \
                 `TreeState::apply` applies; without it the tree is read-only and skipped by \
                 Tab. `TreeState::new` takes the `SelectionMode`: `Single` by default, \
                 `Multiple` allows several and `None` disables selection.",
            ),
            doc::prose(
                "`icon` adds an icon before each label, given whether the node is open, and \
                 `item` replaces the row content. Only the rows in view are laid out and drawn, \
                 so the tree handles large trees; it scrolls itself with the wheel or its \
                 scrollbar.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::collections::{TreeCollection, TreeNode};\n\
                 use oito2_iced::widget::tree_view::{TreeEvent, TreeState, TreeView, tree_view};",
            ),
            doc::note(
                "Collections are immutable: `replace`, `insert`, `remove` and `move_nodes` \
                 return a new `TreeCollection`, which the application stores and passes again. \
                 `TreeCollection::from_paths` builds one from file paths.",
            ),
            doc::heading("Examples"),
            doc::subheading("Activating and renaming"),
            doc::prose(
                "`on_activate` publishes the value of a node on Enter or a double click, as in \
                 the tree above; without it, those open or close a branch instead. With \
                 `on_rename`, F2 starts an inline rename of the active node, and the message \
                 receives the value and the new label.",
            ),
            doc::subheading("Controlling from code"),
            doc::prose(
                "`TreeState` opens and closes branches and changes the selection: \
                 `expand_all`, `collapse_all`, `set_expanded`, `set_selected` and \
                 `toggle_selected`.",
            ),
            code,
            doc::subheading("Filtering"),
            doc::prose(
                "A non-empty `query` shows only the matching nodes, their ancestors and \
                 descendants, all open, with the match highlighted. Branches can't be opened \
                 or closed while filtering.",
            ),
            filter,
            doc::subheading("Check boxes"),
            doc::prose(
                "With `checkable(true)`, each row has a tristate check box, kept in the \
                 checked leaves of the `TreeState`. Checking a branch checks all its leaves, \
                 and a branch with only some leaves checked shows a mixed state.",
            ),
            checkable,
            doc::subheading("Lazy loading"),
            doc::prose(
                "A branch with no children that opens for the first time publishes \
                 `on_load_children`. The application marks it with `start_loading`, which \
                 shows a spinner, adds the children to the collection and calls \
                 `finish_loading`; when loading fails, the branch closes, and opening it again \
                 retries.",
            ),
            lazy,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Down, Up",
                        "Moves the active row; a single-selection tree also selects it.",
                    ],
                    vec!["Home, End", "Moves to the first or the last row."],
                    vec![
                        "Shift + arrows, Home, End",
                        "Extends the selection, in a multiple-selection tree.",
                    ],
                    vec![
                        "Right",
                        "Opens a branch, or moves to its first child (Left in right-to-left \
                         layouts).",
                    ],
                    vec![
                        "Left",
                        "Closes a branch, or moves to the parent (Right in right-to-left \
                         layouts).",
                    ],
                    vec![
                        "Space",
                        "Toggles the check box of a checkable tree, or else the selection of a \
                         multiple-selection one.",
                    ],
                    vec![
                        "Ctrl+A, Cmd+A",
                        "Selects every shown, enabled node, in a multiple-selection tree.",
                    ],
                    vec![
                        "Enter",
                        "Publishes `on_activate`, or opens or closes a branch without it.",
                    ],
                    vec![
                        "F2",
                        "Renames the active node, with `on_rename`; Enter confirms and Escape \
                         cancels.",
                    ],
                    vec![
                        "Letters",
                        "Jump to the next shown label that starts with them.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tree_view::tree_view",
                    "oito2_iced::widget::tree_view::TreeView",
                    "oito2_iced::widget::tree_view::TreeState",
                    "oito2_iced::widget::tree_view::TreeEvent",
                    "oito2_iced::collections::TreeCollection",
                    "oito2_iced::collections::TreeNode",
                    "oito2_iced::collections::SelectionMode",
                ],
            ),
        ],
    )
}
