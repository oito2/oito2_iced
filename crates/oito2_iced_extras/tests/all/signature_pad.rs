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

//! Tests of the signature pad.
//! Covers drawing with the mouse and fingers, one pointer at a time, the
//! undo and clear buttons, the placeholder, the read-only pad, the export
//! of drawn strokes, the cursor and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{Space, column, container, scrollable};
use iced::{Color, Event, Length, Point, Size, mouse, touch};
use iced_test::simulator;
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::spacing;
use oito2_iced::{Element, Theme, density};
use oito2_iced_extras::signature_pad::{self, Signature, StrokePoint};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Signed(signature_pad::Event),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// The window: the pad fills its width and is 180 pixels tall.
const WINDOW: (f32, f32) = (400.0, 180.0);

fn pad(signature: &Signature) -> Element<'_, Message> {
    signature_pad::signature_pad(signature)
        .on_event(Message::Signed)
        .into()
}

fn sim(element: Element<'_, Message>) -> Simulator<'_> {
    Simulator::with_size(support::settings(), WINDOW, element)
}

fn press(ui: &mut Simulator<'_>, at: Point, button: mouse::Button) {
    ui.point_at(at);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(button))]);
}

fn release(ui: &mut Simulator<'_>, button: mouse::Button) {
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(button))]);
}

fn drag(ui: &mut Simulator<'_>, from: Point, by: (f32, f32)) {
    press(ui, from, mouse::Button::Left);

    for step in 1..=4 {
        let t = step as f32 / 4.0;
        let at = Point::new(from.x + by.0 * t, from.y + by.1 * t);

        ui.point_at(at);
        ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    }

    release(ui, mouse::Button::Left);
}

fn events(ui: Simulator<'_>) -> Vec<signature_pad::Event> {
    ui.into_messages()
        .map(|Message::Signed(event)| event)
        .collect()
}

fn signed(signature: &mut Signature, events: Vec<signature_pad::Event>) {
    for event in events {
        signature.apply(event);
    }
}

/// The centers of the undo and clear buttons in the top right corner.
fn buttons() -> (Point, Point) {
    let side = density::control_height();
    let y = spacing::XS + side / 2.0;
    let clear = Point::new(WINDOW.0 - spacing::XS - side / 2.0, y);

    (Point::new(clear.x - side, y), clear)
}

fn click(ui: &mut Simulator<'_>, at: Point) {
    press(ui, at, mouse::Button::Left);
    release(ui, mouse::Button::Left);
}

#[test]
fn draws_strokes_with_the_mouse() {
    let empty = Signature::new();
    let mut ui = sim(pad(&empty));

    assert!(ui.find("Sign here").is_ok());

    drag(&mut ui, Point::new(100.0, 80.0), (80.0, 20.0));
    drag(&mut ui, Point::new(100.0, 120.0), (60.0, 0.0));

    let mut signature = Signature::new();

    signed(&mut signature, events(ui));

    assert_eq!(signature.strokes().len(), 2);
    assert_eq!(signature.size(), Size::new(400.0, 180.0));

    let first = &signature.strokes()[0];

    assert_eq!(first.len(), 5);
    assert_eq!(first[0].position, Point::new(100.0, 80.0));
    assert_eq!(first[4].position, Point::new(180.0, 100.0));
    assert!(first.iter().all(|point| (1.2..=3.6).contains(&point.width)));

    let mut ui = sim(pad(&signature));

    assert!(ui.find("Sign here").is_err());
}

#[test]
fn undo_and_clear_publish_their_events() {
    let mut signature = Signature::new();

    signature.add_stroke([StrokePoint::new(Point::new(50.0, 50.0), 2.0)]);
    signature.add_stroke([StrokePoint::new(Point::new(80.0, 50.0), 2.0)]);

    let (undo, clear) = buttons();
    let mut ui = sim(pad(&signature));

    click(&mut ui, undo);
    click(&mut ui, clear);

    let published = events(ui);

    assert_eq!(
        published,
        [signature_pad::Event::Undo, signature_pad::Event::Clear]
    );

    signature.apply(published[0]);
    assert_eq!(signature.strokes().len(), 1);
    signature.apply(published[1]);
    assert!(signature.is_empty());
}

#[test]
fn the_buttons_are_disabled_while_empty() {
    let empty = Signature::new();
    let (undo, clear) = buttons();
    let mut ui = sim(pad(&empty));

    click(&mut ui, undo);
    click(&mut ui, clear);

    assert!(events(ui).is_empty());
}

#[test]
fn the_buttons_sit_in_the_top_left_corner_right_to_left() {
    let mut signature = Signature::new();

    signature.add_stroke([StrokePoint::new(Point::new(50.0, 50.0), 2.0)]);

    let side = density::control_height();
    let clear = Point::new(spacing::XS + side * 1.5, spacing::XS + side / 2.0);
    let mut ui = sim(signature_pad::signature_pad(&signature)
        .on_event(Message::Signed)
        .direction(oito2_iced::widget::helpers::ReadingDirection::RightToLeft)
        .into());

    click(&mut ui, clear);

    assert_eq!(events(ui), [signature_pad::Event::Clear]);
}

#[test]
fn without_actions_the_corner_draws() {
    let mut signature = Signature::new();

    signature.add_stroke([StrokePoint::new(Point::new(50.0, 50.0), 2.0)]);

    let (_, clear) = buttons();
    let mut ui = sim(signature_pad::signature_pad(&signature)
        .on_event(Message::Signed)
        .show_actions(false)
        .into());

    click(&mut ui, clear);

    assert!(matches!(
        events(ui).as_slice(),
        [signature_pad::Event::Begin { .. }]
    ));
}

#[test]
fn only_the_left_button_and_the_first_pointer_draw() {
    let empty = Signature::new();
    let mut ui = sim(pad(&empty));

    press(&mut ui, Point::new(100.0, 60.0), mouse::Button::Right);
    ui.point_at(Point::new(140.0, 60.0));
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(140.0, 60.0),
    })]);
    release(&mut ui, mouse::Button::Right);

    let first = touch::Finger(1);
    let second = touch::Finger(2);

    ui.simulate([
        Event::Touch(touch::Event::FingerPressed {
            id: first,
            position: Point::new(100.0, 100.0),
        }),
        Event::Touch(touch::Event::FingerPressed {
            id: second,
            position: Point::new(100.0, 140.0),
        }),
        Event::Touch(touch::Event::FingerMoved {
            id: first,
            position: Point::new(130.0, 100.0),
        }),
        Event::Touch(touch::Event::FingerMoved {
            id: second,
            position: Point::new(130.0, 140.0),
        }),
        Event::Touch(touch::Event::FingerLifted {
            id: first,
            position: Point::new(130.0, 100.0),
        }),
        Event::Touch(touch::Event::FingerLifted {
            id: second,
            position: Point::new(130.0, 140.0),
        }),
    ]);

    let mut signature = Signature::new();

    signed(&mut signature, events(ui));

    assert_eq!(signature.strokes().len(), 1);
    assert!(
        signature.strokes()[0]
            .iter()
            .all(|point| point.position.y == 100.0)
    );
    assert_eq!(signature.strokes()[0].len(), 2);
}

#[test]
fn moves_under_a_pixel_add_no_point() {
    let empty = Signature::new();
    let mut ui = sim(pad(&empty));

    press(&mut ui, Point::new(100.0, 60.0), mouse::Button::Left);

    for x in [100.5, 100.8, 102.0] {
        let at = Point::new(x, 60.0);

        ui.point_at(at);
        ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    }

    release(&mut ui, mouse::Button::Left);

    let mut signature = Signature::new();

    signed(&mut signature, events(ui));

    let xs: Vec<f32> = signature.strokes()[0]
        .iter()
        .map(|point| point.position.x)
        .collect();

    assert_eq!(xs, [100.0, 102.0]);
}

#[test]
fn a_pad_without_on_event_only_shows() {
    let empty = Signature::new();
    let mut ui = sim(signature_pad::signature_pad(&empty).into());

    drag(&mut ui, Point::new(100.0, 80.0), (80.0, 20.0));

    assert!(events(ui).is_empty());
}

#[test]
fn a_drag_inside_a_scrollable_draws() {
    let empty = Signature::new();
    let content = scrollable(column![pad(&empty), Space::new().height(2000.0)]);
    let mut ui = sim(content.into());

    drag(&mut ui, Point::new(300.0, 90.0), (0.0, -60.0));

    let mut signature = Signature::new();

    signed(&mut signature, events(ui));

    assert_eq!(signature.strokes().len(), 1);
    assert_eq!(signature.strokes()[0].len(), 5);
}

#[test]
fn exports_drawn_strokes() {
    let empty = Signature::new();
    let mut ui = sim(pad(&empty));

    drag(&mut ui, Point::new(150.0, 90.0), (50.0, 10.0));

    let mut signature = Signature::new();

    signed(&mut signature, events(ui));

    let svg = signature.to_svg(None, Color::BLACK);

    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("width=\"400.0\" height=\"180.0\""));
    assert!(svg.contains("<path d=\"M150.0 90.0L"));

    let png = signature.to_png(Color::BLACK, 2.0).expect("png");

    assert_eq!(&png[1..4], b"PNG");
    assert_eq!(
        signature.to_image(Color::BLACK, 2.0).dimensions(),
        (800, 360)
    );
}

#[test]
fn the_cursor_is_a_crosshair_while_drawing_is_possible() {
    let empty = Signature::new();
    let _ = sim(pad(&empty));

    assert_eq!(
        support::mouse_interaction(pad(&empty), WINDOW, Point::new(100.0, 100.0)),
        mouse::Interaction::Crosshair
    );
    assert_eq!(
        support::mouse_interaction(
            signature_pad::signature_pad::<Message>(&empty),
            WINDOW,
            Point::new(100.0, 100.0)
        ),
        mouse::Interaction::None
    );
}

/// A wave across `width`, around `y`, getting thinner toward the end.
fn wave(width: f32, y: f32) -> Vec<StrokePoint> {
    (0..=40)
        .map(|i| {
            let t = i as f32 / 40.0;

            StrokePoint::new(
                Point::new(24.0 + t * (width - 48.0), y + (t * 12.0).sin() * 18.0),
                3.4 - t * 1.8,
            )
        })
        .collect()
}

fn snapshot(theme: &Theme, name: &str, signature: &Signature, localizations: &Localizations) {
    let element: Element<'_, Message> = container(
        signature_pad::signature_pad(signature)
            .height(140.0)
            .localizations(localizations)
            .on_event(Message::Signed),
    )
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|theme: &Theme| container::Style::default().background(theme.roles().surface))
    .into();
    let mut ui = Simulator::with_size(support::settings(), (420.0, 172.0), element);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn signed_scene() -> Signature {
    let mut signature = Signature::new();

    signature.add_stroke(wave(388.0, 60.0));
    signature.add_stroke([StrokePoint::new(Point::new(340.0, 96.0), 4.0)]);
    signature
}

#[test]
fn signature_pad_empty_light() {
    snapshot(
        &Theme::light(),
        "signature_pad_empty_light",
        &Signature::new(),
        &Localizations::en(),
    );
}

#[test]
fn signature_pad_empty_dark_pt_br() {
    snapshot(
        &Theme::dark(),
        "signature_pad_empty_dark_pt_br",
        &Signature::new(),
        &Localizations::pt_br(),
    );
}

#[test]
fn signature_pad_signed_light() {
    snapshot(
        &Theme::light(),
        "signature_pad_signed_light",
        &signed_scene(),
        &Localizations::en(),
    );
}

#[test]
fn signature_pad_signed_dark() {
    snapshot(
        &Theme::dark(),
        "signature_pad_signed_dark",
        &signed_scene(),
        &Localizations::en(),
    );
}
