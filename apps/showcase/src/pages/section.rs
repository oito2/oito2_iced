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

//! The Section page: a headline with actions above a bordered container.

use iced::widget::{container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::containers;
use oito2_iced::widget::info::InfoType;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("section.rs");

/// The width of the examples.
const WIDTH: f32 = 420.0;

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// An action of an example was pressed.
    Pressed(&'static str),
}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pressed(action) => shell.toast(action, InfoType::Information),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            containers::section(text(
                "A section groups related content under a headline, with optional actions \
                 beside it.",
            ))
            .headline("Storage")
            .action(button::text("Clean up").on_press(Message::Pressed("Clean up"))),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let plain = doc::example(
        ctx,
        SOURCE,
        &["plain"],
        container(
            // #region plain
            containers::section(text("Only the content, in its bordered container.")),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let padding = doc::example(
        ctx,
        SOURCE,
        &["padding"],
        container(
            // #region padding
            containers::section(
                containers::tile()
                    .leading(icon(lucide::USER))
                    .title("Profile")
                    .on_press(Message::Pressed("Profile")),
            )
            .headline("Account")
            .padding(0)
            .margin(spacing::SM),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let actions = doc::example(
        ctx,
        SOURCE,
        &["actions"],
        container(
            // #region actions
            containers::section(text("Three backups, the last one today."))
                .headline("Backups")
                .action(button::text("Restore").on_press(Message::Pressed("Restore")))
                .action(button::tonal("Back up now").on_press(Message::Pressed("Back up now"))),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "Section",
        "Groups related content under a headline, with optional actions beside it, in a \
         bordered container.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `section` its content, an optional `headline` and optional actions, \
                 added one by one with `action`. The headline takes the medium title style \
                 and the actions sit at the end of its row. The content goes inside a \
                 `border_container` with `padding` around it, 16 pixels by default. The \
                 section fills the available width.",
            ),
            doc::code(ctx, "use oito2_iced::widget::containers;"),
            doc::heading("Examples"),
            doc::subheading("Without a headline"),
            doc::prose(
                "Without a headline and actions the headline row is left out, and only the \
                 bordered container shows.",
            ),
            plain,
            doc::subheading("Several actions"),
            doc::prose("Each `action` call adds one more element to the end of the headline row."),
            actions,
            doc::subheading("Padding and margin"),
            doc::prose(
                "`padding` sets the space inside the container, and `margin` the space around \
                 the whole section, headline included. With no padding, a tile fills the \
                 container.",
            ),
            padding,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::containers::section",
                    "oito2_iced::widget::containers::Section",
                ],
            ),
        ],
    )
}
