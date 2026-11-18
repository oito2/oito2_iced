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

//! Behaviour and golden tests of the animated icons widget.
//!
//! Animated icons: the progress of each mode over time, the redraws they
//! request, replays on new data or mode, the size and color, and golden
//! snapshots of every icon at its final frame and midway. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::cell::RefCell;
use std::time::Duration;

use iced::advanced::clipboard;
use iced::advanced::renderer::{Headless, Style};
use iced::advanced::widget::operation::{Operation, Outcome, black_box};
use iced::time::Instant;
use iced::widget::{column, container, row, text};
use iced::{Color, Event, Point, Size, mouse, window};
use iced_test::runtime::user_interface::{self, Cache, UserInterface};
use iced_test::selector::{self, Selector};
use iced_test::simulator;
use oito2_iced::density::{self, Density};
use oito2_iced::icon::animated::{self, Canvas, Data, Mode};
use oito2_iced::{Element, Theme};

type Ui<'a> = UserInterface<'a, (), Theme, iced::Renderer>;

thread_local! {
    /// The frames the recording painters drew on this thread: the painter
    /// and the progress.
    static FRAMES: RefCell<Vec<(char, f32)>> = const { RefCell::new(Vec::new()) };
}

fn record_a(_canvas: &mut Canvas<'_>, progress: f32, _color: Color) {
    FRAMES.with(|frames| frames.borrow_mut().push(('a', progress)));
}

fn record_b(_canvas: &mut Canvas<'_>, progress: f32, _color: Color) {
    FRAMES.with(|frames| frames.borrow_mut().push(('b', progress)));
}

const RECORD_A: Data = Data::new(record_a).with_duration(Duration::from_millis(500));
const RECORD_B: Data = Data::new(record_b).with_duration(Duration::from_millis(500));

/// The last frame drawn on this thread.
fn last_frame() -> Option<(char, f32)> {
    FRAMES.with(|frames| frames.borrow().last().copied())
}

fn renderer() -> iced::Renderer {
    let backend = std::env::var("ICED_TEST_BACKEND").ok();

    iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
        oito2_iced::font::INTER,
        iced::Pixels(16.0),
        backend.as_deref(),
    ))
    .expect("headless renderer")
}

fn build<'a>(
    element: impl Into<Element<'a, ()>>,
    cache: Cache,
    renderer: &mut iced::Renderer,
) -> Ui<'a> {
    UserInterface::build(element, Size::new(100.0, 100.0), cache, renderer)
}

/// Sends a frame at `at`, draws it, and returns the redraw the interface
/// requests.
fn frame(ui: &mut Ui<'_>, renderer: &mut iced::Renderer, at: Instant) -> window::RedrawRequest {
    let mut messages = Vec::new();
    let (state, _) = ui.update(
        &[Event::Window(window::Event::RedrawRequested(at))],
        mouse::Cursor::Unavailable,
        renderer,
        &mut clipboard::Null,
        &mut messages,
    );

    ui.draw(
        renderer,
        &Theme::light(),
        &Style {
            text_color: Color::BLACK,
        },
        mouse::Cursor::Unavailable,
    );

    match state {
        user_interface::State::Updated { redraw_request, .. } => redraw_request,
        user_interface::State::Outdated => window::RedrawRequest::NextFrame,
    }
}

fn assert_progress(painter: char, expected: f32) {
    let frame = last_frame();

    assert!(frame.is_some(), "no frame was drawn");

    let (drawn, progress) = frame.unwrap_or_default();

    assert_eq!(drawn, painter);
    assert!(
        (progress - expected).abs() < 0.01,
        "progress {progress}, expected {expected}"
    );
}

#[test]
fn once_plays_to_the_end_and_stops() {
    let mut renderer = renderer();
    let mut ui = build(animated::icon(RECORD_A), Cache::default(), &mut renderer);
    let start = Instant::now();

    assert_eq!(
        frame(&mut ui, &mut renderer, start),
        window::RedrawRequest::NextFrame
    );
    assert_progress('a', 0.0);

    frame(&mut ui, &mut renderer, start + Duration::from_millis(250));
    assert_progress('a', 0.5);

    assert_eq!(
        frame(&mut ui, &mut renderer, start + Duration::from_millis(600)),
        window::RedrawRequest::Wait
    );
    assert_progress('a', 1.0);

    frame(&mut ui, &mut renderer, start + Duration::from_millis(900));
    assert_progress('a', 1.0);
}

#[test]
fn repeat_loops() {
    let mut renderer = renderer();
    let mut ui = build(
        animated::icon(RECORD_A).mode(Mode::Repeat),
        Cache::default(),
        &mut renderer,
    );
    let start = Instant::now();

    frame(&mut ui, &mut renderer, start);
    assert_eq!(
        frame(&mut ui, &mut renderer, start + Duration::from_millis(600)),
        window::RedrawRequest::NextFrame
    );
    assert_progress('a', 0.2);
}

#[test]
fn still_and_reduced_motion_show_the_final_frame_without_redraws() {
    let mut renderer = renderer();

    for icon in [
        animated::icon(RECORD_A).mode(Mode::Still),
        animated::icon(RECORD_A).mode(Mode::Repeat).animated(false),
        animated::icon(RECORD_A).animated(false),
    ] {
        let mut ui = build(icon, Cache::default(), &mut renderer);

        assert_eq!(
            frame(&mut ui, &mut renderer, Instant::now()),
            window::RedrawRequest::Wait
        );
        assert_progress('a', 1.0);
    }
}

#[test]
fn the_duration_overrides_the_one_of_the_data() {
    let mut renderer = renderer();
    let mut ui = build(
        animated::icon(RECORD_A).duration(Duration::from_secs(1)),
        Cache::default(),
        &mut renderer,
    );
    let start = Instant::now();

    frame(&mut ui, &mut renderer, start);
    frame(&mut ui, &mut renderer, start + Duration::from_millis(500));
    assert_progress('a', 0.5);
}

/// Plays `first` to its end, rebuilds the interface with `second`, and
/// returns the progress of the next frame.
fn replay(first: animated::AnimatedIcon, second: animated::AnimatedIcon) -> Option<(char, f32)> {
    let mut renderer = renderer();
    let mut ui = build(first, Cache::default(), &mut renderer);
    let start = Instant::now();

    frame(&mut ui, &mut renderer, start);
    frame(&mut ui, &mut renderer, start + Duration::from_secs(1));

    let mut ui = build(second, ui.into_cache(), &mut renderer);

    frame(&mut ui, &mut renderer, start + Duration::from_secs(2));
    last_frame()
}

#[test]
fn new_data_or_mode_plays_again() {
    assert_eq!(
        replay(animated::icon(RECORD_A), animated::icon(RECORD_B)),
        Some(('b', 0.0))
    );
    assert_eq!(
        replay(
            animated::icon(RECORD_A).mode(Mode::Still),
            animated::icon(RECORD_A)
        ),
        Some(('a', 0.0))
    );
}

#[test]
fn the_same_data_does_not_play_again() {
    assert_eq!(
        replay(animated::icon(RECORD_A), animated::icon(RECORD_A)),
        Some(('a', 1.0))
    );
}

#[test]
fn the_size_follows_the_density_unless_set() {
    let mut renderer = renderer();
    let size_of = |icon: animated::AnimatedIcon, renderer: &mut iced::Renderer| {
        let mut ui = build(container(icon).id("icon"), Cache::default(), renderer);
        let mut find = selector::id("icon").find();

        ui.operate(renderer, &mut black_box(&mut find));

        match find.finish() {
            Outcome::Some(Some(target)) => Some(target.bounds().size()),
            _ => None,
        }
    };

    for (density, side) in [(Density::Pointer, 20.0), (Density::Touch, 24.0)] {
        density::set(density);
        assert_eq!(
            size_of(animated::icon(animated::OK), &mut renderer),
            Some(Size::new(side, side))
        );
    }

    density::set(Density::Pointer);
    assert_eq!(
        size_of(animated::icon(animated::OK).size(40.0), &mut renderer),
        Some(Size::new(40.0, 40.0))
    );
}

#[test]
fn the_color_follows_the_parent_text_unless_set() {
    let red = Color::from_rgb(1.0, 0.0, 0.0);
    let blue = Color::from_rgb(0.0, 0.0, 1.0);
    let view = || -> Element<'static, ()> {
        row![
            container(
                animated::icon(animated::COMPASS_FILLED)
                    .mode(Mode::Still)
                    .size(48.0)
            )
            .style(move |_: &Theme| container::Style {
                text_color: Some(red),
                ..container::Style::default()
            }),
            animated::icon(animated::COMPASS_FILLED)
                .mode(Mode::Still)
                .size(48.0)
                .color(blue),
        ]
        .into()
    };
    let mut ui = simulator::Simulator::with_size(support::settings(), (96.0, 48.0), view());
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");
    let image = support::pixels(&snapshot, "animated_icon_color");

    // A point of the filled circle beside the needle, and the needle hole.
    let filled = support::pixel_at(&image, Point::new(10.0, 24.0), 96.0);
    let other = support::pixel_at(&image, Point::new(58.0, 24.0), 96.0);
    let hole = support::pixel_at(&image, Point::new(24.0, 24.0), 96.0);

    assert!(
        filled.r > 0.9 && filled.g < 0.1 && filled.b < 0.1,
        "{filled:?}"
    );
    assert!(other.b > 0.9 && other.r < 0.1 && other.g < 0.1, "{other:?}");
    assert!(
        hole.r > 0.9 && hole.g > 0.9,
        "the needle is cut out: {hole:?}"
    );
}

/// Every icon with its name.
const ICONS: [(&str, Data); 10] = [
    ("ok", animated::OK),
    ("ok_filled", animated::OK_FILLED),
    ("heart", animated::HEART),
    ("heart_filled", animated::HEART_FILLED),
    ("star", animated::STAR),
    ("star_filled", animated::STAR_FILLED),
    ("star_half_filled", animated::STAR_HALF_FILLED),
    ("compass", animated::COMPASS),
    ("compass_filled", animated::COMPASS_FILLED),
    ("no_network", animated::NO_NETWORK),
];

/// Every icon frozen at `progress`, by painters that ignore the time.
macro_rules! frozen {
    ($progress:expr) => {
        [
            Data::new(|c, _, color| (animated::OK.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::OK_FILLED.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::HEART.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::HEART_FILLED.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::STAR.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::STAR_FILLED.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::STAR_HALF_FILLED.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::COMPASS.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::COMPASS_FILLED.painter())(c, $progress, color)),
            Data::new(|c, _, color| (animated::NO_NETWORK.painter())(c, $progress, color)),
        ]
    };
}

/// A row of `icons` at `size`, still.
fn strip<'a>(icons: impl IntoIterator<Item = Data>, size: f32) -> Element<'a, ()> {
    row(icons
        .into_iter()
        .map(|data| animated::icon(data).size(size).mode(Mode::Still).into()))
    .spacing(8)
    .into()
}

/// The final frames at 20 and 48 pixels, then three rows of frames at a
/// quarter, half and three quarters of the play.
fn catalog() -> Element<'static, ()> {
    column![
        text("final"),
        strip(ICONS.map(|(_, data)| data), 20.0),
        strip(ICONS.map(|(_, data)| data), 48.0),
        text("0.25 / 0.5 / 0.75"),
        strip(frozen!(0.25), 48.0),
        strip(frozen!(0.5), 48.0),
        strip(frozen!(0.75), 48.0),
    ]
    .spacing(8)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (600.0, 340.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn animated_icon_light() {
    snapshot(&Theme::light(), "animated_icon_light");
}

#[test]
fn animated_icon_dark() {
    snapshot(&Theme::dark(), "animated_icon_dark");
}

#[test]
fn every_icon_draws_at_every_progress() {
    let mut renderer = renderer();

    for (_, data) in ICONS {
        let mut ui = build(
            animated::icon(data).mode(Mode::Repeat),
            Cache::default(),
            &mut renderer,
        );
        let start = Instant::now();

        for step in 0..=24 {
            frame(&mut ui, &mut renderer, start + data.duration() * step / 24);
        }
    }
}
