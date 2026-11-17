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

//! Parsed and segmented entries: typed dates and times, errors after a
//! blur or a submit, the calendar and time popovers, the combined date and
//! time, the segments and their focus moves, the localized formats, and
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
use oito2_iced::date::{Date, Time};
use oito2_iced::density;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::entries::{self, Part};
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
}

impl<M: Model> Harness<M> {
    /// Types `text` one character per update, as a person does, so the
    /// model sees each intermediate value.
    fn type_text(&mut self, text: &str) {
        let mut buffer = [0; 4];

        for character in text.chars() {
            self.send(simulator::typewrite(character.encode_utf8(&mut buffer)));
        }
    }

    fn select_all(&mut self) {
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::COMMAND,
            )),
            simulator::press_key(keyboard::Key::Character("a".into()), None),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }

    /// Replaces the text of the focused field with `text`.
    fn replace_text(&mut self, text: &str) {
        self.select_all();
        self.type_text(text);
    }

    /// Sends an event that changes nothing, like the redraw that follows
    /// every update.
    fn nudge(&mut self) {
        self.send([Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(470.0, 470.0),
        })]);
    }
}

fn day(year: i32, month: u32, day: u32) -> Date {
    Date::new(year, month, day).expect("valid date")
}

fn time(hour: u32, minute: u32) -> Time {
    Time::new(hour, minute).expect("valid time")
}

/// The center of the picker button at the end of the field `input`.
fn picker_button(input: Rectangle) -> Point {
    let height = density::control_height();

    Point::new(input.x + input.width - height / 2.0, input.center_y())
}

/// The center of the cell of `date` in a calendar grid whose body has
/// `body` bounds and whose weeks start on Sunday.
fn cell(body: Rectangle, date: Date) -> Point {
    let side = density::control_height() + 2.0;
    let index = date.first_of_month().weekday() + date.day() - 1;
    let (row, column) = (index / 7, index % 7);

    Point::new(
        body.x + side * (column as f32 + 0.5),
        body.y + side * 0.75 + side * (row as f32 + 0.5),
    )
}

/// A point away from every field.
const OUTSIDE: Point = Point::new(470.0, 470.0);

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Date(Option<Date>),
    Time(Option<Time>),
    DateTime(Option<(Date, Time)>),
    Error(Option<String>),
    Open(bool),
    OpenPart(Option<Part>),
    Segments(Vec<String>),
}

/// A date entry of 2020 to 2030 whose calendar marks June 10, 2025.
struct DateModel {
    value: Option<Date>,
    errors: Vec<Option<String>>,
    open: bool,
    enabled: bool,
    localizations: Localizations,
}

impl DateModel {
    fn new() -> Self {
        Self {
            value: None,
            errors: Vec::new(),
            open: false,
            enabled: true,
            localizations: Localizations::en(),
        }
    }

    fn values(messages: &[Message]) -> Vec<Option<Date>> {
        messages
            .iter()
            .filter_map(|message| match message {
                Message::Date(date) => Some(*date),
                _ => None,
            })
            .collect()
    }
}

impl Model for DateModel {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut entry = entries::date_entry(day(2025, 6, 10), self.value)
            .limits(day(2020, 1, 1), day(2030, 12, 31))
            .open(self.open)
            .localizations(&self.localizations)
            .id("date")
            .width(300);

        if self.enabled {
            entry = entry
                .on_change(Message::Date)
                .on_error_change(Message::Error)
                .on_open_change(Message::Open);
        }

        oito2_iced::focus::navigation(column![entry].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Date(date) => self.value = date,
            Message::Error(error) => self.errors.push(error),
            Message::Open(open) => self.open = open,
            _ => {}
        }
    }
}

#[test]
fn a_date_entry_reports_valid_dates_in_the_locale_format() {
    let mut ui = Harness::new(DateModel::new());
    assert!(ui.shows("mm/dd/yyyy"));
    assert!(ui.shows("Enter Date"));

    let input = ui.bounds("date");
    ui.click_at(input.center());
    ui.type_text("03/15/2026");

    assert_eq!(ui.model.value, Some(day(2026, 3, 15)));
}

#[test]
fn errors_show_after_a_submit_and_hide_on_valid_text() {
    let mut ui = Harness::new(DateModel::new());
    let input = ui.bounds("date");

    ui.click_at(input.center());
    ui.type_text("15/45/2026");
    assert!(!ui.shows("Invalid format."), "no error while typing");

    ui.key(Named::Enter);
    assert!(ui.shows("Invalid format."));

    ui.replace_text("01/01/2040");
    ui.key(Named::Enter);
    assert!(ui.shows("Out of range."));
    assert!(
        DateModel::values(&ui.messages)
            .iter()
            .all(|value| value.is_none_or(|date| date.year() >= 2020)),
        "no invalid date is reported"
    );

    ui.replace_text("01/01/2025");
    assert!(!ui.shows("Out of range."));
    assert_eq!(ui.model.value, Some(day(2025, 1, 1)));
}

#[test]
fn the_error_is_reported_when_the_text_stops_and_starts_parsing() {
    let mut ui = Harness::new(DateModel::new());
    let input = ui.bounds("date");

    ui.click_at(input.center());
    ui.type_text("03/15/");
    assert_eq!(ui.model.errors, [Some("Invalid format.".to_owned())]);

    ui.type_text("2026");
    assert_eq!(ui.model.errors.last(), Some(&None));
    assert_eq!(ui.model.value, Some(day(2026, 3, 15)));
}

#[test]
fn the_error_shows_after_the_field_loses_the_focus() {
    let mut ui = Harness::new(DateModel::new());
    let input = ui.bounds("date");

    ui.click_at(input.center());
    ui.type_text("99/99");
    assert!(!ui.shows("Invalid format."));

    ui.click_at(OUTSIDE);
    ui.nudge();
    assert!(ui.shows("Invalid format."));
}

#[test]
fn an_emptied_field_reports_none() {
    let mut ui = Harness::new(DateModel {
        value: Some(day(2025, 1, 1)),
        ..DateModel::new()
    });
    assert!(ui.shows("01/01/2025"));

    let input = ui.bounds("date");
    ui.click_at(input.center());
    ui.select_all();
    ui.key(Named::Backspace);

    assert_eq!(ui.model.value, None);
}

#[test]
fn a_new_value_from_the_app_replaces_the_text() {
    let mut ui = Harness::new(DateModel::new());

    ui.model.value = Some(day(2026, 7, 4));
    ui.send([]);
    assert!(ui.shows("07/04/2026"));
}

#[test]
fn the_calendar_button_picks_a_date() {
    let mut ui = Harness::new(DateModel::new());
    let input = ui.bounds("date");

    ui.click_at(picker_button(input));
    assert!(ui.model.open);

    // The calendar opens on the month of today and takes the focus.
    ui.send([]);
    let body = ui.focused().expect("the grid has the focus");

    ui.click_at(cell(body, day(2025, 6, 15)));
    assert_eq!(DateModel::values(&ui.messages), [Some(day(2025, 6, 15))]);
    assert!(!ui.model.open);
    assert!(ui.shows("06/15/2025"));
}

#[test]
fn the_calendar_opens_focused_and_picks_with_enter() {
    let mut ui = Harness::new(DateModel::new());
    let input = ui.bounds("date");

    ui.click_at(picker_button(input));
    ui.send([]);
    ui.key(Named::ArrowRight);
    ui.key(Named::Enter);

    assert_eq!(DateModel::values(&ui.messages), [Some(day(2025, 6, 11))]);
    assert!(!ui.model.open);
}

#[test]
fn a_pick_replaces_invalid_text_even_with_the_same_date() {
    let mut ui = Harness::new(DateModel {
        value: Some(day(2025, 6, 15)),
        ..DateModel::new()
    });
    let input = ui.bounds("date");

    ui.click_at(input.center());
    ui.replace_text("06/15/20x");
    ui.key(Named::Enter);
    assert!(ui.shows("Invalid format."));

    ui.click_at(picker_button(input));
    ui.send([]);
    let body = ui.focused().expect("the grid has the focus");
    ui.click_at(cell(body, day(2025, 6, 15)));
    ui.nudge();

    assert!(ui.shows("06/15/2025"));
    assert!(!ui.shows("Invalid format."));
    assert_eq!(ui.model.errors.last(), Some(&None));
    assert_eq!(ui.model.value, Some(day(2025, 6, 15)));
}

#[test]
fn a_date_entry_without_a_message_is_disabled() {
    let mut ui = Harness::new(DateModel {
        enabled: false,
        value: Some(day(2025, 1, 1)),
        ..DateModel::new()
    });
    // The disabled input is out of the operations, so it is reached by
    // position: under the label, at the padding.
    ui.click_at(Point::new(100.0, 60.0));
    ui.type_text("1");
    ui.key(Named::Tab);

    assert!(ui.messages.is_empty());
    assert_eq!(ui.focused(), None);
}

#[test]
fn a_date_entry_follows_the_portuguese_format() {
    let mut ui = Harness::new(DateModel {
        localizations: Localizations::pt_br(),
        ..DateModel::new()
    });
    assert!(ui.shows("dd/mm/aaaa"));
    assert!(ui.shows("Inserir data"));

    let input = ui.bounds("date");
    ui.click_at(input.center());
    ui.type_text("15/03/2026");
    assert_eq!(ui.model.value, Some(day(2026, 3, 15)));

    ui.replace_text("15/03");
    ui.key(Named::Enter);
    assert!(ui.shows("Formato inválido."));
}

/// A time entry.
struct TimeModel {
    value: Option<Time>,
    open: bool,
    use_24_hour: bool,
    localizations: Localizations,
}

impl TimeModel {
    fn new() -> Self {
        Self {
            value: Some(time(14, 5)),
            open: false,
            use_24_hour: false,
            localizations: Localizations::en(),
        }
    }
}

impl Model for TimeModel {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let entry = entries::time_entry(self.value)
            .use_24_hour(self.use_24_hour)
            .open(self.open)
            .localizations(&self.localizations)
            .on_change(Message::Time)
            .on_open_change(Message::Open)
            .id("time")
            .width(240);

        oito2_iced::focus::navigation(column![entry].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Time(time) => self.value = time,
            Message::Open(open) => self.open = open,
            _ => {}
        }
    }
}

#[test]
fn a_time_entry_reports_valid_times_and_flags_invalid_ones() {
    let mut ui = Harness::new(TimeModel::new());
    assert!(ui.shows("2:05 PM"));
    assert!(ui.shows("Enter time"));

    let input = ui.bounds("time");
    ui.click_at(input.center());
    ui.replace_text("18:45");
    assert_eq!(ui.model.value, Some(time(18, 45)));

    ui.replace_text("7h30 pm");
    assert_eq!(ui.model.value, Some(time(19, 30)));

    ui.replace_text("25:00");
    ui.key(Named::Enter);
    assert!(ui.shows("Enter a valid time"));
    assert_eq!(ui.model.value, Some(time(19, 30)));
}

#[test]
fn the_time_format_follows_the_24_hour_option_and_the_locale() {
    let mut ui = Harness::new(TimeModel {
        use_24_hour: true,
        ..TimeModel::new()
    });
    assert!(ui.shows("14:05"));

    let mut ui = Harness::new(TimeModel {
        localizations: Localizations::pt_br(),
        ..TimeModel::new()
    });
    assert!(ui.shows("14:05"));
    assert!(ui.shows("Insira o horário"));
}

#[test]
fn the_clock_button_opens_hour_and_minute_entries() {
    let mut ui = Harness::new(TimeModel::new());
    let input = ui.bounds("time");

    ui.click_at(picker_button(input));
    assert!(ui.model.open);
    ui.send([]);

    // The hour entry takes the focus; Up adds an hour.
    ui.key(Named::ArrowUp);
    assert_eq!(ui.model.value, Some(time(15, 5)));

    ui.send([]);
    assert!(ui.shows("3:05 PM"));

    // The first Escape leaves the hour entry, the second closes.
    ui.key(Named::Escape);
    ui.key(Named::Escape);
    assert!(!ui.model.open);
}

/// A date and time entry.
struct DateTimeModel {
    value: Option<(Date, Time)>,
    errors: Vec<Option<String>>,
    open: Option<Part>,
}

impl Model for DateTimeModel {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let entry = entries::date_time_entry(day(2025, 6, 10), self.value)
            .limits(day(2020, 1, 1), day(2030, 12, 31))
            .open(self.open)
            .on_change(Message::DateTime)
            .on_error_change(Message::Error)
            .on_open_change(Message::OpenPart)
            .date_id("date")
            .time_id("time")
            .width(440);

        oito2_iced::focus::navigation(column![entry].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DateTime(value) => self.value = value,
            Message::Error(error) => self.errors.push(error),
            Message::OpenPart(open) => self.open = open,
            _ => {}
        }
    }
}

fn date_time_model() -> DateTimeModel {
    DateTimeModel {
        value: None,
        errors: Vec::new(),
        open: None,
    }
}

#[test]
fn a_date_time_entry_combines_both_fields() {
    let mut ui = Harness::new(date_time_model());
    let date = ui.bounds("date");
    let time_input = ui.bounds("time");

    ui.click_at(date.center());
    ui.type_text("06/01/2026");
    assert_eq!(ui.model.value, Some((day(2026, 6, 1), Time::MIDNIGHT)));
    assert!(!ui.shows("12:00 AM"), "an empty time stays empty");

    ui.click_at(time_input.center());
    ui.type_text("08:30");
    assert_eq!(ui.model.value, Some((day(2026, 6, 1), time(8, 30))));
}

#[test]
fn a_date_time_entry_stays_quiet_while_a_field_is_invalid() {
    let mut ui = Harness::new(date_time_model());
    let date = ui.bounds("date");
    let time_input = ui.bounds("time");

    ui.click_at(date.center());
    ui.type_text("06/01/2026");
    ui.click_at(time_input.center());
    ui.type_text("99:99");
    ui.key(Named::Enter);
    assert!(ui.model.errors.last().is_some_and(Option::is_some));
    assert!(ui.shows("Enter a valid time"));

    let reported = ui.model.value;
    ui.click_at(date.center());
    ui.replace_text("07/01/2026");
    assert_eq!(
        ui.model.value, reported,
        "no report while the time is invalid"
    );

    ui.click_at(time_input.center());
    ui.replace_text("08:30");
    ui.key(Named::Enter);
    assert_eq!(ui.model.errors.last(), Some(&None));
    assert_eq!(ui.model.value, Some((day(2026, 7, 1), time(8, 30))));
}

#[test]
fn the_date_error_shows_when_the_focus_moves_to_the_time_field() {
    let mut ui = Harness::new(date_time_model());
    let date = ui.bounds("date");

    ui.click_at(date.center());
    ui.type_text("13/45");
    ui.key(Named::Tab);
    ui.nudge();

    assert!(ui.shows("Invalid format."));
}

/// An IP address entry.
struct SegmentsModel {
    values: Vec<String>,
    delimiter: &'static str,
    numeric: bool,
    enabled: bool,
}

impl SegmentsModel {
    fn new() -> Self {
        Self {
            values: vec![String::new(); 3],
            delimiter: ".",
            numeric: true,
            enabled: true,
        }
    }
}

impl Model for SegmentsModel {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let ids = ["s0", "s1", "s2"];
        let mut entry = entries::segmented_entry(
            ids.map(|id| entries::segment(3).hint("000").numeric(self.numeric).id(id)),
            &self.values,
        )
        .delimiter(self.delimiter);

        if self.enabled {
            entry = entry.on_change(Message::Segments);
        }

        oito2_iced::focus::navigation(column![entry].padding(20)).into()
    }

    fn update(&mut self, message: Message) {
        if let Message::Segments(values) = message {
            self.values = values;
        }
    }
}

#[test]
fn a_full_segment_moves_to_the_next_one() {
    let mut ui = Harness::new(SegmentsModel::new());
    let first = ui.bounds("s0");

    ui.click_at(first.center());
    ui.type_text("192");

    assert_eq!(ui.model.values, ["192", "", ""]);
    assert_eq!(ui.focused(), Some(ui.bounds("s1")));
}

#[test]
fn segments_keep_only_digits_up_to_their_length() {
    let mut ui = Harness::new(SegmentsModel::new());
    let first = ui.bounds("s0");

    ui.click_at(first.center());
    ui.type_text("1a2b3c4");

    assert_eq!(ui.model.values, ["123", "4", ""]);
}

#[test]
fn backspace_on_an_empty_segment_goes_back() {
    let mut ui = Harness::new(SegmentsModel::new());
    let second = ui.bounds("s1");

    ui.click_at(second.center());
    ui.key(Named::Backspace);

    assert_eq!(ui.focused(), Some(ui.bounds("s0")));
}

#[test]
fn the_arrows_cross_segments_at_their_edges() {
    let mut ui = Harness::new(SegmentsModel {
        values: vec!["12".to_owned(), "34".to_owned(), String::new()],
        ..SegmentsModel::new()
    });
    let first = ui.bounds("s0");

    ui.click_at(first.center());
    ui.send([simulator::press_key(Named::End, None)]);
    ui.key(Named::ArrowRight);
    assert_eq!(ui.focused(), Some(ui.bounds("s1")));

    ui.key(Named::ArrowRight);
    assert_eq!(ui.focused(), Some(ui.bounds("s1")), "inside the text");

    ui.key(Named::ArrowLeft);
    ui.key(Named::ArrowLeft);
    assert_eq!(ui.focused(), Some(ui.bounds("s0")));
}

#[test]
fn a_spaced_delimiter_advances_on_its_visible_character() {
    let mut ui = Harness::new(SegmentsModel {
        delimiter: " - ",
        numeric: false,
        ..SegmentsModel::new()
    });
    let first = ui.bounds("s0");

    ui.click_at(first.center());
    ui.type_text("ab-");

    assert_eq!(ui.model.values, ["ab", "", ""]);
    assert_eq!(ui.focused(), Some(ui.bounds("s1")));
}

#[test]
fn a_segmented_entry_without_a_message_is_disabled() {
    let mut ui = Harness::new(SegmentsModel {
        enabled: false,
        ..SegmentsModel::new()
    });
    ui.click_at(Point::new(40.0, 37.0));
    ui.type_text("1");
    ui.key(Named::Tab);

    assert!(ui.messages.is_empty());
    assert_eq!(ui.focused(), None);
}

fn catalog() -> Element<'static, Message> {
    let pt = Localizations::pt_br();
    let address = ["192", "168", "0", "1"].map(String::from);

    column![
        row![
            entries::date_entry(day(2026, 10, 20), Some(day(2026, 10, 23)))
                .on_change(Message::Date)
                .on_open_change(Message::Open)
                .width(240),
            entries::time_entry(Some(time(14, 5)))
                .on_change(Message::Time)
                .on_open_change(Message::Open)
                .width(200),
        ]
        .spacing(16),
        entries::date_time_entry(day(2026, 10, 20), Some((day(2026, 10, 23), time(9, 30))))
            .localizations(&pt)
            .on_change(Message::DateTime)
            .on_open_change(Message::OpenPart)
            .width(456),
        row![
            entries::date_entry::<Message>(day(2026, 10, 20), Some(day(2026, 10, 23)))
                .label("Disabled")
                .width(240),
            entries::time_entry(None)
                .use_24_hour(true)
                .on_change(Message::Time)
                .width(200),
        ]
        .spacing(16),
        entries::segmented_entry((0..4).map(|_| entries::segment(3).hint("000")), &address,)
            .on_change(Message::Segments),
        entries::segmented_entry(
            (0..3).map(|_| entries::segment(4).numeric(false).hint("XXXX")),
            &["AB12".to_owned()],
        )
        .delimiter(" - ")
        .on_change(Message::Segments),
    ]
    .spacing(16)
    .padding(16)
    .into()
}

/// Shows the catalog with an invalid date submitted in the first field,
/// which then loses the focus.
fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (520.0, 520.0), catalog());
    let _ = ui.click("10/23/2026");
    let _ = ui.tap_key(Named::End);
    let _ = ui.typewrite("x");
    let _ = ui.tap_key(Named::Enter);
    ui.point_at(Point::new(500.0, 500.0));
    let _ = ui.simulate(simulator::click());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn entries_light() {
    snapshot(&Theme::light(), "entries_light");
}

#[test]
fn entries_dark() {
    snapshot(&Theme::dark(), "entries_dark");
}

#[test]
fn entries_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "entries_light_high_contrast");
}

fn time_picker() -> Element<'static, Message> {
    column![
        entries::time_entry(Some(time(14, 5)))
            .open(true)
            .on_change(Message::Time)
            .on_open_change(Message::Open)
            .width(240),
    ]
    .padding(16)
    .into()
}

#[test]
fn entries_time_picker_light() {
    let mut ui =
        simulator::Simulator::with_size(support::settings(), (480.0, 220.0), time_picker());
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "entries_time_picker_light");
}

#[test]
fn entries_time_picker_compact() {
    density::set(density::Density::Touch);
    let mut ui =
        simulator::Simulator::with_size(support::settings(), (480.0, 220.0), time_picker());
    let snapshot = ui.snapshot(&Theme::dark()).expect("snapshot");
    density::set(density::Density::Pointer);

    support::golden!(&snapshot, "entries_time_picker_touch_dark");
}
