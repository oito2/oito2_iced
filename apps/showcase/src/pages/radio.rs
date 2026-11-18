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

//! The Radio page: a group, a toggleable group and disabled radios.

use iced::widget::row;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::toggles;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("radio.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    size: Option<u8>,
    option: Option<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            size: Some(1),
            option: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A size was chosen.
    Size(Option<u8>),
    /// An option of the toggleable group was chosen, or the group cleared.
    Option(Option<u8>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Size(size) => state.size = size,
        Message::Option(option) => state.option = option,
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
        row((0..3).map(|size| {
            toggles::radio(size, state.size)
                .on_change(Message::Size)
                .into()
        }))
        .spacing(spacing::XS),
        // #endregion
    );

    let toggleable = doc::example(
        ctx,
        SOURCE,
        &["toggleable"],
        // #region toggleable
        row((0..3).map(|option| {
            toggles::radio(option, state.option)
                .toggleable(true)
                .on_change(Message::Option)
                .into()
        }))
        .spacing(spacing::XS),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        row![
            toggles::radio::<u8, Message>(0, Some(1)),
            toggles::radio::<u8, Message>(1, Some(1)),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    let toggleable_state = match state.option {
        Some(option) => format!("Option {} selected.", option + 1),
        None => "No option selected.".to_owned(),
    };

    doc::page(
        "Radio",
        "A round button that selects one value of a group.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give each `radio` of a group its own value and the same selection, the \
                 selected value. A radio is selected when both are equal; activating it \
                 publishes the message of `on_change` with `Some(value)`, and activating \
                 the selected one does nothing. Without `on_change` the radio is disabled.",
            ),
            doc::code(ctx, "use oito2_iced::widget::toggles;"),
            doc::note(
                "A radio usually comes with a label. `controls::radio_button` adds a \
                 clickable one beside it, and `list_tile::radio_list_tile` puts the radio \
                 in a list tile.",
            ),
            doc::heading("Examples"),
            doc::subheading("Toggleable"),
            doc::prose(
                "With `toggleable(true)`, activating the selected radio clears the group: \
                 `on_change` receives `None`.",
            ),
            toggleable,
            doc::caption(toggleable_state),
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change`, or with `enabled(false)`, the radio still shows \
                 whether it is selected, dimmed, and ignores clicks and keys.",
            ),
            disabled,
            doc::subheading("Size"),
            doc::prose(
                "The ring is 20 pixels for a pointer and 24 on a touch screen, centered \
                 in a square target of the control size of the density.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next radio."],
                    vec![
                        "Enter, Space",
                        "Selects the focused radio, or clears it when toggleable.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::toggles::radio",
                    "oito2_iced::widget::toggles::Radio",
                ],
            ),
        ],
    )
}
