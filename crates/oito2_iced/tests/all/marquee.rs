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

//! Behaviour and golden tests of the marquee widget: the scroll toward the
//! start, the copy and the wrap, the reading direction, the vertical axis,
//! pausing on hover and focus, reduced motion and input routing.
//! Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{Space, column, container, row, text};
use iced::{Color, Event, Length, Point, Rectangle, mouse, window};
use iced_test::{selector, simulator};
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::marquee::{self, Axis, Marquee};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Go,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// The width of the window in these tests.
const WIDTH: f32 = 200.0;

/// A block of `length` × 20 in a color of the theme.
fn block(length: f32, color: fn(&Theme) -> Color) -> Element<'static, Message> {
    container(Space::new().width(length).height(20))
        .style(move |theme: &Theme| container::Style {
            background: Some(color(theme).into()),
            ..container::Style::default()
        })
        .into()
}

/// Content 120 pixels long: the primary color, then the tertiary color,
/// 60 pixels each.
fn stripes() -> Element<'static, Message> {
    row![
        block(60.0, |theme| theme.roles().primary),
        block(60.0, |theme| theme.roles().tertiary),
    ]
    .into()
}

/// A 100 pixel wide marquee of `content`, at 40 pixels per second, without
/// faded ends, in a container named "marquee" at the top left.
fn sim(marquee: Marquee<'static, Message>) -> Simulator<'static> {
    Simulator::with_size(
        support::settings(),
        (WIDTH, 120.0),
        focus::navigation(
            column![
                container(marquee.fade_edges(false).width(100)).id("marquee"),
                button::filled("After").on_press(Message::Go).id("after"),
            ]
            .spacing(8),
        ),
    )
}

fn bounds(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("marquee")).expect("marquee").bounds()
}

fn redraw_at(ui: &mut Simulator<'_>, now: Instant) {
    ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
}

/// Redraws now and `seconds` later; a snapshot afterwards redraws at the
/// real time, which does not move the content back.
fn run(ui: &mut Simulator<'_>, seconds: u64) {
    let now = Instant::now();

    redraw_at(ui, now);
    redraw_at(ui, now + Duration::from_secs(seconds));
}

/// The color at `point` of a snapshot.
fn probe(ui: &mut Simulator<'_>, point: Point) -> Color {
    let image = support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), "marquee");

    support::pixel_at(&image, point, WIDTH)
}

fn assert_close(actual: Color, expected: Color) {
    let close = |a: f32, b: f32| (a - b).abs() < 0.02;

    assert!(
        close(actual.r, expected.r) && close(actual.g, expected.g) && close(actual.b, expected.b),
        "{actual:?} is not {expected:?}"
    );
}

fn primary() -> Color {
    Theme::light().roles().primary
}

fn tertiary() -> Color {
    Theme::light().roles().tertiary
}

fn background() -> Color {
    Theme::light().roles().surface
}

/// The point `x` pixels into the marquee, halfway down the content.
fn at(area: Rectangle, x: f32) -> Point {
    Point::new(area.x + x, area.y + 10.0)
}

#[test]
fn overflowing_content_moves_toward_the_start() {
    let mut ui = sim(marquee::marquee(stripes()));
    let area = bounds(&mut ui);

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    assert_close(probe(&mut ui, at(area, 50.0)), primary());
    run(&mut ui, 1);
    // 40 pixels later the point shows the content at 90.
    assert_close(probe(&mut ui, at(area, 50.0)), tertiary());
    run(&mut ui, 1);
    // At 130 the gap of 32 pixels shows the background.
    assert_close(probe(&mut ui, at(area, 50.0)), background());
}

#[test]
fn the_copy_follows_after_the_gap_and_the_offset_wraps() {
    let mut ui = sim(marquee::marquee(stripes()));
    let area = bounds(&mut ui);

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    // 3 × 40 = 120 pixels: the copy starts at 120 + 32 − 120 = 32.
    run(&mut ui, 3);
    assert_close(probe(&mut ui, at(area, 20.0)), background());
    assert_close(probe(&mut ui, at(area, 40.0)), primary());
    // 152 more pixels (one period, 3.8 s) bring back the same picture.
    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_millis(3800));
    assert_close(probe(&mut ui, at(area, 20.0)), background());
    assert_close(probe(&mut ui, at(area, 40.0)), primary());
}

#[test]
fn right_to_left_and_reverse_move_toward_the_end() {
    for marquee in [
        marquee::marquee(stripes()).direction(ReadingDirection::RightToLeft),
        marquee::marquee(stripes()).reverse(true),
    ] {
        let mut ui = sim(marquee);
        let area = bounds(&mut ui);

        ui.point_at(Point::new(WIDTH - 1.0, 119.0));
        run(&mut ui, 1);
        // The copy came 40 pixels in from the start; the gap is before it.
        assert_close(probe(&mut ui, at(area, 50.0)), primary());
        assert_close(probe(&mut ui, at(area, 20.0)), background());
    }
}

#[test]
fn a_reversed_right_to_left_marquee_moves_toward_the_start() {
    let mut ui = sim(marquee::marquee(stripes())
        .direction(ReadingDirection::RightToLeft)
        .reverse(true));
    let area = bounds(&mut ui);

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 50.0)), tertiary());
}

#[test]
fn a_vertical_marquee_moves_up() {
    let content = column![
        block(20.0, |theme| theme.roles().primary),
        block(20.0, |theme| theme.roles().tertiary),
        block(20.0, |theme| theme.roles().primary),
    ];
    let mut ui = sim(marquee::marquee(content).axis(Axis::Vertical).height(40));
    let area = bounds(&mut ui);
    let probe_at = |area: Rectangle, y: f32| Point::new(area.center_x(), area.y + y);

    assert_eq!(area.height, 40.0);
    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    assert_close(probe(&mut ui, probe_at(area, 30.0)), tertiary());

    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_millis(500));
    // 20 pixels up, the point at 10 shows the content at 30.
    assert_close(probe(&mut ui, probe_at(area, 10.0)), tertiary());

    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_millis(500));
    // 40 pixels up, the point at 30 shows the content at 70: the gap.
    assert_close(probe(&mut ui, probe_at(area, 30.0)), background());
}

#[test]
fn content_that_fits_stays_still_at_the_start() {
    let short = || block(40.0, |theme| theme.roles().primary);

    let mut ui = sim(marquee::marquee(short()));
    let area = bounds(&mut ui);

    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 10.0)), primary());
    assert_close(probe(&mut ui, at(area, 90.0)), background());

    let mut ui = sim(marquee::marquee(short()).direction(ReadingDirection::RightToLeft));
    let area = bounds(&mut ui);

    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 10.0)), background());
    assert_close(probe(&mut ui, at(area, 90.0)), primary());
}

#[test]
fn the_pointer_over_it_pauses_the_scroll() {
    let mut ui = sim(marquee::marquee(stripes()));
    let area = bounds(&mut ui);
    let inside = at(area, 90.0);

    ui.point_at(inside);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: inside })]);
    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 50.0)), primary());

    let outside = Point::new(WIDTH - 1.0, 119.0);
    ui.point_at(outside);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: outside,
    })]);
    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 50.0)), tertiary());
}

#[test]
fn without_pause_on_hover_the_pointer_does_not_stop_it() {
    let mut ui = sim(marquee::marquee(stripes()).pause_on_hover(false));
    let area = bounds(&mut ui);
    let inside = at(area, 90.0);

    ui.point_at(inside);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: inside })]);
    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 50.0)), tertiary());
}

#[test]
fn the_focus_inside_pauses_the_scroll() {
    let content = row![
        button::filled("Go").on_press(Message::Go).id("go"),
        block(160.0, |theme| theme.roles().primary),
    ];
    let mut ui = sim(marquee::marquee(content));

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    ui.tap_key(Named::Tab);
    let go = ui.find(selector::id("go")).expect("go").bounds();
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(go)
    );

    let before = support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), "marquee");
    run(&mut ui, 1);
    let after = support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), "marquee");

    assert_eq!(before.rgba, after.rgba);
}

#[test]
fn without_animations_it_stays_at_the_start() {
    let mut ui = sim(marquee::marquee(stripes()).animated(false));
    let area = bounds(&mut ui);

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    run(&mut ui, 1);
    assert_close(probe(&mut ui, at(area, 50.0)), primary());
}

#[test]
fn clicks_reach_the_content_where_it_is_drawn() {
    let content = row![
        button::filled("Go").on_press(Message::Go),
        Space::new().width(200),
    ];
    let mut ui = sim(marquee::marquee(content));
    let area = bounds(&mut ui);

    ui.point_at(Point::new(WIDTH - 1.0, 119.0));
    // 1.5 s at 40 pixels per second moves the button 60 pixels out.
    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_millis(1500));

    ui.point_at(at(area, 5.0));
    ui.simulate(simulator::click());
    assert!(ui.into_messages().next().is_none());

    let content = row![
        button::filled("Go").on_press(Message::Go),
        Space::new().width(200),
    ];
    let mut ui = sim(marquee::marquee(content));

    ui.point_at(at(area, 5.0));
    ui.simulate(simulator::click());
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), vec![Message::Go]);
}

fn snapshot(theme: &Theme, name: &str, marquee: Marquee<'static, Message>) {
    let mut ui = Simulator::with_size(
        support::settings(),
        (320.0, 120.0),
        container(
            container(marquee.fade_color(|theme| theme.roles().surface_container))
                .padding([8, 0])
                .style(|theme: &Theme| container::Style {
                    background: Some(theme.roles().surface_container.into()),
                    ..container::Style::default()
                }),
        )
        .padding(16),
    );

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn headline() -> Marquee<'static, Message> {
    marquee::marquee(text(
        "oito2_ui 0.1.0 · Linux, Windows, macOS, Android, iOS and web · GPL-3.0",
    ))
}

fn ticker() -> Marquee<'static, Message> {
    marquee::marquee(
        column![
            text("Build 128 passed"),
            text("Deploy to Pages done"),
            text("3 new issues"),
            text("Release 0.1.0 tagged"),
            text("Goldens updated"),
        ]
        .width(Length::Shrink),
    )
    .axis(Axis::Vertical)
    .height(64)
}

#[test]
fn marquee_light() {
    snapshot(&Theme::light(), "marquee_light", headline());
}

#[test]
fn marquee_dark() {
    snapshot(&Theme::dark(), "marquee_dark", headline());
}

#[test]
fn marquee_vertical_light() {
    snapshot(&Theme::light(), "marquee_vertical_light", ticker());
}

#[test]
fn marquee_fits_right_to_left_dark() {
    snapshot(
        &Theme::dark(),
        "marquee_fits_rtl_dark",
        marquee::marquee(text("Short")).direction(ReadingDirection::RightToLeft),
    );
}
