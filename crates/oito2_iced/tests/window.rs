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

//! Window chrome: the title bar and the resize handles publish the window
//! actions for each pointer gesture.

use oito2_iced_test_utils as support;

use iced::widget::{column, text};
use iced::window::Direction;
use iced::{Event, Length, Point, mouse};
use iced_test::simulator::{self, Simulator};
use oito2_iced::Element;
use oito2_iced::window::{self, Action};

const SIZE: (f32, f32) = (800.0, 600.0);

fn view() -> Element<'static, Action> {
    window::resize_handles(
        column![
            window::title_bar("Title", text(""), |action| action),
            text("Content").width(Length::Fill).height(Length::Fill),
        ],
        |action| action,
    )
}

fn press_at(point: Point, events: impl IntoIterator<Item = Event>) -> Vec<String> {
    let mut ui = Simulator::with_size(support::settings(), SIZE, view());
    ui.point_at(point);
    ui.simulate(events);

    ui.into_messages()
        .map(|action| format!("{action:?}"))
        .collect()
}

fn right_press() -> [Event; 2] {
    [
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
    ]
}

#[test]
fn a_press_on_the_title_bar_drags_the_window() {
    let actions = press_at(Point::new(200.0, 23.0), simulator::click());

    assert_eq!(actions, vec![format!("{:?}", Action::Drag)]);
}

#[test]
fn a_double_click_on_the_title_bar_toggles_maximization() {
    let actions = press_at(
        Point::new(200.0, 23.0),
        simulator::click().chain(simulator::click()),
    );

    assert!(actions.contains(&format!("{:?}", Action::ToggleMaximize)));
}

#[test]
fn a_right_press_on_the_title_bar_opens_the_system_menu() {
    let actions = press_at(Point::new(200.0, 23.0), right_press());

    assert_eq!(actions, vec![format!("{:?}", Action::ShowSystemMenu)]);
}

#[test]
fn the_close_button_closes_without_dragging() {
    let mut ui = Simulator::with_size(support::settings(), SIZE, view());
    ui.click("\u{2715}").expect("close button");

    let actions: Vec<String> = ui.into_messages().map(|a| format!("{a:?}")).collect();
    assert_eq!(actions, vec![format!("{:?}", Action::Close)]);
}

#[test]
fn the_edges_and_corners_resize() {
    let cases = [
        (Point::new(400.0, 2.0), Direction::North),
        (Point::new(400.0, 598.0), Direction::South),
        (Point::new(2.0, 300.0), Direction::West),
        (Point::new(798.0, 300.0), Direction::East),
        (Point::new(2.0, 2.0), Direction::NorthWest),
        (Point::new(798.0, 2.0), Direction::NorthEast),
        (Point::new(2.0, 598.0), Direction::SouthWest),
        (Point::new(798.0, 598.0), Direction::SouthEast),
    ];

    for (point, direction) in cases {
        let actions = press_at(point, simulator::click());

        assert_eq!(
            actions,
            vec![format!("{:?}", Action::Resize(direction))],
            "press at {point:?}"
        );
    }
}

#[test]
fn the_content_is_not_covered_by_the_handles() {
    let actions = press_at(Point::new(400.0, 300.0), simulator::click());

    assert!(actions.is_empty(), "{actions:?}");
}
