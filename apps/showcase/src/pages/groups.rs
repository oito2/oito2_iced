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

//! The groups page: segment groups, toggle groups, choice chips and sliders.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::{chips, segment_group, slider, toggle_group};

use super::{CHIPS, Gallery, Message, card, page};

/// The groups page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let segments = segment_group::segment_group(
        [
            segment_group::segment("list", "List").icon(lucide::LIST),
            segment_group::segment("grid", "Grid").icon(lucide::LAYOUT_GRID),
            segment_group::segment("columns", "Columns").icon(lucide::COLUMNS_3),
        ],
        Some(gallery.layout),
    )
    .on_change(Message::Layout);

    let toggles = toggle_group::toggle_group(
        [
            toggle_group::icon_item("b", lucide::LETTER_B).tooltip("Bold"),
            toggle_group::icon_item("i", lucide::ITALIC).tooltip("Italic"),
            toggle_group::icon_item("u", lucide::UNDERLINE).tooltip("Underline"),
        ],
        &gallery.styles,
    )
    .multiple(true)
    .on_change(Message::Styles);

    let chip_row = container(
        chips::choice_chip_bar(CHIPS, &gallery.chips)
            .localizations(l10n)
            .on_select(Message::Chip),
    )
    .width(Length::Fixed(360.0));

    let chip_wrap = chips::choice_chip_bar(CHIPS, &gallery.chips)
        .pill(true)
        .style(chips::BarStyle::Wrap)
        .on_select(Message::Chip);

    let sliders = row![
        column![
            slider::slider(gallery.volume)
                .range(0.0, 100.0)
                .label("Volume")
                .value_text(|value| format!("{value:.0}%"))
                .marks([
                    slider::mark(0.0).label("0%"),
                    slider::mark(50.0).label("50%"),
                    slider::mark(100.0).label("100%"),
                ])
                .on_change(Message::Volume),
            slider::slider(gallery.volume)
                .range(0.0, 100.0)
                .divisions(10)
                .show_value_indicator(true)
                .on_change(Message::Volume),
            slider::slider(0.4).label("Disabled"),
        ]
        .spacing(spacing::LG)
        .width(Length::Fixed(360.0)),
        slider::slider(gallery.volume)
            .range(0.0, 100.0)
            .vertical(true)
            .on_change(Message::Volume),
    ]
    .spacing(spacing::XL);

    page(
        "Groups",
        "A group is one Tab stop: the arrow keys move inside it, and Space or Enter select.",
        [
            card(
                "Segment and toggle groups",
                row![segments, toggles]
                    .spacing(spacing::LG)
                    .align_y(Alignment::Center),
            ),
            card(
                "Choice chips",
                column![
                    chip_row,
                    chip_wrap,
                    text(format!(
                        "Selected: {}",
                        CHIPS
                            .iter()
                            .zip(&gallery.chips)
                            .filter(|(_, selected)| **selected)
                            .map(|(label, _)| *label)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ))
                    .class(TextClass::Variant),
                ]
                .spacing(spacing::MD),
            ),
            card("Sliders", sliders),
        ],
    )
}
