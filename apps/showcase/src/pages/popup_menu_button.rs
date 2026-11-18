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

//! The PopupMenuButton page: popup menu buttons, their items, menus
//! anchored to any element and selects.

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::collections::ListCollection;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::widget::button;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::menu;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("popup_menu_button.rs");

/// The sort orders of the preview.
const SORTS: [&str; 3] = ["Name", "Size", "Modified"];

/// The columns of the multiple choice example.
const COLUMNS: [&str; 4] = ["Size", "Type", "Modified", "Owner"];

// #region select-data
/// The desktops of the select: value, label and toolkit, the group.
const DESKTOPS: [(&str, &str, &str); 5] = [
    ("gnome", "GNOME", "GTK"),
    ("cinnamon", "Cinnamon", "GTK"),
    ("xfce", "Xfce", "GTK"),
    ("kde", "KDE Plasma", "Qt"),
    ("lxqt", "LXQt (unavailable)", "Qt"),
];

/// The desktops grouped by toolkit, with LXQt disabled; `None` when two
/// values repeat.
fn desktops() -> Option<ListCollection<(&'static str, &'static str, &'static str)>> {
    ListCollection::with_value_of(DESKTOPS, |desktop| desktop.0.to_owned())
        .ok()
        .map(|desktops| {
            desktops
                .with_label_of(|desktop| desktop.1.to_owned())
                .with_group_of(|desktop| Some(desktop.2.to_owned()))
                .with_disabled(|desktop| desktop.0 == "lxqt")
        })
}
// #endregion

/// The menu that is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// The sort menu of the preview.
    Sort,
    /// The columns menu.
    Columns,
    /// The desktop select.
    Desktop,
    /// The menu of the more button.
    More,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    open: Option<Menu>,
    sort: &'static str,
    columns: Vec<&'static str>,
    desktops: Option<ListCollection<(&'static str, &'static str, &'static str)>>,
    desktop: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: None,
            sort: SORTS[0],
            columns: vec!["Size", "Modified"],
            desktops: desktops(),
            desktop: Some("gnome".to_owned()),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A menu opened, or closed when `false`.
    Open(Menu, bool),
    /// A sort order was chosen.
    Sort(&'static str),
    /// A column was shown or hidden.
    Column(&'static str, bool),
    /// A desktop was chosen.
    Desktop(String),
    /// An item of the more menu was activated.
    Action(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Open(menu, true) => state.open = Some(menu),
        Message::Open(menu, false) => {
            if state.open == Some(menu) {
                state.open = None;
            }
        }
        Message::Sort(sort) => state.sort = sort,
        Message::Column(column, shown) => {
            state.columns.retain(|shown| *shown != column);
            if shown {
                state.columns.push(column);
            }
        }
        Message::Desktop(desktop) => state.desktop = Some(desktop),
        Message::Action(action) => shell.toast(action, InfoType::Information),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let open = |menu: Menu| state.open == Some(menu);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        menu::popup_menu_button(
            format!("Sort by: {}", state.sort),
            SORTS.map(|sort| {
                menu::checked_item(state.sort == sort, sort).on_press(Message::Sort(sort))
            }),
        )
        .leading(icon(lucide::ARROW_DOWN_UP).size(18.0))
        .open(open(Menu::Sort))
        .on_open_change(|open| Message::Open(Menu::Sort, open)),
        // #endregion
    );

    let multiple = doc::example(
        ctx,
        SOURCE,
        &["multiple"],
        // #region multiple
        menu::popup_menu_button(
            format!("Columns ({})", state.columns.len()),
            COLUMNS.map(|column| {
                menu::multi_select_item(state.columns.contains(&column), column)
                    .on_change(move |shown| Message::Column(column, shown))
            }),
        )
        .open(open(Menu::Columns))
        .on_open_change(|open| Message::Open(Menu::Columns, open)),
        // #endregion
    );

    let select = state.desktops.as_ref().map(|desktops| {
        doc::example(
            ctx,
            SOURCE,
            &["select-data", "select"],
            // #region select
            menu::select(desktops, state.desktop.as_deref())
                .placeholder("Desktop")
                .open(open(Menu::Desktop))
                .on_open_change(|open| Message::Open(Menu::Desktop, open))
                .on_change(Message::Desktop),
            // #endregion
        )
    });

    let anchor = doc::example(
        ctx,
        SOURCE,
        &["anchor"],
        // #region anchor
        menu::anchor(
            button::icon_button(lucide::ELLIPSIS_VERTICAL)
                .tooltip("More")
                .on_press(Message::Open(Menu::More, !open(Menu::More))),
            [
                menu::header("Document"),
                menu::item("Rename")
                    .icon(lucide::PENCIL)
                    .on_press(Message::Action("Rename")),
                menu::item("Duplicate")
                    .icon(lucide::COPY)
                    .on_press(Message::Action("Duplicate")),
                menu::item("Share").icon(lucide::SHARE),
                menu::separator(),
                menu::item("Delete")
                    .icon(lucide::TRASH)
                    .on_press(Message::Action("Delete")),
            ],
        )
        .open(open(Menu::More))
        .on_close(Message::Open(Menu::More, false)),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        menu::popup_menu_button::<Message>("Disabled", []),
        // #endregion
    );

    doc::page(
        "PopupMenuButton",
        "A button that looks like a field and opens a menu of choices below it.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`popup_menu_button` shows its label, usually the current choice, with a \
                 chevron, and opens a menu of items below it. Use `checked_item` for a \
                 single choice, `multi_select_item` for several, and `item`, `header` and \
                 `separator` for plain entries. The button is as tall as the text fields.",
            ),
            doc::code(ctx, "use oito2_iced::widget::menu;"),
            doc::prose(
                "The open state lives in the application: `open` shows the menu, and \
                 `on_open_change` turns each request to open or close it into a message. \
                 Without `on_open_change` the button is disabled.",
            ),
            doc::note(
                "For a few short choices that fit side by side, a segment group is quicker \
                 to use. To pick a value of a collection, use `select`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Multiple choices"),
            doc::prose(
                "`multi_select_item` shows a check box, and `on_change` receives the state \
                 the item takes when activated. The menu stays open, so several items can be \
                 toggled in a row.",
            ),
            multiple,
            doc::subheading("Select"),
            doc::prose(
                "`select` picks one value of a `ListCollection`. The button shows the \
                 selected item's label, or `placeholder` when nothing is selected; the menu \
                 lists the items under their group names, with a check mark on the selected \
                 one and disabled items dimmed. Without `on_change` it is disabled.",
            ),
        ]
        .into_iter()
        .chain(select)
        .chain([
            doc::subheading("Menu on any element"),
            doc::prose(
                "`anchor` opens a menu below any element, such as an icon button that \
                 publishes the message that opens it. Items take an `icon`, and an item \
                 without `on_press` is dimmed and skipped by the arrow keys.",
            ),
            anchor,
            doc::subheading("Disabled"),
            doc::prose("Without `on_open_change` the button is dimmed and does not open its menu."),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Enter, Space", "Opens or closes the menu from the button."],
                    vec!["Down", "Opens the menu from the button."],
                    vec!["Up, Down", "Move between the enabled items."],
                    vec!["Home, End", "Move to the first or the last item."],
                    vec!["Enter, Space", "Activate the highlighted item."],
                    vec!["Escape, Tab", "Close the menu."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::menu::popup_menu_button",
                    "oito2_iced::widget::menu::PopupMenuButton",
                    "oito2_iced::widget::menu::item",
                    "oito2_iced::widget::menu::checked_item",
                    "oito2_iced::widget::menu::multi_select_item",
                    "oito2_iced::widget::menu::header",
                    "oito2_iced::widget::menu::separator",
                    "oito2_iced::widget::menu::Item",
                    "oito2_iced::widget::menu::anchor",
                    "oito2_iced::widget::menu::MenuAnchor",
                    "oito2_iced::widget::menu::select",
                    "oito2_iced::widget::menu::Select",
                ],
            ),
        ]),
    )
}
