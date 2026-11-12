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

//! Checks the localized strings against `l10n.json`, the parity fixture
//! exported by `oito2_ui`.

use oito2_iced::l10n::{Localizations, TimeUnit};
use serde_json::Value;

fn fixture() -> Value {
    let text = include_str!("fixtures/parity/l10n.json");
    serde_json::from_str(text).expect("l10n.json is valid JSON")
}

fn snake(key: &str) -> String {
    let mut out = String::new();

    for c in key.chars() {
        if c.is_ascii_uppercase() {
            out.push('_');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }

    out
}

fn sorted<S: AsRef<str>>(keys: &[S]) -> Vec<String> {
    let mut keys: Vec<String> = keys.iter().map(|k| k.as_ref().to_owned()).collect();
    keys.sort();
    keys
}

fn locales() -> [(&'static str, Localizations); 2] {
    [
        ("en", Localizations::en()),
        ("pt-BR", Localizations::for_locale("pt-BR")),
    ]
}

fn int(value: &Value) -> i64 {
    value.as_i64().expect("integer argument")
}

fn text(value: &Value) -> &str {
    value.as_str().expect("string argument")
}

fn call(l: &Localizations, name: &str, args: &[Value]) -> String {
    match name {
        "pageOf" => l.page_of(int(&args[0]), int(&args[1])),
        "partOf" => l.part_of(int(&args[0]), int(&args[1])),
        "timeAgo" => l.time_ago(int(&args[0]), unit(&args[1])),
        "timeFromNow" => l.time_from_now(int(&args[0]), unit(&args[1])),
        "resultCount" => l.result_count(int(&args[0])),
        "tourProgress" => l.tour_progress(int(&args[0]), int(&args[1])),
        "codeEntry" => l.code_entry(int(&args[0])),
        "ratingValue" => l.rating_value(text(&args[0]), int(&args[1])),
        "removeItem" => l.remove_item(text(&args[0])),
        "weekNumber" => l.week_number(int(&args[0])),
        "pageNumber" => l.page_number(int(&args[0])),
        "stepOf" => l.step_of(int(&args[0]), int(&args[1])),
        "dateRange" => l.date_range(text(&args[0]), text(&args[1])),
        "moreCount" => l.more_count(int(&args[0])),
        "acceptedFileTypes" => l.accepted_file_types(text(&args[0])),
        "maxFileSizeHint" => l.max_file_size_hint(text(&args[0])),
        "fileTypeNotAccepted" => l.file_type_not_accepted(text(&args[0])),
        "fileTooLarge" => l.file_too_large(text(&args[0]), text(&args[1])),
        "tooManyFiles" => l.too_many_files(text(&args[0]), int(&args[1])),
        "folderNotAccepted" => l.folder_not_accepted(text(&args[0])),
        other => unreachable!("unknown call {other}"),
    }
}

fn unit(value: &Value) -> TimeUnit {
    TimeUnit::from_name(text(value)).expect("known time unit")
}

#[test]
fn locale_names_match() {
    let fixture = fixture();

    for (key, l) in locales() {
        assert_eq!(l.locale_name, fixture[key]["localeName"].as_str().unwrap());
    }
}

#[test]
fn material_strings_match() {
    let fixture = fixture();
    let mut checked = 0;

    for (key, l) in locales() {
        let expected = fixture[key]["material"].as_object().expect("material");

        for (name, value) in expected {
            assert_eq!(
                l.material.get(&snake(name)),
                value.as_str(),
                "{key} material {name}"
            );
            checked += 1;
        }
    }

    println!("l10n: {checked} material strings checked");
    assert_eq!(checked, 40);
}

#[test]
fn getters_match() {
    let fixture = fixture();
    let mut checked = 0;

    for (key, l) in locales() {
        let expected = fixture[key]["getters"].as_object().expect("getters");

        for (name, value) in expected {
            assert_eq!(l.get(&snake(name)), value.as_str(), "{key} getter {name}");
            checked += 1;
        }
    }

    println!("l10n: {checked} getters checked");
    assert_eq!(checked, 166);
}

#[test]
fn calls_match() {
    let fixture = fixture();
    let mut checked = 0;

    for (key, l) in locales() {
        let calls = fixture[key]["calls"].as_array().expect("calls");

        for case in calls {
            let name = text(&case["name"]);
            let args = case["args"].as_array().expect("args");

            assert_eq!(
                call(&l, name, args),
                text(&case["result"]),
                "{key} call {name} {args:?}"
            );
            checked += 1;
        }
    }

    println!("l10n: {checked} calls checked");
    assert_eq!(checked, 98);
}

#[test]
fn locales_expose_the_same_keys() {
    let fixture = fixture();

    for section in ["material", "getters"] {
        let en: Vec<_> = fixture["en"][section].as_object().unwrap().keys().collect();
        let pt: Vec<_> = fixture["pt-BR"][section]
            .as_object()
            .unwrap()
            .keys()
            .collect();

        assert_eq!(en, pt, "{section} keys differ");
    }

    let names = |key: &str| -> Vec<String> {
        fixture[key]["calls"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| text(&c["name"]).to_owned())
            .collect()
    };

    assert_eq!(names("en"), names("pt-BR"));
    assert_eq!(Localizations::en().get("close"), Some("Close"));
    assert_eq!(Localizations::pt_br().get("close"), Some("Fechar"));
}

#[test]
fn every_key_is_covered_by_the_fixture() {
    let fixture = fixture();
    let getters = fixture["en"]["getters"].as_object().unwrap();
    let material = fixture["en"]["material"].as_object().unwrap();

    let fixture_getters: Vec<_> = getters.keys().map(|k| snake(k)).collect();
    let fixture_material: Vec<_> = material.keys().map(|k| snake(k)).collect();

    assert_eq!(
        sorted(&fixture_getters),
        sorted(oito2_iced::l10n::Strings::KEYS)
    );
    assert_eq!(
        sorted(&fixture_material),
        sorted(oito2_iced::l10n::MaterialStrings::KEYS)
    );
}

#[test]
fn locale_lookup_follows_the_language_code() {
    for tag in ["pt", "pt-BR", "pt_BR", "pt_PT", "PT-br"] {
        assert_eq!(Localizations::for_locale(tag).locale_name, "pt_BR", "{tag}");
    }

    for tag in ["en", "en-US", "de", "xx", ""] {
        assert_eq!(Localizations::for_locale(tag).locale_name, "en", "{tag}");
    }
}
