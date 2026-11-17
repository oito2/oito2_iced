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

//! Calendar arithmetic of `date::Date` and `date::Time`.

use oito2_iced::date::{Date, Time, is_leap_year};

fn date(year: i32, month: u32, day: u32) -> Date {
    Date::new(year, month, day).expect("valid date")
}

#[test]
fn new_rejects_days_outside_the_month() {
    assert!(Date::new(2026, 2, 29).is_none());
    assert!(Date::new(2024, 2, 29).is_some());
    assert!(Date::new(1900, 2, 29).is_none());
    assert!(Date::new(2000, 2, 29).is_some());
    assert!(Date::new(2026, 4, 31).is_none());
    assert!(Date::new(2026, 13, 1).is_none());
    assert!(Date::new(2026, 0, 1).is_none());
    assert!(Date::new(0, 1, 1).is_none());
    assert!(Date::new(10_000, 1, 1).is_none());
    assert!(is_leap_year(2000) && !is_leap_year(2100));
}

#[test]
fn unix_days_round_trip() {
    assert_eq!(date(1970, 1, 1).to_unix_days(), 0);
    assert_eq!(date(2000, 3, 1).to_unix_days(), 11_017);
    assert_eq!(date(1969, 12, 31).to_unix_days(), -1);

    let mut day = Date::MIN;
    while day <= Date::MAX {
        assert_eq!(Date::from_unix_days(day.to_unix_days()), Some(day), "{day}");
        match day.add_months(1) {
            Some(next) => day = next,
            None => break,
        }
    }

    let mut day = date(1990, 1, 1);
    while day < date(2101, 1, 1) {
        let next = day.add_days(1).expect("in range");
        assert_eq!(next.to_unix_days(), day.to_unix_days() + 1, "{day}");
        assert_eq!(Date::from_unix_days(next.to_unix_days()), Some(next));
        day = next;
    }

    assert_eq!(Date::from_unix_days(Date::MAX.to_unix_days() + 1), None);
    assert_eq!(Date::from_unix_days(Date::MIN.to_unix_days() - 1), None);
}

#[test]
fn weekdays_start_on_sunday() {
    assert_eq!(date(1970, 1, 1).weekday(), 4);
    assert_eq!(date(2026, 11, 15).weekday(), 0);
    assert_eq!(date(2026, 11, 17).weekday(), 2);
    assert_eq!(date(2026, 11, 21).weekday(), 6);
    assert_eq!(date(1, 1, 1).weekday(), 1);
}

#[test]
fn add_months_clamps_to_the_month_end() {
    assert_eq!(date(2026, 1, 31).add_months(1), Some(date(2026, 2, 28)));
    assert_eq!(date(2024, 1, 31).add_months(1), Some(date(2024, 2, 29)));
    assert_eq!(date(2026, 3, 15).add_months(-3), Some(date(2025, 12, 15)));
    assert_eq!(date(2026, 11, 17).add_months(12), Some(date(2027, 11, 17)));
    assert_eq!(Date::MAX.add_months(1), None);
    assert_eq!(Date::MIN.add_months(-1), None);
}

#[test]
fn iso_weeks() {
    assert_eq!(date(2026, 1, 1).iso_week(), 1);
    assert_eq!(date(2026, 11, 17).iso_week(), 47);
    assert_eq!(date(2021, 1, 3).iso_week(), 53);
    assert_eq!(date(2024, 12, 30).iso_week(), 1);
    assert_eq!(date(2020, 12, 31).iso_week(), 53);
}

#[test]
fn display_and_order() {
    assert_eq!(date(26, 1, 5).to_string(), "0026-01-05");
    assert!(date(2026, 1, 31) < date(2026, 2, 1));
    assert!(date(2025, 12, 31) < date(2026, 1, 1));

    let time = Time::new(9, 5).expect("valid time");
    assert_eq!(time.to_string(), "09:05");
    assert!(Time::new(24, 0).is_none() && Time::new(0, 60).is_none());
    assert!(Time::MIDNIGHT < time);
}
