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

//! File uploads: the listed files with their sizes, the refusals, the close
//! and remove buttons, the full upload, drops into the controller and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::container;
use iced::{Event, Size, window};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::{Element, Theme, focus};
use oito2_iced_files::file::{File, TypeGroup};
use oito2_iced_files::upload::{Controller, Event as UploadEvent, FileUpload, file_upload};

use crate::temp::TempDir;

const SIZE: Size = Size::new(420.0, 520.0);

type Simulator<'a> = simulator::Simulator<'a, UploadEvent, Theme>;

fn images() -> TypeGroup {
    TypeGroup::new()
        .label("Images")
        .extensions(["png", "jpg"])
        .mime_types(["image/*"])
}

fn upload(controller: &Controller) -> FileUpload<'_, UploadEvent> {
    file_upload(controller, &Localizations::en())
        .accepts_drops(true)
        .on_event(std::convert::identity)
}

fn view(upload: FileUpload<'_, UploadEvent>) -> Element<'_, UploadEvent> {
    focus::navigation(container(upload).padding(16)).into()
}

fn open(upload: FileUpload<'_, UploadEvent>) -> Simulator<'_> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(upload))
}

/// Moves the focus `tabs` times and presses Enter.
fn activate(ui: &mut Simulator<'_>, tabs: usize) {
    for _ in 0..tabs {
        ui.tap_key(Named::Tab);
    }

    ui.tap_key(Named::Enter);
}

#[test]
fn lists_files_with_their_size_and_the_refusals() {
    let mut controller = Controller::new().accept([images()]).max_file_size(1000);
    controller.add([File::new("dot.png", 70), File::new("notes.pdf", 10)]);

    let mut ui = open(upload(&controller));

    assert!(ui.find("Accepted: Images · Up to 1 kB each").is_ok());
    assert!(ui.find("dot.png").is_ok());
    assert!(ui.find("70 B").is_ok());
    assert!(ui.find("notes.pdf: this file type is not accepted").is_ok());
    assert!(ui.find("notes.pdf").is_err());
}

#[test]
fn the_close_and_remove_buttons_publish_their_events() {
    let mut controller = Controller::new().accept([images()]);
    controller.add([File::new("dot.png", 70), File::new("notes.pdf", 10)]);

    // The zone, the close button of the refusals, the remove button.
    let mut ui = open(upload(&controller));
    activate(&mut ui, 2);
    let messages: Vec<UploadEvent> = ui.into_messages().collect();
    assert_eq!(messages, vec![UploadEvent::ClearRejected]);

    let mut ui = open(upload(&controller));
    activate(&mut ui, 3);
    let messages: Vec<UploadEvent> = ui.into_messages().collect();
    assert_eq!(messages, vec![UploadEvent::Remove(0)]);

    let mut ui = open(upload(&controller));
    activate(&mut ui, 1);
    let messages: Vec<UploadEvent> = ui.into_messages().collect();
    assert_eq!(messages, vec![UploadEvent::Choose]);
}

#[test]
fn a_full_upload_disables_its_zone() {
    let mut controller = Controller::new().max_files(1);
    controller.add([File::new("a.txt", 1)]);

    let mut ui = open(upload(&controller));
    assert!(ui.find("Choose a file").is_ok());

    let zone = ui.find("Drop files here").expect("zone title").bounds();
    ui.point_at(zone.center());
    ui.simulate(simulator::click());

    // Tab skips the zone and reaches the remove button.
    activate(&mut ui, 1);

    let messages: Vec<UploadEvent> = ui.into_messages().collect();
    assert_eq!(messages, vec![UploadEvent::Remove(0)]);
}

#[test]
fn a_disabled_upload_has_no_remove_buttons() {
    let controller = Controller::new().files([File::new("a.txt", 1)]);

    let mut ui = open(upload(&controller).enabled(false));
    activate(&mut ui, 1);

    assert!(ui.find(selector::is_focused()).is_err());
    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn dropped_files_go_to_the_controller() {
    let directory = TempDir::new("oito2_upload_drop");
    let file = directory.file("dot.png", &[1, 2, 3]);
    let folder = directory.folder("photos");
    let mut controller = Controller::new();

    let messages: Vec<UploadEvent> = {
        let mut ui = open(upload(&controller));
        ui.simulate([
            Event::Window(window::Event::FileDropped(file.clone())),
            Event::Window(window::Event::FileDropped(folder)),
        ]);
        ui.simulate([Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )]);
        ui.into_messages().collect()
    };

    for message in messages {
        let _ = controller.update(message);
    }

    assert_eq!(controller.accepted().len(), 1);
    assert_eq!(controller.accepted()[0].path, file);
    assert_eq!(
        controller.reason(&controller.rejected()[0], &Localizations::en()),
        "photos: folders are not accepted"
    );

    let mut ui = open(upload(&controller));
    assert!(ui.find("photos: folders are not accepted").is_ok());
}

/// A controller with an image preview, a plain file and a refusal.
fn catalog_controller(directory: &TempDir) -> Controller {
    let mut pixels = Vec::new();
    for y in 0..80u8 {
        for x in 0..80u8 {
            pixels.extend_from_slice(&[x * 3, 96, y * 3, 255]);
        }
    }
    let png = directory.path().join("gradient.png");
    support::write_png(
        &png,
        &support::Image {
            width: 80,
            height: 80,
            rgba: pixels,
            file_name: String::new(),
        },
    );

    let mut controller = Controller::new()
        .accept([images(), TypeGroup::new().label("PDF").extensions(["pdf"])])
        .max_files(3)
        .max_file_size(5_000_000);
    controller.add([
        File::from_path(&png).expect("the image exists"),
        File::new("report.pdf", 1_250_000),
        File::new("archive.zip", 10),
    ]);
    controller
}

fn snapshot(theme: &Theme, name: &str) {
    let directory = TempDir::new("oito2_upload_golden");
    let controller = catalog_controller(&directory);
    let content = view(upload(&controller).zone_min_height(120.0));
    let mut ui = simulator::Simulator::with_size(support::settings(), (420.0, 390.0), content);

    let snapshot = ui.snapshot(theme).expect("snapshot");
    support::golden!(&snapshot, name);
}

#[test]
fn file_upload_light() {
    snapshot(&Theme::light(), "file_upload_light");
}

#[test]
fn file_upload_dark() {
    snapshot(&Theme::dark(), "file_upload_dark");
}
