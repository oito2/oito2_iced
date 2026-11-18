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

//! The Pagination page: a page size selector, the compact form, more page
//! numbers and the page list function.

use iced::widget::{container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::widget::pagination::{items, pagination};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("pagination.rs");

/// The number of items the page size example splits into pages.
const ITEMS: u32 = 480;

/// The state of the page.
#[derive(Debug)]
pub struct State {
    preview: u32,
    page: u32,
    size: u32,
    size_open: bool,
    small: u32,
    wide: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            preview: 1,
            page: 4,
            size: 20,
            size_open: false,
            small: 1,
            wide: 10,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A page of the preview was chosen.
    Preview(u32),
    /// A page of the page size example was chosen.
    Page(u32),
    /// A page size was chosen.
    Size(u32),
    /// The page size menu opened or closed.
    SizeOpen(bool),
    /// A page of the compact example was chosen.
    Small(u32),
    /// A page of the wide example was chosen.
    Wide(u32),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Preview(page) => state.preview = page,
        // #endregion
        Message::Page(page) => state.page = page,
        // #region page-size-update
        Message::Size(size) => {
            state.size = size;
            state.page = 1;
            state.size_open = false;
        }
        Message::SizeOpen(open) => state.size_open = open,
        // #endregion
        Message::Small(page) => state.small = page,
        Message::Wide(page) => state.wide = page,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        // #region preview
        pagination(state.preview, 10)
            .on_change(Message::Preview)
            .localizations(l10n),
        // #endregion
    );

    let page_size = doc::example(
        ctx,
        SOURCE,
        &["page-size", "page-size-update"],
        // #region page-size
        pagination(state.page, ITEMS.div_ceil(state.size))
            .on_change(Message::Page)
            .page_sizes([10, 20, 50, 100], state.size, Message::Size)
            .page_size_menu(state.size_open, Message::SizeOpen)
            .localizations(l10n),
        // #endregion
    );

    let compact = doc::example(
        ctx,
        SOURCE,
        &["compact"],
        container(
            // #region compact
            pagination(state.small, 8)
                .on_change(Message::Small)
                .localizations(l10n),
            // #endregion
        )
        .width(Length::Fixed(300.0)),
    );

    let siblings = doc::example(
        ctx,
        SOURCE,
        &["siblings"],
        // #region siblings
        pagination(state.wide, 20)
            .sibling_count(2)
            .boundary_count(2)
            .on_change(Message::Wide)
            .localizations(l10n),
        // #endregion
    );

    let list = doc::example(
        ctx,
        SOURCE,
        &["items"],
        // #region items
        text(
            items(state.wide, 20, 1, 1)
                .into_iter()
                .map(|page| page.map_or_else(|| "…".to_owned(), |page| page.to_string()))
                .collect::<Vec<_>>()
                .join("  "),
        ),
        // #endregion
    );

    doc::page(
        "Pagination",
        "Moves between the pages of a long list with previous and next buttons, page numbers \
         and an optional page size selector.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `pagination` the current page and the number of pages, counting from 1, \
                 and update the page from `on_change`; without it every button is disabled. \
                 `localizations` takes the tooltips of the arrows, the \"Page 3 of 10\" text \
                 and the label of the page size selector from the strings of the language.",
            ),
            doc::code(ctx, "use oito2_iced::widget::pagination::pagination;"),
            doc::prose(
                "The page numbers keep the first and last pages and the pages next to the \
                 current one, with an ellipsis for the others. An ellipsis never stands for a \
                 single page, and the number of buttons stays the same for a given page \
                 count, so they don't shift as the page changes.",
            ),
            doc::heading("Examples"),
            doc::subheading("Page size"),
            doc::prose(
                "`page_sizes` adds an \"Items per page\" selector after the buttons, showing \
                 the chosen size, and `page_size_menu` gives it the open state of its menu, \
                 which lives in the application. Here 480 items are split by the chosen size, \
                 and changing it returns to the first page.",
            ),
            page_size,
            doc::subheading("Compact"),
            doc::prose(
                "Narrower than `compact_width`, 420 pixels by default, only the previous and \
                 next buttons remain, around \"Page 1 of 8\". A page size selector wraps below \
                 them.",
            ),
            compact,
            doc::subheading("More page numbers"),
            doc::prose(
                "`sibling_count` sets how many pages show on each side of the current one, \
                 and `boundary_count` how many at each end; both are 1 by default.",
            ),
            siblings,
            doc::subheading("Page list"),
            doc::prose(
                "`items` returns the page numbers a pagination shows, with `None` for each \
                 ellipsis, for building a custom one. This list follows the example above \
                 with the default counts.",
            ),
            list,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next button."],
                    vec!["`Enter` / `Space`", "Activates the focused button."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::pagination::pagination",
                    "oito2_iced::widget::pagination::Pagination",
                    "oito2_iced::widget::pagination::items",
                    "oito2_iced::widget::pagination::COMPACT_WIDTH",
                ],
            ),
        ],
    )
}
