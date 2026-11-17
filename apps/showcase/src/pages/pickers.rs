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

//! The pickers page: rating, range and angle sliders, PIN and tags
//! entries, date and time entries, calendars and color pickers.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Color, Element, Length};

use oito2_iced::Theme;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::angle_slider::angle_slider;
use oito2_iced::widget::calendar::{calendar, date_picker};
use oito2_iced::widget::color_picker::{color_picker, color_picker_button};
use oito2_iced::widget::controls::color_disk;
use oito2_iced::widget::entries::{
    date_entry, date_time_entry, segment, segmented_entry, time_entry,
};
use oito2_iced::widget::pin_entry::pin_entry;
use oito2_iced::widget::rating::rating;
use oito2_iced::widget::slider::range_slider;
use oito2_iced::widget::tags_field::tags_field;

use super::{Gallery, Message, card, page};

/// The colors of the color disks, with their names.
pub const DISKS: [(&str, Color); 5] = [
    ("Blue", Color::from_rgb8(0x35, 0x84, 0xE4)),
    ("Green", Color::from_rgb8(0x2E, 0xC2, 0x7E)),
    ("Yellow", Color::from_rgb8(0xF6, 0xD3, 0x2D)),
    ("Orange", Color::from_rgb8(0xFF, 0x78, 0x00)),
    ("Red", Color::from_rgb8(0xE0, 0x1B, 0x24)),
];

/// The pickers page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let today = gallery.today;

    let ratings = column![
        row![
            rating(gallery.rating)
                .allow_half(true)
                .on_change(Message::Rating),
            text(oito2_iced::widget::rating::value_text(
                gallery.rating,
                5,
                l10n
            ))
            .class(TextClass::Variant),
        ]
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
        row![
            column![
                range_slider(gallery.price_range.0, gallery.price_range.1)
                    .range(0.0, 100.0)
                    .divisions(20)
                    .label("Price")
                    .value_text(|value| format!("{value:.0}"))
                    .on_change(Message::PriceRange),
            ]
            .width(Length::Fixed(320.0)),
            angle_slider(gallery.angle)
                .localizations(l10n)
                .on_change(Message::Angle),
        ]
        .spacing(spacing::XL)
        .align_y(Alignment::Center)
        .wrap(),
    ]
    .spacing(spacing::LG);

    let codes = column![
        pin_entry(&gallery.pin)
            .localizations(l10n)
            .on_change(Message::Pin),
        tags_field(&gallery.tags)
            .label("Tags")
            .placeholder("Add tag")
            .clearable(true)
            .localizations(l10n)
            .on_change(Message::Tags)
            .width(Length::Fixed(420.0)),
        segmented_entry(
            [segment(3), segment(3), segment(3), segment(3)],
            &gallery.address
        )
        .on_change(Message::Address),
    ]
    .spacing(spacing::LG);

    let dates = column![
        row![
            date_entry(today, gallery.date)
                .open(gallery.date_open)
                .on_open_change(Message::DateOpen)
                .localizations(l10n)
                .on_change(Message::Date)
                .width(Length::Fixed(240.0)),
            time_entry(gallery.time)
                .open(gallery.time_open)
                .on_open_change(Message::TimeOpen)
                .use_24_hour(l10n.locale_name != "en")
                .localizations(l10n)
                .on_change(Message::Time)
                .width(Length::Fixed(200.0)),
        ]
        .spacing(spacing::MD)
        .wrap(),
        container(
            date_time_entry(today, gallery.date_time)
                .open(gallery.date_time_open)
                .on_open_change(Message::DateTimeOpen)
                .localizations(l10n)
                .on_change(Message::DateTime)
                .width(Length::Fill),
        )
        .max_width(456),
        date_picker(today)
            .label("Trip")
            .range(gallery.trip)
            .on_range_change(Message::Trip)
            .open(gallery.trip_open)
            .on_open_change(Message::TripOpen)
            .localizations(l10n)
            .width(Length::Fixed(320.0)),
    ]
    .spacing(spacing::MD);

    let month = calendar(today)
        .selected(gallery.day)
        .on_select(Message::Day)
        .show_week_numbers(true)
        .localizations(l10n);

    let disks = row(DISKS.iter().enumerate().map(|(index, (label, color))| {
        color_disk(*color, *label)
            .selected(gallery.disk == index)
            .on_press(Message::Disk(index))
            .into()
    }))
    .spacing(spacing::SM);

    let colors = row![
        color_picker(gallery.color)
            .show_alpha(true)
            .swatches(DISKS.iter().map(|(_, color)| *color))
            .localizations(l10n)
            .on_change(Message::Color),
        column![
            disks,
            color_picker_button(gallery.color)
                .open(gallery.color_open)
                .on_open_change(Message::ColorOpen)
                .localizations(l10n)
                .on_change(Message::Color),
        ]
        .spacing(spacing::LG),
    ]
    .spacing(spacing::XL)
    .wrap()
    .vertical_spacing(spacing::LG);

    page(
        "Pickers",
        "Values picked with the pointer or the keyboard; dates and times follow the language of the app.",
        [
            card("Rating, range and angle", ratings),
            card("Codes, tags and segments", codes),
            card("Dates and times", dates),
            card("Calendar", month),
            card("Colors", colors),
        ],
    )
}
