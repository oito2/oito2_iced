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

//! Copy buttons and copy fields: clicks and keys, the clipboard, the timed
//! copied state, the read-only field, sizes per density and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{column, container, row};
use iced::{Event, Point, window};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::copy;
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Copied,
    FieldCopied,
}

fn view() -> Element<'static, Message> {
    let localizations = Localizations::en();

    oito2_iced::focus::navigation(
        column![
            copy::copy_button(&localizations, "secret")
                .on_copied(Message::Copied)
                .id("copy"),
            container(
                copy::copy_field(&localizations, "https://oito2.dev")
                    .id("field")
                    .on_copied(Message::FieldCopied)
            )
            .width(300),
        ]
        .spacing(8),
    )
    .into()
}

fn focused(ui: &mut simulator::Simulator<'_, Message, Theme>) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn bounds(ui: &mut simulator::Simulator<'_, Message, Theme>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn click_at(ui: &mut simulator::Simulator<'_, Message, Theme>, point: Point) {
    ui.point_at(point);
    ui.simulate(simulator::click());
}

fn redraw_at(ui: &mut simulator::Simulator<'_, Message, Theme>, now: Instant) {
    ui.simulate([Event::Window(window::Event::RedrawRequested(now))]);
}

fn pixels(ui: &mut simulator::Simulator<'_, Message, Theme>, name: &str) -> support::Image {
    support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), name)
}

#[test]
fn a_click_publishes_the_copied_message() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    let button = bounds(&mut ui, "copy");
    click_at(&mut ui, button.center());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Copied]);
}

#[test]
fn enter_and_space_copy_and_the_focus_stays_on_the_button() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    ui.tap_key(Named::Tab);
    let button = bounds(&mut ui, "copy");
    assert_eq!(focused(&mut ui), Some(button));

    ui.tap_key(Named::Enter);
    assert_eq!(focused(&mut ui), Some(button));

    ui.tap_key(Named::Space);
    assert_eq!(focused(&mut ui), Some(button));

    let now = Instant::now();
    redraw_at(&mut ui, now);
    redraw_at(&mut ui, now + Duration::from_secs(2));
    assert_eq!(focused(&mut ui), Some(button));

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Copied, Message::Copied]);
}

#[test]
fn the_check_mark_shows_for_the_feedback_duration() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());
    let idle = pixels(&mut ui, "copy_idle");

    let button = bounds(&mut ui, "copy");
    click_at(&mut ui, button.center());
    ui.point_at(Point::new(390.0, 390.0));

    let now = Instant::now();
    redraw_at(&mut ui, now);
    let copied = pixels(&mut ui, "copy_copied");
    assert!(support::difference(&idle, &copied) > 0.0);

    redraw_at(
        &mut ui,
        now + copy::FEEDBACK_DURATION - Duration::from_millis(100),
    );
    let still = pixels(&mut ui, "copy_still");
    assert_eq!(support::difference(&copied, &still), 0.0);

    redraw_at(&mut ui, now + copy::FEEDBACK_DURATION);
    let reset = pixels(&mut ui, "copy_reset");
    assert_eq!(support::difference(&idle, &reset), 0.0);
}

#[test]
fn the_copy_field_is_read_only_and_its_button_copies() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    let field = bounds(&mut ui, "field");
    click_at(&mut ui, Point::new(field.x + 20.0, field.center_y()));
    ui.typewrite("typed");
    ui.tap_key(Named::Backspace);

    click_at(
        &mut ui,
        Point::new(field.x + field.width - field.height / 2.0, field.center_y()),
    );

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::FieldCopied]);
}

#[test]
fn the_copy_button_writes_its_text_to_the_clipboard() {
    use iced::advanced::clipboard::{Clipboard, Kind};
    use iced::advanced::renderer::Headless;
    use iced::mouse;
    use iced_test::runtime::user_interface::{self, UserInterface};

    #[derive(Default)]
    struct Recording(Option<String>);

    impl Clipboard for Recording {
        fn read(&self, _kind: Kind) -> Option<String> {
            None
        }

        fn write(&mut self, kind: Kind, contents: String) {
            assert_eq!(kind, Kind::Standard);
            self.0 = Some(contents);
        }
    }

    // Loads the fonts of the settings into the text stack.
    let _ = simulator::Simulator::with_settings(support::settings(), view());

    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let mut renderer =
        iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
            oito2_iced::font::INTER,
            iced::Pixels(16.0),
            backend.as_deref(),
        ))
        .expect("headless renderer");

    let mut ui = UserInterface::build(
        view(),
        iced::Size::new(400.0, 200.0),
        user_interface::Cache::default(),
        &mut renderer,
    );

    let point = Point::new(17.0, 17.0);
    let cursor = mouse::Cursor::Available(point);
    let mut clipboard = Recording::default();
    let mut messages = Vec::new();

    let _ = ui.update(
        &[
            Event::Mouse(mouse::Event::CursorMoved { position: point }),
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        ],
        cursor,
        &mut renderer,
        &mut clipboard,
        &mut messages,
    );

    assert_eq!(clipboard.0.as_deref(), Some("secret"));
    assert_eq!(messages, vec![Message::Copied]);

    let now = Instant::now();
    let (state, _) = ui.update(
        &[Event::Window(window::Event::RedrawRequested(now))],
        cursor,
        &mut renderer,
        &mut clipboard,
        &mut messages,
    );

    match state {
        user_interface::State::Updated { redraw_request, .. } => {
            assert!(redraw_request <= window::RedrawRequest::At(now + copy::FEEDBACK_DURATION));
        }
        user_interface::State::Outdated => unreachable!("the interface is up to date"),
    }
}

#[test]
fn copy_buttons_follow_the_control_height_of_the_density() {
    for (density, side) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), view());
        let button = bounds(&mut ui, "copy");
        let field = bounds(&mut ui, "field");

        assert_eq!((button.width, button.height), (side, side), "{density:?}");
        assert_eq!(field.height, side, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let localizations = Localizations::en();

    column![
        row![
            copy::copy_button(&localizations, "idle").on_copied(Message::Copied),
            copy::copy_button(&localizations, "copied")
                .on_copied(Message::Copied)
                .id("copied"),
        ]
        .spacing(8),
        copy::copy_field(&localizations, "https://oito2.dev").label("Share link"),
        copy::copy_field(&localizations, "token-1234").obscure(true),
    ]
    .spacing(16)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 200.0), catalog());

    let copied = ui.find(selector::id("copied")).expect("button").bounds();
    ui.point_at(copied.center());
    ui.simulate(simulator::click());
    ui.point_at(Point::new(359.0, 199.0));

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn copy_light() {
    snapshot(&Theme::light(), "copy_light");
}

#[test]
fn copy_dark() {
    snapshot(&Theme::dark(), "copy_dark");
}

#[test]
fn copy_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "copy_light_high_contrast");
}
