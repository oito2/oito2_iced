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
//! The CheckboxListTile page: list tiles with a check box, a radio or a
//! switch, toggled by a click anywhere on the tile.

use iced::widget::{column, container};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::list_tile::{
    ControlAffinity, checkbox_list_tile, radio_list_tile, switch_list_tile,
};
use oito2_iced::widget::toggles::{checkbox, radio, switch};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("checkbox_list_tile.rs");

/// A layout of the file view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Icons in a grid.
    Grid,
    /// Rows in a list.
    List,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    thumbnails: bool,
    view: View,
    hidden: bool,
    wifi: bool,
    all: Option<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            thumbnails: true,
            view: View::Grid,
            hidden: false,
            wifi: true,
            all: Some(false),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The thumbnails tile changed.
    Thumbnails(Option<bool>),
    /// A view tile was chosen.
    View(Option<View>),
    /// The hidden files tile changed.
    Hidden(bool),
    /// The Wi-Fi tile changed.
    Wifi(bool),
    /// The select all tile changed.
    All(Option<bool>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Thumbnails(value) => state.thumbnails = value.unwrap_or(false),
        Message::View(view) => {
            if let Some(view) = view {
                state.view = view;
            }
        }
        Message::Hidden(on) => state.hidden = on,
        Message::Wifi(on) => state.wifi = on,
        Message::All(value) => state.all = value,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let width = Length::Fixed(420.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            border_container(column![
                checkbox_list_tile(checkbox(state.thumbnails).on_change(Message::Thumbnails))
                    .title("Show thumbnails")
                    .secondary(icon(lucide::IMAGE)),
                radio_list_tile(radio(View::Grid, Some(state.view)).on_change(Message::View))
                    .title("Grid view"),
                radio_list_tile(radio(View::List, Some(state.view)).on_change(Message::View))
                    .title("List view"),
                switch_list_tile(switch(state.hidden).on_toggle(Message::Hidden))
                    .title("Hidden files")
                    .subtitle("Files whose name starts with a dot"),
            ])
            .padding(1)
            .width(Length::Fill),
            // #endregion
        )
        .width(width),
    );

    let affinity = doc::example(
        ctx,
        SOURCE,
        &["affinity"],
        container(
            // #region affinity
            switch_list_tile(switch(state.wifi).on_toggle(Message::Wifi))
                .title("Wi-Fi")
                .secondary(icon(lucide::WIFI))
                .control_affinity(ControlAffinity::Leading),
            // #endregion
        )
        .width(width),
    );

    let tristate = doc::example(
        ctx,
        SOURCE,
        &["tristate"],
        container(
            // #region tristate
            checkbox_list_tile(checkbox(state.all).tristate(true).on_change(Message::All))
                .title("Select all"),
            // #endregion
        )
        .width(width),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        container(
            // #region disabled
            checkbox_list_tile(checkbox(false)).title("Unavailable option"),
            // #endregion
        )
        .width(width),
    );

    doc::page(
        "CheckboxListTile",
        "List tiles with a check box, a radio or a switch, toggled by a click anywhere on \
         the tile.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Each tile is a list tile around a control: `checkbox_list_tile` takes a \
                 `checkbox`, `radio_list_tile` a `radio` and `switch_list_tile` a `switch`, \
                 each with its value and change message. A click anywhere on the tile, \
                 Enter or Space changes the value, publishing the message of the control. \
                 A control without a change message disables the whole tile.",
            ),
            doc::prose(
                "`title` and `subtitle` set the text, and a `secondary` element, such as \
                 an icon, sits on the side opposite the control. `padding` changes the \
                 space around the content.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::list_tile::{checkbox_list_tile, switch_list_tile};\n\
                 use oito2_iced::widget::toggles::{checkbox, switch};",
            ),
            doc::note(
                "Use these tiles in lists and settings panels. For a compact row outside a \
                 list, use `check_button`, `radio_button` or `switch_button`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Control side"),
            doc::prose(
                "`control_affinity` places the control. With the default, \
                 `ControlAffinity::Platform`, check boxes and radios lead and switches \
                 trail; `Leading` and `Trailing` force a side.",
            ),
            affinity,
            doc::subheading("Three states"),
            doc::prose(
                "With `tristate(true)` on its check box, the tile cycles through the mixed \
                 state, `None`, shown as a dash.",
            ),
            tristate,
            doc::subheading("Disabled"),
            doc::prose(
                "Without a change message, the tile and its control are dimmed, ignore \
                 clicks and are skipped by Tab.",
            ),
            disabled,
            doc::heading("Keyboard"),
            doc::prose(
                "The tile takes the focus as a whole; the control inside stays out of focus \
                 traversal.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next tile."],
                    vec!["`Space` / `Enter`", "Toggles the focused tile."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::list_tile::checkbox_list_tile",
                    "oito2_iced::widget::list_tile::radio_list_tile",
                    "oito2_iced::widget::list_tile::switch_list_tile",
                    "oito2_iced::widget::list_tile::ToggleListTile",
                    "oito2_iced::widget::list_tile::ControlAffinity",
                ],
            ),
        ],
    )
}
