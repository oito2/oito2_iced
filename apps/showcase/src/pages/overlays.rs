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

//! The overlays page: menus, selects, context menus, split buttons,
//! popovers, hover cards and autocomplete, and the buttons that open the
//! sheets, the dialog, toasts and the command palette of the window.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::autocomplete::autocomplete;
use oito2_iced::widget::button;
use oito2_iced::widget::context_menu::context_menu;
use oito2_iced::widget::hover_card::hover_card;
use oito2_iced::widget::menu::{self, popup_menu_button, select};
use oito2_iced::widget::popover::popover;
use oito2_iced::widget::split_button::{Variant, split_button};
use oito2_iced::widget::styled_text;

use super::{Gallery, Message, card, page};

/// The sort orders of the popup menu.
pub const SORTS: [&str; 3] = ["Name", "Size", "Date modified"];

/// The overlays page.
pub fn view(gallery: &Gallery) -> Element<'_, Message, Theme> {
    let sort = popup_menu_button(
        gallery.sort,
        [menu::header("Sort by")]
            .into_iter()
            .chain(SORTS.map(|sort| {
                menu::checked_item(gallery.sort == sort, sort).on_press(Message::Sort(sort))
            }))
            .chain([
                menu::separator(),
                menu::multi_select_item(gallery.hidden_files, "Hidden files")
                    .on_change(Message::HiddenFiles),
                menu::item("Reset").on_press(Message::Sort(SORTS[0])),
            ]),
    )
    .open(gallery.sort_open)
    .on_open_change(Message::SortOpen);

    let fruit = gallery.fruits.as_ref().map(|fruits| {
        select(fruits, gallery.fruit.as_deref())
            .placeholder("Pick a fruit")
            .open(gallery.fruit_open)
            .on_open_change(Message::FruitOpen)
            .on_change(Message::Fruit)
    });

    let save = split_button(
        "Save",
        [
            menu::item("Save as").on_press(Message::Press),
            menu::item("Save all").on_press(Message::Press),
        ],
    )
    .variant(Variant::Filled)
    .on_press(Message::Press)
    .open(gallery.split_open)
    .on_open_change(Message::SplitOpen);

    let menus = row![sort]
        .push(fruit)
        .push(save)
        .spacing(spacing::MD)
        .align_y(Alignment::Center)
        .wrap();

    let region = context_menu(
        container(text(
            "Right click here, or focus a control inside and press Shift+F10.",
        ))
        .padding(spacing::LG)
        .width(Length::Fill)
        .class(ContainerClass::Popover),
        [
            menu::item("Cut").on_press(Message::Press),
            menu::item("Copy").on_press(Message::Press),
            menu::item("Paste"),
            menu::separator(),
            menu::item("Select all").on_press(Message::Press),
        ],
    )
    .open(gallery.context_open)
    .on_open_change(Message::ContextOpen);

    let anchored = row![
        popover(
            button::tonal("Popover").on_press(Message::PopoverOpen(!gallery.popover_open)),
            column![
                styled_text("Anchored panel", typography::TITLE_SMALL),
                text("Escape or a click outside closes it.").class(TextClass::Variant),
                button::text("Close").on_press(Message::PopoverOpen(false)),
            ]
            .spacing(spacing::XS),
        )
        .open(gallery.popover_open)
        .arrow(true)
        .on_dismiss(Message::PopoverOpen(false)),
        hover_card(
            button::text("Ada Lovelace").on_press(Message::Press),
            column![
                styled_text("Ada Lovelace", typography::TITLE_SMALL),
                text("Mathematician and writer, London.").class(TextClass::Variant),
            ]
            .spacing(spacing::XS),
        ),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center);

    let city = gallery.cities.as_ref().map(|cities| {
        autocomplete("Type a city", &gallery.city, cities)
            .label("City")
            .on_input(Message::City)
            .on_select(Message::City)
            .width(Length::Fixed(320.0))
    });

    let window = row![
        button::outlined("End sheet").on_press(Message::OpenSheet(false)),
        button::outlined("Bottom sheet").on_press(Message::OpenSheet(true)),
        button::outlined("Dialog").on_press(Message::OpenDialog),
        button::outlined("Toast").on_press(Message::ShowToast),
        button::outlined("Command palette (Ctrl+K)").on_press(Message::OpenPalette),
    ]
    .spacing(spacing::SM)
    .wrap();

    page(
        "Overlays",
        "Menus move with the arrows and close with Escape; modal surfaces keep the focus inside until they close.",
        [
            card("Popup menu, select and split button", menus),
            card("Context menu", region),
            card("Popover and hover card", anchored),
            card("Autocomplete", column![].push(city)),
            card("Sheets, dialog, toasts and command palette", window),
        ],
    )
}
