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

//! Locale-aware formatting of byte sizes, compact numbers and relative times.

use std::time::SystemTime;

use crate::l10n::{Localizations, TimeUnit};

/// The units of [`Format::bytes`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ByteUnits {
    /// Powers of 1000: B, kB, MB, GB, TB, PB.
    #[default]
    Si,
    /// Powers of 1024: B, KiB, MiB, GiB, TiB, PiB.
    Iec,
}

/// The number conventions of a locale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberLocale {
    English,
    Portuguese,
}

impl NumberLocale {
    /// The digit group separator.
    fn group(self) -> char {
        match self {
            Self::English => ',',
            Self::Portuguese => '.',
        }
    }

    /// The decimal separator.
    fn decimal(self) -> char {
        match self {
            Self::English => '.',
            Self::Portuguese => ',',
        }
    }
}

const SI_UNITS: [&str; 6] = ["B", "kB", "MB", "GB", "TB", "PB"];
const IEC_UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];

/// Locale-aware formatting of byte sizes, compact numbers and relative
/// times, such as "1.5 MB", "1.23K" and "3 minutes ago".
///
/// Numbers follow the locale (English or Brazilian Portuguese); relative
/// times use the [`Localizations`] of the same locale.
#[derive(Debug, Clone, Copy)]
pub struct Format {
    locale: NumberLocale,
    strings: Localizations,
}

impl Default for Format {
    fn default() -> Self {
        Self::new("en")
    }
}

impl Format {
    /// Creates a formatter for a language tag such as `en`, `pt-BR` or
    /// `pt_BR`; unknown locales fall back to English.
    #[must_use]
    pub fn new(tag: &str) -> Self {
        let strings = Localizations::for_locale(tag);
        let locale = if strings.locale_name == "pt_BR" {
            NumberLocale::Portuguese
        } else {
            NumberLocale::English
        };

        Self { locale, strings }
    }

    /// The strings used for relative times.
    #[must_use]
    pub fn strings(&self) -> &Localizations {
        &self.strings
    }

    /// Formats `value` rounded half away from zero to at most
    /// `max_fraction_digits` decimals, without trailing zeros and with the
    /// locale separators.
    fn decimal(&self, value: f64, max_fraction_digits: usize) -> String {
        let negative = value < 0.0;
        let factor = 10f64.powi(i32::try_from(max_fraction_digits).unwrap_or(0));
        let rounded = (value.abs() * factor).round() / factor;
        let text = format!("{rounded:.max_fraction_digits$}");
        let (integer, fraction) = text.split_once('.').unwrap_or((&text, ""));
        let fraction = fraction.trim_end_matches('0');

        let mut out = String::new();

        if negative && rounded != 0.0 {
            out.push('-');
        }

        let digits = integer.len();

        for (index, digit) in integer.chars().enumerate() {
            if index > 0 && (digits - index) % 3 == 0 {
                out.push(self.locale.group());
            }

            out.push(digit);
        }

        if !fraction.is_empty() {
            out.push(self.locale.decimal());
            out.push_str(fraction);
        }

        out
    }

    /// Formats a byte count in `units`, such as `1536` as "1.5 kB" (or
    /// "1,5 kB" in Portuguese), with at most `fraction_digits` decimals.
    ///
    /// Picks the largest unit, up to petabytes, that keeps the value at
    /// least 1; counts below one kilobyte keep no decimals. Negative counts
    /// keep their sign.
    #[must_use]
    pub fn bytes(&self, bytes: i64, units: ByteUnits, fraction_digits: usize) -> String {
        let (base, names) = match units {
            ByteUnits::Si => (1000.0, &SI_UNITS),
            ByteUnits::Iec => (1024.0, &IEC_UNITS),
        };
        let sign = if bytes < 0 { "-" } else { "" };
        let mut value = bytes.unsigned_abs() as f64;
        let mut unit = 0;

        while value >= base && unit < names.len() - 1 {
            value /= base;
            unit += 1;
        }

        // Moves to the next unit when rounding would show the base itself.
        let digits = if unit == 0 { 0 } else { fraction_digits };
        let factor = 10f64.powi(i32::try_from(digits).unwrap_or(0));

        if unit > 0 && unit < names.len() - 1 && (value * factor).round() / factor >= base {
            value /= base;
            unit += 1;
        }

        let digits = if unit == 0 { 0 } else { fraction_digits };

        format!("{sign}{} {}", self.decimal(value, digits), names[unit])
    }

    /// Formats `value` in compact form with three significant digits, such
    /// as `1234` as "1.23K" (or "1,23 mil" in Portuguese).
    #[must_use]
    pub fn compact_number(&self, value: f64) -> String {
        const SUFFIXES_EN: [&str; 5] = ["", "K", "M", "B", "T"];
        const SUFFIXES_PT: [&str; 5] = ["", "\u{a0}mil", "\u{a0}mi", "\u{a0}bi", "\u{a0}tri"];

        let suffixes = match self.locale {
            NumberLocale::English => &SUFFIXES_EN,
            NumberLocale::Portuguese => &SUFFIXES_PT,
        };
        let magnitude = value.abs();
        let mut scale = 0;
        let mut scaled = magnitude;

        while scaled >= 1000.0 && scale < suffixes.len() - 1 {
            scaled /= 1000.0;
            scale += 1;
        }

        let mut text = self.significant(scaled);

        // Three significant digits can round up to 1000: move to the next unit.
        if scale < suffixes.len() - 1 && text_value(scaled) >= 1000.0 {
            scaled /= 1000.0;
            scale += 1;
            text = self.significant(scaled);
        }

        let sign = if value < 0.0 && text != "0" { "-" } else { "" };

        format!("{sign}{text}{}", suffixes[scale])
    }

    /// Formats a non-negative `value` with three significant digits.
    fn significant(&self, value: f64) -> String {
        self.decimal(value, fraction_digits_for(value))
    }

    /// Describes `time` relative to `now`, such as "3 minutes ago" or
    /// "in 2 days".
    ///
    /// See [`Format::relative_seconds`] for the rounding rules.
    #[must_use]
    pub fn relative_time(&self, time: SystemTime, now: SystemTime) -> String {
        let offset = match time.duration_since(now) {
            Ok(ahead) => i64::try_from(ahead.as_secs()).unwrap_or(i64::MAX),
            Err(behind) => -i64::try_from(behind.duration().as_secs()).unwrap_or(i64::MAX),
        };

        self.relative_seconds(offset)
    }

    /// Describes a time `offset_seconds` away from now, negative for the
    /// past and positive for the future.
    ///
    /// Offsets within 45 seconds read as the "just now" string. Larger
    /// offsets are rounded to minutes below 45 minutes, hours below 22
    /// hours, days below six and a half days, weeks below 30 days, months
    /// (of 30 days) below 365 days and years beyond.
    #[must_use]
    pub fn relative_seconds(&self, offset_seconds: i64) -> String {
        const DAY: i64 = 86_400;

        let future = offset_seconds >= 0;
        let seconds = offset_seconds.unsigned_abs().min(i64::MAX as u64) as i64;

        if seconds < 45 {
            return self.strings.just_now.to_string();
        }

        let round = |divisor: i64| (seconds as f64 / divisor as f64).round() as i64;
        let (count, unit) = if seconds < 45 * 60 {
            (round(60).max(1), TimeUnit::Minute)
        } else if seconds < 22 * 3600 {
            (round(3600), TimeUnit::Hour)
        } else if seconds < 13 * 43_200 {
            (round(DAY), TimeUnit::Day)
        } else if seconds < 30 * DAY {
            (round(7 * DAY), TimeUnit::Week)
        } else if seconds < 365 * DAY {
            (round(30 * DAY).max(1), TimeUnit::Month)
        } else {
            (round(365 * DAY), TimeUnit::Year)
        };

        if future {
            self.strings.time_from_now(count, unit)
        } else {
            self.strings.time_ago(count, unit)
        }
    }
}

/// The number of decimals that leave `value` with three significant digits.
fn fraction_digits_for(value: f64) -> usize {
    if value >= 100.0 {
        0
    } else if value >= 10.0 {
        1
    } else {
        2
    }
}

/// Returns `value` rounded to three significant digits.
fn text_value(value: f64) -> f64 {
    let factor = 10f64.powi(i32::try_from(fraction_digits_for(value)).unwrap_or(0));

    (value * factor).round() / factor
}
