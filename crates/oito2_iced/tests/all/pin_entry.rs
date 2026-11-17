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

//! Pin entries: typing, Backspace, paste, completion, masking, the
//! localized description, the single focus stop, the disabled entry, the
//! cell size per density, and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard::{Clipboard, Kind};
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::{self, key::Named};
use iced::widget::{column, row, text_input};
use iced::{Event, Point, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::pin_entry;
use oito2_iced::{Element, Theme};

/// A clipboard holding a fixed text.
struct Holding(Option<String>);

impl Clipboard for Holding {
    fn read(&self, _kind: Kind) -> Option<String> {
        self.0.clone()
    }

    fn write(&mut self, _kind: Kind, contents: String) {
        self.0 = Some(contents);
    }
}

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness {
    model: Entry,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    clipboard: Holding,
    messages: Vec<Message>,
}

const SIZE: Size = Size::new(400.0, 200.0);

impl Harness {
    fn new(model: Entry) -> Self {
        let settings = support::settings();
        let _fonts = simulator::Simulator::<()>::with_settings(
            support::settings(),
            iced::widget::Space::new(),
        );
        let backend = std::env::var("ICED_TEST_BACKEND").ok();
        let renderer =
            iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
                settings.default_font,
                settings.default_text_size,
                backend.as_deref(),
            ))
            .expect("headless renderer");

        Self {
            model,
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            clipboard: Holding(None),
            messages: Vec::new(),
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.model.view(),
            SIZE,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let _ = ui.update(
            &events,
            self.cursor,
            &mut self.renderer,
            &mut self.clipboard,
            &mut messages,
        );
        self.cache = Some(ui.into_cache());

        for message in messages {
            self.messages.push(message.clone());
            self.model.update(message);
        }
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut ui = UserInterface::build(
            self.model.view(),
            SIZE,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let mut find = selector.find();
        ui.operate(&self.renderer, &mut operation::black_box(&mut find));
        self.cache = Some(ui.into_cache());

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn shows(&mut self, text: &str) -> bool {
        self.find(text).is_some()
    }

    fn focused(&mut self) -> Option<iced::Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn click_entry(&mut self) {
        let bounds = self.find(selector::id("pin")).expect("entry").bounds();
        self.click_at(bounds.center());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    fn paste(&mut self, text: &str) {
        self.clipboard = Holding(Some(text.to_owned()));
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::COMMAND,
            )),
            simulator::press_key(keyboard::Key::Character("v".into()), None),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Code(String),
    Complete(String),
    Other(String),
}

struct Entry {
    code: String,
    length: usize,
    numeric: bool,
    obscure: bool,
    disabled: bool,
    localizations: Localizations,
    completed: Vec<String>,
    other: String,
}

impl Entry {
    fn new(length: usize) -> Self {
        Self {
            code: String::new(),
            length,
            numeric: true,
            obscure: false,
            disabled: false,
            localizations: Localizations::en(),
            completed: Vec::new(),
            other: String::new(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut entry = pin_entry::pin_entry(&self.code)
            .length(self.length)
            .numeric(self.numeric)
            .obscure(self.obscure)
            .localizations(&self.localizations)
            .id("pin");

        if !self.disabled {
            entry = entry
                .on_change(Message::Code)
                .on_complete(Message::Complete);
        }

        let other = text_input("Other", &self.other)
            .id("other")
            .on_input(Message::Other);

        oito2_iced::focus::navigation(column![entry, other].spacing(12).padding(12)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Code(code) => self.code = code,
            Message::Complete(code) => self.completed.push(code),
            Message::Other(text) => self.other = text,
        }
    }
}

#[test]
fn typing_fills_the_cells_completes_and_gives_up_the_focus() {
    let mut ui = Harness::new(Entry::new(4));

    ui.click_entry();
    assert!(ui.focused().is_some(), "a click focuses the entry");

    ui.type_text("12");
    assert_eq!(ui.model.code, "12");
    assert!(ui.shows("1"));
    assert!(ui.shows("2"));

    ui.type_text("a34");
    assert_eq!(ui.model.code, "1234");
    assert_eq!(ui.model.completed, ["1234"]);
    assert_eq!(ui.focused(), None, "a complete code releases the focus");
}

#[test]
fn backspace_empties_the_last_cell() {
    let mut ui = Harness::new(Entry::new(4));

    ui.click_entry();
    ui.type_text("123");
    ui.key(Named::Backspace);
    assert_eq!(ui.model.code, "12");

    ui.key(Named::Home);
    ui.type_text("9");
    assert_eq!(ui.model.code, "129", "the caret stays at the end");
}

#[test]
fn an_emoji_fills_one_cell_of_a_text_code() {
    let mut ui = Harness::new(Entry {
        numeric: false,
        ..Entry::new(3)
    });

    ui.click_entry();
    ui.type_text("a \u{1F600}b");
    assert!(ui.shows("\u{1F600}"));
    assert_eq!(
        ui.model.completed,
        ["a\u{1F600}b"],
        "white space is dropped"
    );
}

#[test]
fn a_pasted_code_fills_every_cell_and_masking_hides_it() {
    let mut ui = Harness::new(Entry {
        obscure: true,
        ..Entry::new(4)
    });

    ui.click_entry();
    ui.paste("98-76 54");
    assert_eq!(ui.model.code, "9876");
    assert_eq!(ui.model.completed, ["9876"]);
    assert!(!ui.shows("9"));
    assert!(ui.shows("\u{2022}"));
}

#[test]
fn the_entry_is_described_in_the_locale() {
    let mut ui = Harness::new(Entry::new(6));
    assert!(ui.shows("Code of 6 characters"));

    let mut ui = Harness::new(Entry {
        localizations: Localizations::pt_br(),
        ..Entry::new(4)
    });
    assert!(ui.shows("Código de 4 caracteres"));
}

#[test]
fn the_entry_is_one_tab_stop() {
    let mut ui = Harness::new(Entry::new(4));
    let pin = ui.find(selector::id("pin")).expect("entry").bounds();
    let other = ui.find(selector::id("other")).expect("other").bounds();

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(pin));

    ui.type_text("7");
    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(other), "one Tab leaves the cells");
    assert_eq!(ui.model.code, "7");
}

#[test]
fn a_disabled_entry_takes_no_focus_and_no_input() {
    let mut ui = Harness::new(Entry {
        disabled: true,
        ..Entry::new(4)
    });
    let other = ui.find(selector::id("other")).expect("other").bounds();

    ui.click_entry();
    ui.type_text("12");
    assert!(ui.messages.is_empty());

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(other), "Tab skips the disabled entry");
}

#[test]
fn cells_follow_the_control_height_of_the_density() {
    for (density, height) in [(Density::Pointer, 40.0), (Density::Touch, 46.0)] {
        density::set(density);

        let mut ui = Harness::new(Entry::new(4));
        let bounds = ui.find(selector::id("pin")).expect("entry").bounds();
        assert_eq!(bounds.height, height, "{density:?}");
        assert_eq!(
            bounds.width,
            4.0 * (height - 4.0) + 3.0 * 8.0,
            "{density:?}"
        );
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    column![
        row![
            pin_entry::pin_entry("42")
                .length(4)
                .on_change(Message::Code),
            pin_entry::pin_entry("1234")
                .length(4)
                .obscure(true)
                .on_change(Message::Code),
        ]
        .spacing(16),
        row![
            pin_entry::pin_entry("98")
                .length(4)
                .error(true)
                .on_change(Message::Code),
            pin_entry::pin_entry::<Message>("5").length(4),
        ]
        .spacing(16),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (420.0, 120.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn pin_entry_light() {
    snapshot(&Theme::light(), "pin_entry_light");
}

#[test]
fn pin_entry_dark() {
    snapshot(&Theme::dark(), "pin_entry_dark");
}

#[test]
fn pin_entry_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "pin_entry_light_high_contrast");
}

#[test]
fn pin_entry_focused_compact() {
    density::set(Density::Pointer);

    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (420.0, 70.0),
        column![
            pin_entry::pin_entry("42")
                .length(6)
                .id("pin")
                .on_change(Message::Code)
        ]
        .padding(12),
    );
    let _ = ui.click(selector::id("pin")).expect("entry");
    assert!(ui.find(selector::is_focused()).is_ok(), "focused");
    let snapshot = ui.snapshot(&Theme::dark()).expect("snapshot");

    support::golden!(&snapshot, "pin_entry_focused_dark");
}
