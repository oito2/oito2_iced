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

//! Anchored panels: side placements, nesting (a press inside an inner panel
//! keeps the outer one open; Escape closes the top one first) and the arrow.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{button, column, container, text};
use iced::{Length, Point};
use iced_test::simulator;
use oito2_iced::overlay::{Placement, anchored};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Outer,
    Inner,
    DismissOuter,
    DismissInner,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn side_view(placement: Placement) -> Element<'static, Message> {
    container(
        anchored(button("Anchor").on_press(Message::Outer), text("Panel"))
            .placement(placement)
            .open(true),
    )
    .center(Length::Fill)
    .into()
}

#[test]
fn start_and_end_open_beside_the_anchor() {
    for (placement, after) in [(Placement::End, true), (Placement::Start, false)] {
        let mut ui =
            Simulator::with_size(support::settings(), (600.0, 400.0), side_view(placement));
        let anchor = ui.find("Anchor").expect("anchor").bounds();
        let panel = ui.find("Panel").expect("panel").bounds();

        if after {
            assert!(panel.x >= anchor.x + anchor.width, "{placement:?}");
        } else {
            assert!(panel.x + panel.width <= anchor.x, "{placement:?}");
        }

        assert!(panel.y < anchor.y + anchor.height && panel.y + panel.height > anchor.y);
    }
}

fn nested_view(inner_open: bool) -> Element<'static, Message> {
    let inner = anchored(
        button("Inner anchor").on_press(Message::Inner),
        text("Inner item"),
    )
    .placement(Placement::End)
    .open(inner_open)
    .on_dismiss(Message::DismissInner);

    column![
        anchored(
            button("Outer anchor").on_press(Message::Outer),
            column![text("Outer panel"), inner].spacing(8),
        )
        .open(true)
        .on_dismiss(Message::DismissOuter),
        text("Below").width(Length::Fill).height(Length::Fill),
    ]
    .padding(20)
    .into()
}

#[test]
fn a_press_inside_the_inner_panel_keeps_both_open() {
    let mut ui = Simulator::with_size(support::settings(), (800.0, 600.0), nested_view(true));
    let item = ui.find("Inner item").expect("inner panel").bounds();

    ui.point_at(Point::new(item.x + item.width - 2.0, item.y + 2.0));
    ui.simulate(simulator::click());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![]);
}

#[test]
fn escape_dismisses_the_top_panel_first() {
    let mut ui = Simulator::with_size(support::settings(), (800.0, 600.0), nested_view(true));

    ui.tap_key(Named::Escape);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::DismissInner]);
}

#[test]
fn a_press_outside_both_dismisses_the_top_panel_first() {
    let mut ui = Simulator::with_size(support::settings(), (800.0, 600.0), nested_view(true));

    ui.point_at(Point::new(700.0, 550.0));
    ui.simulate(simulator::click());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::DismissInner]);
}

fn arrow_view() -> Element<'static, Message> {
    container(
        column![
            anchored(
                button("Below").on_press(Message::Outer),
                text("Popover with an arrow"),
            )
            .arrow(true)
            .open(true),
        ]
        .padding(24),
    )
    .class(ContainerClass::Background)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

#[test]
fn arrow_golden() {
    for (theme, name) in [
        (Theme::light(), "anchored_arrow_light"),
        (Theme::dark(), "anchored_arrow_dark"),
    ] {
        let mut ui = Simulator::with_size(support::settings(), (320.0, 140.0), arrow_view());
        let snapshot = ui.snapshot(&theme).expect("snapshot");

        oito2_iced_test_utils::golden!(&snapshot, name);
    }
}
