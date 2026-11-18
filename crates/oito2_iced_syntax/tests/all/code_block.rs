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

//! Code blocks: the copy button, line numbers, plain text for unknown
//! languages, recoloring on a theme change, the width and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard::{Clipboard, Kind};
use iced::advanced::renderer::Headless;
use iced::{Event, Length, Point, Settings, Size, mouse};
use iced_test::runtime::user_interface::{self, UserInterface};
use iced_test::simulator::Simulator;
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{sizes, spacing};
use oito2_iced::{Element, Theme};
use oito2_iced_syntax::code_block::{self, CodeBlock};
use oito2_iced_syntax::font;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Copied,
}

/// The code of the golden snapshots of `oito2_ui`.
const CODE: &str = "@override
String greet(String name, {int times = 2}) {
  final text = 'Hello, $name!'; // 42
  return List.filled(times, text).join(' ');
}";

/// The simulator settings with Inter and JetBrains Mono.
fn settings() -> Settings {
    let mut settings = support::settings();
    settings
        .fonts
        .extend(font::JETBRAINS_MONO_FONTS.into_iter().map(Into::into));
    settings
}

fn block(code: &str) -> CodeBlock<'_, Message> {
    code_block::code_block(&Localizations::en(), code)
}

/// The center of the copy button of a block at the origin `width` wide.
fn copy_button_center(width: f32) -> Point {
    let half = sizes::BUTTON_HEIGHT / 2.0;
    Point::new(width - spacing::XS - half, spacing::XS + half)
}

#[derive(Default)]
struct Recording(Option<String>);

impl Clipboard for Recording {
    fn read(&self, _kind: Kind) -> Option<String> {
        None
    }

    fn write(&mut self, kind: Kind, contents: String) {
        assert_eq!(kind, Kind::Standard);
        self.0 = Some(contents);
    }
}

/// Clicks at `point` of `element` in a 400 by 200 window and returns what
/// was written to the clipboard and the published messages.
fn click(element: Element<'_, Message>, point: Point) -> (Option<String>, Vec<Message>) {
    // Loads the fonts of the settings into the text stack.
    let empty: Element<'_, Message> = block("").into();
    let _ = Simulator::with_settings(settings(), empty);

    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let mut renderer =
        iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
            oito2_iced::font::INTER,
            iced::Pixels(16.0),
            backend.as_deref(),
        ))
        .expect("headless renderer");

    let mut ui = UserInterface::build(
        element,
        Size::new(400.0, 200.0),
        user_interface::Cache::default(),
        &mut renderer,
    );

    let mut clipboard = Recording::default();
    let mut messages = Vec::new();

    let _ = ui.update(
        &[
            Event::Mouse(mouse::Event::CursorMoved { position: point }),
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        ],
        mouse::Cursor::Available(point),
        &mut renderer,
        &mut clipboard,
        &mut messages,
    );

    (clipboard.0, messages)
}

#[test]
fn the_copy_button_copies_the_whole_code() {
    let code = "fn main() {\n    println!(\"hi\");\n}";
    let element = block(code)
        .language("rust")
        .on_copied(Message::Copied)
        .width(300)
        .into();

    let (copied, messages) = click(element, copy_button_center(300.0));

    assert_eq!(copied.as_deref(), Some(code));
    assert_eq!(messages, vec![Message::Copied]);
}

#[test]
fn the_copy_button_can_be_hidden() {
    let element = block("print(1)")
        .copy_button(false)
        .on_copied(Message::Copied)
        .width(300)
        .into();

    let (copied, messages) = click(element, copy_button_center(300.0));

    assert_eq!(copied, None);
    assert!(messages.is_empty());
}

#[test]
fn line_numbers_count_every_line() {
    let numbered: Element<'_, Message> = block("a\nb\nc").line_numbers(true).into();
    let mut ui = Simulator::with_settings(settings(), numbered);
    assert!(ui.find("1\n2\n3").is_ok());

    let plain: Element<'_, Message> = block("a\nb\nc").into();
    let mut ui = Simulator::with_settings(settings(), plain);
    assert!(ui.find("1\n2\n3").is_err());
}

fn pixels(element: Element<'_, Message>, theme: &Theme, name: &str) -> support::Image {
    let mut ui = Simulator::with_size(settings(), (300.0, 80.0), element);
    support::pixels(&ui.snapshot(theme).expect("snapshot"), name)
}

#[test]
fn unknown_languages_show_plain_text() {
    let code = "fn main() { let x = 42; }";
    let theme = Theme::dark();

    let plain = pixels(block(code).into(), &theme, "plain");
    let unknown = pixels(block(code).language("cobol").into(), &theme, "unknown");
    let plaintext = pixels(
        block(code).language("plaintext").into(),
        &theme,
        "plaintext",
    );
    let rust = pixels(block(code).language("rust").into(), &theme, "rust");

    assert!(support::difference(&plain, &unknown) == 0.0);
    assert!(support::difference(&plain, &plaintext) == 0.0);
    assert!(support::difference(&plain, &rust) > 0.0);
}

#[test]
fn a_theme_change_recolors_the_code() {
    let view = || block("let x = 42; // answer").language("rust").into();

    let mut ui = Simulator::with_size(settings(), (300.0, 80.0), view());
    let _ = ui.snapshot(&Theme::light()).expect("light snapshot");
    let redrawn = support::pixels(&ui.snapshot(&Theme::dark()).expect("snapshot"), "redrawn");

    let fresh = pixels(view(), &Theme::dark(), "fresh");

    assert!(support::difference(&redrawn, &fresh) == 0.0);
}

#[test]
fn the_block_is_as_wide_as_its_code_up_to_the_available_width() {
    let framed = |code| -> Element<'_, Message> {
        iced::widget::container(block(code).copy_button(false))
            .id("block")
            .into()
    };
    let long_code = "x".repeat(200);
    let short = Simulator::with_size(settings(), (400.0, 100.0), framed("x"));
    let long = Simulator::with_size(settings(), (400.0, 100.0), framed(&long_code));

    for (mut ui, expected) in [(short, None), (long, Some(400.0))] {
        let bounds = ui
            .find(iced::widget::Id::new("block"))
            .expect("block")
            .bounds();

        match expected {
            Some(width) => assert_eq!(bounds.width, width),
            None => assert!(bounds.width < 100.0, "{}", bounds.width),
        }
    }
}

/// A golden snapshot of `element` in a window of `size`.
fn snapshot(element: Element<'_, Message>, size: (f32, f32), theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(settings(), size, element);
    ui.point_at(Point::new(size.0 - 1.0, size.1 - 1.0));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn golden_block() -> Element<'static, Message> {
    block(CODE).language("dart").line_numbers(true).into()
}

#[test]
fn code_block_light() {
    snapshot(
        golden_block(),
        (560.0, 200.0),
        &Theme::light(),
        "code_block_light",
    );
}

#[test]
fn code_block_dark() {
    snapshot(
        golden_block(),
        (560.0, 200.0),
        &Theme::dark(),
        "code_block_dark",
    );
}

#[test]
fn code_block_long_line() {
    let code = "SELECT id, name, email, created_at, updated_at FROM users WHERE active = TRUE ORDER BY name;\nLIMIT 10;";
    let element = iced::widget::container(block(code).language("sql").width(Length::Fill))
        .padding(16)
        .into();

    snapshot(
        element,
        (360.0, 140.0),
        &Theme::dark(),
        "code_block_long_line",
    );
}
