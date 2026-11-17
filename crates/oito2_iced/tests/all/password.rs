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

//! Password fields: the show and hide button, the Caps Lock warning, the
//! disabled field, the height per density, and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

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
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::password;
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
    Password(String),
    LogIn,
}

#[derive(Default)]
struct Login {
    password: String,
    error: Option<&'static str>,
    disabled: bool,
}

impl Model for Login {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut field = password::password_field(&Localizations::en(), &self.password)
            .id("password")
            .error(self.error)
            .on_submit(Message::LogIn);

        if !self.disabled {
            field = field.on_input(Message::Password);
        }

        oito2_iced::focus::navigation(column![field]).into()
    }

    fn update(&mut self, message: Message) {
        if let Message::Password(password) = message {
            self.password = password;
        }
    }
}

/// The center of the reveal button at the end of `field`.
fn reveal(field: Rectangle) -> Point {
    Point::new(field.x + field.width - field.height / 2.0, field.center_y())
}

fn caps_lock(ui: &mut Harness<Login>) {
    ui.key(Named::CapsLock);
}

fn warns(ui: &mut Harness<Login>) -> bool {
    ui.find("Caps Lock is on").is_some()
}

#[test]
fn the_eye_button_shows_and_hides_the_text() {
    let mut ui = Harness::new(Login {
        password: "secret".into(),
        ..Login::default()
    });
    let field = ui.bounds("password");

    assert!(ui.find("secret").is_none(), "hidden at first");

    ui.click_at(reveal(field));
    assert!(ui.find("secret").is_some());

    ui.click_at(reveal(field));
    assert!(ui.find("secret").is_none());
}

#[test]
fn typing_publishes_the_text_and_keeps_it_while_revealed() {
    let mut ui = Harness::new(Login::default());
    let field = ui.bounds("password");

    ui.click_at(field.center());
    ui.type_text("AbC");
    ui.key(Named::Enter);

    assert_eq!(ui.model.password, "AbC");
    assert_eq!(
        ui.messages,
        vec![
            Message::Password("A".into()),
            Message::Password("Ab".into()),
            Message::Password("AbC".into()),
            Message::LogIn,
        ]
    );

    ui.click_at(reveal(field));
    assert!(ui.find("AbC").is_some());
}

#[test]
fn warns_while_caps_lock_is_on() {
    let mut ui = Harness::new(Login::default());
    let field = ui.bounds("password");

    ui.click_at(field.center());
    assert!(!warns(&mut ui));

    caps_lock(&mut ui);
    assert!(warns(&mut ui));

    caps_lock(&mut ui);
    assert!(!warns(&mut ui));

    ui.type_text("Q");
    assert!(warns(&mut ui), "a capital without Shift");

    ui.type_text("q");
    assert!(!warns(&mut ui), "a small letter without Shift");
}

#[test]
fn warns_about_caps_lock_only_while_focused() {
    let mut ui = Harness::new(Login::default());
    let field = ui.bounds("password");

    caps_lock(&mut ui);
    assert!(!warns(&mut ui), "not focused");

    ui.click_at(field.center());
    assert!(warns(&mut ui));

    ui.click_at(Point::new(390.0, 290.0));
    assert!(!warns(&mut ui));
}

#[test]
fn warns_about_caps_lock_next_to_an_error() {
    let mut ui = Harness::new(Login {
        error: Some("Wrong password"),
        ..Login::default()
    });
    let field = ui.bounds("password");

    ui.click_at(field.center());
    caps_lock(&mut ui);
    assert!(ui.find("Wrong password").is_some());
    assert!(warns(&mut ui));

    caps_lock(&mut ui);
    assert!(ui.find("Wrong password").is_some());
    assert!(!warns(&mut ui));
}

#[test]
fn a_disabled_field_ignores_the_button_and_tab() {
    let field = Harness::new(Login::default()).bounds("password");
    let mut ui = Harness::new(Login {
        password: "secret".into(),
        disabled: true,
        ..Login::default()
    });

    ui.click_at(reveal(field));
    ui.key(Named::Tab);
    assert!(ui.messages.is_empty());
    assert!(ui.find(selector::is_focused()).is_none());
}

#[test]
fn password_fields_are_as_tall_as_the_controls_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = Harness::new(Login::default());
        assert_eq!(ui.bounds("password").height, height, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let en = Localizations::en();

    column![
        password::password_field(&en, "")
            .label("Password")
            .placeholder("Your password")
            .on_input(Message::Password),
        password::password_field(&en, "secret")
            .label("Password")
            .on_input(Message::Password),
        password::password_field(&en, "secret")
            .error(Some("Wrong password"))
            .on_input(Message::Password),
        password::password_field::<Message>(&Localizations::pt_br(), "secret").label("Disabled"),
        password::password_field(&en, "")
            .id("caps")
            .on_input(Message::Password),
    ]
    .spacing(12)
    .padding(12)
    .width(360)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (360.0, 400.0), catalog());
    ui.click(selector::id("caps")).expect("caps field");
    ui.tap_key(Named::CapsLock);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn password_light() {
    snapshot(&Theme::light(), "password_light");
}

#[test]
fn password_dark() {
    snapshot(&Theme::dark(), "password_dark");
}

#[test]
fn password_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "password_light_high_contrast");
}
