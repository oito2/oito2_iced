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

//! Command palette: it shows only while open with the field focused, the
//! query filters the commands, the arrows move the highlight, Enter and a
//! click run a command after closing, Escape and the scrim close, Tab stays
//! inside, the view behind gets no input, and golden snapshots of an open
//! palette. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, text};
use iced::{Length, Point};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::shortcuts::{Command, OperatingSystem, Shortcut};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, command_palette};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Query(String),
    Close,
    Content,
    Run(&'static str),
}

const SIZE: (f32, f32) = (640.0, 480.0);

fn commands() -> Vec<Command<Message>> {
    let shortcut = |text: &str| Shortcut::parse(text).expect("shortcut");

    vec![
        Command::new("file.new", "New file", Message::Run("new"))
            .with_group("File")
            .with_shortcut(shortcut("Mod+N")),
        Command::new("file.open", "Open file", Message::Run("open"))
            .with_group("File")
            .with_description("Open a file from the disk")
            .with_shortcut(shortcut("Mod+O")),
        Command::new("view.sidebar", "Toggle sidebar", Message::Run("sidebar"))
            .with_group("View")
            .with_shortcut(shortcut("Mod+Shift+B")),
        Command::new("go.home", "Go home", Message::Run("home")).with_shortcut(shortcut("G H")),
        Command::new("app.settings", "Preferences", Message::Run("settings"))
            .with_description("Theme and language")
            .with_keywords(["settings", "options"]),
    ]
}

fn view(open: bool, query: &str) -> Element<'static, Message> {
    let commands = commands();
    let refs: Vec<&Command<Message>> = commands.iter().collect();
    let content = container(column![
        button::filled("Content")
            .on_press(Message::Content)
            .id("content"),
        text("Behind"),
    ])
    .padding(12)
    .width(Length::Fill)
    .height(Length::Fill)
    .class(oito2_iced::theme::container::Class::Background);

    oito2_iced::focus::navigation(
        command_palette::command_palette(
            content,
            &Localizations::en(),
            &refs,
            query,
            Message::Query,
            Message::Close,
        )
        .open(open)
        .os(OperatingSystem::Linux),
    )
    .into()
}

fn open(query: &str) -> simulator::Simulator<'static, Message, Theme> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(true, query))
}

fn messages(ui: simulator::Simulator<'_, Message, Theme>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn the_palette_shows_only_while_open() {
    let mut closed = simulator::Simulator::with_size(support::settings(), SIZE, view(false, ""));
    assert!(closed.find("Behind").is_ok());
    assert!(closed.find("View").is_err());

    // The labels are rich text; the subtitles and key caps are plain text.
    let mut ui = open("");
    for text in [
        "File \u{b7} Open a file from the disk",
        "View",
        "G",
        "Theme and language",
    ] {
        assert!(ui.find(text).is_ok(), "{text} is listed");
    }
}

#[test]
fn the_field_takes_the_focus_and_typing_publishes_the_query() {
    let mut ui = open("");
    assert!(
        ui.find(selector::is_focused()).is_ok(),
        "the field has the focus"
    );

    ui.typewrite("o");

    assert_eq!(messages(ui), vec![Message::Query("o".into())]);
}

#[test]
fn the_query_filters_and_ranks_the_commands() {
    let mut ui = open("settings");
    assert!(ui.find("Theme and language").is_ok());
    assert!(ui.find("View").is_err());

    let mut ui = open("nothing at all");
    assert!(ui.find("No commands found").is_ok());
}

#[test]
fn enter_closes_and_runs_the_highlighted_command() {
    let mut ui = open("");
    ui.tap_key(Named::Enter);

    assert_eq!(messages(ui), vec![Message::Close, Message::Run("new")]);
}

#[test]
fn the_arrows_move_the_highlight_and_wrap() {
    let mut ui = open("");
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), vec![Message::Close, Message::Run("sidebar")]);

    let mut ui = open("");
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), vec![Message::Close, Message::Run("settings")]);
}

#[test]
fn a_click_closes_and_runs_the_command() {
    let mut ui = open("");
    // The key cap of the "G H" shortcut sits inside the "Go home" row.
    ui.click("H").expect("command row");

    assert_eq!(messages(ui), vec![Message::Close, Message::Run("home")]);
}

#[test]
fn escape_on_an_empty_field_closes() {
    let mut ui = open("");
    ui.tap_key(Named::Escape);

    assert_eq!(messages(ui), vec![Message::Close]);
}

#[test]
fn escape_with_a_query_clears_it_first() {
    let mut ui = open("file");
    ui.tap_key(Named::Escape);

    assert_eq!(messages(ui), vec![Message::Query(String::new())]);
}

#[test]
fn a_click_on_the_scrim_closes_without_reaching_the_view_behind() {
    let mut closed = simulator::Simulator::with_size(support::settings(), SIZE, view(false, ""));
    let behind = closed.find("Content").expect("content button").bounds();

    let mut ui = open("");
    ui.point_at(behind.center());
    ui.simulate(simulator::click());
    ui.point_at(Point::new(SIZE.0 / 2.0, SIZE.1 - 4.0));
    ui.simulate(simulator::click());

    assert_eq!(messages(ui), vec![Message::Close, Message::Close]);
}

#[test]
fn tab_cycles_inside_the_palette() {
    let mut ui = open("");
    let field = ui
        .find(selector::is_focused())
        .expect("focused field")
        .bounds();

    // The field, then the five command rows: the sixth Tab returns to the
    // field instead of leaving the palette.
    for _ in 0..6 {
        ui.tap_key(Named::Tab);
        assert!(
            ui.find(selector::is_focused()).is_ok(),
            "the focus stays inside"
        );
    }

    let focused = ui.find(selector::is_focused()).expect("focus").bounds();
    assert_eq!(focused, field);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), vec![Message::Close, Message::Run("new")]);
}

#[test]
fn shortcut_labels_use_the_notation_of_the_system() {
    let mut ui = open("sidebar");

    for key in ["Ctrl", "Shift", "B"] {
        assert!(ui.find(key).is_ok(), "{key} key cap");
    }
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = open("file");
    ui.tap_key(Named::ArrowDown);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn command_palette_light() {
    snapshot(&Theme::light(), "command_palette_light");
}

#[test]
fn command_palette_dark() {
    snapshot(&Theme::dark(), "command_palette_dark");
}

#[test]
fn command_palette_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "command_palette_light_high_contrast");
}

fn long_view() -> Element<'static, Message> {
    let commands: Vec<Command<Message>> = (0..30)
        .map(|index| {
            Command::new(
                format!("c{index}"),
                format!("Command {index}"),
                Message::Content,
            )
            .with_description(format!("Item {index}"))
        })
        .collect();
    let refs: Vec<&Command<Message>> = commands.iter().collect();

    command_palette::command_palette(
        text("Behind"),
        &Localizations::en(),
        &refs,
        "",
        Message::Query,
        Message::Close,
    )
    .open(true)
    .into()
}

#[test]
fn the_highlighted_command_scrolls_into_view() {
    let mut ui = simulator::Simulator::with_size(support::settings(), SIZE, long_view());
    let first = ui.find("Item 0").expect("first row");
    assert!(first.visible_bounds().is_some());
    let last = ui.find("Item 29").expect("last row");
    assert!(
        last.visible_bounds().is_none(),
        "the last row starts hidden"
    );

    ui.tap_key(Named::ArrowUp);
    let last = ui.find("Item 29").expect("last row");
    assert!(
        last.visible_bounds().is_some(),
        "the last row scrolled into view"
    );

    ui.tap_key(Named::ArrowDown);
    let first = ui.find("Item 0").expect("first row");
    assert!(
        first.visible_bounds().is_some(),
        "the first row scrolled back"
    );
}

#[test]
fn the_rows_follow_the_density() {
    for (density, height) in [(Density::Pointer, 52.0), (Density::Touch, 72.0)] {
        density::set(density);
        // Without a query the field has no clear button: Tab reaches the row.
        let mut ui = open("");
        ui.tap_key(Named::Tab);
        let row = ui.find(selector::is_focused()).expect("first row").bounds();

        assert!(row.height >= height, "{density:?}: {row:?}");
    }

    density::set(Density::Pointer);
}
