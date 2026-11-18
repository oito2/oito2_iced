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
//! The DateTimeEntry page: text fields that read a typed date, time or
//! both, with calendar and time picker buttons.

use iced::widget::{column, container};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::date::{Date, Time};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::entries::{Part, date_entry, date_time_entry, time_entry};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("date_time_entry.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    today: Date,
    first: Date,
    last: Date,
    date: Option<Date>,
    date_open: bool,
    time: Option<Time>,
    time_open: bool,
    date_time: Option<(Date, Time)>,
    date_time_open: Option<Part>,
    meeting: Option<Date>,
    meeting_open: bool,
    error: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            today: today(),
            first: Date::new(2000, 1, 1).unwrap_or(Date::MIN),
            last: Date::new(2100, 12, 31).unwrap_or(Date::MAX),
            date: None,
            date_open: false,
            time: None,
            time_open: false,
            date_time: None,
            date_time_open: None,
            meeting: None,
            meeting_open: false,
            error: None,
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
    /// The date entry changed.
    Date(Option<Date>),
    /// The calendar of the date entry opened or closed.
    DateOpen(bool),
    /// The time entry changed.
    Time(Option<Time>),
    /// The time popover opened or closed.
    TimeOpen(bool),
    /// The date and time entry changed.
    DateTime(Option<(Date, Time)>),
    /// A popover of the date and time entry opened, or both closed.
    DateTimeOpen(Option<Part>),
    /// The meeting day changed.
    Meeting(Option<Date>),
    /// The calendar of the meeting day opened or closed.
    MeetingOpen(bool),
    /// The meeting day stopped (`Some`) or started (`None`) parsing.
    Error(Option<String>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Date(date) => state.date = date,
        Message::DateOpen(open) => state.date_open = open,
        Message::Time(time) => state.time = time,
        Message::TimeOpen(open) => state.time_open = open,
        Message::DateTime(value) => state.date_time = value,
        Message::DateTimeOpen(part) => state.date_time_open = part,
        Message::Meeting(date) => state.meeting = date,
        Message::MeetingOpen(open) => state.meeting_open = open,
        Message::Error(error) => state.error = error,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let (today, first, last) = (state.today, state.first, state.last);
    let none = || "—".to_owned();

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            column![
                // #region preview
                date_entry(today, state.date)
                    .limits(first, last)
                    .on_change(Message::Date)
                    .open(state.date_open)
                    .on_open_change(Message::DateOpen)
                    .localizations(l10n),
                // #endregion
                doc::caption(format!(
                    "Value: {}",
                    state.date.map_or_else(none, |date| l10n.compact_date(date))
                )),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(260.0)),
    );

    let time = doc::example(
        ctx,
        SOURCE,
        &["time"],
        container(
            column![
                // #region time
                time_entry(state.time)
                    .on_change(Message::Time)
                    .open(state.time_open)
                    .on_open_change(Message::TimeOpen)
                    .localizations(l10n),
                // #endregion
                doc::caption(format!(
                    "Value: {}",
                    state
                        .time
                        .map_or_else(none, |time| l10n.time_of_day(time, false))
                )),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(220.0)),
    );

    let date_time = doc::example(
        ctx,
        SOURCE,
        &["date-time"],
        container(
            column![
                // #region date-time
                date_time_entry(today, state.date_time)
                    .limits(first, last)
                    .use_24_hour(true)
                    .on_change(Message::DateTime)
                    .open(state.date_time_open)
                    .on_open_change(Message::DateTimeOpen)
                    .localizations(l10n)
                    .width(Length::Fill),
                // #endregion
                doc::caption(format!(
                    "Value: {}",
                    state.date_time.map_or_else(none, |(date, time)| format!(
                        "{} {}",
                        l10n.compact_date(date),
                        l10n.time_of_day(time, true)
                    ))
                )),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(456.0)),
    );

    let errors = doc::example(
        ctx,
        SOURCE,
        &["errors"],
        container(
            column![
                // #region errors
                date_entry(today, state.meeting)
                    .limits(first, last)
                    .label("Meeting day")
                    .accept_empty(false)
                    .selectable(|day| (1..=5).contains(&day.weekday()))
                    .on_change(Message::Meeting)
                    .on_error_change(Message::Error)
                    .open(state.meeting_open)
                    .on_open_change(Message::MeetingOpen)
                    .localizations(l10n),
                // #endregion
                doc::caption(format!(
                    "Error: {}",
                    state.error.clone().unwrap_or_else(none)
                )),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(260.0)),
    );

    doc::page(
        "DateTimeEntry",
        "Text fields that read a typed date, time or both, with calendar and time picker \
         buttons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`date_entry` reads the compact date format of the language, mm/dd/yyyy in \
                 English and dd/mm/aaaa in Portuguese, and accepts dates within `limits`. \
                 `on_change` publishes each valid date, or `None` when the field is \
                 emptied; text that does not parse publishes nothing, so the last valid \
                 date stays. A new value from the application replaces the text unless the \
                 text already reads as it.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::entries::{date_entry, time_entry};",
            ),
            doc::prose(
                "The format, the hint, the labels and the error messages come from \
                 `localizations`. An error shows only after the field loses the focus or is \
                 submitted with Enter, so partial input is not flagged while typing. The \
                 calendar button opens a calendar in a popover whose open state lives in \
                 the application; without `on_open_change` the button is hidden.",
            ),
            doc::heading("Examples"),
            doc::subheading("Time"),
            doc::prose(
                "`time_entry` accepts `H:MM` and `HH:MM`, also with `.` or `h` as the \
                 separator, and an AM/PM marker. It shows the time as `h:mm AM` in English, \
                 or `HH:mm` with `use_24_hour(true)` and always in Portuguese. Its clock \
                 button opens hour and minute entries in a popover.",
            ),
            time,
            doc::subheading("Date and time"),
            doc::prose(
                "`date_time_entry` puts a date entry and a time entry side by side and \
                 reports one value; an empty time means midnight, and without a date the \
                 value is `None`. `on_change` waits while either field holds invalid text. \
                 `open` takes the `Part` whose popover shows, or `None`.",
            ),
            date_time,
            doc::subheading("Required date and allowed days"),
            doc::prose(
                "With `accept_empty(false)` an empty field is an error. `selectable` \
                 refuses days, here weekends, both typed and in the calendar. \
                 `on_error_change` publishes the error message when the text stops being \
                 valid, and `None` when it is valid again.",
            ),
            errors,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::entries::date_entry",
                    "oito2_iced::widget::entries::DateEntry",
                    "oito2_iced::widget::entries::time_entry",
                    "oito2_iced::widget::entries::TimeEntry",
                    "oito2_iced::widget::entries::date_time_entry",
                    "oito2_iced::widget::entries::DateTimeEntry",
                    "oito2_iced::widget::entries::Part",
                    "oito2_iced::date::Time",
                ],
            ),
        ],
    )
}
