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

//! The Switch page: plain, with on and off shapes, right to left and
//! disabled.

use iced::widget::row;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::toggles;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("switch.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    plain: bool,
    shapes: bool,
    mirrored: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            plain: true,
            shapes: false,
            mirrored: true,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview switch changed.
    Plain(bool),
    /// The switch with shapes changed.
    Shapes(bool),
    /// The right-to-left switch changed.
    Mirrored(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Plain(on) => state.plain = on,
        Message::Shapes(on) => state.shapes = on,
        Message::Mirrored(on) => state.mirrored = on,
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
        toggles::switch(state.plain).on_toggle(Message::Plain),
        // #endregion
    );

    let shapes = doc::example(
        ctx,
        SOURCE,
        &["shapes"],
        // #region shapes
        toggles::switch(state.shapes)
            .on_off_shapes(true)
            .on_toggle(Message::Shapes),
        // #endregion
    );

    let mirrored = doc::example(
        ctx,
        SOURCE,
        &["direction"],
        // #region direction
        toggles::switch(state.mirrored)
            .direction(ReadingDirection::RightToLeft)
            .on_toggle(Message::Mirrored),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        row![
            toggles::switch::<Message>(false),
            toggles::switch::<Message>(true),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    doc::page(
        "Switch",
        "A track with a sliding thumb that turns a setting on or off.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Pass the current value to `switch` and update it in the message of \
                 `on_toggle`, which receives the new value. Without `on_toggle`, or with \
                 `enabled(false)`, the switch is disabled.",
            ),
            doc::code(ctx, "use oito2_iced::widget::toggles;"),
            doc::note(
                "A switch usually comes with a label. `controls::switch_button` adds a \
                 clickable one before it, and `list_tile::switch_list_tile` puts the \
                 switch in a list tile.",
            ),
            doc::heading("Examples"),
            doc::subheading("On and off shapes"),
            doc::prose(
                "With `on_off_shapes(true)`, the track also shows an \"I\" while on and an \
                 \"O\" while off, so the state doesn't rely on color alone.",
            ),
            shapes,
            doc::subheading("Reading direction"),
            doc::prose(
                "The thumb of an on switch sits at the end of the reading direction: the \
                 right by default, the left with `direction(ReadingDirection::RightToLeft)`.",
            ),
            mirrored,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_toggle`, the switch shows its value dimmed and ignores clicks \
                 and keys.",
            ),
            disabled,
            doc::subheading("Size"),
            doc::prose(
                "The track is 42 × 24 pixels for a pointer and 52 × 30 on a touch screen, \
                 in a target 8 pixels wider.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the switch, and out of it."],
                    vec!["Enter, Space", "Toggles the focused switch."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::toggles::switch",
                    "oito2_iced::widget::toggles::Switch",
                ],
            ),
        ],
    )
}
