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

//! Icons: the freedesktop.org name lookup, the generated Lucide sources, the
//! icon widget sizes per density, and golden snapshots of icons and the
//! placeholder. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::collections::HashSet;

use iced::widget::{column, container, row, text};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::{self, Freedesktop, lucide};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::tokens::spacing;
use oito2_iced::{Element, Theme};

#[test]
fn freedesktop_names_resolve_with_suffixes_and_fallbacks() {
    assert_eq!(
        Freedesktop::by_name("document-open"),
        Some(Freedesktop::DocumentOpen)
    );
    assert_eq!(
        Freedesktop::by_name("document-open-symbolic"),
        Some(Freedesktop::DocumentOpen)
    );
    assert_eq!(Freedesktop::by_name("no-such-icon"), None);
    assert_eq!(
        Freedesktop::by_name("go-next-rtl-symbolic"),
        Freedesktop::by_name("go-next")
    );
    assert_eq!(
        Freedesktop::by_name("document-save-as-template"),
        Freedesktop::by_name("document-save-as")
    );
    assert_eq!(
        Freedesktop::by_name("folder-documents"),
        Some(Freedesktop::Folder)
    );
}

#[test]
fn freedesktop_names_are_unique_and_map_back() {
    let names: HashSet<&str> = Freedesktop::ALL.iter().map(|icon| icon.name()).collect();

    assert_eq!(names.len(), Freedesktop::ALL.len());

    for icon in Freedesktop::ALL {
        let name = icon.name();

        assert!(
            name.split('-').all(|part| !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())),
            "{name}"
        );
        assert_eq!(Freedesktop::by_name(name), Some(icon), "{name}");
    }
}

#[test]
fn lucide_sources_are_complete_svg_documents() {
    for icon in [
        lucide::FOLDER_OPEN,
        lucide::X,
        lucide::A_ARROW_DOWN,
        Freedesktop::DialogWarning.icon(),
    ] {
        let svg = icon.svg();

        assert!(svg.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg.contains("stroke=\"currentColor\""));
        assert!(svg.ends_with("</svg>"));
        assert!(svg.len() > 200, "{}", icon.name());
    }

    assert_eq!(lucide::FOLDER_OPEN.name(), "folder-open");
    assert_eq!(lucide::VERSION, "1.70.0");
}

fn sized() -> Element<'static, ()> {
    container(icon::icon(lucide::HEART)).id("icon").into()
}

#[test]
fn icons_take_the_icon_size_of_the_density() {
    for (density, size) in [(Density::Pointer, 20.0), (Density::Touch, 24.0)] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), sized());
        let bounds = ui.find(selector::id("icon")).expect("icon").bounds();

        assert_eq!((bounds.width, bounds.height), (size, size), "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, ()> {
    let icons = row(Freedesktop::ALL
        .iter()
        .take(12)
        .map(|icon| icon::icon(icon.icon()).into()))
    .spacing(spacing::SM);

    column![
        icons,
        container(
            row![
                icon::icon(lucide::CIRCLE_CHECK),
                text("On the inverse surface")
            ]
            .spacing(spacing::SM)
        )
        .padding(spacing::SM)
        .class(ContainerClass::Inverse),
        row![
            icon::placeholder(),
            icon::placeholder().size(32.0),
            icon::icon(lucide::STAR).size(32.0),
        ]
        .spacing(spacing::SM)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(spacing::MD)
    .padding(spacing::MD)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 160.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn icons_light() {
    snapshot(&Theme::light(), "icons_light");
}

#[test]
fn icons_dark() {
    snapshot(&Theme::dark(), "icons_dark");
}
