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

//! List box: single and multiple selection with the keyboard and the
//! pointer, type-ahead, activation by Enter and double click, the
//! read-only state, scrolling the active row into view and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use std::time::Duration;

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::Id;
use iced::advanced::widget::operation::scrollable::Scrollable;
use iced::advanced::widget::operation::{self, Operation};
use iced::keyboard::{self, Modifiers, key::Named};
use iced::widget::column;
use iced::{Event, Point, Rectangle, Size, Vector, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::collections::{
    ListCollection, SelectionController, SelectionMode, SelectionOptions,
};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::list_box::{self, SelectionChange};
use oito2_iced::{Element, Theme, focus};

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness {
    model: App,
    size: Size,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
}

impl Harness {
    fn new(model: App) -> Self {
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
            size: Size::new(320.0, 400.0),
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
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
            self.model.update(message);
        }
    }

    fn operate(&mut self, operation: &mut dyn operation::Operation) {
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        ui.operate(&self.renderer, operation);
        self.cache = Some(ui.into_cache());
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut find = selector.find();
        self.operate(&mut operation::black_box(&mut find));

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    /// Clicks the label `text`.
    fn click(&mut self, text: &'static str) {
        let bounds = self.find(text).expect("the label shows").bounds();
        self.cursor = mouse::Cursor::Available(bounds.center());
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn key_with(&mut self, key: keyboard::Key, modifiers: Modifiers) {
        let mut event = simulator::press_key(key, None);
        if let Event::Keyboard(keyboard::Event::KeyPressed {
            modifiers: pressed, ..
        }) = &mut event
        {
            *pressed = modifiers;
        }
        self.send([event]);
    }

    fn modifiers(&mut self, modifiers: Modifiers) {
        self.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            modifiers,
        ))]);
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    fn selected(&self) -> Vec<String> {
        self.model.selection.selected_values()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Selection(SelectionChange),
    Activate(String),
    After,
}

struct App {
    selection: SelectionController<&'static str>,
    activated: Vec<String>,
    read_only: bool,
    max_height: f32,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let mut list = list_box::list_box(&self.selection)
            .id("list")
            .max_height(self.max_height)
            .on_activate(|fruit| Message::Activate((*fruit).to_owned()));

        if !self.read_only {
            list = list.on_change(Message::Selection);
        }

        focus::navigation(
            column![
                list,
                button::filled("After").on_press(Message::After).id("after"),
            ]
            .spacing(8)
            .padding(8)
            .width(240),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selection(change) => {
                let _ = change.apply(&mut self.selection);
            }
            Message::Activate(fruit) => self.activated.push(fruit),
            Message::After => {}
        }
    }
}

fn fruits() -> ListCollection<&'static str> {
    ListCollection::new(["Apple", "Banana", "Blueberry", "Cherry", "Date"])
        .expect("unique fruits")
        .with_disabled(|fruit| *fruit == "Cherry")
        .with_group_of(|fruit| fruit.starts_with('B').then(|| "B".to_owned()))
}

fn app(mode: SelectionMode) -> Harness {
    let options = SelectionOptions {
        mode,
        ..SelectionOptions::default()
    };

    Harness::new(App {
        selection: SelectionController::new(fruits(), options),
        activated: Vec::new(),
        read_only: false,
        max_height: f32::INFINITY,
    })
}

#[test]
fn single_the_arrows_select_skipping_disabled_items() {
    let mut ui = app(SelectionMode::Single);

    ui.key(Named::Tab);
    let list = ui.find(selector::id("list")).expect("the list").bounds();
    assert_eq!(ui.focused(), Some(list));
    assert!(ui.selected().is_empty());

    ui.key(Named::ArrowDown);
    assert_eq!(ui.selected(), ["Banana"]);
    ui.key(Named::End);
    assert_eq!(ui.selected(), ["Date"]);
    ui.key(Named::ArrowUp);
    assert_eq!(ui.selected(), ["Blueberry"]);
    ui.key(Named::Home);
    assert_eq!(ui.selected(), ["Apple"]);
    ui.key(Named::ArrowDown);
    ui.key(Named::Enter);
    assert_eq!(ui.model.activated, ["Banana"]);

    assert!(ui.find("B").is_some(), "the group heading shows");
}

#[test]
fn a_double_click_activates_and_two_slow_clicks_do_not() {
    let mut ui = app(SelectionMode::Single);

    ui.click("Date");
    std::thread::sleep(Duration::from_millis(350));
    ui.click("Date");
    assert!(ui.model.activated.is_empty());

    ui.click("Date");
    assert_eq!(ui.model.activated, ["Date"]);
}

#[test]
fn disabled_items_ignore_clicks_and_enter() {
    let mut ui = app(SelectionMode::Single);

    ui.click("Cherry");
    ui.click("Cherry");
    assert!(ui.selected().is_empty());
    assert!(ui.model.activated.is_empty());
}

#[test]
fn type_ahead_jumps_to_matching_labels() {
    let mut ui = app(SelectionMode::Single);

    ui.key(Named::Tab);
    ui.type_text("b");
    assert_eq!(ui.selected(), ["Banana"]);
    ui.type_text("l");
    assert_eq!(ui.selected(), ["Blueberry"]);

    std::thread::sleep(list_box::TYPE_AHEAD_TIMEOUT);
    ui.type_text("d");
    assert_eq!(ui.selected(), ["Date"]);
    ui.type_text("c");
    assert_eq!(ui.selected(), ["Date"], "no enabled label starts with dc");
}

#[test]
fn multiple_space_shift_arrows_control_a_and_clicks() {
    let mut ui = app(SelectionMode::Multiple);

    ui.click("Apple");
    assert_eq!(ui.selected(), ["Apple"]);

    ui.key_with(Named::ArrowDown.into(), Modifiers::SHIFT);
    assert_eq!(ui.selected(), ["Apple", "Banana"]);

    ui.key(Named::ArrowDown);
    ui.key(Named::Space);
    assert_eq!(ui.selected(), ["Apple", "Banana", "Blueberry"]);

    ui.modifiers(Modifiers::CTRL);
    ui.click("Banana");
    ui.modifiers(Modifiers::empty());
    assert_eq!(ui.selected(), ["Apple", "Blueberry"]);

    ui.key_with(keyboard::Key::Character("a".into()), Modifiers::CTRL);
    assert_eq!(ui.selected(), ["Apple", "Banana", "Blueberry", "Date"]);

    // Control+click made Banana the anchor.
    ui.modifiers(Modifiers::SHIFT);
    ui.click("Date");
    ui.modifiers(Modifiers::empty());
    assert_eq!(
        ui.selected(),
        ["Banana", "Blueberry", "Date"],
        "Shift selects the range from the anchor"
    );
}

#[test]
fn a_read_only_list_ignores_input_and_is_skipped() {
    let mut ui = app(SelectionMode::Single);
    ui.model.read_only = true;

    ui.click("Apple");
    ui.key(Named::Tab);
    let after = ui.find(selector::id("after")).expect("the button").bounds();

    assert_eq!(ui.focused(), Some(after));
    assert!(ui.selected().is_empty());
}

/// Finds the bounds and the translation of the first scrollable.
#[derive(Default)]
struct Scroll {
    found: Option<(Rectangle, Vector)>,
}

impl Operation for Scroll {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        _content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.found.get_or_insert((bounds, translation));
    }
}

#[test]
fn the_active_row_scrolls_into_view() {
    let mut ui = app(SelectionMode::Single);
    ui.model.max_height = 80.0;

    ui.key(Named::Tab);
    ui.key(Named::End);

    let mut scroll = Scroll::default();
    ui.operate(&mut scroll);
    let (bounds, translation) = scroll.found.expect("the rows scroll");
    let date = ui.find("Date").expect("the label").bounds();

    assert_eq!(bounds.height, 80.0);
    assert!(translation.y > 0.0);
    assert!(date.y - translation.y + date.height <= bounds.y + bounds.height);

    ui.key(Named::Home);
    let mut scroll = Scroll::default();
    ui.operate(&mut scroll);
    // The first row starts below the padding of the list.
    assert_eq!(
        scroll.found.map(|(_, translation)| translation.y),
        Some(spacing::XS)
    );
}

#[test]
fn a_click_outside_removes_the_focus() {
    let mut ui = app(SelectionMode::Single);

    ui.click("Apple");
    let list = ui.find(selector::id("list")).expect("the list").bounds();
    assert_eq!(ui.focused(), Some(list));

    ui.cursor = mouse::Cursor::Available(Point::new(300.0, 390.0));
    ui.send(simulator::click());
    assert_eq!(ui.focused(), None);
}

fn catalog<'a>(
    single: &'a SelectionController<&'static str>,
    multiple: &'a SelectionController<&'static str>,
) -> Element<'a, ()> {
    let single = list_box::list_box(single)
        .id("single")
        .on_change(|_| ())
        .width(200);
    let multiple = list_box::list_box(multiple).on_change(|_| ()).width(200);

    iced::widget::row![single, multiple]
        .spacing(16)
        .padding(16)
        .into()
}

fn controllers() -> (
    SelectionController<&'static str>,
    SelectionController<&'static str>,
) {
    let single = SelectionController::with_values(fruits(), SelectionOptions::default(), ["Date"]);
    let multiple = SelectionController::with_values(
        fruits(),
        SelectionOptions {
            mode: SelectionMode::Multiple,
            ..SelectionOptions::default()
        },
        ["Apple", "Blueberry"],
    );

    (single, multiple)
}

fn snapshot(theme: &Theme, name: &str, focused: bool) {
    let (single, multiple) = controllers();
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (464.0, 260.0),
        focus::navigation(catalog(&single, &multiple)),
    );

    if focused {
        ui.tap_key(Named::Tab);
        ui.tap_key(Named::ArrowUp);
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn list_box_light() {
    snapshot(&Theme::light(), "list_box_light", false);
}

#[test]
fn list_box_dark() {
    snapshot(&Theme::dark(), "list_box_dark", false);
}

#[test]
fn list_box_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "list_box_light_high_contrast", false);
}

#[test]
fn list_box_focused() {
    snapshot(&Theme::light(), "list_box_focused", true);
}
