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

//! The containers page: bordered containers, sections, tiles, info boxes,
//! badges, banners and selectable containers.

use iced::widget::{column, row, text};
use iced::{Color, Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::{self, tile, tile_list};
use oito2_iced::widget::info::{self, InfoType};
use oito2_iced::widget::{button, selectable};

use super::{Gallery, Message, card, page};

/// The containers page.
pub fn view(gallery: &Gallery) -> Element<'_, Message, Theme> {
    let surfaces = row![
        containers::border_container(text("Bordered")).padding(spacing::MD),
        containers::border_container(text("Tinted"))
            .padding(spacing::MD)
            .tint(Color::from_rgb8(0x2E, 0xC2, 0x7E)),
        selectable::selectable_container("Selectable", gallery.selectable)
            .on_toggle(Message::Selectable),
        info::banner(text("Banner"))
            .selected(gallery.banner)
            .on_press(Message::Banner),
    ]
    .spacing(spacing::SM)
    .align_y(iced::Alignment::Center);

    let section = containers::section(tile_list([
        tile()
            .leading(icon(lucide::WIFI))
            .title("Wi-Fi")
            .subtitle("Connected")
            .trailing(text("On"))
            .on_press(Message::Press),
        tile()
            .leading(icon(lucide::BLUETOOTH))
            .title("Bluetooth")
            .subtitle(if gallery.bluetooth { "On" } else { "Off" })
            .on_press(Message::Bluetooth(!gallery.bluetooth)),
        tile()
            .leading(icon(lucide::PLANE))
            .title("Airplane mode")
            .enabled(false),
    ]))
    .headline("Network")
    .action(button::text("Reset").on_press(Message::Press))
    .padding(0);

    let banners = column![
        info::banner_tile("Weather")
            .subtitle("Sunny all week")
            .icon(icon(lucide::SUN).size(48.0))
            .tint(Color::from_rgb8(0xE9, 0x1E, 0x63))
            .width(Length::Fill)
            .on_press(Message::Press),
        containers::border_container(containers::watermark(
            icon(lucide::CLOUD).size(64.0),
            iced::widget::container(text("Content over a watermark"))
                .padding(spacing::LG)
                .width(Length::Fill)
                .height(Length::Fill),
        ))
        .width(Length::Fill)
        .height(120),
    ]
    .spacing(spacing::SM);

    let boxes = column(InfoType::ALL.into_iter().map(|kind| {
        info::info_box(kind)
            .title(format!("{kind:?}"))
            .subtitle("The message text of the box.")
            .into()
    }))
    .spacing(spacing::SM);

    let badges = row(InfoType::ALL
        .into_iter()
        .map(|kind| info::info_badge(kind, format!("{kind:?}"))))
    .spacing(spacing::SM);

    page(
        "Containers",
        "Surfaces that group content: bordered and tinted containers, sections of tiles, banners, info boxes and badges.",
        [
            card("Surfaces", surfaces),
            card(
                "Section and banners",
                row![section, banners].spacing(spacing::LG),
            ),
            card("Info boxes", boxes),
            card("Info badges", badges),
        ],
    )
}
