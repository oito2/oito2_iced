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

//! Checks the color roles of the base themes against `schemes.json`, the
//! parity fixture exported by `oito2_ui`.

use iced::Color;
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, ColorRoles};
use oito2_iced::tokens::Accent;
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/schemes.json");
    serde_json::from_str(text).expect("schemes.json is valid JSON")
}

fn hex(color: Color) -> String {
    let [r, g, b, a] = color.into_rgba8();

    if a == 255 {
        format!("#{r:02X}{g:02X}{b:02X}")
    } else {
        format!("#{r:02X}{g:02X}{b:02X}{a:02X}")
    }
}

/// The roles compared, with their fixture path.
fn roles(roles: &ColorRoles) -> Vec<(&'static str, Color)> {
    vec![
        ("colorScheme.primary", roles.primary),
        ("colorScheme.onPrimary", roles.on_primary),
        ("colorScheme.primaryContainer", roles.primary_container),
        ("colorScheme.onPrimaryContainer", roles.on_primary_container),
        ("colorScheme.secondary", roles.secondary),
        ("colorScheme.onSecondary", roles.on_secondary),
        ("colorScheme.secondaryContainer", roles.secondary_container),
        (
            "colorScheme.onSecondaryContainer",
            roles.on_secondary_container,
        ),
        ("colorScheme.tertiary", roles.tertiary),
        ("colorScheme.onTertiary", roles.on_tertiary),
        ("colorScheme.tertiaryContainer", roles.tertiary_container),
        (
            "colorScheme.onTertiaryContainer",
            roles.on_tertiary_container,
        ),
        ("colorScheme.error", roles.error),
        ("colorScheme.onError", roles.on_error),
        ("colorScheme.errorContainer", roles.error_container),
        ("colorScheme.onErrorContainer", roles.on_error_container),
        ("colorScheme.surface", roles.surface),
        ("colorScheme.onSurface", roles.on_surface),
        ("colorScheme.onSurfaceVariant", roles.on_surface_variant),
        (
            "colorScheme.surfaceContainerLowest",
            roles.surface_container_lowest,
        ),
        (
            "colorScheme.surfaceContainerLow",
            roles.surface_container_low,
        ),
        ("colorScheme.surfaceContainer", roles.surface_container),
        (
            "colorScheme.surfaceContainerHigh",
            roles.surface_container_high,
        ),
        (
            "colorScheme.surfaceContainerHighest",
            roles.surface_container_highest,
        ),
        ("colorScheme.outline", roles.outline),
        ("colorScheme.outlineVariant", roles.outline_variant),
        ("colorScheme.inverseSurface", roles.inverse_surface),
        ("colorScheme.onInverseSurface", roles.on_inverse_surface),
        ("colorScheme.inversePrimary", roles.inverse_primary),
        ("background", roles.background),
        ("chrome.background", roles.chrome.background),
        ("chrome.sidebar", roles.chrome.sidebar),
        ("chrome.mainbar", roles.chrome.mainbar),
        ("chrome.titleBar", roles.chrome.title_bar),
        ("chrome.dialog", roles.chrome.dialog),
        ("semantic.primaryText", roles.semantic.primary_text),
        ("semantic.errorText", roles.semantic.error_text),
        ("semantic.success", roles.semantic.success),
        ("semantic.warning", roles.semantic.warning),
        ("semantic.info", roles.semantic.info),
        ("semantic.link", roles.semantic.link),
    ]
}

fn lookup<'a>(theme: &'a Value, path: &str) -> Option<&'a str> {
    path.split('.')
        .fold(theme, |value, key| &value[key])
        .as_str()
}

#[test]
fn base_themes_match_the_fixture() {
    let fixture = fixture();
    let entries = fixture["base"].as_array().expect("base is an array");
    let mut mismatches = Vec::new();
    let mut compared = 0;

    for entry in entries {
        if entry["highContrast"].as_bool() == Some(true) {
            continue;
        }

        let brightness = match entry["brightness"].as_str() {
            Some("dark") => Brightness::Dark,
            _ => Brightness::Light,
        };
        let accent = Accent::ALL
            .into_iter()
            .find(|accent| Some(accent.name()) == entry["accent"].as_str())
            .expect("known accent");

        let theme = Theme::new(brightness, accent);

        for (path, color) in roles(theme.roles()) {
            let expected = lookup(&entry["theme"], path).unwrap_or("missing");
            compared += 1;

            if hex(color) != expected {
                mismatches.push(format!(
                    "{} {:?} {path}: {} != {expected}",
                    accent.name(),
                    brightness,
                    hex(color)
                ));
            }
        }
    }

    assert_eq!(compared, 18 * 41);
    assert!(
        mismatches.is_empty(),
        "{} of {compared} roles differ:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}
