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

//! Avatar: initials, the color picked from the name, the localized
//! description, the group overflow and geometry, the picture of the `image`
//! feature and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{column, container, row};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::avatar::{self, Shape, Status};
use oito2_iced::{Element, Theme};

type Simulator<'a> = simulator::Simulator<'a, (), Theme>;

#[test]
fn initials_take_the_first_letters_of_the_first_and_last_words() {
    assert_eq!(avatar::initials("Ana Maria Souza"), "AS");
    assert_eq!(avatar::initials("  joão "), "J");
    assert_eq!(avatar::initials(""), "");
    assert_eq!(avatar::initials("   "), "");
    // Decomposed accents and emoji sequences stay whole.
    assert_eq!(avatar::initials("e\u{301}lise"), "E\u{301}");
    assert_eq!(
        avatar::initials("\u{1F469}\u{200D}\u{1F4BB} dev"),
        "\u{1F469}\u{200D}\u{1F4BB}D"
    );
    assert_eq!(
        avatar::initials("\u{1F1E7}\u{1F1F7} brasil"),
        "\u{1F1E7}\u{1F1F7}B"
    );
}

#[test]
fn the_color_comes_from_a_stable_hash_of_the_name() {
    assert_eq!(
        avatar::color_for("Ana Souza"),
        avatar::color_for("Ana Souza")
    );
    assert!(
        Accent::ALL
            .iter()
            .any(|accent| accent.color() == avatar::color_for("Bruno Lima"))
    );
    // "A" is code unit 65, and 65 % 9 picks the third accent.
    assert_eq!(avatar::color_for("A"), Accent::Green.color());
}

#[test]
fn the_description_reads_the_name_and_the_status() {
    let en = Localizations::en();
    let pt = Localizations::pt_br();
    let busy = avatar::avatar().name("Ana Souza").status(Status::Busy);

    assert_eq!(busy.description(&en), "Ana Souza, Busy");
    assert_eq!(busy.description(&pt), "Ana Souza, Ocupado");
    assert_eq!(
        avatar::description(None, Some(Status::Away), &pt),
        "Ausente"
    );
    assert_eq!(avatar::description(Some("Eva"), None, &en), "Eva");
    assert_eq!(avatar::more_description(2, &en), "2 more");
    assert_eq!(avatar::more_description(2, &pt), "Mais 2");
}

#[test]
fn an_avatar_shows_its_initials_at_its_size() {
    let content: Element<'_, ()> = container(
        avatar::avatar()
            .name("Ana Souza")
            .status(Status::Busy)
            .size(48.0),
    )
    .id("avatar")
    .into();
    let mut ui = Simulator::with_settings(support::settings(), content);

    assert!(ui.find("AS").is_ok());

    let bounds = ui.find(selector::id("avatar")).expect("avatar").bounds();
    assert_eq!((bounds.width, bounds.height), (48.0, 48.0));
}

#[test]
fn a_group_shows_the_extra_count_and_overlaps() {
    let names = ["A B", "C D", "E F", "G H"];
    let group = avatar::group(names.map(|name| avatar::avatar().name(name).size(32.0))).max(2);

    assert_eq!(group.hidden(), 2);

    let content: Element<'_, ()> = container(group).id("group").into();
    let mut ui = Simulator::with_settings(support::settings(), content);

    assert!(ui.find("+2").is_ok());
    assert!(ui.find("AB").is_ok());
    assert!(ui.find("CD").is_ok());
    assert!(ui.find("EF").is_err());

    // Three items of 36 (32 and the rings) each 70 % past the previous.
    let bounds = ui.find(selector::id("group")).expect("group").bounds();
    assert!((bounds.width - (36.0 * 0.7 * 2.0 + 36.0)).abs() < 1e-3);
    assert_eq!(bounds.height, 36.0);
}

#[test]
fn a_group_without_overflow_has_no_bubble() {
    let group = avatar::group(["Ana", "Bia"].map(|name| avatar::avatar().name(name)))
        .overlap(0.0)
        .size(40.0);

    assert_eq!(group.hidden(), 0);

    let content: Element<'_, ()> = container(group).id("group").into();
    let mut ui = Simulator::with_settings(support::settings(), content);

    assert!(ui.find("+0").is_err());
    let bounds = ui.find(selector::id("group")).expect("group").bounds();
    assert_eq!(bounds.width, 88.0);
}

#[cfg(feature = "image")]
#[test]
fn a_picture_covers_the_fallback() {
    use iced::Point;
    use iced::widget::image::Handle;

    let red = [0xE0_u8, 0x1B, 0x24, 0xFF].repeat(4 * 2);
    let content: Element<'_, ()> = container(
        avatar::avatar()
            .name("Ana Souza")
            .image(Handle::from_rgba(4, 2, red))
            .size(40.0),
    )
    .padding(10)
    .into();
    let mut ui = Simulator::with_size(support::settings(), (60.0, 60.0), content);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");
    let image = support::pixels(&snapshot, "avatar_picture");
    let center = support::pixel_at(&image, Point::new(30.0, 30.0), 60.0);
    // The wide picture is cropped, so it also covers the top edge.
    let top = support::pixel_at(&image, Point::new(30.0, 11.0), 60.0);

    for color in [center, top] {
        let [r, g, b, _] = color.into_rgba8();
        assert_eq!((r, g, b), (0xE0, 0x1B, 0x24));
    }
}

fn catalog() -> Element<'static, ()> {
    let names = ["Ana Souza", "Bruno Lima", "Caio", "Dani", "Eva"];

    column![
        row![
            avatar::avatar().name("Ana Souza").status(Status::Online),
            avatar::avatar()
                .name("Bruno Lima")
                .shape(Shape::Square)
                .status(Status::Busy),
            avatar::avatar(),
            avatar::group(names.map(|name| avatar::avatar().name(name).size(32.0))).max(3),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        row![
            avatar::avatar()
                .name("Away")
                .size(24.0)
                .status(Status::Away),
            avatar::avatar()
                .name("Offline User")
                .size(32.0)
                .status(Status::Offline),
            avatar::avatar()
                .size(56.0)
                .shape(Shape::Square)
                .status(Status::Online),
            avatar::avatar()
                .name("Custom")
                .background(iced::Color::from_rgb8(0x2E, 0xC2, 0x7E))
                .size(64.0),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        avatar::group(
            names.map(|name| { avatar::avatar().name(name).shape(Shape::Square).size(40.0) })
        )
        .shape(Shape::Square)
        .size(40.0)
        .max(4),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (340.0, 220.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn avatar_light() {
    snapshot(&Theme::light(), "avatar_light");
}

#[test]
fn avatar_dark() {
    snapshot(&Theme::dark(), "avatar_dark");
}

#[test]
fn avatar_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "avatar_light_high_contrast");
}
