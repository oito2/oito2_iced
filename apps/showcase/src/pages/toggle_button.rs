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

//! The ToggleButton page: a clickable row that pairs any control with a
//! label.

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::widget::controls::toggle_button;
use oito2_iced::widget::swap::{SwapEffect, swap};
use oito2_iced::widget::toggles;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("toggle_button.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    sync: bool,
    notify: bool,
    preview: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sync: true,
            notify: false,
            preview: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The sync row was toggled.
    Sync(bool),
    /// The notify row was toggled.
    Notify(bool),
    /// The preview row was toggled.
    Preview(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Sync(sync) => state.sync = sync,
        Message::Notify(notify) => state.notify = notify,
        Message::Preview(preview) => state.preview = preview,
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
        toggle_button(
            toggles::checkbox(state.sync)
                .on_change(|value| Message::Sync(value.unwrap_or_default())),
            "Sync files",
        )
        .subtitle("Control before the text")
        .on_toggle(Message::Sync(!state.sync)),
        // #endregion
    );

    let end = doc::example(
        ctx,
        SOURCE,
        &["end"],
        // #region end
        toggle_button(
            toggles::switch(state.notify).on_toggle(Message::Notify),
            "Notify me",
        )
        .subtitle("Control after the text")
        .control_at_end(true)
        .on_toggle(Message::Notify(!state.notify)),
        // #endregion
    );

    let custom = doc::example(
        ctx,
        SOURCE,
        &["custom"],
        // #region custom
        toggle_button(
            swap(state.preview, icon(lucide::EYE_OFF), icon(lucide::EYE)).effect(SwapEffect::Scale),
            if state.preview {
                "Preview shown"
            } else {
                "Preview hidden"
            },
        )
        .on_toggle(Message::Preview(!state.preview)),
        // #endregion
    );

    doc::page(
        "ToggleButton",
        "A clickable row that pairs any toggle control with a label, the base of the check, \
         radio and switch buttons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `toggle_button` the control and the title, and optionally a \
                 `subtitle`. A click on the row outside the control, or Space or Enter while \
                 it has the focus, publishes the message of `on_toggle`, where you change the \
                 value the control shows. The control keeps its own input, which runs when the \
                 control itself is clicked, but never takes the focus.",
            ),
            doc::prose(
                "Without `on_toggle` the row is disabled and its text is dimmed, but not the \
                 control: disable the control too.",
            ),
            doc::code(ctx, "use oito2_iced::widget::controls::toggle_button;"),
            doc::note(
                "For a check box, radio or switch, `check_button`, `radio_button` and \
                 `switch_button` already wire the control and the row together. Use \
                 `toggle_button` for another control, or a different arrangement.",
            ),
            doc::heading("Examples"),
            doc::subheading("Control at the end"),
            doc::prose(
                "With `control_at_end(true)`, the control follows the text, as in a switch \
                 button.",
            ),
            end,
            doc::subheading("Custom control"),
            doc::prose(
                "The control can be any widget. Here a `swap` changes an eye icon, and the row \
                 handles the clicks.",
            ),
            custom,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to the row, and out of it; the control is skipped.",
                    ],
                    vec!["Space, Enter", "Publishes the `on_toggle` message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::toggle_button",
                    "oito2_iced::widget::controls::ToggleButton",
                    "oito2_iced::widget::controls::TOGGLE_BUTTON_PADDING",
                ],
            ),
        ],
    )
}
