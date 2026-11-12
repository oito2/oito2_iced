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

//! Checks the byte, compact number and relative time formats against
//! `format.json`, the parity fixture exported by `oito2_ui`.

use std::time::{Duration, UNIX_EPOCH};

use oito2_iced::format::{ByteUnits, Format};
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/format.json");
    serde_json::from_str(text).expect("format.json is valid JSON")
}

fn cases<'a>(fixture: &'a Value, locale: &str, kind: &str) -> &'a Vec<Value> {
    fixture[locale][kind].as_array().expect("case list")
}

fn result(case: &Value) -> &str {
    case["result"].as_str().expect("result")
}

#[test]
fn bytes_match() {
    let fixture = fixture();
    let mut checked = 0;

    for locale in ["en", "pt-BR", "xx"] {
        let format = Format::new(locale);

        for case in cases(&fixture, locale, "bytes") {
            let units = match case["units"].as_str() {
                Some("si") => ByteUnits::Si,
                Some("iec") => ByteUnits::Iec,
                other => unreachable!("unknown units {other:?}"),
            };
            let digits = usize::try_from(case["fractionDigits"].as_u64().unwrap()).unwrap();
            let bytes = case["bytes"].as_i64().expect("bytes");

            assert_eq!(
                format.bytes(bytes, units, digits),
                result(case),
                "{locale} {case}"
            );
            checked += 1;
        }
    }

    println!("format: {checked} bytes cases checked");
    assert_eq!(checked, 342);
}

#[test]
fn compact_numbers_match() {
    let fixture = fixture();
    let mut checked = 0;

    for locale in ["en", "pt-BR", "xx"] {
        let format = Format::new(locale);

        for case in cases(&fixture, locale, "compactNumber") {
            let value = case["value"].as_f64().expect("value");

            assert_eq!(
                format.compact_number(value),
                result(case),
                "{locale} {case}"
            );
            checked += 1;
        }
    }

    println!("format: {checked} compact number cases checked");
    assert_eq!(checked, 30);
}

#[test]
fn relative_times_match() {
    let fixture = fixture();
    let now = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    let mut checked = 0;

    for locale in ["en", "pt-BR", "xx"] {
        let format = Format::new(locale);

        for case in cases(&fixture, locale, "relativeTime") {
            let offset = case["offsetSeconds"].as_i64().expect("offset");
            let time = if offset >= 0 {
                now + Duration::from_secs(offset.unsigned_abs())
            } else {
                now - Duration::from_secs(offset.unsigned_abs())
            };

            assert_eq!(
                format.relative_time(time, now),
                result(case),
                "{locale} {case}"
            );
            assert_eq!(
                format.relative_seconds(offset),
                result(case),
                "{locale} {case}"
            );
            checked += 1;
        }
    }

    println!("format: {checked} relative time cases checked");
    assert_eq!(checked, 75);
}
