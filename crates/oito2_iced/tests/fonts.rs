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

//! Checks that the embedded Inter variable font renders distinct weights:
//! the same text at a heavier weight must cover more pixels.

mod support;

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

fn view(weight: Weight) -> Element<'static, ()> {
    column![
        text("Hamburgefonstiv 0123456789")
            .font(font::inter(weight))
            .size(32)
    ]
    .padding(10)
    .into()
}

/// Sum of the darkness of every pixel (dark text on a light background).
fn ink(weight: Weight) -> f64 {
    let mut ui = simulator::Simulator::with_size(support::settings(), (640.0, 80.0), view(weight));
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");
    let image = support::pixels(&snapshot, "weight");

    image
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|pixel| {
            255.0 - (f64::from(pixel[0]) + f64::from(pixel[1]) + f64::from(pixel[2])) / 3.0
        })
        .sum()
}

#[test]
fn heavier_weights_cover_more_pixels() {
    let inks: Vec<f64> = WEIGHTS.into_iter().map(ink).collect();

    println!("ink per weight (400, 500, 600, 700): {inks:?}");

    for pair in inks.windows(2) {
        assert!(
            pair[1] > pair[0] * 1.03,
            "weights do not get heavier: {inks:?}"
        );
    }
}
