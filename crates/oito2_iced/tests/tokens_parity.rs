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

//! Checks the design tokens against `tokens.json`, the parity fixture
//! exported by `oito2_ui`.

use iced::Color;
use iced::font::Weight;
use oito2_iced::tokens::{Accent, Neutrals, Status, radius, sizes, spacing, typography};
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/tokens.json");
    serde_json::from_str(text).expect("tokens.json is valid JSON")
}

fn hex(color: Color) -> String {
    let [r, g, b, _] = color.into_rgba8();

    format!("#{r:02X}{g:02X}{b:02X}")
}

fn number(value: &Value) -> f32 {
    value.as_f64().expect("a number") as f32
}

fn assert_section(fixture: &Value, section: &str, values: &[(&str, f32)]) {
    let map = fixture[section].as_object().expect("an object");

    assert_eq!(map.len(), values.len(), "{section}: key count");

    for (key, value) in values {
        assert_eq!(number(&fixture[section][key]), *value, "{section}.{key}");
    }
}

fn weight(value: &Value) -> Weight {
    match value.as_u64() {
        Some(500) => Weight::Medium,
        Some(600) => Weight::Semibold,
        Some(700) => Weight::Bold,
        _ => Weight::Normal,
    }
}

#[test]
fn geometry_matches_the_fixture() {
    let fixture = fixture();

    assert_section(
        &fixture,
        "radius",
        &[
            ("check", radius::CHECK),
            ("small", radius::SMALL),
            ("medium", radius::MEDIUM),
            ("large", radius::LARGE),
            ("window", radius::WINDOW),
        ],
    );
    assert_section(
        &fixture,
        "spacing",
        &[
            ("xxs", spacing::XXS),
            ("xs", spacing::XS),
            ("sm", spacing::SM),
            ("md", spacing::MD),
            ("lg", spacing::LG),
            ("page", spacing::PAGE),
            ("xl", spacing::XL),
            ("xxl", spacing::XXL),
            ("xxxl", spacing::XXXL),
        ],
    );
    assert_section(
        &fixture,
        "sizes",
        &[
            ("buttonHeight", sizes::BUTTON_HEIGHT),
            ("touchButtonHeight", sizes::TOUCH_BUTTON_HEIGHT),
            ("titleBarHeight", sizes::TITLE_BAR_HEIGHT),
            ("titleBarItemHeight", sizes::TITLE_BAR_ITEM_HEIGHT),
            ("iconSize", sizes::ICON_SIZE),
            ("progressHeight", sizes::PROGRESS_HEIGHT),
            ("focusBorderWidth", sizes::FOCUS_BORDER_WIDTH),
            ("windowControlSize", sizes::WINDOW_CONTROL_SIZE),
            (
                "windowsWindowControlWidth",
                sizes::WINDOWS_WINDOW_CONTROL_WIDTH,
            ),
            ("minPointerTarget", sizes::MIN_POINTER_TARGET),
            ("minTouchTarget", sizes::MIN_TOUCH_TARGET),
        ],
    );
}

#[test]
fn colors_match_the_fixture() {
    let fixture = fixture();

    assert_eq!(
        fixture["accents"].as_object().map(|map| map.len()),
        Some(Accent::ALL.len())
    );

    for accent in Accent::ALL {
        assert_eq!(
            fixture["accents"][accent.name()].as_str(),
            Some(hex(accent.color()).as_str()),
            "accent {}",
            accent.name()
        );
    }

    for (name, neutrals, status) in [
        ("light", Neutrals::LIGHT, Status::LIGHT),
        ("dark", Neutrals::DARK, Status::DARK),
    ] {
        let expected_neutrals = [
            ("surface", neutrals.surface),
            ("surfaceContainerLowest", neutrals.surface_container_lowest),
            ("surfaceContainerLow", neutrals.surface_container_low),
            ("surfaceContainer", neutrals.surface_container),
            ("surfaceContainerHigh", neutrals.surface_container_high),
            (
                "surfaceContainerHighest",
                neutrals.surface_container_highest,
            ),
            ("titleBar", neutrals.title_bar),
            ("onSurface", neutrals.on_surface),
            ("onSurfaceVariant", neutrals.on_surface_variant),
            ("outline", neutrals.outline),
            ("outlineVariant", neutrals.outline_variant),
        ];
        let expected_status = [
            ("success", status.success),
            ("warning", status.warning),
            ("error", status.error),
            ("info", status.info),
            ("link", status.link),
        ];

        assert_eq!(
            fixture["neutrals"][name].as_object().map(|map| map.len()),
            Some(expected_neutrals.len())
        );
        assert_eq!(
            fixture["status"][name].as_object().map(|map| map.len()),
            Some(expected_status.len())
        );

        for (key, color) in expected_neutrals {
            assert_eq!(
                fixture["neutrals"][name][key].as_str(),
                Some(hex(color).as_str()),
                "neutrals.{name}.{key}"
            );
        }

        for (key, color) in expected_status {
            assert_eq!(
                fixture["status"][name][key].as_str(),
                Some(hex(color).as_str()),
                "status.{name}.{key}"
            );
        }
    }
}

#[test]
fn typography_matches_the_fixture() {
    let fixture = fixture();
    let typography_fixture = &fixture["typography"];

    assert_eq!(
        number(&typography_fixture["bodyLargeSize"]),
        typography::BODY_LARGE_SIZE
    );
    assert_eq!(
        number(&typography_fixture["bodyMediumSize"]),
        typography::BODY_MEDIUM_SIZE
    );
    assert_eq!(
        number(&typography_fixture["bodySmallSize"]),
        typography::BODY_SMALL_SIZE
    );
    assert_eq!(
        weight(&typography_fixture["emphasisWeight"]),
        typography::EMPHASIS_WEIGHT
    );

    let fallback: Vec<&str> = typography_fixture["fontFamilyFallback"]
        .as_array()
        .expect("an array")
        .iter()
        .filter_map(Value::as_str)
        .collect();

    assert_eq!(fallback, typography::FONT_FAMILY_FALLBACK);

    let scale = typography_fixture["scale"].as_object().expect("an object");

    assert_eq!(scale.len(), typography::SCALE.len());

    for (name, style) in typography::SCALE {
        let expected = &scale[name];

        assert_eq!(number(&expected["fontSize"]), style.size, "{name} size");
        assert!(
            (number(&expected["lineHeight"]) - style.line_height).abs() < 1e-3,
            "{name} line height"
        );
        assert_eq!(
            weight(&expected["fontWeight"]),
            style.weight,
            "{name} weight"
        );
        assert_eq!(
            number(&expected["letterSpacing"]),
            style.letter_spacing,
            "{name} letter spacing"
        );
    }
}
