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

//! QR codes: module grids equal to the parity fixture, the size, the
//! "too much data" message and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{column, container, image, row, text};
use iced::{Color, Point};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::qr_code::Class;
use oito2_iced::{Element, Theme};
use oito2_iced_extras::qr_code::{self, ErrorLevel};

#[derive(Debug, Clone)]
enum Message {}

fn fixture() -> serde_json::Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/parity/qr.json");
    let text = std::fs::read_to_string(path).expect("read the QR fixture");
    serde_json::from_str(&text).expect("parse the QR fixture")
}

fn level(name: &str) -> ErrorLevel {
    ErrorLevel::ALL
        .into_iter()
        .find(|level| level.name() == name)
        .expect("a known error level")
}

#[test]
fn every_fixture_code_has_the_same_modules() {
    let fixture = fixture();
    let codes = fixture["codes"].as_array().expect("codes");
    assert_eq!(codes.len(), 24);

    let mut differing = Vec::new();
    for code in codes {
        let data = code["data"].as_str().expect("data");
        let level_name = code["errorLevel"].as_str().expect("level");
        let expected: Vec<&str> = code["rows"]
            .as_array()
            .expect("rows")
            .iter()
            .map(|row| row.as_str().expect("row"))
            .collect();

        let modules = qr_code::modules(data, level(level_name)).expect("the data fits");
        let actual: Vec<String> = modules
            .rows()
            .map(|row| {
                row.iter()
                    .map(|&dark| if dark { '1' } else { '0' })
                    .collect()
            })
            .collect();

        assert_eq!(
            modules.count() as u64,
            code["moduleCount"].as_u64().expect("module count"),
            "module count of {data:?} at {level_name}"
        );
        if actual != expected {
            differing.push(format!("{data:?} at {level_name}"));
        }
    }
    assert!(differing.is_empty(), "differing codes: {differing:?}");
}

#[test]
fn data_that_does_not_fit_is_an_error_as_in_the_fixture() {
    let too_long = fixture()["tooLong"].as_bool().expect("tooLong");
    assert_eq!(
        qr_code::modules(&"x".repeat(3000), ErrorLevel::High).is_err(),
        too_long
    );
}

#[test]
fn the_code_is_the_size_and_data_too_long_shows_the_message() {
    let localizations = Localizations::en();
    let view: Element<'_, Message> = column![
        container(qr_code::qr_code("oito2")).id("code"),
        container(qr_code::qr_code("x".repeat(8000)).localizations(&localizations)).id("long"),
    ]
    .into();
    let mut ui = simulator::Simulator::with_settings(support::settings(), view);

    let code = ui.find(selector::id("code")).expect("code").bounds();
    assert_eq!((code.width, code.height), (160.0, 160.0));
    let long = ui.find(selector::id("long")).expect("long").bounds();
    assert_eq!((long.width, long.height), (160.0, 160.0));
    assert!(ui.find(localizations.qr_too_long).is_ok());
}

#[test]
fn the_error_element_replaces_the_message_and_portuguese_is_localized() {
    let view: Element<'_, Message> = column![
        qr_code::qr_code("x".repeat(8000)).error(text("Too long to encode")),
        qr_code::qr_code("x".repeat(8000)).localizations(&Localizations::pt_br()),
    ]
    .into();
    let mut ui = simulator::Simulator::with_settings(support::settings(), view);

    assert!(ui.find("Too long to encode").is_ok());
    assert!(ui.find(Localizations::en().qr_too_long).is_err());
    assert!(ui.find(Localizations::pt_br().qr_too_long).is_ok());
}

#[test]
fn a_center_image_clears_the_middle_modules() {
    let data = "https://oito2.github.io/oito2_ui/";
    let view = |with_image: bool| -> Element<'static, Message> {
        let code = qr_code::qr_code(data).error_level(ErrorLevel::High);
        if with_image {
            code.center_image(logo()).into()
        } else {
            code.into()
        }
    };

    let mut plain =
        simulator::Simulator::with_size(support::settings(), (160.0, 160.0), view(false));
    let plain = support::pixels(&plain.snapshot(&Theme::light()).expect("snapshot"), "plain");
    let mut logo = simulator::Simulator::with_size(support::settings(), (160.0, 160.0), view(true));
    let logo = support::pixels(&logo.snapshot(&Theme::light()).expect("snapshot"), "logo");

    // The quiet zone stays white and the corners equal; the middle changes.
    let corner = Point::new(2.0, 2.0);
    assert_eq!(support::pixel_at(&plain, corner, 160.0), Color::WHITE);
    assert_eq!(
        support::pixel_at(&plain, Point::new(20.0, 20.0), 160.0),
        support::pixel_at(&logo, Point::new(20.0, 20.0), 160.0)
    );
    assert!(support::difference(&plain, &logo) > 0.0);
}

/// A 24 by 24 blue disc.
fn logo() -> image::Handle {
    let side = 24u32;
    let mut rgba = Vec::with_capacity((side * side * 4) as usize);
    for y in 0..side {
        for x in 0..side {
            let (dx, dy) = (x as f32 - 11.5, y as f32 - 11.5);
            let inside = dx * dx + dy * dy <= 144.0;
            rgba.extend_from_slice(if inside {
                &[0x35, 0x84, 0xE4, 0xFF]
            } else {
                &[0, 0, 0, 0]
            });
        }
    }
    image::Handle::from_rgba(side, side, rgba)
}

fn catalog() -> Element<'static, Message> {
    let data = "https://oito2.github.io/oito2_ui/";
    column![
        row![
            qr_code::qr_code(data).size(120.0),
            qr_code::qr_code(data).size(120.0).rounded_modules(true),
            qr_code::qr_code(data).size(120.0).class(Class::Themed),
        ]
        .spacing(16),
        row![
            qr_code::qr_code(data)
                .size(120.0)
                .color(Color::from_rgb8(0x1A, 0x3A, 0x6B))
                .background_color(Color::from_rgb8(0xF1, 0xF5, 0xFB)),
            qr_code::qr_code(data)
                .size(120.0)
                .rounded_modules(true)
                .center_image(logo()),
            qr_code::qr_code("x".repeat(8000)).size(120.0),
        ]
        .spacing(16),
    ]
    .spacing(16)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (424.0, 288.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn qr_code_light() {
    snapshot(&Theme::light(), "qr_code_light");
}

#[test]
fn qr_code_dark() {
    snapshot(&Theme::dark(), "qr_code_dark");
}
