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

//! The TabBar page: tabs sharing the width, scrollable tabs and the
//! subtle style.

use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::tab_bar::{TabBarStyle, tab, tab_bar};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tab_bar.rs");

/// The tabs of the preview.
const SETTINGS: [&str; 3] = ["General", "Appearance", "About"];

/// The tabs of the scrollable example.
const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];

/// The tabs of the subtle example.
const ACCOUNT: [&str; 3] = ["Account", "Password", "Billing"];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    settings: usize,
    month: usize,
    account: usize,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A tab of the preview was chosen.
    Settings(usize),
    /// A month was chosen.
    Month(usize),
    /// A tab of the subtle bar was chosen.
    Account(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Settings(index) => state.settings = index,
        Message::Month(index) => state.month = index,
        Message::Account(index) => state.account = index,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            column![
                tab_bar(
                    [
                        tab("General").icon(lucide::SETTINGS),
                        tab("Appearance").icon(lucide::PALETTE),
                        tab("About").icon(lucide::INFO),
                    ],
                    state.settings,
                )
                .on_select(Message::Settings),
                container(text(format!("{} settings", SETTINGS[state.settings])))
                    .center(Length::Fill),
            ],
            // #endregion
        )
        .width(Length::Fixed(520.0))
        .height(Length::Fixed(160.0)),
    );

    let scrollable = doc::example(
        ctx,
        SOURCE,
        &["scrollable"],
        container(
            // #region scrollable
            tab_bar(MONTHS.map(tab), state.month)
                .scrollable(true)
                .on_select(Message::Month),
            // #endregion
        )
        .width(Length::Fixed(360.0)),
    );

    let subtle = doc::example(
        ctx,
        SOURCE,
        &["subtle"],
        container(
            // #region subtle
            column![
                tab_bar(ACCOUNT.map(tab), state.account)
                    .style(TabBarStyle::Subtle)
                    .on_select(Message::Account),
                container(text(format!("{} details", ACCOUNT[state.account]))).padding(spacing::MD),
            ],
            // #endregion
        )
        .width(Length::Fixed(520.0)),
    );

    doc::page(
        "TabBar",
        "A row of tabs with a rounded marker that slides to the selected one, for switching \
         between views of the same content.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the tabs with `tab`, each with a label and an optional `icon`. The \
                 selection lives in the application: `tab_bar` shows the selected index and \
                 publishes the index of a chosen tab through `on_select`; show the content of \
                 the selected tab below it. Without `on_select` the bar is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::tab_bar::{TabBarStyle, tab, tab_bar};",
            ),
            doc::note(
                "For tabs at the side of their content, use `vertical_tabs`; for a few short \
                 options that change a view, a segment group.",
            ),
            doc::heading("Examples"),
            doc::subheading("Scrollable"),
            doc::prose(
                "By default the tabs share the bar width equally. With `scrollable(true)`, \
                 the tabs keep their own width, start at the leading edge and scroll with the \
                 mouse wheel when they don't fit; the bar scrolls to keep the selected and \
                 the focused tab in view.",
            ),
            scrollable,
            doc::subheading("Subtle style"),
            doc::prose(
                "`TabBarStyle::Subtle` tints the selected tab with a faint accent fill and \
                 accent text instead of the pill. Its tabs always keep their own width and \
                 start at the leading edge.",
            ),
            subtle,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to the selected tab, and out of the bar.",
                    ],
                    vec![
                        "Left, Right",
                        "Select the previous or next tab, stopping at the ends.",
                    ],
                    vec!["Home, End", "Select the first or the last tab."],
                    vec!["Enter, Space", "Publish the focused tab again."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tab_bar::tab_bar",
                    "oito2_iced::widget::tab_bar::TabBar",
                    "oito2_iced::widget::tab_bar::tab",
                    "oito2_iced::widget::tab_bar::Tab",
                    "oito2_iced::widget::tab_bar::TabBarStyle",
                ],
            ),
        ],
    )
}
