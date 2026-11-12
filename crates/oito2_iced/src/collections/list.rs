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

//! An immutable list of items with value, label, disabled and group
//! accessors.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use super::CollectionError;
use super::text_folding::fold_text;

type Accessor<T, R> = Arc<dyn Fn(&T) -> R + Send + Sync>;

/// A group of consecutive-by-first-appearance items of a
/// [`ListCollection`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListGroup<'a, T> {
    /// The group name; `None` for the ungrouped items.
    pub group: Option<String>,
    /// The items of the group, in collection order.
    pub items: Vec<&'a T>,
}

/// An immutable list of items with the accessors a list, select or
/// autocomplete needs: a unique string value, a display label, a disabled
/// flag and an optional group per item.
///
/// Navigation methods skip disabled items. Methods that change the list
/// return a new collection that shares the accessors.
pub struct ListCollection<T> {
    items: Vec<T>,
    values: Vec<String>,
    folded_labels: Vec<String>,
    indexes: HashMap<String, usize>,
    value_of: Accessor<T, String>,
    label_of: Option<Accessor<T, String>>,
    is_disabled: Option<Accessor<T, bool>>,
    group_of: Option<Accessor<T, Option<String>>>,
}

impl<T> fmt::Debug for ListCollection<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListCollection")
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

impl<T> Clone for ListCollection<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            values: self.values.clone(),
            folded_labels: self.folded_labels.clone(),
            indexes: self.indexes.clone(),
            value_of: Arc::clone(&self.value_of),
            label_of: self.label_of.clone(),
            is_disabled: self.is_disabled.clone(),
            group_of: self.group_of.clone(),
        }
    }
}

impl<T: 'static> ListCollection<T> {
    /// Creates a collection of `items` whose value is their text form
    /// ([`ToString`]); the label is the value, items are enabled and
    /// ungrouped. Use the `with_` methods to change that.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::DuplicateValue`] when two items share a
    /// value.
    pub fn new(items: impl IntoIterator<Item = T>) -> Result<Self, CollectionError>
    where
        T: ToString,
    {
        Self::with_value_of(items, T::to_string)
    }

    /// Creates a collection of `items` whose unique value is given by
    /// `value_of`.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::DuplicateValue`] when two items share a
    /// value.
    pub fn with_value_of(
        items: impl IntoIterator<Item = T>,
        value_of: impl Fn(&T) -> String + Send + Sync + 'static,
    ) -> Result<Self, CollectionError> {
        Self::build(
            items.into_iter().collect(),
            Arc::new(value_of),
            None,
            None,
            None,
        )
    }

    fn build(
        items: Vec<T>,
        value_of: Accessor<T, String>,
        label_of: Option<Accessor<T, String>>,
        is_disabled: Option<Accessor<T, bool>>,
        group_of: Option<Accessor<T, Option<String>>>,
    ) -> Result<Self, CollectionError> {
        let mut indexes = HashMap::with_capacity(items.len());
        let mut values = Vec::with_capacity(items.len());
        for (index, item) in items.iter().enumerate() {
            let value = value_of(item);
            if indexes.insert(value.clone(), index).is_some() {
                return Err(CollectionError::DuplicateValue(value));
            }
            values.push(value);
        }
        let mut collection = Self {
            items,
            values,
            folded_labels: Vec::new(),
            indexes,
            value_of,
            label_of,
            is_disabled,
            group_of,
        };
        collection.refresh_labels();
        Ok(collection)
    }

    /// Recomputes the folded labels from the current label accessor.
    fn refresh_labels(&mut self) {
        self.folded_labels = self
            .items
            .iter()
            .map(|item| fold_text(&self.label_of(item)))
            .collect();
    }

    /// Returns the collection with the display label of each item given by
    /// `label_of`.
    #[must_use]
    pub fn with_label_of(
        mut self,
        label_of: impl Fn(&T) -> String + Send + Sync + 'static,
    ) -> Self {
        self.label_of = Some(Arc::new(label_of));
        self.refresh_labels();
        self
    }

    /// Returns the collection with the disabled state of each item given by
    /// `is_disabled`.
    #[must_use]
    pub fn with_disabled(
        mut self,
        is_disabled: impl Fn(&T) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.is_disabled = Some(Arc::new(is_disabled));
        self
    }

    /// Returns the collection with the group of each item given by
    /// `group_of`.
    #[must_use]
    pub fn with_group_of(
        mut self,
        group_of: impl Fn(&T) -> Option<String> + Send + Sync + 'static,
    ) -> Self {
        self.group_of = Some(Arc::new(group_of));
        self
    }

    /// The items, in order.
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// The number of items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Whether the collection has no items.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// The values of every item, in order.
    #[must_use]
    pub fn values(&self) -> &[String] {
        &self.values
    }

    /// Returns the unique value of `item`.
    #[must_use]
    pub fn value_of(&self, item: &T) -> String {
        (self.value_of)(item)
    }

    /// Returns the display label of `item`; its value when no label
    /// accessor is set.
    #[must_use]
    pub fn label_of(&self, item: &T) -> String {
        match &self.label_of {
            Some(label_of) => label_of(item),
            None => (self.value_of)(item),
        }
    }

    /// Whether `item` can't be selected or focused.
    #[must_use]
    pub fn is_disabled(&self, item: &T) -> bool {
        self.is_disabled.as_ref().is_some_and(|test| test(item))
    }

    /// Returns the group of `item`, or `None` when it has none.
    #[must_use]
    pub fn group_of(&self, item: &T) -> Option<String> {
        self.group_of.as_ref().and_then(|group_of| group_of(item))
    }

    /// Whether an item has `value`.
    #[must_use]
    pub fn has(&self, value: &str) -> bool {
        self.indexes.contains_key(value)
    }

    /// Returns the index of the item with `value`, or `None` when there is
    /// none.
    #[must_use]
    pub fn index_of(&self, value: &str) -> Option<usize> {
        self.indexes.get(value).copied()
    }

    /// Returns the item with `value`, or `None` when there is none.
    #[must_use]
    pub fn find(&self, value: &str) -> Option<&T> {
        self.index_of(value).map(|index| &self.items[index])
    }

    /// Whether the item with `value` exists and is enabled.
    #[must_use]
    pub fn is_enabled(&self, value: &str) -> bool {
        self.find(value).is_some_and(|item| !self.is_disabled(item))
    }

    /// The value of the first enabled item, or `None`.
    #[must_use]
    pub fn first_value(&self) -> Option<&str> {
        self.scan(-1, 1, false)
    }

    /// The value of the last enabled item, or `None`.
    #[must_use]
    pub fn last_value(&self) -> Option<&str> {
        self.scan(self.items.len().cast_signed(), -1, false)
    }

    /// Returns the value of the first enabled item after `value`, or `None`
    /// at the end. With `looping`, the search wraps around to the start.
    ///
    /// When `value` isn't in the collection, the search starts before the
    /// first item.
    #[must_use]
    pub fn next_value(&self, value: &str, looping: bool) -> Option<&str> {
        let from = self.index_of(value).map_or(-1, usize::cast_signed);
        self.scan(from, 1, looping)
    }

    /// Returns the value of the first enabled item before `value`, or
    /// `None` at the start. With `looping`, the search wraps around to the
    /// end.
    ///
    /// When `value` isn't in the collection, the search starts after the
    /// last item.
    #[must_use]
    pub fn previous_value(&self, value: &str, looping: bool) -> Option<&str> {
        let from = self
            .index_of(value)
            .unwrap_or(self.items.len())
            .cast_signed();
        self.scan(from, -1, looping)
    }

    /// Returns the value of the first enabled item found by moving `step`
    /// positions at a time from the index `from`, wrapping around with
    /// `looping` and stopping at either end otherwise.
    fn scan(&self, from: isize, step: isize, looping: bool) -> Option<&str> {
        let length = self.items.len().cast_signed();
        for n in 1..=length {
            let mut index = from + step * n;
            if looping {
                index = index.rem_euclid(length);
            } else if index < 0 || index >= length {
                return None;
            }
            let item = &self.items[index.cast_unsigned()];
            if !self.is_disabled(item) {
                return Some(&self.values[index.cast_unsigned()]);
            }
        }
        None
    }

    /// Returns the value of the first enabled item whose label starts with
    /// `query`, ignoring case and accents, searching after the item with
    /// value `from` (from the first item when `from` is `None` or unknown)
    /// and wrapping around; `None` when none matches or `query` is empty.
    ///
    /// It implements typeahead: typing letters while a list has focus moves
    /// to the next matching item.
    #[must_use]
    pub fn search(&self, query: &str, from: Option<&str>) -> Option<&str> {
        if query.is_empty() || self.is_empty() {
            return None;
        }
        let needle = fold_text(query);
        let length = self.items.len().cast_signed();
        let start = from
            .and_then(|value| self.index_of(value))
            .map_or(-1, usize::cast_signed);
        for n in 1..=length {
            let index = (start + n).rem_euclid(length).cast_unsigned();
            if !self.is_disabled(&self.items[index])
                && self.folded_labels[index].starts_with(&needle)
            {
                return Some(&self.values[index]);
            }
        }
        None
    }

    /// The items grouped by their group, in the order each group first
    /// appears; ungrouped items form a group named `None`.
    #[must_use]
    pub fn groups(&self) -> Vec<ListGroup<'_, T>> {
        let mut result: Vec<ListGroup<'_, T>> = Vec::new();
        for item in &self.items {
            let group = self.group_of(item);
            match result.iter_mut().find(|entry| entry.group == group) {
                Some(entry) => entry.items.push(item),
                None => result.push(ListGroup {
                    group,
                    items: vec![item],
                }),
            }
        }
        result
    }

    /// Returns the enabled items whose label contains `query`, ignoring
    /// case and accents, in order, leaving out those whose label is in
    /// `exclude`: the suggestions for a text typed in a field.
    #[must_use]
    pub fn suggestions(&self, query: &str, exclude: &HashSet<String>) -> Vec<&T> {
        let needle = fold_text(query);
        self.items
            .iter()
            .zip(&self.folded_labels)
            .filter(|(item, folded)| {
                !self.is_disabled(item)
                    && folded.contains(&needle)
                    && !exclude.contains(&self.label_of(item))
            })
            .map(|(item, _)| item)
            .collect()
    }
}

impl<T> ListCollection<T>
where
    T: Clone + 'static,
{
    /// Returns a collection of `items` with the same accessors.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::DuplicateValue`] when two of `items`
    /// share a value.
    pub fn copy_with(&self, items: impl IntoIterator<Item = T>) -> Result<Self, CollectionError> {
        Self::build(
            items.into_iter().collect(),
            Arc::clone(&self.value_of),
            self.label_of.clone(),
            self.is_disabled.clone(),
            self.group_of.clone(),
        )
    }

    /// Returns a collection of the items for which `test` returns true.
    #[must_use]
    pub fn filter_by(&self, test: impl Fn(&T) -> bool) -> Self {
        self.keep(|index| test(&self.items[index]))
    }

    /// Returns a collection of the items whose label contains `query`,
    /// ignoring case and accents.
    #[must_use]
    pub fn filter(&self, query: &str) -> Self {
        let needle = fold_text(query);
        self.keep(|index| self.folded_labels[index].contains(&needle))
    }

    /// Returns a collection of the items whose index passes `keep`; a subset
    /// of a collection never has duplicate values, so no check is needed.
    fn keep(&self, keep: impl Fn(usize) -> bool) -> Self {
        let kept: Vec<usize> = (0..self.items.len()).filter(|index| keep(*index)).collect();
        let values: Vec<String> = kept.iter().map(|&i| self.values[i].clone()).collect();
        Self {
            items: kept.iter().map(|&i| self.items[i].clone()).collect(),
            folded_labels: kept
                .iter()
                .map(|&i| self.folded_labels[i].clone())
                .collect(),
            indexes: values
                .iter()
                .enumerate()
                .map(|(index, value)| (value.clone(), index))
                .collect(),
            values,
            value_of: Arc::clone(&self.value_of),
            label_of: self.label_of.clone(),
            is_disabled: self.is_disabled.clone(),
            group_of: self.group_of.clone(),
        }
    }

    /// Returns the collection with the item at `from` moved to `to`.
    ///
    /// # Errors
    ///
    /// Returns [`CollectionError::IndexOutOfRange`] when either index is
    /// outside the items.
    pub fn reorder(&self, from: usize, to: usize) -> Result<Self, CollectionError> {
        let len = self.items.len();
        for index in [from, to] {
            if index >= len {
                return Err(CollectionError::IndexOutOfRange { index, len });
            }
        }
        let mut list = self.items.clone();
        let item = list.remove(from);
        list.insert(to, item);
        self.copy_with(list)
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Fruit {
        pub id: &'static str,
        pub name: &'static str,
        pub group: &'static str,
        pub disabled: bool,
    }

    /// Apple, banana (disabled), cherry, pear and plum, grouped.
    pub(crate) fn fruits() -> ListCollection<Fruit> {
        let fruit = |id, name, group, disabled| Fruit {
            id,
            name,
            group,
            disabled,
        };
        ListCollection::with_value_of(
            [
                fruit("apple", "Apple", "Pome", false),
                fruit("banana", "Banana", "Berry", true),
                fruit("cherry", "Cherry", "Drupe", false),
                fruit("pear", "Pêra", "Pome", false),
                fruit("plum", "Plum", "Drupe", false),
            ],
            |f| f.id.to_owned(),
        )
        .map(|c| {
            c.with_label_of(|f| f.name.to_owned())
                .with_disabled(|f| f.disabled)
                .with_group_of(|f| Some(f.group.to_owned()))
        })
        .expect("unique values")
    }

    fn ids(items: &[&Fruit]) -> Vec<&'static str> {
        items.iter().map(|f| f.id).collect()
    }

    #[test]
    fn looks_items_up_by_value() {
        let fruits = fruits();
        assert_eq!(fruits.len(), 5);
        assert_eq!(
            fruits.values(),
            ["apple", "banana", "cherry", "pear", "plum"]
        );
        assert_eq!(fruits.find("cherry").map(|f| f.name), Some("Cherry"));
        assert_eq!(fruits.index_of("pear"), Some(3));
        assert!(!fruits.has("kiwi"));
        assert_eq!(fruits.label_of(&fruits.items()[0]), "Apple");
        assert!(!fruits.is_enabled("banana"));
        assert!(fruits.is_enabled("apple"));
        assert!(!fruits.is_enabled("kiwi"));
    }

    #[test]
    fn strings_are_their_own_value_and_label() {
        let list = ListCollection::new(["a".to_owned(), "b".to_owned()]).expect("unique");
        assert_eq!(list.value_of(&"a".to_owned()), "a");
        assert_eq!(list.label_of(&"b".to_owned()), "b");
    }

    #[test]
    fn navigation_skips_disabled_items() {
        let fruits = fruits();
        assert_eq!(fruits.first_value(), Some("apple"));
        assert_eq!(fruits.last_value(), Some("plum"));
        assert_eq!(fruits.next_value("apple", false), Some("cherry"));
        assert_eq!(fruits.previous_value("cherry", false), Some("apple"));
        assert_eq!(fruits.next_value("plum", false), None);
        assert_eq!(fruits.next_value("plum", true), Some("apple"));
        assert_eq!(fruits.previous_value("apple", false), None);
        assert_eq!(fruits.previous_value("apple", true), Some("plum"));
    }

    #[test]
    fn unknown_values_start_from_the_ends() {
        let fruits = fruits();
        assert_eq!(fruits.next_value("kiwi", false), Some("apple"));
        assert_eq!(fruits.previous_value("kiwi", false), Some("plum"));
    }

    #[test]
    fn navigation_of_empty_and_fully_disabled_lists() {
        let empty = ListCollection::new(Vec::<String>::new()).expect("empty");
        assert_eq!(empty.first_value(), None);
        assert_eq!(empty.next_value("a", true), None);
        assert_eq!(empty.search("a", None), None);
        let off = ListCollection::new(["a".to_owned()])
            .expect("unique")
            .with_disabled(|_| true);
        assert_eq!(off.first_value(), None);
        assert_eq!(off.next_value("a", true), None);
    }

    #[test]
    fn typeahead_ignores_case_and_accents_and_wraps_around() {
        let fruits = fruits();
        assert_eq!(fruits.search("p", None), Some("pear"));
        assert_eq!(fruits.search("p", Some("pear")), Some("plum"));
        assert_eq!(fruits.search("p", Some("plum")), Some("pear"));
        assert_eq!(fruits.search("PERA", None), Some("pear"));
        assert_eq!(fruits.search("b", None), None);
        assert_eq!(fruits.search("", None), None);
        assert_eq!(fruits.search("a", Some("kiwi")), Some("apple"));
    }

    #[test]
    fn groups_keep_the_order_of_first_appearance() {
        let fruits = fruits();
        let groups = fruits.groups();
        let names: Vec<_> = groups.iter().map(|g| g.group.as_deref()).collect();
        assert_eq!(names, [Some("Pome"), Some("Berry"), Some("Drupe")]);
        assert_eq!(ids(&groups[0].items), ["apple", "pear"]);
    }

    #[test]
    fn ungrouped_items_form_a_none_group() {
        let list = ListCollection::new(["a".to_owned()]).expect("unique");
        let groups = list.groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].group, None);
    }

    #[test]
    fn filters_and_reorders_into_new_collections() {
        let fruits = fruits();
        assert_eq!(fruits.filter("er").values(), ["cherry", "pear"]);
        assert_eq!(
            fruits.reorder(0, 4).expect("in range").values(),
            ["banana", "cherry", "pear", "plum", "apple"]
        );
        assert_eq!(fruits.values()[0], "apple");
        assert_eq!(
            fruits.filter_by(|f| f.group == "Pome").values(),
            ["apple", "pear"]
        );
        assert_eq!(fruits.filter("er").index_of("pear"), Some(1));
        assert!(fruits.filter("er").is_enabled("cherry"));
    }

    #[test]
    fn rejects_duplicate_values_and_reorders_within_range() {
        assert_eq!(
            ListCollection::new(["a", "a"]).unwrap_err(),
            CollectionError::DuplicateValue("a".to_owned())
        );
        let list = ListCollection::new(["a", "b"]).expect("unique");
        assert_eq!(
            list.reorder(0, 5).unwrap_err(),
            CollectionError::IndexOutOfRange { index: 5, len: 2 }
        );
        assert!(list.copy_with(["x", "x"]).is_err());
        assert_eq!(list.copy_with(["x", "y", "z"]).expect("ok").len(), 3);
    }

    #[test]
    fn suggests_enabled_matches_leaving_out_excluded_labels() {
        let fruits = fruits();
        let none = HashSet::new();
        assert_eq!(
            ids(&fruits.suggestions("p", &none)),
            ["apple", "pear", "plum"]
        );
        let exclude: HashSet<String> = ["Pêra".to_owned()].into();
        assert!(fruits.suggestions("pe", &exclude).is_empty());
        assert!(fruits.suggestions("BAN", &none).is_empty());
    }
}
