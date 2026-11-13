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

//! The buttons page.

use iced::widget::{row, text};
use iced::{Alignment, Element};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;

use super::{Gallery, Message, card, page};

/// The buttons page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let variants = row![
        button::filled("Filled").on_press(Message::Press),
        button::tonal("Tonal").on_press(Message::Press),
        button::elevated("Elevated").on_press(Message::Press),
        button::outlined("Outlined").on_press(Message::Press),
        button::text("Text").on_press(Message::Press),
        button::danger("Danger").on_press(Message::Press),
    ]
    .spacing(spacing::SM)
    .wrap();

    let with_icons = row![
        button::filled("Download")
            .icon(lucide::DOWNLOAD)
            .on_press(Message::Press),
        button::tonal("Share")
            .icon(lucide::SHARE_2)
            .on_press(Message::Press),
        button::outlined("Upload")
            .icon(lucide::UPLOAD)
            .on_press(Message::Press),
    ]
    .spacing(spacing::SM)
    .wrap();

    let disabled = row![
        button::filled("Filled"),
        button::tonal("Tonal"),
        button::outlined("Outlined"),
        button::text("Text"),
    ]
    .spacing(spacing::SM)
    .wrap();

    let icon_buttons = row![
        button::icon_button(lucide::SETTINGS)
            .tooltip("Settings")
            .on_press(Message::Press),
        button::icon_button(lucide::HEART)
            .selected(gallery.favorite)
            .tooltip("Favorite")
            .on_press(Message::ToggleFavorite),
        button::back_button(l10n, ReadingDirection::LeftToRight).on_press(Message::Press),
        button::close_button(l10n).on_press(Message::Press),
        button::icon_button(lucide::TRASH),
    ]
    .spacing(spacing::SM)
    .align_y(Alignment::Center);

    let fabs = row![
        button::fab(lucide::PENCIL)
            .tooltip("Compose")
            .on_press(Message::Press),
        button::fab(lucide::PLUS)
            .label("New item")
            .on_press(Message::Press),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center);

    page(
        "Buttons",
        "Buttons are at least as tall as the control height of the density and grow with their label. Tab moves the focus; Enter or Space press the focused button.",
        [
            card(
                "Variants",
                iced::widget::column![
                    variants,
                    text(format!("Presses: {}", gallery.presses)).class(TextClass::Variant)
                ]
                .spacing(spacing::SM),
            ),
            card("With icons", with_icons),
            card("Disabled", disabled),
            card("Icon buttons", icon_buttons),
            card("Floating action buttons", fabs),
        ],
    )
}
