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

//! Text with the matches of a search query emphasized, such as search
//! results that show why each item matched.
//!
//! Each word of the query, and of the extra queries, is matched on its
//! own, ignoring case and accents ("acao" matches "Ação"); overlapping and
//! touching matches merge. [`Options`] change how terms match. Matches are
//! drawn bold in the text-safe primary tone by default.
//!
//! The text has no line limit with an ellipsis: Iced text cannot elide.
//! [`HighlightedText::max_lines`] clips the text after a number of lines
//! instead.
//!
//! ```no_run
//! use oito2_iced::widget::highlighted_text;
//!
//! # fn view<'a, Message: 'a>(query: &str) -> oito2_iced::Element<'a, Message> {
//! highlighted_text::highlighted_text("Show hidden files", query).into()
//! # }
//! ```

use std::borrow::Cow;
use std::ops::Range;

use iced::font::Weight;
use iced::widget::text::{self, IntoFragment, LineHeight, Span, Wrapping};
use iced::widget::{Stack, container, rich_text};
use iced::{Color, Length};

use crate::Element;
use crate::theme::text::Class;
use crate::tokens::typography::{self, TextStyle};

/// How the terms of the queries match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Options {
    /// Whether each word of a query matches on its own; when false, each
    /// query matches as a whole phrase.
    pub split_words: bool,
    /// Whether matching ignores case and accents; when false, matches must
    /// have the same case and accents.
    pub ignore_case: bool,
    /// Whether a term matches only whole words, not the start, middle or
    /// end of a longer word.
    pub whole_words: bool,
    /// Whether every occurrence of a term matches; when false, only its
    /// first occurrence.
    pub match_all: bool,
}

impl Default for Options {
    /// Splits words, ignores case and accents, matches inside words and
    /// matches every occurrence.
    fn default() -> Self {
        Self {
            split_words: true,
            ignore_case: true,
            whole_words: false,
            match_all: true,
        }
    }
}

/// Returns the byte ranges of `text` that match `query` and `queries`,
/// sorted and merged where they overlap or touch.
///
/// Each range starts and ends on a character boundary of `text`; a match
/// of decomposed text covers the combining accents inside it.
pub fn matches(text: &str, query: &str, queries: &[&str], options: Options) -> Vec<Range<usize>> {
    let (haystack, starts) = if options.ignore_case {
        fold_with_offsets(text)
    } else {
        let mut starts: Vec<usize> = text.char_indices().map(|(index, _)| index).collect();
        starts.push(text.len());
        (text.chars().collect(), starts)
    };

    let mut terms: Vec<Vec<char>> = Vec::new();

    for raw in std::iter::once(query).chain(queries.iter().copied()) {
        let term: String = if options.ignore_case {
            fold_with_offsets(raw).0.into_iter().collect()
        } else {
            raw.to_owned()
        };

        if options.split_words {
            terms.extend(term.split_whitespace().map(|word| word.chars().collect()));
        } else {
            terms.push(term.trim().chars().collect());
        }
    }

    let is_word_char = |index: Option<usize>| {
        index
            .and_then(|index| haystack.get(index))
            .is_some_and(|c| c.is_alphanumeric() || *c == '_')
    };

    let mut ranges: Vec<Range<usize>> = Vec::new();

    for term in terms.iter().filter(|term| !term.is_empty()) {
        if term.len() > haystack.len() {
            continue;
        }

        for start in 0..=haystack.len() - term.len() {
            let end = start + term.len();

            if haystack[start..end] != term[..] {
                continue;
            }

            if options.whole_words
                && (is_word_char(start.checked_sub(1)) || is_word_char(Some(end)))
            {
                continue;
            }

            if starts[start] < starts[end] {
                ranges.push(starts[start]..starts[end]);
            }

            if !options.match_all {
                break;
            }
        }
    }

    ranges.sort_by_key(|range| range.start);

    let mut merged: Vec<Range<usize>> = Vec::new();

    for range in ranges {
        match merged.last_mut() {
            Some(last) if range.start <= last.end => last.end = last.end.max(range.end),
            _ => merged.push(range),
        }
    }

    merged
}

/// Folds `text` to lower case without accents and returns its characters
/// with the byte index in `text` of the character each one came from, plus
/// a final entry holding `text.len()`.
///
/// Lowercases every character, drops combining diacritical marks
/// (U+0300 to U+036F), replaces accented Latin letters with their plain
/// letter and spells out `ß`, `æ` and `œ` as `ss`, `ae` and `oe`.
fn fold_with_offsets(text: &str) -> (Vec<char>, Vec<usize>) {
    let mut folded = Vec::with_capacity(text.len());
    let mut starts = Vec::with_capacity(text.len() + 1);

    for (index, c) in text.char_indices() {
        for lower in c.to_lowercase() {
            if ('\u{0300}'..='\u{036F}').contains(&lower) {
                continue;
            }

            for plain in fold_char(lower).chars() {
                folded.push(plain);
                starts.push(index);
            }
        }
    }

    starts.push(text.len());

    (folded, starts)
}

/// The plain spelling of a lowercase Latin letter with an accent, or the
/// letter itself.
fn fold_char(c: char) -> Cow<'static, str> {
    let plain = match c {
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' => "a",
        'ç' | 'ć' | 'č' => "c",
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ę' | 'ě' => "e",
        'ì' | 'í' | 'î' | 'ï' | 'ī' | 'ı' => "i",
        'ñ' | 'ń' | 'ň' => "n",
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' => "o",
        'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ů' => "u",
        'ý' | 'ÿ' => "y",
        'ś' | 'š' => "s",
        'ź' | 'ż' | 'ž' => "z",
        'ł' => "l",
        'ř' => "r",
        'ť' => "t",
        'ď' => "d",
        'ğ' => "g",
        'ß' => "ss",
        'æ' => "ae",
        'œ' => "oe",
        other => return Cow::Owned(other.to_string()),
    };

    Cow::Borrowed(plain)
}

/// Text showing `text` with the matches of the words of `query` drawn
/// bold in the text-safe primary tone, in the body medium style.
pub fn highlighted_text<'a>(text: impl IntoFragment<'a>, query: &str) -> HighlightedText<'a> {
    HighlightedText {
        text: text.into_fragment(),
        query: query.to_owned(),
        queries: Vec::new(),
        options: Options::default(),
        style: typography::BODY_MEDIUM,
        class: Class::Default,
        highlight_color: None,
        highlight_weight: Weight::Bold,
        width: Length::Shrink,
        align_x: text::Alignment::Default,
        wrapping: Wrapping::default(),
        max_lines: None,
    }
}

/// Text with highlighted matches. See [`highlighted_text`].
pub struct HighlightedText<'a> {
    text: Cow<'a, str>,
    query: String,
    queries: Vec<String>,
    options: Options,
    style: TextStyle,
    class: Class<'a>,
    highlight_color: Option<Color>,
    highlight_weight: Weight,
    width: Length,
    align_x: text::Alignment,
    wrapping: Wrapping,
    max_lines: Option<u16>,
}

impl<'a> HighlightedText<'a> {
    /// Highlights the words of `queries` along with the query.
    #[must_use]
    pub fn queries<S: Into<String>>(mut self, queries: impl IntoIterator<Item = S>) -> Self {
        self.queries = queries.into_iter().map(Into::into).collect();
        self
    }

    /// Sets every matching option at once.
    #[must_use]
    pub fn options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }

    /// Sets whether each word of a query matches on its own (true by
    /// default); when false, each query matches as a whole phrase.
    #[must_use]
    pub fn split_words(mut self, split_words: bool) -> Self {
        self.options.split_words = split_words;
        self
    }

    /// Sets whether matching ignores case and accents (true by default).
    #[must_use]
    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.options.ignore_case = ignore_case;
        self
    }

    /// Sets whether terms match only whole words (false by default).
    #[must_use]
    pub fn whole_words(mut self, whole_words: bool) -> Self {
        self.options.whole_words = whole_words;
        self
    }

    /// Sets whether every occurrence of a term is highlighted (true by
    /// default); when false, only its first occurrence.
    #[must_use]
    pub fn match_all(mut self, match_all: bool) -> Self {
        self.options.match_all = match_all;
        self
    }

    /// Sets the style of the type scale (body medium by default).
    #[must_use]
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the color class of the text outside the matches (the color of
    /// the parent by default).
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Draws the matches in `color` instead of the text-safe primary tone.
    #[must_use]
    pub fn highlight_color(mut self, color: Color) -> Self {
        self.highlight_color = Some(color);
        self
    }

    /// Sets the weight of the matches (bold by default).
    #[must_use]
    pub fn highlight_weight(mut self, weight: Weight) -> Self {
        self.highlight_weight = weight;
        self
    }

    /// Sets the width (shrinks to the text by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the horizontal alignment of the lines.
    #[must_use]
    pub fn align_x(mut self, alignment: impl Into<text::Alignment>) -> Self {
        self.align_x = alignment.into();
        self
    }

    /// Sets how lines wrap (at words by default).
    #[must_use]
    pub fn wrapping(mut self, wrapping: Wrapping) -> Self {
        self.wrapping = wrapping;
        self
    }

    /// Clips the text after `lines` lines.
    #[must_use]
    pub fn max_lines(mut self, lines: u16) -> Self {
        self.max_lines = Some(lines);
        self
    }

    /// Returns `range` of the text as a fragment that borrows the text when
    /// the text is borrowed.
    fn slice(&self, range: Range<usize>) -> Cow<'a, str> {
        match &self.text {
            Cow::Borrowed(text) => Cow::Borrowed(&text[range]),
            Cow::Owned(text) => Cow::Owned(text[range].to_owned()),
        }
    }

    /// Splits the text into the pieces between and inside the matches, each
    /// with whether it is a match.
    fn pieces(&self) -> Vec<(Cow<'a, str>, bool)> {
        let queries: Vec<&str> = self.queries.iter().map(String::as_str).collect();
        let mut pieces = Vec::new();
        let mut position = 0;

        for range in matches(&self.text, &self.query, &queries, self.options) {
            if range.start > position {
                pieces.push((self.slice(position..range.start), false));
            }

            position = range.end;
            pieces.push((self.slice(range), true));
        }

        if position < self.text.len() {
            pieces.push((self.slice(position..self.text.len()), false));
        }

        join_inside_words(pieces)
    }
}

/// The word joiner: an invisible character that forbids a line break.
const WORD_JOINER: char = '\u{2060}';

/// Starts with a word joiner every piece that begins inside a word, so the
/// text shaper, which shapes a word of ASCII letters in the font of its
/// first letter, shapes each piece of the word in its own weight.
fn join_inside_words(pieces: Vec<(Cow<'_, str>, bool)>) -> Vec<(Cow<'_, str>, bool)> {
    let mut previous_end: Option<char> = None;

    pieces
        .into_iter()
        .map(|(piece, matched)| {
            let inside_word = previous_end.is_some_and(|end| !end.is_whitespace())
                && piece
                    .chars()
                    .next()
                    .is_some_and(|start| !start.is_whitespace());

            previous_end = piece.chars().next_back();

            if inside_word {
                (Cow::Owned(format!("{WORD_JOINER}{piece}")), matched)
            } else {
                (piece, matched)
            }
        })
        .collect()
}

impl<'a, Message: 'a> From<HighlightedText<'a>> for Element<'a, Message> {
    fn from(highlighted: HighlightedText<'a>) -> Self {
        let pieces = highlighted.pieces();
        let regular = crate::font::inter(highlighted.style.weight);
        let emphasis = crate::font::inter(highlighted.highlight_weight);
        let has_matches = pieces.iter().any(|(_, matched)| *matched);
        let style = highlighted.style;
        let width = highlighted.width;
        let align_x = highlighted.align_x;
        let wrapping = highlighted.wrapping;

        // Builds one layer: `color` gives the color of each piece from
        // whether it is a match; `None` inherits the color of the layer.
        let layer = |color: &dyn Fn(bool) -> Option<Color>| {
            let spans: Vec<Span<'a, ()>> = pieces
                .iter()
                .map(|(piece, matched)| {
                    Span::new(piece.clone())
                        .font(if *matched { emphasis } else { regular })
                        .color_maybe(color(*matched))
                })
                .collect();

            rich_text::<(), Message, crate::Theme, iced::Renderer>(spans)
                .size(style.size)
                .line_height(LineHeight::Absolute(style.line_height.into()))
                .font(regular)
                .width(width)
                .align_x(align_x)
                .wrapping(wrapping)
        };

        let content: Element<'a, Message> = match (has_matches, highlighted.highlight_color) {
            (false, _) => layer(&|_| None).class(highlighted.class).into(),
            (true, Some(highlight)) => layer(&|matched| matched.then_some(highlight))
                .class(highlighted.class)
                .into(),
            // The highlight tone comes from the theme, known only while
            // drawing, so the matches are drawn by a second, identical
            // layer in the primary class; each layer hides the pieces of
            // the other with transparent text.
            (true, None) => Stack::new()
                .push(
                    layer(&|matched| matched.then_some(Color::TRANSPARENT))
                        .class(highlighted.class),
                )
                .push(
                    layer(&|matched| (!matched).then_some(Color::TRANSPARENT))
                        .class(Class::Primary),
                )
                .width(width)
                .into(),
        };

        match highlighted.max_lines {
            Some(lines) => container(content)
                .max_height(f32::from(lines) * style.line_height)
                .clip(true)
                .width(width)
                .into(),
            None => content,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::{Options, join_inside_words, matches};

    #[test]
    fn pieces_inside_a_word_start_with_a_word_joiner() {
        let pieces = vec![
            (Cow::Borrowed("Show "), false),
            (Cow::Borrowed("hid"), true),
            (Cow::Borrowed("den files"), false),
        ];

        let joined: Vec<String> = join_inside_words(pieces)
            .into_iter()
            .map(|(piece, _)| piece.into_owned())
            .collect();

        assert_eq!(joined, vec!["Show ", "hid", "\u{2060}den files"]);
    }

    fn find(text: &str, query: &str) -> Vec<std::ops::Range<usize>> {
        matches(text, query, &[], Options::default())
    }

    #[test]
    fn matches_every_word_ignoring_case_and_accents() {
        let text = "Configurações de ação";
        let ranges = find(text, "acao CONFIG");

        assert_eq!(ranges, vec![0..6, 19..25]);
        assert_eq!(&text[19..25], "ação");
    }

    #[test]
    fn matches_decomposed_accents_sharp_s_and_dotted_capitals_in_place() {
        let decomposed = "ac\u{0327}a\u{0303}o";
        assert_eq!(find(decomposed, "acao"), vec![0..decomposed.len()]);

        assert_eq!(find("Straße", "strasse"), vec![0.."Straße".len()]);

        let istanbul = "İstanbul";
        let ranges = find(istanbul, "stan");
        assert_eq!(ranges, vec![2..6]);
        assert_eq!(&istanbul[2..6], "stan");
    }

    #[test]
    fn matches_several_queries_phrases_case_and_whole_words() {
        let text = "Ark UI builds UI kits; the ark sails";

        assert_eq!(
            matches(text, "ark", &["kits"], Options::default()),
            vec![0..3, 17..21, 27..30]
        );
        assert_eq!(
            matches(
                text,
                "ark",
                &[],
                Options {
                    ignore_case: false,
                    ..Options::default()
                }
            ),
            vec![27..30]
        );
        assert_eq!(
            matches(
                text,
                "ark",
                &[],
                Options {
                    match_all: false,
                    ..Options::default()
                }
            ),
            vec![0..3]
        );

        let whole = Options {
            whole_words: true,
            ..Options::default()
        };
        assert_eq!(matches(text, "ui", &[], whole), vec![4..6, 14..16]);
        assert!(matches(text, "build", &[], whole).is_empty());

        assert_eq!(
            matches(
                text,
                "ui kits",
                &[],
                Options {
                    split_words: false,
                    ..Options::default()
                }
            ),
            vec![14..21]
        );
    }

    #[test]
    fn merges_overlapping_matches() {
        assert_eq!(find("banana", "ana nan"), vec![1..6]);
        assert!(find("text", "").is_empty());
        assert!(find("text", "   ").is_empty());
    }

    #[test]
    fn merges_touching_matches_and_keeps_separate_ones_apart() {
        assert_eq!(find("abcdef", "ab cd"), vec![0..4]);
        assert_eq!(find("ab cd", "ab cd"), vec![0..2, 3..5]);
    }

    #[test]
    fn a_term_longer_than_the_text_matches_nothing() {
        assert!(find("ab", "abc").is_empty());
        assert!(find("", "a").is_empty());
    }

    #[test]
    fn case_sensitive_matching_keeps_accents() {
        let exact = Options {
            ignore_case: false,
            ..Options::default()
        };

        assert!(matches("Ação", "Acao", &[], exact).is_empty());
        assert_eq!(matches("Ação", "ção", &[], exact), vec![1..6]);
    }

    #[test]
    fn whole_words_treat_letters_of_any_script_as_word_characters() {
        let whole = Options {
            whole_words: true,
            ..Options::default()
        };

        assert!(matches("açaí", "a", &[], whole).is_empty());
        assert_eq!(matches("o_o o", "o", &[], whole), vec![4..5]);
    }

    #[test]
    fn a_match_inside_an_expanded_letter_covers_the_whole_letter() {
        let text = "Straße";
        let ranges = find(text, "e");

        assert_eq!(ranges, vec![6..7]);
        assert_eq!(find(text, "sse"), vec![4..7]);
    }
}
