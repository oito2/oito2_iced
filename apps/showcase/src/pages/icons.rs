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

//! The icons page.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::{self, Freedesktop, lucide};
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;

use super::{Message, card, page};

/// The icons page.
pub fn view<'a>() -> Element<'a, Message, Theme> {
    let freedesktop = row(Freedesktop::ALL.iter().map(|named| {
        column![
            icon::icon(named.icon()),
            text(named.name()).size(11).class(TextClass::Variant),
        ]
        .spacing(spacing::XS)
        .align_x(Alignment::Center)
        .width(Length::Fixed(132.0))
        .into()
    }))
    .spacing(spacing::SM)
    .wrap();

    let sizes = row![
        icon::icon(lucide::STAR).size(16.0),
        icon::icon(lucide::STAR),
        icon::icon(lucide::STAR).size(24.0),
        icon::icon(lucide::STAR).size(32.0),
        icon::icon(lucide::STAR).size(48.0),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center);

    let placeholders = row![
        icon::placeholder().size(32.0),
        icon::placeholder(),
        icon::placeholder().size(72.0),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center);

    page(
        "Icons",
        "Every Lucide icon is a constant of oito2_iced::icon::lucide and takes the text color of its parent; freedesktop.org names map to Lucide icons.",
        [
            card("Sizes", sizes),
            card("Placeholder", placeholders),
            card("Freedesktop names", freedesktop),
        ],
    )
}
