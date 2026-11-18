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

//! The OptionButton page: icon option buttons, a color swatch and a
//! disabled button.

use iced::widget::{column, row};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::controls::{option_button, option_color};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("option_button.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    items: u32,
    accent: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            items: 3,
            accent: 0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// An item was added.
    Add,
    /// An item was removed.
    Remove,
    /// The next accent color was chosen.
    NextAccent,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Add => state.items += 1,
        Message::Remove => state.items = state.items.saturating_sub(1),
        Message::NextAccent => state.accent = (state.accent + 1) % Accent::ALL.len(),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            // #region preview
            row![
                option_button(icon(lucide::PLUS))
                    .label("Add")
                    .on_press(Message::Add),
                option_button(icon(lucide::MINUS))
                    .label("Remove")
                    .on_press_maybe((state.items > 0).then_some(Message::Remove)),
            ]
            .spacing(spacing::XS),
            // #endregion
            doc::caption(format!("{} items", state.items)),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let accent = Accent::ALL
        .get(state.accent)
        .copied()
        .unwrap_or(Accent::ALL[0]);
    let color = doc::example(
        ctx,
        SOURCE,
        &["color"],
        // #region color
        option_color(accent.color(), format!("Accent color: {}", accent.name()))
            .on_press(Message::NextAccent),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        option_button::<Message>(icon(lucide::MINUS)).label("Remove"),
        // #endregion
    );

    doc::page(
        "OptionButton",
        "A small square outlined button for secondary options, such as the buttons next to a \
         list.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `option_button` its content, usually an icon, and a press message with \
                 `on_press`. The button is as tall and wide as a pointer button. `label` shows \
                 as a tooltip, and is the only name an icon-only button has, so give it one. \
                 Without `on_press` the button is disabled and skipped by focus traversal.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::controls::{option_button, option_color};",
            ),
            doc::heading("Examples"),
            doc::subheading("Color swatch"),
            doc::prose(
                "`option_color` shows a swatch of a color, outlined with the check radius, and \
                 takes a label that names it. This one cycles through the accent colors.",
            ),
            color,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_press`, or with `on_press_maybe(None)` as the Remove button above \
                 when the list is empty, the button is dimmed.",
            ),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the button."],
                    vec!["`Enter` / `Space`", "Presses the focused button."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::option_button",
                    "oito2_iced::widget::controls::option_color",
                    "oito2_iced::widget::controls::OptionButton",
                ],
            ),
        ],
    )
}
