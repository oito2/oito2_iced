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

//! Carousels: the previous and next buttons, the arrow keys and the
//! reading direction, the dots, swipes, automatic scrolling, the disabled
//! state and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{column, container, text};
use iced::{Border, Color, Event, Length, Point, Rectangle, mouse, window};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, radius, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::carousel::{self, Controller};
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Page(usize),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// The height of the pages in these tests.
const HEIGHT: f32 = 120.0;

/// `count` pages, each showing "Slide <index>".
fn slides(count: usize) -> Vec<Element<'static, Message>> {
    (0..count)
        .map(|index| {
            container(text(format!("Slide {index}")))
                .center(Length::Fill)
                .into()
        })
        .collect()
}

fn view(carousel: carousel::Carousel<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            carousel.height(HEIGHT).id("carousel"),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn sim(carousel: carousel::Carousel<'static, Message>) -> Simulator<'static> {
    Simulator::with_size(support::settings(), (400.0, 300.0), view(carousel))
}

fn bounds(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("carousel"))
        .expect("carousel")
        .bounds()
}

fn click(ui: &mut Simulator<'_>, at: Point) {
    ui.point_at(at);
    ui.simulate(simulator::click());
}

/// A point on the previous button (or the next one when `forward`).
fn control(area: Rectangle, forward: bool) -> Point {
    let x = if forward {
        area.x + area.width - spacing::SM - 6.0
    } else {
        area.x + spacing::SM + 6.0
    };

    Point::new(x, area.y + HEIGHT / 2.0)
}

fn redraw_at(ui: &mut Simulator<'_>, now: Instant) {
    ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[cfg(feature = "svg")]
#[test]
fn the_buttons_ask_for_the_next_and_previous_page_and_wrap() {
    let mut ui =
        sim(carousel::carousel(&Controller::new(), slides(3)).on_page_change(Message::Page));
    let area = bounds(&mut ui);

    assert!(ui.find("Slide 0").is_ok());
    click(&mut ui, control(area, true));
    click(&mut ui, control(area, false));

    assert_eq!(messages(ui), vec![Message::Page(1), Message::Page(2)]);
}

#[cfg(feature = "svg")]
#[test]
fn without_looping_the_ends_disable_the_buttons() {
    let mut ui = sim(carousel::carousel(&Controller::new(), slides(2))
        .looping(false)
        .on_page_change(Message::Page));
    let area = bounds(&mut ui);

    click(&mut ui, control(area, false));
    click(&mut ui, control(area, true));
    assert_eq!(messages(ui), vec![Message::Page(1)]);

    let last = Controller::new().with_initial_page(1);
    let mut ui = sim(carousel::carousel(&last, slides(2))
        .looping(false)
        .on_page_change(Message::Page));
    let area = bounds(&mut ui);

    click(&mut ui, control(area, true));
    click(&mut ui, control(area, false));
    assert_eq!(messages(ui), vec![Message::Page(0)]);
}

#[test]
fn the_arrow_keys_step_and_wrap_while_focused() {
    let mut ui =
        sim(carousel::carousel(&Controller::new(), slides(3)).on_page_change(Message::Page));
    let area = bounds(&mut ui);

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(area));

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(messages(ui), vec![Message::Page(1), Message::Page(2)]);
}

#[test]
fn right_to_left_swaps_the_arrow_keys() {
    let mut ui = sim(carousel::carousel(&Controller::new(), slides(3))
        .direction(ReadingDirection::RightToLeft)
        .on_page_change(Message::Page));

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(messages(ui), vec![Message::Page(1)]);
}

#[test]
fn the_dots_ask_for_their_page() {
    let mut ui =
        sim(carousel::carousel(&Controller::new(), slides(4)).on_page_change(Message::Page));
    let area = bounds(&mut ui);

    // Four interactive dots are four 24-pixel targets with no gap between
    // them, centered below the pages.
    let dots_y = area.y + HEIGHT + spacing::SM + 12.0;
    click(&mut ui, Point::new(area.center_x() + 36.0, dots_y));

    assert_eq!(messages(ui), vec![Message::Page(3)]);
}

#[test]
fn a_swipe_snaps_to_the_nearest_page() {
    let mut ui =
        sim(carousel::carousel(&Controller::new(), slides(3)).on_page_change(Message::Page));
    let area = bounds(&mut ui);
    let start = Point::new(area.center_x(), area.y + 20.0);
    let end = Point::new(start.x - area.width * 0.6, start.y);

    ui.point_at(start);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(end);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: end })]);
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    assert_eq!(messages(ui), vec![Message::Page(1)]);

    // A short swipe returns to the page it started on.
    let mut ui =
        sim(carousel::carousel(&Controller::new(), slides(3)).on_page_change(Message::Page));
    let back = Point::new(start.x - area.width * 0.2, start.y);
    ui.point_at(start);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(back);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: back })]);
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    assert!(messages(ui).is_empty());
}

#[test]
fn automatic_scrolling_advances_and_wraps() {
    let interval = Duration::from_secs(2);
    let controller = Controller::new()
        .with_auto_scroll(true)
        .with_auto_scroll_interval(interval);
    let mut ui = sim(carousel::carousel(&controller, slides(2)).on_page_change(Message::Page));
    let now = Instant::now();

    ui.point_at(Point::new(399.0, 299.0));
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + interval / 2);
    redraw_at(&mut ui, now + interval);
    assert_eq!(messages(ui), vec![Message::Page(1)]);

    let last = controller.clone().with_initial_page(1);
    let mut ui = sim(carousel::carousel(&last, slides(2)).on_page_change(Message::Page));

    ui.point_at(Point::new(399.0, 299.0));
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + interval);
    assert_eq!(messages(ui), vec![Message::Page(0)]);
}

#[test]
fn automatic_scrolling_pauses_on_hover_and_without_animations() {
    let interval = Duration::from_secs(1);
    let controller = Controller::new()
        .with_auto_scroll(true)
        .with_auto_scroll_interval(interval);
    let now = Instant::now();

    let mut ui = sim(carousel::carousel(&controller, slides(2)).on_page_change(Message::Page));
    let area = bounds(&mut ui);
    let inside = Point::new(area.center_x(), area.y + 10.0);

    ui.point_at(inside);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: inside })]);
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + interval * 3);
    assert!(messages(ui).is_empty());

    let mut ui = sim(carousel::carousel(&controller, slides(2))
        .animated(false)
        .on_page_change(Message::Page));

    ui.point_at(Point::new(399.0, 299.0));
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + interval * 3);
    assert!(messages(ui).is_empty());
}

#[test]
fn without_a_message_the_carousel_is_still_and_skipped() {
    let mut ui = sim(carousel::carousel(
        &Controller::new().with_auto_scroll(true),
        slides(3),
    ));
    let area = bounds(&mut ui);
    let after = ui.find(selector::id("after")).expect("after").bounds();

    click(&mut ui, control(area, true));
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(after));

    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_secs(10));

    assert!(messages(ui).is_empty());
}

#[test]
fn the_pages_follow_a_swipe() {
    let mut ui = sim(colored(&Controller::new()).on_page_change(Message::Page));
    let area = bounds(&mut ui);
    let start = Point::new(area.center_x(), area.y + 20.0);
    let end = Point::new(start.x - area.width / 2.0, start.y);

    ui.point_at(start);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(end);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: end })]);

    let theme = Theme::light();
    let image = support::pixels(&ui.snapshot(&theme).expect("snapshot"), "carousel_swipe");
    let roles = theme.roles();
    let probe = |x: f32| support::pixel_at(&image, Point::new(x, area.y + 30.0), 400.0);

    assert_close(probe(area.x + area.width * 0.25), roles.primary_container);
    assert_close(probe(area.x + area.width * 0.75), roles.secondary_container);
}

fn assert_close(actual: Color, expected: Color) {
    let close = |a: f32, b: f32| (a - b).abs() < 0.02;

    assert!(
        close(actual.r, expected.r) && close(actual.g, expected.g) && close(actual.b, expected.b),
        "{actual:?} is not {expected:?}"
    );
}

/// A carousel of three pages in the container colors.
fn colored(controller: &Controller) -> carousel::Carousel<'static, Message> {
    let page = |color: fn(&Theme) -> Color| -> Element<'static, Message> {
        container(
            container(iced::widget::Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |theme: &Theme| container::Style {
                    background: Some(color(theme).into()),
                    border: Border::default().rounded(radius::LARGE),
                    ..container::Style::default()
                }),
        )
        .padding([0, 4])
        .into()
    };

    carousel::carousel(
        controller,
        [
            page(|theme| theme.roles().primary_container),
            page(|theme| theme.roles().secondary_container),
            page(|theme| theme.roles().tertiary_container),
        ],
    )
}

fn snapshot(theme: &Theme, name: &str, focused: bool) {
    let mut ui = Simulator::with_size(
        support::settings(),
        (400.0, 220.0),
        focus::navigation(
            container(
                colored(&Controller::new().with_initial_page(1))
                    .height(140.0)
                    .on_page_change(Message::Page),
            )
            .padding(16),
        ),
    );

    if focused {
        ui.tap_key(Named::Tab);
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn carousel_light() {
    snapshot(&Theme::light(), "carousel_light", false);
}

#[test]
fn carousel_dark() {
    snapshot(&Theme::dark(), "carousel_dark", false);
}

#[test]
fn carousel_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "carousel_light_high_contrast", false);
}

#[test]
fn carousel_focused() {
    snapshot(&Theme::light(), "carousel_focused", true);
}
