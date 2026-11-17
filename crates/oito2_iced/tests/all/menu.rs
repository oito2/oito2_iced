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

//! Menus, selects, context menus and split buttons: opening, item
//! activation by click and keyboard, disabled items, sizes per density and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::{self, key::Named};
use iced::widget::{column, container, row, text};
use iced::{Event, Length, Point, mouse};
use iced_test::{selector, simulator};
use oito2_iced::collections::ListCollection;
use oito2_iced::density::{self, Density};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, context_menu, menu, split_button};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Open(bool),
    Sort(u8),
    Hidden(bool),
    Choose(String),
    Save,
    Context(bool),
    Copy,
}

type Ui = simulator::Simulator<'static, Message, Theme>;

fn sort_items(sort: u8) -> Vec<menu::Item<'static, Message>> {
    vec![
        menu::checked_item(sort == 0, "Name").on_press(Message::Sort(0)),
        menu::checked_item(sort == 1, "Size"),
        menu::checked_item(sort == 2, "Date").on_press(Message::Sort(2)),
        menu::separator(),
        menu::multi_select_item(false, "Hidden files").on_change(Message::Hidden),
    ]
}

fn popup(open: bool) -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            menu::popup_menu_button("Sort", sort_items(0))
                .open(open)
                .on_open_change(Message::Open)
                .id("popup"),
            menu::popup_menu_button("Disabled", sort_items(0)).id("disabled"),
            text("Below").width(Length::Fill).height(Length::Fill),
        ]
        .spacing(8)
        .padding(20),
    )
    .into()
}

fn messages(ui: Ui) -> Vec<Message> {
    ui.into_messages().collect()
}

fn bounds(ui: &mut Ui, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn focused(ui: &mut Ui) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn key(key: Named, modifiers: keyboard::Modifiers) -> Event {
    Event::Keyboard(keyboard::Event::KeyPressed {
        key: keyboard::Key::Named(key),
        modified_key: keyboard::Key::Named(key),
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
        text: None,
        repeat: false,
    })
}

#[test]
fn a_click_on_the_button_requests_the_menu() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(false));
    assert!(ui.find("Name").is_err(), "closed menus show no items");

    ui.click("Sort").expect("button");
    ui.click("Disabled").expect("disabled button");

    assert_eq!(messages(ui), vec![Message::Open(true)]);
}

#[test]
fn enter_space_and_down_open_the_focused_button_and_disabled_ones_are_skipped() {
    for open_key in [Named::Enter, Named::Space, Named::ArrowDown] {
        let mut ui = simulator::Simulator::with_settings(support::settings(), popup(false));
        let popup_bounds = bounds(&mut ui, "popup");

        ui.tap_key(Named::Tab);
        assert_eq!(focused(&mut ui), Some(popup_bounds));
        ui.tap_key(open_key);

        ui.tap_key(Named::Tab);
        assert_eq!(focused(&mut ui), None, "the disabled button is skipped");

        assert_eq!(messages(ui), vec![Message::Open(true)], "{open_key:?}");
    }
}

#[test]
fn the_menu_opens_below_the_button() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));
    let button = bounds(&mut ui, "popup");
    let name = ui.find("Name").expect("menu item").bounds();

    assert!(name.y >= button.y + button.height);
    assert!(name.x >= button.x);
}

#[test]
fn a_click_on_an_item_publishes_it_and_closes_the_menu() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

    ui.click("Date").expect("item");

    assert_eq!(messages(ui), vec![Message::Sort(2), Message::Open(false)]);
}

#[test]
fn disabled_items_ignore_clicks() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

    ui.click("Size").expect("disabled item");

    assert_eq!(messages(ui), vec![]);
}

#[test]
fn a_multi_select_item_toggles_and_keeps_the_menu_open() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

    ui.click("Hidden files").expect("item");

    assert_eq!(messages(ui), vec![Message::Hidden(true)]);
}

#[test]
fn arrows_skip_disabled_items_and_stop_at_the_ends() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

    // Name, then Date (Size is disabled), then Hidden files, which stays.
    for _ in 0..4 {
        ui.tap_key(Named::ArrowDown);
    }
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::Enter);

    assert_eq!(messages(ui), vec![Message::Sort(2), Message::Open(false)]);
}

#[test]
fn home_end_and_up_from_nothing_pick_the_ends() {
    let cases = [
        (Named::Home, vec![Message::Sort(0), Message::Open(false)]),
        (Named::End, vec![Message::Hidden(true)]),
        (Named::ArrowUp, vec![Message::Hidden(true)]),
    ];

    for (jump, expected) in cases {
        let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

        ui.tap_key(jump);
        ui.tap_key(Named::Space);

        assert_eq!(messages(ui), expected, "{jump:?}");
    }
}

#[test]
fn escape_a_click_outside_and_tab_close_the_menu() {
    let mut escape = simulator::Simulator::with_settings(support::settings(), popup(true));
    escape.tap_key(Named::Escape);
    assert_eq!(messages(escape), vec![Message::Open(false)]);

    let mut outside = simulator::Simulator::with_settings(support::settings(), popup(true));
    outside.point_at(Point::new(600.0, 600.0));
    outside.simulate(simulator::click());
    assert_eq!(messages(outside), vec![Message::Open(false)]);

    let mut tab = simulator::Simulator::with_settings(support::settings(), popup(true));
    tab.tap_key(Named::Tab);
    assert_eq!(messages(tab), vec![Message::Open(false)]);
}

#[test]
fn a_click_on_the_button_while_open_closes_the_menu() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));

    ui.click("Sort").expect("button");

    assert_eq!(messages(ui), vec![Message::Open(false)]);
}

#[test]
fn buttons_and_items_follow_the_control_height_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), popup(true));
        let button = bounds(&mut ui, "popup");
        let name = ui.find("Name").expect("item").bounds();
        let date = ui.find("Date").expect("item").bounds();

        assert_eq!(button.height, height, "{density:?}");
        assert!(button.width >= menu::MIN_WIDTH, "{density:?}");
        assert!((date.y - name.y - 2.0 * height).abs() < 0.01, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn fruits() -> ListCollection<(&'static str, &'static str, &'static str, bool)> {
    ListCollection::with_value_of(
        [
            ("apple", "Apple", "Fruits", false),
            ("banana", "Banana", "Fruits", false),
            ("cherry", "Cherry", "Fruits", true),
            ("carrot", "Carrot", "Vegetables", false),
        ],
        |item| item.0.to_owned(),
    )
    .expect("unique values")
    .with_label_of(|item| item.1.to_owned())
    .with_disabled(|item| item.3)
    .with_group_of(|item| Some(item.2.to_owned()))
}

fn select_view(open: bool, value: Option<&str>) -> Element<'static, Message> {
    let fruits = fruits();

    oito2_iced::focus::navigation(
        column![
            menu::select(&fruits, value)
                .placeholder("Pick one")
                .open(open)
                .on_open_change(Message::Open)
                .on_change(Message::Choose)
                .id("select"),
            text("Below").width(Length::Fill).height(Length::Fill),
        ]
        .padding(20),
    )
    .into()
}

#[test]
fn a_select_shows_the_selected_label_or_the_placeholder() {
    let mut chosen = simulator::Simulator::with_settings(
        support::settings(),
        select_view(false, Some("banana")),
    );
    assert!(chosen.find("Banana").is_ok());

    let mut empty =
        simulator::Simulator::with_settings(support::settings(), select_view(false, None));
    assert!(empty.find("Pick one").is_ok());
}

#[test]
fn a_select_opens_from_the_keyboard() {
    for open_key in [Named::Enter, Named::Space, Named::ArrowDown] {
        let mut ui =
            simulator::Simulator::with_settings(support::settings(), select_view(false, None));

        ui.tap_key(Named::Tab);
        ui.tap_key(open_key);

        assert_eq!(messages(ui), vec![Message::Open(true)], "{open_key:?}");
    }
}

#[test]
fn a_select_lists_groups_and_picks_enabled_values() {
    let mut ui =
        simulator::Simulator::with_settings(support::settings(), select_view(true, Some("apple")));

    assert!(ui.find("Fruits").is_ok());
    assert!(ui.find("Vegetables").is_ok());

    ui.click("Fruits").expect("header");
    ui.click("Cherry").expect("disabled item");
    ui.click("Carrot").expect("item");

    assert_eq!(
        messages(ui),
        vec![Message::Choose("carrot".to_owned()), Message::Open(false)]
    );
}

#[test]
fn a_select_without_on_change_is_disabled() {
    let fruits = fruits();
    let view: Element<'static, Message> = oito2_iced::focus::navigation(
        menu::select(&fruits, None)
            .placeholder("Pick one")
            .on_open_change(Message::Open),
    )
    .into();
    let mut ui = simulator::Simulator::with_settings(support::settings(), view);

    ui.click("Pick one").expect("select");
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), None);
    assert_eq!(messages(ui), vec![]);
}

fn region(open: bool) -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            context_menu::context_menu(
                container(
                    button::filled("Target")
                        .on_press(Message::Save)
                        .id("target")
                )
                .padding(40)
                .width(Length::Fill),
                [
                    menu::item("Copy").on_press(Message::Copy),
                    menu::item("Paste"),
                ],
            )
            .open(open)
            .on_open_change(Message::Context),
            text("Below").width(Length::Fill).height(Length::Fill),
        ]
        .padding(20),
    )
    .into()
}

fn right_click() -> [Event; 2] {
    [
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
    ]
}

#[test]
fn a_right_click_in_the_region_requests_the_menu() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), region(false));

    ui.point_at(Point::new(30.0, 30.0));
    ui.simulate(right_click());
    ui.point_at(Point::new(30.0, 400.0));
    ui.simulate(right_click());

    assert_eq!(messages(ui), vec![Message::Context(true)]);
}

#[test]
fn shift_f10_and_the_menu_key_open_the_menu_only_with_the_focus_inside() {
    let mut unfocused = simulator::Simulator::with_settings(support::settings(), region(false));
    unfocused.simulate([key(Named::F10, keyboard::Modifiers::SHIFT)]);
    unfocused.simulate([key(Named::ContextMenu, keyboard::Modifiers::empty())]);
    assert_eq!(messages(unfocused), vec![]);

    for event in [
        key(Named::F10, keyboard::Modifiers::SHIFT),
        key(Named::ContextMenu, keyboard::Modifiers::empty()),
    ] {
        let mut ui = simulator::Simulator::with_settings(support::settings(), region(false));

        ui.tap_key(Named::Tab);
        ui.simulate([key(Named::F10, keyboard::Modifiers::empty())]);
        ui.simulate([event]);

        assert_eq!(messages(ui), vec![Message::Context(true)]);
    }
}

#[test]
fn the_open_context_menu_publishes_its_items_and_closes() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), region(true));

    ui.click("Paste").expect("disabled item");
    ui.click("Copy").expect("item");

    assert_eq!(messages(ui), vec![Message::Copy, Message::Context(false)]);

    let mut escape = simulator::Simulator::with_settings(support::settings(), region(true));
    escape.tap_key(Named::Escape);
    assert_eq!(messages(escape), vec![Message::Context(false)]);
}

fn split(open: bool, enabled: bool) -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            split_button::split_button("Save", [menu::item("Save as").on_press(Message::Copy)],)
                .variant(split_button::Variant::Filled)
                .on_press_maybe(enabled.then_some(Message::Save))
                .open(open)
                .on_open_change(Message::Open)
                .id("main")
                .menu_id("arrow"),
            text("Below").width(Length::Fill).height(Length::Fill),
        ]
        .padding(20),
    )
    .into()
}

#[test]
fn the_split_button_parts_publish_the_action_and_the_menu_request() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), split(false, true));
    let main = bounds(&mut ui, "main");
    let arrow = bounds(&mut ui, "arrow");

    assert_eq!(main.height, arrow.height);
    assert_eq!(arrow.x, main.x + main.width + 1.0);

    ui.click("Save").expect("main part");
    ui.click(selector::id("arrow")).expect("arrow part");

    assert_eq!(messages(ui), vec![Message::Save, Message::Open(true)]);
}

#[test]
fn the_split_button_has_two_tab_stops() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), split(false, true));
    let main = bounds(&mut ui, "main");
    let arrow = bounds(&mut ui, "arrow");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(main));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(arrow));
    ui.tap_key(Named::Space);

    assert_eq!(messages(ui), vec![Message::Save, Message::Open(true)]);
}

#[test]
fn a_split_button_without_action_disables_both_parts() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), split(false, false));

    ui.click("Save").expect("main part");
    ui.click(selector::id("arrow")).expect("arrow part");
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), None);
    assert_eq!(messages(ui), vec![]);
}

#[test]
fn the_split_button_menu_opens_below_the_arrow() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), split(true, true));
    let arrow = bounds(&mut ui, "arrow");
    let item = ui.find("Save as").expect("menu item").bounds();

    assert!(item.y >= arrow.y + arrow.height);
    assert!(item.x >= arrow.x);

    ui.click("Save as").expect("menu item");
    assert_eq!(messages(ui), vec![Message::Copy, Message::Open(false)]);
}

fn high_contrast() -> Theme {
    Theme::base(Brightness::Light, Accent::default().color(), Contrast::High)
}

fn snapshot(
    view: Element<'static, Message>,
    size: (f32, f32),
    hover: Option<Point>,
    theme: &Theme,
    name: &str,
) {
    let mut ui = simulator::Simulator::with_size(support::settings(), size, view);

    if let Some(point) = hover {
        ui.point_at(point);
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn menu_catalog() -> Element<'static, Message> {
    let items = vec![
        menu::header("Sort by"),
        menu::checked_item(true, "Name").on_press(Message::Sort(0)),
        menu::checked_item(false, "Size"),
        menu::checked_item(false, "Date modified").on_press(Message::Sort(2)),
        menu::separator(),
        menu::multi_select_item(true, "Hidden files").on_change(Message::Hidden),
        menu::multi_select_item(false, "Folders first").on_change(Message::Hidden),
        menu::item("Reset").on_press(Message::Copy),
    ];

    row![
        menu::popup_menu_button("Name", items)
            .open(true)
            .on_open_change(Message::Open),
        menu::popup_menu_button("Closed", sort_items(0)).on_open_change(Message::Open),
        menu::popup_menu_button("Disabled", sort_items(0)),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

/// The pointer rests on the "Date modified" item of the open menu.
const MENU_HOVER: Point = Point::new(40.0, 160.0);

#[test]
fn menu_light() {
    snapshot(
        menu_catalog(),
        (480.0, 340.0),
        Some(MENU_HOVER),
        &Theme::light(),
        "menu_light",
    );
}

#[test]
fn menu_dark() {
    snapshot(
        menu_catalog(),
        (480.0, 340.0),
        Some(MENU_HOVER),
        &Theme::dark(),
        "menu_dark",
    );
}

#[test]
fn menu_light_high_contrast() {
    snapshot(
        menu_catalog(),
        (480.0, 340.0),
        Some(MENU_HOVER),
        &high_contrast(),
        "menu_light_high_contrast",
    );
}

fn select_catalog() -> Element<'static, Message> {
    let fruits = fruits();

    row![
        column![
            menu::select(&fruits, Some("banana"))
                .on_open_change(Message::Open)
                .on_change(Message::Choose),
            menu::select(&fruits, None)
                .placeholder("Pick a fruit")
                .on_open_change(Message::Open)
                .on_change(Message::Choose),
            menu::select(&fruits, Some("apple")),
        ]
        .spacing(12),
        menu::select(&fruits, Some("apple"))
            .open(true)
            .on_open_change(Message::Open)
            .on_change(Message::Choose),
    ]
    .spacing(24)
    .padding(12)
    .into()
}

#[test]
fn select_light() {
    snapshot(
        select_catalog(),
        (400.0, 260.0),
        None,
        &Theme::light(),
        "select_light",
    );
}

#[test]
fn select_dark() {
    snapshot(
        select_catalog(),
        (400.0, 260.0),
        None,
        &Theme::dark(),
        "select_dark",
    );
}

#[test]
fn select_light_high_contrast() {
    snapshot(
        select_catalog(),
        (400.0, 260.0),
        None,
        &high_contrast(),
        "select_light_high_contrast",
    );
}

fn split_catalog() -> Element<'static, Message> {
    let items = || {
        [
            menu::item("Save as").on_press(Message::Copy),
            menu::item("Save all").on_press(Message::Copy),
        ]
    };
    let split = |variant| {
        split_button::split_button("Save", items())
            .variant(variant)
            .on_press(Message::Save)
            .on_open_change(Message::Open)
    };

    column![
        row![
            split(split_button::Variant::Elevated),
            split(split_button::Variant::Filled),
            split(split_button::Variant::Outlined),
            split_button::split_button("Disabled", items()),
        ]
        .spacing(12),
        split(split_button::Variant::Filled).open(true),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

#[test]
fn split_button_light() {
    snapshot(
        split_catalog(),
        (560.0, 200.0),
        None,
        &Theme::light(),
        "split_button_light",
    );
}

#[test]
fn split_button_dark() {
    snapshot(
        split_catalog(),
        (560.0, 200.0),
        None,
        &Theme::dark(),
        "split_button_dark",
    );
}

#[test]
fn split_button_light_high_contrast() {
    snapshot(
        split_catalog(),
        (560.0, 200.0),
        None,
        &high_contrast(),
        "split_button_light_high_contrast",
    );
}
