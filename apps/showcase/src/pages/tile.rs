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

//! The Tile page: settings rows grouped in a bordered list.

use iced::widget::{container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::{tile, tile_list};
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::toggles;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tile.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    wifi: bool,
    bluetooth: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            wifi: true,
            bluetooth: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The Wi-Fi switch changed.
    Wifi(bool),
    /// The Bluetooth switch changed.
    Bluetooth(bool),
    /// The VPN row was pressed.
    Vpn,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Wifi(on) => state.wifi = on,
        Message::Bluetooth(on) => state.bluetooth = on,
        Message::Vpn => shell.toast("VPN settings", InfoType::Information),
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
            tile_list([
                tile()
                    .leading(icon(lucide::WIFI))
                    .title("Wi-Fi")
                    .subtitle("Home network")
                    .trailing(toggles::switch(state.wifi).on_toggle(Message::Wifi)),
                tile()
                    .leading(icon(lucide::BLUETOOTH))
                    .title("Bluetooth")
                    .subtitle("No devices")
                    .trailing(toggles::switch(state.bluetooth).on_toggle(Message::Bluetooth)),
                tile()
                    .leading(icon(lucide::SHIELD))
                    .title("VPN")
                    .subtitle("Opens the VPN settings")
                    .trailing(icon(lucide::CHEVRON_RIGHT))
                    .on_press(Message::Vpn),
                tile()
                    .leading(icon(lucide::PLANE))
                    .title("Airplane mode")
                    .subtitle("Unavailable on this device")
                    .enabled(false),
            ]),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let values = doc::example(
        ctx,
        SOURCE,
        &["values"],
        container(
            // #region values
            tile_list([
                tile().title("Version").trailing(text("2.1.0")),
                tile().title("License").trailing(text("GPL-3.0")),
                tile()
                    .title("Storage")
                    .subtitle("Local cache")
                    .trailing(text("128 MB")),
            ]),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let padding = doc::example(
        ctx,
        SOURCE,
        &["padding"],
        container(
            // #region padding
            tile_list([tile()
                .padding([spacing::XS, spacing::MD])
                .leading(icon(lucide::BELL))
                .title("Compact row")]),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    doc::page(
        "Tile",
        "A settings row with an icon, a title, a subtitle and a trailing control, grouped in \
         a bordered list.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`tile` lays out an optional `leading` widget, usually an icon, a `title`, a \
                 `subtitle` under it and a `trailing` widget, such as a switch or a value. \
                 `tile_list` puts tiles in a bordered container with dividers between them.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::containers::{tile, tile_list};",
            ),
            doc::prose(
                "A tile is static by default: the controls inside it handle their own input. \
                 With `on_press` the whole row is a button: it shows a hover overlay, takes \
                 the focus and publishes its message on a click, Enter or Space. \
                 `enabled(false)` dims the tile, ignores clicks and skips it in focus \
                 traversal.",
            ),
            doc::note(
                "For rows meant to be activated or selected, with hover, selection and the \
                 OITO2 focus ring, use `list_tile`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Read-only values"),
            doc::prose(
                "Without `on_press`, a tile with a text `trailing` shows a property and its \
                 value.",
            ),
            values,
            doc::subheading("Padding"),
            doc::prose(
                "`padding` sets the space around the content, 16 pixels across and 12 down by \
                 default.",
            ),
            padding,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to a tile with `on_press` or to the controls inside \
                         a static tile.",
                    ],
                    vec!["Enter, Space", "Activates a focused tile."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::containers::tile",
                    "oito2_iced::widget::containers::Tile",
                    "oito2_iced::widget::containers::tile_list",
                ],
            ),
        ],
    )
}
