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

//! Checks the color roles of every base theme (each accent, light and
//! dark, standard and high contrast) against `schemes.json`, the parity
//! fixture exported by `oito2_ui`.

use iced::Color;
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, ColorRoles, Contrast};
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

/// Every color role, with its path in the fixture.
fn roles(roles: &ColorRoles) -> Vec<(&'static str, Color)> {
    vec![
        ("colorScheme.primary", roles.primary),
        ("colorScheme.onPrimary", roles.on_primary),
        ("colorScheme.primaryContainer", roles.primary_container),
        ("colorScheme.onPrimaryContainer", roles.on_primary_container),
        ("colorScheme.primaryFixed", roles.primary_fixed),
        ("colorScheme.primaryFixedDim", roles.primary_fixed_dim),
        ("colorScheme.onPrimaryFixed", roles.on_primary_fixed),
        (
            "colorScheme.onPrimaryFixedVariant",
            roles.on_primary_fixed_variant,
        ),
        ("colorScheme.secondary", roles.secondary),
        ("colorScheme.onSecondary", roles.on_secondary),
        ("colorScheme.secondaryContainer", roles.secondary_container),
        (
            "colorScheme.onSecondaryContainer",
            roles.on_secondary_container,
        ),
        ("colorScheme.secondaryFixed", roles.secondary_fixed),
        ("colorScheme.secondaryFixedDim", roles.secondary_fixed_dim),
        ("colorScheme.onSecondaryFixed", roles.on_secondary_fixed),
        (
            "colorScheme.onSecondaryFixedVariant",
            roles.on_secondary_fixed_variant,
        ),
        ("colorScheme.tertiary", roles.tertiary),
        ("colorScheme.onTertiary", roles.on_tertiary),
        ("colorScheme.tertiaryContainer", roles.tertiary_container),
        (
            "colorScheme.onTertiaryContainer",
            roles.on_tertiary_container,
        ),
        ("colorScheme.tertiaryFixed", roles.tertiary_fixed),
        ("colorScheme.tertiaryFixedDim", roles.tertiary_fixed_dim),
        ("colorScheme.onTertiaryFixed", roles.on_tertiary_fixed),
        (
            "colorScheme.onTertiaryFixedVariant",
            roles.on_tertiary_fixed_variant,
        ),
        ("colorScheme.error", roles.error),
        ("colorScheme.onError", roles.on_error),
        ("colorScheme.errorContainer", roles.error_container),
        ("colorScheme.onErrorContainer", roles.on_error_container),
        ("colorScheme.surface", roles.surface),
        ("colorScheme.onSurface", roles.on_surface),
        ("colorScheme.surfaceDim", roles.surface_dim),
        ("colorScheme.surfaceBright", roles.surface_bright),
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
        ("colorScheme.onSurfaceVariant", roles.on_surface_variant),
        ("colorScheme.outline", roles.outline),
        ("colorScheme.outlineVariant", roles.outline_variant),
        ("colorScheme.shadow", roles.shadow),
        ("colorScheme.scrim", roles.scrim),
        ("colorScheme.inverseSurface", roles.inverse_surface),
        ("colorScheme.onInverseSurface", roles.on_inverse_surface),
        ("colorScheme.inversePrimary", roles.inverse_primary),
        ("colorScheme.surfaceTint", roles.surface_tint),
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
        ("syntax.foreground", roles.syntax.foreground),
        ("syntax.comment", roles.syntax.comment),
        ("syntax.keyword", roles.syntax.keyword),
        ("syntax.string", roles.syntax.string),
        ("syntax.number", roles.syntax.number),
        ("syntax.function", roles.syntax.function),
        ("syntax.type", roles.syntax.r#type),
        ("syntax.tag", roles.syntax.tag),
        ("syntax.attribute", roles.syntax.attribute),
        ("syntax.constant", roles.syntax.constant),
        ("syntax.invalid", roles.syntax.invalid),
    ]
}

/// Every leaf path of a fixture theme that holds a color.
fn color_paths(theme: &Value, prefix: &str, paths: &mut Vec<String>) {
    if let Some(map) = theme.as_object() {
        for (key, value) in map {
            let path = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{prefix}.{key}")
            };

            match value {
                Value::Object(_) => color_paths(value, &path, paths),
                Value::String(text) if text.starts_with('#') => paths.push(path),
                _ => {}
            }
        }
    }
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
        let brightness = match entry["brightness"].as_str() {
            Some("dark") => Brightness::Dark,
            _ => Brightness::Light,
        };
        let contrast = if entry["highContrast"].as_bool() == Some(true) {
            Contrast::High
        } else {
            Contrast::Standard
        };
        let accent = Accent::ALL
            .into_iter()
            .find(|accent| Some(accent.name()) == entry["accent"].as_str())
            .expect("known accent");

        let theme = Theme::base(brightness, accent.color(), contrast);
        let roles = theme.roles();

        assert_eq!(
            entry["theme"]["highContrast"].as_bool(),
            Some(roles.high_contrast)
        );

        for (path, color) in self::roles(roles) {
            let expected = lookup(&entry["theme"], path).unwrap_or("missing");
            compared += 1;

            if hex(color) != expected {
                mismatches.push(format!(
                    "{} {brightness:?} {contrast:?} {path}: {} != {expected}",
                    accent.name(),
                    hex(color)
                ));
            }
        }
    }

    assert_eq!(compared, 36 * 69);
    assert!(
        mismatches.is_empty(),
        "{} of {compared} roles differ:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

#[test]
fn every_fixture_color_is_compared() {
    let fixture = fixture();
    let theme = Theme::light();
    let compared: Vec<&str> = roles(theme.roles())
        .into_iter()
        .map(|(path, _)| path)
        .collect();
    let mut paths = Vec::new();

    color_paths(&fixture["base"][0]["theme"], "", &mut paths);

    let missing: Vec<&String> = paths
        .iter()
        .filter(|path| !compared.contains(&path.as_str()))
        .collect();

    assert!(missing.is_empty(), "not compared: {missing:?}");
    assert_eq!(paths.len(), compared.len());
}
