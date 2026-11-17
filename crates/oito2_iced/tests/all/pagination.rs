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

//! Pagination: page buttons and arrows, the compact form, the page size
//! selector, the reading direction, the disabled state, localization and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

#![cfg(feature = "svg")]

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::widget::{column, container};
use iced::{Event, Length, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, sizes};
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::pagination;
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
    fn new(model: App, width: f32) -> Self {
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
            size: Size::new(width, 400.0),
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

    fn shows(&mut self, text: &'static str) -> bool {
        self.find(text).is_some()
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
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Page(u32),
    PageSize(u32),
    Menu(bool),
    After,
}

struct App {
    page: u32,
    count: u32,
    page_size: Option<u32>,
    menu: bool,
    enabled: bool,
    direction: ReadingDirection,
    localizations: Localizations,
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        let mut pager = pagination::pagination(self.page, self.count)
            .direction(self.direction)
            .localizations(&self.localizations);

        if self.enabled {
            pager = pager.on_change(Message::Page);
        }

        if let Some(size) = self.page_size {
            pager = pager
                .page_sizes([10, 20, 50], size, Message::PageSize)
                .page_size_menu(self.menu, Message::Menu);
        }

        focus::navigation(
            column![
                pager,
                button::filled("After").on_press(Message::After).id("after"),
            ]
            .spacing(8),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Page(page) => self.page = page,
            Message::PageSize(size) => self.page_size = Some(size),
            Message::Menu(open) => self.menu = open,
            Message::After => {}
        }
    }
}

fn app(page: u32, count: u32, width: f32) -> Harness {
    Harness::new(
        App {
            page,
            count,
            page_size: None,
            menu: false,
            enabled: true,
            direction: ReadingDirection::LeftToRight,
            localizations: Localizations::en(),
        },
        width,
    )
}

#[test]
fn page_buttons_and_arrows_change_the_page() {
    let mut ui = app(5, 10, 600.0);

    assert!(ui.shows("1") && ui.shows("4") && ui.shows("6") && ui.shows("…"));
    assert!(!ui.shows("3"), "an ellipsis stands for pages 2 and 3");

    ui.click("10");
    assert_eq!(ui.model.page, 10);
    assert!(ui.shows("6") && !ui.shows("5"));

    // The previous page button comes first.
    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert_eq!(ui.model.page, 9);
}

#[test]
fn page_buttons_are_as_tall_and_wide_as_the_control_height() {
    let mut ui = app(5, 10, 600.0);

    ui.key(Named::Tab);
    ui.key(Named::Tab);
    let button = ui.focused().expect("the button of page 1");
    let label = ui.find("1").expect("the label").bounds();

    assert!(button.contains(label.center()));
    assert_eq!(button.height, sizes::BUTTON_HEIGHT);
    assert_eq!(button.width, sizes::BUTTON_HEIGHT);
}

#[test]
fn narrow_widths_show_the_compact_form() {
    let mut ui = app(5, 10, 300.0);

    assert!(ui.shows("Page 5 of 10"));
    assert!(!ui.shows("1"));

    ui.key(Named::Tab);
    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert!(ui.shows("Page 6 of 10"));
}

#[test]
fn the_compact_form_follows_the_locale() {
    let mut ui = app(5, 10, 300.0);
    ui.model.localizations = Localizations::pt_br();
    ui.model.page_size = Some(20);

    assert!(ui.shows("Página 5 de 10"));
    assert!(ui.shows("Itens por página"));
}

#[test]
fn the_compact_form_keeps_the_page_size_selector() {
    let mut ui = app(1, 4, 300.0);
    ui.model.page_size = Some(20);

    assert!(ui.shows("Page 1 of 4"));
    assert!(ui.shows("Items per page"));
    assert!(ui.shows("20"));
}

#[test]
fn the_page_size_selector_picks_a_size() {
    let mut ui = app(1, 4, 600.0);
    ui.model.page_size = Some(20);

    ui.click("20");
    assert!(ui.model.menu, "the click opens the menu");

    ui.click("50");
    assert_eq!(ui.model.page_size, Some(50));
    assert!(!ui.model.menu);
}

#[test]
fn the_first_page_disables_the_previous_arrow_and_the_last_the_next() {
    let mut ui = app(1, 3, 600.0);
    let after = ui.find(selector::id("after")).expect("the button").bounds();

    // The previous arrow is disabled: pages 1, 2 and 3, then the next arrow.
    for _ in 0..4 {
        ui.key(Named::Tab);
    }
    ui.key(Named::Enter);
    assert_eq!(ui.model.page, 2);

    // The previous arrow and pages 1, 2 and 3, then the button after.
    let mut ui = app(3, 3, 600.0);
    for _ in 0..5 {
        ui.key(Named::Tab);
    }
    assert_eq!(ui.focused(), Some(after), "the next arrow is disabled");
}

#[test]
fn right_to_left_puts_the_next_arrow_first() {
    let mut ui = app(5, 10, 600.0);
    ui.model.direction = ReadingDirection::RightToLeft;

    let one = ui.find("1").expect("page 1").bounds();
    let ten = ui.find("10").expect("page 10").bounds();
    assert!(ten.x < one.x);

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert_eq!(ui.model.page, 6);
}

#[test]
fn without_a_message_every_button_is_disabled() {
    let mut ui = app(5, 10, 600.0);
    ui.model.enabled = false;
    let after = ui.find(selector::id("after")).expect("the button").bounds();

    ui.click("6");
    ui.key(Named::Tab);

    assert_eq!(ui.model.page, 5);
    assert_eq!(ui.focused(), Some(after));
}

fn catalog() -> Element<'static, ()> {
    let pager = |page, count| pagination::pagination(page, count).on_change(|_| ());

    column![
        pager(5, 10),
        pager(1, 4)
            .page_sizes([10, 20, 50], 20, |_| ())
            .page_size_menu(false, |_| ()),
        container(pager(3, 12).page_sizes([10, 20], 10, |_| ())).width(260),
        pagination::pagination::<()>(2, 3),
    ]
    .spacing(16)
    .padding(16)
    .width(Length::Fill)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (480.0, 260.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn pagination_light() {
    snapshot(&Theme::light(), "pagination_light");
}

#[test]
fn pagination_dark() {
    snapshot(&Theme::dark(), "pagination_dark");
}

#[test]
fn pagination_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "pagination_light_high_contrast");
}
