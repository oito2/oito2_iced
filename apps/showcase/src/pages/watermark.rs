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

//! The Watermark page: a faint decoration behind an empty state.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::typography;
use oito2_iced::widget::containers::{border_container, watermark};
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("watermark.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {}
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        border_container(
            // #region preview
            watermark(
                icon(lucide::INBOX).size(120.0),
                container(styled_text("No messages", typography::TITLE_MEDIUM))
                    .center(Length::Fill),
            ),
            // #endregion
        )
        .width(Length::Fixed(420.0))
        .height(Length::Fixed(200.0)),
    );

    let alignment = doc::example(
        ctx,
        SOURCE,
        &["alignment"],
        border_container(
            // #region alignment
            watermark(
                icon(lucide::CLOUD_OFF).size(96.0),
                container(styled_text("You are offline", typography::TITLE_MEDIUM))
                    .center(Length::Fill),
            )
            .align(Horizontal::Center, Vertical::Center)
            .padding(0)
            .opacity(0.2),
            // #endregion
        )
        .width(Length::Fixed(420.0))
        .height(Length::Fixed(200.0)),
    );

    doc::page(
        "Watermark",
        "Places a faint decoration, such as a large icon, behind content like an empty state.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `watermark` the decorative mark and the content drawn over it. The mark \
                 covers the area the content takes, so to watermark a whole area give the \
                 content the full size, for example with `container(...).center(Length::Fill)`.",
            ),
            doc::code(ctx, "use oito2_iced::widget::containers::watermark;"),
            doc::prose(
                "By default the mark sits at the center of the end side, 20 pixels from the \
                 edges. Text and icons in the mark are drawn in the theme content color at \
                 10 % opacity. The mark is drawn below the content, which receives the pointer \
                 first, so it never blocks it.",
            ),
            doc::heading("Examples"),
            doc::subheading("Alignment and opacity"),
            doc::prose(
                "`align` places the mark, `padding` keeps it away from the edges and \
                 `opacity`, from 0 to 1, sets how faint it is.",
            ),
            alignment,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::containers::watermark",
                    "oito2_iced::widget::containers::Watermark",
                ],
            ),
        ],
    )
}
