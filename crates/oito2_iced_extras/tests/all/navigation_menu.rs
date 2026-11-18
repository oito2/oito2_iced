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

//! Navigation menus: presses, Escape, the hover delays, Down with the focus
//! moving into the panel, disabled triggers, the line under the selected
//! item, right to left and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{column, container, text};
use iced::{Event, Length, Point, Rectangle, Size, mouse, window};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::{Element, Theme};
use oito2_iced_extras::navigation_menu::{self, Event as MenuEvent, MenuState};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Menu(MenuEvent),
    Blog,
    Inside,
}

struct App {
    menu: MenuState,
    selected: Option<usize>,
    interactive: bool,
    direction: ReadingDirection,
}

impl App {
    fn new() -> Self {
        Self {
            menu: MenuState::default(),
            selected: Some(2),
            interactive: true,
            direction: ReadingDirection::LeftToRight,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu = navigation_menu::navigation_menu(
            &self.menu,
            [
                navigation_menu::item("Products").panel(
                    oito2_iced::widget::button::text("Products panel").on_press(Message::Inside),
                ),
                navigation_menu::item("Docs").panel(text("Docs panel")),
                navigation_menu::item("Blog").on_press(Message::Blog),
                navigation_menu::item("Soon"),
            ],
        )
        .selected(self.selected)
        .direction(self.direction);
        let menu = if self.interactive {
            menu.on_event(Message::Menu)
        } else {
            menu
        };

        oito2_iced::focus::navigation(
            container(menu)
                .center_x(Length::Fill)
                .padding(20)
                .height(Length::Fill),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        if let Message::Menu(event) = message {
            self.menu.apply(event);
        }
    }
}

/// Runs the application: rebuilds the interface from the model for every
/// batch of events and applies the published messages.
struct Harness {
    app: App,
    size: Size,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    messages: Vec<Message>,
    now: Instant,
}

impl Harness {
    fn new(app: App) -> Self {
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

        let mut harness = Self {
            app,
            size: Size::new(600.0, 400.0),
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            messages: Vec::new(),
            now: Instant::now(),
        };
        harness.redraw(Duration::ZERO);
        harness
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.app.view(),
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
            self.app.update(message);
        }
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut find = selector.find();
        let mut ui = UserInterface::build(
            self.app.view(),
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

    fn bounds(&mut self, label: &'static str) -> Rectangle {
        self.find(label).expect("label shows").bounds()
    }

    fn shows(&mut self, label: &'static str) -> bool {
        self.find(label).is_some()
    }

    fn focus_is_on(&mut self, label: &'static str) -> bool {
        let target = self.bounds(label);

        self.find(selector::is_focused())
            .is_some_and(|focused| focused.bounds().contains(target.center()))
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

    /// Sends a redraw `after` the start of the test.
    fn redraw(&mut self, after: Duration) {
        let now = self.now + after;
        self.send([Event::Window(window::Event::RedrawRequested(now))]);
    }
}

#[test]
fn a_press_opens_a_panel_and_escape_closes_it() {
    let mut ui = Harness::new(App::new());
    assert!(!ui.shows("Products panel"));

    let products = ui.bounds("Products");
    ui.click_at(products.center());
    assert_eq!(ui.app.menu.open(), Some(0));
    assert!(ui.shows("Products panel"));
    let panel = ui.bounds("Products panel");
    assert!(
        panel.y > products.y + products.height,
        "the panel opens below"
    );

    ui.key(Named::Escape);
    assert_eq!(ui.app.menu.open(), None);
    assert!(!ui.shows("Products panel"));

    let blog = ui.bounds("Blog");
    ui.click_at(blog.center());
    assert_eq!(ui.messages.last(), Some(&Message::Blog));
}

#[test]
fn a_second_press_on_the_trigger_closes_its_panel() {
    let mut ui = Harness::new(App::new());

    let docs = ui.bounds("Docs");
    ui.click_at(docs.center());
    assert_eq!(ui.app.menu.open(), Some(1));
    ui.click_at(docs.center());
    assert_eq!(ui.app.menu.open(), None);
}

#[test]
fn hover_opens_after_a_delay_switches_at_once_and_leaving_closes() {
    let mut ui = Harness::new(App::new());

    let products = ui.bounds("Products");
    ui.move_to(products.center());
    ui.redraw(Duration::from_millis(10));
    ui.redraw(Duration::from_millis(60));
    assert!(!ui.shows("Products panel"), "the panel waits for the delay");

    ui.redraw(Duration::from_millis(200));
    assert_eq!(
        ui.messages.last(),
        Some(&Message::Menu(MenuEvent::Opened {
            index: 0,
            focus: false
        }))
    );
    assert!(ui.shows("Products panel"));
    assert!(
        !ui.focus_is_on("Products panel"),
        "a hover leaves the focus where it was"
    );

    let docs = ui.bounds("Docs");
    ui.move_to(docs.center());
    assert!(ui.shows("Docs panel"), "another trigger switches at once");
    assert!(!ui.shows("Products panel"));

    // From the trigger into its panel, the panel stays open.
    let panel = ui.bounds("Docs panel");
    ui.move_to(Point::new(docs.center_x(), docs.y + docs.height + 2.0));
    ui.move_to(panel.center());
    ui.redraw(Duration::from_millis(300));
    ui.redraw(Duration::from_millis(700));
    assert!(ui.shows("Docs panel"));

    ui.move_to(Point::new(5.0, 390.0));
    ui.redraw(Duration::from_millis(800));
    ui.redraw(Duration::from_millis(900));
    assert!(ui.shows("Docs panel"), "the panel waits before closing");
    ui.redraw(Duration::from_millis(1100));
    assert!(!ui.shows("Docs panel"));
}

#[test]
fn leaving_the_trigger_before_the_delay_cancels_the_opening() {
    let mut ui = Harness::new(App::new());

    let products = ui.bounds("Products");
    ui.move_to(products.center());
    ui.redraw(Duration::from_millis(10));
    ui.move_to(Point::new(5.0, 390.0));
    ui.redraw(Duration::from_millis(200));
    ui.redraw(Duration::from_millis(600));
    assert!(!ui.shows("Products panel"));
    assert!(ui.messages.is_empty());
}

#[test]
fn down_opens_a_panel_with_the_focus_inside() {
    let mut ui = Harness::new(App::new());

    ui.key(Named::Tab);
    assert!(ui.focus_is_on("Products"));
    ui.key(Named::ArrowDown);
    assert_eq!(ui.app.menu.open(), Some(0));
    assert!(ui.focus_is_on("Products panel"));

    ui.key(Named::Escape);
    assert!(!ui.shows("Products panel"));
    assert!(
        ui.focus_is_on("Products"),
        "the focus returns to the trigger"
    );
}

#[test]
fn enter_on_a_trigger_opens_its_panel_with_the_focus_inside() {
    let mut ui = Harness::new(App::new());

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert_eq!(ui.app.menu.open(), Some(0));
    assert!(ui.focus_is_on("Products panel"));
    ui.key(Named::Enter);
    assert_eq!(ui.messages.last(), Some(&Message::Inside));
}

#[test]
fn tab_skips_disabled_items_and_triggers_without_events() {
    let mut ui = Harness::new(App {
        interactive: false,
        ..App::new()
    });

    let products = ui.bounds("Products");
    ui.click_at(products.center());
    ui.move_to(products.center());
    ui.redraw(Duration::from_millis(10));
    ui.redraw(Duration::from_millis(400));
    assert!(!ui.shows("Products panel"));

    ui.key(Named::Tab);
    assert!(ui.focus_is_on("Blog"), "only the link takes the focus");
    ui.key(Named::Tab);
    assert!(!ui.focus_is_on("Products") && !ui.focus_is_on("Soon"));
    assert!(ui.messages.is_empty());
}

#[test]
fn right_to_left_places_the_first_item_on_the_right() {
    let mut ltr = Harness::new(App::new());
    let mut rtl = Harness::new(App {
        direction: ReadingDirection::RightToLeft,
        ..App::new()
    });

    let (products, blog) = (ltr.bounds("Products"), ltr.bounds("Blog"));
    assert!(products.x < blog.x);
    let (products, blog) = (rtl.bounds("Products"), rtl.bounds("Blog"));
    assert!(products.x > blog.x);

    // The panel lines up with the right edge of its trigger, where the
    // label ends (the chevron comes first): the panel text ends a large
    // padding before the panel edge, the label a medium one before the
    // trigger edge.
    let label = rtl.bounds("Docs");
    rtl.click_at(label.center());
    let panel_text = rtl.bounds("Docs panel");
    let panel_end = panel_text.x + panel_text.width + spacing::LG;
    let trigger_end = label.x + label.width + spacing::MD;
    assert!(
        (panel_end - trigger_end).abs() < 1.0,
        "{panel_end} {trigger_end}"
    );

    // Tab still follows the reading order.
    rtl.key(Named::Escape);
    let mut rtl = Harness::new(App {
        direction: ReadingDirection::RightToLeft,
        ..App::new()
    });
    rtl.key(Named::Tab);
    assert!(rtl.focus_is_on("Products"));
}

fn snapshot_view(menu: MenuState, direction: ReadingDirection) -> Element<'static, ()> {
    container(
        column![
            navigation_menu::navigation_menu(
                &menu,
                [
                    navigation_menu::item("Home").on_press(()),
                    navigation_menu::item("Components").panel(column![
                        text("Inputs"),
                        text("Fields, pickers and toggles"),
                        oito2_iced::widget::button::text("Overlays").on_press(()),
                    ]),
                    navigation_menu::item("Docs").panel(text("Getting started")),
                    navigation_menu::item("Blog").on_press(()),
                ],
            )
            .selected(Some(0))
            .direction(direction)
            .on_event(|_| ()),
        ]
        .width(Length::Fill),
    )
    .padding(16)
    .into()
}

/// The line under the selected item is drawn in the accent color, centered
/// on the item and on its bottom edge.
#[test]
fn the_line_sits_under_the_selected_item() {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (480.0, 240.0),
        snapshot_view(MenuState::default(), ReadingDirection::LeftToRight),
    );
    let home = ui.find("Home").expect("home").bounds();
    let theme = Theme::light();
    let image = support::pixels(&ui.snapshot(&theme).expect("snapshot"), "line");

    // The Home button is the parent of its label: the line is at the
    // bottom of the row of items, below the label.
    let accent = theme.roles().semantic.primary_text;
    let row_bottom = 16.0 + oito2_iced::density::control_height();
    let at = Point::new(home.center_x(), row_bottom - 1.0);
    let pixel = support::pixel_at(&image, at, 480.0);
    let close = |a: f32, b: f32| (a - b).abs() < 0.05;
    assert!(
        close(pixel.r, accent.r) && close(pixel.g, accent.g) && close(pixel.b, accent.b),
        "{pixel:?} is the accent {accent:?}"
    );
}

fn opened(index: usize) -> MenuState {
    let mut menu = MenuState::default();
    menu.apply(MenuEvent::Opened {
        index,
        focus: false,
    });
    menu
}

fn snapshot(theme: &Theme, menu: MenuState, direction: ReadingDirection, name: &str) {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (480.0, 240.0),
        snapshot_view(menu, direction),
    );
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn navigation_menu_light() {
    snapshot(
        &Theme::light(),
        MenuState::default(),
        ReadingDirection::LeftToRight,
        "navigation_menu_light",
    );
}

#[test]
fn navigation_menu_dark() {
    snapshot(
        &Theme::dark(),
        MenuState::default(),
        ReadingDirection::LeftToRight,
        "navigation_menu_dark",
    );
}

#[test]
fn navigation_menu_open_light() {
    snapshot(
        &Theme::light(),
        opened(1),
        ReadingDirection::LeftToRight,
        "navigation_menu_open_light",
    );
}

#[test]
fn navigation_menu_open_dark() {
    snapshot(
        &Theme::dark(),
        opened(1),
        ReadingDirection::LeftToRight,
        "navigation_menu_open_dark",
    );
}

#[test]
fn navigation_menu_open_rtl_light() {
    snapshot(
        &Theme::light(),
        opened(1),
        ReadingDirection::RightToLeft,
        "navigation_menu_open_rtl_light",
    );
}
