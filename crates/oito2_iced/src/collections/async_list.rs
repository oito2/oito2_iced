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

//! A list loaded page by page, modelled as a state machine.

use std::fmt;

/// The direction of a [`SortDescriptor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SortDirection {
    /// Smallest first.
    #[default]
    Ascending,
    /// Largest first.
    Descending,
}

/// What a list is sorted by: a column name and a direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SortDescriptor {
    /// The sorted column or field.
    pub column: String,
    /// The sort direction.
    pub direction: SortDirection,
}

impl SortDescriptor {
    /// Creates an ascending sort of `column`.
    #[must_use]
    pub fn new(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            direction: SortDirection::Ascending,
        }
    }

    /// Returns the descriptor with `direction`.
    #[must_use]
    pub fn with_direction(mut self, direction: SortDirection) -> Self {
        self.direction = direction;
        self
    }
}

/// The identity of one load: a request and its result carry the same id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadId(u64);

/// The parameters of one page load, produced by an
/// [`AsyncListController`].
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncListRequest<C> {
    /// Identifies this load; pass it to [`AsyncListController::finish`].
    pub id: LoadId,
    /// The cursor of the next page, or `None` for the first page.
    pub cursor: Option<C>,
    /// The current filter text.
    pub filter_text: String,
    /// The current sort, or `None`.
    pub sort: Option<SortDescriptor>,
}

/// One page of items loaded for an [`AsyncListController`].
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncListPage<T, C> {
    /// The loaded items.
    pub items: Vec<T>,
    /// The cursor of the next page, or `None` when it is the last one.
    pub cursor: Option<C>,
}

impl<T, C> AsyncListPage<T, C> {
    /// Creates the last page, with `items` and no cursor.
    #[must_use]
    pub fn last(items: Vec<T>) -> Self {
        Self {
            items,
            cursor: None,
        }
    }

    /// Creates a page with `items` followed by the page at `cursor`.
    #[must_use]
    pub fn with_cursor(items: Vec<T>, cursor: C) -> Self {
        Self {
            items,
            cursor: Some(cursor),
        }
    }
}

type LocalSort<T> = Box<dyn Fn(&mut Vec<T>, &SortDescriptor) + Send + Sync>;

/// The state of a list loaded page by page, with filter text, sorting,
/// loading and error states, and stale results ignored.
///
/// The controller never loads anything itself and needs no async runtime.
/// The application drives it in two steps:
///
/// 1. A method such as [`AsyncListController::reload`] updates the state
///    (loading flags, error cleared) and returns an [`AsyncListRequest`]
///    describing what to fetch: the cursor, the filter text and the sort.
/// 2. The application fetches the page however it likes (an Iced `Task`, a
///    thread, a blocking call) and hands the outcome back to
///    [`AsyncListController::finish`] together with the request's
///    [`LoadId`].
///
/// Every request gets a new [`LoadId`]. A request started while another is
/// running supersedes it: the older result is ignored by `finish`, and
/// [`AsyncListController::is_cancelled`] tells a long load that its result
/// will be dropped so it can stop early.
///
/// `T` is the item type, `C` the cursor of the next page and `E` the error
/// the loader reports.
///
/// [`reload`](Self::reload) starts the first page and
/// [`load_more`](Self::load_more) the next one while
/// [`has_more`](Self::has_more). Changing the filter text or the sort
/// starts a reload, unless a local sort function sorts the loaded items in
/// place.
pub struct AsyncListController<T, C, E = String> {
    items: Vec<T>,
    filter_text: String,
    sort_descriptor: Option<SortDescriptor>,
    local_sort: Option<LocalSort<T>>,
    cursor: Option<C>,
    loaded: bool,
    loading: bool,
    loading_more: bool,
    error: Option<E>,
    generation: u64,
}

impl<T, C, E> fmt::Debug for AsyncListController<T, C, E>
where
    T: fmt::Debug,
    C: fmt::Debug,
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncListController")
            .field("items", &self.items)
            .field("filter_text", &self.filter_text)
            .field("sort_descriptor", &self.sort_descriptor)
            .field("cursor", &self.cursor)
            .field("loaded", &self.loaded)
            .field("loading", &self.loading)
            .field("loading_more", &self.loading_more)
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<T, C, E> Default for AsyncListController<T, C, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, C, E> AsyncListController<T, C, E> {
    /// Creates an empty controller; it doesn't load until
    /// [`reload`](Self::reload) is called.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            filter_text: String::new(),
            sort_descriptor: None,
            local_sort: None,
            cursor: None,
            loaded: false,
            loading: false,
            loading_more: false,
            error: None,
            generation: 0,
        }
    }

    /// Returns the controller starting with `items`.
    #[must_use]
    pub fn with_items(mut self, items: Vec<T>) -> Self {
        self.items = items;
        self
    }

    /// Returns the controller starting with `text` as the filter text.
    #[must_use]
    pub fn with_filter_text(mut self, text: impl Into<String>) -> Self {
        self.filter_text = text.into();
        self
    }

    /// Returns the controller starting with `descriptor` as the sort.
    #[must_use]
    pub fn with_sort(mut self, descriptor: SortDescriptor) -> Self {
        self.sort_descriptor = Some(descriptor);
        self
    }

    /// Returns the controller sorting loaded items locally with `sort`;
    /// without it, a sort change reloads.
    #[must_use]
    pub fn with_local_sort(
        mut self,
        sort: impl Fn(&mut Vec<T>, &SortDescriptor) + Send + Sync + 'static,
    ) -> Self {
        self.local_sort = Some(Box::new(sort));
        self
    }

    /// The loaded items.
    #[must_use]
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// The filter text sent with each load.
    #[must_use]
    pub fn filter_text(&self) -> &str {
        &self.filter_text
    }

    /// The current sort, or `None`.
    #[must_use]
    pub fn sort_descriptor(&self) -> Option<&SortDescriptor> {
        self.sort_descriptor.as_ref()
    }

    /// Whether the first page is loading.
    #[must_use]
    pub fn is_loading(&self) -> bool {
        self.loading
    }

    /// Whether a further page is loading.
    #[must_use]
    pub fn is_loading_more(&self) -> bool {
        self.loading_more
    }

    /// The error of the last load, or `None` when it succeeded.
    #[must_use]
    pub fn error(&self) -> Option<&E> {
        self.error.as_ref()
    }

    /// Whether the last successful load returned a cursor for a further
    /// page.
    #[must_use]
    pub fn has_more(&self) -> bool {
        self.loaded && self.cursor.is_some()
    }

    /// Whether `request` was superseded by a newer one, or cancelled; its
    /// result would be ignored, so a long load can stop early.
    #[must_use]
    pub fn is_cancelled(&self, request: &AsyncListRequest<C>) -> bool {
        request.id.0 != self.generation
    }

    /// Cancels the running load, if any: its result will be ignored and the
    /// loading flags are cleared.
    pub fn cancel(&mut self) {
        self.generation += 1;
        self.loading = false;
        self.loading_more = false;
    }

    /// Sorts the loaded items locally with `descriptor` when a local sort
    /// function exists.
    fn apply_local_sort(&mut self) {
        if let (Some(sort), Some(descriptor)) = (&self.local_sort, &self.sort_descriptor) {
            sort(&mut self.items, descriptor);
        }
    }
}

impl<T, C, E> AsyncListController<T, C, E>
where
    C: Clone,
{
    /// Starts replacing the items with the first page, superseding any
    /// running load, and returns the request to fetch.
    pub fn reload(&mut self) -> AsyncListRequest<C> {
        self.start(false)
    }

    /// Starts appending the next page and returns the request to fetch;
    /// `None` when there is no further page ([`has_more`](Self::has_more))
    /// or a load is already running.
    pub fn load_more(&mut self) -> Option<AsyncListRequest<C>> {
        if !self.has_more() || self.loading || self.loading_more {
            return None;
        }
        Some(self.start(true))
    }

    /// Sets the filter text and starts a reload; `None` and no change when
    /// `text` is the current filter text.
    pub fn set_filter_text(&mut self, text: impl Into<String>) -> Option<AsyncListRequest<C>> {
        let text = text.into();
        if text == self.filter_text {
            return None;
        }
        self.filter_text = text;
        Some(self.reload())
    }

    /// Sets the sort to `descriptor`. When a local sort function exists and
    /// `descriptor` is `Some`, the loaded items are sorted in place and no
    /// request is returned; otherwise a reload starts.
    pub fn set_sort(&mut self, descriptor: Option<SortDescriptor>) -> Option<AsyncListRequest<C>> {
        self.sort_descriptor = descriptor;
        if self.local_sort.is_some() && self.sort_descriptor.is_some() {
            self.apply_local_sort();
            return None;
        }
        Some(self.reload())
    }

    /// Applies the outcome of the load `id`: the page replaces or extends
    /// the items, an error is kept in [`error`](Self::error). Returns `false`
    /// and changes nothing when a newer load superseded `id`.
    pub fn finish(&mut self, id: LoadId, result: Result<AsyncListPage<T, C>, E>) -> bool {
        if id.0 != self.generation {
            return false;
        }
        let more = self.loading_more;
        match result {
            Ok(page) => {
                if more {
                    self.items.extend(page.items);
                } else {
                    self.items = page.items;
                }
                self.apply_local_sort();
                self.cursor = page.cursor;
                self.loaded = true;
            }
            Err(error) => {
                self.error = Some(error);
                if !more {
                    // A failed first page invalidates the paging of the
                    // previous load, so a later page can't mix its cursor
                    // with the new filter or sort.
                    self.cursor = None;
                    self.loaded = false;
                }
            }
        }
        self.loading = false;
        self.loading_more = false;
        true
    }

    /// Clears the error, marks the load kind and returns its request.
    fn start(&mut self, more: bool) -> AsyncListRequest<C> {
        self.generation += 1;
        self.error = None;
        if more {
            self.loading_more = true;
        } else {
            self.loading = true;
            self.loading_more = false;
        }
        AsyncListRequest {
            id: LoadId(self.generation),
            cursor: if more { self.cursor.clone() } else { None },
            filter_text: self.filter_text.clone(),
            sort: self.sort_descriptor.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Numbers = AsyncListController<i32, usize, String>;

    /// Serves pages of ten numbers below 25 whose text contains the filter.
    fn serve(request: &AsyncListRequest<usize>) -> AsyncListPage<i32, usize> {
        let start = request.cursor.unwrap_or(0);
        let all: Vec<i32> = (0..25)
            .filter(|i| i.to_string().contains(&request.filter_text))
            .collect();
        let page = all.iter().copied().skip(start).take(10).collect();
        let next = start + 10;
        AsyncListPage {
            items: page,
            cursor: (next < all.len()).then_some(next),
        }
    }

    /// Runs `request` against [`serve`] and applies the result.
    fn complete(list: &mut Numbers, request: &AsyncListRequest<usize>) {
        assert!(list.finish(request.id, Ok(serve(request))));
    }

    #[test]
    fn loads_pages_until_the_cursor_runs_out() {
        let mut list = Numbers::new();
        assert!(!list.has_more());
        assert!(list.load_more().is_none());
        let request = list.reload();
        assert!(list.is_loading());
        complete(&mut list, &request);
        assert!(!list.is_loading());
        assert_eq!(list.items().len(), 10);
        assert!(list.has_more());
        for _ in 0..2 {
            let request = list.load_more().expect("more pages");
            assert!(list.is_loading_more());
            assert!(!list.is_loading());
            complete(&mut list, &request);
        }
        assert_eq!(list.items().len(), 25);
        assert!(!list.has_more());
        assert!(list.load_more().is_none());
    }

    #[test]
    fn load_more_waits_for_the_running_load() {
        let mut list = Numbers::new();
        let request = list.reload();
        complete(&mut list, &request);
        let more = list.load_more().expect("more pages");
        assert!(list.load_more().is_none());
        complete(&mut list, &more);
    }

    #[test]
    fn a_failed_reload_stops_paging_with_the_old_cursor() {
        let mut list = Numbers::new();
        let request = list.reload();
        assert!(list.finish(request.id, Ok(AsyncListPage::with_cursor(vec![1, 2], 2))));
        assert!(list.has_more());
        let request = list.set_filter_text("x").expect("reloads");
        assert!(list.finish(request.id, Err("offline".to_owned())));
        assert_eq!(list.error().map(String::as_str), Some("offline"));
        assert!(!list.has_more());
        assert!(list.load_more().is_none());
        assert!(!list.is_loading());
    }

    #[test]
    fn a_failed_page_keeps_the_items_and_the_cursor() {
        let mut list = Numbers::new();
        let request = list.reload();
        complete(&mut list, &request);
        let more = list.load_more().expect("more pages");
        assert!(list.finish(more.id, Err("slow".to_owned())));
        assert_eq!(list.items().len(), 10);
        assert!(list.has_more());
        assert!(list.error().is_some());
        let retry = list.load_more().expect("retry");
        assert_eq!(retry.cursor, Some(10));
        assert!(list.error().is_none());
    }

    #[test]
    fn filter_text_reloads_from_the_first_page() {
        let mut list = Numbers::new();
        let request = list.reload();
        complete(&mut list, &request);
        let more = list.load_more().expect("more pages");
        complete(&mut list, &more);
        let request = list.set_filter_text("2").expect("reloads");
        assert_eq!(request.cursor, None);
        assert_eq!(request.filter_text, "2");
        complete(&mut list, &request);
        assert_eq!(list.items(), [2, 12, 20, 21, 22, 23, 24]);
        assert_eq!(list.filter_text(), "2");
    }

    #[test]
    fn the_same_filter_text_does_not_reload() {
        let mut list = Numbers::new().with_filter_text("a");
        assert!(list.set_filter_text("a").is_none());
        assert!(!list.is_loading());
    }

    #[test]
    fn a_local_sort_reorders_without_loading() {
        let mut list = Numbers::new().with_local_sort(|items, sort| match sort.direction {
            SortDirection::Descending => items.sort_by(|a, b| b.cmp(a)),
            SortDirection::Ascending => items.sort(),
        });
        let request = list.reload();
        complete(&mut list, &request);
        let sort = SortDescriptor::new("value").with_direction(SortDirection::Descending);
        assert!(list.set_sort(Some(sort.clone())).is_none());
        assert_eq!(list.items().first(), Some(&9));
        assert_eq!(list.sort_descriptor(), Some(&sort));
        assert!(!list.is_loading());
    }

    #[test]
    fn a_local_sort_applies_to_later_pages() {
        let mut list = Numbers::new().with_local_sort(|items, _| items.sort_by(|a, b| b.cmp(a)));
        assert!(list.set_sort(Some(SortDescriptor::new("n"))).is_none());
        let request = list.reload();
        complete(&mut list, &request);
        let more = list.load_more().expect("more pages");
        complete(&mut list, &more);
        assert_eq!(list.items().first(), Some(&19));
    }

    #[test]
    fn a_sort_without_local_function_reloads_with_the_sort() {
        let mut list = Numbers::new();
        let sort = SortDescriptor::new("value");
        let request = list.set_sort(Some(sort.clone())).expect("reloads");
        assert_eq!(request.sort, Some(sort));
        assert!(list.is_loading());
    }

    #[test]
    fn a_newer_load_supersedes_a_slower_one() {
        let mut list: AsyncListController<&str, usize> = AsyncListController::new();
        let slow = list.reload();
        assert!(list.is_loading());
        let fast = list.reload();
        assert!(list.is_cancelled(&slow));
        assert!(!list.is_cancelled(&fast));
        assert!(list.finish(fast.id, Ok(AsyncListPage::last(vec!["new"]))));
        assert!(!list.finish(slow.id, Ok(AsyncListPage::last(vec!["old"]))));
        assert_eq!(list.items(), ["new"]);
        assert!(!list.is_loading());
    }

    #[test]
    fn a_stale_error_is_ignored_too() {
        let mut list: AsyncListController<&str, usize> = AsyncListController::new();
        let slow = list.reload();
        let fast = list.reload();
        assert!(!list.finish(slow.id, Err("late".to_owned())));
        assert!(list.error().is_none());
        assert!(list.is_loading());
        assert!(list.finish(fast.id, Ok(AsyncListPage::last(vec![]))));
    }

    #[test]
    fn keeps_the_error_of_a_failed_load() {
        let mut list: AsyncListController<i32, usize> = AsyncListController::new();
        let request = list.reload();
        list.finish(request.id, Err("offline".to_owned()));
        assert_eq!(list.error().map(String::as_str), Some("offline"));
        assert!(!list.is_loading());
        list.reload();
        assert!(list.error().is_none());
    }

    #[test]
    fn cancel_drops_the_running_load() {
        let mut list: AsyncListController<i32, usize> = AsyncListController::new();
        let request = list.reload();
        list.cancel();
        assert!(list.is_cancelled(&request));
        assert!(!list.is_loading());
        assert!(!list.finish(request.id, Ok(AsyncListPage::last(vec![1]))));
        assert!(list.items().is_empty());
    }

    #[test]
    fn starts_with_the_given_items_filter_and_sort() {
        let list: AsyncListController<i32, usize> = AsyncListController::new()
            .with_items(vec![3, 1])
            .with_filter_text("f")
            .with_sort(SortDescriptor::new("n"));
        assert_eq!(list.items(), [3, 1]);
        assert_eq!(list.filter_text(), "f");
        assert_eq!(list.sort_descriptor().map(|s| s.column.as_str()), Some("n"));
        assert!(!list.is_loading());
        assert!(!list.has_more());
    }
}
