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

//! The SwitchButton page: switches with a clickable label.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::widget::{controls, toggles};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("switch_button.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    sounds: bool,
    dark: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sounds: true,
            dark: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The alert sounds switch changed.
    Sounds(bool),
    /// The dark style switch changed.
    Dark(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Sounds(on) => state.sounds = on,
        Message::Dark(on) => state.dark = on,
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
        controls::switch_button(
            toggles::switch(state.sounds).on_toggle(Message::Sounds),
            "Alert sounds",
        ),
        // #endregion
    );

    let subtitle = doc::example(
        ctx,
        SOURCE,
        &["subtitle"],
        container(
            // #region subtitle
            controls::switch_button(
                toggles::switch(state.dark).on_toggle(Message::Dark),
                "Dark style",
            )
            .subtitle("Applies to this window"),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        controls::switch_button(toggles::switch::<Message>(false), "Unavailable option"),
        // #endregion
    );

    doc::page(
        "SwitchButton",
        "A switch with a clickable label before it, for a setting that takes effect at once.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`switch_button` puts a title before a `switch`; a click anywhere on the \
                 row, or Enter or Space while it has the focus, publishes the message of \
                 the switch's `on_toggle`. A switch without `on_toggle` disables the row \
                 and dims its text.",
            ),
            doc::code(ctx, "use oito2_iced::widget::{controls, toggles};"),
            doc::note("In a list of tiles, use `list_tile::switch_list_tile`."),
            doc::heading("Examples"),
            doc::subheading("With a subtitle"),
            doc::prose(
                "`subtitle` adds secondary text under the title. Give the row a bounded \
                 width so the text wraps.",
            ),
            subtitle,
            doc::subheading("Disabled"),
            doc::prose("Without `on_toggle`, the row keeps showing the value but can't change it."),
            disabled,
            doc::note(
                "The row is a single focus stop with the focus ring around itself; the \
                 switch inside never takes the focus.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the row, and out of it."],
                    vec!["Enter, Space", "Toggles the focused row."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::switch_button",
                    "oito2_iced::widget::controls::ToggleButton",
                    "oito2_iced::widget::toggles::switch",
                ],
            ),
        ],
    )
}
