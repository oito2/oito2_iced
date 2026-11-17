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

//! Draggables: pointer drags, cancelled drags, the arrow keys, the pointer
//! through to the layers below, the still state and golden snapshots. Run
//! with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, stack, text};
use iced::{Border, Event, Length, Point, Rectangle, mouse};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, radius};
use oito2_iced::widget::button;
use oito2_iced::widget::draggable::{self, Draggable};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Moved(Point),
    Started,
    Ended,
    Below,
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// A 40 × 40 square in the primary color.
fn square() -> Element<'static, Message> {
    container(text(""))
        .width(40)
        .height(40)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.roles().primary.into()),
            border: Border::default().rounded(radius::MEDIUM),
            ..container::Style::default()
        })
        .into()
}

fn view(draggable: Draggable<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            container(stack![
                container(button::filled("Below").on_press(Message::Below))
                    .align_right(Length::Fill)
                    .align_bottom(Length::Fill)
                    .padding(8),
                draggable.id("note"),
            ])
            .width(300)
            .height(200),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8),
    )
    .into()
}

fn sim(draggable: Draggable<'static, Message>) -> Simulator<'static> {
    Simulator::with_size(support::settings(), (320.0, 260.0), view(draggable))
}

fn note(position: Point) -> Draggable<'static, Message> {
    draggable::draggable(position, square())
        .on_move(Message::Moved)
        .on_drag_start(Message::Started)
        .on_drag_end(Message::Ended)
}

fn drag(ui: &mut Simulator<'_>, from: Point, to: Point, end: Event) {
    ui.point_at(from);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    for step in 1..=3 {
        let t = step as f32 / 3.0;
        let at = Point::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t);

        ui.point_at(at);
        ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    }

    ui.simulate([end]);
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn a_pointer_drag_moves_the_content() {
    let mut ui = sim(note(Point::new(20.0, 20.0)));

    drag(
        &mut ui,
        Point::new(40.0, 40.0),
        Point::new(70.0, 50.0),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    );

    let messages = messages(ui);

    assert_eq!(messages.len(), 5);
    assert_eq!(messages[0], Message::Started);
    assert_eq!(messages[3], Message::Moved(Point::new(50.0, 30.0)));
    assert_eq!(messages[4], Message::Ended);
}

#[test]
fn a_drag_cut_short_ends() {
    let mut ui = sim(note(Point::ORIGIN));

    drag(
        &mut ui,
        Point::new(20.0, 20.0),
        Point::new(50.0, 20.0),
        Event::Mouse(mouse::Event::CursorLeft),
    );

    let messages = messages(ui);
    assert_eq!(messages.first(), Some(&Message::Started));
    assert_eq!(messages.last(), Some(&Message::Ended));
}

#[test]
fn the_arrow_keys_move_the_focused_content() {
    let mut ui = sim(note(Point::new(50.0, 30.0)).keyboard_step(8.0));

    ui.tap_key(Named::ArrowDown);
    // The button of the layer below comes first.
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    assert_eq!(
        focused(&mut ui),
        Some(Rectangle::new(
            Point::new(50.0, 30.0),
            iced::Size::new(40.0, 40.0)
        ))
    );
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(
        messages(ui),
        vec![
            Message::Moved(Point::new(50.0, 38.0)),
            Message::Moved(Point::new(42.0, 30.0)),
        ]
    );
}

#[test]
fn the_pointer_reaches_the_layers_below_outside_the_content() {
    let mut ui = sim(note(Point::ORIGIN));
    let below = ui.find("Below").expect("button below").bounds();

    ui.point_at(below.center());
    ui.simulate(simulator::click());

    assert_eq!(messages(ui), vec![Message::Below]);
}

#[test]
fn without_a_move_message_the_content_stays_still_and_is_skipped() {
    let mut ui = sim(draggable::draggable(Point::new(20.0, 20.0), square()));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    drag(
        &mut ui,
        Point::new(40.0, 40.0),
        Point::new(90.0, 60.0),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    );
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), Some(after));
    assert!(messages(ui).is_empty());
}

#[test]
fn the_cursor_grabs_the_content() {
    let _fonts = sim(note(Point::ORIGIN));

    assert_eq!(
        support::mouse_interaction(
            view(note(Point::ORIGIN)),
            (320.0, 260.0),
            Point::new(20.0, 20.0)
        ),
        mouse::Interaction::Grab
    );
}

fn snapshot(theme: &Theme, name: &str, focused: bool) {
    let mut ui = Simulator::with_size(
        support::settings(),
        (320.0, 260.0),
        view(note(Point::new(130.0, 40.0))),
    );

    if focused {
        ui.tap_key(Named::Tab);
        ui.tap_key(Named::Tab);
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn draggable_light() {
    snapshot(&Theme::light(), "draggable_light", false);
}

#[test]
fn draggable_dark() {
    snapshot(&Theme::dark(), "draggable_dark", false);
}

#[test]
fn draggable_light_high_contrast_focused() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "draggable_light_high_contrast_focused", true);
}
