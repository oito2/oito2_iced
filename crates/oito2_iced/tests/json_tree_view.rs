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

//! JSON tree view: JSON paths, the depth the tree opens to, the keys,
//! values and sizes it shows, filtering and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced_test::simulator;
use oito2_iced::collections::TreeCollection;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::json_tree_view::{self, JsonEntry, JsonSummary, JsonValue};
use oito2_iced::widget::tree_view::{TreeEvent, TreeState};
use oito2_iced::{Element, Theme, focus};
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tree(TreeEvent),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// Converts a `serde_json` value into a [`JsonValue`].
fn value(data: &serde_json::Value) -> JsonValue {
    match data {
        serde_json::Value::Null => JsonValue::Null,
        serde_json::Value::Bool(b) => JsonValue::Bool(*b),
        serde_json::Value::Number(n) => match n.as_i64() {
            Some(i) => JsonValue::Integer(i),
            None => JsonValue::Float(n.as_f64().expect("a finite number")),
        },
        serde_json::Value::String(s) => JsonValue::String(s.clone()),
        serde_json::Value::Array(items) => JsonValue::Array(items.iter().map(value).collect()),
        serde_json::Value::Object(map) => {
            JsonValue::Object(map.iter().map(|(k, v)| (k.clone(), value(v))).collect())
        }
    }
}

fn view<'a>(
    tree: &'a TreeCollection<JsonEntry>,
    state: &'a TreeState,
    query: &str,
) -> Element<'a, Message> {
    focus::navigation(
        json_tree_view::json_tree_view(tree, state)
            .on_event(Message::Tree)
            .query(query),
    )
    .into()
}

fn opened(tree: &TreeCollection<JsonEntry>, depth: usize) -> TreeState {
    TreeState::default().with_expanded(json_tree_view::expanded_to_depth(tree, depth))
}

#[test]
fn paths_for_keys_and_indexes() {
    let tree = json_tree_view::json_tree(
        &value(&json!({"name": "oito2", "tags": ["ui", "iced"], "a b": null})),
        json_tree_view::PATH_ROOT_LABEL,
    );

    assert_eq!(
        tree.values(),
        [
            "$",
            "$[\"a b\"]",
            "$.name",
            "$.tags",
            "$.tags[0]",
            "$.tags[1]"
        ]
    );
    assert_eq!(tree.find("$.tags").map(|n| n.label()), Some("tags"));
    assert_eq!(
        tree.find("$.name").map(|n| n.label()),
        Some("name: \"oito2\"")
    );
}

#[test]
fn the_tree_opens_to_the_expand_depth() {
    let data = value(&json!({"a": {"b": {"c": 1}}, "z": [1]}));
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);

    assert!(json_tree_view::expanded_to_depth(&tree, 0).is_empty());
    assert_eq!(json_tree_view::expanded_to_depth(&tree, 1), ["$"]);
    assert_eq!(
        json_tree_view::expanded_to_depth(&tree, 2),
        ["$", "$.a", "$.z"]
    );

    let state = opened(&tree, 2);
    let mut ui = Simulator::with_settings(support::settings(), view(&tree, &state, ""));
    assert!(ui.find("b").is_ok());
    assert!(ui.find("c").is_err());
}

#[test]
fn shows_keys_values_and_sizes() {
    let data =
        value(&json!({"name": "oito2", "stars": 42, "tags": ["ui"], "ok": true, "none": null}));
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
    let state = opened(&tree, 1);
    let mut ui = Simulator::with_settings(support::settings(), view(&tree, &state, ""));

    for text in [
        "root",
        "{5}",
        "name",
        "\"oito2\"",
        "42",
        "[1]",
        "true",
        "null",
    ] {
        assert!(ui.find(text).is_ok(), "{text} is shown");
    }
}

#[test]
fn shows_values_json_cannot_encode() {
    let data = JsonValue::Object(vec![
        ("ratio".into(), JsonValue::Float(f64::NAN)),
        (
            "when".into(),
            JsonValue::Other("2026-01-01 00:00:00.000Z".into()),
        ),
        ("half".into(), JsonValue::Float(0.5)),
    ]);
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
    let state = opened(&tree, 1);
    let mut ui = Simulator::with_settings(support::settings(), view(&tree, &state, ""));

    assert!(ui.find("NaN").is_ok());
    assert!(ui.find("2026-01-01 00:00:00.000Z").is_ok());
    assert!(ui.find("0.5").is_ok());
    assert_eq!(
        tree.find("$.ratio")
            .and_then(|n| n.data.as_ref())
            .map(|e| &e.summary),
        Some(&JsonSummary::Number("NaN".into()))
    );
}

#[test]
fn a_query_filters_by_key_and_value() {
    let data = value(&json!({"name": "oito2", "tags": ["ui", "iced"]}));
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
    let state = opened(&tree, 0);
    let mut ui = Simulator::with_settings(support::settings(), view(&tree, &state, "iced"));

    assert!(
        ui.find("\"iced\"").is_ok(),
        "the match shows inside closed branches"
    );
    assert!(ui.find("tags").is_ok());
    assert!(ui.find("\"ui\"").is_err());
    assert!(ui.find("name").is_err());
}

#[test]
fn the_keyboard_opens_branches() {
    let data = value(&json!({"a": {"b": 1}}));
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
    let state = opened(&tree, 1);
    let mut ui = Simulator::with_settings(support::settings(), view(&tree, &state, ""));

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::ArrowRight);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        [
            Message::Tree(TreeEvent::Select(vec!["$.a".into()])),
            Message::Tree(TreeEvent::Expand {
                value: "$.a".into(),
                expanded: true
            }),
        ]
    );
}

fn snapshot(theme: &Theme, name: &str) {
    let data = value(&json!({
        "name": "oito2_iced",
        "version": "0.1.0",
        "stars": 42,
        "ratio": 0.75,
        "published": false,
        "license": null,
        "tags": ["iced", "ui"],
    }));
    let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
    let state = opened(&tree, 2);
    let mut ui = Simulator::with_size(support::settings(), (300.0, 360.0), view(&tree, &state, ""));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn json_tree_view_light() {
    snapshot(&Theme::light(), "json_tree_view_light");
}

#[test]
fn json_tree_view_dark() {
    snapshot(&Theme::dark(), "json_tree_view_dark");
}

#[test]
fn json_tree_view_dark_high_contrast() {
    let theme = Theme::base(Brightness::Dark, Accent::default().color(), Contrast::High);

    snapshot(&theme, "json_tree_view_dark_high_contrast");
}
