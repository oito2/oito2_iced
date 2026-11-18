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
//! The ContextMenuRegion page: a menu that opens over an element on a
//! right click or from the keyboard, with check marks, groups and a
//! disabled region.

use iced::widget::{column, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::context_menu::context_menu;
use oito2_iced::widget::info::banner;
use oito2_iced::widget::menu;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("context_menu_region.rs");

/// A menu of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// The menu of the preview.
    Preview,
    /// The menu with check marks and groups.
    Groups,
    /// The menu whose open state is shown.
    Status,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    open: Option<Menu>,
    picked: &'static str,
    pinned: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: None,
            picked: "Nothing yet",
            pinned: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A menu opened (true) or closed (false).
    Open(Menu, bool),
    /// An entry was chosen.
    Pick(&'static str),
    /// The pinned entry was toggled.
    Pinned(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Open(menu, true) => state.open = Some(menu),
        Message::Open(menu, false) => {
            if state.open == Some(menu) {
                state.open = None;
            }
        }
        Message::Pick(picked) => state.picked = picked,
        Message::Pinned(pinned) => state.pinned = pinned,
    }

    Task::none()
}

/// A focusable card the menus of this page open over; activating it
/// picks "Open".
fn card<'a>(label: impl text::IntoFragment<'a>) -> Element<'a, Message, Theme> {
    banner(
        text(label)
            .width(Length::Fill)
            .height(Length::Fixed(88.0))
            .center(),
    )
    .width(Length::Fixed(280.0))
    .on_press(Message::Pick("Open"))
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        context_menu(
            card(format!("Right-click or press Shift+F10: {}", state.picked)),
            [
                menu::item("Copy")
                    .icon(lucide::COPY)
                    .on_press(Message::Pick("Copy")),
                menu::item("Rename")
                    .icon(lucide::PENCIL)
                    .on_press(Message::Pick("Rename")),
                menu::item("Delete")
                    .icon(lucide::TRASH)
                    .on_press(Message::Pick("Delete")),
            ],
        )
        .open(state.open == Some(Menu::Preview))
        .on_open_change(|open| Message::Open(Menu::Preview, open))
        .direction(ctx.direction),
        // #endregion
    );

    let groups = doc::example(
        ctx,
        SOURCE,
        &["groups"],
        column![
            // #region groups
            context_menu(
                card("Report.pdf"),
                [
                    menu::item("Copy")
                        .icon(lucide::COPY)
                        .on_press(Message::Pick("Copy")),
                    menu::checked_item(state.pinned, "Pinned").on_change(Message::Pinned),
                    menu::separator(),
                    menu::header("Export"),
                    menu::item("PDF").on_press(Message::Pick("Export as PDF")),
                    menu::item("PNG").on_press(Message::Pick("Export as PNG")),
                    menu::item("Markdown"),
                    menu::separator(),
                    menu::item("Delete")
                        .icon(lucide::TRASH)
                        .on_press(Message::Pick("Delete")),
                ],
            )
            .open(state.open == Some(Menu::Groups))
            .on_open_change(|open| Message::Open(Menu::Groups, open)),
            // #endregion
            doc::caption(format!("Last choice: {}", state.picked)),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    );

    let status = doc::example(
        ctx,
        SOURCE,
        &["status"],
        // #region status
        context_menu(
            card(if state.open == Some(Menu::Status) {
                "Menu open"
            } else {
                "Menu closed"
            }),
            [menu::item("Open").on_press(Message::Pick("Open"))],
        )
        .open(state.open == Some(Menu::Status))
        .on_open_change(|open| Message::Open(Menu::Status, open)),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        context_menu(card("No menu here"), []),
        // #endregion
    );

    doc::page(
        "ContextMenuRegion",
        "Opens a menu of actions over an element on a right click or from the keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Wrap the element in `context_menu` and give it the menu items. The items \
                 are the ones of the `menu` module: plain items, items with a check mark, \
                 group headers and separators. The open state lives in the application: \
                 `on_open_change` publishes `true` to open the menu and `false` to close \
                 it, and `open` shows it.",
            ),
            doc::prose(
                "A right click opens the menu where the pointer is. Shift+F10 or the Menu \
                 key open it over the center of the region, with its first enabled item \
                 highlighted, which needs the focus inside the region: the cards of this \
                 page are banners, which take the focus.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::{context_menu::context_menu, menu};",
            ),
            doc::note(
                "For a menu that opens from a visible button, use `popup_menu_button` \
                 instead.",
            ),
            doc::heading("Examples"),
            doc::subheading("Check marks and groups"),
            doc::prose(
                "`checked_item` shows a check mark when checked, and `on_change` publishes \
                 the state it takes. `header` names a group and `separator` draws a line \
                 between groups. An item without a message is disabled.",
            ),
            groups,
            doc::subheading("Open state"),
            doc::prose(
                "Since the open state is the application's, the content can follow it, for \
                 example to highlight the element the menu belongs to.",
            ),
            status,
            doc::subheading("Disabled"),
            doc::prose("Without `on_open_change` the menu never opens."),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Shift+F10` / `Menu`",
                        "Opens the menu over the center of the region, when the focus is \
                         inside it.",
                    ],
                    vec!["`Up` / `Down`", "Move between the enabled entries."],
                    vec!["`Home` / `End`", "Jump to the first or the last entry."],
                    vec!["`Enter` / `Space`", "Activates the highlighted entry."],
                    vec!["`Escape` / `Tab`", "Closes the menu."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::context_menu::context_menu",
                    "oito2_iced::widget::context_menu::ContextMenu",
                    "oito2_iced::widget::menu::item",
                    "oito2_iced::widget::menu::checked_item",
                    "oito2_iced::widget::menu::header",
                    "oito2_iced::widget::menu::separator",
                    "oito2_iced::widget::menu::Item",
                ],
            ),
        ],
    )
}
