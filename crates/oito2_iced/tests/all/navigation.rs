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

//! Navigation rails and navigation pages: clicks, the one keyboard stop,
//! disabled rails, widths per style, the layout chosen by width and input,
//! scrolling to the selected destination and golden snapshots of the
//! catalog. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, row, text};
use iced::{Length, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::lucide;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::navigation_page;
use oito2_iced::widget::navigation_rail::{self, NavigationItem, NavigationRailStyle};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Go(usize),
    Pressed,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn items() -> [NavigationItem<'static>; 3] {
    [
        navigation_rail::item(lucide::HOUSE, "Home"),
        navigation_rail::item(lucide::STAR, "Starred"),
        navigation_rail::item(lucide::TRASH, "Trash"),
    ]
}

fn rail(style: NavigationRailStyle, enabled: bool) -> Element<'static, Message> {
    focus::navigation(
        row![
            navigation_rail::navigation_rail(items(), Some(0))
                .style(style)
                .on_select_maybe(enabled.then_some(Message::Go)),
            oito2_iced::widget::button::filled("After").on_press(Message::Pressed),
        ]
        .height(400),
    )
    .into()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

/// Whether the focused element holds the text `label`.
fn focus_holds(ui: &mut Simulator<'_>, label: &'static str) -> bool {
    let center = ui.find(label).expect("text exists").bounds().center();

    focused(ui).is_some_and(|bounds| bounds.contains(center))
}

#[test]
fn a_click_on_the_pill_or_the_label_selects_a_destination() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        rail(NavigationRailStyle::Labelled, true),
    );

    ui.click("Trash").expect("label");
    ui.click("Home").expect("label");

    assert_eq!(messages(ui), vec![Message::Go(2), Message::Go(0)]);
}

#[test]
fn the_rail_is_one_stop_and_the_arrows_move_the_focus() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        rail(NavigationRailStyle::LabelledExtended, true),
    );

    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "Home"));

    ui.tap_key(Named::ArrowDown);
    assert!(focus_holds(&mut ui, "Starred"));
    ui.tap_key(Named::End);
    ui.tap_key(Named::ArrowDown);
    assert!(focus_holds(&mut ui, "Trash"), "stops at the end");
    ui.tap_key(Named::Home);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert!(focus_holds(&mut ui, "Starred"), "keeps the focus");

    ui.tap_key(Named::Tab);
    assert!(!focus_holds(&mut ui, "Starred"), "Tab leaves the rail");

    assert_eq!(messages(ui), vec![Message::Go(1)], "moving does not select");
}

#[test]
fn a_disabled_rail_publishes_nothing_and_is_skipped() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        rail(NavigationRailStyle::Labelled, false),
    );

    ui.click("Starred").expect("label");
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);

    assert_eq!(messages(ui), vec![Message::Pressed]);
}

#[test]
fn the_rail_width_follows_the_style() {
    for (style, width) in [
        (NavigationRailStyle::Compact, 64.0),
        (NavigationRailStyle::Labelled, 88.0),
        (NavigationRailStyle::LabelledExtended, 240.0),
    ] {
        let view: Element<'static, Message> = container(
            navigation_rail::navigation_rail(items(), Some(0))
                .style(style)
                .on_select(Message::Go),
        )
        .id("rail")
        .height(300)
        .into();
        let mut ui = Simulator::with_settings(support::settings(), view);

        let rail = ui.find(selector::id("rail")).expect("rail").bounds();
        assert_eq!(rail.width, width, "{style:?}");
    }
}

#[test]
fn a_compact_rail_hides_the_labels() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        rail(NavigationRailStyle::Compact, true),
    );

    assert!(ui.find("Starred").is_err());
}

#[test]
fn a_rail_item_alone_publishes_its_message() {
    let view: Element<'static, Message> = focus::navigation(
        navigation_rail::rail_item(
            navigation_rail::item(lucide::STAR, "Starred"),
            true,
            NavigationRailStyle::LabelledExtended,
        )
        .on_press(Message::Pressed),
    )
    .into();
    let mut ui = Simulator::with_settings(support::settings(), view);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Space);
    ui.click("Starred").expect("item");

    assert_eq!(messages(ui), vec![Message::Pressed, Message::Pressed]);
}

fn page(selected: usize, extended: Option<f32>) -> Element<'static, Message> {
    navigation_page::navigation_page(items(), selected, text(format!("Page {selected}")))
        .extended_breakpoint(extended)
        .on_select(Message::Go)
        .into()
}

#[test]
fn wide_layouts_use_a_labelled_rail() {
    let mut ui = Simulator::with_size(support::settings(), (800.0, 500.0), page(0, None));

    let page = ui.find("Page 0").expect("page").bounds();
    assert_eq!(page.x, 88.0 + 1.0, "after the rail and the separator");

    ui.click("Trash").expect("label");
    assert_eq!(messages(ui), vec![Message::Go(2)]);
}

#[test]
fn the_rail_extends_past_the_extended_breakpoint() {
    let mut extended =
        Simulator::with_size(support::settings(), (1000.0, 500.0), page(0, Some(900.0)));
    let mut labelled =
        Simulator::with_size(support::settings(), (1000.0, 500.0), page(0, Some(1200.0)));

    assert_eq!(extended.find("Page 0").expect("page").bounds().x, 241.0);
    assert_eq!(labelled.find("Page 0").expect("page").bounds().x, 89.0);
}

#[test]
fn narrow_pointer_layouts_use_a_compact_rail() {
    let mut ui = Simulator::with_size(support::settings(), (400.0, 500.0), page(1, None));

    assert_eq!(ui.find("Page 1").expect("page").bounds().x, 65.0);
    assert!(ui.find("Starred").is_err(), "no labels");
}

#[test]
fn narrow_touch_layouts_use_a_bottom_bar() {
    density::set(Density::Touch);

    let mut ui = Simulator::with_size(support::settings(), (400.0, 500.0), page(0, None));
    let page = ui.find("Page 0").expect("page").bounds();
    let label = ui.find("Starred").expect("label").bounds();

    assert_eq!(page.x, 0.0);
    assert!(label.y > 500.0 - 80.0, "{label:?}");

    ui.click("Starred").expect("label");
    assert_eq!(messages(ui), vec![Message::Go(1)]);

    density::set(Density::Pointer);
}

#[test]
fn the_rail_shows_a_selected_destination_far_below() {
    let items: Vec<NavigationItem<'static>> = (0..40)
        .map(|index| navigation_rail::item(lucide::STAR, format!("Item {index}")))
        .collect();
    let view: Element<'static, Message> =
        navigation_page::navigation_page(items, 35, text("Page 35"))
            .on_select(Message::Go)
            .into();
    let mut ui = Simulator::with_size(support::settings(), (800.0, 400.0), view);

    let item = ui.find("Item 35").expect("item").bounds();
    assert!(item.y >= 0.0 && item.y + item.height <= 400.0, "{item:?}");

    ui.click("Item 35").expect("item");
    assert_eq!(messages(ui), vec![Message::Go(35)]);
}

fn catalog() -> Element<'static, Message> {
    let rail = |style, selected| {
        navigation_rail::navigation_rail(items(), Some(selected))
            .style(style)
            .on_select(Message::Go)
    };

    container(
        row![
            rail(NavigationRailStyle::Compact, 0),
            rail(NavigationRailStyle::Labelled, 1),
            rail(NavigationRailStyle::LabelledExtended, 2),
            column![
                container(page(0, None)).height(200),
                navigation_rail::navigation_rail(items(), None)
                    .style(NavigationRailStyle::Labelled)
            ]
            .spacing(8)
            .width(Length::Fill),
        ]
        .spacing(8),
    )
    .class(oito2_iced::theme::container::Class::Background)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (720.0, 360.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn navigation_rail_light() {
    snapshot(&Theme::light(), "navigation_rail_light");
}

#[test]
fn navigation_rail_dark() {
    snapshot(&Theme::dark(), "navigation_rail_dark");
}

#[test]
fn navigation_rail_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "navigation_rail_light_high_contrast");
}
