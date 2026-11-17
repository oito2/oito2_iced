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

//! Table of contents: the tracker follows the scroll position, a picked
//! entry scrolls its heading into view, the keyboard, the disabled state,
//! the depth indent and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::Id;
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::operation::{self, Operation, Outcome};
use iced::keyboard::key::Named;
use iced::widget::{Space, column, container, row, scrollable, text};
use iced::{Event, Length, Point, Rectangle, Size, Vector, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::table_of_contents::{self as toc, Entry};
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
            size: Size::new(600.0, 400.0),
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

    /// Runs `operation` and the operations it chains.
    fn operate(&mut self, operation: &mut dyn Operation) {
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        ui.operate(&self.renderer, operation);

        let mut outcome = operation.finish();
        while let Outcome::Chain(mut next) = outcome {
            ui.operate(&self.renderer, next.as_mut());
            outcome = next.finish();
        }

        self.cache = Some(ui.into_cache());
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut find = selector.find();
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        ui.operate(&self.renderer, &mut operation::black_box(&mut find));
        self.cache = Some(ui.into_cache());

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    /// Scrolls the document to `offset` and lets the tracker see it.
    fn scroll(&mut self, offset: f32) {
        self.operate(&mut operation::scrollable::scroll_to(
            Id::new("document"),
            AbsoluteOffset {
                x: None,
                y: Some(offset),
            },
        ));
        self.moved();
    }

    /// Moves the pointer, an event the tracker measures on.
    fn moved(&mut self) {
        self.send([Event::Mouse(mouse::Event::CursorMoved {
            position: Point::ORIGIN,
        })]);
    }

    /// The scroll offset of the document.
    fn offset(&mut self) -> f32 {
        let mut probe = Offset::default();
        self.operate(&mut probe);
        probe.offset.expect("the document scrolls")
    }

    fn click(&mut self, text: &'static str) {
        let bounds = self.find(text).expect("the label shows").bounds();
        self.cursor = mouse::Cursor::Available(bounds.center());
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }
}

/// Reads the translation of the scrollable with the document id.
#[derive(Default)]
struct Offset {
    offset: Option<f32>,
}

impl Operation for Offset {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        if id == Some(&Id::new("document")) {
            self.offset = Some(translation.y);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Section(usize),
    Pick(usize),
}

struct App {
    entries: Vec<Entry>,
    section: usize,
    picked: Vec<usize>,
    selectable: bool,
    /// The height of each section and of the space after the last one.
    heights: (f32, f32),
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let mut list = toc::table_of_contents(&self.entries, self.section);

        if self.selectable {
            list = list.on_select(Message::Pick);
        }

        let mut body = column![];
        for index in 0..3 {
            body = body.push(
                container(text(format!("Body {index}")))
                    .id(format!("section-{index}"))
                    .height(self.heights.0),
            );
        }
        body = body.push(Space::new().height(self.heights.1));

        let document = scrollable(body.width(Length::Fill))
            .id("document")
            .height(Length::Fill);

        focus::navigation(row![
            container(list).width(200),
            toc::tracker(document, &self.entries, self.section).on_change(Message::Section),
        ])
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Section(section) => self.section = section,
            Message::Pick(index) => self.picked.push(index),
        }
    }
}

fn entries() -> Vec<Entry> {
    (0..3)
        .map(|index| toc::entry(format!("Section {index}"), format!("section-{index}")))
        .collect()
}

fn app() -> Harness {
    Harness::new(App {
        entries: entries(),
        section: 0,
        picked: Vec::new(),
        selectable: true,
        heights: (600.0, 900.0),
    })
}

#[test]
fn follows_the_scroll_and_scrolls_to_a_section() {
    let mut ui = app();

    ui.moved();
    assert_eq!(ui.model.section, 0);

    ui.scroll(650.0);
    assert_eq!(ui.model.section, 1);

    ui.click("Section 2");
    assert_eq!(ui.model.picked, [2]);

    let target = ui.model.entries[2].target.clone();
    ui.operate(&mut toc::scroll_operation::<()>(
        Id::new("document"),
        target,
    ));
    ui.moved();
    assert_eq!(ui.offset(), 1200.0);
    assert_eq!(ui.model.section, 2);

    ui.scroll(0.0);
    assert_eq!(ui.model.section, 0);
}

#[test]
fn a_heading_counts_from_the_activation_offset() {
    let mut ui = app();

    ui.scroll(600.0 - toc::ACTIVATION_OFFSET - 1.0);
    assert_eq!(ui.model.section, 0);

    ui.scroll(600.0 - toc::ACTIVATION_OFFSET);
    assert_eq!(ui.model.section, 1);
}

#[test]
fn the_end_of_the_document_makes_the_last_section_current() {
    let mut ui = Harness::new(App {
        entries: entries(),
        section: 0,
        picked: Vec::new(),
        selectable: true,
        // The last heading never reaches the top: the document ends first.
        heights: (300.0, 100.0),
    });

    ui.scroll(10_000.0);
    assert_eq!(ui.model.section, 2);
}

#[test]
fn scrolling_to_a_section_stops_at_the_end_of_the_document() {
    let mut ui = Harness::new(App {
        entries: entries(),
        section: 0,
        picked: Vec::new(),
        selectable: true,
        heights: (300.0, 100.0),
    });

    let target = ui.model.entries[2].target.clone();
    ui.operate(&mut toc::scroll_operation::<()>(
        Id::new("document"),
        target,
    ));

    // 3 sections of 300 and 100 after them, in a 400 pixel viewport.
    assert_eq!(ui.offset(), 600.0);
}

#[test]
fn entries_take_the_keyboard_focus() {
    let mut ui = app();

    ui.key(Named::Tab);
    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert_eq!(ui.model.picked, [1]);

    let focused = ui.focused().expect("an entry is focused");
    let label = ui.find("Section 1").expect("the label").bounds();
    assert!(focused.contains(label.center()));
}

#[test]
fn without_a_message_the_entries_cannot_be_picked() {
    let mut ui = app();
    ui.model.selectable = false;

    ui.click("Section 1");
    ui.key(Named::Tab);

    assert!(ui.model.picked.is_empty());
    assert_eq!(ui.focused(), None);
}

#[test]
fn unknown_targets_publish_nothing() {
    let mut ui = app();
    ui.model.entries = vec![toc::entry("Elsewhere", "missing")];

    ui.scroll(650.0);
    assert_eq!(ui.model.section, 0);

    let target = ui.model.entries[0].target.clone();
    ui.operate(&mut toc::scroll_operation::<()>(
        Id::new("document"),
        target,
    ));
    assert_eq!(ui.offset(), 650.0);
}

#[test]
fn each_level_indents_the_entry() {
    let mut ui = app();
    ui.model.entries = vec![
        toc::entry("Top", "section-0"),
        toc::entry("Nested", "section-1").depth(1),
        toc::entry("Deeper", "section-2").depth(2),
    ];

    let top = ui.find("Top").expect("top").bounds();
    let nested = ui.find("Nested").expect("nested").bounds();
    let deeper = ui.find("Deeper").expect("deeper").bounds();

    assert_eq!(top.x, spacing::MD);
    assert_eq!(nested.x - top.x, 12.0);
    assert_eq!(deeper.x - top.x, 24.0);
}

fn catalog(entries: &[Entry], direction: ReadingDirection) -> Element<'_, ()> {
    container(
        toc::table_of_contents(entries, 1)
            .direction(direction)
            .on_select(|_| ()),
    )
    .width(220)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str, direction: ReadingDirection) {
    let entries = vec![
        toc::entry("Introduction", "intro"),
        toc::entry("Installation", "install"),
        toc::entry("From source", "source").depth(1),
        toc::entry("Packages", "packages").depth(1),
        toc::entry("Usage", "usage"),
    ];
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (252.0, 220.0),
        catalog(&entries, direction),
    );
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn table_of_contents_light() {
    snapshot(
        &Theme::light(),
        "table_of_contents_light",
        ReadingDirection::LeftToRight,
    );
}

#[test]
fn table_of_contents_dark() {
    snapshot(
        &Theme::dark(),
        "table_of_contents_dark",
        ReadingDirection::LeftToRight,
    );
}

#[test]
fn table_of_contents_light_high_contrast_rtl() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(
        &theme,
        "table_of_contents_light_high_contrast_rtl",
        ReadingDirection::RightToLeft,
    );
}
