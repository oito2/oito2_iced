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

//! Download triggers: the press message, the busy and unsupported states
//! and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh
//! the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::Size;
use iced::widget::{column, row};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::{Element, Theme};
use oito2_iced_files::download::{DownloadTrigger, download_trigger};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Download,
}

fn trigger<'a>() -> DownloadTrigger<'a, Message> {
    download_trigger(&Localizations::en())
        .on_press(Message::Download)
        .id("trigger")
}

fn clicks(trigger: DownloadTrigger<'static, Message>) -> usize {
    let content: Element<'static, Message> = column![trigger].padding(16).into();
    let mut ui = simulator::Simulator::with_size(support::settings(), (300.0, 120.0), content);

    let bounds = ui.find(selector::id("trigger")).expect("trigger").bounds();
    ui.point_at(bounds.center());
    ui.simulate(simulator::click());

    ui.into_messages().count()
}

#[test]
fn a_click_publishes_the_download_message() {
    assert_eq!(clicks(trigger()), 1);
}

#[test]
fn busy_and_unsupported_triggers_ignore_clicks() {
    assert_eq!(clicks(trigger().busy(true)), 0);
    assert_eq!(clicks(trigger().supported(false)), 0);
    assert_eq!(
        clicks(download_trigger(&Localizations::en()).id("trigger")),
        0
    );
}

#[test]
fn the_label_is_localized_or_custom() {
    let content: Element<'static, Message> = column![
        trigger(),
        download_trigger(&Localizations::pt_br()),
        trigger().label("Export CSV"),
    ]
    .into();
    let mut ui = simulator::Simulator::with_settings(support::settings(), content);

    assert!(ui.find("Download").is_ok());
    assert!(ui.find("Baixar").is_ok());
    assert!(ui.find("Export CSV").is_ok());
}

fn snapshot(theme: &Theme, name: &str) {
    let content: Element<'static, Message> = row![
        trigger(),
        trigger().label("Export CSV"),
        download_trigger(&Localizations::en()),
    ]
    .spacing(8)
    .padding(16)
    .into();
    let mut ui =
        simulator::Simulator::with_size(support::settings(), Size::new(480.0, 70.0), content);

    let snapshot = ui.snapshot(theme).expect("snapshot");
    support::golden!(&snapshot, name);
}

#[test]
fn download_trigger_light() {
    snapshot(&Theme::light(), "download_trigger_light");
}

#[test]
fn download_trigger_dark() {
    snapshot(&Theme::dark(), "download_trigger_dark");
}
