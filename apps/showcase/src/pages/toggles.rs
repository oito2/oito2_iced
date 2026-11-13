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

//! The toggles page: check boxes, radios, switches, toggle buttons and
//! list tiles.

use iced::widget::{column, row};
use iced::{Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::{controls, list_tile, toggles};

use super::{Gallery, Message, card, page};

/// The toggles page.
pub fn view(gallery: &Gallery) -> Element<'_, Message, Theme> {
    let controls_row = row![
        toggles::checkbox(gallery.check)
            .tristate(true)
            .on_change(Message::Check),
        toggles::radio(1, Some(gallery.size)).on_change(Message::Size),
        toggles::radio(2, Some(gallery.size)).on_change(Message::Size),
        toggles::switch(gallery.dark).on_toggle(Message::Dark),
        toggles::switch(gallery.dark)
            .on_off_shapes(true)
            .on_toggle(Message::Dark),
        toggles::switch(gallery.dark)
            .direction(ReadingDirection::RightToLeft)
            .on_toggle(Message::Dark),
        toggles::checkbox::<Message>(true),
        toggles::switch::<Message>(true),
    ]
    .spacing(spacing::SM)
    .align_y(iced::Alignment::Center);

    let buttons = column![
        controls::check_button(
            toggles::checkbox(gallery.check)
                .tristate(true)
                .on_change(Message::Check),
            "Show hidden files"
        )
        .subtitle("Files starting with a dot"),
        controls::radio_button(
            toggles::radio(1, Some(gallery.size)).on_change(Message::Size),
            "Small"
        ),
        controls::radio_button(
            toggles::radio(2, Some(gallery.size)).on_change(Message::Size),
            "Large"
        ),
        controls::switch_button(
            toggles::switch(gallery.dark).on_toggle(Message::Dark),
            "Dark style"
        ),
        row![
            controls::option_button(icon(lucide::PLUS))
                .label("Add")
                .on_press(Message::Press),
            controls::option_button(icon(lucide::MINUS)).label("Remove"),
            controls::option_color(iced::Color::from_rgb8(0x35, 0x84, 0xE4), "Blue")
                .on_press(Message::Press),
        ]
        .spacing(spacing::SM),
    ]
    .spacing(spacing::XS);

    let tiles = column![
        list_tile::list_tile()
            .title("Network")
            .subtitle("Connected")
            .leading(icon(lucide::WIFI))
            .trailing(icon(lucide::CHEVRON_RIGHT))
            .on_press(Message::Press),
        list_tile::list_tile()
            .title("Documents")
            .leading(icon(lucide::FOLDER))
            .selected(true)
            .on_press(Message::Press),
        list_tile::checkbox_list_tile(
            toggles::checkbox(gallery.check)
                .tristate(true)
                .on_change(Message::Check)
        )
        .title("Hidden files")
        .secondary(icon(lucide::EYE_OFF)),
        list_tile::switch_list_tile(
            toggles::switch(gallery.bluetooth).on_toggle(Message::Bluetooth)
        )
        .title("Bluetooth")
        .subtitle(if gallery.bluetooth { "On" } else { "Off" }),
    ]
    .width(Length::Fixed(360.0));

    page(
        "Toggles",
        "Check boxes, radios and switches take the focus with Tab and toggle with Space or Enter; toggle buttons and list tiles toggle from anywhere on the row.",
        [
            card("Check boxes, radios and switches", controls_row),
            card("Toggle buttons and option buttons", buttons),
            card("List tiles", tiles),
        ],
    )
}
