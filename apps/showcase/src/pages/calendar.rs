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
//! The Calendar page: calendars and date pickers for dates and ranges, and
//! a date entry.

use iced::widget::{column, container, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::date::Date;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::calendar::{DateRange, calendar, date_picker};
use oito2_iced::widget::entries::date_entry;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("calendar.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    today: Date,
    first: Date,
    last: Date,
    day: Option<Date>,
    date: Option<Date>,
    range: Option<DateRange>,
    picked: Option<Date>,
    picked_open: bool,
    trip: Option<DateRange>,
    trip_open: bool,
    typed: Option<Date>,
    typed_open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            today: today(),
            first: Date::new(2020, 1, 1).unwrap_or(Date::MIN),
            last: Date::new(2030, 12, 31).unwrap_or(Date::MAX),
            day: None,
            date: None,
            range: None,
            picked: None,
            picked_open: false,
            trip: None,
            trip_open: false,
            typed: None,
            typed_open: false,
        }
    }
}

/// Today's date in UTC, or 2026-01-01 when the clock is before 1970.
fn today() -> Date {
    let days = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() / 86_400)
        .unwrap_or_default();

    i64::try_from(days)
        .ok()
        .filter(|days| *days > 0)
        .and_then(Date::from_unix_days)
        .or_else(|| Date::new(2026, 1, 1))
        .unwrap_or(Date::MIN)
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A day of the preview calendar was picked.
    Day(Date),
    /// A weekday of the calendar of allowed days was picked.
    Date(Date),
    /// A range of the range calendar was picked.
    Range(DateRange),
    /// The due date picker changed.
    Picked(Date),
    /// The due date picker opened or closed.
    PickedOpen(bool),
    /// The trip picker changed.
    Trip(DateRange),
    /// The trip picker opened or closed.
    TripOpen(bool),
    /// The date entry changed.
    Typed(Option<Date>),
    /// The calendar of the date entry opened or closed.
    TypedOpen(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Day(day) => state.day = Some(day),
        Message::Date(day) => state.date = Some(day),
        Message::Range(range) => state.range = Some(range),
        Message::Picked(day) => state.picked = Some(day),
        Message::PickedOpen(open) => state.picked_open = open,
        Message::Trip(range) => state.trip = Some(range),
        Message::TripOpen(open) => state.trip_open = open,
        Message::Typed(day) => state.typed = day,
        Message::TypedOpen(open) => state.typed_open = open,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let (today, first, last) = (state.today, state.first, state.last);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        calendar(today)
            .limits(first, last)
            .selected(state.day)
            .localizations(l10n)
            .direction(ctx.direction)
            .on_select(Message::Day),
        // #endregion
    );

    let date = state
        .date
        .map_or_else(|| "No date".to_owned(), |day| l10n.compact_date(day));

    let allowed = doc::example(
        ctx,
        SOURCE,
        &["allowed-days"],
        column![
            // #region allowed-days
            calendar(today)
                .limits(first, last)
                .selected(state.date)
                .selectable(|day| (1..=5).contains(&day.weekday()))
                .localizations(l10n)
                .on_select(Message::Date),
            // #endregion
            doc::caption(date),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let range = doc::example(
        ctx,
        SOURCE,
        &["range"],
        // #region range
        calendar(today)
            .limits(first, last)
            .selected_range(state.range)
            .show_week_numbers(true)
            .localizations(l10n)
            .on_range_select(Message::Range),
        // #endregion
    );

    let pickers = doc::example(
        ctx,
        SOURCE,
        &["picker"],
        row![
            // #region picker
            date_picker(today)
                .limits(first, last)
                .value(state.picked)
                .label("Due date")
                .on_change(Message::Picked)
                .open(state.picked_open)
                .on_open_change(Message::PickedOpen)
                .localizations(l10n)
                .width(Length::Fixed(220.0)),
            date_picker(today)
                .limits(first, last)
                .range(state.trip)
                .label("Trip")
                .on_range_change(Message::Trip)
                .open(state.trip_open)
                .on_open_change(Message::TripOpen)
                .localizations(l10n)
                .width(Length::Fixed(280.0)),
            // #endregion
        ]
        .spacing(spacing::MD)
        .wrap()
        .vertical_spacing(spacing::MD),
    );

    let entry = doc::example(
        ctx,
        SOURCE,
        &["entry"],
        container(
            // #region entry
            date_entry(today, state.typed)
                .limits(first, last)
                .on_change(Message::Typed)
                .open(state.typed_open)
                .on_open_change(Message::TypedOpen)
                .localizations(l10n),
            // #endregion
        )
        .width(Length::Fixed(260.0)),
    );

    doc::page(
        "Calendar",
        "A month grid that picks a date or a range of dates, inline or from a field that \
         opens it in a popover.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`calendar` is controlled: give it the `selected` day and update it in \
                 `on_select`. It takes today's date from the application, since Iced has no \
                 time zone support, and outlines that day. Days outside `limits` cannot be \
                 picked, and the heading changes the month within those limits. It opens on \
                 the month of the selection, or of today; `initial_month` chooses another.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::date::Date;\n\
                 use oito2_iced::widget::calendar::{DateRange, calendar, date_picker};",
            ),
            doc::prose(
                "`localizations` sets the month and weekday names and the first day of the \
                 week, and `direction` mirrors the grid, the month buttons and the arrow \
                 keys for right-to-left languages.",
            ),
            doc::heading("Examples"),
            doc::subheading("Allowed days"),
            doc::prose(
                "`selectable` decides which days can be picked; here weekends are refused \
                 and show dimmed.",
            ),
            allowed,
            doc::subheading("Range and week numbers"),
            doc::prose(
                "Setting `on_range_select` turns on range mode, with the range in \
                 `selected_range`. A range takes two picks, and the second may come before \
                 the first; between them the band previews the range up to the hovered or \
                 focused day. `show_week_numbers` adds a column of ISO 8601 week numbers.",
            ),
            range,
            doc::subheading("Date picker"),
            doc::prose(
                "`date_picker` is a field-like button that shows the date and opens the \
                 calendar in a popover; a pick closes it. The open state lives in the \
                 application, through `open` and `on_open_change`. Like the calendar, it \
                 picks a range when `on_range_change` is set.",
            ),
            pickers,
            doc::subheading("Typed date"),
            doc::prose(
                "To let people type the date as well, use `date_entry`: a text field with a \
                 button that opens the same calendar. The DateTimeEntry page shows it in \
                 detail.",
            ),
            entry,
            doc::heading("Keyboard"),
            doc::prose("The grid takes the focus as a whole and moves a focused day:"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Left` / `Right`",
                        "Moves to the previous or next day (mirrored right to left).",
                    ],
                    vec![
                        "`Up` / `Down`",
                        "Moves to the same day a week before or after.",
                    ],
                    vec!["`Home` / `End`", "Moves to the start or end of the week."],
                    vec![
                        "`Page Up` / `Page Down`",
                        "Moves to the same day a month before or after.",
                    ],
                    vec![
                        "`Shift` + `Page Up` / `Page Down`",
                        "Moves to the same day a year before or after.",
                    ],
                    vec!["`Enter` / `Space`", "Picks the focused day."],
                    vec![
                        "`Enter` / `Space` / `Alt` + `Down` on a date picker",
                        "Opens the calendar.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::calendar::calendar",
                    "oito2_iced::widget::calendar::Calendar",
                    "oito2_iced::widget::calendar::date_picker",
                    "oito2_iced::widget::calendar::DatePicker",
                    "oito2_iced::widget::calendar::DateRange",
                    "oito2_iced::date::Date",
                ],
            ),
        ],
    )
}
