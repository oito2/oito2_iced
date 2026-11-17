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

//! Focus groups: arrow keys inside an edge interceptor, and the focus ring
//! drawn outside a focused element or around a group with the focus.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{button, column, container, row, text_input};
use iced::{Color, Point, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::widget::focusable;
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Pressed,
    Input(String),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn item(id: &'static str) -> Element<'static, Message> {
    focusable(button(id).on_press(Message::Pressed))
        .id(id)
        .into()
}

fn group_view() -> Element<'static, Message> {
    focus::navigation(
        column![
            item("before"),
            focus::edge_interceptor(row![item("a"), item("b"), item("c")].spacing(8)),
            item("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn bounds(ui: &mut Simulator<'_>, id: &'static str) -> Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

#[test]
fn arrows_move_inside_the_group_and_stop_at_its_ends() {
    let mut ui = Simulator::with_settings(support::settings(), group_view());
    let [a, b, c, after] = ["a", "b", "c", "after"].map(|id| bounds(&mut ui, id));

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(a));

    ui.tap_key(Named::ArrowRight);
    assert_eq!(focused(&mut ui), Some(b));

    ui.tap_key(Named::ArrowDown);
    assert_eq!(focused(&mut ui), Some(c));

    ui.tap_key(Named::ArrowRight);
    assert_eq!(focused(&mut ui), Some(c), "stops at the last widget");

    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::ArrowUp);
    assert_eq!(focused(&mut ui), Some(a));

    ui.tap_key(Named::ArrowLeft);
    assert_eq!(focused(&mut ui), Some(a), "stops at the first widget");

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(after), "Tab leaves the group");
}

#[test]
fn arrows_do_nothing_while_the_focus_is_outside_the_group() {
    let mut ui = Simulator::with_settings(support::settings(), group_view());
    let before = bounds(&mut ui, "before");

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowRight);

    assert_eq!(focused(&mut ui), Some(before));
}

/// Logical size of the simulated window.
const SIZE: (f32, f32) = (400.0, 200.0);

/// The color of the pixel at `point`, in logical pixels.
fn pixel(ui: &mut Simulator<'_>, point: Point) -> Color {
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::pixel_at(&support::pixels(&snapshot, "focus-ring"), point, SIZE.0)
}

fn close(a: Color, b: Color) -> bool {
    let [ar, ag, ab, _] = a.into_rgba8();
    let [br, bg, bb, _] = b.into_rgba8();

    ar.abs_diff(br) <= 8 && ag.abs_diff(bg) <= 8 && ab.abs_diff(bb) <= 8
}

/// A point one pixel outside the middle of the left edge of `bounds`.
fn left_of(bounds: Rectangle) -> Point {
    Point::new(bounds.x - 1.0, bounds.center_y())
}

#[test]
fn the_ring_of_a_focusable_is_drawn_outside_it() {
    let view = || -> Element<'static, Message> {
        focus::navigation(
            container(item("only"))
                .padding(24)
                .class(oito2_iced::theme::container::Class::Background),
        )
        .into()
    };
    let mut ui = Simulator::with_size(support::settings(), SIZE, view());
    let only = bounds(&mut ui, "only");
    let ring = Theme::light().focus_ring();

    assert!(!close(pixel(&mut ui, left_of(only)), ring));

    ui.tap_key(Named::Tab);

    assert!(close(pixel(&mut ui, left_of(only)), ring));
}

#[test]
fn the_group_ring_follows_the_keyboard_focus() {
    let view = || -> Element<'static, Message> {
        focus::navigation(
            container(focus::ring(
                text_input("Name", "").on_input(Message::Input).id("name"),
            ))
            .padding(24)
            .width(300)
            .class(oito2_iced::theme::container::Class::Background),
        )
        .into()
    };
    let mut ui = Simulator::with_size(support::settings(), SIZE, view());
    let field = bounds(&mut ui, "name");
    let ring = Theme::light().focus_ring();

    assert!(!close(pixel(&mut ui, left_of(field)), ring), "no focus");

    ui.tap_key(Named::Tab);
    assert!(
        close(pixel(&mut ui, left_of(field)), ring),
        "keyboard focus"
    );

    ui.point_at(field.center());
    ui.simulate(simulator::click());
    assert!(
        !close(pixel(&mut ui, left_of(field)), ring),
        "hidden after a mouse press"
    );
}
