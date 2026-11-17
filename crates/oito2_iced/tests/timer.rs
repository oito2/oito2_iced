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

//! Timer: the stopwatch and countdown arithmetic, the formatting, the text
//! that redraws itself while the timer runs, its tick and completion
//! messages and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::time::Instant;
use iced::widget::{column, row};
use iced::{Event, window};
use iced_test::simulator;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, typography};
use oito2_iced::widget::timer::{self, Timer};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tick(Instant),
    Completed(Instant),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn secs(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

fn redraw(ui: &mut Simulator<'_>, now: Instant) {
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
}

fn shows(ui: &mut Simulator<'_>, text: &str) -> bool {
    ui.find(text).is_ok()
}

#[test]
fn durations_format_like_a_clock() {
    assert_eq!(timer::format_duration(secs(5), false), "0:05");
    assert_eq!(timer::format_duration(secs(61 * 60 + 2), false), "1:01:02");
    assert_eq!(
        timer::format_duration(Duration::from_millis(1250), true),
        "0:01.2"
    );
    assert_eq!(timer::format_duration(Duration::ZERO, true), "0:00.0");
}

#[test]
fn a_countdown_pauses_resumes_and_completes() {
    let t0 = Instant::now();
    let mut countdown = Timer::countdown(secs(5));

    assert_eq!(countdown.remaining(t0), Some(secs(5)));
    countdown.start(t0);
    assert!(countdown.is_running());
    assert_eq!(countdown.remaining(t0 + secs(2)), Some(secs(3)));

    countdown.pause(t0 + secs(2));
    assert!(!countdown.is_running());
    assert_eq!(countdown.remaining(t0 + secs(5)), Some(secs(3)));

    countdown.start(t0 + secs(5));
    assert_eq!(countdown.completes_at(), Some(t0 + secs(8)));
    assert!(!countdown.tick(t0 + secs(7)));
    assert_eq!(countdown.remaining(t0 + secs(9)), Some(Duration::ZERO));
    assert!(countdown.tick(t0 + secs(9)));
    assert!(countdown.is_completed());
    assert!(!countdown.is_running());
    assert_eq!(countdown.elapsed(t0 + secs(20)), secs(5));

    // A completed countdown starts again only after a reset.
    countdown.start(t0 + secs(10));
    assert!(!countdown.is_running());
    countdown.restart(t0 + secs(10));
    assert!(countdown.is_running());
    assert!(!countdown.is_completed());
    assert_eq!(countdown.remaining(t0 + secs(11)), Some(secs(4)));

    countdown.reset();
    assert_eq!(countdown.remaining(t0 + secs(30)), Some(secs(5)));
}

#[test]
fn a_stopwatch_counts_up_and_never_completes() {
    let t0 = Instant::now();
    let mut stopwatch = Timer::stopwatch();

    stopwatch.start(t0);
    assert_eq!(stopwatch.elapsed(t0 + secs(65)), secs(65));
    assert_eq!(stopwatch.remaining(t0 + secs(65)), None);
    assert!(!stopwatch.tick(t0 + secs(65)));
    assert_eq!(timer::timer_label(&stopwatch, t0 + secs(65), false), "1:05");
}

#[test]
fn a_countdown_rounds_up_without_tenths() {
    let t0 = Instant::now();
    let mut countdown = Timer::countdown(secs(5));
    countdown.start(t0);

    let later = t0 + Duration::from_millis(250);
    assert_eq!(timer::timer_label(&countdown, later, false), "0:05");
    assert_eq!(timer::timer_label(&countdown, later, true), "0:04.7");
    assert_eq!(timer::timer_label(&countdown, t0 + secs(5), false), "0:00");
}

fn view<'a>(timer: &Timer) -> Element<'a, Message> {
    column![
        timer::timer_text(timer)
            .on_tick(Message::Tick)
            .on_complete(Message::Completed)
    ]
    .padding(8)
    .into()
}

#[test]
fn the_text_redraws_while_the_countdown_runs() {
    let t0 = Instant::now();
    let mut countdown = Timer::countdown(secs(5));
    countdown.start(t0);

    let mut ui = Simulator::with_settings(support::settings(), view(&countdown));

    redraw(&mut ui, t0);
    assert!(shows(&mut ui, "0:05"));

    redraw(&mut ui, t0 + secs(2));
    assert!(shows(&mut ui, "0:03"));

    redraw(&mut ui, t0 + Duration::from_millis(2500));
    assert!(shows(&mut ui, "0:03"));

    redraw(&mut ui, t0 + secs(6));
    assert!(shows(&mut ui, "0:00"));

    // The completion is published once, with the instant the countdown
    // reached zero.
    redraw(&mut ui, t0 + secs(7));

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Tick(t0 + secs(2)),
            Message::Tick(t0 + secs(5)),
            Message::Completed(t0 + secs(5)),
        ]
    );
}

#[test]
fn a_stopped_timer_publishes_nothing() {
    let t0 = Instant::now();
    let mut stopwatch = Timer::stopwatch();
    stopwatch.start(t0);
    stopwatch.pause(t0 + secs(3));

    let mut ui = Simulator::with_settings(support::settings(), view(&stopwatch));

    redraw(&mut ui, t0 + secs(10));
    assert!(shows(&mut ui, "0:03"));
    redraw(&mut ui, t0 + secs(20));

    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn a_stopwatch_ticks_every_interval() {
    let t0 = Instant::now();
    let mut stopwatch = Timer::stopwatch().with_interval(Duration::from_millis(500));
    stopwatch.start(t0);

    let mut ui = Simulator::with_settings(support::settings(), view(&stopwatch));

    redraw(&mut ui, t0);
    redraw(&mut ui, t0 + Duration::from_millis(400));
    redraw(&mut ui, t0 + Duration::from_millis(600));
    redraw(&mut ui, t0 + Duration::from_millis(1100));
    assert!(shows(&mut ui, "0:01"));

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Tick(t0 + Duration::from_millis(600)),
            Message::Tick(t0 + Duration::from_millis(1100)),
        ]
    );
}

fn catalog() -> Element<'static, Message> {
    let t0 = Instant::now();
    let mut paused = Timer::stopwatch();
    paused.start(t0);
    paused.pause(t0 + Duration::from_millis(65_300));

    let mut long = Timer::stopwatch();
    long.start(t0);
    long.pause(t0 + secs(3723));

    column![
        timer::timer_text(&Timer::countdown(secs(270))).style(typography::TITLE_LARGE),
        row![
            timer::timer_text(&Timer::stopwatch()),
            timer::timer_text(&paused).tenths(true),
            timer::timer_text(&long),
        ]
        .spacing(16),
        timer::timer_text(&Timer::countdown(secs(42)))
            .style(typography::HEADLINE_LARGE)
            .color(iced::Color::from_rgb8(0x35, 0x84, 0xE4)),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (240.0, 160.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn timer_light() {
    snapshot(&Theme::light(), "timer_light");
}

#[test]
fn timer_dark() {
    snapshot(&Theme::dark(), "timer_dark");
}

#[test]
fn timer_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "timer_light_high_contrast");
}
