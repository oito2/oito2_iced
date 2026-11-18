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

//! Highlighting: code split into runs of [`Token`]s, and a highlighter for
//! the Iced text editor.
//!
//! [`highlight`] tokenizes a whole text with the grammar of a language
//! (see [`language`]) and returns its runs. The colors
//! are not part of a run: they come from the role of its token in the
//! [`SyntaxColors`](oito2_iced::theme::SyntaxColors) of the theme when the
//! text is drawn, so a theme change recolors without tokenizing again.
//!
//! [`Highlighter`] highlights a [`text_editor()`](iced::widget::text_editor())
//! line by line, keeping the parser state at the start of every line so
//! an edit tokenizes again only from the changed line down. Its
//! [`Settings`] carry the language, and [`to_format`] turns each token
//! into the color of its role in the theme and the JetBrains Mono italic
//! or bold face where the token asks for one:
//!
//! ```no_run
//! use iced::widget::text_editor;
//! use oito2_iced_syntax::font;
//! use oito2_iced_syntax::highlighter::{self, Highlighter, Settings};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Edit(text_editor::Action),
//! }
//!
//! # fn view(content: &text_editor::Content) -> oito2_iced::Element<'_, Message> {
//! text_editor(content)
//!     .font(font::JETBRAINS_MONO)
//!     .on_action(Message::Edit)
//!     .highlight_with::<Highlighter>(Settings::new("rust"), highlighter::to_format)
//!     .into()
//! # }
//! ```

use std::collections::VecDeque;
use std::ops::Range;
use std::sync::{Arc, LazyLock, Mutex};

use iced::Font;
use iced::advanced::text::highlighter::{self, Format};
use iced::font::Weight;
use oito2_iced::Theme;
use syntect::parsing::{ParseState, ScopeStack, SyntaxReference};
use syntect::util::LinesWithEndings;

use crate::font::{jetbrains_mono, jetbrains_mono_italic};
use crate::language;
use crate::role::Token;

/// A piece of text drawn with one token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// The byte range of the piece in the text.
    pub range: Range<usize>,
    /// How the piece is drawn.
    pub token: Token,
}

/// The parser state of one position in a text: the grammar state and the
/// scopes open there.
#[derive(Debug, Clone)]
struct Parser {
    state: ParseState,
    scopes: ScopeStack,
}

impl Parser {
    /// The state at the start of a text in `syntax`.
    fn new(syntax: &SyntaxReference) -> Self {
        Self {
            state: ParseState::new(syntax),
            scopes: ScopeStack::new(),
        }
    }

    /// Tokenizes `line`, which ends with its line break when it has one,
    /// pushing its runs, offset by `offset`, to `runs`. A line the grammar
    /// fails on is pushed as plain text.
    fn line(&mut self, line: &str, offset: usize, runs: &mut Vec<Run>) {
        let Ok(operations) = self.state.parse_line(line, language::syntax_set()) else {
            push(runs, offset..offset + line.len(), Token::PLAIN);
            return;
        };

        let mut start = 0;

        for (index, operation) in operations {
            let index = index.min(line.len());

            if index > start {
                let token = Token::of_scopes(self.scopes.as_slice());
                push(runs, offset + start..offset + index, token);
                start = index;
            }

            // A scope error leaves the stack as it was; the next lines
            // keep tokenizing.
            let _ = self.scopes.apply(&operation);
        }

        if start < line.len() {
            let token = Token::of_scopes(self.scopes.as_slice());
            push(runs, offset + start..offset + line.len(), token);
        }
    }
}

/// Appends a run to `runs`, merging it into the last run when both have
/// the same token and touch.
fn push(runs: &mut Vec<Run>, range: Range<usize>, token: Token) {
    if range.is_empty() {
        return;
    }

    if let Some(last) = runs.last_mut()
        && last.token == token
        && last.range.end == range.start
    {
        last.range.end = range.end;
        return;
    }

    runs.push(Run { range, token });
}

/// The runs of `code` highlighted as `language`.
///
/// The runs cover the whole text, in order. Plain text and an unknown
/// `language` give one run of [`Token::PLAIN`] (none for an empty text).
pub fn highlight(code: &str, language: &str) -> Vec<Run> {
    let mut runs = Vec::new();

    let Some(syntax) = language::find(language) else {
        push(&mut runs, 0..code.len(), Token::PLAIN);
        return runs;
    };

    let mut parser = Parser::new(syntax);
    let mut offset = 0;

    for line in LinesWithEndings::from(code) {
        parser.line(line, offset, &mut runs);
        offset += line.len();
    }

    runs
}

/// How many highlighted texts [`highlight_cached`] keeps.
const CACHE_SIZE: usize = 32;

/// One highlighted text of the cache: its language, its code and its runs.
type Entry = (String, String, Arc<[Run]>);

/// The recently highlighted texts, the most recent last.
static CACHE: LazyLock<Mutex<VecDeque<Entry>>> =
    LazyLock::new(|| Mutex::new(VecDeque::with_capacity(CACHE_SIZE)));

/// The runs of `code` highlighted as `language`, like [`highlight`], from
/// a cache of the last 32 texts highlighted by any caller, so a view that
/// is built again with the same code tokenizes it only once. The same text
/// and language give the same shared runs while they stay in the cache.
pub fn highlight_cached(code: &str, language: &str) -> Arc<[Run]> {
    if let Ok(mut cache) = CACHE.lock()
        && let Some(index) = cache
            .iter()
            .position(|(lang, text, _)| lang == language && text == code)
        && let Some(entry) = cache.remove(index)
    {
        let runs = Arc::clone(&entry.2);
        cache.push_back(entry);
        return runs;
    }

    let runs: Arc<[Run]> = highlight(code, language).into();

    if let Ok(mut cache) = CACHE.lock() {
        if cache.len() >= CACHE_SIZE {
            cache.pop_front();
        }

        cache.push_back((language.to_owned(), code.to_owned(), Arc::clone(&runs)));
    }

    runs
}

/// The JetBrains Mono face of `token`: `None` for the regular face, or the
/// italic, bold or bold italic face.
pub fn font_of(token: &Token) -> Option<Font> {
    let weight = if token.bold {
        Weight::Bold
    } else {
        Weight::Normal
    };

    match (token.italic, token.bold) {
        (false, false) => None,
        (true, _) => Some(jetbrains_mono_italic(weight)),
        (false, true) => Some(jetbrains_mono(weight)),
    }
}

/// The settings of a [`Highlighter`]: the language of the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    /// The language: a name, alias or extension (see
    /// [`language`]); plain text and unknown languages
    /// show the text unhighlighted in the foreground color.
    pub language: String,
}

impl Settings {
    /// The settings that highlight `language`.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
        }
    }
}

/// A highlighter for the Iced text editor: tokenizes each line with the
/// grammar of the language of its [`Settings`].
///
/// It keeps the parser state at the start of every line it highlighted,
/// so after an edit it starts again from the changed line.
#[derive(Debug)]
pub struct Highlighter {
    syntax: Option<&'static SyntaxReference>,
    /// The parser state at the start of each line highlighted so far, and
    /// of the next one.
    starts: Vec<Parser>,
    current_line: usize,
}

impl Highlighter {
    /// The parser state at the start of a text in the current grammar.
    fn first(&self) -> Vec<Parser> {
        self.syntax.map(Parser::new).into_iter().collect()
    }
}

impl highlighter::Highlighter for Highlighter {
    type Settings = Settings;
    type Highlight = Token;
    type Iterator<'a> = std::vec::IntoIter<(Range<usize>, Token)>;

    fn new(settings: &Self::Settings) -> Self {
        let mut highlighter = Self {
            syntax: language::find(&settings.language),
            starts: Vec::new(),
            current_line: 0,
        };
        highlighter.starts = highlighter.first();
        highlighter
    }

    fn update(&mut self, new_settings: &Self::Settings) {
        self.syntax = language::find(&new_settings.language);
        self.starts = self.first();
        self.current_line = 0;
    }

    fn change_line(&mut self, line: usize) {
        let line = line.min(self.starts.len().saturating_sub(1));
        self.starts.truncate(line + 1);
        self.current_line = self.current_line.min(line);
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let index = self.current_line;
        self.current_line += 1;

        let Some(mut parser) = self.starts.get(index).cloned() else {
            return vec![(0..line.len(), Token::PLAIN)].into_iter();
        };

        let mut runs = Vec::new();
        let text = format!("{line}\n");
        parser.line(&text, 0, &mut runs);

        if self.starts.len() == index + 1 {
            self.starts.push(parser);
        } else if let Some(start) = self.starts.get_mut(index + 1) {
            *start = parser;
        }

        runs.into_iter()
            .filter_map(|run| {
                let end = run.range.end.min(line.len());
                (run.range.start < end).then_some((run.range.start..end, run.token))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn current_line(&self) -> usize {
        self.current_line
    }
}

/// The format of `token` in `theme`: the color of its role in the syntax
/// colors of the theme, and the JetBrains Mono italic or bold face when
/// the token is italic or bold (see [`font_of`]).
pub fn to_format(token: &Token, theme: &Theme) -> Format<Font> {
    Format {
        color: Some(token.role.color_in(&theme.roles().syntax)),
        font: font_of(token),
    }
}

#[cfg(test)]
mod tests {
    use iced::advanced::text::highlighter::Highlighter as _;

    use super::{Highlighter, Settings, highlight, highlight_cached};
    use crate::role::{Role, Token};

    fn role_of(code: &str, language: &str, piece: &str) -> Option<Role> {
        let start = code.find(piece)?;
        highlight(code, language)
            .into_iter()
            .find(|run| run.range.contains(&start))
            .map(|run| run.token.role)
    }

    #[test]
    fn runs_cover_the_whole_text_in_order() {
        let code = "fn main() {\n    // hi\n    let x = \"a\\n\";\n}\n";
        let runs = highlight(code, "rust");
        let mut end = 0;

        for run in &runs {
            assert_eq!(run.range.start, end);
            end = run.range.end;
        }

        assert_eq!(end, code.len());
    }

    #[test]
    fn rust_tokens_take_their_roles() {
        let code = "// note\nfn main() { let s = \"hi\\n\"; let n = 42; }\n";

        assert_eq!(role_of(code, "rust", "// note"), Some(Role::Comment));
        assert_eq!(role_of(code, "rust", "main"), Some(Role::Function));
        assert_eq!(role_of(code, "rust", "let"), Some(Role::Keyword));
        assert_eq!(role_of(code, "rust", "hi"), Some(Role::String));
        assert_eq!(role_of(code, "rust", "\\n"), Some(Role::Constant));
        assert_eq!(role_of(code, "rust", "42"), Some(Role::Number));
    }

    #[test]
    fn unknown_languages_are_one_plain_run() {
        let runs = highlight("x = 1", "cobol");
        assert_eq!(runs.len(), 1);
        assert_eq!(runs[0].token, Token::PLAIN);
        assert!(highlight("", "plaintext").is_empty());
    }

    #[test]
    fn the_cache_shares_the_runs_of_the_same_text() {
        let a = highlight_cached("let a = 1;", "rust");
        let b = highlight_cached("let a = 1;", "rust");
        assert!(std::sync::Arc::ptr_eq(&a, &b));
        let c = highlight_cached("let a = 1;", "js");
        assert!(!std::sync::Arc::ptr_eq(&a, &c));
    }

    #[test]
    fn the_editor_highlighter_resumes_from_the_changed_line() {
        let mut highlighter = Highlighter::new(&Settings::new("rust"));
        let lines = ["/* open", "still comment */", "let x = 1;"];

        let roles: Vec<Vec<Role>> = lines
            .iter()
            .map(|line| {
                highlighter
                    .highlight_line(line)
                    .map(|(_, t)| t.role)
                    .collect()
            })
            .collect();
        assert_eq!(roles[1], vec![Role::Comment]);
        assert!(roles[2].contains(&Role::Keyword));
        assert_eq!(highlighter.current_line(), 3);

        highlighter.change_line(1);
        assert_eq!(highlighter.current_line(), 1);
        let again: Vec<Role> = highlighter
            .highlight_line("still comment */")
            .map(|(_, t)| t.role)
            .collect();
        assert_eq!(again, roles[1]);

        let ranges: Vec<_> = highlighter
            .highlight_line("let x = 1;")
            .map(|(r, _)| r)
            .collect();
        assert_eq!(ranges.last().map(|r| r.end), Some("let x = 1;".len()));
    }
}
