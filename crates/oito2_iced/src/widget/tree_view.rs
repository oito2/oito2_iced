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

//! Tree views with expansion, selection and keyboard navigation.
//!
//! [`tree_view`] shows the nodes of a [`TreeCollection`] with the open,
//! selected and checked nodes of a [`TreeState`]. Only the rows in view
//! are laid out and drawn, so it handles large trees; the tree scrolls
//! itself with the wheel or its scrollbar. The tree takes the focus as a
//! whole and marks its active row:
//!
//! - Up, Down, Home and End move the active row; a single-selection tree
//!   selects it, and Shift extends a multiple selection.
//! - Right opens a branch or moves to its first child; Left closes it or
//!   moves to the parent (mirrored by [`TreeView::direction`]).
//! - Space toggles the check box of a [checkable](TreeView::checkable)
//!   tree, or else the selection of a multiple-selection one;
//!   Control+A (Command+A) selects every shown, enabled node of a
//!   multiple-selection tree.
//! - Enter, or a double click, publishes [`TreeView::on_activate`], or
//!   opens or closes a branch when it is not set.
//! - F2 starts an inline rename of the active node when
//!   [`TreeView::on_rename`] is set; Enter or leaving the field confirms it
//!   and Escape cancels it.
//! - Typing letters jumps to the next shown label that starts with them.
//!
//! A click selects a row; with Control (Command) it toggles the row and
//! with Shift it extends a multiple selection. A click on the disclosure
//! arrow opens or closes the branch, and a click on a check box toggles
//! it.
//!
//! The state lives in the application: the tree publishes [`TreeEvent`]s
//! through [`TreeView::on_event`], which [`TreeState::apply`] applies.
//! Without it the tree is read-only and skipped by Tab. The active row,
//! the rename field and the scroll offset live in the tree.
//!
//! A branch with no children that opens for the first time publishes
//! [`TreeView::on_load_children`]; the app marks it with
//! [`TreeState::start_loading`], which shows a spinner, adds the children
//! to the collection and calls [`TreeState::finish_loading`]. A non-empty
//! [query](TreeView::query) shows only the matching nodes, their
//! ancestors and descendants, all open, with the match highlighted;
//! branches can't be opened or closed while filtering.
//!
//! ```no_run
//! use oito2_iced::collections::{TreeCollection, TreeNode};
//! use oito2_iced::widget::tree_view::{self, TreeEvent, TreeState};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Tree(TreeEvent),
//!     Open(String),
//! }
//!
//! # fn view<'a>(files: &'a TreeCollection<()>, state: &'a TreeState) -> oito2_iced::Element<'a, Message> {
//! tree_view::tree_view(files, state)
//!     .on_event(Message::Tree)
//!     .on_activate(Message::Open)
//!     .into()
//! # }
//! # fn update(files: &TreeCollection<()>, state: &mut TreeState, message: Message) {
//! // In `update`:
//! if let Message::Tree(event) = message {
//!     state.apply(files, event);
//! }
//! # }
//! ```

use std::cell::OnceCell;
use std::collections::{HashMap, HashSet};
use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation, TextInput};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::widget::scrollable::{self, Status as ScrollStatus};
use iced::widget::text::{LineHeight, Wrapping};
use iced::widget::{Row, Space, container, text_input};
use iced::{
    Border, Color, Element as IcedElement, Event, Length, Padding, Pixels, Point, Rectangle, Size,
    Vector, alignment, mouse, touch, window,
};

use super::helpers::{DoubleClick, ReadingDirection};
use super::highlighted_text::highlighted_text;
use super::scroll_area::{SCROLLBAR_WIDTH, SCROLLER_WIDTH};
use super::{progress, styled_text, toggles, tooltip};
use crate::collections::{SelectionMode, TreeCollection, TreeNode, fold_text};
use crate::l10n::Localizations;
use crate::theme::DISABLED_CONTENT_OPACITY;
use crate::theme::text_input::Class as InputClass;
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme};

/// The default indent of each level, in logical pixels.
pub const DEFAULT_INDENT: f32 = 16.0;

/// The time the disclosure arrow takes to turn.
pub const TURN_DURATION: Duration = Duration::from_millis(150);

/// The pause after which type-ahead starts a new prefix.
pub const TYPE_AHEAD_RESET: Duration = Duration::from_millis(600);

/// The width of the disclosure column (a literal in the OITO2 design).
const DISCLOSURE_WIDTH: f32 = 20.0;

/// The side of the disclosure arrow (a literal in the OITO2 design).
const ARROW_SIZE: f32 = 16.0;

/// The diameter of the loading spinner (a literal in the OITO2 design).
const SPINNER_SIZE: f32 = 14.0;

/// The ring thickness of the loading spinner (a literal in the OITO2
/// design).
const SPINNER_STROKE: f32 = 2.0;

/// The space above the first row and below the last one.
const LIST_PADDING: f32 = spacing::XS;

/// The distance one line of a mouse wheel scrolls.
const WHEEL_LINE: f32 = 60.0;

/// A change of the open, selected or checked nodes that a [`TreeView`]
/// publishes; [`TreeState::apply`] applies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeEvent {
    /// The branch `value` opens (`expanded`) or closes.
    Expand {
        /// The value of the branch.
        value: String,
        /// Whether it opens.
        expanded: bool,
    },
    /// The selection becomes these values.
    Select(Vec<String>),
    /// The node joins the selection or leaves it.
    ToggleSelected(String),
    /// The check box of the node toggles, with every leaf of a branch.
    ToggleChecked(String),
}

/// The open, selected, checked and loading nodes of a [`TreeView`], by
/// node value. The application owns it and changes it in `update`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeState {
    selection_mode: SelectionMode,
    expanded: HashSet<String>,
    selected: HashSet<String>,
    checked: HashSet<String>,
    loading: HashSet<String>,
    loaded: HashSet<String>,
}

impl TreeState {
    /// A state with nothing open, selected or checked, where
    /// `selection_mode` says how many nodes can be selected.
    #[must_use]
    pub fn new(selection_mode: SelectionMode) -> Self {
        Self {
            selection_mode,
            ..Self::default()
        }
    }

    /// Returns the state with the branches in `values` open.
    #[must_use]
    pub fn with_expanded<S: Into<String>>(mut self, values: impl IntoIterator<Item = S>) -> Self {
        self.expanded.extend(values.into_iter().map(Into::into));
        self
    }

    /// Returns the state with `values` selected, as
    /// [`set_selected`](Self::set_selected) selects them.
    #[must_use]
    pub fn with_selected<S: Into<String>>(mut self, values: impl IntoIterator<Item = S>) -> Self {
        self.set_selected(values);
        self
    }

    /// Returns the state with the leaves in `values` checked.
    #[must_use]
    pub fn with_checked<S: Into<String>>(mut self, values: impl IntoIterator<Item = S>) -> Self {
        self.checked.extend(values.into_iter().map(Into::into));
        self
    }

    /// How many nodes can be selected.
    #[must_use]
    pub fn selection_mode(&self) -> SelectionMode {
        self.selection_mode
    }

    /// The values of the open branches.
    #[must_use]
    pub fn expanded(&self) -> &HashSet<String> {
        &self.expanded
    }

    /// The values of the selected nodes.
    #[must_use]
    pub fn selected(&self) -> &HashSet<String> {
        &self.selected
    }

    /// The values of the checked leaves.
    #[must_use]
    pub fn checked(&self) -> &HashSet<String> {
        &self.checked
    }

    /// Whether the branch `value` is open.
    #[must_use]
    pub fn is_expanded(&self, value: &str) -> bool {
        self.expanded.contains(value)
    }

    /// Whether `value` is selected.
    #[must_use]
    pub fn is_selected(&self, value: &str) -> bool {
        self.selected.contains(value)
    }

    /// Opens the branch `value` when `expanded` is true, or closes it;
    /// returns whether it changed.
    pub fn set_expanded(&mut self, value: &str, expanded: bool) -> bool {
        if expanded {
            self.expanded.insert(value.to_owned())
        } else {
            self.expanded.remove(value)
        }
    }

    /// Opens exactly the branches in `values` and closes the others.
    pub fn set_expanded_values<S: Into<String>>(&mut self, values: impl IntoIterator<Item = S>) {
        self.expanded = values.into_iter().map(Into::into).collect();
    }

    /// Opens every branch of `collection`, keeping the branches already
    /// open.
    pub fn expand_all<T>(&mut self, collection: &TreeCollection<T>) {
        self.expanded
            .extend(collection.branch_values().into_iter().map(str::to_owned));
    }

    /// Closes every branch.
    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }

    /// Makes `values` the selection: only the first one in single mode,
    /// and none in [`SelectionMode::None`].
    pub fn set_selected<S: Into<String>>(&mut self, values: impl IntoIterator<Item = S>) {
        self.selected.clear();
        let values = values.into_iter().map(Into::into);

        match self.selection_mode {
            SelectionMode::None => {}
            SelectionMode::Single => self.selected.extend(values.take(1)),
            SelectionMode::Multiple => self.selected.extend(values),
        }
    }

    /// Adds `value` to the selection or removes it, in multiple mode; in
    /// the other modes it behaves as [`set_selected`](Self::set_selected)
    /// with `value` alone.
    pub fn toggle_selected(&mut self, value: &str) {
        if self.selection_mode != SelectionMode::Multiple {
            self.set_selected([value]);
        } else if !self.selected.remove(value) {
            self.selected.insert(value.to_owned());
        }
    }

    /// The check state of `value` in `collection`: `Some(true)` or
    /// `Some(false)`, or `None` when only some of a branch's leaves are
    /// checked. A branch without leaves is unchecked.
    #[must_use]
    pub fn check_state_of<T>(&self, collection: &TreeCollection<T>, value: &str) -> Option<bool> {
        match collection.find(value) {
            Some(node) if node.is_branch() => {
                let leaves = collection.leaves_of(value);
                let count = leaves
                    .iter()
                    .filter(|leaf| self.checked.contains(**leaf))
                    .count();

                mixed(count, leaves.len())
            }
            _ => Some(self.checked.contains(value)),
        }
    }

    /// Checks `value`, or every leaf of a branch, unless already fully
    /// checked, in which case it unchecks them. Does nothing when
    /// `collection` has no node `value`.
    pub fn toggle_checked<T>(&mut self, collection: &TreeCollection<T>, value: &str) {
        if !collection.has(value) {
            return;
        }

        let check = self.check_state_of(collection, value) != Some(true);

        for leaf in collection.leaves_of(value) {
            if check {
                self.checked.insert(leaf.to_owned());
            } else {
                self.checked.remove(leaf);
            }
        }
    }

    /// Marks the children of the branch `value` as loading: the tree shows
    /// a spinner instead of its disclosure arrow.
    pub fn start_loading(&mut self, value: &str) {
        self.loading.insert(value.to_owned());
    }

    /// Ends the load of the children of `value`: when `loaded` the branch
    /// never loads again; otherwise it closes, and opening it again
    /// retries the load.
    pub fn finish_loading(&mut self, value: &str, loaded: bool) {
        self.loading.remove(value);

        if loaded {
            self.loaded.insert(value.to_owned());
        } else {
            self.expanded.remove(value);
        }
    }

    /// Whether the children of `value` are loading.
    #[must_use]
    pub fn is_loading(&self, value: &str) -> bool {
        self.loading.contains(value)
    }

    /// Whether the children of `value` have loaded.
    #[must_use]
    pub fn is_loaded(&self, value: &str) -> bool {
        self.loaded.contains(value)
    }

    /// Applies an `event` a [`TreeView`] of `collection` published.
    pub fn apply<T>(&mut self, collection: &TreeCollection<T>, event: TreeEvent) {
        match event {
            TreeEvent::Expand { value, expanded } => {
                self.set_expanded(&value, expanded);
            }
            TreeEvent::Select(values) => self.set_selected(values),
            TreeEvent::ToggleSelected(value) => self.toggle_selected(&value),
            TreeEvent::ToggleChecked(value) => self.toggle_checked(collection, &value),
        }
    }
}

/// The check state of a branch with `count` of its `leaves` checked.
fn mixed(count: usize, leaves: usize) -> Option<bool> {
    if leaves == 0 || count == 0 {
        Some(false)
    } else if count == leaves {
        Some(true)
    } else {
        None
    }
}

/// Builds the content of a row after its disclosure arrow and check box.
type ItemFn<'a, T, Message> = Box<dyn Fn(&'a TreeNode<T>) -> Element<'a, Message> + 'a>;

/// Builds the icon of a row, given whether the node is open.
type IconFn<'a, T, Message> =
    Box<dyn Fn(&'a TreeNode<T>, bool) -> Option<Element<'a, Message>> + 'a>;

/// A tree of the nodes of a [`TreeCollection`]. See [`tree_view`].
pub struct TreeView<'a, T, Message> {
    collection: &'a TreeCollection<T>,
    state: &'a TreeState,
    on_event: Option<Box<dyn Fn(TreeEvent) -> Message + 'a>>,
    on_activate: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_rename: Option<Box<dyn Fn(String, String) -> Message + 'a>>,
    on_load_children: Option<Box<dyn Fn(String) -> Message + 'a>>,
    item: Option<ItemFn<'a, T, Message>>,
    icon: Option<IconFn<'a, T, Message>>,
    checkable: bool,
    query: String,
    row_height: Option<f32>,
    indent: f32,
    width: Length,
    height: Length,
    direction: ReadingDirection,
    animated: bool,
    loading_label: &'static str,
    id: Option<Id>,
    entries: OnceCell<Vec<Entry<'a, T>>>,
    checks: OnceCell<HashMap<&'a str, Option<bool>>>,
    children: Vec<Child<'a, Message>>,
}

/// A tree of the nodes of `collection` with the open, selected and
/// checked nodes of `state`; read-only until [`TreeView::on_event`] is
/// set.
pub fn tree_view<'a, T, Message>(
    collection: &'a TreeCollection<T>,
    state: &'a TreeState,
) -> TreeView<'a, T, Message> {
    TreeView {
        collection,
        state,
        on_event: None,
        on_activate: None,
        on_rename: None,
        on_load_children: None,
        item: None,
        icon: None,
        checkable: false,
        query: String::new(),
        row_height: None,
        indent: DEFAULT_INDENT,
        width: Length::Fill,
        height: Length::Fill,
        direction: ReadingDirection::LeftToRight,
        animated: true,
        loading_label: Localizations::en().loading,
        id: None,
        entries: OnceCell::new(),
        checks: OnceCell::new(),
        children: Vec::new(),
    }
}

impl<'a, T, Message> TreeView<'a, T, Message> {
    /// Sets the function that turns a change of the open, selected or
    /// checked nodes into a message; without it the tree is read-only.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(TreeEvent) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(on_event));
        self
    }

    /// Sets the message published with the node value on Enter or a double
    /// click; without it those open or close a branch.
    #[must_use]
    pub fn on_activate(mut self, on_activate: impl Fn(String) -> Message + 'a) -> Self {
        self.on_activate = Some(Box::new(on_activate));
        self
    }

    /// Sets the message published with a node value and its new, trimmed
    /// label when a rename is confirmed with a non-empty label that
    /// differs from the old one; F2 renames nothing without it.
    #[must_use]
    pub fn on_rename(mut self, on_rename: impl Fn(String, String) -> Message + 'a) -> Self {
        self.on_rename = Some(Box::new(on_rename));
        self
    }

    /// Sets the message published with the value of a branch without
    /// children that opens while it is neither loading nor loaded (see
    /// [`TreeState::start_loading`]); the app loads its children.
    #[must_use]
    pub fn on_load_children(mut self, on_load: impl Fn(String) -> Message + 'a) -> Self {
        self.on_load_children = Some(Box::new(on_load));
        self
    }

    /// Sets the builder of a row's content; the node label by default.
    #[must_use]
    pub fn item(mut self, item: impl Fn(&'a TreeNode<T>) -> Element<'a, Message> + 'a) -> Self {
        self.item = Some(Box::new(item));
        self
    }

    /// Sets the builder of the icon before the label, given whether the
    /// node is open; no icon shows when it returns `None`.
    #[must_use]
    pub fn icon(
        mut self,
        icon: impl Fn(&'a TreeNode<T>, bool) -> Option<Element<'a, Message>> + 'a,
    ) -> Self {
        self.icon = Some(Box::new(icon));
        self
    }

    /// Sets whether the rows have tristate check boxes, kept in
    /// [`TreeState::checked`] (false by default).
    #[must_use]
    pub fn checkable(mut self, checkable: bool) -> Self {
        self.checkable = checkable;
        self
    }

    /// Sets the text that filters the nodes; blank shows the whole tree.
    #[must_use]
    pub fn query(mut self, query: &str) -> Self {
        query.clone_into(&mut self.query);
        self.entries = OnceCell::new();
        self
    }

    /// Sets the row height; the control height of the density by default.
    #[must_use]
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = Some(height.max(1.0));
        self
    }

    /// Sets the indent of each level ([`DEFAULT_INDENT`] by default).
    #[must_use]
    pub fn indent(mut self, indent: f32) -> Self {
        self.indent = indent.max(0.0);
        self
    }

    /// Sets the width (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height (the available height by default); a shrinking
    /// height shows every row.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the reading direction: right to left mirrors the rows, the
    /// disclosure arrows and the left and right arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the disclosure arrows turn and the spinner moves (true
    /// by default); false shows the final state at once, for reduced
    /// motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Takes the tooltip of the loading spinner from `localizations`
    /// (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.loading_label = localizations.loading;
        self
    }

    /// Sets the widget [`Id`], used to focus the tree programmatically and
    /// to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// A shown node and its depth.
struct Entry<'a, T> {
    node: &'a TreeNode<T>,
    depth: usize,
}

impl<T> Clone for Entry<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Entry<'_, T> {}

/// The children of a tree node, a leaf when `None`.
fn kids<T>(node: &TreeNode<T>) -> &[TreeNode<T>] {
    node.children.as_deref().unwrap_or(&[])
}

/// The rows of a search for the folded `needle`: every node whose label
/// contains it, with its descendants and its ancestors.
fn filtered<'a, T>(nodes: &'a [TreeNode<T>], needle: &str) -> Vec<Entry<'a, T>> {
    fn all<'a, T>(node: &'a TreeNode<T>, depth: usize, rows: &mut Vec<Entry<'a, T>>) {
        rows.push(Entry { node, depth });

        for child in kids(node) {
            all(child, depth + 1, rows);
        }
    }

    fn go<'a, T>(
        nodes: &'a [TreeNode<T>],
        depth: usize,
        needle: &str,
        rows: &mut Vec<Entry<'a, T>>,
    ) {
        for node in nodes {
            if fold_text(node.label()).contains(needle) {
                all(node, depth, rows);
            } else {
                let mut below = Vec::new();
                go(kids(node), depth + 1, needle, &mut below);

                if !below.is_empty() {
                    rows.push(Entry { node, depth });
                    rows.extend(below);
                }
            }
        }
    }

    let mut rows = Vec::new();
    go(nodes, 0, needle, &mut rows);
    rows
}

/// The own messages of the elements inside a tree.
#[derive(Debug, Clone)]
enum Inner<Message> {
    /// A message of the app, from a row's content.
    App(Message),
    /// The text of the rename field changed.
    Draft(String),
    /// Enter was pressed in the rename field.
    Submit,
    /// A message that does nothing.
    Nothing,
}

/// An element inside a tree.
type Child<'a, Message> = IcedElement<'a, Inner<Message>, Theme, iced::Renderer>;

/// What the left button pressed.
#[derive(Debug, Clone, PartialEq)]
enum Press {
    /// The row of the node with this value.
    Row(String),
    /// The scroller, grabbed this far below its top.
    Scroller(f32),
}

/// The turn of a disclosure arrow.
#[derive(Debug, Clone, Copy)]
struct Turn {
    open: bool,
    started: Option<Instant>,
}

/// The tree state of a [`TreeView`].
#[derive(Debug, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which marks the active
    /// row.
    keyboard: bool,
    active: Option<String>,
    anchor: Option<String>,
    renaming: Option<String>,
    draft: String,
    /// Whether the rename field takes the focus after the next layout.
    focus_input: bool,
    typed: String,
    typed_at: Option<Instant>,
    clicks: DoubleClick<String>,
    pressed: Option<Press>,
    modifiers: keyboard::Modifiers,
    offset: f32,
    /// The node to scroll into view at the next layout.
    reveal: Option<String>,
    /// The row index of the first row laid out.
    first: usize,
    turns: HashMap<String, Turn>,
    now: Option<Instant>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// Focuses every focusable element it visits.
struct FocusAll;

impl Operation for FocusAll {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn operation::Focusable,
    ) {
        state.focus();
    }
}

/// Selects the whole text of every text input it visits.
struct SelectAll;

impl Operation for SelectAll {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
        state.select_all();
    }
}

/// Finds whether any element it visits has the focus.
#[derive(Default)]
struct Probe {
    focused: bool,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn operation::Focusable,
    ) {
        self.focused |= state.is_focused();
    }
}

/// Returns the trimmed `draft` when it is not empty and differs from
/// `label`: the label a confirmed rename publishes.
fn renamed(draft: &str, label: &str) -> Option<String> {
    let trimmed = draft.trim();

    (!trimmed.is_empty() && trimmed != label).then(|| trimmed.to_owned())
}

/// The height of the rows of `count` nodes with the list padding.
fn content_height(count: usize, row_height: f32) -> f32 {
    2.0 * LIST_PADDING + count as f32 * row_height
}

impl<'a, T, Message: Clone + 'a> TreeView<'a, T, Message> {
    fn enabled(&self) -> bool {
        self.on_event.is_some()
    }

    fn filtering(&self) -> bool {
        !self.query.trim().is_empty()
    }

    fn rtl(&self) -> bool {
        self.direction == ReadingDirection::RightToLeft
    }

    fn row_extent(&self) -> f32 {
        self.row_height
            .unwrap_or_else(crate::density::control_height)
    }

    /// The rows to show: the open part of the tree, or every node of the
    /// search result while filtering.
    fn entries(&self) -> &[Entry<'a, T>] {
        self.entries.get_or_init(|| {
            let query = self.query.trim();
            let collection: &'a TreeCollection<T> = self.collection;

            if query.is_empty() {
                collection
                    .visible(self.state.expanded())
                    .into_iter()
                    .map(|entry| Entry {
                        node: entry.node,
                        depth: entry.depth,
                    })
                    .collect()
            } else {
                filtered(collection.roots(), &fold_text(query))
            }
        })
    }

    /// The row index of the node `value`, when it is shown.
    fn index_of(&self, value: &str) -> Option<usize> {
        self.entries()
            .iter()
            .position(|entry| entry.node.value == value)
    }

    /// The check state of every node of a checkable tree, in one pass: a
    /// branch is checked when all of its leaves are, unchecked when none
    /// or it has no leaves, and mixed otherwise.
    fn check_state(&self, value: &str) -> Option<bool> {
        let checks = self.checks.get_or_init(|| {
            fn visit<'a, T>(
                node: &'a TreeNode<T>,
                checked: &HashSet<String>,
                states: &mut HashMap<&'a str, Option<bool>>,
            ) -> (usize, usize) {
                let Some(children) = &node.children else {
                    let on = checked.contains(&node.value);
                    states.insert(&node.value, Some(on));
                    return (usize::from(on), 1);
                };

                let (mut count, mut leaves) = (0, 0);

                for child in children {
                    let (c, l) = visit(child, checked, states);
                    count += c;
                    leaves += l;
                }

                states.insert(&node.value, mixed(count, leaves));
                (count, leaves)
            }

            let mut states = HashMap::new();
            let collection: &'a TreeCollection<T> = self.collection;

            for root in collection.roots() {
                visit(root, self.state.checked(), &mut states);
            }

            states
        });

        checks.get(value).copied().unwrap_or(Some(false))
    }

    /// Whether `node` shows as open; every branch is open while filtering.
    fn is_open(&self, node: &TreeNode<T>) -> bool {
        self.filtering() || self.state.is_expanded(&node.value)
    }

    fn publish(&self, shell: &mut Shell<'_, Message>, event: TreeEvent) {
        if let Some(on_event) = &self.on_event {
            shell.publish(on_event(event));
        }
    }

    /// Publishes the selection `values` when it differs from the current
    /// one.
    fn select(&self, shell: &mut Shell<'_, Message>, values: Vec<String>) {
        let current = self.state.selected();
        let same = values.len() == current.len() && values.iter().all(|v| current.contains(v));

        if !same {
            self.publish(shell, TreeEvent::Select(values));
        }
    }

    /// The values of the enabled rows between `from` and `to`, both
    /// included.
    fn range(&self, from: usize, to: usize) -> Vec<String> {
        let (low, high) = if from <= to { (from, to) } else { (to, from) };

        self.entries()[low..=high]
            .iter()
            .filter(|entry| !entry.node.disabled)
            .map(|entry| entry.node.value.clone())
            .collect()
    }

    /// Forgets the active row, the anchor and the rename when their node
    /// is no longer shown, and makes the first selected row, or else the
    /// first row, active when the tree has the focus without one.
    fn reconcile(&self, state: &mut State) {
        let shown = |value: &Option<String>| {
            value
                .as_deref()
                .is_some_and(|value| self.index_of(value).is_some())
        };

        if !shown(&state.active) {
            state.active = None;
        }
        if !shown(&state.anchor) {
            state.anchor = None;
        }
        if !shown(&state.renaming) {
            state.renaming = None;
            state.focus_input = false;
        }

        if state.focused && state.active.is_none() {
            let entries = self.entries();

            state.active = entries
                .iter()
                .find(|entry| self.state.is_selected(&entry.node.value))
                .or(entries.first())
                .map(|entry| entry.node.value.clone());
        }
    }

    /// Makes the node `value` active and reveals it; a single-selection
    /// tree selects it, and with `extend` a multiple-selection tree selects
    /// the enabled rows from the anchor to it.
    fn activate(
        &self,
        state: &mut State,
        value: &str,
        extend: bool,
        shell: &mut Shell<'_, Message>,
    ) {
        state.active = Some(value.to_owned());

        match self.state.selection_mode() {
            SelectionMode::Single => {
                self.select(shell, vec![value.to_owned()]);
                state.anchor = Some(value.to_owned());
            }
            SelectionMode::Multiple if extend => {
                if let Some(to) = self.index_of(value) {
                    let from = match state.anchor.as_deref().and_then(|a| self.index_of(a)) {
                        Some(from) => from,
                        None => {
                            state.anchor = Some(value.to_owned());
                            to
                        }
                    };

                    self.select(shell, self.range(from, to));
                }
            }
            _ if !extend => state.anchor = Some(value.to_owned()),
            _ => {}
        }

        state.reveal = Some(value.to_owned());
        shell.invalidate_layout();
        shell.request_redraw();
    }

    /// Opens or closes the branch `node`, asking for its children the
    /// first time an empty branch opens.
    fn set_expanded(&self, node: &TreeNode<T>, expanded: bool, shell: &mut Shell<'_, Message>) {
        if !node.is_branch() || self.filtering() {
            return;
        }

        if self.state.is_expanded(&node.value) != expanded {
            self.publish(
                shell,
                TreeEvent::Expand {
                    value: node.value.clone(),
                    expanded,
                },
            );
        }

        if expanded
            && kids(node).is_empty()
            && !self.state.is_loaded(&node.value)
            && !self.state.is_loading(&node.value)
            && let Some(on_load) = &self.on_load_children
        {
            shell.publish(on_load(node.value.clone()));
        }
    }

    /// Runs the primary action of `node`: [`TreeView::on_activate`], or
    /// opening or closing a branch.
    fn primary(&self, node: &TreeNode<T>, shell: &mut Shell<'_, Message>) {
        if let Some(on_activate) = &self.on_activate {
            shell.publish(on_activate(node.value.clone()));
        } else if node.is_branch() {
            self.set_expanded(node, !self.is_open(node), shell);
        }
    }

    /// Toggles the check box of `node`, or else its selection in a
    /// multiple-selection tree.
    fn toggle_space(&self, node: &TreeNode<T>, shell: &mut Shell<'_, Message>) {
        if self.checkable {
            self.publish(shell, TreeEvent::ToggleChecked(node.value.clone()));
        } else if self.state.selection_mode() == SelectionMode::Multiple {
            self.publish(shell, TreeEvent::ToggleSelected(node.value.clone()));
        }
    }

    /// Focuses the tree and activates `node`, toggling it with Control or
    /// Command and extending the selection with Shift in a
    /// multiple-selection tree.
    fn tap(&self, state: &mut State, node: &TreeNode<T>, shell: &mut Shell<'_, Message>) {
        if node.disabled {
            return;
        }

        state.focused = true;
        state.keyboard = false;
        let multiple = self.state.selection_mode() == SelectionMode::Multiple;
        let value = node.value.clone();

        if multiple && (state.modifiers.control() || state.modifiers.logo()) {
            state.active = Some(value.clone());
            self.publish(shell, TreeEvent::ToggleSelected(value.clone()));
            state.anchor = Some(value);
        } else if multiple && state.modifiers.shift() {
            self.activate(state, &value, true, shell);
        } else {
            state.active = Some(value.clone());
            state.anchor = Some(value.clone());

            if self.state.selection_mode() != SelectionMode::None {
                self.select(shell, vec![value]);
            }
        }

        shell.request_redraw();
    }

    /// Adds `character` to the typed prefix, which resets after a pause,
    /// and activates the next enabled row after `index` whose label starts
    /// with it; repeating one character cycles through its matches.
    fn type_ahead(
        &self,
        state: &mut State,
        character: &str,
        index: Option<usize>,
        shell: &mut Shell<'_, Message>,
    ) {
        let now = Instant::now();

        if state
            .typed_at
            .is_none_or(|at| now.saturating_duration_since(at) >= TYPE_AHEAD_RESET)
        {
            state.typed.clear();
        }
        state.typed_at = Some(now);

        let repeat = state.typed == character;
        if repeat {
            character.clone_into(&mut state.typed);
        } else {
            state.typed.push_str(character);
        }

        let needle = fold_text(&state.typed);
        let entries = self.entries();
        let length = entries.len() as isize;
        let index = index.map_or(-1, |index| index as isize);
        let start = if repeat || state.typed.chars().count() == 1 {
            index + 1
        } else {
            index
        };

        let found = (0..length)
            .map(|n| entries[(start + n).rem_euclid(length) as usize])
            .find(|entry| {
                !entry.node.disabled && fold_text(entry.node.label()).starts_with(&needle)
            });

        if let Some(entry) = found {
            self.activate(state, &entry.node.value, false, shell);
        }
    }

    /// Handles a key press of the focused tree, capturing the keys it
    /// takes.
    fn on_key(
        &self,
        state: &mut State,
        key: &keyboard::Key,
        modifiers: keyboard::Modifiers,
        text: Option<&str>,
        shell: &mut Shell<'_, Message>,
    ) {
        let entries = self.entries();
        if entries.is_empty() {
            return;
        }

        let shift = modifiers.shift();
        let command = modifiers.control() || modifiers.logo();
        let index = state
            .active
            .as_deref()
            .and_then(|value| self.index_of(value));
        let current = index.map(|index| entries[index]);
        let step = |from: isize, delta: isize| -> Option<&'a str> {
            let mut i = from + delta;

            while i >= 0 && (i as usize) < entries.len() {
                let node = entries[i as usize].node;
                if !node.disabled {
                    return Some(node.value.as_str());
                }
                i += delta;
            }

            None
        };
        let (close, open) = self.direction.arrows();
        let last = entries.len() as isize;

        match key {
            keyboard::Key::Named(key::Named::ArrowDown) => {
                if let Some(next) = step(index.map_or(-1, |i| i as isize), 1) {
                    self.activate(state, next, shift, shell);
                }
            }
            keyboard::Key::Named(key::Named::ArrowUp) => {
                if let Some(previous) = step(index.map_or(last, |i| i as isize), -1) {
                    self.activate(state, previous, shift, shell);
                }
            }
            keyboard::Key::Named(key::Named::Home) => {
                if let Some(first) = step(-1, 1) {
                    self.activate(state, first, shift, shell);
                }
            }
            keyboard::Key::Named(key::Named::End) => {
                if let Some(end) = step(last, -1) {
                    self.activate(state, end, shift, shell);
                }
            }
            keyboard::Key::Named(named) if *named == open => {
                if let (Some(index), Some(current)) = (index, current) {
                    let node = current.node;

                    if node.is_branch() && !self.is_open(node) {
                        self.set_expanded(node, true, shell);
                    } else if node.is_branch()
                        && let Some(child) = entries.get(index + 1)
                        && child.depth > current.depth
                    {
                        self.activate(state, &child.node.value, false, shell);
                    }
                }
            }
            keyboard::Key::Named(named) if *named == close => {
                if let Some(current) = current {
                    let node = current.node;

                    if node.is_branch() && self.is_open(node) && !self.filtering() {
                        self.set_expanded(node, false, shell);
                    } else if let Some(parent) = self.collection.parent_of(&node.value)
                        && self.index_of(parent).is_some()
                    {
                        self.activate(state, parent, false, shell);
                    }
                }
            }
            keyboard::Key::Named(key::Named::Enter) => {
                if let Some(current) = current {
                    self.primary(current.node, shell);
                }
            }
            keyboard::Key::Named(key::Named::Space) => {
                if let Some(current) = current {
                    self.toggle_space(current.node, shell);
                }
            }
            keyboard::Key::Character(c)
                if command
                    && c.eq_ignore_ascii_case("a")
                    && self.state.selection_mode() == SelectionMode::Multiple =>
            {
                let all = entries
                    .iter()
                    .filter(|entry| !entry.node.disabled)
                    .map(|entry| entry.node.value.clone())
                    .collect();
                self.select(shell, all);
            }
            keyboard::Key::Named(key::Named::F2)
                if self.on_rename.is_some() && current.is_some_and(|c| !c.node.disabled) =>
            {
                if let Some(current) = current {
                    state.renaming = Some(current.node.value.clone());
                    current.node.label().clone_into(&mut state.draft);
                    state.focus_input = true;
                    state.focused = false;
                    shell.invalidate_layout();
                }
            }
            _ => {
                let Some(character) = text else {
                    return;
                };

                if command
                    || character.trim().is_empty()
                    || character.chars().next().is_some_and(char::is_control)
                {
                    return;
                }

                self.type_ahead(state, character, index, shell);
            }
        }

        shell.request_redraw();
        state.keyboard = true;
        shell.capture_event();
    }

    /// Ends the rename, publishing the new label when `keep` and it
    /// changed, and focuses the tree again when `refocus`.
    fn finish_rename(
        &self,
        state: &mut State,
        keep: bool,
        refocus: bool,
        shell: &mut Shell<'_, Message>,
    ) {
        let Some(value) = state.renaming.take() else {
            return;
        };

        state.focus_input = false;

        if keep
            && let Some(node) = self.collection.find(&value)
            && let Some(label) = renamed(&state.draft, node.label())
            && let Some(on_rename) = &self.on_rename
        {
            shell.publish(on_rename(value, label));
        }

        if refocus {
            state.focused = true;
            state.keyboard = true;
        }

        shell.invalidate_layout();
        shell.request_redraw();
    }

    /// The bounds of the scrollbar and of its scroller in a tree with
    /// `bounds` and `content` height, when the rows overflow.
    fn scrollbar(
        &self,
        bounds: Rectangle,
        content: f32,
        offset: f32,
    ) -> Option<(Rectangle, Rectangle)> {
        if content <= bounds.height || bounds.height <= 0.0 {
            return None;
        }

        let x = if self.rtl() {
            bounds.x
        } else {
            bounds.x + bounds.width - SCROLLBAR_WIDTH
        };
        let rail = Rectangle::new(
            Point::new(x, bounds.y),
            Size::new(SCROLLBAR_WIDTH, bounds.height),
        );
        let length = (bounds.height * bounds.height / content)
            .max(sizes::MIN_POINTER_TARGET)
            .min(bounds.height);
        let max = content - bounds.height;
        let y = bounds.y + (bounds.height - length) * (offset / max).clamp(0.0, 1.0);
        let scroller = Rectangle::new(
            Point::new(x + (SCROLLBAR_WIDTH - SCROLLER_WIDTH) / 2.0, y),
            Size::new(SCROLLER_WIDTH, length),
        );

        Some((rail, scroller))
    }

    /// The row index under the point `y` of a tree with `bounds` scrolled
    /// by `offset`.
    fn row_at(&self, bounds: Rectangle, offset: f32, y: f32) -> Option<usize> {
        let position = y - bounds.y - LIST_PADDING + offset;

        if position < 0.0 {
            return None;
        }

        let index = (position / self.row_extent()) as usize;
        (index < self.entries().len()).then_some(index)
    }

    /// The layouts of the parts of a row, in reading order: the disclosure
    /// column first, then the check box of a checkable tree.
    fn parts<'l>(&self, row: Layout<'l>) -> Vec<Layout<'l>> {
        let mut parts: Vec<Layout<'l>> = row.children().collect();

        if self.rtl() {
            parts.reverse();
        }

        parts
    }

    /// Builds the row of `entry`: disclosure, check box, icon and label,
    /// or a blank where the rename field goes.
    fn build_row(
        &self,
        entry: Entry<'a, T>,
        renaming: bool,
        width: f32,
        height: f32,
    ) -> Child<'a, Message> {
        let node = entry.node;
        let open = self.is_open(node);

        let disclosure: Child<'a, Message> =
            if node.is_branch() && self.state.is_loading(&node.value) {
                let spinner = progress::circular_indeterminate()
                    .size(SPINNER_SIZE)
                    .stroke_width(SPINNER_STROKE)
                    .animated(self.animated);

                container(tooltip(spinner, self.loading_label))
                    .center(DISCLOSURE_WIDTH)
                    .into()
            } else {
                Space::new()
                    .width(DISCLOSURE_WIDTH)
                    .height(DISCLOSURE_WIDTH)
                    .into()
            };

        let mut items = vec![disclosure];

        if self.checkable {
            items.push(
                toggles::checkbox(self.check_state(&node.value))
                    .tristate(true)
                    .on_change(|_| Inner::Nothing)
                    .enabled(!node.disabled)
                    .into_toggle()
                    .passive()
                    .into(),
            );
        }

        if let Some(icon) = self.icon.as_ref().and_then(|icon| icon(node, open)) {
            items.push(icon.map(Inner::App));
        }

        let label: Child<'a, Message> = if renaming {
            Space::new().width(Length::Fill).into()
        } else if let Some(item) = &self.item {
            item(node).map(Inner::App)
        } else if self.filtering() {
            highlighted_text(node.label().to_owned(), self.query.trim())
                .wrapping(Wrapping::None)
                .into()
        } else {
            styled_text(node.label().to_owned(), typography::BODY_MEDIUM)
                .wrapping(Wrapping::None)
                .into()
        };

        let align = if self.rtl() {
            alignment::Horizontal::Right
        } else {
            alignment::Horizontal::Left
        };
        items.push(
            container(label)
                .width(Length::Fill)
                .align_x(align)
                .clip(true)
                .into(),
        );

        if self.rtl() {
            items.reverse();
        }

        let start = spacing::XS + entry.depth as f32 * self.indent;
        let end = spacing::SM;
        let (left, right) = if self.rtl() {
            (end, start)
        } else {
            (start, end)
        };

        Row::with_children(items)
            .spacing(spacing::XS)
            .align_y(alignment::Vertical::Center)
            .padding(Padding {
                top: 0.0,
                right,
                bottom: 0.0,
                left,
            })
            .width(width)
            .height(height)
            .into()
    }

    /// Builds the rename field.
    fn rename_field(draft: &str) -> Child<'a, Message> {
        let style = typography::BODY_MEDIUM;

        text_input::TextInput::new("", draft)
            .on_input(Inner::Draft)
            .on_submit(Inner::Submit)
            .padding(0)
            .size(style.size)
            .line_height(LineHeight::Absolute(Pixels(style.line_height)))
            .font(crate::font::inter(style.weight))
            .width(Length::Fill)
            .class(InputClass::Custom(Box::new(bare_input)))
            .into()
    }

    /// Draws the disclosure arrow of a row in `bounds`, turned by `turn`
    /// (0 closed, 1 open).
    fn draw_arrow(
        &self,
        renderer: &mut iced::Renderer,
        bounds: Rectangle,
        turn: f32,
        color: Color,
    ) {
        let unit = ARROW_SIZE / 24.0;
        let center = bounds.center();
        let origin = Point::new(center.x - ARROW_SIZE / 2.0, center.y - ARROW_SIZE / 2.0);
        let mut frame = Frame::new(renderer, Size::new(ARROW_SIZE, ARROW_SIZE));
        let middle = frame.center();
        let (tip, tail) = if self.rtl() { (9.0, 15.0) } else { (15.0, 9.0) };
        let path = Path::new(|builder| {
            builder.move_to(Point::new(tail * unit, 6.0 * unit));
            builder.line_to(Point::new(tip * unit, 12.0 * unit));
            builder.line_to(Point::new(tail * unit, 18.0 * unit));
        });
        let angle = if self.rtl() { -turn } else { turn } * FRAC_PI_2;

        frame.with_save(|frame| {
            frame.translate(Vector::new(middle.x, middle.y));
            frame.rotate(angle);
            frame.translate(Vector::new(-middle.x, -middle.y));
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(color)
                    .with_width(2.0 * unit)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            );
        });

        draw_frame(renderer, frame, origin);
    }
}

/// Draws the geometry of `frame` with its origin at `origin`.
fn draw_frame<R: geometry::Renderer>(renderer: &mut R, frame: Frame<R>, origin: Point) {
    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

/// The style of the rename field: the text field style without its fill
/// and outline.
fn bare_input(theme: &Theme, status: text_input::Status) -> text_input::Style {
    text_input::Style {
        background: Color::TRANSPARENT.into(),
        border: Border::default(),
        ..text_input::Catalog::style(theme, &InputClass::Default, status)
    }
}

impl<'a, T, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer>
    for TreeView<'a, T, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut Tree) {
        // The rows are built and reconciled during the layout.
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let row_height = self.row_extent();
        let count = self.entries().len();
        let content = content_height(count, row_height);
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(
            self.width,
            self.height,
            Size::new(limits.max().width, content),
        );

        let (first, last, offset, renaming, focus_input, draft) = {
            let state = tree.state.downcast_mut::<State>();
            self.reconcile(state);

            let max = (content - size.height).max(0.0);

            if let Some(value) = state.reveal.take()
                && let Some(index) = self.index_of(&value)
            {
                let top = LIST_PADDING + index as f32 * row_height;
                let bottom = top + row_height;

                if top - LIST_PADDING < state.offset {
                    state.offset = (top - LIST_PADDING).max(0.0);
                } else if bottom + LIST_PADDING > state.offset + size.height {
                    state.offset = (bottom + LIST_PADDING - size.height).min(max);
                }
            }

            state.offset = state.offset.clamp(0.0, max);

            let first = (((state.offset - LIST_PADDING) / row_height)
                .floor()
                .max(0.0) as usize)
                .min(count);
            let last = (((state.offset + size.height - LIST_PADDING) / row_height)
                .ceil()
                .max(0.0) as usize)
                .min(count);
            state.first = first;

            let renaming = state
                .renaming
                .as_deref()
                .and_then(|value| self.index_of(value))
                .filter(|index| (first..last).contains(index));

            if renaming.is_none() {
                state.renaming = None;
                state.focus_input = false;
            }

            (
                first,
                last,
                state.offset,
                renaming,
                std::mem::take(&mut state.focus_input),
                state.draft.clone(),
            )
        };

        let width = (size.width - 2.0 * spacing::XS).max(0.0);
        let shown: Vec<Entry<'a, T>> = self.entries()[first..last].to_vec();
        let mut children = Vec::with_capacity(shown.len() + 1);

        children.push(match renaming {
            Some(_) => Self::rename_field(&draft),
            None => Space::new().into(),
        });

        for (k, entry) in shown.iter().enumerate() {
            children.push(self.build_row(*entry, renaming == Some(first + k), width, row_height));
        }

        self.children = children;
        tree.diff_children(&self.children);

        let row_limits = layout::Limits::new(Size::ZERO, Size::new(width, row_height));
        let mut rows = Vec::with_capacity(shown.len());

        for (k, (child, child_tree)) in self
            .children
            .iter_mut()
            .zip(tree.children.iter_mut())
            .enumerate()
            .skip(1)
        {
            let y = LIST_PADDING + (first + k - 1) as f32 * row_height - offset;
            let node = child
                .as_widget_mut()
                .layout(child_tree, renderer, &row_limits)
                .move_to(Point::new(spacing::XS, y));
            rows.push(node);
        }

        let slot = match renaming.and_then(|index| rows.get(index - first)) {
            Some(row) => {
                let label = if self.rtl() {
                    row.children().first()
                } else {
                    row.children().last()
                }
                .map_or(Rectangle::default(), layout::Node::bounds);
                let origin = row.bounds().position() + Vector::new(label.x, 0.0);
                let field_limits =
                    layout::Limits::new(Size::ZERO, Size::new(label.width, row_height));
                let node = self.children[0].as_widget_mut().layout(
                    &mut tree.children[0],
                    renderer,
                    &field_limits,
                );
                let y = origin.y + (row_height - node.size().height) / 2.0;

                node.move_to(Point::new(origin.x, y))
            }
            None => self.children[0].as_widget_mut().layout(
                &mut tree.children[0],
                renderer,
                &layout::Limits::new(Size::ZERO, Size::ZERO),
            ),
        };

        if focus_input && renaming.is_some() {
            let field = &mut self.children[0];
            let field_tree = &mut tree.children[0];

            field
                .as_widget_mut()
                .operate(field_tree, Layout::new(&slot), renderer, &mut FocusAll);
            field
                .as_widget_mut()
                .operate(field_tree, Layout::new(&slot), renderer, &mut SelectAll);
        }

        let mut nodes = Vec::with_capacity(rows.len() + 1);
        nodes.push(slot);
        nodes.extend(rows);

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), bounds, state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), bounds);
        }

        operation.traverse(&mut |operation| {
            for ((child, child_tree), child_layout) in self
                .children
                .iter_mut()
                .zip(children.iter_mut())
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(child_tree, child_layout, renderer, operation);
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let Tree {
            state: tree_state,
            children,
            ..
        } = tree;
        let state = tree_state.downcast_mut::<State>();
        let enabled = self.enabled();

        if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            state.modifiers = *modifiers;
        }

        // Escape cancels a rename before the field sees it.
        if state.renaming.is_some()
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            }) = event
        {
            self.finish_rename(state, false, true, shell);
            shell.capture_event();
            return;
        }

        let inner_cursor = if cursor.is_over(bounds) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        };
        let inner_viewport = bounds.intersection(viewport).unwrap_or(bounds);
        let mut messages = Vec::new();
        let captured = {
            let mut local = Shell::new(&mut messages);

            for ((child, child_tree), child_layout) in self
                .children
                .iter_mut()
                .zip(children.iter_mut())
                .zip(layout.children())
            {
                child.as_widget_mut().update(
                    child_tree,
                    event,
                    child_layout,
                    inner_cursor,
                    renderer,
                    clipboard,
                    &mut local,
                    &inner_viewport,
                );
            }

            shell.request_redraw_at(local.redraw_request());
            shell.request_input_method(local.input_method());
            if local.is_layout_invalid() {
                shell.invalidate_layout();
            }
            if local.are_widgets_invalid() {
                shell.invalidate_widgets();
            }

            local.is_event_captured()
        };

        if captured {
            shell.capture_event();
        }

        for message in messages {
            match message {
                Inner::App(message) => shell.publish(message),
                Inner::Draft(draft) => {
                    state.draft = draft;
                    shell.invalidate_layout();
                }
                Inner::Submit => self.finish_rename(state, true, true, shell),
                Inner::Nothing => {}
            }
        }

        // Leaving the rename field confirms the rename.
        if state.renaming.is_some()
            && !state.focus_input
            && let (Some(field), Some(field_tree), Some(field_layout)) = (
                self.children.first_mut(),
                children.first_mut(),
                layout.children().next(),
            )
        {
            let mut probe = Probe::default();
            field
                .as_widget_mut()
                .operate(field_tree, field_layout, renderer, &mut probe);

            if !probe.focused {
                self.finish_rename(state, true, false, shell);
            }
        }

        self.reconcile(state);

        let row_height = self.row_extent();
        let content = content_height(self.entries().len(), row_height);

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);
            let rows = self.children.len().saturating_sub(1);
            let mut animating = false;

            for entry in self.entries().iter().skip(state.first).take(rows) {
                if !entry.node.is_branch() {
                    continue;
                }

                let open = self.is_open(entry.node);
                let turn = state.turns.entry(entry.node.value.clone()).or_insert(Turn {
                    open,
                    started: None,
                });

                if turn.open != open {
                    *turn = Turn {
                        open,
                        started: self.animated.then_some(*now),
                    };
                }

                animating |= turn
                    .started
                    .is_some_and(|started| now.saturating_duration_since(started) < TURN_DURATION);
            }

            if animating {
                shell.request_redraw();
            }
        }

        match event {
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.is_over(bounds) && !shell.is_event_captured() {
                    let dy = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => -y * WHEEL_LINE,
                        mouse::ScrollDelta::Pixels { y, .. } => -y,
                    };
                    let max = (content - bounds.height).max(0.0);
                    let offset = (state.offset + dy).clamp(0.0, max);

                    if offset != state.offset {
                        state.offset = offset;
                        shell.invalidate_layout();
                        shell.request_redraw();
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let Some(position) = cursor.position_over(bounds) else {
                    if state.focused {
                        state.focused = false;
                        shell.request_redraw();
                    }
                    return;
                };

                if captured {
                    return;
                }

                if let Some((rail, scroller)) = self.scrollbar(bounds, content, state.offset)
                    && rail.contains(position)
                {
                    let grab = if scroller.contains(position) {
                        position.y - scroller.y
                    } else {
                        scroller.height / 2.0
                    };
                    state.pressed = Some(Press::Scroller(grab));
                    self.drag_scroller(state, bounds, content, position.y, grab, shell);
                    shell.capture_event();
                    return;
                }

                if !enabled {
                    return;
                }

                let Some(index) = self.row_at(bounds, state.offset, position.y) else {
                    return;
                };
                let Some(row) = index
                    .checked_sub(state.first)
                    .and_then(|k| layout.children().nth(k + 1))
                else {
                    return;
                };
                let node = self.entries()[index].node;
                let parts = self.parts(row);
                let within = |part: Option<&Layout<'_>>| {
                    part.is_some_and(|part| {
                        let b = part.bounds();
                        position.x >= b.x && position.x < b.x + b.width
                    })
                };

                if node.is_branch() && !self.filtering() && within(parts.first()) {
                    if !self.state.is_loading(&node.value) {
                        self.set_expanded(node, !self.is_open(node), shell);
                    }
                } else if self.checkable && !node.disabled && within(parts.get(1)) {
                    self.publish(shell, TreeEvent::ToggleChecked(node.value.clone()));
                } else {
                    state.pressed = Some(Press::Row(node.value.clone()));
                }

                shell.capture_event();
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some(Press::Scroller(grab)) = state.pressed {
                    self.drag_scroller(state, bounds, content, position.y, grab, shell);
                    shell.capture_event();
                } else if cursor.is_over(bounds) {
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => match state.pressed.take() {
                Some(Press::Row(value)) => {
                    let index = cursor
                        .position_over(bounds)
                        .and_then(|position| self.row_at(bounds, state.offset, position.y));

                    if let Some(index) = index
                        && let node = self.entries()[index].node
                        && node.value == value
                    {
                        self.tap(state, node, shell);

                        if !node.disabled && state.clicks.click(value, Instant::now()) {
                            self.primary(node, shell);
                        }
                        shell.capture_event();
                    }
                }
                Some(Press::Scroller(_)) => shell.request_redraw(),
                None => {}
            },
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if matches!(state.pressed, Some(Press::Row(_))) {
                    state.pressed = None;
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                text,
                ..
            }) if enabled
                && state.focused
                && state.renaming.is_none()
                && !shell.is_event_captured() =>
            {
                self.on_key(state, key, *modifiers, text.as_deref(), shell);
            }
            _ => {}
        }

        if !enabled {
            state.focused = false;
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let bounds = layout.bounds();
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };
        let entries = self.entries();
        let inner_cursor = if cursor.is_over(bounds) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        };

        renderer.with_layer(clip, |renderer| {
            let mut layouts = layout.children();
            let slot = layouts.next();

            for (k, ((row, row_tree), row_layout)) in self
                .children
                .iter()
                .zip(&tree.children)
                .skip(1)
                .zip(layouts)
                .enumerate()
            {
                let Some(entry) = entries.get(state.first + k) else {
                    continue;
                };
                let node = entry.node;
                let rect = row_layout.bounds();
                let selected = self.state.is_selected(&node.value);
                let active = state.focused
                    && state.keyboard
                    && state.active.as_deref() == Some(node.value.as_str());

                if selected || active {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: rect,
                            border: Border {
                                color: roles.semantic.primary_text,
                                width: if active {
                                    sizes::FOCUS_BORDER_WIDTH
                                } else {
                                    0.0
                                },
                                radius: radius::SMALL.into(),
                            },
                            ..renderer::Quad::default()
                        },
                        if selected {
                            roles.secondary_container
                        } else {
                            Color::TRANSPARENT
                        },
                    );
                }

                let foreground = if node.disabled {
                    roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
                } else if selected {
                    roles.on_secondary_container
                } else {
                    roles.on_surface
                };

                row.as_widget().draw(
                    row_tree,
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: foreground,
                    },
                    row_layout,
                    inner_cursor,
                    &clip,
                );

                if node.is_branch()
                    && !self.state.is_loading(&node.value)
                    && let Some(disclosure) = self.parts(row_layout).first()
                {
                    let open = self.is_open(node);
                    let turn = match (state.turns.get(&node.value), state.now) {
                        (Some(turn), Some(now)) if turn.open == open => {
                            turn.started.map_or(f32::from(u8::from(open)), |started| {
                                let progress =
                                    (now.saturating_duration_since(started).as_secs_f32()
                                        / TURN_DURATION.as_secs_f32())
                                    .clamp(0.0, 1.0);

                                if open { progress } else { 1.0 - progress }
                            })
                        }
                        _ => f32::from(u8::from(open)),
                    };

                    self.draw_arrow(renderer, disclosure.bounds(), turn, foreground);
                }
            }

            if let (Some(field), Some(field_tree), Some(slot)) =
                (self.children.first(), tree.children.first(), slot)
                && state.renaming.is_some()
            {
                field.as_widget().draw(
                    field_tree,
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: roles.on_surface,
                    },
                    slot,
                    cursor,
                    &clip,
                );
            }

            let content = content_height(entries.len(), self.row_extent());

            if let Some((rail, scroller)) = self.scrollbar(bounds, content, state.offset) {
                let dragged = matches!(state.pressed, Some(Press::Scroller(_)));
                let hovered = cursor.is_over(rail);
                let status = if dragged {
                    ScrollStatus::Dragged {
                        is_horizontal_scrollbar_dragged: false,
                        is_vertical_scrollbar_dragged: true,
                        is_horizontal_scrollbar_disabled: true,
                        is_vertical_scrollbar_disabled: false,
                    }
                } else if cursor.is_over(bounds) {
                    ScrollStatus::Hovered {
                        is_horizontal_scrollbar_hovered: false,
                        is_vertical_scrollbar_hovered: hovered,
                        is_horizontal_scrollbar_disabled: true,
                        is_vertical_scrollbar_disabled: false,
                    }
                } else {
                    ScrollStatus::Active {
                        is_horizontal_scrollbar_disabled: true,
                        is_vertical_scrollbar_disabled: false,
                    }
                };
                let style = scrollable::Catalog::style(
                    theme,
                    &crate::theme::scrollable::Class::Default,
                    status,
                );
                let scroller_style = style.vertical_rail.scroller;

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: scroller,
                        border: scroller_style.border,
                        ..renderer::Quad::default()
                    },
                    scroller_style.background,
                );
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let Some(position) = cursor.position_over(bounds) else {
            return mouse::Interaction::default();
        };

        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            let interaction = child.as_widget().mouse_interaction(
                child_tree,
                child_layout,
                cursor,
                viewport,
                renderer,
            );

            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        let content = content_height(self.entries().len(), self.row_extent());

        if !self.enabled()
            || state.pressed.is_some()
            || self
                .scrollbar(bounds, content, state.offset)
                .is_some_and(|(rail, _)| rail.contains(position))
        {
            return mouse::Interaction::default();
        }

        match self.row_at(bounds, state.offset, position.y) {
            Some(index) if !self.entries()[index].node.disabled => mouse::Interaction::Pointer,
            Some(index) => {
                let node = self.entries()[index].node;
                let row = index
                    .checked_sub(state.first)
                    .and_then(|k| layout.children().nth(k + 1));
                let over_arrow = row.is_some_and(|row| {
                    self.parts(row).first().is_some_and(|part| {
                        let b = part.bounds();
                        position.x >= b.x && position.x < b.x + b.width
                    })
                });

                if node.is_branch() && !self.filtering() && over_arrow {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                }
            }
            None => mouse::Interaction::default(),
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let content = overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )?;

        Some(overlay::Element::new(Box::new(AppOverlay { content })))
    }
}

/// The overlay of the rows of a tree, such as a tooltip, publishing only
/// the app messages of its content.
struct AppOverlay<'b, Message> {
    content: overlay::Element<'b, Inner<Message>, Theme, iced::Renderer>,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for AppOverlay<'_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        self.content.as_overlay_mut().layout(renderer, bounds)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content
            .as_overlay()
            .draw(renderer, theme, style, layout, cursor);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut messages = Vec::new();
        {
            let mut local = Shell::new(&mut messages);
            self.content
                .as_overlay_mut()
                .update(event, layout, cursor, renderer, clipboard, &mut local);

            if local.is_event_captured() {
                shell.capture_event();
            }
            shell.request_redraw_at(local.redraw_request());
            if local.is_layout_invalid() {
                shell.invalidate_layout();
            }
            if local.are_widgets_invalid() {
                shell.invalidate_widgets();
            }
        }

        for message in messages {
            if let Inner::App(message) = message {
                shell.publish(message);
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content
            .as_overlay()
            .mouse_interaction(layout, cursor, renderer)
    }

    fn index(&self) -> f32 {
        self.content.as_overlay().index()
    }
}

impl<'a, T, Message: Clone + 'a> TreeView<'a, T, Message> {
    /// Moves the scroller so its top sits `grab` above the pointer at `y`.
    fn drag_scroller(
        &self,
        state: &mut State,
        bounds: Rectangle,
        content: f32,
        y: f32,
        grab: f32,
        shell: &mut Shell<'_, Message>,
    ) {
        let Some((_, scroller)) = self.scrollbar(bounds, content, state.offset) else {
            return;
        };
        let travel = bounds.height - scroller.height;
        let max = content - bounds.height;

        if travel > 0.0 {
            let offset = ((y - grab - bounds.y) / travel * max).clamp(0.0, max);

            if offset != state.offset {
                state.offset = offset;
                shell.invalidate_layout();
                shell.request_redraw();
            }
        }
    }
}

impl<'a, T: 'a, Message: Clone + 'a> From<TreeView<'a, T, Message>> for Element<'a, Message> {
    fn from(tree: TreeView<'a, T, Message>) -> Self {
        IcedElement::new(tree)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files() -> TreeCollection<()> {
        TreeCollection::new(vec![
            TreeNode::branch(
                "src",
                vec![
                    TreeNode::leaf("src/main.rs").with_label("main.rs"),
                    TreeNode::branch(
                        "src/widgets",
                        vec![
                            TreeNode::leaf("src/widgets/a.rs").with_label("a.rs"),
                            TreeNode::leaf("src/widgets/b.rs").with_label("b.rs"),
                        ],
                    )
                    .with_label("widgets"),
                ],
            ),
            TreeNode::leaf("README.md"),
        ])
        .expect("unique values")
    }

    #[test]
    fn check_states_cascade() {
        let files = files();
        let mut state = TreeState::default();

        state.toggle_checked(&files, "src/widgets");
        assert_eq!(state.check_state_of(&files, "src"), None);
        assert_eq!(state.check_state_of(&files, "src/widgets"), Some(true));

        state.toggle_checked(&files, "src");
        assert_eq!(state.check_state_of(&files, "src"), Some(true));
        state.toggle_checked(&files, "src");
        assert!(state.checked().is_empty());
    }

    #[test]
    fn a_search_keeps_ancestors_and_descendants() {
        let files = files();
        let rows: Vec<&str> = filtered(files.roots(), &fold_text("WIDG"))
            .iter()
            .map(|entry| entry.node.value.as_str())
            .collect();

        assert_eq!(
            rows,
            ["src", "src/widgets", "src/widgets/a.rs", "src/widgets/b.rs"]
        );
    }

    #[test]
    fn single_selection_keeps_one_value() {
        let mut state = TreeState::new(SelectionMode::Single).with_selected(["a", "b"]);
        assert_eq!(state.selected().len(), 1);

        state.toggle_selected("c");
        assert!(state.is_selected("c"));

        let mut none = TreeState::new(SelectionMode::None);
        none.set_selected(["a"]);
        assert!(none.selected().is_empty());
    }

    #[test]
    fn a_failed_load_closes_the_branch() {
        let mut state = TreeState::default().with_expanded(["remote"]);

        state.start_loading("remote");
        assert!(state.is_loading("remote"));
        state.finish_loading("remote", false);
        assert!(!state.is_expanded("remote"));
        assert!(!state.is_loaded("remote"));

        state.finish_loading("remote", true);
        assert!(state.is_loaded("remote"));
    }

    #[test]
    fn renames_need_a_new_non_empty_label() {
        assert_eq!(renamed(" READ.md ", "README.md"), Some("READ.md".into()));
        assert_eq!(renamed("README.md", "README.md"), None);
        assert_eq!(renamed("  ", "README.md"), None);
    }
}
