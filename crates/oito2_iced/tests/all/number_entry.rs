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

//! Number entries: the buttons, the keys, typed text, the wheel, the drag
//! on the buttons, the read-only entry, the height per density, and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::{self, key::Named};
use iced::widget::{column, row};
use iced::{Event, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::number_entry;
use oito2_iced::{Element, Theme};

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness<M: Model> {
    model: M,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    messages: Vec<M::Message>,
}

trait Model {
    type Message: Clone;

    fn view(&self) -> Element<'_, Self::Message>;

    fn update(&mut self, message: Self::Message);
}

const SIZE: Size = Size::new(400.0, 300.0);

impl<M: Model> Harness<M> {
    fn new(model: M) -> Self {
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
            &mut clipboard::Null,
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

    fn bounds(&mut self, id: &'static str) -> Rectangle {
        self.find(selector::id(id)).expect("widget exists").bounds()
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn select_all(&mut self) {
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::COMMAND,
            )),
            simulator::press_key(keyboard::Key::Character("a".into()), None),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Value(f64),
}

struct Entry {
    value: f64,
    decimals: usize,
    step: f64,
    read_only: bool,
}

impl Entry {
    fn new() -> Self {
        Self {
            value: 5.0,
            decimals: 0,
            step: 1.0,
            read_only: false,
        }
    }
}

impl Model for Entry {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut entry = number_entry::number_entry(&Localizations::en(), self.value)
            .range(0.0, 10.0)
            .step(self.step)
            .decimals(self.decimals)
            .id("entry")
            .width(220);

        if !self.read_only {
            entry = entry.on_change(Message::Value);
        }

        oito2_iced::focus::navigation(column![entry]).into()
    }

    fn update(&mut self, message: Message) {
        let Message::Value(value) = message;
        self.value = value;
    }
}

/// The centers of the decrease and increase buttons of the entry.
fn buttons(ui: &mut Harness<Entry>) -> (Point, Point) {
    let input = ui.bounds("entry");
    let side = input.height;
    let y = input.center_y();

    (
        Point::new(input.x + input.width - 1.5 * side, y),
        Point::new(input.x + input.width - 0.5 * side, y),
    )
}

fn shows(ui: &mut Harness<Entry>, text: &str) -> bool {
    ui.find(text).is_some()
}

#[test]
fn the_buttons_step_and_stop_at_the_limits() {
    let mut ui = Harness::new(Entry::new());
    let (_, increase) = buttons(&mut ui);

    ui.click_at(increase);
    assert_eq!(ui.model.value, 6.0);
    assert!(shows(&mut ui, "6"));

    for _ in 0..6 {
        ui.click_at(increase);
    }

    assert_eq!(ui.model.value, 10.0);
    assert_eq!(ui.messages.len(), 5, "no message past the maximum");

    let (decrease, _) = buttons(&mut ui);
    ui.click_at(decrease);
    assert_eq!(ui.model.value, 9.0);
}

#[test]
fn keys_step_page_and_jump_to_the_limits() {
    let mut ui = Harness::new(Entry::new());
    let input = ui.bounds("entry");

    ui.click_at(input.center());
    ui.key(Named::ArrowUp);
    assert_eq!(ui.model.value, 6.0);

    ui.key(Named::ArrowDown);
    ui.key(Named::ArrowDown);
    assert_eq!(ui.model.value, 4.0);

    ui.key(Named::PageUp);
    assert_eq!(ui.model.value, 10.0);

    ui.key(Named::Home);
    assert_eq!(ui.model.value, 0.0);

    ui.key(Named::End);
    assert_eq!(ui.model.value, 10.0);
}

#[test]
fn typed_text_is_rounded_clamped_or_replaced() {
    let mut ui = Harness::new(Entry {
        decimals: 1,
        step: 0.5,
        ..Entry::new()
    });
    let input = ui.bounds("entry");

    ui.click_at(input.center());
    ui.select_all();
    ui.type_text("7.26");
    assert_eq!(ui.model.value, 5.0, "typing alone changes nothing");

    ui.key(Named::Enter);
    assert_eq!(ui.model.value, 7.3);
    assert!(shows(&mut ui, "7.3"));

    ui.select_all();
    ui.type_text("42");
    ui.key(Named::Enter);
    assert_eq!(ui.model.value, 10.0);

    ui.select_all();
    ui.type_text("..");
    ui.key(Named::Enter);
    assert_eq!(ui.model.value, 10.0);
    assert!(shows(&mut ui, "10.0"));

    ui.select_all();
    ui.type_text("3x");
    assert!(shows(&mut ui, "3"), "letters are not taken");
}

#[test]
fn leaving_the_field_applies_the_typed_text() {
    let mut ui = Harness::new(Entry::new());
    let input = ui.bounds("entry");

    ui.click_at(input.center());
    ui.select_all();
    ui.type_text("8");
    ui.click_at(Point::new(390.0, 290.0));

    assert_eq!(ui.model.value, 8.0);
}

#[test]
fn the_wheel_steps_while_focused() {
    let mut ui = Harness::new(Entry::new());
    let input = ui.bounds("entry");
    let wheel = |x, y| {
        Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x, y },
        })
    };

    ui.cursor = mouse::Cursor::Available(input.center());
    ui.send([wheel(0.0, 1.0)]);
    assert_eq!(ui.model.value, 5.0, "no step without the focus");

    ui.click_at(input.center());
    ui.send([wheel(1.0, 0.0)]);
    assert_eq!(ui.model.value, 5.0, "a sideways scroll leaves the value");

    ui.send([wheel(0.0, 1.0)]);
    assert_eq!(ui.model.value, 6.0);

    ui.send([wheel(0.0, -1.0)]);
    ui.send([wheel(0.0, -1.0)]);
    assert_eq!(ui.model.value, 4.0);
}

#[test]
fn dragging_sideways_on_the_buttons_scrubs_the_value() {
    let mut ui = Harness::new(Entry::new());
    let (decrease, _) = buttons(&mut ui);
    let moved = |x: f32| {
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(x, decrease.y),
        })
    };

    ui.cursor = mouse::Cursor::Available(decrease);
    ui.send([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    for step in 1..=3 {
        let x = decrease.x + 4.0 * step as f32;
        ui.cursor = mouse::Cursor::Available(Point::new(x, decrease.y));
        ui.send([moved(x)]);
    }

    assert_eq!(ui.model.value, 8.0);

    let x = decrease.x - 4.0;
    ui.cursor = mouse::Cursor::Available(Point::new(x, decrease.y));
    ui.send([moved(x)]);
    assert_eq!(ui.model.value, 4.0, "four steps back");

    ui.cursor = mouse::Cursor::Available(decrease);
    ui.send([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    assert_eq!(ui.model.value, 4.0, "the release after a drag is no click");
}

#[test]
fn a_read_only_entry_ignores_keys_buttons_and_typing() {
    let mut ui = Harness::new(Entry {
        read_only: true,
        ..Entry::new()
    });
    let input = ui.bounds("entry");
    let (decrease, increase) = buttons(&mut ui);

    ui.click_at(increase);
    ui.click_at(decrease);
    ui.click_at(input.center());
    ui.key(Named::Home);
    ui.key(Named::End);
    ui.key(Named::ArrowUp);
    ui.type_text("7");
    ui.key(Named::Enter);

    assert!(ui.messages.is_empty());
    assert!(shows(&mut ui, "5"));
}

#[test]
fn the_buttons_are_reached_by_tab_and_step_on_enter() {
    let mut ui = Harness::new(Entry::new());

    ui.key(Named::Tab);
    let first = ui.find(selector::is_focused()).expect("focus").bounds();
    assert_eq!(first, ui.bounds("entry"));

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert_eq!(ui.model.value, 4.0, "the decrease button");

    ui.key(Named::Tab);
    ui.key(Named::Space);
    assert_eq!(ui.model.value, 5.0, "the increase button");
}

#[test]
fn entries_are_as_tall_as_the_controls_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = Harness::new(Entry::new());
        assert_eq!(ui.bounds("entry").height, height, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let en = Localizations::en();
    let pt = Localizations::pt_br();

    column![
        row![
            number_entry::number_entry(&en, 12.0)
                .label("Copies")
                .on_change(Message::Value)
                .width(180),
            number_entry::number_entry(&en, 1234.5)
                .decimals(2)
                .on_change(Message::Value)
                .width(180),
        ]
        .spacing(12),
        row![
            number_entry::number_entry(&pt, 1234.5)
                .decimals(1)
                .range(0.0, 1234.5)
                .on_change(Message::Value)
                .width(180),
            number_entry::number_entry::<Message>(&en, 5.0)
                .label("Read-only")
                .width(180),
        ]
        .spacing(12),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (400.0, 160.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn number_entry_light() {
    snapshot(&Theme::light(), "number_entry_light");
}

#[test]
fn number_entry_dark() {
    snapshot(&Theme::dark(), "number_entry_dark");
}

#[test]
fn number_entry_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "number_entry_light_high_contrast");
}
