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

//! The ToggleGroup page: joined buttons with one or several items on,
//! down, and disabled.

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::widget::toggle_group::{icon_item, item, toggle_group};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("toggle_group.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    styles: Vec<&'static str>,
    align: Vec<&'static str>,
    view: Vec<&'static str>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            styles: vec!["bold"],
            align: vec!["start"],
            view: vec!["week"],
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The text styles changed.
    Styles(Vec<&'static str>),
    /// The alignment changed.
    Align(Vec<&'static str>),
    /// The calendar view changed.
    View(Vec<&'static str>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Styles(styles) => state.styles = styles,
        Message::Align(align) => state.align = align,
        Message::View(view) => state.view = view,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        toggle_group(
            [
                icon_item("bold", lucide::LETTER_B).tooltip("Bold"),
                icon_item("italic", lucide::ITALIC).tooltip("Italic"),
                icon_item("underline", lucide::UNDERLINE).tooltip("Underline"),
            ],
            &state.styles,
        )
        .multiple(true)
        .on_change(Message::Styles),
        // #endregion
    );

    let single = doc::example(
        ctx,
        SOURCE,
        &["single"],
        // #region single
        toggle_group(
            [
                icon_item("start", lucide::TEXT_ALIGN_START).tooltip("Align left"),
                icon_item("center", lucide::TEXT_ALIGN_CENTER).tooltip("Center"),
                icon_item("end", lucide::TEXT_ALIGN_END).tooltip("Align right"),
            ],
            &state.align,
        )
        .deselectable(false)
        .on_change(Message::Align),
        // #endregion
    );

    let vertical = doc::example(
        ctx,
        SOURCE,
        &["vertical"],
        // #region vertical
        toggle_group(
            [
                item("day", "Day"),
                item("week", "Week"),
                item("year", "Year").enabled(false),
            ],
            &state.view,
        )
        .vertical(true)
        .on_change(Message::View),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        toggle_group::<_, Message>(
            [
                icon_item("list", lucide::LIST).tooltip("List"),
                icon_item("grid", lucide::LAYOUT_GRID).tooltip("Grid"),
            ],
            &["grid"],
        ),
        // #endregion
    );

    doc::page(
        "ToggleGroup",
        "A joined row of buttons that turn on and off, such as text formatting or alignment \
         buttons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The selection lives in the application: `toggle_group` shows the items of \
                 the `selected` values on, and `on_change` receives the new selection, in item \
                 order. `item` makes a text item, `icon_item` an icon item and `custom_item` \
                 an item with any content; give icon items a `tooltip`. Items that are on are \
                 raised from the track. Without `on_change` the group is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::toggle_group::{icon_item, item, toggle_group};",
            ),
            doc::prose(
                "With `multiple(true)`, as above, each item turns on and off by itself. \
                 Without it, turning one item on turns the others off.",
            ),
            doc::note(
                "Use a toggle group for options that turn on and off. For two to five \
                 mutually exclusive options that change a view, `segment_group` is clearer.",
            ),
            doc::heading("Examples"),
            doc::subheading("Always one on"),
            doc::prose(
                "In a single group the item that is on turns off when pressed again, leaving \
                 none on. With `deselectable(false)` it stays on, so one item is always on.",
            ),
            single,
            doc::subheading("Text items, down, with a disabled item"),
            doc::prose(
                "`vertical(true)` stacks the items, all as wide as the widest. An item with \
                 `enabled(false)` is dimmed and can't be toggled or focused.",
            ),
            vertical,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change` every item is disabled; the items that are on still show \
                 raised.",
            ),
            disabled,
            doc::heading("Keyboard"),
            doc::prose(
                "The group is one stop in keyboard navigation: Tab reaches the last focused \
                 item, or the first enabled one.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Left, Right",
                        "Moves to the previous or next enabled item, wrapping at the ends. Up \
                         and Down in a vertical group.",
                    ],
                    vec!["Home, End", "Moves to the first or last enabled item."],
                    vec!["Space, Enter", "Turns the focused item on or off."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::toggle_group::toggle_group",
                    "oito2_iced::widget::toggle_group::ToggleGroup",
                    "oito2_iced::widget::toggle_group::item",
                    "oito2_iced::widget::toggle_group::icon_item",
                    "oito2_iced::widget::toggle_group::custom_item",
                    "oito2_iced::widget::toggle_group::Item",
                ],
            ),
        ],
    )
}
