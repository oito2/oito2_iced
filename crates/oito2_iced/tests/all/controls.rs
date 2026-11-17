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

//! Toggle buttons, option buttons and list tiles: clicks on the label,
//! focus as one stop, keyboard activation, sizes per density and golden
//! snapshots of the catalog. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row, text};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::list_tile::{self, ControlAffinity};
use oito2_iced::widget::{controls, toggles};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Check(Option<bool>),
    Pick(Option<u8>),
    Switch(bool),
    Toggled,
    Add,
    Open,
}

type Ui = simulator::Simulator<'static, Message, Theme>;

fn ui(view: impl Into<Element<'static, Message>>) -> Ui {
    simulator::Simulator::with_settings(support::settings(), oito2_iced::focus::navigation(view))
}

fn bounds(ui: &mut Ui, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn focused(ui: &mut Ui) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn click_at(ui: &mut Ui, point: iced::Point) {
    ui.point_at(point);
    let _ = ui.simulate(simulator::click());
}

#[test]
fn the_label_toggles_the_control() {
    let mut ui = ui(column![
        controls::check_button(
            toggles::checkbox(false).on_change(Message::Check),
            "Show hidden files"
        ),
        controls::radio_button(toggles::radio(2, Some(1)).on_change(Message::Pick), "Two"),
        controls::switch_button(
            toggles::switch(false).on_toggle(Message::Switch),
            "Dark style"
        ),
        controls::check_button(
            toggles::checkbox(true)
                .tristate(true)
                .on_change(Message::Check),
            "Tristate"
        )
        .id("tristate"),
    ]);

    ui.click("Show hidden files").expect("check button");
    ui.click("Two").expect("radio button");
    ui.click("Dark style").expect("switch button");

    let tristate = bounds(&mut ui, "tristate");
    click_at(
        &mut ui,
        iced::Point::new(tristate.x + 17.0, tristate.center_y()),
    );

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Check(Some(true)),
            Message::Pick(Some(2)),
            Message::Switch(true),
            Message::Check(None),
        ]
    );
}

#[test]
fn the_row_is_one_focus_stop_and_space_toggles_it() {
    let mut ui = ui(column![
        controls::check_button(
            toggles::checkbox(false).on_change(Message::Check),
            "Show hidden files"
        )
        .id("check"),
        controls::toggle_button(
            toggles::checkbox(false).on_change(Message::Check),
            "Generic"
        )
        .on_toggle(Message::Toggled)
        .id("generic"),
        controls::option_button(text("+"))
            .label("Add")
            .on_press(Message::Add)
            .id("add"),
    ]);
    let check = bounds(&mut ui, "check");
    let generic = bounds(&mut ui, "generic");
    let add = bounds(&mut ui, "add");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(check));
    ui.tap_key(Named::Space);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(generic));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(add));
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![Message::Check(Some(true)), Message::Toggled, Message::Add]
    );
}

#[test]
fn a_generic_toggle_button_keeps_the_input_of_its_control() {
    let mut ui = ui(controls::toggle_button(
        toggles::switch(false).on_toggle(Message::Switch),
        "Label",
    )
    .on_toggle(Message::Toggled)
    .id("row"));
    let row = bounds(&mut ui, "row");

    ui.click("Label").expect("label");
    click_at(
        &mut ui,
        iced::Point::new(row.x + 4.0 + 25.0, row.center_y()),
    );

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Toggled, Message::Switch(true)]);
}

#[test]
fn disabled_rows_ignore_clicks_and_tab() {
    let mut ui = ui(column![
        controls::check_button(toggles::checkbox::<Message>(false), "Unavailable"),
        controls::toggle_button(toggles::checkbox::<Message>(false), "No message"),
        list_tile::switch_list_tile(toggles::switch::<Message>(false)).title("Disabled tile"),
        list_tile::list_tile()
            .title("Disabled row")
            .on_press(Message::Open)
            .enabled(false),
        controls::option_button::<Message>(text("+")).id("option"),
        list_tile::list_tile().title("Static row").id("static"),
        controls::switch_button(toggles::switch(false).on_toggle(Message::Switch), "Enabled")
            .id("enabled"),
    ]);
    let enabled = bounds(&mut ui, "enabled");

    for label in [
        "Unavailable",
        "No message",
        "Disabled tile",
        "Disabled row",
        "Static row",
    ] {
        ui.click(label).expect("label");
    }

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(enabled));

    let messages: Vec<Message> = ui.into_messages().collect();
    assert!(messages.is_empty(), "{messages:?}");
}

#[test]
fn list_tiles_toggle_their_control_from_anywhere() {
    let mut ui = ui(column![
        list_tile::checkbox_list_tile(toggles::checkbox(false).on_change(Message::Check))
            .title("Wi-Fi")
            .id("wifi"),
        list_tile::radio_list_tile(toggles::radio(2, Some(1)).on_change(Message::Pick))
            .title("Two"),
        list_tile::switch_list_tile(toggles::switch(false).on_toggle(Message::Switch))
            .title("Bluetooth")
            .id("bluetooth"),
        list_tile::switch_list_tile(toggles::switch(false).on_toggle(Message::Switch))
            .control_affinity(ControlAffinity::Leading)
            .title("Airplane mode")
            .id("airplane"),
        list_tile::list_tile()
            .title("Network")
            .on_press(Message::Open)
            .id("network"),
    ]);
    let wifi = bounds(&mut ui, "wifi");
    let bluetooth = bounds(&mut ui, "bluetooth");

    ui.click("Wi-Fi").expect("tile");
    ui.click("Two").expect("tile");
    ui.click("Bluetooth").expect("tile");
    // The switch of a trailing tile sits at the end of the row.
    click_at(
        &mut ui,
        iced::Point::new(bluetooth.x + bluetooth.width - 30.0, bluetooth.center_y()),
    );
    ui.click("Network").expect("tile");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(wifi));
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Check(Some(true)),
            Message::Pick(Some(2)),
            Message::Switch(true),
            Message::Switch(true),
            Message::Open,
            Message::Check(Some(true)),
        ]
    );
}

#[test]
fn sizes_follow_the_density() {
    for (density, one_line, two_lines) in
        [(Density::Pointer, 40.0, 52.0), (Density::Touch, 56.0, 72.0)]
    {
        density::set(density);

        let mut ui = ui(column![
            list_tile::list_tile().title("One").id("one"),
            list_tile::list_tile()
                .title("Two")
                .subtitle("Lines")
                .id("two"),
            controls::option_button(text("+"))
                .on_press(Message::Add)
                .id("option"),
            controls::check_button(toggles::checkbox(false).on_change(Message::Check), "Row")
                .id("row"),
        ]);
        let one = bounds(&mut ui, "one");
        let two = bounds(&mut ui, "two");
        let option = bounds(&mut ui, "option");
        let row = bounds(&mut ui, "row");

        assert!(one.height >= one_line, "{density:?} {one:?}");
        assert!(one.height < one_line + 1.0, "{density:?} {one:?}");
        assert!(two.height >= two_lines, "{density:?} {two:?}");
        assert!(two.height < two_lines + 1.0, "{density:?} {two:?}");
        assert_eq!((option.width, option.height), (34.0, 34.0), "{density:?}");
        let target = if density == Density::Touch {
            48.0
        } else {
            34.0
        };
        assert_eq!(row.height, target + 4.0, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let buttons = column![
        controls::check_button(
            toggles::checkbox(true).on_change(Message::Check),
            "Show hidden files"
        )
        .subtitle("Files starting with a dot"),
        controls::check_button(
            toggles::checkbox(None)
                .tristate(true)
                .on_change(Message::Check),
            "Mixed"
        ),
        controls::radio_button(toggles::radio(1, Some(1)).on_change(Message::Pick), "Small"),
        controls::radio_button(toggles::radio(2, Some(1)).on_change(Message::Pick), "Large"),
        controls::switch_button(
            toggles::switch(true).on_toggle(Message::Switch),
            "Dark style"
        ),
        controls::check_button(toggles::checkbox::<Message>(false), "Unavailable")
            .subtitle("Disabled"),
        row![
            controls::option_button(icon(lucide::PLUS))
                .label("Add")
                .on_press(Message::Add),
            controls::option_button(icon(lucide::MINUS)).label("Remove"),
            controls::option_color(iced::Color::from_rgb8(0x35, 0x84, 0xE4), "Blue")
                .on_press(Message::Add),
            controls::option_color(iced::Color::WHITE, "White").on_press(Message::Add),
        ]
        .spacing(8),
    ]
    .spacing(4)
    .width(260);

    let tiles = column![
        list_tile::list_tile()
            .title("Network")
            .subtitle("Connected")
            .leading(icon(lucide::WIFI))
            .trailing(icon(lucide::CHEVRON_RIGHT))
            .on_press(Message::Open),
        list_tile::list_tile()
            .title("Selected")
            .leading(icon(lucide::FOLDER))
            .selected(true)
            .on_press(Message::Open),
        list_tile::list_tile()
            .title("Disabled")
            .leading(icon(lucide::FOLDER))
            .on_press(Message::Open)
            .enabled(false),
        list_tile::checkbox_list_tile(toggles::checkbox(true).on_change(Message::Check))
            .title("Wi-Fi")
            .secondary(icon(lucide::WIFI)),
        list_tile::radio_list_tile(toggles::radio(1, Some(2)).on_change(Message::Pick))
            .title("Radio")
            .subtitle("Subtitle"),
        list_tile::switch_list_tile(toggles::switch(false).on_toggle(Message::Switch))
            .title("Bluetooth"),
        list_tile::switch_list_tile(toggles::switch::<Message>(true)).title("Unavailable"),
    ]
    .width(300);

    row![buttons, tiles].spacing(16).padding(8).into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (600.0, 340.0),
        oito2_iced::focus::navigation(catalog()),
    );
    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn controls_light() {
    snapshot(&Theme::light(), "controls_light");
}

#[test]
fn controls_dark() {
    snapshot(&Theme::dark(), "controls_dark");
}

#[test]
fn controls_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "controls_light_high_contrast");
}

#[test]
fn a_selected_radio_row_still_shows_the_hand_cursor() {
    let view = || -> Element<'static, Message> {
        column![
            iced::widget::container(controls::radio_button(
                toggles::radio(1, Some(1)).on_change(Message::Pick),
                "Selected"
            ))
            .id("row"),
            iced::widget::container(
                list_tile::radio_list_tile(toggles::radio(1, Some(1)).on_change(Message::Pick))
                    .title("Selected tile")
            )
            .id("tile"),
        ]
        .into()
    };

    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    for id in ["row", "tile"] {
        let bounds = ui.find(selector::id(id)).expect("row").bounds();
        let interaction = support::mouse_interaction(view(), (1024.0, 768.0), bounds.center());

        assert_eq!(interaction, iced::mouse::Interaction::Pointer, "{id}");
    }
}

const YELLOW: iced::Color = iced::Color::from_rgb8(0xF8, 0xE4, 0x5C);
const DISK_BLUE: iced::Color = iced::Color::from_rgb8(0x1A, 0x5F, 0xB4);

#[test]
fn a_color_disk_rings_itself_in_a_contrasting_shade_of_its_color() {
    use oito2_iced::theme::contrast;

    let surface = Theme::light().roles().surface;
    let ring = controls::color_disk_ring(YELLOW, surface);

    // Light yellow on a light surface is darkened to stand out, keeping its
    // hue; a contrasting blue stays as it is.
    assert!(contrast::ratio(ring, surface) >= contrast::NON_TEXT_AA);
    let hue = |color: iced::Color| {
        let [r, g, b, _] = color.into_rgba8();
        let (r, g, b) = (f32::from(r), f32::from(g), f32::from(b));
        let (max, min) = (r.max(g).max(b), r.min(g).min(b));
        // Red is the largest channel of both yellows.
        60.0 * ((g - b) / (max - min))
    };
    assert!((hue(ring) - hue(YELLOW)).abs() < 1.0);
    assert_eq!(controls::color_disk_ring(DISK_BLUE, surface), DISK_BLUE);

    assert!(controls::color_disk_blends(iced::Color::WHITE, surface));
    assert!(!controls::color_disk_blends(DISK_BLUE, surface));
}

#[test]
fn a_color_disk_is_a_focus_stop_pressed_by_click_and_keys() {
    let mut ui = ui(row![
        controls::color_disk(YELLOW, "Yellow")
            .selected(true)
            .on_press(Message::Add)
            .id("yellow"),
        controls::color_disk(DISK_BLUE, "Blue").id("blue"),
        controls::color_disk(DISK_BLUE, "Blue")
            .on_press(Message::Open)
            .id("open"),
    ]);

    let yellow = bounds(&mut ui, "yellow");
    assert_eq!(yellow.size(), iced::Size::new(28.0, 28.0));

    click_at(&mut ui, yellow.center());
    let blue = bounds(&mut ui, "blue");
    click_at(&mut ui, blue.center());

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(yellow));
    ui.tap_key(Named::Tab);
    assert_eq!(
        focused(&mut ui),
        Some(bounds(&mut ui, "open")),
        "the disabled disk is skipped"
    );
    ui.tap_key(Named::Space);
    ui.tap_key(Named::Enter);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Add, Message::Open, Message::Open]
    );
}

fn color_disks() -> Element<'static, Message> {
    let colors = [
        iced::Color::WHITE,
        YELLOW,
        DISK_BLUE,
        iced::Color::from_rgb8(0x24, 0x1F, 0x31),
        iced::Color::from_rgba8(0xE0, 0x1B, 0x24, 0.5),
    ];
    let disks = |selected: bool| {
        row(colors.map(|color| {
            controls::color_disk(color, "Color")
                .selected(selected)
                .on_press(Message::Add)
                .into()
        }))
        .spacing(4)
    };

    column![
        disks(false),
        disks(true),
        controls::color_disk(DISK_BLUE, "Large")
            .size(32.0)
            .selected(true)
            .on_press(Message::Add),
    ]
    .spacing(8)
    .padding(8)
    .into()
}

fn disk_snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (180.0, 130.0),
        oito2_iced::focus::navigation(color_disks()),
    );
    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn color_disk_light() {
    disk_snapshot(&Theme::light(), "color_disk_light");
}

#[test]
fn color_disk_dark() {
    disk_snapshot(&Theme::dark(), "color_disk_dark");
}

#[test]
fn color_disk_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    disk_snapshot(&theme, "color_disk_light_high_contrast");
}
