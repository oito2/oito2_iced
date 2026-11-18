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

//! The HoverCard page: a profile preview, a placement without an arrow
//! and custom delays.

use std::time::Duration;

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::hover_card::hover_card;
use oito2_iced::widget::popover::Placement;
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("hover_card.rs");

/// The people of the profile example, with their roles.
const PEOPLE: [(&str, &str); 2] = [("Ana Souza", "Maintainer"), ("Bruno Lima", "Designer")];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    opened: Option<&'static str>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// An anchor button was pressed.
    Open(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Open(name) => state.opened = Some(name),
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
            hover_card(
                button::text("oito2_iced").on_press(Message::Open("oito2_iced")),
                text("github.com/oito2/oito2_iced"),
            ),
            // #endregion
            doc::caption(state.opened.map_or_else(
                || "Rest the pointer on the button, or focus it.".to_owned(),
                |name| format!("Opened: {name}"),
            )),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let profiles = doc::example(
        ctx,
        SOURCE,
        &["profile"],
        row(PEOPLE.iter().map(|(name, role)| {
            // #region profile
            hover_card(
                button::text(*name).on_press(Message::Open(name)),
                row![
                    icon::placeholder().size(48.0).radius(24.0),
                    column![styled_text(*name, typography::TITLE_MEDIUM), text(*role)],
                ]
                .spacing(spacing::MD)
                .align_y(Alignment::Center),
            )
            // #endregion
            .into()
        }))
        .spacing(spacing::SM),
    );

    let placement = doc::example(
        ctx,
        SOURCE,
        &["placement"],
        // #region placement
        hover_card(
            button::text("Repository").on_press(Message::Open("Repository")),
            text("github.com/oito2/oito2_iced"),
        )
        .placement(Placement::Top)
        .arrow(false),
        // #endregion
    );

    let delays = doc::example(
        ctx,
        SOURCE,
        &["delays"],
        // #region delays
        hover_card(
            button::text("Quick card").on_press(Message::Open("Quick card")),
            text("Opens at once, closes after a second."),
        )
        .open_delay(Duration::ZERO)
        .close_delay(Duration::from_secs(1)),
        // #endregion
    );

    doc::page(
        "HoverCard",
        "A card of extra detail that opens while the pointer rests on an element or the focus \
         is inside it, such as a profile preview on a user name.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `hover_card` the anchor and the card content. The card opens \
                 `open_delay` after the pointer enters the anchor or the focus moves inside \
                 it, 500 ms by default, and closes `close_delay` after the pointer leaves \
                 both the anchor and the card and the focus leaves the anchor, 300 ms by \
                 default. `Escape` and a mouse press outside close it at once.",
            ),
            doc::code(ctx, "use oito2_iced::widget::hover_card::hover_card;"),
            doc::prose(
                "The card keeps its open state itself, on the popover surface; the delays run \
                 on the redraw requests of the window, which the card asks for only while a \
                 change is pending.",
            ),
            doc::note(
                "The card never takes the focus, so its content must only repeat or preview \
                 what the anchor leads to. For interactive content, use `popover`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Profile preview"),
            doc::prose(
                "The card content can be any element, up to `max_width` wide, 320 by default.",
            ),
            profiles,
            doc::subheading("Above, without an arrow"),
            doc::prose(
                "`placement` and `alignment` position the card as in `popover`, below and \
                 centered by default; `arrow(false)` removes the arrow. `direction` maps the \
                 start and end placements to the sides of the reading direction.",
            ),
            placement,
            doc::subheading("Delays"),
            doc::prose(
                "Shorter delays make the card feel quicker; longer ones avoid opening it while \
                 the pointer only passes over the anchor.",
            ),
            delays,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab`",
                        "Focusing the anchor opens the card after the open delay.",
                    ],
                    vec!["`Escape`", "Closes the open card, wherever the focus is."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::hover_card::hover_card",
                    "oito2_iced::widget::hover_card::HoverCard",
                    "oito2_iced::widget::hover_card::OPEN_DELAY",
                    "oito2_iced::widget::hover_card::CLOSE_DELAY",
                    "oito2_iced::widget::hover_card::MAX_WIDTH",
                ],
            ),
        ],
    )
}
