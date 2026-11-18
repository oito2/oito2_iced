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
//! The CheckButton page: a check box with a clickable label, with a
//! subtitle, three states and disabled.

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::widget::controls::check_button;
use oito2_iced::widget::toggles::checkbox;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("check_button.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    symlinks: bool,
    backups: bool,
    hidden: Option<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            symlinks: true,
            backups: false,
            hidden: Some(false),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The symbolic links option changed.
    Symlinks(Option<bool>),
    /// The backups option changed.
    Backups(Option<bool>),
    /// The three-state option changed.
    Hidden(Option<bool>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Symlinks(value) => state.symlinks = value.unwrap_or(false),
        Message::Backups(value) => state.backups = value.unwrap_or(false),
        Message::Hidden(value) => state.hidden = value,
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
        check_button(
            checkbox(state.symlinks).on_change(Message::Symlinks),
            "Follow symbolic links",
        ),
        // #endregion
    );

    let subtitle = doc::example(
        ctx,
        SOURCE,
        &["subtitle"],
        // #region subtitle
        check_button(
            checkbox(state.backups).on_change(Message::Backups),
            "Keep backups",
        )
        .subtitle("Saves a copy before each change"),
        // #endregion
    );

    let tristate = doc::example(
        ctx,
        SOURCE,
        &["tristate"],
        // #region tristate
        check_button(
            checkbox(state.hidden)
                .tristate(true)
                .on_change(Message::Hidden),
            "Show hidden files",
        )
        .subtitle("Checked, mixed or unchecked"),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        check_button(checkbox(true), "Disabled option"),
        // #endregion
    );

    let at_end = doc::example(
        ctx,
        SOURCE,
        &["at-end"],
        // #region at-end
        check_button(
            checkbox(state.backups).on_change(Message::Backups),
            "Keep backups",
        )
        .control_at_end(true),
        // #endregion
    );

    doc::page(
        "CheckButton",
        "A check box with a clickable label, for an option that is on or off.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `check_button` a `checkbox` with the current value and its \
                 `on_change`, and the title. A click anywhere on the row, the box or the \
                 label, toggles it, publishing the message of the check box. A check box \
                 without `on_change` disables the button and dims its text.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::controls::check_button;\n\
                 use oito2_iced::widget::toggles::checkbox;",
            ),
            doc::note(
                "Use a bare `checkbox` when the label is elsewhere, such as in a table \
                 cell, and `checkbox_list_tile` in a list of tiles.",
            ),
            doc::heading("Examples"),
            doc::subheading("With a subtitle"),
            doc::prose("`subtitle` adds secondary text under the label."),
            subtitle,
            doc::subheading("Three states"),
            doc::prose(
                "With `tristate(true)` on the check box, `None` is the mixed state, shown \
                 as a dash. Each click moves from unchecked to checked, from checked to \
                 mixed, and from mixed back to unchecked.",
            ),
            tristate,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change`, the button keeps showing its value but cannot be \
                 changed, and Tab skips it.",
            ),
            disabled,
            doc::subheading("Box at the end"),
            doc::prose("`control_at_end(true)` places the box after the text."),
            at_end,
            doc::heading("Keyboard"),
            doc::prose(
                "The row takes the focus as a whole and shows the focus ring around \
                 itself; the box inside stays out of focus traversal.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the button, and out of it."],
                    vec!["`Space` / `Enter`", "Toggles the focused button."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::check_button",
                    "oito2_iced::widget::controls::ToggleButton",
                    "oito2_iced::widget::toggles::checkbox",
                ],
            ),
        ],
    )
}
