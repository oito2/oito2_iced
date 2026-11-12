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

//! Checks the embedded Inter instances: each weight is rendered with Inter
//! instead of a system fallback font, and the same text at a heavier weight
//! covers more pixels.

mod support;

use iced::Font;
use iced::font::Weight;
use iced::widget::{column, text};
use iced_test::simulator;
use oito2_iced::{Element, Theme, font};

const WEIGHTS: [Weight; 4] = [
    Weight::Normal,
    Weight::Medium,
    Weight::Semibold,
    Weight::Bold,
];

fn view(font: Font) -> Element<'static, ()> {
    column![text("Hamburgefonstiv 0123456789").font(font).size(32)]
        .padding(10)
        .into()
}

/// RGBA pixels of the sample text rendered with `font`.
fn render(font: Font) -> Vec<u8> {
    let mut ui = simulator::Simulator::with_size(support::settings(), (640.0, 80.0), view(font));
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::pixels(&snapshot, "weight").rgba
}

/// Sum of the darkness of every pixel (dark text on a light background).
fn ink(rgba: &[u8]) -> f64 {
    rgba.as_chunks::<4>()
        .0
        .iter()
        .map(|pixel| {
            255.0 - (f64::from(pixel[0]) + f64::from(pixel[1]) + f64::from(pixel[2])) / 3.0
        })
        .sum()
}

#[test]
fn every_weight_renders_with_inter() {
    for weight in WEIGHTS {
        let inter = render(font::inter(weight));
        let fallback = render(Font {
            weight,
            ..Font::with_name("No Such Family")
        });

        assert!(inter != fallback, "{weight:?} renders with a fallback font");
    }
}

#[test]
fn heavier_weights_cover_more_pixels() {
    let inks: Vec<f64> = WEIGHTS
        .into_iter()
        .map(|weight| ink(&render(font::inter(weight))))
        .collect();

    println!("ink per weight (400, 500, 600, 700): {inks:?}");

    for pair in inks.windows(2) {
        assert!(
            pair[1] > pair[0] * 1.03,
            "weights do not get heavier: {inks:?}"
        );
    }
}
