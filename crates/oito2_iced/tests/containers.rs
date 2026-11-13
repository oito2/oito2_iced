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

//! Containers, tiles, selectable containers, banners and info boxes:
//! clicks, keyboard activation, focus traversal, hover reports and golden
//! snapshots of the catalogs. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row, text};
use iced::{Color, Event, Length, mouse};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::containers::{self, tile, tile_list};
use oito2_iced::widget::info::{self, InfoType};
use oito2_iced::widget::{button, selectable};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Wifi,
    Bluetooth,
    Dark(bool),
    Light(bool),
    Open,
    Hover(bool),
    Go,
}

type Ui<'a> = simulator::Simulator<'a, Message, Theme>;

fn focused(ui: &mut Ui<'_>) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn bounds(ui: &mut Ui<'_>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn settings_list() -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            tile_list([
                tile()
                    .title("Wi-Fi")
                    .subtitle("Connected")
                    .on_press(Message::Wifi)
                    .id("wifi"),
                tile()
                    .title("Bluetooth")
                    .enabled(false)
                    .on_press(Message::Bluetooth)
                    .id("bluetooth"),
                tile().title("Static").id("static"),
            ]),
            selectable::selectable_container("Dark", true)
                .on_toggle(Message::Dark)
                .id("dark"),
            selectable::selectable_container("Light", false)
                .on_toggle(Message::Light)
                .id("light"),
            selectable::selectable_container("Inert", false).id("inert"),
            info::banner_tile("Open")
                .subtitle("A banner")
                .on_press(Message::Open)
                .id("banner"),
        ]
        .spacing(8)
        .width(320),
    )
    .into()
}

#[test]
fn clicks_reach_enabled_tiles_and_toggle_selectable_containers() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), settings_list());

    ui.click("Wi-Fi").expect("wifi tile");
    ui.click("Bluetooth").expect("bluetooth tile");
    ui.click("Static").expect("static tile");
    ui.click("Dark").expect("dark container");
    ui.click("Light").expect("light container");
    ui.click("Inert").expect("inert container");
    ui.click("Open").expect("banner");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Wifi,
            Message::Dark(false),
            Message::Light(true),
            Message::Open
        ]
    );
}

#[test]
fn tab_skips_static_and_disabled_parts_and_keys_activate() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), settings_list());
    let wifi = bounds(&mut ui, "wifi");
    let dark = bounds(&mut ui, "dark");
    let light = bounds(&mut ui, "light");
    let banner = bounds(&mut ui, "banner");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(wifi));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(dark));
    ui.tap_key(Named::Space);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(light));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(banner));
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Wifi,
            Message::Dark(false),
            Message::Light(true),
            Message::Open
        ]
    );
}

#[test]
fn tiles_fill_the_width_of_their_list() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), settings_list());
    let wifi = bounds(&mut ui, "wifi");

    // The list border takes one pixel on each side.
    assert_eq!(wifi.width, 318.0);
    // Two lines of text between 12-pixel paddings.
    assert!(wifi.height >= 24.0 + 20.0 + 24.0, "{wifi:?}");
}

#[test]
fn a_banner_reports_the_pointer_entering_and_leaving() {
    let view: Element<'static, Message> = column![
        info::banner_tile("Weather")
            .subtitle("Sunny all week")
            .on_hover(Message::Hover)
            .width(Length::Fixed(320.0)),
        text("Outside"),
    ]
    .into();

    let mut ui = simulator::Simulator::with_settings(support::settings(), view);
    let weather = ui.find("Weather").expect("title").bounds().center();
    let outside = ui.find("Outside").expect("text").bounds().center();

    for position in [weather, outside] {
        ui.point_at(position);
        let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position })]);
    }

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Hover(true), Message::Hover(false)]);
}

#[test]
fn a_watermark_lets_the_content_take_the_pointer() {
    let view: Element<'static, Message> = containers::watermark(
        icon(lucide::CLOUD).size(96.0),
        iced::widget::center(button::filled("Go").on_press(Message::Go)),
    )
    .into();

    let mut ui = simulator::Simulator::with_size(support::settings(), (400.0, 200.0), view);
    ui.click("Go").expect("button");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Go]);
}

#[test]
fn tile_icons_follow_the_density() {
    for (density, height) in [(Density::Pointer, 48.0), (Density::Touch, 48.0)] {
        density::set(density);

        let view: Element<'static, Message> = column![
            tile()
                .leading(icon(lucide::WIFI))
                .title("Wi-Fi")
                .on_press(Message::Wifi)
                .id("wifi"),
        ]
        .width(320)
        .into();

        let mut ui = simulator::Simulator::with_settings(support::settings(), view);
        let wifi = bounds(&mut ui, "wifi");

        // A 24-pixel title line is at least as tall as the icon of either
        // density, so the tile keeps its height.
        assert_eq!(wifi.height, height, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn containers_catalog() -> Element<'static, Message> {
    let green = Color::from_rgb8(0x2E, 0xC2, 0x7E);

    column![
        row![
            containers::border_container(text("Bordered")).padding(12),
            containers::border_container(text("Tinted"))
                .padding(12)
                .tint(green),
            selectable::selectable_container("Selected", true).on_toggle(Message::Dark),
            selectable::selectable_container("Unselected", false).on_toggle(Message::Light),
            info::banner(text("Banner")).on_press(Message::Open),
            info::banner(text("Selected banner"))
                .selected(true)
                .on_press(Message::Open),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
        row![
            containers::section(tile_list([
                tile()
                    .leading(icon(lucide::WIFI))
                    .title("Wi-Fi")
                    .subtitle("Connected")
                    .trailing(text("On"))
                    .on_press(Message::Wifi),
                tile()
                    .leading(icon(lucide::BLUETOOTH))
                    .title("Bluetooth")
                    .subtitle("Off")
                    .enabled(false),
                tile().title("Airplane mode"),
            ]))
            .headline("Network")
            .action(button::text("Reset").on_press(Message::Go))
            .padding(0),
            column![
                info::banner_tile("Weather")
                    .subtitle("Sunny all week")
                    .icon(icon(lucide::SUN).size(48.0))
                    .tint(Color::from_rgb8(0xE9, 0x1E, 0x63))
                    .width(Length::Fill)
                    .on_press(Message::Open),
                containers::border_container(containers::watermark(
                    icon(lucide::CLOUD).size(64.0),
                    iced::widget::container(text("Content"))
                        .padding(16)
                        .width(Length::Fill)
                        .height(Length::Fill),
                ))
                .width(Length::Fill)
                .height(120),
            ]
            .spacing(8)
            .width(Length::Fill),
        ]
        .spacing(12),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn info_catalog() -> Element<'static, Message> {
    let boxes = InfoType::ALL.into_iter().map(|kind| {
        info::info_box(kind)
            .title(format!("{kind:?}"))
            .subtitle("The message text of the box.")
            .into()
    });
    let badges = InfoType::ALL
        .into_iter()
        .map(|kind| info::info_badge(kind, format!("{kind:?}")));

    column![
        column(boxes).spacing(8),
        row(badges).spacing(8),
        info::info_box(InfoType::Information)
            .title("With a trailing button")
            .trailing(button::text("Dismiss").on_press(Message::Go)),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(view: Element<'static, Message>, size: (f32, f32), theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), size, view);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn high_contrast() -> Theme {
    Theme::base(Brightness::Light, Accent::default().color(), Contrast::High)
}

#[test]
fn containers_light() {
    snapshot(
        containers_catalog(),
        (720.0, 340.0),
        &Theme::light(),
        "containers_light",
    );
}

#[test]
fn containers_dark() {
    snapshot(
        containers_catalog(),
        (720.0, 340.0),
        &Theme::dark(),
        "containers_dark",
    );
}

#[test]
fn containers_light_high_contrast() {
    snapshot(
        containers_catalog(),
        (720.0, 340.0),
        &high_contrast(),
        "containers_light_high_contrast",
    );
}

#[test]
fn info_light() {
    snapshot(
        info_catalog(),
        (480.0, 520.0),
        &Theme::light(),
        "info_light",
    );
}

#[test]
fn info_dark() {
    snapshot(info_catalog(), (480.0, 520.0), &Theme::dark(), "info_dark");
}

#[test]
fn info_light_high_contrast() {
    snapshot(
        info_catalog(),
        (480.0, 520.0),
        &high_contrast(),
        "info_light_high_contrast",
    );
}
