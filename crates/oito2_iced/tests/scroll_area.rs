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

//! Scroll areas: wheel scrolling on each axis, the vertical wheel on a
//! horizontal area with undershoot, the undershoot shadows and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{Space, column, container, row, text};
use iced::{Event, Length, Point, mouse};
use iced_test::simulator;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::scroll_area::{self, Direction};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
struct Scrolled {
    x: f32,
    y: f32,
}

fn area(
    direction: Direction,
    undershoot: bool,
    width: f32,
    height: f32,
) -> Element<'static, Scrolled> {
    container(
        scroll_area::scroll_area(Space::new().width(width).height(height))
            .direction(direction)
            .undershoot(undershoot)
            .on_scroll(|viewport| {
                let offset = viewport.absolute_offset();
                Scrolled {
                    x: offset.x,
                    y: offset.y,
                }
            }),
    )
    .width(200)
    .height(200)
    .into()
}

fn wheel(ui: &mut simulator::Simulator<'_, Scrolled, Theme>, x: f32, y: f32) {
    ui.point_at(Point::new(100.0, 100.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Pixels { x, y },
    })]);
}

fn last(ui: simulator::Simulator<'_, Scrolled, Theme>) -> Option<Scrolled> {
    ui.into_messages().last()
}

#[test]
fn scrolls_vertically() {
    let mut ui = simulator::Simulator::with_settings(
        support::settings(),
        area(Direction::Vertical, false, 100.0, 1000.0),
    );
    wheel(&mut ui, 0.0, -300.0);

    assert_eq!(last(ui), Some(Scrolled { x: 0.0, y: 300.0 }));
}

#[test]
fn scrolls_both_ways() {
    let mut ui = simulator::Simulator::with_settings(
        support::settings(),
        area(Direction::Both, false, 1000.0, 1000.0),
    );
    wheel(&mut ui, -300.0, 0.0);
    wheel(&mut ui, 0.0, -200.0);

    let messages: Vec<Scrolled> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Scrolled { x: 300.0, y: 0.0 },
            Scrolled { x: 300.0, y: 200.0 }
        ]
    );
}

#[test]
fn the_vertical_wheel_scrolls_a_horizontal_area_with_undershoot() {
    let mut plain = simulator::Simulator::with_settings(
        support::settings(),
        area(Direction::Horizontal, false, 1000.0, 100.0),
    );
    wheel(&mut plain, 0.0, -300.0);
    assert_eq!(last(plain).map_or(0.0, |scrolled| scrolled.x), 0.0);

    let mut ui = simulator::Simulator::with_settings(
        support::settings(),
        area(Direction::Horizontal, true, 1000.0, 100.0),
    );
    wheel(&mut ui, 0.0, -300.0);
    wheel(&mut ui, -10.0, 0.0);

    // The scroll made for the vertical wheel publishes no viewport; the
    // next horizontal step, taken by the scroll view, reports the total.
    assert_eq!(last(ui), Some(Scrolled { x: 310.0, y: 0.0 }));
}

/// A tall column of rows, to scroll.
fn rows(count: usize) -> Element<'static, Scrolled> {
    column((0..count).map(|index| text(format!("Row {index}")).into()))
        .spacing(8)
        .width(Length::Fill)
        .into()
}

fn catalog() -> Element<'static, Scrolled> {
    row![
        container(scroll_area::scroll_area(rows(30)).padding(8))
            .width(140)
            .height(200),
        container(
            scroll_area::scroll_area(rows(30))
                .padding(8)
                .undershoot(true)
        )
        .width(140)
        .height(200),
        container(
            scroll_area::scroll_area(
                row((0..12).map(|index| text(format!("Item {index}")).into())).spacing(12)
            )
            .direction(Direction::Horizontal)
            .padding(8)
            .undershoot(true)
        )
        .width(180)
        .height(60),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) -> support::Image {
    let mut ui = simulator::Simulator::with_size(support::settings(), (520.0, 224.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
    support::pixels(&snapshot, name)
}

#[test]
fn scroll_area_light() {
    let image = snapshot(&Theme::light(), "scroll_area_light");

    // The undershoot darkens the bottom edge, which hides rows, and leaves
    // the top edge, at the start of the content, as the plain area.
    let plain_bottom = support::pixel_at(&image, Point::new(80.0, 210.0), 520.0);
    let shadow_bottom = support::pixel_at(&image, Point::new(232.0, 210.0), 520.0);
    let plain_top = support::pixel_at(&image, Point::new(80.0, 13.0), 520.0);
    let shadow_top = support::pixel_at(&image, Point::new(232.0, 13.0), 520.0);

    assert!(
        shadow_bottom.r < plain_bottom.r,
        "{shadow_bottom:?} {plain_bottom:?}"
    );
    assert_eq!(shadow_top, plain_top);
}

#[test]
fn scroll_area_dark() {
    let _ = snapshot(&Theme::dark(), "scroll_area_dark");
}

#[test]
fn scroll_area_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    let _ = snapshot(&theme, "scroll_area_light_high_contrast");
}

#[test]
fn the_top_shadow_fades_in_after_scrolling() {
    let view = || -> Element<'static, Scrolled> {
        container(scroll_area::scroll_area(rows(30)).undershoot(true))
            .width(200)
            .height(200)
            .into()
    };
    let mut ui = simulator::Simulator::with_size(support::settings(), (200.0, 200.0), view());
    let theme = Theme::light();

    let rest = support::pixels(&ui.snapshot(&theme).expect("snapshot"), "rest");
    wheel(&mut ui, 0.0, -100.0);
    let _ = ui.snapshot(&theme).expect("snapshot");
    std::thread::sleep(scroll_area::UNDERSHOOT_FADE * 2);
    let scrolled = support::pixels(&ui.snapshot(&theme).expect("snapshot"), "scrolled");

    let top = Point::new(120.0, 1.0);
    let before = support::pixel_at(&rest, top, 200.0);
    let after = support::pixel_at(&scrolled, top, 200.0);

    assert!(after.r < before.r, "{after:?} {before:?}");
}
