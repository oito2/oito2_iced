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

//! Editable labels: starting to edit with a double click, Enter and F2,
//! keeping or restoring the text, the focus, the read-only label, and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::widget::column;
use iced::{Event, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, typography};
use oito2_iced::widget::editable_label;
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

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Rename(String),
}

struct Notes {
    text: String,
    read_only: bool,
}

impl Notes {
    fn new() -> Self {
        Self {
            text: "Notes".into(),
            read_only: false,
        }
    }
}

impl Model for Notes {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut label = editable_label::editable_label(&self.text).id("label");

        if !self.read_only {
            label = label.on_submit(Message::Rename);
        }

        oito2_iced::focus::navigation(column![label]).into()
    }

    fn update(&mut self, message: Message) {
        let Message::Rename(text) = message;
        self.text = text;
    }
}

fn focused(ui: &mut Harness<Notes>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .map(|target| target.bounds())
}

fn double_click(ui: &mut Harness<Notes>, point: Point) {
    ui.cursor = mouse::Cursor::Available(point);
    ui.send(simulator::click().chain(simulator::click()));
}

#[test]
fn a_double_click_edits_with_the_text_selected_and_enter_keeps_it() {
    let mut ui = Harness::new(Notes::new());
    let label = ui.bounds("label");

    ui.click_at(Point::new(label.x + 8.0, label.center_y()));
    assert!(ui.find("Notes").is_some());
    assert!(focused(&mut ui).is_none(), "a single click does not edit");

    double_click(&mut ui, Point::new(label.x + 24.0, label.center_y()));
    let field = ui.bounds("label");
    assert_eq!(focused(&mut ui), Some(field), "the field takes the focus");

    ui.type_text("Ideas");
    ui.key(Named::Enter);

    assert_eq!(ui.messages, vec![Message::Rename("Ideas".into())]);
    assert!(ui.find("Ideas").is_some());

    let label = ui.bounds("label");
    assert_eq!(
        focused(&mut ui),
        Some(label),
        "the label gets the focus back"
    );
}

#[test]
fn f2_edits_and_escape_restores_the_text() {
    let mut ui = Harness::new(Notes::new());

    ui.key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(ui.bounds("label")));

    ui.key(Named::F2);
    ui.type_text("Other");
    assert!(ui.find("Other").is_some());

    ui.key(Named::Escape);
    assert!(ui.messages.is_empty());
    assert!(ui.find("Notes").is_some());
    assert!(ui.find("Other").is_none());
    assert_eq!(focused(&mut ui), Some(ui.bounds("label")));
}

#[test]
fn enter_edits_and_leaving_the_field_keeps_the_trimmed_text() {
    let mut ui = Harness::new(Notes::new());

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    ui.type_text("  Plans ");
    ui.click_at(Point::new(390.0, 290.0));

    assert_eq!(ui.messages, vec![Message::Rename("Plans".into())]);
    assert!(ui.find("Plans").is_some());
    assert!(focused(&mut ui).is_none(), "the focus left with the click");
}

#[test]
fn unchanged_or_blank_text_is_not_submitted() {
    let mut ui = Harness::new(Notes::new());

    ui.key(Named::Tab);
    ui.key(Named::F2);
    ui.type_text("Notes");
    ui.key(Named::Enter);

    ui.key(Named::F2);
    ui.type_text("   ");
    ui.key(Named::Enter);

    assert!(ui.messages.is_empty());
    assert!(ui.find("Notes").is_some());
}

#[test]
fn a_read_only_label_is_skipped_and_never_edits() {
    let mut ui = Harness::new(Notes {
        read_only: true,
        ..Notes::new()
    });
    let label = ui.find("Notes").expect("label").bounds();

    ui.key(Named::Tab);
    assert!(focused(&mut ui).is_none());

    double_click(&mut ui, label.center());
    ui.key(Named::F2);
    ui.type_text("x");

    assert!(ui.messages.is_empty());
    assert!(ui.find("Notes").is_some());
}

fn catalog() -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            editable_label::editable_label("Project notes").on_submit(Message::Rename),
            editable_label::editable_label("")
                .placeholder("Untitled")
                .on_submit(Message::Rename),
            editable_label::editable_label("Quarterly report")
                .style(typography::TITLE_LARGE)
                .on_submit(Message::Rename),
            editable_label::editable_label::<Message>("Read-only"),
            editable_label::editable_label("Being renamed")
                .id("editing")
                .width(240)
                .on_submit(Message::Rename),
        ]
        .spacing(12)
        .padding(12),
    )
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (300.0, 260.0), catalog());
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::F2);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn editable_label_light() {
    snapshot(&Theme::light(), "editable_label_light");
}

#[test]
fn editable_label_dark() {
    snapshot(&Theme::dark(), "editable_label_dark");
}

#[test]
fn editable_label_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "editable_label_light_high_contrast");
}
