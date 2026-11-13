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

//! The navigation page: tab bars, navigation rails, a navigation page and
//! a master-detail page.

use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::{self, lucide};
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::master_detail::{detail_page, master_detail, master_tile};
use oito2_iced::widget::navigation_page::navigation_page;
use oito2_iced::widget::navigation_rail::{self, NavigationRailStyle, navigation_rail};
use oito2_iced::widget::tab_bar::{TabBarStyle, tab, tab_bar, vertical_tabs};

use super::{Gallery, Message, card, page};

/// The tabs of the tab bars.
const TABS: [&str; 3] = ["General", "Password", "Billing"];

/// The destinations of the rails and the navigation page.
const DESTINATIONS: [&str; 3] = ["Home", "Starred", "Trash"];

/// The items of the master-detail page.
const DEVICES: [&str; 4] = ["Network", "Bluetooth", "Display", "Sound"];

/// The height of the examples that fill their area.
const AREA_HEIGHT: f32 = 300.0;

/// The destinations with their icons.
fn destinations() -> Vec<navigation_rail::NavigationItem<'static>> {
    [lucide::HOUSE, lucide::STAR, lucide::TRASH]
        .into_iter()
        .zip(DESTINATIONS)
        .map(|(icon, label)| navigation_rail::item(icon, label))
        .collect()
}

/// The navigation page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let tabs = column![
        tab_bar(
            TABS.iter()
                .map(|label| tab(*label))
                .enumerate()
                .map(|(index, tab)| if index == 0 {
                    tab.icon(lucide::SETTINGS)
                } else {
                    tab
                }),
            gallery.tab,
        )
        .on_select(Message::Tab),
        tab_bar(TABS.map(tab), gallery.subtle_tab)
            .style(TabBarStyle::Subtle)
            .on_select(Message::SubtleTab),
        container(
            vertical_tabs(
                TABS.map(tab),
                gallery.vertical_tab,
                container(text(format!("{} settings", TABS[gallery.vertical_tab])))
                    .padding(spacing::MD),
            )
            .on_select(Message::VerticalTab),
        )
        .height(Length::Fixed(160.0)),
    ]
    .spacing(spacing::MD);

    let rails = row![
        navigation_rail(destinations(), Some(gallery.rail))
            .style(NavigationRailStyle::Compact)
            .on_select(Message::Rail),
        navigation_rail(destinations(), Some(gallery.rail)).on_select(Message::Rail),
        navigation_rail(destinations(), Some(gallery.rail))
            .style(NavigationRailStyle::LabelledExtended)
            .on_select(Message::Rail),
    ]
    .spacing(spacing::LG)
    .height(Length::Fixed(240.0));

    let destination = container(text(format!("{} page", DESTINATIONS[gallery.destination])))
        .padding(spacing::MD)
        .width(Length::Fill)
        .height(Length::Fill);
    let navigation = container(
        navigation_page(destinations(), gallery.destination, destination)
            .on_select(Message::Destination),
    )
    .height(Length::Fixed(AREA_HEIGHT));

    let back = *l10n;
    let details = container(
        master_detail(
            DEVICES.len(),
            gallery.device,
            |index, selected, _| {
                master_tile(selected, Message::Device(Some(index)))
                    .leading(icon::icon(lucide::SETTINGS))
                    .title(DEVICES[index])
                    .subtitle("Settings")
                    .into()
            },
            move |index, on_back| {
                detail_page(
                    container(text(format!("{} settings", DEVICES[index]))).padding(spacing::MD),
                )
                .title(DEVICES[index])
                .localizations(&back)
                .back(on_back)
                .into()
            },
        )
        .on_back(Message::Device(None))
        .pane_size(gallery.device_pane)
        .min_page_size(240.0)
        .on_resize(Message::DevicePane),
    )
    .height(Length::Fixed(AREA_HEIGHT));

    page(
        "Navigation",
        "Each bar or rail is one Tab stop; the arrows, Home and End move inside it. Narrow windows show the compact rail and one master-detail side.",
        [
            card("Tab bars (pill, subtle, vertical)", tabs),
            card("Navigation rails (compact, labelled, extended)", rails),
            card("Navigation page", navigation),
            card("Master-detail", details),
        ],
    )
}
