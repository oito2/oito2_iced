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

//! The documented items as the generator collects them, before they are
//! written as Rust source.

/// The kind of a documented item or member, named as the `Kind` enum of
/// the showcase names its variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    /// A module.
    Module,
    /// A struct or a union.
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
    /// A type alias or an associated type.
    Type,
    /// An exported macro.
    Macro,
    /// An enum variant.
    Variant,
    /// A public field.
    Field,
}

impl Kind {
    /// The name of the matching variant of the showcase `Kind` enum.
    pub fn variant(self) -> &'static str {
        match self {
            Kind::Module => "Module",
            Kind::Struct => "Struct",
            Kind::Enum => "Enum",
            Kind::Trait => "Trait",
            Kind::Function => "Function",
            Kind::Method => "Method",
            Kind::Constant => "Constant",
            Kind::Static => "Static",
            Kind::Type => "Type",
            Kind::Macro => "Macro",
            Kind::Variant => "Variant",
            Kind::Field => "Field",
        }
    }
}

/// A member of an item: a method, an associated constant or type, a
/// variant or a public field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// The name of the member.
    pub name: String,
    /// What the member is.
    pub kind: Kind,
    /// The declaration, without its body.
    pub signature: String,
    /// The first paragraph of the doc comment.
    pub summary: String,
}

/// A public item at one of its public paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The full path, starting with the crate name.
    pub path: String,
    /// The last segment of the path.
    pub name: String,
    /// What the item is.
    pub kind: Kind,
    /// The declaration, without its body or private fields.
    pub signature: String,
    /// The doc comment, in Markdown.
    pub docs: String,
    /// The public members, in source order.
    pub members: Vec<Member>,
}
