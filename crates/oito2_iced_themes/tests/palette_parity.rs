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

//! Checks every color of the palette modules against the `palettes` section
//! of `schemes.json`, the parity fixture exported by `oito2_ui`.

use iced::Color;
use oito2_iced_themes::palettes::dracula::Dracula;
use oito2_iced_themes::palettes::everforest::Everforest;
use oito2_iced_themes::palettes::rose_pine::RosePine;
use oito2_iced_themes::palettes::{catppuccin, nord};
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/schemes.json");
    serde_json::from_str(text).expect("schemes.json is valid JSON")
}

fn hex(color: Color) -> String {
    let [r, g, b, _] = color.into_rgba8();

    format!("#{r:02X}{g:02X}{b:02X}")
}

/// Compares `colors` with the fixture object `expected` and returns how
/// many colors matched.
fn check(label: &str, expected: &Value, colors: &[(&str, Color)]) -> usize {
    let object = expected.as_object().expect("palette is an object");

    assert_eq!(object.len(), colors.len(), "{label}: color count");

    for (key, color) in colors {
        assert_eq!(
            expected[*key].as_str(),
            Some(hex(*color).as_str()),
            "{label}.{key}"
        );
    }

    colors.len()
}

#[test]
fn everforest_matches_the_fixture() {
    let f = fixture();
    let mut total = 0;

    for (key, p) in [("light", Everforest::LIGHT), ("dark", Everforest::DARK)] {
        total += check(
            key,
            &f["palettes"]["everforest"][key],
            &[
                ("bgDim", p.bg_dim),
                ("bg0", p.bg0),
                ("bg1", p.bg1),
                ("bg2", p.bg2),
                ("bg3", p.bg3),
                ("bg4", p.bg4),
                ("fg", p.fg),
                ("red", p.red),
                ("orange", p.orange),
                ("yellow", p.yellow),
                ("green", p.green),
                ("aqua", p.aqua),
                ("blue", p.blue),
                ("purple", p.purple),
                ("grey0", p.grey0),
                ("grey1", p.grey1),
                ("grey2", p.grey2),
            ],
        );
    }

    assert_eq!(total, 34);
}

#[test]
fn rose_pine_matches_the_fixture() {
    let f = fixture();
    let mut total = 0;

    for (key, p) in [
        ("main", RosePine::MAIN),
        ("moon", RosePine::MOON),
        ("dawn", RosePine::DAWN),
    ] {
        total += check(
            key,
            &f["palettes"]["rosePine"][key],
            &[
                ("base", p.base),
                ("surface", p.surface),
                ("overlay", p.overlay),
                ("muted", p.muted),
                ("subtle", p.subtle),
                ("text", p.text),
                ("love", p.love),
                ("gold", p.gold),
                ("rose", p.rose),
                ("pine", p.pine),
                ("foam", p.foam),
                ("iris", p.iris),
            ],
        );
    }

    assert_eq!(total, 36);
}

#[test]
fn dracula_matches_the_fixture() {
    let f = fixture();
    let mut total = 0;

    for (key, p) in [("dracula", Dracula::DRACULA), ("alucard", Dracula::ALUCARD)] {
        total += check(
            key,
            &f["palettes"]["dracula"][key],
            &[
                ("background", p.background),
                ("currentLine", p.current_line),
                ("foreground", p.foreground),
                ("comment", p.comment),
                ("cyan", p.cyan),
                ("green", p.green),
                ("orange", p.orange),
                ("pink", p.pink),
                ("purple", p.purple),
                ("red", p.red),
                ("yellow", p.yellow),
            ],
        );
    }

    assert_eq!(total, 22);
}

#[test]
fn catppuccin_matches_the_fixture() {
    let f = fixture();
    let mut total = 0;

    for (key, p) in [
        ("latte", catppuccin::latte()),
        ("frappe", catppuccin::frappe()),
        ("macchiato", catppuccin::macchiato()),
        ("mocha", catppuccin::mocha()),
    ] {
        total += check(
            key,
            &f["palettes"]["catppuccin"][key],
            &[
                ("rosewater", p.rosewater),
                ("flamingo", p.flamingo),
                ("pink", p.pink),
                ("mauve", p.mauve),
                ("red", p.red),
                ("maroon", p.maroon),
                ("peach", p.peach),
                ("yellow", p.yellow),
                ("green", p.green),
                ("teal", p.teal),
                ("sky", p.sky),
                ("sapphire", p.sapphire),
                ("blue", p.blue),
                ("lavender", p.lavender),
                ("text", p.text),
                ("subtext1", p.subtext1),
                ("subtext0", p.subtext0),
                ("overlay2", p.overlay2),
                ("overlay1", p.overlay1),
                ("overlay0", p.overlay0),
                ("surface2", p.surface2),
                ("surface1", p.surface1),
                ("surface0", p.surface0),
                ("base", p.base),
                ("mantle", p.mantle),
                ("crust", p.crust),
            ],
        );
    }

    assert_eq!(total, 104);
}

#[test]
fn nord_matches_the_fixture() {
    let f = fixture();
    let colors = [
        ("nord0", nord::NORD0),
        ("nord1", nord::NORD1),
        ("nord2", nord::NORD2),
        ("nord3", nord::NORD3),
        ("nord4", nord::NORD4),
        ("nord5", nord::NORD5),
        ("nord6", nord::NORD6),
        ("nord7", nord::NORD7),
        ("nord8", nord::NORD8),
        ("nord9", nord::NORD9),
        ("nord10", nord::NORD10),
        ("nord11", nord::NORD11),
        ("nord12", nord::NORD12),
        ("nord13", nord::NORD13),
        ("nord14", nord::NORD14),
        ("nord15", nord::NORD15),
    ];

    assert_eq!(check("nord", &f["palettes"]["nord"], &colors), 16);
}
