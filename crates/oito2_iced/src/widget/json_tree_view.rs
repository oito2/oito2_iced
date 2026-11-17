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

//! A tree view of JSON-like values.
//!
//! [`json_tree`] turns a [`JsonValue`] into a [`TreeCollection`]: objects
//! and arrays become branches, other values leaves, and node values are
//! JSON paths such as `$.items[0].name`. [`json_tree_view`] shows it in a
//! [`TreeView`] with keys, strings, numbers, booleans and null in the
//! theme's syntax colors and the size of each object and array; it has the
//! keyboard and filtering of the tree view, and a
//! [query](TreeView::query) filters by key and value.
//!
//! The open branches live in a [`TreeState`] the app owns;
//! [`expanded_to_depth`] lists the branches to open so the tree starts
//! open to a depth, and the app opens them again when the data changes.
//!
//! ```no_run
//! use oito2_iced::collections::TreeCollection;
//! use oito2_iced::widget::json_tree_view::{self, JsonEntry, JsonValue};
//! use oito2_iced::widget::tree_view::{TreeEvent, TreeState};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Tree(TreeEvent),
//! }
//!
//! let data = JsonValue::Object(vec![
//!     ("name".into(), JsonValue::String("oito2".into())),
//!     ("stars".into(), JsonValue::Integer(42)),
//! ]);
//! let tree = json_tree_view::json_tree(&data, json_tree_view::DEFAULT_ROOT_LABEL);
//! let state = TreeState::default().with_expanded(json_tree_view::expanded_to_depth(&tree, 1));
//!
//! # fn view<'a>(tree: &'a TreeCollection<JsonEntry>, state: &'a TreeState) -> oito2_iced::Element<'a, Message> {
//! json_tree_view::json_tree_view(tree, state)
//!     .on_event(Message::Tree)
//!     .into()
//! # }
//! ```

use std::collections::HashMap;
use std::fmt::Write as _;

use iced::widget::text::{Style, Wrapping};
use iced::widget::{Row, container};
use iced::{Color, Length};

use super::styled_text;
use super::tree_view::{TreeState, TreeView, tree_view};
use crate::collections::{TreeCollection, TreeNode};
use crate::theme::SyntaxColors;
use crate::tokens::typography;
use crate::{Element, Theme};

/// The label of the top node of a [`json_tree_view`] in the OITO2 design.
pub const DEFAULT_ROOT_LABEL: &str = "root";

/// The label of the top node in JSON path notation.
pub const PATH_ROOT_LABEL: &str = "$";

/// A decoded JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// `null`.
    Null,
    /// `true` or `false`.
    Bool(bool),
    /// A whole number.
    Integer(i64),
    /// A number with a fraction, which may be NaN or infinite.
    Float(f64),
    /// A string.
    String(String),
    /// An array.
    Array(Vec<JsonValue>),
    /// An object, with its keys in order; a repeated key keeps its last
    /// value at its first position.
    Object(Vec<(String, JsonValue)>),
    /// A value JSON can't encode, such as a date, shown as this text.
    Other(String),
}

/// What a node of a JSON tree shows after its key.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonSummary {
    /// An object with this many keys.
    Object(usize),
    /// An array with this many items.
    Array(usize),
    /// A string, JSON-encoded with its quotes.
    String(String),
    /// A number, as text.
    Number(String),
    /// A boolean.
    Bool(bool),
    /// `null`.
    Null,
    /// A value JSON can't encode, as text.
    Other(String),
}

impl JsonSummary {
    /// The text shown after the key: the size of an object (`{2}`) or an
    /// array (`[3]`), or the value.
    #[must_use]
    pub fn text(&self) -> String {
        match self {
            JsonSummary::Object(size) => format!("{{{size}}}"),
            JsonSummary::Array(size) => format!("[{size}]"),
            JsonSummary::String(text) | JsonSummary::Number(text) | JsonSummary::Other(text) => {
                text.clone()
            }
            JsonSummary::Bool(value) => value.to_string(),
            JsonSummary::Null => "null".to_owned(),
        }
    }

    /// Whether the node is an object or an array.
    #[must_use]
    pub fn is_container(&self) -> bool {
        matches!(self, JsonSummary::Object(_) | JsonSummary::Array(_))
    }

    /// The syntax color of the text after the key.
    fn color(&self, syntax: &SyntaxColors) -> Color {
        match self {
            JsonSummary::Object(_) | JsonSummary::Array(_) => syntax.comment,
            JsonSummary::String(_) => syntax.string,
            JsonSummary::Number(_) => syntax.number,
            JsonSummary::Bool(_) => syntax.keyword,
            JsonSummary::Null => syntax.constant,
            JsonSummary::Other(_) => syntax.foreground,
        }
    }
}

/// The key and summary of a node of a JSON tree.
#[derive(Debug, Clone, PartialEq)]
pub struct JsonEntry {
    /// The key in its object, the index in its array, or the root label.
    pub key: String,
    /// What the node shows after its key.
    pub summary: JsonSummary,
}

/// Returns `text` JSON-encoded, with quotes and escapes.
#[must_use]
pub fn encode_string(text: &str) -> String {
    let mut encoded = String::with_capacity(text.len() + 2);
    encoded.push('"');

    for c in text.chars() {
        match c {
            '"' => encoded.push_str("\\\""),
            '\\' => encoded.push_str("\\\\"),
            '\n' => encoded.push_str("\\n"),
            '\r' => encoded.push_str("\\r"),
            '\t' => encoded.push_str("\\t"),
            '\u{8}' => encoded.push_str("\\b"),
            '\u{c}' => encoded.push_str("\\f"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(encoded, "\\u{:04x}", u32::from(c));
            }
            c => encoded.push(c),
        }
    }

    encoded.push('"');
    encoded
}

/// Returns `value` as text: `NaN`, `Infinity` and `-Infinity` for the
/// values JSON can't encode, and a whole number with `.0`.
fn float_text(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_owned()
    } else if value.is_infinite() {
        if value > 0.0 { "Infinity" } else { "-Infinity" }.to_owned()
    } else {
        format!("{value:?}")
    }
}

/// Whether `key` can follow a dot in a JSON path.
fn is_identifier(key: &str) -> bool {
    let mut chars = key.chars();

    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
}

/// The JSON path of the member `key` of the object at `parent`.
fn member_path(parent: &str, key: &str) -> String {
    if is_identifier(key) {
        format!("{parent}.{key}")
    } else {
        format!("{parent}[{}]", encode_string(key))
    }
}

/// The members of an object with each repeated key keeping its last value
/// at its first position.
fn members(members: &[(String, JsonValue)]) -> Vec<(&str, &JsonValue)> {
    let mut result: Vec<(&str, &JsonValue)> = Vec::with_capacity(members.len());
    let mut positions: HashMap<&str, usize> = HashMap::new();

    for (key, value) in members {
        if let Some(&position) = positions.get(key.as_str()) {
            result[position].1 = value;
        } else {
            positions.insert(key, result.len());
            result.push((key, value));
        }
    }

    result
}

/// Builds the node of `value` at `path` with `key`.
fn node(path: String, key: &str, value: &JsonValue) -> TreeNode<JsonEntry> {
    let (summary, children) = match value {
        JsonValue::Object(fields) => {
            let fields = members(fields);
            let children = fields
                .iter()
                .map(|(k, v)| node(member_path(&path, k), k, v))
                .collect();

            (JsonSummary::Object(fields.len()), Some(children))
        }
        JsonValue::Array(items) => {
            let children = items
                .iter()
                .enumerate()
                .map(|(index, item)| node(format!("{path}[{index}]"), &index.to_string(), item))
                .collect();

            (JsonSummary::Array(items.len()), Some(children))
        }
        JsonValue::String(text) => (JsonSummary::String(encode_string(text)), None),
        JsonValue::Integer(number) => (JsonSummary::Number(number.to_string()), None),
        JsonValue::Float(number) => (JsonSummary::Number(float_text(*number)), None),
        JsonValue::Bool(value) => (JsonSummary::Bool(*value), None),
        JsonValue::Null => (JsonSummary::Null, None),
        JsonValue::Other(text) => (JsonSummary::Other(text.clone()), None),
    };

    let label = if summary.is_container() {
        key.to_owned()
    } else {
        format!("{key}: {}", summary.text())
    };
    let entry = JsonEntry {
        key: key.to_owned(),
        summary,
    };
    let node = match children {
        Some(children) => TreeNode::branch(path, children),
        None => TreeNode::leaf(path),
    };

    node.with_label(label).with_data(entry)
}

/// Returns the tree of `data`: objects and arrays become branches, other
/// values leaves, and the top node is labeled `root_label`. Node values
/// are JSON paths, such as `$.items[0].name` or `$["a b"]`; labels are the
/// key of a branch and `key: value` for a leaf, which the search matches.
#[must_use]
pub fn json_tree(data: &JsonValue, root_label: &str) -> TreeCollection<JsonEntry> {
    // JSON paths are unique within one value.
    TreeCollection::new(vec![node(PATH_ROOT_LABEL.to_owned(), root_label, data)])
        .unwrap_or_default()
}

/// The values of the branches of `collection` shallower than `depth`: the
/// branches to open so the tree shows `depth` levels below the top; 0
/// keeps every branch closed.
#[must_use]
pub fn expanded_to_depth<T>(collection: &TreeCollection<T>, depth: usize) -> Vec<String> {
    collection
        .flatten()
        .into_iter()
        .filter(|entry| entry.node.is_branch() && entry.depth < depth)
        .map(|entry| entry.node.value.clone())
        .collect()
}

/// A text of the JSON row in the syntax color `pick` chooses.
fn colored<'a, Message: 'a>(
    text: String,
    pick: fn(&SyntaxColors) -> Color,
) -> Element<'a, Message> {
    styled_text(text, typography::BODY_MEDIUM)
        .wrapping(Wrapping::None)
        .style(move |theme: &Theme| Style {
            color: Some(pick(&theme.roles().syntax)),
        })
        .into()
}

/// One JSON row: the key, then the value in its syntax color, or the size
/// of an object or array.
fn json_row<'a, Message: 'a>(node: &'a TreeNode<JsonEntry>) -> Element<'a, Message> {
    let Some(entry) = &node.data else {
        return styled_text(node.label(), typography::BODY_MEDIUM)
            .wrapping(Wrapping::None)
            .into();
    };

    let separator = if entry.summary.is_container() {
        " "
    } else {
        ": "
    };
    let value: Element<'a, Message> = {
        let summary = entry.summary.clone();

        styled_text(summary.text(), typography::BODY_MEDIUM)
            .wrapping(Wrapping::None)
            .style(move |theme: &Theme| Style {
                color: Some(summary.color(&theme.roles().syntax)),
            })
            .into()
    };

    container(
        Row::with_children([
            colored(entry.key.clone(), |syntax| syntax.attribute),
            colored(separator.to_owned(), |syntax| syntax.foreground),
            value,
        ])
        .width(Length::Shrink),
    )
    .width(Length::Fill)
    .clip(true)
    .into()
}

/// A tree view of the JSON tree `collection` (built by [`json_tree`]) with
/// the open branches of `state`; it is a [`TreeView`], so it takes the
/// tree view builders, such as [`TreeView::on_event`] and
/// [`TreeView::query`].
pub fn json_tree_view<'a, Message: Clone + 'a>(
    collection: &'a TreeCollection<JsonEntry>,
    state: &'a TreeState,
) -> TreeView<'a, JsonEntry, Message> {
    tree_view(collection, state).item(json_row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_are_json_encoded() {
        assert_eq!(encode_string("a \"b\"\n"), "\"a \\\"b\\\"\\n\"");
        assert_eq!(encode_string("\u{1}é"), "\"\\u0001é\"");
    }

    #[test]
    fn floats_read_like_the_design() {
        assert_eq!(float_text(1.0), "1.0");
        assert_eq!(float_text(2.5), "2.5");
        assert_eq!(float_text(f64::NAN), "NaN");
        assert_eq!(float_text(f64::NEG_INFINITY), "-Infinity");
    }

    #[test]
    fn a_repeated_key_keeps_its_last_value() {
        let tree = json_tree(
            &JsonValue::Object(vec![
                ("a".into(), JsonValue::Integer(1)),
                ("b".into(), JsonValue::Null),
                ("a".into(), JsonValue::Integer(2)),
            ]),
            PATH_ROOT_LABEL,
        );

        assert_eq!(tree.values(), ["$", "$.a", "$.b"]);
        assert_eq!(tree.find("$.a").map(TreeNode::label), Some("a: 2"));
    }
}
