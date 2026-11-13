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

//! Sliders: pointer and keyboard changes, marks along the track, the
//! vertical orientation, the disabled state and golden snapshots of the
//! catalog. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, row};
use iced::{Point, Rectangle, mouse};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, slider};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(f32),
    Started(f32),
    Ended(f32),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn marks() -> [slider::Mark<'static>; 3] {
    [
        slider::mark(0.0).label("0%"),
        slider::mark(50.0).label("50%"),
        slider::mark(100.0).label("100%"),
    ]
}

fn volume(value: f32) -> slider::Slider<'static, Message> {
    slider::slider(value)
        .range(0.0, 100.0)
        .divisions(10)
        .label("Volume")
        .value_text(|value| format!("{value:.0}%"))
        .on_change(Message::Changed)
        .id("volume")
}

fn view(slider: slider::Slider<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            container(slider).width(300),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn track(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("volume")).expect("track").bounds()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn shows_the_label_and_value_and_changes_with_a_tap() {
    let mut ui = Simulator::with_settings(support::settings(), view(volume(40.0)));

    ui.find("Volume").expect("label");
    ui.find("40%").expect("value text");

    let track = track(&mut ui);
    ui.point_at(track.center());
    ui.simulate(simulator::click());

    assert_eq!(messages(ui), vec![Message::Changed(50.0)]);
}

#[test]
fn keys_change_the_value_while_focused() {
    let mut ui = Simulator::with_settings(support::settings(), view(volume(40.0)));
    let track = track(&mut ui);

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Tab);
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(track)
    );

    for key in [
        Named::ArrowRight,
        Named::ArrowUp,
        Named::ArrowLeft,
        Named::ArrowDown,
        Named::PageUp,
        Named::PageDown,
        Named::Home,
        Named::End,
    ] {
        ui.tap_key(key);
    }

    assert_eq!(
        messages(ui),
        [50.0, 50.0, 30.0, 30.0, 50.0, 30.0, 0.0, 100.0].map(Message::Changed)
    );
}

#[test]
fn a_change_starts_and_ends_around_a_key_press_and_a_drag() {
    let slider = volume(40.0)
        .on_change_start(Message::Started)
        .on_change_end(Message::Ended);
    let mut ui = Simulator::with_settings(support::settings(), view(slider));
    let track = track(&mut ui);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowRight);

    let at = |t: f32| Point::new(track.x + 10.0 + (track.width - 20.0) * t, track.center_y());
    ui.point_at(at(0.2));
    ui.simulate([iced::Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    ui.point_at(at(0.7));
    ui.simulate([iced::Event::Mouse(mouse::Event::CursorMoved {
        position: at(0.7),
    })]);
    ui.simulate([iced::Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    assert_eq!(
        messages(ui),
        vec![
            Message::Started(40.0),
            Message::Changed(50.0),
            Message::Ended(50.0),
            Message::Started(40.0),
            Message::Changed(20.0),
            Message::Changed(70.0),
            Message::Ended(70.0),
        ]
    );
}

#[test]
fn a_disabled_slider_ignores_input_and_is_skipped() {
    let disabled = slider::slider(40.0).range(0.0, 100.0).label("Volume");
    let mut ui = Simulator::with_settings(support::settings(), view(disabled));
    let after = ui.find(selector::id("after")).expect("after").bounds();
    let label = ui.find("Volume").expect("label").bounds();

    ui.point_at(Point::new(label.x + 150.0, label.y + label.height + 14.0));
    ui.simulate(simulator::click());
    ui.tap_key(Named::Tab);

    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(after)
    );
    assert!(messages(ui).is_empty());
}

#[test]
fn marks_sit_under_their_values_on_the_track() {
    let slider = slider::slider(50.0)
        .range(0.0, 100.0)
        .marks(marks())
        .on_change(Message::Changed)
        .id("volume");
    let mut ui = Simulator::with_settings(support::settings(), view(slider));
    let track = track(&mut ui);
    let center = |ui: &mut Simulator<'_>, label: &'static str| {
        ui.find(label).expect("mark label").bounds().center()
    };

    assert!((center(&mut ui, "0%").x - (track.x + 10.0)).abs() <= 0.5);
    assert!((center(&mut ui, "50%").x - track.center_x()).abs() <= 0.5);
    assert!((center(&mut ui, "100%").x - (track.x + track.width - 10.0)).abs() <= 0.5);
    assert!(center(&mut ui, "50%").y > track.y + track.height);
}

#[test]
fn a_vertical_slider_stands_at_its_length_with_side_marks() {
    let slider = slider::slider(30.0)
        .range(0.0, 100.0)
        .vertical(true)
        .length(200.0)
        .marks(marks())
        .on_change(Message::Changed)
        .id("volume");
    let mut ui = Simulator::with_settings(support::settings(), view(slider));
    let track = track(&mut ui);
    let label =
        |ui: &mut Simulator<'_>, label: &'static str| ui.find(label).expect("mark label").bounds();

    assert_eq!(track.height, 200.0);
    assert!((label(&mut ui, "100%").center_y() - (track.y + 10.0)).abs() <= 0.5);
    assert!((label(&mut ui, "0%").center_y() - (track.y + 190.0)).abs() <= 0.5);
    assert!(label(&mut ui, "50%").x > track.x + track.width);

    ui.point_at(Point::new(track.center_x(), track.y + track.height - 12.0));
    ui.simulate(simulator::click());

    let messages = messages(ui);
    assert!(
        matches!(messages.as_slice(), [Message::Changed(value)] if *value < 10.0),
        "{messages:?}"
    );
}

fn catalog() -> Element<'static, Message> {
    row![
        column![
            volume(40.0).marks(marks()),
            slider::slider(0.3).on_change(Message::Changed),
            slider::slider(0.6).label("Disabled"),
        ]
        .spacing(16)
        .width(320),
        slider::slider(70.0)
            .range(0.0, 100.0)
            .vertical(true)
            .length(160.0)
            .marks(marks())
            .on_change(Message::Changed),
    ]
    .spacing(32)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (480.0, 220.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn slider_light() {
    snapshot(&Theme::light(), "slider_light");
}

#[test]
fn slider_dark() {
    snapshot(&Theme::dark(), "slider_dark");
}

#[test]
fn slider_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "slider_light_high_contrast");
}
