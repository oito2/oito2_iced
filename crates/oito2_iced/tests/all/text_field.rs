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

//! Text fields: typing, the trailing element, focus, the height per
//! density, and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::column;
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::lucide;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, text_field};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Name(String),
    Clear,
    Submit,
}

fn form(value: &str) -> Element<'_, Message> {
    oito2_iced::focus::navigation(
        column![
            text_field::text_field("Name", value)
                .id("name")
                .on_input(Message::Name)
                .on_submit(Message::Submit),
            text_field::text_field("Search", value)
                .id("search")
                .leading(lucide::SEARCH)
                .trailing(
                    button::icon_button(lucide::X)
                        .on_press(Message::Clear)
                        .id("clear")
                )
                .on_input(Message::Name),
        ]
        .spacing(8),
    )
    .into()
}

fn bounds(ui: &mut simulator::Simulator<'_, Message, Theme>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

#[test]
fn typing_publishes_the_text_and_enter_submits() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), form(""));

    ui.click(selector::id("name")).expect("name field");
    ui.typewrite("Ana");
    ui.tap_key(Named::Enter);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Name("A".into()),
            Message::Name("An".into()),
            Message::Name("Ana".into()),
            Message::Submit,
        ]
    );
}

#[test]
fn the_trailing_element_takes_its_clicks_and_tab_reaches_it() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), form("text"));
    let clear = bounds(&mut ui, "clear");

    ui.click(selector::id("clear")).expect("clear button");

    ui.tap_key(Named::Tab);
    let first = ui.find(selector::is_focused()).expect("focus").bounds();
    assert_eq!(first, bounds(&mut ui, "name"));

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    let third = ui.find(selector::is_focused()).expect("focus").bounds();
    assert_eq!(third, clear);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Clear]);
}

#[test]
fn fields_are_as_tall_as_the_controls_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), form(""));

        assert_eq!(bounds(&mut ui, "name").height, height, "{density:?}");
        assert_eq!(bounds(&mut ui, "search").height, height, "{density:?}");
        assert_eq!(bounds(&mut ui, "clear").height, height, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    column![
        text_field::text_field("Placeholder", ""),
        text_field::text_field("Placeholder", "")
            .label("Label")
            .helper("Helper text")
            .on_input(Message::Name),
        text_field::text_field("", "Typed value")
            .label("With an error")
            .error(Some("This value is not accepted"))
            .on_input(Message::Name),
        text_field::text_field("Search", "")
            .leading(lucide::SEARCH)
            .trailing(button::icon_button(lucide::X).on_press(Message::Clear))
            .on_input(Message::Name),
        text_field::text_field("", "secret")
            .label("Password")
            .secure(true)
            .on_input(Message::Name),
    ]
    .spacing(12)
    .padding(12)
    .width(360)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 400.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn text_fields_light() {
    snapshot(&Theme::light(), "text_fields_light");
}

#[test]
fn text_fields_dark() {
    snapshot(&Theme::dark(), "text_fields_dark");
}

#[test]
fn text_fields_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "text_fields_light_high_contrast");
}

#[test]
fn tab_skips_a_disabled_field() {
    let view: Element<'static, Message> = oito2_iced::focus::navigation(
        column![
            text_field::text_field("Disabled", ""),
            text_field::text_field("Enabled", "")
                .id("enabled")
                .on_input(Message::Name),
        ]
        .spacing(8),
    )
    .into();

    let mut ui = simulator::Simulator::with_settings(support::settings(), view);
    let enabled = bounds(&mut ui, "enabled");

    ui.tap_key(Named::Tab);

    let focused = ui.find(selector::is_focused()).expect("focus").bounds();
    assert_eq!(focused, enabled);
}
