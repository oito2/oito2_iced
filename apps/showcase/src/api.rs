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

//! The API reference shown at the end of the component pages, generated
//! from the public items of the workspace crates and their doc comments by
//! `cargo run -p oito2_iced_gen_docs` into `generated/api_reference.rs`.

use iced::widget::{column, row};
use iced::{Element, Length};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;

use crate::doc::{self, Context};

// The generated file is written by the generator, not by rustfmt.
#[rustfmt::skip]
#[path = "generated/api_reference.rs"]
mod generated;

/// The kind of a public item.
// The generated reference may hold no item of some kinds.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A module.
    Module,
    /// A struct.
    Struct,
    /// An enum.
    Enum,
    /// A trait.
    Trait,
    /// A free function.
    Function,
    /// A method or associated function.
    Method,
    /// A constant or an associated constant.
    Constant,
    /// A static.
    Static,
    /// A type alias.
    Type,
    /// A macro.
    Macro,
    /// An enum variant.
    Variant,
    /// A public field.
    Field,
}

impl Kind {
    /// The name of the kind as the reference shows it.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Module => "module",
            Kind::Struct => "struct",
            Kind::Enum => "enum",
            Kind::Trait => "trait",
            Kind::Function => "function",
            Kind::Method => "method",
            Kind::Constant => "constant",
            Kind::Static => "static",
            Kind::Type => "type",
            Kind::Macro => "macro",
            Kind::Variant => "variant",
            Kind::Field => "field",
        }
    }
}

/// A member of an item: a method, an associated constant, a variant or a
/// public field.
#[derive(Debug, Clone, Copy)]
pub struct Member {
    /// The name of the member.
    pub name: &'static str,
    /// What the member is.
    pub kind: Kind,
    /// The declaration, without its body.
    pub signature: &'static str,
    /// The first paragraph of the doc comment.
    pub summary: &'static str,
}

/// A public item of a workspace crate.
#[derive(Debug, Clone, Copy)]
pub struct Item {
    /// The full path, such as `oito2_iced::widget::copy::copy_button`.
    pub path: &'static str,
    /// The name of the item.
    pub name: &'static str,
    /// What the item is.
    pub kind: Kind,
    /// The declaration, without its body or private fields.
    pub signature: &'static str,
    /// The doc comment, in Markdown.
    pub docs: &'static str,
    /// The public members, in source order.
    pub members: &'static [Member],
}

/// Every documented item.
pub fn items() -> &'static [Item] {
    generated::ITEMS
}

/// The item at `path`.
pub fn find(path: &str) -> Option<&'static Item> {
    items().iter().find(|item| item.path == path)
}

/// The reference of `item`: its name, kind and crate, its declaration, its
/// doc comment and its members.
pub fn view<'a, Message: Clone + 'a>(
    ctx: &Context,
    item: &'static Item,
) -> Element<'a, Message, Theme> {
    let krate = item.path.split("::").next().unwrap_or_default();

    let mut entry = column![
        column![
            doc::title(item.name),
            doc::caption(format!("{} · {krate}", item.kind.label())),
        ]
        .spacing(spacing::XS),
        doc::code(ctx, item.signature),
    ]
    .spacing(spacing::MD)
    .width(Length::Fill);

    for block in doc::markdown(ctx, item.docs) {
        entry = entry.push(block);
    }

    if !item.members.is_empty() {
        entry = entry.push(
            column(item.members.iter().map(|member| {
                column![
                    row![doc::title(member.name), doc::caption(member.kind.label())]
                        .spacing(spacing::SM)
                        .align_y(iced::Alignment::Center),
                    doc::mono(member.signature),
                    doc::prose(member.summary),
                ]
                .spacing(spacing::XS)
                .into()
            }))
            .spacing(spacing::MD),
        );
    }

    border_container(entry)
        .padding(spacing::LG)
        .width(Length::Fill)
        .into()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    /// The string literals inside the `&[...]` slice of every `doc::api(`
    /// call of `source`.
    fn api_paths(source: &str) -> Vec<String> {
        let mut paths = Vec::new();
        for (start, _) in source.match_indices("doc::api(") {
            let rest = &source[start..];
            let Some(open) = rest.find("&[") else {
                continue;
            };
            let Some(close) = rest[open..].find(']') else {
                continue;
            };
            let list = &rest[open + 2..open + close];
            paths.extend(list.split('"').skip(1).step_by(2).map(str::to_owned));
        }
        paths
    }

    #[test]
    fn every_path_passed_to_doc_api_is_in_the_reference() {
        let pages = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/pages");
        let mut files: Vec<_> = fs::read_dir(&pages)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
            .collect();
        files.sort();

        let mut missing = Vec::new();
        for file in files {
            let source = fs::read_to_string(&file).unwrap();
            for path in api_paths(&source) {
                if super::find(&path).is_none() {
                    let name = file.file_name().unwrap_or_default().to_string_lossy();
                    missing.push(format!("{name}: {path}"));
                }
            }
        }
        assert!(
            missing.is_empty(),
            "paths missing from the API reference:\n{}",
            missing.join("\n")
        );
    }
}
