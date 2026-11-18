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

//! Drop zones: the title, clicks and keys, the window file events, the
//! drag tint, the cursor and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::path::PathBuf;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::container;
use iced::{Event, Point, Size, mouse, window};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::{Element, Theme, focus};
use oito2_iced_files::drop_zone::{DropZone, drop_zone};
use oito2_iced_files::file::File;

use crate::temp::TempDir;

const SIZE: Size = Size::new(400.0, 240.0);

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Choose,
    Files(Vec<File>),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// A zone that takes clicks and drops.
fn zone<'a>() -> DropZone<'a, Message> {
    drop_zone(&Localizations::en())
        .accepts_drops(true)
        .on_choose(Message::Choose)
        .on_drop(Message::Files)
        .id("zone")
}

fn view(zone: DropZone<'static, Message>) -> Element<'static, Message> {
    focus::navigation(container(zone).padding(16)).into()
}

fn open(zone: DropZone<'static, Message>) -> Simulator<'static> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(zone))
}

fn zone_bounds(ui: &mut Simulator<'_>) -> iced::Rectangle {
    ui.find(selector::id("zone")).expect("zone").bounds()
}

fn file_event(ui: &mut Simulator<'_>, event: window::Event) {
    ui.simulate([Event::Window(event)]);
}

fn redraw(ui: &mut Simulator<'_>) {
    file_event(ui, window::Event::RedrawRequested(Instant::now()));
}

/// The color inside the top start corner of the zone, off the border.
fn fill(ui: &mut Simulator<'_>) -> iced::Color {
    let bounds = zone_bounds(ui);
    let image = support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), "fill");

    support::pixel_at(
        &image,
        bounds.position() + iced::Vector::new(24.0, 24.0),
        SIZE.width,
    )
}

#[test]
fn the_title_invites_drops_or_a_click() {
    let mut ui = open(zone());
    assert!(ui.find("Drop files here").is_ok());
    assert!(ui.find("Choose files").is_ok());

    let mut ui = open(
        drop_zone(&Localizations::en())
            .multiple(false)
            .on_choose(Message::Choose),
    );
    assert!(ui.find("Choose a file").is_ok());
    assert!(ui.find("Drop files here").is_err());

    // Without drops on this platform the zone only invites a click.
    let mut ui = open(zone().accepts_drops(false));
    assert!(ui.find("Drop files here").is_err());
    assert!(ui.find("Choose files").is_ok());

    let mut ui = open(
        drop_zone(&Localizations::pt_br())
            .accepts_drops(true)
            .on_drop(Message::Files),
    );
    assert!(ui.find("Solte os arquivos aqui").is_ok());
}

#[test]
fn a_click_publishes_the_choose_message() {
    let mut ui = open(zone());

    let bounds = zone_bounds(&mut ui);
    ui.point_at(bounds.center());
    ui.simulate(simulator::click());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Choose]);
}

#[test]
fn enter_and_space_activate_the_focused_zone() {
    let mut ui = open(zone());

    ui.tap_key(Named::Tab);
    assert!(ui.find(selector::is_focused()).is_ok());
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Choose, Message::Choose]);
}

#[test]
fn a_disabled_zone_ignores_clicks_keys_and_drops() {
    let directory = TempDir::new("oito2_zone_disabled");
    let file = directory.file("a.txt", b"a");
    let mut ui = open(zone().enabled(false));

    let bounds = zone_bounds(&mut ui);
    ui.point_at(bounds.center());
    ui.simulate(simulator::click());
    ui.tap_key(Named::Tab);
    assert!(ui.find(selector::is_focused()).is_err());
    ui.tap_key(Named::Enter);
    file_event(&mut ui, window::Event::FileDropped(file));
    redraw(&mut ui);

    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn hovered_files_tint_the_zone_until_they_leave() {
    let mut ui = open(zone());
    let idle = fill(&mut ui);

    file_event(&mut ui, window::Event::FileHovered(PathBuf::from("a.png")));
    file_event(&mut ui, window::Event::FileHovered(PathBuf::from("b.png")));
    let dragging = fill(&mut ui);
    assert_ne!(idle, dragging);

    file_event(&mut ui, window::Event::FilesHoveredLeft);
    assert_eq!(fill(&mut ui), idle);
}

#[test]
fn a_zone_without_a_drop_message_is_not_tinted() {
    let mut ui = open(
        drop_zone(&Localizations::en())
            .accepts_drops(true)
            .on_choose(Message::Choose)
            .id("zone"),
    );
    let idle = fill(&mut ui);

    file_event(&mut ui, window::Event::FileHovered(PathBuf::from("a.png")));
    assert_eq!(fill(&mut ui), idle);
}

#[test]
fn dropped_files_arrive_together_on_the_next_redraw() {
    let directory = TempDir::new("oito2_zone_drop");
    let file = directory.file("dot.png", &[1, 2, 3]);
    let folder = directory.folder("photos");
    let mut ui = open(zone());

    file_event(&mut ui, window::Event::FileHovered(file.clone()));
    file_event(&mut ui, window::Event::FileDropped(file.clone()));
    file_event(&mut ui, window::Event::FileDropped(folder.clone()));
    file_event(
        &mut ui,
        window::Event::FileDropped(directory.path().join("missing.txt")),
    );
    let after_drop = fill(&mut ui);
    redraw(&mut ui);
    redraw(&mut ui);

    let messages: Vec<Message> = ui.into_messages().collect();
    let [Message::Files(files)] = messages.as_slice() else {
        unreachable!("one drop message expected, got {messages:?}");
    };
    let summary: Vec<(&str, u64, bool)> = files
        .iter()
        .map(|file| (file.name.as_str(), file.size, file.directory))
        .collect();
    assert_eq!(summary, [("dot.png", 3, false), ("photos", 0, true)]);
    assert_eq!(files[0].path, file);
    assert_eq!(files[1].path, folder);

    // The drop ends the drag.
    let mut idle = open(zone());
    assert_eq!(after_drop, fill(&mut idle));
}

#[test]
fn the_zone_shows_a_pointer_while_it_takes_clicks() {
    let _fonts = open(zone());
    let center = Point::new(SIZE.width / 2.0, SIZE.height / 2.0);

    assert_eq!(
        support::mouse_interaction(view(zone()), SIZE, center),
        mouse::Interaction::Pointer
    );
    assert_ne!(
        support::mouse_interaction(view(zone().enabled(false)), SIZE, center),
        mouse::Interaction::Pointer
    );
}

fn snapshot(theme: &Theme, name: &str, hovered: bool) {
    let content = view(zone().hint("PNG or JPEG, up to 5 MB").min_height(180.0));
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 212.0), content);

    if hovered {
        file_event(&mut ui, window::Event::FileHovered(PathBuf::from("a.png")));
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");
    support::golden!(&snapshot, name);
}

#[test]
fn drop_zone_idle_light() {
    snapshot(&Theme::light(), "drop_zone_idle_light", false);
}

#[test]
fn drop_zone_hovered_light() {
    snapshot(&Theme::light(), "drop_zone_hovered_light", true);
}

#[test]
fn drop_zone_hovered_dark() {
    snapshot(&Theme::dark(), "drop_zone_hovered_dark", true);
}

#[test]
fn drop_zone_disabled_dark() {
    let content = view(
        zone()
            .enabled(false)
            .hint("PNG or JPEG, up to 5 MB")
            .min_height(180.0),
    );
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 212.0), content);

    let snapshot = ui.snapshot(&Theme::dark()).expect("snapshot");
    support::golden!(&snapshot, "drop_zone_disabled_dark");
}
