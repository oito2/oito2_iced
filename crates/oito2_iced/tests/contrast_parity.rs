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

//! Checks the contrast functions against the `contrast` section of
//! `foundation.json`, the parity fixture exported by `oito2_ui`.

use iced::Color;
use oito2_iced::theme::contrast;
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/foundation.json");
    let value: Value = serde_json::from_str(text).expect("foundation.json is valid JSON");

    value["contrast"].clone()
}

fn parse(hex: &Value) -> Color {
    let text = hex.as_str().expect("a hex color");
    let value = u32::from_str_radix(text.trim_start_matches('#'), 16).expect("hex digits");
    let [_, r, g, b] = value.to_be_bytes();

    Color::from_rgb8(r, g, b)
}

fn hex(color: Color) -> String {
    let [r, g, b, _] = color.into_rgba8();

    format!("#{r:02X}{g:02X}{b:02X}")
}

fn cases<'a>(fixture: &'a Value, name: &str) -> &'a Vec<Value> {
    fixture[name].as_array().expect("an array of cases")
}

#[test]
fn ratio_matches_the_fixture() {
    let fixture = fixture();
    let cases = cases(&fixture, "ratio");

    for case in cases {
        let expected = case["ratio"].as_f64().expect("a ratio");
        let actual = f64::from(contrast::ratio(parse(&case["a"]), parse(&case["b"])));

        assert!(
            (actual - expected).abs() <= expected * 1e-5,
            "ratio({}, {}) = {actual}, expected {expected}",
            case["a"],
            case["b"]
        );
    }

    assert_eq!(cases.len(), 432);
}

#[test]
fn foreground_for_matches_the_fixture() {
    let fixture = fixture();
    let cases = cases(&fixture, "foregroundFor");

    for case in cases {
        let actual = hex(contrast::foreground_for(parse(&case["background"])));

        assert_eq!(
            Some(actual.as_str()),
            case["foreground"].as_str(),
            "{}",
            case["background"]
        );
    }

    assert_eq!(cases.len(), 36);
}

#[test]
fn ensure_matches_the_fixture() {
    let fixture = fixture();
    let cases = cases(&fixture, "ensure");

    for case in cases {
        let backgrounds: Vec<Color> = case["backgrounds"]
            .as_array()
            .expect("backgrounds")
            .iter()
            .map(parse)
            .collect();
        let target = case["target"].as_f64().expect("a target") as f32;
        let actual = hex(contrast::ensure(
            parse(&case["color"]),
            &backgrounds,
            target,
        ));

        assert_eq!(
            Some(actual.as_str()),
            case["result"].as_str(),
            "ensure({}, {:?}, {target})",
            case["color"],
            case["backgrounds"]
        );
    }

    assert_eq!(cases.len(), 126);
}
