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

//! Popover: the panel shows while open and publishes the dismiss message on
//! a click outside or on Escape, but not on a click inside.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{button, column, text};
use iced::{Length, Point};
use iced_test::simulator;
use oito2_iced::Element;
use oito2_iced::widget::popover;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Toggle,
    Dismiss,
    Inside,
}

fn view(open: bool) -> Element<'static, Message> {
    column![
        popover(
            button("Anchor").on_press(Message::Toggle),
            button("Inside").on_press(Message::Inside),
        )
        .open(open)
        .on_dismiss(Message::Dismiss),
        text("Below").width(Length::Fill).height(Length::Fill),
    ]
    .padding(20)
    .into()
}

#[test]
fn the_panel_shows_only_while_open() {
    let mut closed = simulator::Simulator::with_settings(support::settings(), view(false));
    assert!(closed.find("Inside").is_err());

    let mut open = simulator::Simulator::with_settings(support::settings(), view(true));
    let anchor = open.find("Anchor").expect("anchor").bounds();
    let inside = open.find("Inside").expect("panel content").bounds();

    assert!(
        inside.y >= anchor.y + anchor.height,
        "the panel opens below the anchor"
    );
}

#[test]
fn a_click_outside_dismisses() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.point_at(Point::new(600.0, 600.0));
    ui.simulate(simulator::click());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Dismiss]);
}

#[test]
fn escape_dismisses() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.tap_key(Named::Escape);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Dismiss]);
}

#[test]
fn clicks_inside_the_panel_and_on_the_anchor_do_not_dismiss() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.click("Inside").expect("panel content");
    ui.click("Anchor").expect("anchor");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Inside, Message::Toggle]);
}
