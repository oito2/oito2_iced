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

//! Case and accent folding for searching and comparing text.

/// Returns the plain ASCII spelling of an accented or special lowercase
/// Latin letter, or `None` when the letter has no folding.
fn plain_letter(c: char) -> Option<&'static str> {
    Some(match c {
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
        _ => return None,
    })
}

/// Text folded by [`fold_text_with_offsets`], with the position in the
/// original text of every folded byte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldedText {
    /// The folded text.
    pub folded: String,
    /// For each byte of [`folded`](Self::folded), the byte offset in the
    /// original text of the character that produced it, followed by a final
    /// entry holding the length of the original text.
    pub starts: Vec<usize>,
}

/// Returns `text` in lower case without the accents of Latin letters, with
/// the combining accents of decomposed text removed, and with `ß`, `æ` and
/// `œ` spelled out.
///
/// It is used to compare and search text ignoring case and accents, such as
/// "Ação" matching "acao". [`fold_text_with_offsets`] also maps the result
/// back to `text`.
#[must_use]
pub fn fold_text(text: &str) -> String {
    fold_text_with_offsets(text).folded
}

/// Folds `text` like [`fold_text`] and returns the folded string together
/// with the offsets of the original character behind each folded byte.
///
/// A folded byte range `start..end` covers `starts[start]..starts[end]` of
/// `text`, including the combining accents that follow it. Offsets are byte
/// offsets in UTF-8.
#[must_use]
pub fn fold_text_with_offsets(text: &str) -> FoldedText {
    let mut folded = String::with_capacity(text.len());
    let mut starts = Vec::with_capacity(text.len() + 1);
    let mut buffer = [0_u8; 4];
    for (offset, original) in text.char_indices() {
        for lower in original.to_lowercase() {
            if ('\u{0300}'..='\u{036F}').contains(&lower) {
                continue;
            }
            let piece = plain_letter(lower).unwrap_or_else(|| lower.encode_utf8(&mut buffer));
            folded.push_str(piece);
            starts.extend(std::iter::repeat_n(offset, piece.len()));
        }
    }
    starts.push(text.len());
    FoldedText { folded, starts }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_case_and_accents() {
        assert_eq!(fold_text("Ação"), "acao");
        assert_eq!(fold_text("PÊRA"), "pera");
        assert_eq!(fold_text("Straße æon œuvre"), "strasse aeon oeuvre");
        assert_eq!(fold_text("Łódź ı"), "lodz i");
    }

    #[test]
    fn removes_combining_marks() {
        assert_eq!(fold_text("e\u{0301}a\u{0303}"), "ea");
    }

    #[test]
    fn leaves_other_text_alone() {
        assert_eq!(fold_text("abc 123 -_"), "abc 123 -_");
        assert_eq!(fold_text("日本"), "日本");
        assert_eq!(fold_text(""), "");
    }

    #[test]
    fn maps_folded_bytes_back_to_the_original() {
        let result = fold_text_with_offsets("Ação");
        assert_eq!(result.folded, "acao");
        // 'A' at 0, 'ç' at 1 (two bytes), 'ã' at 3 (two bytes), 'o' at 5.
        assert_eq!(result.starts, vec![0, 1, 3, 5, 6]);
    }

    #[test]
    fn multi_byte_folds_share_one_start() {
        let result = fold_text_with_offsets("aßb");
        assert_eq!(result.folded, "assb");
        assert_eq!(result.starts, vec![0, 1, 1, 3, 4]);
    }

    #[test]
    fn combining_marks_extend_the_previous_range() {
        let result = fold_text_with_offsets("e\u{0301}x");
        assert_eq!(result.folded, "ex");
        assert_eq!(result.starts, vec![0, 3, 4]);
    }
}
