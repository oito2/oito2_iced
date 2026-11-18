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
//! The Checkbox page: two states, three states and disabled.

use iced::widget::{column, row};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::toggles::checkbox;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("checkbox.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    checked: bool,
    tristate: Option<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            checked: true,
            tristate: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The two-state box changed.
    Checked(Option<bool>),
    /// The three-state box changed.
    Tristate(Option<bool>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Checked(value) => state.checked = value.unwrap_or(false),
        Message::Tristate(value) => state.tristate = value,
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
        checkbox(state.checked).on_change(Message::Checked),
        // #endregion
    );

    let value = match state.tristate {
        Some(true) => "Value: checked",
        Some(false) => "Value: unchecked",
        None => "Value: mixed",
    };

    let tristate = doc::example(
        ctx,
        SOURCE,
        &["tristate"],
        column![
            // #region tristate
            checkbox(state.tristate)
                .tristate(true)
                .on_change(Message::Tristate),
            // #endregion
            doc::caption(value),
        ]
        .align_x(Alignment::Center),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        row![
            checkbox(false),
            checkbox(true),
            checkbox(None).tristate(true),
            checkbox(true).enabled(false).on_change(Message::Checked),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    doc::page(
        "Checkbox",
        "A box that is checked or unchecked, with an optional mixed state, painted in the \
         OITO2 style.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Pass the current value to `checkbox` and update it in `on_change`, which \
                 receives the next value as an `Option<bool>`. The value is checked with \
                 `true` or `Some(true)`, unchecked with `false` or `Some(false)`, and mixed \
                 with `None`. A change animates the box as the check mark draws itself.",
            ),
            doc::code(ctx, "use oito2_iced::widget::toggles::checkbox;"),
            doc::note(
                "A check box usually comes with a label. `check_button` adds a clickable \
                 one beside the box, and `checkbox_list_tile` puts the box in a list \
                 tile.",
            ),
            doc::heading("Examples"),
            doc::subheading("Three states"),
            doc::prose(
                "With `tristate(true)`, a click moves from unchecked to checked, from \
                 checked to mixed, and from mixed back to unchecked. Without it, a mixed \
                 box becomes unchecked on the next click and never mixed again.",
            ),
            tristate,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change`, or with `enabled(false)`, the box shows its value \
                 dimmed, ignores clicks and keys, and Tab skips it.",
            ),
            disabled,
            doc::subheading("Size"),
            doc::prose(
                "The box is 20 pixels in a 34-pixel target for a pointer, and 24 pixels in \
                 a 48-pixel target on a touch screen, following the density.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the box, and out of it."],
                    vec!["`Space` / `Enter`", "Toggles the focused box."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::toggles::checkbox",
                    "oito2_iced::widget::toggles::Checkbox",
                    "oito2_iced::widget::toggles::TOGGLE_DURATION",
                ],
            ),
        ],
    )
}
