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

//! Popover: the panel shows while open and publishes the dismiss message on
//! a click outside or on Escape, but not on a click inside; the focus moves
//! into the panel when it opens, Tab stays inside it, and the focus returns
//! to the anchor when it closes; golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::widget::{Space, button, column, container, row, text, text_input};
use iced::{Event, Length, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::focusable;
use oito2_iced::widget::popover::{self, popover};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Toggle,
    Dismiss,
    Inside,
}

fn view(open: bool) -> Element<'static, Message> {
    column![
        popover(
            button("Anchor").on_press(Message::Toggle),
            button("Inside").on_press(Message::Inside),
        )
        .open(open)
        .on_dismiss(Message::Dismiss),
        text("Below").width(Length::Fill).height(Length::Fill),
    ]
    .padding(20)
    .into()
}

#[test]
fn the_panel_shows_only_while_open() {
    let mut closed = simulator::Simulator::with_settings(support::settings(), view(false));
    assert!(closed.find("Inside").is_err());

    let mut open = simulator::Simulator::with_settings(support::settings(), view(true));
    let anchor = open.find("Anchor").expect("anchor").bounds();
    let inside = open.find("Inside").expect("panel content").bounds();

    assert!(
        inside.y >= anchor.y + anchor.height,
        "the panel opens below the anchor"
    );
}

#[test]
fn a_click_outside_dismisses() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.point_at(Point::new(600.0, 600.0));
    ui.simulate(simulator::click());

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Dismiss]);
}

#[test]
fn escape_dismisses() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.tap_key(Named::Escape);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Dismiss]);
}

#[test]
fn clicks_inside_the_panel_and_on_the_anchor_do_not_dismiss() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));

    ui.click("Inside").expect("panel content");
    ui.click("Anchor").expect("anchor");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Inside, Message::Toggle]);
}

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

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Toggle,
    Close,
    First,
    Second,
}

struct App {
    open: bool,
    close_on_click_outside: bool,
}

impl Model for App {
    type Message = Action;

    fn view(&self) -> Element<'_, Action> {
        let item = |label: &'static str, message: Action| {
            focusable(button(label).on_press(message.clone())).on_activate(message)
        };

        oito2_iced::focus::navigation(
            column![
                popover(
                    item("Anchor", Action::Toggle),
                    column![item("First", Action::First), item("Second", Action::Second)],
                )
                .open(self.open)
                .close_on_click_outside(self.close_on_click_outside)
                .on_dismiss(Action::Close),
                text("Below").width(Length::Fill).height(Length::Fill),
            ]
            .padding(20),
        )
        .into()
    }

    fn update(&mut self, message: Action) {
        match message {
            Action::Toggle => self.open = !self.open,
            Action::Close => self.open = false,
            Action::First | Action::Second => {}
        }
    }
}

fn app(close_on_click_outside: bool) -> Harness<App> {
    Harness::new(
        App {
            open: false,
            close_on_click_outside,
        },
        Size::new(500.0, 400.0),
    )
}

/// Whether the focused widget contains the text `label`.
fn focus_is_on(ui: &mut Harness<App>, label: &'static str) -> bool {
    let target = ui.find(label).expect("label shows").bounds();

    ui.focused()
        .is_some_and(|focused| focused.contains(target.center()))
}

#[test]
fn opening_focuses_the_panel_and_closing_returns_the_focus_to_the_anchor() {
    let mut ui = app(true);

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert!(ui.model.open);
    assert!(
        focus_is_on(&mut ui, "First"),
        "the first element takes the focus"
    );

    let anchor = ui.find("Anchor").expect("anchor").bounds();
    let first = ui.find("First").expect("panel").bounds();
    assert!(first.y > anchor.y + anchor.height, "the panel opens below");

    ui.key(Named::Escape);
    assert!(!ui.model.open);
    assert!(ui.find("First").is_none());
    assert!(
        focus_is_on(&mut ui, "Anchor"),
        "the focus returns to the anchor"
    );
}

#[test]
fn tab_cycles_inside_the_open_panel() {
    let mut ui = app(true);

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    ui.key(Named::Tab);
    assert!(focus_is_on(&mut ui, "Second"));

    ui.key(Named::Tab);
    assert!(
        focus_is_on(&mut ui, "First"),
        "Tab wraps to the first element"
    );

    ui.send(
        simulator::tap_key(Named::Tab, None).map(|event| match event {
            Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                text,
                repeat,
                ..
            }) => Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key,
                modified_key,
                physical_key,
                location,
                modifiers: iced::keyboard::Modifiers::SHIFT,
                text,
                repeat,
            }),
            other => other,
        }),
    );
    assert!(
        focus_is_on(&mut ui, "Second"),
        "Shift+Tab wraps to the last one"
    );

    ui.key(Named::Enter);
    assert_eq!(ui.messages.last(), Some(&Action::Second));
}

#[test]
fn a_click_outside_can_be_kept_from_closing_it() {
    let mut ui = app(false);

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    ui.click_at(Point::new(450.0, 350.0));
    assert!(ui.model.open, "a click outside keeps it open");

    ui.key(Named::Escape);
    assert!(!ui.model.open, "Escape still closes it");
}

#[test]
fn the_panel_can_take_the_anchor_width() {
    let view = || -> Element<'static, ()> {
        column![
            popover(
                container(text("Anchor")).width(200),
                text_input("", "").id("content").width(Length::Fill),
            )
            .open(true)
            .match_anchor_width(true),
        ]
        .padding(20)
        .into()
    };
    let mut ui = simulator::Simulator::with_settings(support::settings(), view());

    let content = ui.find(selector::id("content")).expect("content").bounds();
    assert_eq!(content.width, 200.0 - 2.0 * spacing::MD);
}

#[test]
fn the_panel_is_at_most_360_wide() {
    let view: Element<'static, ()> = column![
        popover(
            text("Anchor"),
            text_input("", "").id("content").width(Length::Fill),
        )
        .open(true),
    ]
    .padding(20)
    .into();
    let mut ui = simulator::Simulator::with_settings(support::settings(), view);

    let content = ui.find(selector::id("content")).expect("content").bounds();
    assert_eq!(content.width, popover::MAX_WIDTH - 2.0 * spacing::MD);
}

fn catalog() -> Element<'static, ()> {
    let panel = || {
        column![
            text("Share link"),
            text("Anyone with the link can view.").class(TextClass::Variant),
        ]
        .spacing(spacing::XS)
    };
    let anchor = |label: &'static str| button(label).class(ButtonClass::Outlined).on_press(());

    row![
        popover(anchor("Plain"), panel())
            .open(true)
            .focus_on_open(false),
        Space::new().width(Length::Fill),
        popover(anchor("Arrow"), panel())
            .open(true)
            .arrow(true)
            .focus_on_open(false),
    ]
    .padding(spacing::LG)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (600.0, 200.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn popover_light() {
    snapshot(&Theme::light(), "popover_light");
}

#[test]
fn popover_dark() {
    snapshot(&Theme::dark(), "popover_dark");
}

#[test]
fn popover_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "popover_light_high_contrast");
}
