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

//! Page indicators: the target sizes, clicks and keys, the reading
//! direction, the counter fallback, the layouts, the read-only state and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container};
use iced::{Point, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::page_indicator::{self, PageIndicator, PageIndicatorLayout};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Page(usize),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn view(indicator: PageIndicator<'static, Message>, width: f32) -> Element<'static, Message> {
    focus::navigation(
        column![
            container(indicator.id("dots")).width(width),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn sim(indicator: PageIndicator<'static, Message>, width: f32) -> Simulator<'static> {
    Simulator::with_settings(support::settings(), view(indicator, width))
}

fn dots(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("dots")).expect("dots").bounds()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn interactive_dots_are_24_pixel_targets_and_take_the_keys() {
    let mut ui = sim(
        page_indicator::page_indicator(5, 1).on_select(Message::Page),
        300.0,
    );
    let row = dots(&mut ui);

    // Five 24-pixel targets; the 8-pixel spacing is absorbed by them.
    assert_eq!((row.width, row.height), (120.0, 24.0));

    ui.tap_key(Named::Tab);
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(row)
    );
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::End);
    ui.tap_key(Named::Home);

    assert_eq!(messages(ui), [2, 0, 4, 0].map(Message::Page).to_vec());
}

#[test]
fn a_click_selects_the_page_of_its_dot() {
    let mut ui = sim(
        page_indicator::page_indicator(5, 1).on_select(Message::Page),
        300.0,
    );
    let row = dots(&mut ui);

    ui.point_at(Point::new(row.x + 24.0 * 3.0 + 12.0, row.center_y()));
    ui.simulate(simulator::click());

    assert_eq!(messages(ui), vec![Message::Page(3)]);
}

#[test]
fn right_to_left_mirrors_the_dots_and_the_arrows() {
    let mut ui = sim(
        page_indicator::page_indicator(5, 1)
            .direction(ReadingDirection::RightToLeft)
            .on_select(Message::Page),
        300.0,
    );
    let row = dots(&mut ui);

    ui.point_at(Point::new(row.x + 12.0, row.center_y()));
    ui.simulate(simulator::click());
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(messages(ui), vec![Message::Page(4), Message::Page(2)]);
}

#[test]
fn falls_back_to_text_when_the_dots_do_not_fit() {
    let mut ui = sim(page_indicator::page_indicator(30, 2), 80.0);

    assert!(ui.find("3 / 30").is_ok());
}

#[test]
fn shows_the_dots_when_they_fit_the_width_exactly() {
    // A 24-pixel current pill, four 8-pixel dots and four 8-pixel gaps.
    let mut ui = sim(page_indicator::page_indicator(5, 0), 88.0);

    assert!(ui.find("1 / 5").is_err());
    assert_eq!(dots(&mut ui).width, 88.0);
}

#[test]
fn a_zero_minimum_target_shrinks_the_targets() {
    let mut ui = sim(
        page_indicator::page_indicator(5, 0)
            .min_target_size(0.0)
            .on_select(Message::Page),
        300.0,
    );
    let row = dots(&mut ui);

    assert_eq!((row.width, row.height), (88.0, 8.0));
}

#[test]
fn the_layouts_space_the_dots() {
    let width = |layout: PageIndicatorLayout, length: usize, available: f32| {
        let mut ui = sim(
            page_indicator::page_indicator(length, 0).layout(layout),
            available,
        );

        dots(&mut ui).width
    };

    // 10 dots: a 24-pixel pill, nine 8-pixel dots and nine gaps.
    assert_eq!(
        width(PageIndicatorLayout::Fixed, 10, 600.0),
        24.0 + 72.0 + 72.0
    );
    assert_eq!(
        width(PageIndicatorLayout::Stepped, 10, 600.0),
        24.0 + 72.0 + 54.0
    );
    assert_eq!(
        width(PageIndicatorLayout::Bounded, 10, 600.0),
        24.0 + 72.0 + 72.0
    );
    assert!(width(PageIndicatorLayout::Bounded, 10, 150.0) <= 150.0);
}

#[test]
fn read_only_dots_ignore_input_and_are_skipped() {
    let mut ui = sim(page_indicator::page_indicator(5, 1), 300.0);
    let row = dots(&mut ui);
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.point_at(Point::new(row.x + 4.0, row.center_y()));
    ui.simulate(simulator::click());
    ui.tap_key(Named::Tab);

    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(after)
    );
    assert!(messages(ui).is_empty());
}

fn catalog() -> Element<'static, Message> {
    let sized = |indicator: PageIndicator<'static, Message>| container(indicator).width(200);

    column![
        sized(page_indicator::page_indicator(9, 2).layout(PageIndicatorLayout::Fixed)),
        sized(page_indicator::page_indicator(9, 2).layout(PageIndicatorLayout::Stepped)),
        sized(page_indicator::page_indicator(9, 2).layout(PageIndicatorLayout::Bounded)),
        sized(page_indicator::page_indicator(5, 0).on_select(Message::Page)),
        container(page_indicator::page_indicator(30, 2)).width(80),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (240.0, 200.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn page_indicator_light() {
    snapshot(&Theme::light(), "page_indicator_light");
}

#[test]
fn page_indicator_dark() {
    snapshot(&Theme::dark(), "page_indicator_dark");
}

#[test]
fn page_indicator_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "page_indicator_light_high_contrast");
}

#[test]
fn page_indicator_focused() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (200.0, 90.0),
        view(
            page_indicator::page_indicator(5, 1).on_select(Message::Page),
            160.0,
        ),
    );

    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "page_indicator_focused");
}
