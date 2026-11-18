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

//! Tests of the image cropper.
//! Covers dragging, the mouse wheel, the keyboard, the toolbar buttons,
//! the read-only cropper, applying the events to the crop, the cursor and
//! golden snapshots of the cropper and its preview. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::{self, key::Named};
use iced::widget::{column, container, row};
use iced::{Alignment, Event, Length, Point, Vector, mouse};
use iced_test::simulator;
use oito2_iced::tokens::spacing;
use oito2_iced::{Element, Theme, density, focus};
use oito2_iced_extras::image_cropper::{self, Crop, Source};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Cropper(image_cropper::Event),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// The window: a 320 pixel tall crop area over the toolbar.
const WINDOW: (f32, f32) = (400.0, 400.0);

/// The side of the square crop box in a 400 × 320 area.
const BOX: f32 = 272.0;

/// A 400 × 200 picture: left half red, right half blue.
fn halves() -> Source {
    let image = image::RgbaImage::from_fn(400, 200, |x, _| {
        image::Rgba(if x < 200 {
            [255, 0, 0, 255]
        } else {
            [0, 0, 255, 255]
        })
    });

    Source::new(image.into())
}

/// A 480 × 320 landscape: a sky gradient, a sun and two hills.
fn landscape() -> Source {
    let image = image::RgbaImage::from_fn(480, 320, |x, y| {
        let (fx, fy) = (x as f32, y as f32);
        let sun = (fx - 340.0).hypot(fy - 110.0) < 45.0;
        let far = fy > 210.0 + (fx / 45.0).sin() * 30.0;
        let near = fy > 250.0 + (fx / 70.0).cos() * 25.0;
        let t = fy / 320.0;

        image::Rgba(if near {
            [38, 162, 105, 255]
        } else if far {
            [26, 95, 180, 255]
        } else if sun {
            [255, 245, 192, 255]
        } else {
            [
                (28.0 + t * 218.0) as u8,
                (113.0 + t * 98.0) as u8,
                (216.0 - t * 171.0) as u8,
                255,
            ]
        })
    });

    Source::new(image.into())
}

fn cropper<'a>(source: &'a Source, crop: &Crop) -> Element<'a, Message> {
    focus::navigation(image_cropper::image_cropper(source, crop).on_event(Message::Cropper)).into()
}

fn sim(element: Element<'_, Message>) -> Simulator<'_> {
    Simulator::with_size(support::settings(), WINDOW, element)
}

fn events(ui: Simulator<'_>) -> Vec<image_cropper::Event> {
    ui.into_messages()
        .map(|Message::Cropper(event)| event)
        .collect()
}

fn click(ui: &mut Simulator<'_>, at: Point) {
    ui.point_at(at);
    ui.simulate([
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);
}

/// The centers of the toolbar buttons, in order: zoom out, zoom in,
/// rotate left, rotate right, flip and reset. The toolbar is one centered
/// line: six buttons and the 140 pixel slider, `XS` apart.
fn toolbar() -> [Point; 6] {
    let side = density::control_height();
    let width = side * 6.0 + 140.0 + spacing::XS * 6.0;
    let left = (WINDOW.0 - width) / 2.0;
    let y = 320.0 + spacing::SM + side / 2.0;
    let mut x = left + side / 2.0;
    let mut centers = [Point::ORIGIN; 6];

    for (index, center) in centers.iter_mut().enumerate() {
        *center = Point::new(x, y);
        x += side + spacing::XS;

        if index == 0 {
            x += 140.0 + spacing::XS;
        }
    }

    centers
}

fn key(ui: &mut Simulator<'_>, key: keyboard::Key, modified: keyboard::Key, shift: bool) {
    let modifiers = if shift {
        keyboard::Modifiers::SHIFT
    } else {
        keyboard::Modifiers::default()
    };

    ui.simulate([Event::Keyboard(keyboard::Event::KeyPressed {
        key,
        modified_key: modified,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers,
        repeat: false,
        text: None,
    })]);
}

fn character(text: &str) -> keyboard::Key {
    keyboard::Key::Character(text.into())
}

fn close(a: Vector, b: Vector) -> bool {
    (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5
}

#[test]
fn the_toolbar_zooms_turns_flips_and_resets() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));

    for center in toolbar() {
        click(&mut ui, center);
    }

    let published = events(ui);

    assert_eq!(
        published,
        [
            image_cropper::Event::ZoomBy(1.0 / 1.2),
            image_cropper::Event::ZoomBy(1.2),
            image_cropper::Event::RotateLeft,
            image_cropper::Event::RotateRight,
            image_cropper::Event::FlipHorizontally,
            image_cropper::Event::Reset,
        ]
    );

    let mut crop = crop;

    crop.apply(image_cropper::Event::ZoomBy(1.2));
    crop.apply(image_cropper::Event::RotateRight);
    crop.apply(image_cropper::Event::FlipHorizontally);
    assert!((crop.zoom() - 1.2).abs() < 1e-6);
    assert_eq!(crop.quarter_turns(), 1);
    assert!(crop.flipped());

    crop.apply(image_cropper::Event::Pan(Vector::new(5000.0, 5000.0)));
    assert!(crop.offset().x < 5000.0);

    crop.apply(image_cropper::Event::Reset);
    assert_eq!(crop, Crop::new(source.size()));
}

#[test]
fn dragging_pans_by_parts_of_the_box() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));
    let from = Point::new(200.0, 160.0);

    ui.point_at(from);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    for step in 1..=4 {
        let at = Point::new(from.x + 5.0 * step as f32, from.y);

        ui.point_at(at);
        ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    }

    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    let total = events(ui)
        .into_iter()
        .fold(Vector::ZERO, |total, event| match event {
            image_cropper::Event::Pan(delta) => total + delta,
            _ => total,
        });

    assert!(close(total, Vector::new(20.0 / BOX, 0.0)));
}

#[test]
fn the_wheel_zooms_over_the_area() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));

    ui.point_at(Point::new(200.0, 160.0));
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Pixels { x: 0.0, y: 200.0 },
    })]);

    let published = events(ui);
    let [image_cropper::Event::ZoomBy(factor)] = published.as_slice() else {
        panic!("one zoom expected, got {published:?}");
    };

    assert!((factor - 1.0015_f32.powf(200.0)).abs() < 1e-4);
}

#[test]
fn the_keyboard_pans_zooms_turns_and_flips() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));

    ui.tap_key(Named::Tab);
    key(&mut ui, character("r"), character("r"), false);
    key(&mut ui, character("r"), character("R"), true);
    key(&mut ui, character("h"), character("h"), false);
    key(&mut ui, character("="), character("+"), true);
    key(&mut ui, character("-"), character("-"), false);
    let left = keyboard::Key::Named(Named::ArrowLeft);
    key(&mut ui, left.clone(), left, false);
    let up = keyboard::Key::Named(Named::ArrowUp);
    key(&mut ui, up.clone(), up, true);

    let published = events(ui);

    assert_eq!(
        published[..5],
        [
            image_cropper::Event::RotateRight,
            image_cropper::Event::RotateLeft,
            image_cropper::Event::FlipHorizontally,
            image_cropper::Event::ZoomBy(1.1),
            image_cropper::Event::ZoomBy(1.0 / 1.1),
        ]
    );

    let pans: Vec<Vector> = published[5..]
        .iter()
        .filter_map(|event| match event {
            image_cropper::Event::Pan(delta) => Some(*delta),
            _ => None,
        })
        .collect();

    assert_eq!(pans.len(), 2);
    assert!(close(pans[0], Vector::new(8.0 / BOX, 0.0)));
    assert!(close(pans[1], Vector::new(0.0, 32.0 / BOX)));
}

#[test]
fn keys_do_nothing_without_the_focus() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));

    key(&mut ui, character("r"), character("r"), false);

    assert!(events(ui).is_empty());
}

#[test]
fn a_cropper_without_on_event_only_shows() {
    let source = halves();
    let crop = Crop::new(source.size());
    let mut ui = sim(focus::navigation(image_cropper::image_cropper(&source, &crop)).into());

    ui.tap_key(Named::Tab);
    key(&mut ui, character("r"), character("r"), false);
    ui.point_at(Point::new(200.0, 160.0));
    ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
    })]);

    for center in toolbar() {
        click(&mut ui, center);
    }

    assert!(events(ui).is_empty());
}

#[test]
fn without_the_toolbar_only_the_area_shows() {
    let source = halves();
    let crop = Crop::new(source.size());
    let element: Element<'_, Message> = image_cropper::image_cropper(&source, &crop)
        .show_toolbar(false)
        .height(240.0)
        .on_event(Message::Cropper)
        .into();
    let mut ui = sim(element);

    for center in toolbar() {
        click(&mut ui, center);
    }

    assert!(events(ui).is_empty());
}

#[test]
fn the_crop_follows_the_events() {
    let source = halves();
    let mut crop = Crop::new(source.size());
    let mut ui = sim(cropper(&source, &crop));

    ui.tap_key(Named::Tab);
    key(&mut ui, character("h"), character("h"), false);
    key(&mut ui, character("r"), character("r"), false);

    for event in events(ui) {
        crop.apply(event);
    }

    // Flipped, blue is on the left, and a clockwise turn brings the left
    // to the top.
    let result = source.crop(&crop, Some(40));

    assert_eq!(result.dimensions(), (40, 40));
    assert_eq!(result.get_pixel(20, 5).0, [0, 0, 255, 255]);
    assert_eq!(result.get_pixel(20, 34).0, [255, 0, 0, 255]);
    assert_eq!(source.crop(&crop, None).dimensions(), (200, 200));
}

#[test]
fn the_cursor_moves_the_picture() {
    let source = halves();
    let crop = Crop::new(source.size());
    let _ = sim(cropper(&source, &crop));

    assert_eq!(
        support::mouse_interaction(cropper(&source, &crop), WINDOW, Point::new(200.0, 160.0)),
        mouse::Interaction::Move
    );
}

fn snapshot(theme: &Theme, name: &str, source: &Source, crop: &Crop) {
    let element: Element<'_, Message> = container(
        column![
            cropper(source, crop),
            row![
                image_cropper::preview(source, crop).width(120.0),
                image_cropper::preview(source, crop),
            ]
            .spacing(spacing::MD)
            .align_y(Alignment::Center),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    )
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|theme: &Theme| container::Style::default().background(theme.roles().surface))
    .into();
    let mut ui = Simulator::with_size(support::settings(), (432.0, 520.0), element);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn image_cropper_light() {
    let source = landscape();
    let crop = Crop::new(source.size()).with_aspect_ratio(4.0 / 3.0);

    snapshot(&Theme::light(), "image_cropper_light", &source, &crop);
}

#[test]
fn image_cropper_circle_dark() {
    let source = landscape();
    let mut crop = Crop::new(source.size()).with_circle(true);

    crop.set_zoom(1.5);
    crop.pan_by(Vector::new(-0.2, 0.1));
    crop.rotate_right();
    crop.flip_horizontally();

    snapshot(&Theme::dark(), "image_cropper_circle_dark", &source, &crop);
}
