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

//! Angle slider: pointing with a press and a drag, the keyboard with wrap
//! around, the end of a change, the read-only state, the localized value
//! and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh
//! the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row};
use iced::{Point, Rectangle, mouse};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{angle_slider, button};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Turned(f32),
    Ended(f32),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn view(dial: angle_slider::AngleSlider<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            dial.id("dial"),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn dial(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("dial")).expect("dial").bounds()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn points_at_a_press_and_steps_by_keys_with_wrap() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(
            angle_slider::angle_slider(0.0)
                .size(100.0)
                .on_change(Message::Turned),
        ),
    );
    let dial = dial(&mut ui);

    ui.find("0°").expect("value text");
    ui.point_at(Point::new(dial.x + dial.width - 5.0, dial.center_y()));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Turned(90.0)]);

    let mut ui = Simulator::with_settings(
        support::settings(),
        view(
            angle_slider::angle_slider(90.0)
                .size(100.0)
                .on_change(Message::Turned),
        ),
    );
    ui.find("90°").expect("value text");
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Home);

    let mut keys = Simulator::with_settings(
        support::settings(),
        view(
            angle_slider::angle_slider(0.0)
                .size(100.0)
                .on_change(Message::Turned),
        ),
    );
    keys.tap_key(Named::Tab);
    keys.tap_key(Named::ArrowDown);
    keys.tap_key(Named::ArrowLeft);
    keys.tap_key(Named::ArrowUp);
    keys.tap_key(Named::ArrowRight);
    keys.tap_key(Named::PageUp);
    keys.tap_key(Named::PageDown);

    assert_eq!(messages(ui), vec![Message::Turned(0.0)]);
    assert_eq!(
        messages(keys),
        [359.0, 359.0, 1.0, 1.0, 15.0, 345.0].map(Message::Turned)
    );
}

#[test]
fn a_press_and_each_key_press_end_the_change() {
    let dial_with_end = || {
        angle_slider::angle_slider(0.0)
            .size(100.0)
            .on_change(Message::Turned)
            .on_change_end(Message::Ended)
    };
    let mut ui = Simulator::with_settings(support::settings(), view(dial_with_end()));
    let bounds = dial(&mut ui);

    ui.point_at(Point::new(bounds.x + bounds.width - 5.0, bounds.center_y()));
    ui.simulate(simulator::click());
    ui.tap_key(Named::ArrowUp);

    assert_eq!(
        messages(ui),
        vec![
            Message::Turned(90.0),
            Message::Ended(90.0),
            Message::Turned(1.0),
            Message::Ended(1.0),
        ]
    );
}

#[test]
fn a_drag_turns_the_dial_in_steps() {
    let dial_with_end = angle_slider::angle_slider(0.0)
        .size(100.0)
        .step(45.0)
        .on_change(Message::Turned)
        .on_change_end(Message::Ended);
    let mut ui = Simulator::with_settings(support::settings(), view(dial_with_end));
    let bounds = dial(&mut ui);
    let center = bounds.center();

    ui.point_at(Point::new(center.x + 40.0, center.y - 2.0));
    ui.simulate([iced::Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    for position in [
        Point::new(center.x + 1.0, center.y + 1.0),
        Point::new(center.x + 30.0, center.y + 32.0),
        Point::new(center.x - 3.0, center.y + 40.0),
    ] {
        ui.point_at(position);
        ui.simulate([iced::Event::Mouse(mouse::Event::CursorMoved { position })]);
    }
    ui.simulate([iced::Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    assert_eq!(
        messages(ui),
        vec![
            Message::Turned(90.0),
            Message::Turned(135.0),
            Message::Turned(180.0),
            Message::Ended(180.0),
        ]
    );
}

#[test]
fn a_read_only_dial_ignores_input_and_is_skipped() {
    let mut ui =
        Simulator::with_settings(support::settings(), view(angle_slider::angle_slider(45.0)));
    let bounds = dial(&mut ui);
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.point_at(Point::new(bounds.x + bounds.width - 5.0, bounds.center_y()));
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
fn the_value_follows_the_locale_and_can_hide() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(
            angle_slider::angle_slider(22.5)
                .localizations(&Localizations::pt_br())
                .on_change(Message::Turned),
        ),
    );
    ui.find("22,5°").expect("localized value");

    let mut ui = Simulator::with_settings(
        support::settings(),
        view(angle_slider::angle_slider(22.5).show_value(false)),
    );
    assert!(ui.find("22.5°").is_err());
    assert_eq!(dial(&mut ui).width, angle_slider::DIAL_SIZE);
}

fn catalog() -> Element<'static, Message> {
    row![
        angle_slider::angle_slider(135.0).on_change(Message::Turned),
        angle_slider::angle_slider(300.0)
            .size(72.0)
            .show_value(false)
            .markers([0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0])
            .on_change(Message::Turned),
        angle_slider::angle_slider(45.0),
    ]
    .spacing(16)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (360.0, 130.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn angle_slider_light() {
    snapshot(&Theme::light(), "angle_slider_light");
}

#[test]
fn angle_slider_dark() {
    snapshot(&Theme::dark(), "angle_slider_dark");
}

#[test]
fn angle_slider_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "angle_slider_light_high_contrast");
}

#[test]
fn angle_slider_focused() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (140.0, 160.0),
        view(angle_slider::angle_slider(60.0).on_change(Message::Turned)),
    );

    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "angle_slider_focused");
}
