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

//! Rating: clicks on whole and half stars, the keyboard, the reading
//! direction, the read-only state, the localized value text and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row};
use iced::{Point, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::{button, rating};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Rated(f32),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn view(rating: rating::Rating<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            rating.id("rating"),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn stars(ui: &mut Simulator<'_>) -> Rectangle {
    ui.find(selector::id("rating")).expect("rating").bounds()
}

fn click(ui: &mut Simulator<'_>, at: Point) {
    ui.point_at(at);
    ui.simulate(simulator::click());
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn a_click_picks_a_star_and_a_half_with_allow_half() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(rating::rating(2.0).on_change(Message::Rated)),
    );
    let row = stars(&mut ui);
    click(
        &mut ui,
        Point::new(row.x + 24.0 * 3.0 + 6.0, row.center_y()),
    );
    assert_eq!(messages(ui), vec![Message::Rated(4.0)]);

    let half = rating::rating(2.0)
        .allow_half(true)
        .on_change(Message::Rated);
    let mut ui = Simulator::with_settings(support::settings(), view(half));
    let row = stars(&mut ui);
    click(
        &mut ui,
        Point::new(row.x + 24.0 * 2.0 + 4.0, row.center_y()),
    );
    click(
        &mut ui,
        Point::new(row.x + 24.0 * 2.0 + 18.0, row.center_y()),
    );
    assert_eq!(messages(ui), vec![Message::Rated(2.5), Message::Rated(3.0)]);
}

#[test]
fn a_value_between_steps_steps_to_the_next_one() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(
            rating::rating(3.7)
                .allow_half(true)
                .on_change(Message::Rated),
        ),
    );
    let row = stars(&mut ui);

    click(
        &mut ui,
        Point::new(row.x + 24.0 * 4.0 + 18.0, row.center_y()),
    );
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::ArrowRight);

    // 3.7 aligns to 3.5 first, so the arrows give 3 and 4.
    assert_eq!(messages(ui), [5.0, 3.0, 4.0].map(Message::Rated));
}

#[test]
fn keys_change_the_value_while_focused() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(rating::rating(2.0).on_change(Message::Rated)),
    );
    let row = stars(&mut ui);

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Tab);
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(row)
    );

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::ArrowLeft);
    ui.typewrite("5");
    ui.typewrite("7");
    ui.tap_key(Named::Home);
    ui.tap_key(Named::End);

    assert_eq!(
        messages(ui),
        [3.0, 3.0, 1.0, 1.0, 5.0, 0.0, 5.0].map(Message::Rated)
    );
}

#[test]
fn right_to_left_mirrors_the_pointer_and_the_arrows() {
    let rtl = rating::rating(2.0)
        .direction(ReadingDirection::RightToLeft)
        .on_change(Message::Rated);
    let mut ui = Simulator::with_settings(support::settings(), view(rtl));
    let row = stars(&mut ui);

    click(&mut ui, Point::new(row.x + row.width - 6.0, row.center_y()));
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(messages(ui), vec![Message::Rated(1.0), Message::Rated(3.0)]);
}

#[test]
fn a_read_only_rating_ignores_input_and_is_skipped() {
    let mut ui = Simulator::with_settings(support::settings(), view(rating::rating(3.0)));
    let row = stars(&mut ui);
    let after = ui.find(selector::id("after")).expect("after").bounds();

    click(&mut ui, Point::new(row.x + 6.0, row.center_y()));
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
fn the_size_and_count_set_the_row() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(rating::rating(1.0).max(10).size(16.0)),
    );
    let row = stars(&mut ui);

    assert_eq!((row.width, row.height), (160.0, 16.0));
}

#[test]
fn the_value_text_follows_the_locale() {
    assert_eq!(
        rating::value_text(0.0, 5, &Localizations::en()),
        "0 of 5 stars"
    );
    assert_eq!(
        rating::value_text(2.5, 10, &Localizations::pt_br()),
        "2,5 de 10 estrelas"
    );
}

fn catalog() -> Element<'static, Message> {
    column![
        rating::rating(3.5)
            .allow_half(true)
            .on_change(Message::Rated),
        rating::rating(2.0).on_change(Message::Rated),
        rating::rating(4.0).max(7).size(16.0),
        rating::rating(1.5)
            .allow_half(true)
            .direction(ReadingDirection::RightToLeft),
        row![rating::rating(3.0).size(32.0).max(3)],
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (220.0, 200.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn rating_light() {
    snapshot(&Theme::light(), "rating_light");
}

#[test]
fn rating_dark() {
    snapshot(&Theme::dark(), "rating_dark");
}

#[test]
fn rating_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "rating_light_high_contrast");
}

#[test]
fn rating_focused() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (180.0, 90.0),
        view(rating::rating(3.0).on_change(Message::Rated)),
    );

    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "rating_focused");
}
