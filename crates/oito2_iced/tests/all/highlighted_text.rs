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

//! Highlighted text: the layout of the layers, the line limit and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{column, container, text};
use iced_test::{selector, simulator};
use oito2_iced::theme::text::Class;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, typography};
use oito2_iced::widget::highlighted_text::{self, highlighted_text};
use oito2_iced::{Element, Theme};

const LONG: &str = "Configurações de ação: the ark sails while the UI kits build \
                    the configuration of every action";

fn height(element: Element<'static, ()>) -> f32 {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (200.0, 400.0),
        container(element).id("probe"),
    );

    ui.find(selector::id("probe"))
        .expect("probe exists")
        .bounds()
        .height
}

#[test]
fn highlighted_text_is_as_tall_as_plain_text() {
    let plain = height(
        text(LONG)
            .size(typography::BODY_MEDIUM.size)
            .line_height(iced::widget::text::LineHeight::Absolute(
                typography::BODY_MEDIUM.line_height.into(),
            ))
            .font(oito2_iced::font::INTER)
            .into(),
    );
    let highlighted = height(highlighted_text(LONG, "the ark").into());

    assert!(plain > typography::BODY_MEDIUM.line_height * 2.0);
    assert!((highlighted - plain).abs() <= typography::BODY_MEDIUM.line_height);
}

#[test]
fn max_lines_clips_the_text() {
    let unlimited = height(highlighted_text(LONG, "acao").into());
    let two = height(highlighted_text(LONG, "acao").max_lines(2).into());

    assert!(unlimited > 2.0 * typography::BODY_MEDIUM.line_height);
    assert!((two - 2.0 * typography::BODY_MEDIUM.line_height).abs() < 0.5);
}

#[test]
fn text_without_matches_draws_like_text_with_an_empty_query() {
    let draw = |query: &'static str| {
        let mut ui = simulator::Simulator::with_size(
            support::settings(),
            (200.0, 40.0),
            Element::<()>::from(highlighted_text("Show hidden files", query)),
        );

        support::pixels(&ui.snapshot(&Theme::light()).expect("snapshot"), query)
    };

    let empty = draw("");
    let unmatched = draw("zzz");
    let matched = draw("hid");

    assert_eq!(support::difference(&empty, &unmatched), 0.0);
    assert!(support::difference(&empty, &matched) > 0.0);
}

fn catalog() -> Element<'static, ()> {
    column![
        highlighted_text("Show hidden files", "hid"),
        highlighted_text("Configurações de ação", "acao CONFIG"),
        highlighted_text("Ark UI builds UI kits; the ark sails", "ark").queries(["kits"]),
        highlighted_text("Ark UI builds UI kits; the ark sails", "ui").whole_words(true),
        highlighted_text("Ark UI builds UI kits; the ark sails", "ui kits")
            .split_words(false)
            .class(Class::Variant),
        highlighted_text("A title with a match", "match").style(typography::TITLE_MEDIUM),
        highlighted_text(LONG, "the").max_lines(1).width(320),
        highlighted_text("No match here", "zzz"),
    ]
    .spacing(8)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 280.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn highlighted_text_light() {
    snapshot(&Theme::light(), "highlighted_text_light");
}

#[test]
fn highlighted_text_dark() {
    snapshot(&Theme::dark(), "highlighted_text_dark");
}

#[test]
fn highlighted_text_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "highlighted_text_light_high_contrast");
}

#[test]
fn the_matches_are_byte_ranges_of_the_text() {
    let ranges = highlighted_text::matches(
        "Show hidden files",
        "hid",
        &[],
        highlighted_text::Options::default(),
    );

    assert_eq!(ranges, vec![5..8]);
}
