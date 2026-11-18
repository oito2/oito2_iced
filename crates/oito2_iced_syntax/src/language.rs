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

//! Languages: the grammars that tokenize code and how a language name
//! finds one.
//!
//! The grammars are the TextMate grammars of `bat`, through `two-face`,
//! loaded the first time a language is looked up (about 30 ms). A
//! language is found by one of the names of [`LANGUAGES`], an alias such
//! as `js`, `yml`, `sh` or `c#`, a file extension such as `rs`, or the
//! name of a grammar such as `Rust`, in any letter case. `plaintext` and
//! unknown names have no grammar: their code shows unhighlighted.
//!
//! ```no_run
//! use oito2_iced_syntax::language;
//!
//! assert!(language::supports("yml"));
//! assert!(!language::supports("plaintext"));
//! ```

use std::sync::LazyLock;

use syntect::parsing::{SyntaxReference, SyntaxSet};

/// The languages of the code block of the design system, by name.
pub const LANGUAGES: [&str; 27] = [
    "bash",
    "c",
    "cpp",
    "csharp",
    "css",
    "dart",
    "diff",
    "dockerfile",
    "go",
    "html",
    "ini",
    "java",
    "javascript",
    "json",
    "kotlin",
    "markdown",
    "php",
    "plaintext",
    "python",
    "rust",
    "scss",
    "shell",
    "sql",
    "swift",
    "typescript",
    "xml",
    "yaml",
];

/// Names that stand for a grammar of another name: each alias and the
/// grammar token it finds (an extension or a grammar name).
const ALIASES: &[(&str, &str)] = &[
    ("csharp", "cs"),
    ("c#", "cs"),
    ("shell", "sh"),
    ("console", "sh"),
    ("zsh", "sh"),
    ("golang", "go"),
    ("docker", "dockerfile"),
    ("jsx", "js"),
    ("mjs", "js"),
    ("cjs", "js"),
    ("tsx", "ts"),
    ("c++", "cpp"),
    ("hpp", "cpp"),
    ("htm", "html"),
    ("xhtml", "html"),
    ("svg", "xml"),
    ("patch", "diff"),
];

/// Names that have no grammar and show plain text.
const PLAIN: [&str; 4] = ["plaintext", "text", "txt", "plain"];

/// Every grammar, with the line endings kept in each line.
static SYNTAXES: LazyLock<SyntaxSet> = LazyLock::new(two_face::syntax::extra_newlines);

/// Every grammar the highlighter knows.
pub fn syntax_set() -> &'static SyntaxSet {
    &SYNTAXES
}

/// The grammar of `language`, or `None` for plain text and unknown
/// languages.
pub fn find(language: &str) -> Option<&'static SyntaxReference> {
    let name = language.trim().to_lowercase();

    if name.is_empty() || PLAIN.contains(&name.as_str()) {
        return None;
    }

    let token = ALIASES
        .iter()
        .find(|(alias, _)| *alias == name)
        .map_or(name.as_str(), |(_, token)| token);

    let syntaxes = syntax_set();
    syntaxes
        .find_syntax_by_token(token)
        .or_else(|| syntaxes.find_syntax_by_token(language.trim()))
}

/// Whether `language` has a grammar.
pub fn supports(language: &str) -> bool {
    find(language).is_some()
}

#[cfg(test)]
mod tests {
    use super::{LANGUAGES, find, supports};

    #[test]
    fn every_language_but_plain_text_has_a_grammar() {
        for name in LANGUAGES {
            assert_eq!(supports(name), name != "plaintext", "{name}");
        }
    }

    #[test]
    fn aliases_find_their_grammar_in_any_case() {
        for (alias, grammar) in [
            ("Dart", "Dart"),
            ("js", "JavaScript"),
            ("ts", "TypeScript"),
            ("sh", "Bourne Again Shell (bash)"),
            ("yml", "YAML"),
            ("py", "Python"),
            ("rs", "Rust"),
            ("C#", "C#"),
            ("Shell", "Bourne Again Shell (bash)"),
        ] {
            assert_eq!(
                find(alias).map(|s| s.name.as_str()),
                Some(grammar),
                "{alias}"
            );
        }

        assert!(!supports("cobol"));
        assert!(!supports(""));
    }
}
