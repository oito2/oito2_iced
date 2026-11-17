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

//! Calendars and date pickers: picks with the pointer and the keyboard,
//! refused days, months and limits, ranges, the focus order, the
//! localized heading, the reading direction, the picker popover, and
//! golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

#![cfg(feature = "svg")]

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
use oito2_iced::date::Date;
use oito2_iced::density;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::calendar::{self, DateRange};
use oito2_iced::widget::helpers::ReadingDirection;
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

const SIZE: Size = Size::new(480.0, 480.0);

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

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    fn shows(&mut self, text: &'static str) -> bool {
        self.find(text).is_some()
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn key_with(&mut self, key: Named, modifiers: keyboard::Modifiers) {
        let key = keyboard::Key::Named(key);

        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: key.clone(),
                modified_key: key.clone(),
                physical_key: keyboard::key::Physical::Unidentified(
                    keyboard::key::NativeCode::Unidentified,
                ),
                location: keyboard::Location::Standard,
                modifiers,
                repeat: false,
                text: None,
            }),
            simulator::release_key(key),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }
}

fn day(year: i32, month: u32, day: u32) -> Date {
    Date::new(year, month, day).expect("valid date")
}

/// The side of a day cell at the pointer density.
fn side() -> f32 {
    density::control_height() + 2.0
}

/// The center of the cell of `date` in a grid whose body (weekday names and
/// days) has `body` bounds and whose weeks start on Sunday, without week
/// numbers.
fn cell(body: Rectangle, date: Date) -> Point {
    let offset = date.first_of_month().weekday();
    let index = offset + date.day() - 1;
    let (row, column) = (index / 7, index % 7);
    let side = side();

    Point::new(
        body.x + side * (column as f32 + 0.5),
        body.y + side * 0.75 + side * (row as f32 + 0.5),
    )
}

/// The center of the next month button of a calendar whose grid body has
/// `body` bounds.
fn next_month(body: Rectangle) -> Point {
    let height = density::control_height();

    Point::new(body.x + body.width - height / 2.0, body.y - height / 2.0)
}

/// The center of the previous month button.
fn previous_month(body: Rectangle) -> Point {
    let height = density::control_height();

    Point::new(body.x + body.width - height * 1.5, body.y - height / 2.0)
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Day(Date),
    Range(DateRange),
    Open(bool),
}

/// A calendar of 2026 that refuses the 13th, opened on March.
struct Calendar {
    selected: Option<Date>,
    range: Option<DateRange>,
    range_mode: bool,
    enabled: bool,
    autofocus: bool,
    localizations: Localizations,
    direction: ReadingDirection,
    first: Date,
}

impl Calendar {
    fn new() -> Self {
        Self {
            selected: None,
            range: None,
            range_mode: false,
            enabled: true,
            autofocus: false,
            localizations: Localizations::en(),
            direction: ReadingDirection::LeftToRight,
            first: day(2026, 1, 1),
        }
    }
}

impl Model for Calendar {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut calendar = calendar::calendar(day(2026, 3, 20))
            .limits(self.first, day(2026, 12, 31))
            .initial_month(day(2026, 3, 1))
            .selectable(|date| date.day() != 13)
            .autofocus(self.autofocus)
            .localizations(&self.localizations)
            .direction(self.direction)
            .id("grid");

        if self.range_mode {
            calendar = calendar.selected_range(self.range);
            if self.enabled {
                calendar = calendar.on_range_select(Message::Range);
            }
        } else {
            calendar = calendar.selected(self.selected);
            if self.enabled {
                calendar = calendar.on_select(Message::Day);
            }
        }

        oito2_iced::focus::navigation(column![calendar].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Day(date) => self.selected = Some(date),
            Message::Range(range) => self.range = Some(range),
            Message::Open(_) => {}
        }
    }
}

#[test]
fn the_heading_follows_a_date_selected_from_outside() {
    let mut ui = Harness::new(Calendar {
        selected: Some(day(2026, 3, 15)),
        ..Calendar::new()
    });
    assert!(ui.shows("March 2026"));

    ui.model.selected = Some(day(2026, 8, 2));
    ui.send([]);
    assert!(ui.shows("August 2026"));
}

#[test]
fn picks_a_day_skips_refused_days_and_changes_month() {
    let mut ui = Harness::new(Calendar::new());
    let body = ui.bounds("grid");

    ui.click_at(cell(body, day(2026, 3, 13)));
    assert_eq!(ui.model.selected, None, "the 13th is refused");

    ui.click_at(cell(body, day(2026, 3, 10)));
    assert_eq!(ui.model.selected, Some(day(2026, 3, 10)));

    ui.click_at(next_month(body));
    assert!(ui.shows("April 2026"));
}

#[test]
fn the_keyboard_moves_the_focused_day_across_months() {
    let mut ui = Harness::new(Calendar::new());
    let body = ui.bounds("grid");

    ui.click_at(cell(body, day(2026, 3, 31)));
    assert_eq!(ui.model.selected, Some(day(2026, 3, 31)));

    ui.key(Named::ArrowRight);
    assert!(ui.shows("April 2026"));

    // Shift+Page Down overshoots the last date, so the focus stops on
    // December 31; Page Up then moves it to November 30.
    ui.key_with(Named::PageDown, keyboard::Modifiers::SHIFT);
    assert!(ui.shows("December 2026"));

    ui.key(Named::PageUp);
    ui.key(Named::Enter);
    assert_eq!(ui.model.selected, Some(day(2026, 11, 30)));
}

#[test]
fn home_end_and_the_vertical_arrows_move_within_weeks() {
    let mut ui = Harness::new(Calendar::new());
    let body = ui.bounds("grid");

    // March 10, 2026 is a Tuesday; weeks start on Sunday.
    ui.click_at(cell(body, day(2026, 3, 10)));
    ui.key(Named::Home);
    ui.key(Named::Space);
    assert_eq!(ui.model.selected, Some(day(2026, 3, 8)));

    ui.key(Named::End);
    ui.key(Named::Enter);
    assert_eq!(ui.model.selected, Some(day(2026, 3, 14)));

    ui.key(Named::ArrowUp);
    ui.key(Named::Enter);
    assert_eq!(ui.model.selected, Some(day(2026, 3, 7)));

    ui.key(Named::ArrowDown);
    ui.key(Named::ArrowDown);
    ui.key(Named::Enter);
    assert_eq!(ui.model.selected, Some(day(2026, 3, 21)));
}

#[test]
fn a_range_takes_two_picks_in_any_order() {
    let mut ui = Harness::new(Calendar {
        range_mode: true,
        ..Calendar::new()
    });
    let body = ui.bounds("grid");

    ui.click_at(cell(body, day(2026, 3, 20)));
    assert_eq!(ui.model.range, None);

    ui.click_at(cell(body, day(2026, 3, 5)));
    assert_eq!(
        ui.model.range,
        Some(DateRange::new(day(2026, 3, 5), day(2026, 3, 20)))
    );
    assert_eq!(ui.messages.len(), 1);
}

#[test]
fn the_month_buttons_stop_at_the_limits() {
    let mut ui = Harness::new(Calendar {
        first: day(2026, 3, 5),
        ..Calendar::new()
    });
    let body = ui.bounds("grid");

    ui.click_at(previous_month(body));
    assert!(ui.shows("March 2026"), "no month before the first date");

    ui.key(Named::Tab);
    assert_ne!(
        ui.focused(),
        None,
        "the next month button takes the focus first"
    );
}

#[test]
fn the_grid_follows_the_month_buttons_in_the_focus_order() {
    let mut ui = Harness::new(Calendar::new());
    let body = ui.bounds("grid");

    ui.key(Named::Tab);
    ui.key(Named::Tab);
    assert_ne!(ui.focused(), Some(body), "the month buttons come first");

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(body));

    let mut ui = Harness::new(Calendar {
        autofocus: true,
        ..Calendar::new()
    });

    ui.key(Named::Tab);
    assert_eq!(
        ui.focused(),
        Some(body),
        "with autofocus the grid comes first"
    );
}

#[test]
fn a_calendar_without_a_message_picks_nothing_and_skips_the_grid() {
    let mut ui = Harness::new(Calendar {
        enabled: false,
        ..Calendar::new()
    });
    let body = ui.bounds("grid");

    ui.click_at(cell(body, day(2026, 3, 10)));
    for _ in 0..3 {
        ui.key(Named::Tab);
        assert_ne!(ui.focused(), Some(body));
    }
    ui.key(Named::Enter);

    assert!(ui.messages.is_empty());

    ui.click_at(next_month(body));
    assert!(ui.shows("April 2026"), "the month buttons still browse");
}

#[test]
fn the_heading_is_localized() {
    let mut ui = Harness::new(Calendar {
        localizations: Localizations::pt_br(),
        ..Calendar::new()
    });

    assert!(ui.shows("março de 2026"));
}

#[test]
fn right_to_left_mirrors_the_grid_and_the_arrows() {
    let mut ui = Harness::new(Calendar {
        direction: ReadingDirection::RightToLeft,
        ..Calendar::new()
    });
    let body = ui.bounds("grid");
    let mirrored = |point: Point| Point::new(body.x + body.width - (point.x - body.x), point.y);

    ui.click_at(mirrored(cell(body, day(2026, 3, 10))));
    assert_eq!(ui.model.selected, Some(day(2026, 3, 10)));

    ui.key(Named::ArrowLeft);
    ui.key(Named::Enter);
    assert_eq!(ui.model.selected, Some(day(2026, 3, 11)));
}

/// A date picker of 2026, in single or range mode.
struct Picker {
    value: Option<Date>,
    range: Option<DateRange>,
    range_mode: bool,
    enabled: bool,
    open: bool,
    localizations: Localizations,
}

impl Picker {
    fn new() -> Self {
        Self {
            value: Some(day(2026, 3, 1)),
            range: None,
            range_mode: false,
            enabled: true,
            open: false,
            localizations: Localizations::en(),
        }
    }
}

impl Model for Picker {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut picker = calendar::date_picker(day(2026, 3, 20))
            .limits(day(2026, 1, 1), day(2026, 12, 31))
            .open(self.open)
            .localizations(&self.localizations)
            .id("picker")
            .width(300);

        if self.range_mode {
            picker = picker.range(self.range);
            if self.enabled {
                picker = picker.on_range_change(Message::Range);
            }
        } else {
            picker = picker.value(self.value);
            if self.enabled {
                picker = picker.on_change(Message::Day);
            }
        }

        if self.enabled {
            picker = picker.on_open_change(Message::Open);
        }

        oito2_iced::focus::navigation(column![picker].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Day(date) => self.value = Some(date),
            Message::Range(range) => self.range = Some(range),
            Message::Open(open) => self.open = open,
        }
    }
}

#[test]
fn the_picker_opens_a_calendar_and_shows_the_picked_date() {
    let mut ui = Harness::new(Picker::new());
    assert!(ui.shows("03/01/2026"));

    let field = ui.bounds("picker");
    ui.click_at(field.center());
    assert!(ui.model.open);

    // The calendar takes the focus when it opens.
    ui.send([]);
    let body = ui.focused().expect("the grid has the focus");

    ui.click_at(cell(body, day(2026, 3, 18)));
    assert!(!ui.model.open, "a pick closes the calendar");
    assert_eq!(ui.model.value, Some(day(2026, 3, 18)));
    assert!(ui.shows("03/18/2026"));
}

#[test]
fn a_range_picker_shows_both_ends_and_picks_with_the_keyboard() {
    let mut ui = Harness::new(Picker {
        range_mode: true,
        range: Some(DateRange::new(day(2026, 3, 2), day(2026, 3, 4))),
        ..Picker::new()
    });
    assert!(ui.shows("03/02/2026 – 03/04/2026"));

    ui.key(Named::Tab);
    ui.key(Named::Enter);
    assert!(ui.model.open);
    ui.send([]);

    // The focus starts on the range start, March 2.
    ui.key(Named::ArrowRight);
    ui.key(Named::Enter);
    ui.key(Named::ArrowDown);
    ui.key(Named::Enter);

    assert_eq!(
        ui.model.range,
        Some(DateRange::new(day(2026, 3, 3), day(2026, 3, 10)))
    );
    assert!(!ui.model.open);
    assert!(ui.shows("03/03/2026 – 03/10/2026"));
}

#[test]
fn alt_down_opens_and_escape_closes_the_picker() {
    let mut ui = Harness::new(Picker::new());

    ui.key(Named::Tab);
    ui.key(Named::ArrowDown);
    assert!(!ui.model.open, "Down alone does nothing");

    ui.key_with(Named::ArrowDown, keyboard::Modifiers::ALT);
    assert!(ui.model.open);

    ui.send([]);
    ui.key(Named::Escape);
    assert!(!ui.model.open);
    assert_eq!(ui.model.value, Some(day(2026, 3, 1)));
}

#[test]
fn a_picker_without_a_message_is_disabled() {
    let mut ui = Harness::new(Picker {
        enabled: false,
        ..Picker::new()
    });
    let field = ui.bounds("picker");

    ui.click_at(field.center());
    ui.key(Named::Tab);
    ui.key(Named::Enter);

    assert!(ui.messages.is_empty());
    assert_eq!(ui.focused(), None);
}

#[test]
fn the_picker_shows_the_localized_format() {
    let mut ui = Harness::new(Picker {
        localizations: Localizations::pt_br(),
        ..Picker::new()
    });
    assert!(ui.shows("01/03/2026"));

    let mut ui = Harness::new(Picker {
        value: None,
        localizations: Localizations::pt_br(),
        ..Picker::new()
    });
    assert!(ui.shows("dd/mm/aaaa"));
}

fn catalog() -> Element<'static, Message> {
    let en = Localizations::en();
    let pt = Localizations::pt_br();

    row![
        column![
            calendar::calendar(day(2026, 10, 20))
                .limits(day(2026, 1, 1), day(2026, 12, 31))
                .show_week_numbers(true)
                .selected_range(Some(DateRange::new(day(2026, 10, 6), day(2026, 10, 14))))
                .on_range_select(Message::Range),
            calendar::calendar(day(2026, 10, 20))
                .limits(day(2026, 10, 3), day(2026, 12, 31))
                .selectable(|date| date.weekday() != 0)
                .selected(Some(day(2026, 10, 23)))
                .localizations(&pt)
                .on_select(Message::Day),
        ]
        .spacing(16),
        column![
            calendar::date_picker(day(2026, 10, 20))
                .value(Some(day(2026, 10, 23)))
                .label("Due date")
                .on_change(Message::Day)
                .on_open_change(Message::Open)
                .width(240),
            calendar::date_picker(day(2026, 10, 20))
                .label("Data")
                .localizations(&pt)
                .on_change(Message::Day)
                .on_open_change(Message::Open)
                .width(240),
            calendar::date_picker(day(2026, 10, 20))
                .range(Some(DateRange::new(day(2026, 10, 6), day(2026, 10, 14))))
                .localizations(&en)
                .on_range_change(Message::Range)
                .on_open_change(Message::Open)
                .width(240),
            calendar::date_picker::<Message>(day(2026, 10, 20))
                .value(Some(day(2026, 10, 23)))
                .label("Disabled")
                .width(240),
        ]
        .spacing(16),
    ]
    .spacing(24)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (580.0, 640.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn calendar_light() {
    snapshot(&Theme::light(), "calendar_light");
}

#[test]
fn calendar_dark() {
    snapshot(&Theme::dark(), "calendar_dark");
}

#[test]
fn calendar_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "calendar_light_high_contrast");
}

fn open_picker() -> Element<'static, Message> {
    column![
        calendar::date_picker(day(2026, 10, 20))
            .value(Some(day(2026, 10, 23)))
            .label("Due date")
            .show_week_numbers(true)
            .open(true)
            .on_change(Message::Day)
            .on_open_change(Message::Open)
            .width(260),
    ]
    .padding(16)
    .into()
}

fn snapshot_open(theme: &Theme, name: &str) {
    let mut ui =
        simulator::Simulator::with_size(support::settings(), (380.0, 420.0), open_picker());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn date_picker_open_light() {
    snapshot_open(&Theme::light(), "date_picker_open_light");
}

#[test]
fn date_picker_open_dark() {
    snapshot_open(&Theme::dark(), "date_picker_open_dark");
}
