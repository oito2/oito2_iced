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

//! Keyboard focus: Tab and Shift+Tab traversal, activation with Enter and
//! Space, and focus loss on a click outside.

use oito2_iced_test_utils as support;

use iced::keyboard::{self, key::Named};
use iced::widget::{button, column, text};
use iced::{Event, Length};
use iced_test::{selector, simulator};
use oito2_iced::widget::focusable;
use oito2_iced::{Element, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    First,
    Second,
}

fn view() -> Element<'static, Message> {
    focus::navigation(
        column![
            focusable(button("First").on_press(Message::First))
                .on_activate(Message::First)
                .id("first"),
            focusable(button("Second").on_press(Message::Second))
                .on_activate(Message::Second)
                .id("second"),
            text("Outside").width(Length::Fill).height(200),
        ]
        .spacing(8),
    )
    .into()
}

fn focused_bounds(
    ui: &mut simulator::Simulator<'_, Message, oito2_iced::Theme>,
) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn bounds_of(
    ui: &mut simulator::Simulator<'_, Message, oito2_iced::Theme>,
    id: &'static str,
) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn shift_tab() -> Event {
    Event::Keyboard(keyboard::Event::KeyPressed {
        key: Named::Tab.into(),
        modified_key: Named::Tab.into(),
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers: keyboard::Modifiers::SHIFT,
        text: None,
        repeat: false,
    })
}

#[test]
fn tab_moves_the_focus_in_layout_order() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());
    let first = bounds_of(&mut ui, "first");
    let second = bounds_of(&mut ui, "second");

    assert_eq!(focused_bounds(&mut ui), None);

    ui.tap_key(Named::Tab);
    assert_eq!(focused_bounds(&mut ui), Some(first));

    ui.tap_key(Named::Tab);
    assert_eq!(focused_bounds(&mut ui), Some(second));

    ui.simulate([shift_tab()]);
    assert_eq!(focused_bounds(&mut ui), Some(first));
}

#[test]
fn enter_and_space_activate_the_focused_element() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::First, Message::Second]);
}

#[test]
fn keys_do_nothing_without_focus_and_clicks_still_work() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    ui.tap_key(Named::Enter);
    ui.click("Second").expect("the second button is visible");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Second]);
}

#[test]
fn a_click_outside_removes_the_focus() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    ui.tap_key(Named::Tab);
    assert!(focused_bounds(&mut ui).is_some());

    ui.click("Outside").expect("the text is visible");
    assert_eq!(focused_bounds(&mut ui), None);
}
