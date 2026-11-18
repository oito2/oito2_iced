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

//! The PageIndicator page: interactive dots, the spacing layouts, the
//! counter shown when the dots don't fit, the dot size and the target size.

use iced::widget::{column, container, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::page_indicator::{PageIndicatorLayout, page_indicator};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("page_indicator.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    page: usize,
    large: usize,
    small: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            page: 0,
            large: 1,
            small: 2,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A page of the preview or the counter was chosen.
    Page(usize),
    /// A page of the large dots was chosen.
    Large(usize),
    /// A page of the small targets example was chosen.
    Small(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Page(page) => state.page = page,
        // #endregion
        Message::Large(page) => state.large = page,
        Message::Small(page) => state.small = page,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        // #region preview
        page_indicator(6, state.page % 6)
            .on_select(Message::Page)
            .direction(ctx.direction),
        // #endregion
    );

    let layouts = doc::example(
        ctx,
        SOURCE,
        &["layouts"],
        column(
            [
                ("Fixed", PageIndicatorLayout::Fixed),
                ("Stepped", PageIndicatorLayout::Stepped),
                ("Bounded", PageIndicatorLayout::Bounded),
            ]
            .into_iter()
            .map(|(name, layout)| {
                row![
                    container(doc::caption(name)).width(Length::Fixed(80.0)),
                    // #region layouts
                    container(page_indicator(12, state.page % 12).layout(layout))
                        .width(Length::Fixed(200.0)),
                    // #endregion
                ]
                .spacing(spacing::MD)
                .align_y(Alignment::Center)
                .into()
            }),
        )
        .spacing(spacing::SM),
    );

    let counter = doc::example(
        ctx,
        SOURCE,
        &["counter"],
        container(
            // #region counter
            page_indicator(24, state.page).on_select(Message::Page),
            // #endregion
        )
        .width(Length::Fixed(72.0)),
    );

    let size = doc::example(
        ctx,
        SOURCE,
        &["size"],
        // #region size
        page_indicator(4, state.large)
            .dot_size(12.0)
            .spacing(spacing::MD)
            .on_select(Message::Large),
        // #endregion
    );

    let targets = doc::example(
        ctx,
        SOURCE,
        &["targets"],
        // #region targets
        row![
            icon_button(lucide::CHEVRON_LEFT)
                .tooltip("Previous")
                .on_press_maybe(state.small.checked_sub(1).map(Message::Small)),
            page_indicator(8, state.small)
                .min_target_size(0.0)
                .on_select(Message::Small),
            icon_button(lucide::CHEVRON_RIGHT)
                .tooltip("Next")
                .on_press_maybe((state.small < 7).then_some(Message::Small(state.small + 1))),
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center),
        // #endregion
    );

    doc::page(
        "PageIndicator",
        "Dots that show the current page of a carousel or a paged view, and can move to \
         another page.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `page_indicator` the number of pages and the current page, counting from \
                 0. The current page is a pill three dots wide in the text-safe primary tone; \
                 when the page changes, the old pill shrinks and the new one grows. With \
                 `on_select`, each dot publishes its page on a click and the dots are one stop \
                 of keyboard navigation; without it the dots only show the page.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::page_indicator::{PageIndicatorLayout, \
                 page_indicator};",
            ),
            doc::note(
                "`carousel` shows a page indicator below its pages. For pages with numbered \
                 buttons, use `pagination`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Spacing layouts"),
            doc::prose(
                "`layout` picks how the dots are spaced: `Fixed` always keeps `spacing` \
                 between them; `Stepped` brings them closer as pages are added, the full \
                 spacing up to 5 pages, three quarters up to 10 and half beyond; `Bounded` \
                 keeps the full spacing when it fits and otherwise draws the dots as close as \
                 needed to fit the width, down to half the dot size. Here each indicator shows \
                 12 pages in 200 pixels.",
            ),
            doc::note(
                "These indicators have no `on_select`, so they only show the page. An \
                 interactive dot sits in a target at least 24 pixels wide, and twelve of those \
                 do not fit in 200 pixels.",
            ),
            layouts,
            doc::subheading("Counter"),
            doc::prose(
                "When the dots do not fit the available width, the indicator shows the page \
                 as \"page / length\" text instead. Here 24 pages are given 72 pixels.",
            ),
            counter,
            doc::subheading("Dot size"),
            doc::prose(
                "`dot_size` sets the diameter of the dots, 8 pixels by default; the current \
                 page is three times as wide. `spacing` sets the space between them.",
            ),
            size,
            doc::subheading("Target size"),
            doc::prose(
                "An interactive dot sits in a target at least `min_target_size` wide and tall, \
                 24 pixels by default. When other controls, such as previous and next \
                 buttons, also move between the pages, a smaller value keeps the dots close \
                 together; 0 makes each target the size of its dot.",
            ),
            targets,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the dots."],
                    vec![
                        "`Left` / `Right`",
                        "Moves to the previous or next page in the reading direction: \
                         `Right` moves back in right-to-left text.",
                    ],
                    vec!["`Home` / `End`", "Moves to the first or the last page."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::page_indicator::page_indicator",
                    "oito2_iced::widget::page_indicator::PageIndicator",
                    "oito2_iced::widget::page_indicator::PageIndicatorLayout",
                    "oito2_iced::widget::page_indicator::DOT_SIZE",
                    "oito2_iced::widget::page_indicator::MIN_TARGET_SIZE",
                ],
            ),
        ],
    )
}
