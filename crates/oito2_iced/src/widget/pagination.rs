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

//! Page navigation for paged content.
//!
//! A pagination shows previous and next buttons around page number buttons
//! with ellipses ([`items`] picks the numbers), and an optional page size
//! selector, wrapped below the buttons when the row is too narrow.
//! Narrower than [`Pagination::compact_width`] it shows only the buttons
//! around "Page 3 of 10". Pages count from 1.
//!
//! The page lives in the application: the pagination shows `page` and
//! publishes the page to show through [`Pagination::on_change`]; without
//! it every button is disabled. The page size selector is a
//! [`select`](super::menu::select), whose open state lives in the
//! application too ([`Pagination::page_size_menu`]).
//!
//! ```no_run
//! use oito2_iced::widget::pagination;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Page(u32),
//!     PageSize(u32),
//!     PageSizeMenu(bool),
//! }
//!
//! # fn view<'a>(page: u32, size: u32, open: bool) -> oito2_iced::Element<'a, Message> {
//! pagination::pagination(page, 10)
//!     .on_change(Message::Page)
//!     .page_sizes([10, 20, 50], size, Message::PageSize)
//!     .page_size_menu(open, Message::PageSizeMenu)
//!     .into()
//! # }
//! ```

#![cfg(feature = "svg")]

use std::rc::Rc;

use iced::widget::button::{Status, Style};
use iced::widget::{Row, container, responsive};
use iced::{Alignment, Background, Border, Length};

use super::helpers::{ReadingDirection, Side};
use super::min_size::{Minimum, min_size};
use super::{button, focusable, menu, styled_text};
use crate::collections::ListCollection;
use crate::icon::lucide;
use crate::l10n::Localizations;
use crate::theme::{
    DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY, TRANSPARENT, state_layer,
};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// The default width below which the compact form shows (a literal in the
/// OITO2 design).
pub const COMPACT_WIDTH: f32 = 420.0;

/// The width of an ellipsis between page numbers (a literal in the OITO2
/// design).
const ELLIPSIS_WIDTH: f32 = 24.0;

/// The width of the page size selector (a literal in the OITO2 design).
const PAGE_SIZE_WIDTH: f32 = 96.0;

/// The page numbers a pagination shows for `page` of `count`, with `None`
/// for an ellipsis.
///
/// It keeps `boundary_count` pages at each end and `sibling_count` pages
/// on each side of `page`, and always returns the same number of items for
/// a given `count`, so the buttons don't shift as the page changes. An
/// ellipsis never stands for a single page. Pages count from 1, and a
/// `count` of zero gives an empty list.
pub fn items(page: u32, count: u32, sibling_count: u32, boundary_count: u32) -> Vec<Option<u32>> {
    if count == 0 {
        return Vec::new();
    }

    let (page, count) = (i64::from(page), i64::from(count));
    let (siblings, boundary) = (i64::from(sibling_count), i64::from(boundary_count));
    let start_pages = 1..=boundary.min(count);
    let end_pages = (count - boundary + 1).max(boundary + 1)..=count;
    let siblings_start = (page - siblings)
        .min(count - boundary - siblings * 2 - 1)
        .max(boundary + 2);
    let siblings_end =
        (page + siblings)
            .max(boundary + siblings * 2 + 2)
            .min(if end_pages.is_empty() {
                count - 1
            } else {
                end_pages.start() - 2
            });

    let mut result: Vec<Option<i64>> = start_pages.map(Some).collect();

    if siblings_start > boundary + 2 {
        result.push(None);
    } else if boundary + 1 < count - boundary {
        result.push(Some(boundary + 1));
    }

    result.extend((siblings_start..=siblings_end).map(Some));

    if siblings_end < count - boundary - 1 {
        result.push(None);
    } else if count - boundary > boundary {
        result.push(Some(count - boundary));
    }

    result.extend(end_pages.map(Some));

    result
        .into_iter()
        .filter_map(|item| match item {
            Some(page) => u32::try_from(page).ok().map(Some),
            None => Some(None),
        })
        .collect()
}

/// Page navigation. See [`pagination`].
pub struct Pagination<'a, Message> {
    page: u32,
    page_count: u32,
    on_change: Option<Rc<dyn Fn(u32) -> Message + 'a>>,
    sibling_count: u32,
    boundary_count: u32,
    page_size: Option<u32>,
    page_sizes: Vec<u32>,
    on_page_size_change: Option<Rc<dyn Fn(u32) -> Message + 'a>>,
    page_size_open: bool,
    on_page_size_open_change: Option<Rc<dyn Fn(bool) -> Message + 'a>>,
    compact_width: f32,
    localizations: Localizations,
    direction: ReadingDirection,
}

/// A pagination showing `page` of `page_count`, from 1: `page_count` is
/// at least 1 and `page` is clamped between 1 and it. One page on each
/// side of the current one and one at each end show by default.
pub fn pagination<'a, Message>(page: u32, page_count: u32) -> Pagination<'a, Message> {
    let page_count = page_count.max(1);

    Pagination {
        page: page.clamp(1, page_count),
        page_count,
        on_change: None,
        sibling_count: 1,
        boundary_count: 1,
        page_size: None,
        page_sizes: Vec::new(),
        on_page_size_change: None,
        page_size_open: false,
        on_page_size_open_change: None,
        compact_width: COMPACT_WIDTH,
        localizations: Localizations::en(),
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> Pagination<'a, Message> {
    /// Sets the function that turns the page to show into a message.
    /// Without it every button is disabled.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(u32) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }

    /// Sets the pages shown on each side of the current one (1 by default).
    #[must_use]
    pub fn sibling_count(mut self, sibling_count: u32) -> Self {
        self.sibling_count = sibling_count;
        self
    }

    /// Sets the pages always shown at each end (1 by default).
    #[must_use]
    pub fn boundary_count(mut self, boundary_count: u32) -> Self {
        self.boundary_count = boundary_count;
        self
    }

    /// Shows the page size selector offering `sizes`, with `page_size`
    /// chosen, which publishes the chosen size through `on_change`. A
    /// repeated size shows once; an empty list hides the selector, and a
    /// `page_size` outside `sizes` shows the first size.
    #[must_use]
    pub fn page_sizes(
        mut self,
        sizes: impl IntoIterator<Item = u32>,
        page_size: u32,
        on_change: impl Fn(u32) -> Message + 'a,
    ) -> Self {
        let mut unique: Vec<u32> = Vec::new();

        for size in sizes {
            if !unique.contains(&size) {
                unique.push(size);
            }
        }

        self.page_size = unique.contains(&page_size).then_some(page_size);
        self.page_sizes = unique;
        self.on_page_size_change = Some(Rc::new(on_change));
        self
    }

    /// Shows or hides the menu of the page size selector, and sets the
    /// function that turns the requested open state into a message, as
    /// [`Select::on_open_change`](super::menu::Select::on_open_change).
    /// Without it the selector is disabled.
    #[must_use]
    pub fn page_size_menu(
        mut self,
        open: bool,
        on_open_change: impl Fn(bool) -> Message + 'a,
    ) -> Self {
        self.page_size_open = open;
        self.on_page_size_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// Sets the width below which only the arrows around "Page 3 of 10"
    /// show ([`COMPACT_WIDTH`] by default).
    #[must_use]
    pub fn compact_width(mut self, compact_width: f32) -> Self {
        self.compact_width = compact_width;
        self
    }

    /// Takes the tooltips of the arrows, the "Page 3 of 10" text and the
    /// label of the page size selector from `localizations` (English by
    /// default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the reading direction, which orders the buttons and points the
    /// arrows.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<Message> Clone for Pagination<'_, Message> {
    fn clone(&self) -> Self {
        Self {
            page: self.page,
            page_count: self.page_count,
            on_change: self.on_change.clone(),
            sibling_count: self.sibling_count,
            boundary_count: self.boundary_count,
            page_size: self.page_size,
            page_sizes: self.page_sizes.clone(),
            on_page_size_change: self.on_page_size_change.clone(),
            page_size_open: self.page_size_open,
            on_page_size_open_change: self.on_page_size_open_change.clone(),
            compact_width: self.compact_width,
            localizations: self.localizations,
            direction: self.direction,
        }
    }
}

impl<'a, Message: Clone + 'a> Pagination<'a, Message> {
    /// The message that shows `page`, when the pagination is enabled.
    fn go(&self, page: u32) -> Option<Message> {
        self.on_change.as_ref().map(|on_change| on_change(page))
    }

    /// The pagination laid out in `width`.
    fn view(&self, width: f32) -> Element<'a, Message> {
        let strings = &self.localizations;
        let (page, count) = (self.page, self.page_count);

        let previous = button::icon_button(chevron(self.direction, false))
            .tooltip(strings.material.previous_page)
            .on_press_maybe((page > 1).then(|| self.go(page - 1)).flatten());
        let next = button::icon_button(chevron(self.direction, true))
            .tooltip(strings.material.next_page)
            .on_press_maybe((page < count).then(|| self.go(page + 1)).flatten());

        let mut buttons: Vec<Element<'a, Message>> = vec![previous.into()];

        if width < self.compact_width {
            buttons.push(
                container(styled_text(
                    strings.page_of(i64::from(page), i64::from(count)),
                    typography::BODY_MEDIUM,
                ))
                .padding([0.0, spacing::SM])
                .into(),
            );
        } else {
            for item in items(page, count, self.sibling_count, self.boundary_count) {
                buttons.push(match item {
                    Some(number) => page_button(number, number == page, self.go(number)),
                    None => container(styled_text("…", typography::BODY_MEDIUM))
                        .center_x(ELLIPSIS_WIDTH)
                        .into(),
                });
            }
        }

        buttons.push(next.into());

        if self.direction == ReadingDirection::RightToLeft {
            buttons.reverse();
        }

        let mut parts: Vec<Element<'a, Message>> = vec![
            Row::with_children(buttons)
                .spacing(spacing::XXS)
                .align_y(Alignment::Center)
                .into(),
        ];

        if let Some(selector) = self.page_size_selector() {
            parts.push(selector);
        }

        if self.direction == ReadingDirection::RightToLeft {
            parts.reverse();
        }

        let wrapping = Row::with_children(parts)
            .spacing(spacing::LG)
            .align_y(Alignment::Center)
            .wrap()
            .vertical_spacing(spacing::SM);

        match self.direction {
            ReadingDirection::LeftToRight => wrapping.into(),
            ReadingDirection::RightToLeft => wrapping.align_x(Alignment::End).into(),
        }
    }

    /// The label and select of the page size, when there are sizes to
    /// choose from.
    fn page_size_selector(&self) -> Option<Element<'a, Message>> {
        let on_change = self.on_page_size_change.clone()?;
        let first = *self.page_sizes.first()?;
        let current = self.page_size.unwrap_or(first);
        let collection = ListCollection::new(self.page_sizes.iter().copied()).ok()?;
        let value = current.to_string();

        let mut select = menu::select(&collection, Some(&value))
            .width(Length::Fill)
            .open(self.page_size_open)
            .on_change(move |value| on_change(value.parse().unwrap_or(current)));

        if let Some(on_open_change) = self.on_page_size_open_change.clone() {
            select = select.on_open_change(move |open| on_open_change(open));
        }

        let mut parts: Vec<Element<'a, Message>> = vec![
            styled_text(self.localizations.items_per_page, typography::BODY_MEDIUM).into(),
            container(select).width(PAGE_SIZE_WIDTH).into(),
        ];

        if self.direction == ReadingDirection::RightToLeft {
            parts.reverse();
        }

        Some(
            Row::with_children(parts)
                .spacing(spacing::SM)
                .align_y(Alignment::Center)
                .into(),
        )
    }
}

impl<'a, Message: Clone + 'a> From<Pagination<'a, Message>> for Element<'a, Message> {
    fn from(pagination: Pagination<'a, Message>) -> Self {
        responsive(move |size| pagination.view(size.width))
            .height(Length::Shrink)
            .into()
    }
}

/// The chevron icon that points toward the end of the reading `direction`
/// when `forward`, toward its start otherwise.
fn chevron(direction: ReadingDirection, forward: bool) -> crate::icon::Icon {
    match direction.chevron(forward) {
        Side::Left => lucide::CHEVRON_LEFT,
        Side::Right => lucide::CHEVRON_RIGHT,
    }
}

/// The button of page `page`, filled when it is the `current` page, at
/// least as wide and tall as the control height of the density.
fn page_button<'a, Message: Clone + 'a>(
    page: u32,
    current: bool,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let label = min_size(
        styled_text(page.to_string(), typography::LABEL_LARGE),
        Minimum::Control(2.0 * spacing::SM),
        Minimum::Control(0.0),
    );
    let native = iced::widget::Button::new(label)
        .padding([0.0, spacing::SM])
        .on_press_maybe(on_press.clone())
        .style(move |theme: &Theme, status| page_style(theme, status, current));

    let mut wrapper = focusable(native)
        .radius(radius::MEDIUM)
        .enabled(on_press.is_some());

    if let Some(message) = on_press {
        wrapper = wrapper.on_activate(message);
    }

    wrapper.into()
}

/// The colors of a page button: the selected fill on the current page,
/// transparent with the surface text color on the others.
fn page_style(theme: &Theme, status: Status, current: bool) -> Style {
    let roles = theme.roles();
    let fill = roles.selected_fill();
    let on_fill = if fill == roles.primary {
        roles.on_primary
    } else {
        roles.on_indicator()
    };
    let (background, foreground) = if current {
        (fill, on_fill)
    } else {
        (TRANSPARENT, roles.on_surface)
    };

    let (background, text_color) = match status {
        Status::Active => (background, foreground),
        Status::Hovered => (
            state_layer(background, foreground, HOVER_OPACITY),
            foreground,
        ),
        Status::Pressed => (
            state_layer(background, foreground, PRESSED_OPACITY),
            foreground,
        ),
        Status::Disabled => (
            TRANSPARENT,
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
        ),
    };

    Style {
        background: (background != TRANSPARENT).then_some(Background::Color(background)),
        text_color,
        border: Border::default().rounded(radius::MEDIUM),
        ..Style::default()
    }
}

#[cfg(test)]
mod tests {
    use super::items;

    #[test]
    fn page_items_keep_their_count_and_use_ellipses() {
        assert_eq!(items(1, 5, 1, 1), [1, 2, 3, 4, 5].map(Some));
        assert_eq!(
            items(1, 10, 1, 1),
            [Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(10)]
        );
        assert_eq!(
            items(5, 10, 1, 1),
            [Some(1), None, Some(4), Some(5), Some(6), None, Some(10)]
        );
        assert_eq!(
            items(10, 10, 1, 1),
            [Some(1), None, Some(6), Some(7), Some(8), Some(9), Some(10)]
        );
        assert_eq!(
            items(4, 10, 1, 1),
            [Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(10)]
        );
        assert_eq!(items(1, 1, 1, 1), [Some(1)]);
        assert!(items(1, 0, 1, 1).is_empty());
    }

    #[test]
    fn the_count_stays_the_same_on_every_page() {
        for (siblings, boundary) in [(0, 0), (1, 1), (2, 1), (1, 2), (0, 3)] {
            for count in 1..=20 {
                let lengths: Vec<usize> = (1..=count)
                    .map(|page| items(page, count, siblings, boundary).len())
                    .collect();

                assert!(
                    lengths.windows(2).all(|pair| pair[0] == pair[1]),
                    "{count} pages, {siblings} siblings, {boundary} at the ends: {lengths:?}"
                );
                for page in 1..=count {
                    let shown = items(page, count, siblings, boundary);
                    assert!(shown.contains(&Some(page)));
                    assert!(shown.iter().flatten().all(|n| (1..=count).contains(n)));
                }
            }
        }
    }
}
