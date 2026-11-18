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

//! The VerticalTabs page: a column of tabs beside their content, in both
//! styles, and the tab column alone.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::tab_bar::{TabBarStyle, tab, vertical_tab_bar, vertical_tabs};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("vertical_tabs.rs");

/// The tabs of the preview.
const SETTINGS: [&str; 3] = ["General", "Appearance", "About"];
/// The tabs of the subtle example.
const ACCOUNT: [&str; 3] = ["Account", "Password", "Billing"];
/// The tabs of the bar alone.
const FOLDERS: [&str; 3] = ["Inbox", "Sent", "Archive"];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    settings: usize,
    account: usize,
    folder: usize,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A tab of the preview was chosen.
    Settings(usize),
    /// A tab of the subtle example was chosen.
    Account(usize),
    /// A tab of the bar alone was chosen.
    Folder(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Settings(index) => state.settings = index,
        Message::Account(index) => state.account = index,
        Message::Folder(index) => state.folder = index,
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
            vertical_tabs(
                [
                    tab("General").icon(lucide::SETTINGS),
                    tab("Appearance").icon(lucide::PALETTE),
                    tab("About").icon(lucide::INFO),
                ],
                state.settings,
                container(text(format!("{} settings", SETTINGS[state.settings])))
                    .center(Length::Fill),
            )
            .tab_bar_width(180.0)
            .on_select(Message::Settings),
            // #endregion
        )
        .width(Length::Fixed(520.0))
        .height(Length::Fixed(180.0)),
    );

    let subtle = doc::example(
        ctx,
        SOURCE,
        &["subtle"],
        container(
            // #region subtle
            vertical_tabs(
                ACCOUNT.map(tab),
                state.account,
                container(text(format!("{} details", ACCOUNT[state.account]))).padding(spacing::MD),
            )
            .style(TabBarStyle::Subtle)
            .tab_bar_width(160.0)
            .on_select(Message::Account),
            // #endregion
        )
        .width(Length::Fixed(520.0))
        .height(Length::Fixed(180.0)),
    );

    let bar = doc::example(
        ctx,
        SOURCE,
        &["bar"],
        container(
            // #region bar
            row![
                container(
                    vertical_tab_bar(FOLDERS.map(tab), state.folder).on_select(Message::Folder)
                )
                .width(Length::Fixed(160.0)),
                column![
                    text(FOLDERS[state.folder]),
                    text("No messages in this folder."),
                ]
                .spacing(spacing::XS)
                .padding(spacing::MD),
            ]
            .spacing(spacing::MD),
            // #endregion
        )
        .width(Length::Fixed(520.0)),
    );

    doc::page(
        "VerticalTabs",
        "A column of tabs beside the content of the selected one, with a rounded marker that \
         slides between them.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `vertical_tabs` the tabs, the index of the selected one and the view of \
                 the selected tab, which fills the rest. The selection lives in the \
                 application: `on_select` publishes the index of a chosen tab, and without it \
                 the tabs are disabled. `tab_bar_width` sets the width of the tab column, 200 \
                 pixels by default, which scrolls when the tabs don't fit its height.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::tab_bar::{tab, vertical_tabs};",
            ),
            doc::heading("Examples"),
            doc::subheading("Subtle style"),
            doc::prose(
                "`TabBarStyle::Subtle` tints the selected tab with a faint accent fill and \
                 accent text instead of the pill.",
            ),
            subtle,
            doc::subheading("The tab column alone"),
            doc::prose(
                "To lay the content out yourself, use `vertical_tab_bar`, the tab column \
                 alone, and show the content of the selected tab wherever it fits.",
            ),
            bar,
            doc::heading("Keyboard"),
            doc::prose(
                "The column is one stop in keyboard navigation, and the arrow keys move \
                 between the tabs.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to the selected tab, then past the tabs.",
                    ],
                    vec![
                        "Up, Down",
                        "Selects the previous or next tab, stopping at the ends.",
                    ],
                    vec!["Home, End", "Selects the first or last tab."],
                    vec!["Enter, Space", "Publishes the focused tab again."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tab_bar::vertical_tabs",
                    "oito2_iced::widget::tab_bar::VerticalTabs",
                    "oito2_iced::widget::tab_bar::vertical_tab_bar",
                    "oito2_iced::widget::tab_bar::VerticalTabBar",
                    "oito2_iced::widget::tab_bar::tab",
                    "oito2_iced::widget::tab_bar::TabBarStyle",
                    "oito2_iced::widget::tab_bar::VERTICAL_TABS_WIDTH",
                ],
            ),
        ],
    )
}
