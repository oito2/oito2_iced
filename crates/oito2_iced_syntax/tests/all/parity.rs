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

//! Parity with the highlighter of `oito2_ui`: the role colors, the
//! languages and the share of each sample that takes the same role.
//!
//! `oito2_ui` tokenizes with highlight.js grammars and this crate with
//! TextMate grammars, so token boundaries and some roles differ; the
//! colors of the roles are exact, and the agreement per language is
//! measured against a floor.

use iced::Color;
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast, SyntaxColors};
use oito2_iced::tokens::Accent;
use oito2_iced_syntax::highlighter::highlight;
use oito2_iced_syntax::language;
use oito2_iced_syntax::role::Role;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../fixtures/parity/syntax.json")).expect("valid fixture")
}

/// The field of `colors` named `name`, read without [`Role`].
fn field(colors: &SyntaxColors, name: &str) -> Option<Color> {
    Some(match name {
        "foreground" => colors.foreground,
        "comment" => colors.comment,
        "keyword" => colors.keyword,
        "string" => colors.string,
        "number" => colors.number,
        "function" => colors.function,
        "type" => colors.r#type,
        "tag" => colors.tag,
        "attribute" => colors.attribute,
        "constant" => colors.constant,
        "invalid" => colors.invalid,
        _ => return None,
    })
}

fn themes() -> Vec<Theme> {
    let mut themes = Vec::new();

    for brightness in [Brightness::Light, Brightness::Dark] {
        for accent in Accent::ALL {
            for contrast in [Contrast::Standard, Contrast::High] {
                themes.push(Theme::base(brightness, accent.color(), contrast));
            }
        }
    }

    themes
}

#[test]
fn every_scope_role_takes_the_color_of_its_dart_role_in_every_base_theme() {
    let fixture = fixture();
    let scope_roles = fixture["scopeRoles"].as_object().expect("scopeRoles");
    assert!(!scope_roles.is_empty());

    for theme in themes() {
        let colors = theme.roles().syntax;

        for (scope, role) in scope_roles {
            let name = role.as_str().expect("role name");
            let role = Role::from_name(name).expect("known role");
            assert_eq!(
                Some(role.color_in(&colors)),
                field(&colors, name),
                "{scope} in {:?}",
                theme.roles().brightness
            );
        }
    }
}

#[test]
fn every_dart_role_is_a_role() {
    let fixture = fixture();
    let scope_roles = fixture["scopeRoles"].as_object().expect("scopeRoles");

    for role in Role::ALL {
        let name = format!("{role:?}").to_lowercase();
        assert_eq!(Role::from_name(&name), Some(role));
    }

    for role in scope_roles.values() {
        assert!(
            Role::from_name(role.as_str().unwrap_or_default()).is_some(),
            "{role}"
        );
    }
}

#[test]
fn every_fixture_language_but_plain_text_has_a_grammar() {
    let fixture = fixture();
    let languages = fixture["languages"].as_array().expect("languages");
    assert_eq!(languages.len(), language::LANGUAGES.len());

    for name in languages {
        let name = name.as_str().expect("language name");
        assert!(language::LANGUAGES.contains(&name), "{name}");
        assert_eq!(language::supports(name), name != "plaintext", "{name}");
    }
}

/// The share of the non-blank characters of a sample whose role is the
/// role of the highlight.js run that holds them (plain text when the run
/// has none).
fn agreement(sample: &Value) -> (f64, usize) {
    let language = sample["language"].as_str().unwrap_or_default();
    let code = sample["code"].as_str().unwrap_or_default();
    let runs = highlight(code, language);

    let mut expected = Vec::with_capacity(code.len());

    for run in sample["runs"].as_array().into_iter().flatten() {
        let text = run["text"].as_str().unwrap_or_default();
        let role = run["role"]
            .as_str()
            .and_then(Role::from_name)
            .unwrap_or(Role::Foreground);
        expected.extend(std::iter::repeat_n(role, text.len()));
    }

    assert_eq!(
        expected.len(),
        code.len(),
        "{language}: runs cover the code"
    );

    let mut total = 0;
    let mut same = 0;

    for run in &runs {
        for (offset, character) in code[run.range.clone()].char_indices() {
            if character.is_whitespace() {
                continue;
            }

            total += 1;
            if expected[run.range.start + offset] == run.token.role {
                same += 1;
            }
        }
    }

    if total == 0 {
        return (1.0, 0);
    }

    (same as f64 / total as f64, total)
}

/// The lowest share of characters of a sample that must take the role of
/// highlight.js.
const FLOOR: f64 = 0.6;

/// The lowest share over all samples together.
const OVERALL_FLOOR: f64 = 0.8;

#[test]
fn samples_take_mostly_the_roles_of_highlight_js() {
    let fixture = fixture();
    let samples = fixture["samples"].as_array().expect("samples");
    assert!(!samples.is_empty());

    let mut weighted = 0.0;
    let mut characters = 0;
    let mut report = Vec::new();

    for sample in samples {
        let language = sample["language"].as_str().unwrap_or_default();
        let (share, total) = agreement(sample);
        report.push(format!("{language}: {:.1} %", share * 100.0));
        weighted += share * total as f64;
        characters += total;

        assert!(
            share >= FLOOR,
            "{language}: {:.1} % of the characters take the role of highlight.js",
            share * 100.0
        );
    }

    let overall = weighted / characters as f64;
    println!("agreement per language: {}", report.join(", "));
    println!("overall agreement: {:.1} %", overall * 100.0);

    assert!(
        overall >= OVERALL_FLOOR,
        "overall agreement {:.1} %",
        overall * 100.0
    );
}
