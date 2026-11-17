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

//! Dialogs: the dialog keeps the input over a scrim, Escape and the scrim
//! dismiss it, Tab cycles inside it, the title bar close button closes it,
//! and golden snapshots of a dialog over a page. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::{self, key::Named};
use iced::widget::{column, text};
use iced::{Event, Length, Point};
use iced_test::{selector, simulator};
use oito2_iced::foundation::{WindowButton, WindowButtonLayout, WindowControlStyle};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, dialog};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Behind,
    First,
    Save,
    Dismiss,
}

const SIZE: (f32, f32) = (640.0, 400.0);

fn page() -> Element<'static, Message> {
    column![
        text("Page"),
        button::filled("Behind")
            .on_press(Message::Behind)
            .id("behind"),
    ]
    .spacing(12)
    .padding(12)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn settings(dismissible: bool) -> dialog::Dialog<'static, Message> {
    dialog::dialog(
        column![
            text("Changes are saved automatically."),
            button::outlined("First")
                .on_press(Message::First)
                .id("first"),
        ]
        .spacing(12),
    )
    .title_bar(dialog::title_bar().title("Preferences"))
    .title("Settings")
    .action(button::text("Save").on_press(Message::Save).id("save"))
    .on_dismiss(Message::Dismiss)
    .dismissible(dismissible)
}

fn view(dialog: Option<dialog::Dialog<'static, Message>>) -> Element<'static, Message> {
    focus::navigation(dialog::host(page(), dialog)).into()
}

fn open(
    dialog: Option<dialog::Dialog<'static, Message>>,
) -> simulator::Simulator<'static, Message, Theme> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(dialog))
}

fn messages(ui: simulator::Simulator<'_, Message, Theme>) -> Vec<Message> {
    ui.into_messages().collect()
}

fn shift_tab() -> Event {
    Event::Keyboard(keyboard::Event::KeyPressed {
        key: Named::Tab.into(),
        modified_key: Named::Tab.into(),
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers: keyboard::Modifiers::SHIFT,
        text: None,
        repeat: false,
    })
}

#[test]
fn without_a_dialog_the_page_is_usable() {
    let mut ui = open(None);

    ui.click("Behind").expect("page button");
    assert!(ui.find("Settings").is_err());
    assert_eq!(messages(ui), vec![Message::Behind]);
}

#[test]
fn the_dialog_is_centered() {
    let mut ui = open(Some(settings(true)));
    let title = ui.find("Settings").expect("title").bounds();
    let save = ui.find(selector::id("save")).expect("action").bounds();

    assert!(title.x > 40.0 && title.y > 24.0);
    assert!(save.x + save.width < SIZE.0 - 40.0);
    let middle = (title.y + save.y + save.height) / 2.0;
    assert!((middle - SIZE.1 / 2.0).abs() < 40.0, "centered at {middle}");
}

#[test]
fn the_page_below_receives_no_input() {
    let mut ui = open(Some(settings(false)));

    ui.click("Behind").expect("page button is found");
    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn escape_dismisses() {
    let mut ui = open(Some(settings(false)));

    ui.tap_key(Named::Escape);
    assert_eq!(messages(ui), vec![Message::Dismiss]);
}

#[test]
fn a_press_on_the_scrim_dismisses_when_dismissible() {
    let mut ui = open(Some(settings(true)));
    ui.point_at(Point::new(5.0, 5.0));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Dismiss]);

    let mut ui = open(Some(settings(false)));
    ui.point_at(Point::new(5.0, 5.0));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn clicks_inside_reach_the_content() {
    let mut ui = open(Some(settings(true)));

    ui.click(selector::id("first")).expect("content button");
    ui.click(selector::id("save")).expect("action");
    assert_eq!(messages(ui), vec![Message::First, Message::Save]);
}

#[test]
fn the_title_bar_close_button_dismisses() {
    let mut ui = open(Some(settings(true)));

    ui.click("Close").expect("close button");
    assert_eq!(messages(ui), vec![Message::Dismiss]);
}

#[test]
fn the_close_button_follows_the_button_layout() {
    let layout = WindowButtonLayout::new(vec![WindowButton::Close], Vec::new());
    let bar = dialog::title_bar()
        .title("Preferences")
        .control_style(WindowControlStyle::Macos)
        .button_layout(&layout);
    let mut ui = open(Some(
        dialog::dialog(text("Body"))
            .title_bar(bar)
            .on_dismiss(Message::Dismiss),
    ));

    let close = ui.find("Close").expect("close button").bounds();
    let title = ui.find("Preferences").expect("title").bounds();
    assert!(close.x < title.x, "the close button leads");
}

#[test]
fn tab_cycles_inside_the_dialog() {
    let mut ui = open(Some(settings(true)));
    let first = ui.find(selector::id("first")).expect("first").bounds();
    let save = ui.find(selector::id("save")).expect("save").bounds();
    let focused = |ui: &mut simulator::Simulator<'_, Message, Theme>| {
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds())
    };

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(first));
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(save));
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(first), "wraps to the first");
    ui.simulate([shift_tab()]);
    assert_eq!(focused(&mut ui), Some(save), "wraps to the last");

    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), vec![Message::Save]);
}

#[test]
fn the_dialog_is_at_least_its_minimum_width() {
    let mut ui = open(Some(
        dialog::dialog(text("Hi")).on_dismiss(Message::Dismiss),
    ));
    let body = ui.find("Hi").expect("body").bounds();

    // The body sits 24 pixels inside a centered surface at least 280 wide.
    let surface_left = body.x - 24.0;
    assert!(surface_left <= (SIZE.0 - dialog::MIN_WIDTH) / 2.0 + 0.5);
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = open(Some(settings(true)));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn dialog_light() {
    snapshot(&Theme::light(), "dialog_light");
}

#[test]
fn dialog_dark() {
    snapshot(&Theme::dark(), "dialog_dark");
}

#[test]
fn dialog_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "dialog_light_high_contrast");
}
