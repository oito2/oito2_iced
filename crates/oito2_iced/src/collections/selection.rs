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

//! The selected values of a list, with the desktop selection gestures.

use std::collections::HashSet;

use super::list::ListCollection;

/// How many items a [`SelectionController`] can select.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SelectionMode {
    /// Nothing can be selected.
    None,
    /// One item at most.
    #[default]
    Single,
    /// Any number of items.
    Multiple,
}

/// The behavior options of a [`SelectionController`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionOptions {
    /// How many items can be selected.
    pub mode: SelectionMode,
    /// Whether a selected item can be deselected by clicking it again in
    /// single mode, or by toggling it.
    pub deselectable: bool,
    /// Whether replacing the collection clears the selection; otherwise only
    /// the values that are no longer enabled items are dropped.
    pub reset_on_collection_change: bool,
}

impl Default for SelectionOptions {
    fn default() -> Self {
        Self {
            mode: SelectionMode::Single,
            deselectable: true,
            reset_on_collection_change: false,
        }
    }
}

/// The selected values of a [`ListCollection`], with the desktop selection
/// gestures: plain click, Control/Command click to toggle and Shift click to
/// extend a range from the anchor.
///
/// Disabled items and values outside the collection are never selected.
/// [`selected_values`](Self::selected_values) keeps the collection order.
///
/// The controller is a plain struct kept in the application model. Methods
/// that may change the selection return `true` when something changed, so
/// the caller knows whether to refresh.
#[derive(Debug, Clone)]
pub struct SelectionController<T> {
    collection: ListCollection<T>,
    options: SelectionOptions,
    selected: HashSet<String>,
    anchor: Option<String>,
}

impl<T: 'static> SelectionController<T> {
    /// Creates an empty selection over `collection`.
    #[must_use]
    pub fn new(collection: ListCollection<T>, options: SelectionOptions) -> Self {
        Self {
            collection,
            options,
            selected: HashSet::new(),
            anchor: None,
        }
    }

    /// Creates a selection over `collection` with `initial_values`.
    ///
    /// Initial values that [`can_select`](Self::can_select) rejects are
    /// dropped, and in single mode only the first remaining value in
    /// collection order is kept.
    #[must_use]
    pub fn with_values<S: AsRef<str>>(
        collection: ListCollection<T>,
        options: SelectionOptions,
        initial_values: impl IntoIterator<Item = S>,
    ) -> Self {
        let mut controller = Self::new(collection, options);
        let valid: Vec<String> = initial_values
            .into_iter()
            .filter(|value| controller.can_select(value.as_ref()))
            .map(|value| value.as_ref().to_owned())
            .collect();
        controller.selected.extend(valid);
        if options.mode == SelectionMode::Single
            && controller.selected.len() > 1
            && let Some(first) = controller.first_selected_value()
        {
            controller.selected.clear();
            controller.selected.insert(first);
        }
        controller
    }

    /// The selection options.
    #[must_use]
    pub fn options(&self) -> SelectionOptions {
        self.options
    }

    /// The collection the values belong to.
    #[must_use]
    pub fn collection(&self) -> &ListCollection<T> {
        &self.collection
    }

    /// Replaces the collection. The selection is cleared when
    /// `reset_on_collection_change` is set; otherwise the values that are no
    /// longer enabled items are dropped. The anchor is forgotten when its
    /// item is gone. Returns whether the selection may have changed.
    pub fn set_collection(&mut self, collection: ListCollection<T>) -> bool {
        self.collection = collection;
        let before = self.selected.len();
        if self.options.reset_on_collection_change {
            self.selected.clear();
        } else {
            let collection = &self.collection;
            self.selected.retain(|value| collection.is_enabled(value));
        }
        if self
            .anchor
            .as_deref()
            .is_some_and(|anchor| !self.collection.has(anchor))
        {
            self.anchor = None;
        }
        self.selected.len() != before || self.options.reset_on_collection_change
    }

    /// The selected values, in collection order.
    #[must_use]
    pub fn selected_values(&self) -> Vec<String> {
        let mut list: Vec<String> = self.selected.iter().cloned().collect();
        list.sort_by_key(|value| self.collection.index_of(value));
        list
    }

    /// Whether nothing is selected.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// The first selected value in collection order, or `None`.
    #[must_use]
    pub fn first_selected_value(&self) -> Option<String> {
        self.selected_values().into_iter().next()
    }

    /// The last selected value in collection order, or `None`.
    #[must_use]
    pub fn last_selected_value(&self) -> Option<String> {
        self.selected_values().pop()
    }

    /// The value range selections extend from: the last value selected by
    /// [`select`](Self::select), [`toggle`](Self::toggle) or
    /// [`replace`](Self::replace).
    #[must_use]
    pub fn anchor_value(&self) -> Option<&str> {
        self.anchor.as_deref()
    }

    /// Whether `value` is selected.
    #[must_use]
    pub fn is_selected(&self, value: &str) -> bool {
        self.selected.contains(value)
    }

    /// Whether `value` can be selected: it is an enabled item and the mode
    /// allows selection.
    #[must_use]
    pub fn can_select(&self, value: &str) -> bool {
        self.options.mode != SelectionMode::None && self.collection.is_enabled(value)
    }

    /// Whether every enabled item is selected.
    #[must_use]
    pub fn is_all_selected(&self) -> bool {
        let mut enabled = self
            .collection
            .values()
            .iter()
            .filter(|value| self.collection.is_enabled(value))
            .peekable();
        enabled.peek().is_some() && enabled.all(|value| self.selected.contains(value))
    }

    /// Whether some, but not all, enabled items are selected; drives a mixed
    /// "select all" check box.
    #[must_use]
    pub fn is_some_selected(&self) -> bool {
        !self.selected.is_empty() && !self.is_all_selected()
    }

    /// Selects `value` and makes it the anchor; does nothing when
    /// [`can_select`](Self::can_select) rejects it.
    ///
    /// In multiple mode it adds `value` to the selection. In single mode it
    /// replaces the selection; selecting the selected value again deselects
    /// it when `deselectable` and `force_toggle` are set. Returns whether
    /// the selection changed.
    pub fn select(&mut self, value: &str, force_toggle: bool) -> bool {
        if !self.can_select(value) {
            return false;
        }
        self.anchor = Some(value.to_owned());
        if self.options.mode == SelectionMode::Single {
            if self.selected.contains(value) {
                if force_toggle && self.options.deselectable {
                    self.selected.clear();
                    return true;
                }
                return false;
            }
            self.selected.clear();
            self.selected.insert(value.to_owned());
            true
        } else {
            self.selected.insert(value.to_owned())
        }
    }

    /// Deselects `value`; returns whether it was selected.
    pub fn deselect(&mut self, value: &str) -> bool {
        self.selected.remove(value)
    }

    /// Selects `value` when it isn't selected, otherwise deselects it if
    /// `deselectable`. `value` becomes the anchor when the selection
    /// changes. Returns whether the selection changed.
    pub fn toggle(&mut self, value: &str) -> bool {
        if self.selected.contains(value) {
            if !self.options.deselectable {
                return false;
            }
            self.anchor = Some(value.to_owned());
            self.deselect(value)
        } else {
            self.select(value, false)
        }
    }

    /// Makes `value` the only selected value and the anchor; `None` clears
    /// the selection. Does nothing when [`can_select`](Self::can_select)
    /// rejects `value`. Returns whether the selection changed.
    pub fn replace(&mut self, value: Option<&str>) -> bool {
        if value.is_some_and(|value| !self.can_select(value)) {
            return false;
        }
        self.anchor = value.map(str::to_owned);
        self.set_selected(value.map(str::to_owned))
    }

    /// Replaces the selection with `values`; returns whether it changed.
    fn set_selected(&mut self, values: impl IntoIterator<Item = String>) -> bool {
        let next: HashSet<String> = values.into_iter().collect();
        if next == self.selected {
            return false;
        }
        self.selected = next;
        true
    }

    /// Selects the enabled values from `anchor` to `target`, both included,
    /// replacing the previous selection and keeping `anchor` as the anchor.
    /// Does nothing when either value isn't in the collection. In single and
    /// none modes it behaves as [`select`](Self::select) with `target`.
    /// Returns whether the selection may have changed.
    pub fn extend(&mut self, anchor: &str, target: &str) -> bool {
        if self.options.mode != SelectionMode::Multiple {
            return self.select(target, false);
        }
        let (Some(a), Some(b)) = (
            self.collection.index_of(anchor),
            self.collection.index_of(target),
        ) else {
            return false;
        };
        let (from, to) = if a > b { (b, a) } else { (a, b) };
        let range: HashSet<String> = self.collection.values()[from..=to]
            .iter()
            .filter(|value| self.can_select(value))
            .cloned()
            .collect();
        self.selected = range;
        self.anchor = Some(anchor.to_owned());
        true
    }

    /// Applies a pointer click on `value` with the desktop conventions:
    /// `extend` (Shift) selects the range from the anchor, `toggle`
    /// (Control or Command) toggles the value, and a plain click selects
    /// only it. `extend` and `toggle` apply in multiple mode only; in single
    /// mode a click on the selected value deselects it when `deselectable`.
    /// Returns whether the selection may have changed.
    pub fn click(&mut self, value: &str, extend: bool, toggle: bool) -> bool {
        let multiple = self.options.mode == SelectionMode::Multiple;
        if let (true, true, Some(anchor)) = (extend, multiple, self.anchor.clone()) {
            self.extend(&anchor, value)
        } else if toggle && multiple {
            self.toggle(value)
        } else if self.options.mode == SelectionMode::Single {
            self.select(value, true)
        } else {
            self.replace(Some(value))
        }
    }

    /// Selects every enabled item; does nothing outside multiple mode.
    /// Returns whether the selection changed.
    pub fn select_all(&mut self) -> bool {
        if self.options.mode != SelectionMode::Multiple {
            return false;
        }
        let all: Vec<String> = self
            .collection
            .values()
            .iter()
            .filter(|value| self.can_select(value))
            .cloned()
            .collect();
        let mut next = self.selected.clone();
        next.extend(all);
        self.set_selected(next)
    }

    /// Deselects everything; returns whether anything was selected.
    pub fn clear(&mut self) -> bool {
        if self.selected.is_empty() {
            return false;
        }
        self.selected.clear();
        true
    }

    /// Replaces the selection with `values`, keeping only the selectable
    /// ones (and the first of them in single mode). The last of them becomes
    /// the anchor of Shift ranges. Returns whether the selection changed.
    pub fn set_selected_values<S: AsRef<str>>(
        &mut self,
        values: impl IntoIterator<Item = S>,
    ) -> bool {
        let mut valid: Vec<String> = values
            .into_iter()
            .filter(|value| self.can_select(value.as_ref()))
            .map(|value| value.as_ref().to_owned())
            .collect();
        if self.options.mode == SelectionMode::Single {
            valid.truncate(1);
        }
        self.anchor = valid.last().cloned();
        self.set_selected(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::super::list::tests::{Fruit, fruits};
    use super::*;

    fn multiple() -> SelectionOptions {
        SelectionOptions {
            mode: SelectionMode::Multiple,
            ..SelectionOptions::default()
        }
    }

    fn single() -> SelectionController<Fruit> {
        SelectionController::new(fruits(), SelectionOptions::default())
    }

    #[test]
    fn single_mode_replaces_and_toggles_with_force_toggle() {
        let mut selection = single();
        selection.select("apple", false);
        selection.select("cherry", false);
        assert_eq!(selection.selected_values(), ["cherry"]);
        selection.select("cherry", true);
        assert!(selection.is_empty());
    }

    #[test]
    fn never_selects_disabled_or_unknown_values() {
        let mut selection =
            SelectionController::with_values(fruits(), multiple(), ["banana", "kiwi", "apple"]);
        assert_eq!(selection.selected_values(), ["apple"]);
        assert!(!selection.select("banana", false));
        assert!(!selection.is_selected("banana"));
    }

    #[test]
    fn single_mode_keeps_the_first_initial_value_in_collection_order() {
        let selection = SelectionController::with_values(
            fruits(),
            SelectionOptions::default(),
            ["plum", "cherry"],
        );
        assert_eq!(selection.selected_values(), ["cherry"]);
    }

    #[test]
    fn extends_ranges_from_the_anchor_and_keeps_collection_order() {
        let mut selection = SelectionController::new(fruits(), multiple());
        selection.click("plum", false, false);
        selection.click("apple", true, false);
        assert_eq!(
            selection.selected_values(),
            ["apple", "cherry", "pear", "plum"]
        );
        assert_eq!(selection.anchor_value(), Some("plum"));
        selection.click("pear", false, true);
        assert_eq!(selection.selected_values(), ["apple", "cherry", "plum"]);
        selection.click("cherry", false, false);
        assert_eq!(selection.selected_values(), ["cherry"]);
    }

    #[test]
    fn extend_without_anchor_or_with_unknown_values_does_nothing() {
        let mut selection = SelectionController::new(fruits(), multiple());
        selection.click("pear", true, false);
        assert_eq!(selection.selected_values(), ["pear"]);
        assert!(!selection.extend("kiwi", "plum"));
        assert!(!selection.extend("pear", "kiwi"));
        assert_eq!(selection.selected_values(), ["pear"]);
    }

    #[test]
    fn extend_in_single_mode_selects_the_target() {
        let mut selection = single();
        selection.extend("apple", "plum");
        assert_eq!(selection.selected_values(), ["plum"]);
    }

    #[test]
    fn single_mode_click_toggles_the_selected_value() {
        let mut selection = single();
        selection.click("apple", false, false);
        assert!(selection.is_selected("apple"));
        selection.click("apple", false, false);
        assert!(selection.is_empty());
    }

    #[test]
    fn reports_all_some_and_none_for_a_select_all_check_box() {
        let mut selection = SelectionController::new(fruits(), multiple());
        assert!(!selection.is_some_selected());
        assert!(!selection.is_all_selected());
        selection.select("apple", false);
        assert!(selection.is_some_selected());
        selection.select_all();
        assert!(selection.is_all_selected());
        assert!(!selection.is_some_selected());
        assert_eq!(selection.selected_values().len(), 4);
    }

    #[test]
    fn non_deselectable_selections_stay_selected() {
        let options = SelectionOptions {
            deselectable: false,
            ..SelectionOptions::default()
        };
        let mut selection = SelectionController::with_values(fruits(), options, ["apple"]);
        assert!(!selection.toggle("apple"));
        assert!(!selection.select("apple", true));
        assert_eq!(selection.selected_values(), ["apple"]);
    }

    #[test]
    fn none_mode_selects_nothing() {
        let options = SelectionOptions {
            mode: SelectionMode::None,
            ..SelectionOptions::default()
        };
        let mut selection = SelectionController::new(fruits(), options);
        selection.select("apple", false);
        selection.click("apple", false, false);
        assert!(!selection.select_all());
        assert!(selection.is_empty());
    }

    #[test]
    fn a_new_collection_drops_the_values_that_left_it() {
        let mut selection =
            SelectionController::with_values(fruits(), multiple(), ["apple", "plum"]);
        let kept = selection.collection().filter("p");
        assert!(!selection.set_collection(kept));
        assert_eq!(selection.selected_values(), ["apple", "plum"]);
        let narrower = selection.collection().filter("plu");
        assert!(selection.set_collection(narrower));
        assert_eq!(selection.selected_values(), ["plum"]);
    }

    #[test]
    fn a_new_collection_can_reset_the_selection_and_forgets_the_anchor() {
        let options = SelectionOptions {
            reset_on_collection_change: true,
            ..multiple()
        };
        let mut selection = SelectionController::new(fruits(), options);
        selection.select("apple", false);
        assert_eq!(selection.anchor_value(), Some("apple"));
        let same = selection.collection().clone();
        assert!(selection.set_collection(same));
        assert!(selection.is_empty());
        selection.select("plum", false);
        let without_plum = selection.collection().filter("e");
        selection.set_collection(without_plum);
        assert_eq!(selection.anchor_value(), None);
    }

    #[test]
    fn reports_changes_only() {
        let mut selection = single();
        assert!(selection.select("apple", false));
        assert!(!selection.select("apple", false));
        assert!(selection.clear());
        assert!(!selection.clear());
    }

    #[test]
    fn replace_select_all_and_set_values_report_changes_only() {
        let mut selection = SelectionController::new(fruits(), multiple());
        let mut changes = 0;
        changes += usize::from(selection.replace(Some("apple")));
        changes += usize::from(selection.replace(Some("apple")));
        changes += usize::from(selection.select_all());
        changes += usize::from(selection.select_all());
        changes += usize::from(selection.set_selected_values(["cherry", "plum"]));
        changes += usize::from(selection.set_selected_values(["cherry", "plum"]));
        assert_eq!(changes, 3);
        assert_eq!(selection.anchor_value(), Some("plum"));
        selection.click("apple", true, false);
        assert_eq!(
            selection.selected_values(),
            ["apple", "cherry", "pear", "plum"]
        );
    }

    #[test]
    fn replace_with_none_clears_and_unselectable_values_are_ignored() {
        let mut selection = SelectionController::new(fruits(), multiple());
        selection.replace(Some("apple"));
        assert!(!selection.replace(Some("banana")));
        assert_eq!(selection.selected_values(), ["apple"]);
        assert!(selection.replace(None));
        assert!(selection.is_empty());
        assert_eq!(selection.anchor_value(), None);
    }

    #[test]
    fn set_selected_values_keeps_one_value_in_single_mode() {
        let mut selection = single();
        selection.set_selected_values(["banana", "pear", "plum"]);
        assert_eq!(selection.selected_values(), ["pear"]);
        assert_eq!(selection.first_selected_value().as_deref(), Some("pear"));
        assert_eq!(selection.last_selected_value().as_deref(), Some("pear"));
    }

    #[test]
    fn toggle_selects_then_deselects_and_moves_the_anchor() {
        let mut selection = SelectionController::new(fruits(), multiple());
        assert!(selection.toggle("pear"));
        assert!(selection.toggle("plum"));
        assert_eq!(selection.anchor_value(), Some("plum"));
        assert!(selection.toggle("pear"));
        assert_eq!(selection.anchor_value(), Some("pear"));
        assert_eq!(selection.selected_values(), ["plum"]);
    }
}
