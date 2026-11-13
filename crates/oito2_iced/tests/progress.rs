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

//! Progress indicators: sizes, the indeterminate animation driven by the
//! redraw events, and golden snapshots of the determinate catalog. Run
//! with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use std::time::Duration;

use oito2_iced_test_utils as support;

use iced::time::Instant;
use iced::widget::{column, container, row, text};
use iced::{Color, Event, Point, window};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::progress::{self, Class};
use oito2_iced::{Element, Theme};

type Ui<'a> = simulator::Simulator<'a, (), Theme>;

fn bounds(ui: &mut Ui<'_>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn sized() -> Element<'static, ()> {
    column![
        container(progress::linear(0.25)).width(300).id("linear"),
        container(progress::linear_indeterminate())
            .width(300)
            .id("indeterminate"),
        container(progress::circular(0.42)).id("circular"),
        container(progress::circular_indeterminate().size(48.0)).id("large"),
    ]
    .into()
}

#[test]
fn indicators_keep_their_size_at_every_density() {
    for density in [Density::Pointer, Density::Touch] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), sized());
        let linear = bounds(&mut ui, "linear");
        let indeterminate = bounds(&mut ui, "indeterminate");
        let circular = bounds(&mut ui, "circular");
        let large = bounds(&mut ui, "large");

        assert_eq!((linear.width, linear.height), (300.0, 6.0), "{density:?}");
        assert_eq!(
            (indeterminate.width, indeterminate.height),
            (300.0, 6.0),
            "{density:?}"
        );
        assert_eq!(
            (circular.width, circular.height),
            (32.0, 32.0),
            "{density:?}"
        );
        assert_eq!((large.width, large.height), (48.0, 48.0), "{density:?}");
    }

    density::set(Density::Pointer);
}

fn close(a: Color, b: Color) -> bool {
    (a.r - b.r).abs() < 0.03 && (a.g - b.g).abs() < 0.03 && (a.b - b.b).abs() < 0.03
}

/// The colors at the start and at the middle of a 200-pixel indeterminate
/// bar, after its first frame came `elapsed` before the snapshot.
fn indeterminate_pixels(animated: bool, elapsed: Duration) -> (Color, Color) {
    let view: Element<'static, ()> = container(
        progress::linear_indeterminate()
            .animated(animated)
            .height(20.0),
    )
    .width(200)
    .into();

    let mut ui = simulator::Simulator::with_size(support::settings(), (200.0, 20.0), view);

    let first = Instant::now()
        .checked_sub(elapsed)
        .expect("an instant in the past");
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(first))]);

    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");
    let image = support::pixels(&snapshot, "indeterminate");

    (
        support::pixel_at(&image, Point::new(12.0, 10.0), 200.0),
        support::pixel_at(&image, Point::new(100.0, 10.0), 200.0),
    )
}

#[test]
fn indeterminate_progress_moves_with_the_redraw_time() {
    let theme = Theme::light();
    let bar = theme.roles().selected_fill();
    let track = theme.roles().surface_container_highest;

    let (start, middle) = indeterminate_pixels(true, Duration::ZERO);
    assert!(close(start, bar), "{start:?}");
    assert!(close(middle, track), "{middle:?}");

    // Half of the way forth: the eased bar is centered on the track.
    let (start, middle) = indeterminate_pixels(true, progress::LINEAR_PERIOD / 2);
    assert!(close(start, track), "{start:?}");
    assert!(close(middle, bar), "{middle:?}");
}

#[test]
fn still_indeterminate_progress_stays_at_the_start() {
    let theme = Theme::light();
    let bar = theme.roles().selected_fill();

    let (start, _) = indeterminate_pixels(false, progress::LINEAR_PERIOD / 2);
    assert!(close(start, bar), "{start:?}");
}

fn catalog() -> Element<'static, ()> {
    let linear = |label: &'static str, bar: progress::Linear<'static>| {
        row![text(label).width(90), bar]
            .spacing(8)
            .align_y(iced::Alignment::Center)
    };

    column![
        linear("0 %", progress::linear(0.0)),
        linear("1 %", progress::linear(0.01)),
        linear("25 %", progress::linear(0.25)),
        linear("60 %", progress::linear(0.6)),
        linear("100 %", progress::linear(1.0)),
        linear(
            "RTL 25 %",
            progress::linear(0.25).direction(ReadingDirection::RightToLeft)
        ),
        linear("Success", progress::linear(0.7).class(Class::Success)),
        linear("Warning", progress::linear(0.5).class(Class::Warning)),
        linear("Error", progress::linear(0.3).class(Class::Error)),
        linear("Tall", progress::linear(0.45).height(12.0)),
        row![
            progress::circular(0.0),
            progress::circular(0.01),
            progress::circular(0.25),
            progress::circular(0.6),
            progress::circular(1.0),
            progress::circular(0.7).class(Class::Success),
            progress::circular(0.5).class(Class::Warning),
            progress::circular(0.3).class(Class::Error),
            progress::circular(0.66).size(48.0).stroke_width(6.0),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(10)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (480.0, 400.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn progress_light() {
    snapshot(&Theme::light(), "progress_light");
}

#[test]
fn progress_dark() {
    snapshot(&Theme::dark(), "progress_dark");
}

#[test]
fn progress_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "progress_light_high_contrast");
}
