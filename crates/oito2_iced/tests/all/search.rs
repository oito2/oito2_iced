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

//! Search fields and the search title field: the clear button, Escape,
//! the toggle and its focus, the fixed size, the height per density, and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::widget::{column, row, text};
use iced::{Event, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::search;
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
    Query(String),
    Cleared,
    Escaped,
    Active(bool),
}

#[derive(Default)]
struct Search {
    query: String,
    cleared: usize,
    escaped: usize,
    active: bool,
}

impl Model for Search {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        oito2_iced::focus::navigation(column![
            search::search_field(&Localizations::en(), &self.query)
                .id("search")
                .on_input(Message::Query)
                .on_clear(Message::Cleared)
                .on_escape(Message::Escaped),
            row![
                search::search_title_field(
                    &Localizations::en(),
                    text("Files"),
                    &self.query,
                    self.active,
                    Message::Active,
                )
                .id("title-search")
                .on_input(Message::Query),
                text("|"),
            ],
        ])
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Query(query) => self.query = query,
            Message::Cleared => self.cleared += 1,
            Message::Escaped => self.escaped += 1,
            Message::Active(active) => self.active = active,
        }
    }
}

fn focused(ui: &mut Harness<Search>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .map(|target| target.bounds())
}

/// The bounds of the marker after the title field.
fn after_marker(ui: &mut Harness<Search>) -> Rectangle {
    ui.find("|").expect("marker").bounds()
}

/// The center of the square box at the end of `field`.
fn trailing(field: Rectangle) -> Point {
    Point::new(field.x + field.width - field.height / 2.0, field.center_y())
}

/// The center of the square box at the start of `field`.
fn leading(field: Rectangle) -> Point {
    Point::new(field.x + field.height / 2.0, field.center_y())
}

#[test]
fn the_search_field_clears_with_the_button_and_with_escape() {
    let mut ui = Harness::new(Search::default());
    let field = ui.bounds("search");

    assert!(ui.find("Search").is_some(), "the localized hint");

    ui.click_at(trailing(field));
    assert!(ui.messages.is_empty(), "no clear button while empty");

    ui.click_at(field.center());
    ui.type_text("files");
    ui.click_at(trailing(field));
    assert_eq!(ui.model.query, "");
    assert_eq!(ui.model.cleared, 1);
    assert_eq!(ui.messages.last(), Some(&Message::Cleared));

    ui.click_at(field.center());
    ui.type_text("docs");
    ui.key(Named::Escape);
    assert_eq!(ui.model.query, "");
    assert_eq!(ui.model.cleared, 2);
    assert_eq!(focused(&mut ui), Some(field), "Escape keeps the focus");
    assert_eq!(ui.model.escaped, 0);

    ui.key(Named::Escape);
    assert_eq!(ui.model.escaped, 1);
}

#[test]
fn the_title_field_toggles_between_title_and_search() {
    let mut ui = Harness::new(Search::default());
    let after = after_marker(&mut ui);

    assert!(ui.find("Files").is_some());
    let toggle = leading(Rectangle {
        x: 0.0,
        width: after.x,
        ..after
    });

    ui.click_at(toggle);
    assert!(ui.model.active);
    assert!(ui.find("Files").is_none());

    let field = ui.bounds("title-search");
    assert_eq!(focused(&mut ui), Some(field), "opening focuses the field");
    assert_eq!(after_marker(&mut ui), after, "the size does not change");
    assert_eq!(field.width, search::TITLE_FIELD_WIDTH);

    ui.type_text("x");
    ui.key(Named::Escape);
    assert!(ui.model.active, "Escape first clears the text");
    assert_eq!(ui.model.query, "");
    assert_eq!(focused(&mut ui), Some(field));

    ui.key(Named::Escape);
    assert!(!ui.model.active);
    assert!(ui.find("Files").is_some());

    let toggle_bounds = focused(&mut ui).expect("the toggle has the focus");
    assert!(toggle_bounds.contains(toggle));
    assert_eq!(after_marker(&mut ui), after);
}

#[test]
fn the_toggle_closes_an_open_search() {
    let mut ui = Harness::new(Search {
        active: true,
        ..Search::default()
    });
    let field = ui.bounds("title-search");

    ui.send([]);
    assert_eq!(
        focused(&mut ui),
        Some(field),
        "an open search takes the focus"
    );

    ui.click_at(leading(field));
    assert!(!ui.model.active);
    assert!(focused(&mut ui).is_some_and(|toggle| toggle.contains(leading(field))));
}

#[test]
fn search_fields_are_as_tall_as_the_controls_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = Harness::new(Search {
            active: true,
            ..Search::default()
        });
        assert_eq!(ui.bounds("search").height, height, "{density:?}");
        assert_eq!(ui.bounds("title-search").height, height, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let en = Localizations::en();
    let pt = Localizations::pt_br();

    column![
        search::search_field(&en, "").on_input(Message::Query),
        search::search_field(&en, "invoices").on_input(Message::Query),
        search::search_field::<Message>(&pt, ""),
        search::search_title_field(&en, text("Files"), "", false, Message::Active)
            .on_input(Message::Query),
        search::search_title_field(&en, text("Files"), "report", true, Message::Active)
            .on_input(Message::Query),
    ]
    .spacing(12)
    .padding(12)
    .width(360)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 240.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn search_light() {
    snapshot(&Theme::light(), "search_light");
}

#[test]
fn search_dark() {
    snapshot(&Theme::dark(), "search_dark");
}

#[test]
fn search_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "search_light_high_contrast");
}
