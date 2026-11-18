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

//! The token roles of the syntax colors and the TextMate scopes that map
//! to them.
//!
//! The grammars name every token with a stack of TextMate scopes, such as
//! `source.rust meta.function entity.name.function.rust`. A token takes
//! the role of its innermost scope that [`SCOPE_ROLES`] lists, matched by
//! the longest listed prefix of that scope, unless a scope around it is
//! one of [`OUTER_SCOPE_ROLES`], such as the key of a mapping, whose role
//! covers everything inside it; a token with no listed scope is plain
//! [`Role::Foreground`] text. Comments and `markup.italic` are
//! italic, and `markup.bold` is bold.
//!
//! ```no_run
//! use oito2_iced::Theme;
//! use oito2_iced_syntax::role::Role;
//!
//! let keyword = Role::Keyword.color_in(&Theme::dark().roles().syntax);
//! # let _ = keyword;
//! ```

use std::sync::LazyLock;

use iced::Color;
use oito2_iced::theme::SyntaxColors;
use syntect::parsing::Scope;

/// A token role of [`SyntaxColors`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// Plain code text.
    Foreground,
    /// Comments.
    Comment,
    /// Keywords and control flow.
    Keyword,
    /// String literals.
    String,
    /// Numeric literals.
    Number,
    /// Function and method names.
    Function,
    /// Type and class names.
    Type,
    /// Markup tags.
    Tag,
    /// Markup attributes, annotations and properties.
    Attribute,
    /// Constants, booleans and enum values.
    Constant,
    /// Invalid code and removed lines.
    Invalid,
}

impl Role {
    /// Every role, in declaration order.
    pub const ALL: [Role; 11] = [
        Role::Foreground,
        Role::Comment,
        Role::Keyword,
        Role::String,
        Role::Number,
        Role::Function,
        Role::Type,
        Role::Tag,
        Role::Attribute,
        Role::Constant,
        Role::Invalid,
    ];

    /// The color of this role in `colors`.
    pub fn color_in(self, colors: &SyntaxColors) -> Color {
        match self {
            Role::Foreground => colors.foreground,
            Role::Comment => colors.comment,
            Role::Keyword => colors.keyword,
            Role::String => colors.string,
            Role::Number => colors.number,
            Role::Function => colors.function,
            Role::Type => colors.r#type,
            Role::Tag => colors.tag,
            Role::Attribute => colors.attribute,
            Role::Constant => colors.constant,
            Role::Invalid => colors.invalid,
        }
    }

    /// The role of its name in lower case, such as `keyword`; `None` for
    /// any other text.
    pub fn from_name(name: &str) -> Option<Role> {
        Some(match name {
            "foreground" => Role::Foreground,
            "comment" => Role::Comment,
            "keyword" => Role::Keyword,
            "string" => Role::String,
            "number" => Role::Number,
            "function" => Role::Function,
            "type" => Role::Type,
            "tag" => Role::Tag,
            "attribute" => Role::Attribute,
            "constant" => Role::Constant,
            "invalid" => Role::Invalid,
            _ => return None,
        })
    }
}

/// The role of every TextMate scope prefix the syntax colors color.
///
/// A scope matches the entries that are prefixes of it, atom by atom, and
/// takes the role of the longest one: `constant.character.escape.rust`
/// matches `constant.character` and `constant.character.escape` and is a
/// [`Role::Constant`]. An entry with [`Role::Foreground`] keeps a token
/// plain even inside a colored scope, such as an operator inside an
/// interpolation.
pub const SCOPE_ROLES: &[(&str, Role)] = &[
    // Comments.
    ("comment", Role::Comment),
    ("punctuation.definition.comment", Role::Comment),
    ("markup.quote", Role::Comment),
    ("meta.diff.header", Role::Comment),
    // Keywords; most operators stay plain.
    ("keyword", Role::Keyword),
    ("keyword.operator", Role::Foreground),
    ("keyword.operator.word", Role::Keyword),
    ("keyword.operator.new", Role::Keyword),
    ("keyword.operator.logical.python", Role::Keyword),
    ("keyword.operator.logical.sql", Role::Keyword),
    ("keyword.operator.assignment.alias.sql", Role::Keyword),
    ("keyword.other.unit", Role::Number),
    ("storage", Role::Keyword),
    ("storage.type.function.arrow", Role::Foreground),
    ("storage.type.primitive", Role::Type),
    ("storage.type.buildin", Role::Type),
    ("storage.type.c", Role::Type),
    ("storage.type.c++", Role::Type),
    ("storage.type.cs", Role::Type),
    ("storage.type.string", Role::String),
    ("storage.type.numeric", Role::Number),
    ("storage.modifier.lifetime", Role::Constant),
    ("storage.modifier.attribute", Role::Attribute),
    ("punctuation.definition.list_item", Role::Keyword),
    // Strings.
    ("string", Role::String),
    ("constant.character", Role::String),
    ("constant.other.placeholder", Role::String),
    ("markup.inserted", Role::String),
    ("markup.raw", Role::String),
    ("meta.link.inline.description", Role::String),
    // Numbers.
    ("constant.numeric", Role::Number),
    ("constant.other.color", Role::Number),
    // Functions.
    ("entity.name.function", Role::Function),
    ("variable.function", Role::Function),
    ("support.function", Role::Type),
    ("support.macro", Role::Type),
    // Types.
    ("entity.name.type", Role::Type),
    ("entity.name.class", Role::Type),
    ("entity.name.struct", Role::Type),
    ("entity.name.enum", Role::Type),
    ("entity.name.union", Role::Type),
    ("entity.name.trait", Role::Type),
    ("entity.name.interface", Role::Type),
    ("entity.name.impl", Role::Type),
    ("entity.other.inherited-class", Role::Type),
    ("support.type", Role::Type),
    ("support.type.package", Role::Foreground),
    ("support.type.module", Role::Foreground),
    ("support.class", Role::Type),
    // Markup tags and sections.
    ("entity.name.tag", Role::Tag),
    ("punctuation.definition.tag", Role::Tag),
    ("entity.name.section", Role::Tag),
    ("entity.section", Role::Tag),
    ("markup.heading", Role::Tag),
    // Attributes, properties, annotations and variables.
    ("entity.name.tag.yaml", Role::Attribute),
    ("entity.other.attribute-name", Role::Attribute),
    ("entity.other.pseudo-class", Role::Attribute),
    ("entity.other.pseudo-element", Role::Attribute),
    ("support.type.property-name", Role::Attribute),
    ("keyword.name.ini", Role::Attribute),
    ("meta.annotation", Role::Attribute),
    ("storage.type.annotation", Role::Attribute),
    ("variable.annotation", Role::Attribute),
    ("punctuation.definition.annotation", Role::Attribute),
    ("punctuation.decorator", Role::Attribute),
    ("meta.attribute", Role::Attribute),
    ("meta.preprocessor", Role::Attribute),
    ("keyword.control.import.include", Role::Attribute),
    ("meta.diff.range", Role::Attribute),
    ("variable.other.readwrite.shell", Role::Attribute),
    ("variable.other.php", Role::Attribute),
    ("variable.parameter.php", Role::Attribute),
    ("variable.other.sass", Role::Attribute),
    ("variable.declaration.sass", Role::Attribute),
    ("variable.parameter.sass", Role::Attribute),
    ("punctuation.definition.variable", Role::Attribute),
    // Constants.
    ("constant.language", Role::Constant),
    ("constant.boolean", Role::Constant),
    ("constant.nil", Role::Constant),
    ("keyword.constant", Role::Constant),
    ("constant.character.escape", Role::Constant),
    ("constant.character.entity", Role::Constant),
    ("constant.other", Role::Constant),
    ("constant.other.database-name", Role::Foreground),
    ("constant.other.table-name", Role::Foreground),
    ("support.constant", Role::Constant),
    ("variable.language", Role::Constant),
    ("markup.underline.link", Role::Constant),
    // Invalid code and removed lines.
    ("invalid", Role::Invalid),
    ("markup.deleted", Role::Invalid),
];

/// Scopes whose role covers every token inside them, whatever its own
/// scopes: keys of mappings (a string that is a key is an attribute),
/// regular expressions, INI sections and CSS attribute selectors. The
/// outermost one of a token wins.
pub const OUTER_SCOPE_ROLES: &[(&str, Role)] = &[
    ("meta.mapping.key", Role::Attribute),
    ("string.regexp", Role::String),
    ("meta.tag.section", Role::Tag),
    ("meta.attribute-selector", Role::Attribute),
];

/// Parses the scopes of a table.
fn parse(table: &[(&str, Role)]) -> Vec<(Scope, Role)> {
    table
        .iter()
        .filter_map(|(scope, role)| Scope::new(scope).ok().map(|scope| (scope, *role)))
        .collect()
}

/// The scopes of [`SCOPE_ROLES`], parsed once.
static TABLE: LazyLock<Vec<(Scope, Role)>> = LazyLock::new(|| parse(SCOPE_ROLES));

/// The scopes of [`OUTER_SCOPE_ROLES`], parsed once.
static OUTER: LazyLock<Vec<(Scope, Role)>> = LazyLock::new(|| parse(OUTER_SCOPE_ROLES));

/// The scopes that make a token italic or bold.
static EMPHASIS: LazyLock<[Option<Scope>; 2]> = LazyLock::new(|| {
    [
        Scope::new("markup.italic").ok(),
        Scope::new("markup.bold").ok(),
    ]
});

/// The role of one scope: the role of the longest entry of [`SCOPE_ROLES`]
/// that is a prefix of it, or `None` when no entry is.
pub fn scope_role(scope: Scope) -> Option<Role> {
    TABLE
        .iter()
        .filter(|(prefix, _)| prefix.is_prefix_of(scope))
        .max_by_key(|(prefix, _)| prefix.len())
        .map(|(_, role)| *role)
}

/// How one token is drawn: its role and whether it is italic or bold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Token {
    /// The role, which gives the color.
    pub role: Role,
    /// Whether the token is italic: comments and emphasized markup.
    pub italic: bool,
    /// Whether the token is bold: strong markup.
    pub bold: bool,
}

impl Token {
    /// Plain text in the foreground color.
    pub const PLAIN: Token = Token {
        role: Role::Foreground,
        italic: false,
        bold: false,
    };

    /// The token of a stack of scopes, outermost first: the role of the
    /// outermost scope of [`OUTER_SCOPE_ROLES`], or else of the innermost
    /// scope that has one (see [`scope_role`]); italic for comments and
    /// `markup.italic`, bold for `markup.bold`.
    pub fn of_scopes(scopes: &[Scope]) -> Token {
        let outer = scopes.iter().find_map(|scope| {
            OUTER
                .iter()
                .find(|(prefix, _)| prefix.is_prefix_of(*scope))
                .map(|(_, role)| *role)
        });
        let role = outer
            .or_else(|| scopes.iter().rev().find_map(|scope| scope_role(*scope)))
            .unwrap_or(Role::Foreground);
        let [italic, bold] = &*EMPHASIS;
        let any = |prefix: &Option<Scope>| {
            prefix.is_some_and(|prefix| scopes.iter().any(|scope| prefix.is_prefix_of(*scope)))
        };

        Token {
            role,
            italic: role == Role::Comment || any(italic),
            bold: any(bold),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Role, Token, scope_role};
    use syntect::parsing::Scope;

    fn scope(text: &str) -> Scope {
        Scope::new(text).unwrap_or_default()
    }

    #[test]
    fn the_longest_prefix_wins() {
        assert_eq!(
            scope_role(scope("constant.character.escape.rust")),
            Some(Role::Constant)
        );
        assert_eq!(
            scope_role(scope("constant.character.rust")),
            Some(Role::String)
        );
        assert_eq!(
            scope_role(scope("keyword.operator.arithmetic")),
            Some(Role::Foreground)
        );
        assert_eq!(
            scope_role(scope("keyword.control.rust")),
            Some(Role::Keyword)
        );
        assert_eq!(scope_role(scope("source.rust")), None);
    }

    #[test]
    fn the_innermost_scope_with_a_role_wins() {
        let token = Token::of_scopes(&[
            scope("source.rust"),
            scope("string.quoted.double.rust"),
            scope("punctuation.definition.string.begin.rust"),
        ]);
        assert_eq!(token.role, Role::String);

        let token = Token::of_scopes(&[scope("source.rust"), scope("comment.line.rust")]);
        assert_eq!(token.role, Role::Comment);
        assert!(token.italic);

        let token = Token::of_scopes(&[scope("text.html.markdown"), scope("markup.bold")]);
        assert_eq!(
            token,
            Token {
                role: Role::Foreground,
                italic: false,
                bold: true
            }
        );
    }

    #[test]
    fn every_role_is_used_by_some_scope() {
        for role in Role::ALL {
            let used = super::SCOPE_ROLES.iter().any(|(_, r)| *r == role);
            assert!(used, "{role:?}");
        }
    }

    #[test]
    fn every_listed_scope_parses() {
        assert_eq!(super::TABLE.len(), super::SCOPE_ROLES.len());
        assert_eq!(super::OUTER.len(), super::OUTER_SCOPE_ROLES.len());
    }
}
