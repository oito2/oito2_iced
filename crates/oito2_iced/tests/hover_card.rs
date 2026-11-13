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

//! Hover cards: the card opens after the hover delay, stays open while the
//! pointer is over the anchor or the card, closes after the close delay,
//! opens on keyboard focus and closes on Escape and on a press outside;
//! golden snapshots. Time advances through redraw requests. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{button, column, text};
use iced::{Event, Length, Point, Rectangle, Size, mouse, window};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::focusable;
use oito2_iced::widget::hover_card::hover_card;
use oito2_iced::{Element, Theme};

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness<M: Model> {
    model: M,
    size: Size,
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

impl<M: Model> Harness<M> {
    fn new(model: M, size: Size) -> Self {
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
            size,
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
            self.messages.push(message.clone());
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

    fn move_to(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send([Event::Mouse(mouse::Event::CursorMoved { position: point })]);
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }
}

impl<M: Model> Harness<M> {
    /// Delivers a redraw request at `start + millis`.
    fn at(&mut self, start: Instant, millis: u64) {
        self.send([Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(millis),
        ))]);
    }

    fn shows(&mut self, label: &'static str) -> bool {
        self.find(label).is_some()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Open,
}

struct App;

impl Model for App {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        oito2_iced::focus::navigation(
            column![
                hover_card(
                    focusable(button("Name").on_press(Message::Open)).on_activate(Message::Open),
                    text("Card"),
                ),
                text("Below").width(Length::Fill).height(Length::Fill),
            ]
            .padding(200),
        )
        .into()
    }

    fn update(&mut self, _message: Message) {}
}

fn app() -> Harness<App> {
    Harness::new(App, Size::new(600.0, 500.0))
}

/// Moves the pointer onto the anchor and lets the open delay pass.
fn hover(ui: &mut Harness<App>, start: Instant) {
    let name = ui.find("Name").expect("anchor").bounds();
    ui.move_to(name.center());
    ui.at(start, 0);
    ui.at(start, 600);
}

#[test]
fn opens_after_the_hover_delay_and_closes_after_leaving() {
    let mut ui = app();
    let start = Instant::now();

    let name = ui.find("Name").expect("anchor").bounds();
    ui.move_to(name.center());
    ui.at(start, 0);
    ui.at(start, 200);
    assert!(!ui.shows("Card"), "closed before the delay");

    ui.at(start, 600);
    assert!(ui.shows("Card"), "open after the delay");

    let card = ui.find("Card").expect("card").bounds();
    assert!(card.y > name.y + name.height, "the card opens below");
    assert!(
        (card.center_x() - name.center_x()).abs() < 1.0,
        "the card is centered on the anchor"
    );

    ui.move_to(Point::new(2.0, 2.0));
    ui.at(start, 700);
    ui.at(start, 900);
    assert!(ui.shows("Card"), "still open before the close delay");

    ui.at(start, 1100);
    assert!(!ui.shows("Card"), "closed after the close delay");
}

#[test]
fn stays_open_while_the_pointer_is_over_the_card() {
    let mut ui = app();
    let start = Instant::now();

    hover(&mut ui, start);
    let card = ui.find("Card").expect("card").bounds();

    ui.move_to(card.center());
    ui.at(start, 700);
    ui.at(start, 1500);
    assert!(ui.shows("Card"));
}

#[test]
fn leaving_before_the_delay_cancels_the_opening() {
    let mut ui = app();
    let start = Instant::now();

    let name = ui.find("Name").expect("anchor").bounds();
    ui.move_to(name.center());
    ui.at(start, 0);
    ui.move_to(Point::new(2.0, 2.0));
    ui.at(start, 600);
    assert!(!ui.shows("Card"));
}

#[test]
fn opens_on_keyboard_focus_and_closes_on_escape() {
    let mut ui = app();
    let start = Instant::now();

    ui.key(Named::Tab);
    ui.at(start, 0);
    ui.at(start, 600);
    assert!(ui.shows("Card"), "the focus opens it");
    assert!(ui.focused().is_some(), "the anchor keeps the focus");

    ui.key(Named::Escape);
    assert!(!ui.shows("Card"));
}

#[test]
fn escape_closes_a_hovered_card() {
    let mut ui = app();
    let start = Instant::now();

    hover(&mut ui, start);
    assert!(ui.shows("Card"));

    ui.key(Named::Escape);
    assert!(!ui.shows("Card"));

    ui.at(start, 2000);
    assert!(!ui.shows("Card"), "it stays closed while the pointer rests");
}

#[test]
fn a_press_outside_closes_it() {
    let mut ui = app();
    let start = Instant::now();

    hover(&mut ui, start);
    ui.click_at(Point::new(5.0, 5.0));
    assert!(!ui.shows("Card"));
}

fn catalog() -> Element<'static, ()> {
    column![hover_card(
        button("Ada Lovelace").class(ButtonClass::Text).on_press(()),
        column![
            text("Ada Lovelace"),
            text("Mathematician and writer, London.").class(TextClass::Variant),
        ]
        .spacing(spacing::XS),
    )]
    .width(Length::Fill)
    .align_x(iced::Alignment::Center)
    .padding(spacing::LG)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (420.0, 180.0), catalog());
    let anchor = ui.find("Ada Lovelace").expect("anchor").bounds();
    let start = Instant::now();

    ui.point_at(anchor.center());
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::CursorMoved {
            position: anchor.center(),
        }),
        Event::Window(window::Event::RedrawRequested(start)),
        Event::Window(window::Event::RedrawRequested(
            start + Duration::from_millis(600),
        )),
    ]);
    ui.point_at(Point::new(2.0, 2.0));

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn hover_card_light() {
    snapshot(&Theme::light(), "hover_card_light");
}

#[test]
fn hover_card_dark() {
    snapshot(&Theme::dark(), "hover_card_dark");
}

#[test]
fn hover_card_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "hover_card_light_high_contrast");
}
