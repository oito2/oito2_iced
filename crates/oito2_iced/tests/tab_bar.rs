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

//! Tab bars: clicks, the one keyboard stop with arrow keys, disabled bars,
//! scrolling, sizes per density and golden snapshots of the catalog. Run
//! with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, row, text};
use iced::{Event, Length, Point, Rectangle, mouse};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::lucide;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::button;
use oito2_iced::widget::tab_bar::{self, TabBarStyle};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tab(usize),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn tabs() -> [tab_bar::Tab<'static>; 3] {
    [
        tab_bar::tab("General").icon(lucide::SETTINGS),
        tab_bar::tab("Password"),
        tab_bar::tab("Billing"),
    ]
}

fn with_after(content: impl Into<Element<'static, Message>>) -> Element<'static, Message> {
    focus::navigation(
        column![
            content.into(),
            button::filled("After").on_press(Message::After).id("after")
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn horizontal(selected: usize) -> Element<'static, Message> {
    with_after(tab_bar::tab_bar(tabs(), selected).on_select(Message::Tab))
}

fn vertical(selected: usize) -> Element<'static, Message> {
    with_after(
        container(tab_bar::vertical_tab_bar(tabs(), selected).on_select(Message::Tab)).width(200),
    )
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
fn a_click_selects_a_tab() {
    let mut ui = Simulator::with_settings(support::settings(), horizontal(0));

    ui.click("Billing").expect("billing");
    ui.click("General").expect("general");

    assert_eq!(messages(ui), vec![Message::Tab(2), Message::Tab(0)]);
}

#[test]
fn the_selected_tab_is_the_keyboard_stop_and_the_arrows_select() {
    let mut ui = Simulator::with_settings(support::settings(), horizontal(1));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "Password"));

    ui.tap_key(Named::ArrowRight);
    assert!(focus_holds(&mut ui, "Billing"));
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Home);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::End);
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(after), "Tab leaves the bar");

    assert_eq!(
        messages(ui),
        vec![
            Message::Tab(2),
            Message::Tab(0),
            Message::Tab(2),
            Message::Tab(2)
        ],
        "the arrows stop at the ends"
    );
}

#[test]
fn the_vertical_bar_moves_with_up_and_down() {
    let mut ui = Simulator::with_settings(support::settings(), vertical(0));

    ui.click("Billing").expect("billing");
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::ArrowUp);

    assert_eq!(
        messages(ui),
        vec![Message::Tab(2), Message::Tab(1), Message::Tab(0)]
    );
}

#[test]
fn a_disabled_bar_publishes_nothing_and_is_skipped() {
    let mut ui =
        Simulator::with_settings(support::settings(), with_after(tab_bar::tab_bar(tabs(), 0)));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.click("Billing").expect("billing");
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), Some(after));
    assert!(messages(ui).is_empty());
}

#[test]
fn pill_tabs_share_the_width_and_subtle_tabs_start_at_the_edge() {
    let view = |style| -> Element<'static, Message> {
        container(
            tab_bar::tab_bar([tab_bar::tab("General"), tab_bar::tab("About")], 0)
                .style(style)
                .on_select(Message::Tab),
        )
        .width(600)
        .into()
    };

    let mut pill = Simulator::with_settings(support::settings(), view(TabBarStyle::Pill));
    let about = pill.find("About").expect("about").bounds();
    assert!(about.center_x() > 400.0, "{about:?}");

    let mut subtle = Simulator::with_settings(support::settings(), view(TabBarStyle::Subtle));
    let general = subtle.find("General").expect("general").bounds();
    let about = subtle.find("About").expect("about").bounds();
    assert!(general.x < 100.0, "{general:?}");
    assert!(about.x < 200.0, "{about:?}");
}

fn many(width: f32) -> Element<'static, Message> {
    container(
        tab_bar::tab_bar((0..10).map(|index| tab_bar::tab(format!("Tab {index}"))), 0)
            .scrollable(true)
            .on_select(Message::Tab),
    )
    .width(width)
    .into()
}

#[test]
fn the_keyboard_scrolls_the_focused_tab_into_view() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (300.0, 100.0),
        focus::navigation(many(300.0)),
    );

    let hidden = ui.find("Tab 9").expect("tab").bounds();
    assert!(hidden.x > 300.0, "{hidden:?}");
    ui.click("Tab 9").expect("tab");

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::End);

    let shown = ui.find("Tab 9").expect("tab").bounds();
    assert!(shown.x + shown.width <= 300.0, "{shown:?}");
    ui.click("Tab 9").expect("tab");

    assert_eq!(messages(ui), vec![Message::Tab(9), Message::Tab(9)]);
}

#[test]
fn the_wheel_scrolls_overflowing_tabs() {
    let mut ui = Simulator::with_size(support::settings(), (300.0, 100.0), many(300.0));
    let before = ui.find("Tab 1").expect("tab").bounds();

    ui.point_at(Point::new(150.0, 20.0));
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: -1.0 },
    })]);

    let after = ui.find("Tab 1").expect("tab").bounds();
    assert!(after.x < before.x, "{before:?} {after:?}");
}

#[test]
fn tabs_are_as_tall_as_a_control() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let view: Element<'static, Message> = column![
            container(tab_bar::tab_bar(tabs(), 0).on_select(Message::Tab)).id("bar"),
            container(tab_bar::vertical_tab_bar(tabs(), 0).on_select(Message::Tab))
                .width(200)
                .id("column"),
        ]
        .into();
        let mut ui = Simulator::with_settings(support::settings(), view);
        let bar = ui.find(selector::id("bar")).expect("bar").bounds();
        let column = ui.find(selector::id("column")).expect("column").bounds();

        assert_eq!(bar.height, height + 8.0, "{density:?}");
        assert_eq!(column.height, 3.0 * height + 2.0 * 2.0, "{density:?}");
    }

    density::set(Density::Pointer);
}

#[test]
fn vertical_tabs_show_the_content_beside_the_bar() {
    let view = tab_bar::vertical_tabs(tabs(), 1, text("Password page")).on_select(Message::Tab);
    let mut ui = Simulator::with_size(support::settings(), (600.0, 300.0), view);

    let page = ui.find("Password page").expect("page").bounds();
    assert!(page.x >= 200.0, "{page:?}");

    ui.click("Billing").expect("billing");
    assert_eq!(messages(ui), vec![Message::Tab(2)]);
}

fn catalog() -> Element<'static, Message> {
    let bars = column![
        tab_bar::tab_bar(tabs(), 1).on_select(Message::Tab),
        tab_bar::tab_bar(tabs(), 0)
            .style(TabBarStyle::Subtle)
            .on_select(Message::Tab),
        tab_bar::tab_bar(tabs(), 2),
        container(many(300.0)).width(Length::Shrink),
    ]
    .spacing(8)
    .width(Length::Fill);

    let vertical = row![
        container(tab_bar::vertical_tab_bar(tabs(), 0).on_select(Message::Tab)).width(180),
        container(
            tab_bar::vertical_tab_bar(tabs(), 2)
                .style(TabBarStyle::Subtle)
                .on_select(Message::Tab)
        )
        .width(180),
    ]
    .spacing(16);

    container(column![bars, vertical].spacing(16).padding(12))
        .class(oito2_iced::theme::container::Class::Background)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (640.0, 360.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn tab_bar_light() {
    snapshot(&Theme::light(), "tab_bar_light");
}

#[test]
fn tab_bar_dark() {
    snapshot(&Theme::dark(), "tab_bar_dark");
}

#[test]
fn tab_bar_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "tab_bar_light_high_contrast");
}
