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

//! The progress page: linear and circular indicators.

use iced::widget::{column, row, text};
use iced::{Alignment, Element};

use oito2_iced::Theme;
use oito2_iced::theme::progress_bar::Class;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::{progress, slider};

use super::{Gallery, Message, card, page};

/// The progress page.
pub fn view(gallery: &Gallery) -> Element<'_, Message, Theme> {
    let value = gallery.volume / 100.0;

    let determinate = column![
        slider::slider(gallery.volume)
            .range(0.0, 100.0)
            .label("Progress")
            .value_text(|value| format!("{value:.0}%"))
            .on_change(Message::Volume),
        progress::linear(value),
        progress::linear(value).class(Class::Success),
        progress::linear(value).class(Class::Warning),
        progress::linear(value).class(Class::Error),
        row![
            progress::circular(value),
            progress::circular(value).class(Class::Success),
            progress::circular(value).size(48.0).stroke_width(6.0),
        ]
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
    ]
    .spacing(spacing::MD)
    .width(360);

    let indeterminate = row![
        progress::linear_indeterminate(),
        progress::circular_indeterminate(),
        text("Working\u{2026}"),
    ]
    .spacing(spacing::LG)
    .align_y(Alignment::Center);

    page(
        "Progress",
        "Determinate indicators show a value from 0 to 1; indeterminate ones move while the work has no known length.",
        [
            card("Determinate", determinate),
            card("Indeterminate", indeterminate),
        ],
    )
}
