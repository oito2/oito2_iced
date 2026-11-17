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

//! Calendar dates and times of day for the date and time widgets.
//!
//! [`Date`] is a day of the proleptic Gregorian calendar, without time zone;
//! [`Time`] is an hour and minute of the day. Both are plain values that
//! order chronologically, so applications convert them from and to the date
//! library they use. Formatting and parsing for a language live in
//! [`Localizations`](crate::l10n::Localizations).

use std::fmt;

/// A day of the proleptic Gregorian calendar, without time zone.
///
/// Dates order chronologically. [`Date::new`] accepts years from 1 to 9999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// The first supported date, 0001-01-01.
    pub const MIN: Self = Self {
        year: 1,
        month: 1,
        day: 1,
    };

    /// The last supported date, 9999-12-31.
    pub const MAX: Self = Self {
        year: 9999,
        month: 12,
        day: 31,
    };

    /// The date `year`-`month`-`day`, or `None` when the year is outside
    /// 1..=9999, the month outside 1..=12 or the day outside the month.
    #[must_use]
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        if !(1..=9999).contains(&year) || !(1..=12).contains(&month) {
            return None;
        }

        let month = u8::try_from(month).ok()?;

        if day < 1 || day > u32::from(days_in_month(year, month)) {
            return None;
        }

        Some(Self {
            year,
            month,
            day: u8::try_from(day).ok()?,
        })
    }

    /// The year.
    #[must_use]
    pub fn year(self) -> i32 {
        self.year
    }

    /// The month, from 1 (January) to 12 (December).
    #[must_use]
    pub fn month(self) -> u32 {
        u32::from(self.month)
    }

    /// The day of the month, from 1.
    #[must_use]
    pub fn day(self) -> u32 {
        u32::from(self.day)
    }

    /// The day of the week, from 0 (Sunday) to 6 (Saturday).
    #[must_use]
    pub fn weekday(self) -> u32 {
        // 1970-01-01 was a Thursday (4).
        let days = self.to_unix_days();
        u32::try_from((days + 4).rem_euclid(7)).unwrap_or(0)
    }

    /// The number of days in the month of this date.
    #[must_use]
    pub fn days_in_month(self) -> u32 {
        u32::from(days_in_month(self.year, self.month))
    }

    /// The first day of the month of this date.
    #[must_use]
    pub fn first_of_month(self) -> Self {
        Self { day: 1, ..self }
    }

    /// The number of days from 1970-01-01 to this date (negative before).
    #[must_use]
    pub fn to_unix_days(self) -> i64 {
        let year = i64::from(self.year) - i64::from(self.month <= 2);
        let era = year.div_euclid(400);
        let year_of_era = year - era * 400;
        let month = i64::from(self.month);
        let shifted_month = if month > 2 { month - 3 } else { month + 9 };
        let day_of_year = (153 * shifted_month + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;

        era * 146_097 + day_of_era - 719_468
    }

    /// The date `days` days after 1970-01-01 (before it when negative), or
    /// `None` outside [`Date::MIN`]..=[`Date::MAX`].
    #[must_use]
    pub fn from_unix_days(days: i64) -> Option<Self> {
        let days = days + 719_468;
        let era = days.div_euclid(146_097);
        let day_of_era = days - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let shifted_month = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
        let month = if shifted_month < 10 {
            shifted_month + 3
        } else {
            shifted_month - 9
        };
        let year = year_of_era + era * 400 + i64::from(month <= 2);

        Self::new(
            i32::try_from(year).ok()?,
            u32::try_from(month).ok()?,
            u32::try_from(day).ok()?,
        )
    }

    /// The date `days` days later (earlier when negative), or `None` outside
    /// [`Date::MIN`]..=[`Date::MAX`].
    #[must_use]
    pub fn add_days(self, days: i64) -> Option<Self> {
        Self::from_unix_days(self.to_unix_days().checked_add(days)?)
    }

    /// The same day `months` months later (earlier when negative), moved to
    /// the last day of the target month when it is shorter, or `None`
    /// outside [`Date::MIN`]..=[`Date::MAX`].
    #[must_use]
    pub fn add_months(self, months: i32) -> Option<Self> {
        let index = i64::from(self.year) * 12 + i64::from(self.month) - 1 + i64::from(months);
        let year = i32::try_from(index.div_euclid(12)).ok()?;
        let month = u32::try_from(index.rem_euclid(12) + 1).ok()?;
        let first = Self::new(year, month, 1)?;
        let day = self.day().min(first.days_in_month());

        Self::new(year, month, day)
    }

    /// The ISO 8601 week number of this date, from 1 to 53: weeks start on
    /// Monday and week 1 holds the year's first Thursday.
    #[must_use]
    pub fn iso_week(self) -> u32 {
        let iso_weekday = (self.weekday() + 6) % 7; // Monday = 0
        let thursday = self.add_days(3 - i64::from(iso_weekday)).unwrap_or(self);
        let first = Self {
            year: thursday.year,
            month: 1,
            day: 1,
        };
        let ordinal = thursday.to_unix_days() - first.to_unix_days();

        u32::try_from(ordinal / 7 + 1).unwrap_or(1)
    }
}

impl fmt::Display for Date {
    /// Writes the date as ISO 8601, `yyyy-mm-dd`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

/// An hour and minute of the day, without time zone.
///
/// Times order chronologically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    hour: u8,
    minute: u8,
}

impl Time {
    /// Midnight, 00:00.
    pub const MIDNIGHT: Self = Self { hour: 0, minute: 0 };

    /// The time `hour`:`minute`, or `None` when the hour is outside 0..=23
    /// or the minute outside 0..=59.
    #[must_use]
    pub fn new(hour: u32, minute: u32) -> Option<Self> {
        if hour > 23 || minute > 59 {
            return None;
        }

        Some(Self {
            hour: u8::try_from(hour).ok()?,
            minute: u8::try_from(minute).ok()?,
        })
    }

    /// The hour, from 0 to 23.
    #[must_use]
    pub fn hour(self) -> u32 {
        u32::from(self.hour)
    }

    /// The minute, from 0 to 59.
    #[must_use]
    pub fn minute(self) -> u32 {
        u32::from(self.minute)
    }
}

impl fmt::Display for Time {
    /// Writes the time as `hh:mm`, on the 24-hour clock.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

/// Whether `year` is a leap year of the Gregorian calendar.
#[must_use]
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// The number of days in `month` (1 to 12) of `year`.
fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
