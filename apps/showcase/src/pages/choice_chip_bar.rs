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
//! The ChoiceChipBar page: a scrolling row and a wrap of chips, with
//! single and multiple selection.

use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::chips::{BarStyle, choice_chip_bar};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("choice_chip_bar.rs");

/// The categories the chips filter by.
const CATEGORIES: [&str; 10] = [
    "All",
    "Audio",
    "Development",
    "Education",
    "Games",
    "Graphics",
    "Internet",
    "Office",
    "Science",
    "Utilities",
];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    // #region state
    selected: Vec<bool>,
    // #endregion
    sort: usize,
    plain: Vec<bool>,
}

impl Default for State {
    fn default() -> Self {
        let first = CATEGORIES.iter().enumerate().map(|(index, _)| index == 0);

        Self {
            selected: first.clone().collect(),
            sort: 0,
            plain: first.collect(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A chip of the shared selection was activated.
    Toggle(usize),
    /// A sort order was chosen.
    Sort(usize),
    /// A chip of the fill-only bar was activated.
    Plain(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region toggle
        Message::Toggle(index) => {
            if let Some(selected) = state.selected.get_mut(index) {
                *selected = !*selected;
            }
        }
        // #endregion
        Message::Sort(index) => state.sort = index,
        Message::Plain(index) => {
            if let Some(selected) = state.plain.get_mut(index) {
                *selected = !*selected;
            }
        }
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let width = Length::Fixed(420.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["state", "toggle", "preview"],
        container(
            // #region preview
            choice_chip_bar(CATEGORIES, &state.selected)
                .localizations(l10n)
                .on_select(Message::Toggle),
            // #endregion
        )
        .width(width),
    );

    let scrolling = doc::example(
        ctx,
        SOURCE,
        &["row"],
        container(
            // #region row
            choice_chip_bar(CATEGORIES, &state.selected)
                .style(BarStyle::Row)
                .scroll_step(160.0)
                .localizations(l10n)
                .on_select(Message::Toggle),
            // #endregion
        )
        .width(width),
    );

    let wrap = doc::example(
        ctx,
        SOURCE,
        &["wrap"],
        container(
            // #region wrap
            choice_chip_bar(CATEGORIES, &state.selected)
                .style(BarStyle::Wrap)
                .on_select(Message::Toggle),
            // #endregion
        )
        .width(width),
    );

    let pills = doc::example(
        ctx,
        SOURCE,
        &["pills"],
        container(
            // #region pills
            choice_chip_bar(CATEGORIES, &state.selected)
                .style(BarStyle::Wrap)
                .pill(true)
                .on_select(Message::Toggle),
            // #endregion
        )
        .width(width),
    );

    let single = doc::example(
        ctx,
        SOURCE,
        &["single"],
        container(
            // #region single
            choice_chip_bar(
                ["Name", "Date", "Size"],
                &[0, 1, 2].map(|index| index == state.sort),
            )
            .style(BarStyle::Wrap)
            .on_select(Message::Sort),
            // #endregion
        )
        .width(width),
    );

    let chosen: Vec<&str> = CATEGORIES
        .iter()
        .zip(&state.plain)
        .filter_map(|(name, selected)| selected.then_some(*name))
        .collect();

    let listed: Element<'a, Message, Theme> = if chosen.is_empty() {
        text("Nothing selected").into()
    } else {
        column(
            chosen
                .into_iter()
                .map(|name| styled_text(name, typography::TITLE_MEDIUM).into()),
        )
        .spacing(spacing::XS)
        .into()
    };

    let fill = doc::example(
        ctx,
        SOURCE,
        &["fill"],
        container(
            column![
                // #region fill
                choice_chip_bar(CATEGORIES, &state.plain)
                    .show_checkmark(false)
                    .localizations(l10n)
                    .on_select(Message::Plain),
                // #endregion
                border_container(listed)
                    .padding(spacing::MD)
                    .width(Length::Fill),
            ]
            .spacing(spacing::MD),
        )
        .width(width),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        container(
            // #region disabled
            choice_chip_bar::<Message, _>(["Audio", "Games", "Office"], &[false, true, false])
                .style(BarStyle::Wrap),
            // #endregion
        )
        .width(width),
    );

    doc::page(
        "ChoiceChipBar",
        "A bar of chips for picking one or more filters, in a scrolling row or wrapped over \
         several rows.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`choice_chip_bar` is controlled: the labels are the chips, `selected` says \
                 which are selected, in the same order, and `on_select` publishes the index \
                 of the chip the user activated. The bar does not change the selection \
                 itself, so the same widget serves single and multiple choice. Without \
                 `on_select` every chip is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::chips::{BarStyle, choice_chip_bar};",
            ),
            doc::heading("Examples"),
            doc::subheading("Row with scroll arrows"),
            doc::prose(
                "The default `BarStyle::Row` keeps the chips on one line that scrolls \
                 sideways, with the mouse wheel too, and follows the keyboard focus. An \
                 arrow button and a soft edge shadow appear only on a side that hides \
                 chips; each press scrolls `scroll_step` pixels, 200 by default. The \
                 tooltips of the arrows come from `localizations`, or from \
                 `previous_label` and `next_label`.",
            ),
            scrolling,
            doc::subheading("Wrap"),
            doc::prose(
                "`BarStyle::Wrap` lays the chips over as many rows as they need, `spacing` \
                 apart in both directions.",
            ),
            wrap,
            doc::subheading("Pills"),
            doc::prose("The chips have the medium radius; `pill(true)` makes them pill-shaped."),
            pills,
            doc::subheading("Single choice"),
            doc::prose(
                "For one choice at a time, keep a single index and select only the chip \
                 that matches it.",
            ),
            single,
            doc::subheading("Fill only"),
            doc::prose(
                "With `show_checkmark(false)` a selected chip shows only the secondary \
                 container fill, without the check mark before its label. Here the \
                 selection is also listed below the bar.",
            ),
            fill,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_select` the whole bar is disabled: the chips keep showing \
                 their selection but ignore input.",
            ),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next chip or arrow button."],
                    vec!["`Space` / `Enter`", "Activates the focused chip or arrow."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::chips::choice_chip_bar",
                    "oito2_iced::widget::chips::ChoiceChipBar",
                    "oito2_iced::widget::chips::BarStyle",
                    "oito2_iced::widget::chips::SCROLL_STEP",
                ],
            ),
        ],
    )
}
