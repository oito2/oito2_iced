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

//! The building blocks of the documentation pages: the page frame,
//! headings, prose with inline code, code blocks, notes, tables, example
//! panels that show the source of their example, Markdown documents and
//! the API reference.
//!
//! Prose is a small Markdown subset: `#` to `####` headings, paragraphs,
//! `-`, `*` and `1.` lists, fenced code blocks, `|` tables and `>` quotes;
//! inline `code`, `**bold**` and `[text](link)`, whose text is shown and
//! whose target is dropped.

use std::cell::RefCell;
use std::collections::HashMap;

use iced::font::Weight;
use iced::widget::text::Span;
use iced::widget::{Column, column, container, rich_text, row, span, text};
use iced::{Element, Font, Length, Padding};

use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::info::{InfoType, info_box};
use oito2_iced::widget::styled_text;
use oito2_iced::{Theme, font};
use oito2_iced_syntax::code_block::code_block;

use crate::api;

/// The widest a documentation page grows.
pub const CONTENT_WIDTH: f32 = 820.0;

/// What every page view receives from the application.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The strings of the chosen language.
    pub l10n: Localizations,
    /// The reading direction of the chosen language.
    pub direction: ReadingDirection,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            l10n: Localizations::en(),
            direction: ReadingDirection::LeftToRight,
        }
    }
}

/// A documentation page: its title, a one-line description, then its
/// blocks, no wider than [`CONTENT_WIDTH`].
pub fn page<'a, Message: 'a>(
    title: &'a str,
    description: &'a str,
    blocks: impl IntoIterator<Item = Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    container(
        column![
            styled_text(title, typography::HEADLINE_MEDIUM),
            styled_text(description, typography::BODY_LARGE).class(TextClass::Variant),
        ]
        .spacing(spacing::SM)
        .push(column(blocks).spacing(spacing::LG).padding(Padding {
            top: spacing::MD,
            ..Padding::ZERO
        })),
    )
    .max_width(CONTENT_WIDTH)
    .width(Length::Fill)
    .into()
}

/// A section heading.
pub fn heading<'a, Message: 'a>(title: &'a str) -> Element<'a, Message, Theme> {
    heading_at(title, 2)
}

/// A subsection heading.
pub fn subheading<'a, Message: 'a>(title: &'a str) -> Element<'a, Message, Theme> {
    heading_at(title, 3)
}

/// A heading of Markdown `level` (2 for `##`).
fn heading_at<'a, Message: 'a>(title: &'a str, level: usize) -> Element<'a, Message, Theme> {
    let style = match level {
        0..=2 => typography::HEADLINE_SMALL,
        3 => typography::TITLE_LARGE,
        _ => typography::TITLE_MEDIUM,
    };

    container(styled_text(title, style))
        .padding(Padding {
            top: spacing::SM,
            ..Padding::ZERO
        })
        .into()
}

/// A paragraph of prose with inline Markdown.
pub fn prose<'a, Message: 'a>(markdown: &'a str) -> Element<'a, Message, Theme> {
    paragraph(markdown, Font::default(), typography::BODY_MEDIUM.size)
}

/// Inline Markdown in `font` at `size`.
fn paragraph<'a, Message: 'a>(
    markdown: &'a str,
    font: Font,
    size: f32,
) -> Element<'a, Message, Theme> {
    let spans = inline(markdown, font);

    rich_text::<(), Message, Theme, iced::Renderer>(spans)
        .size(size)
        .line_height(1.5)
        .width(Length::Fill)
        .into()
}

/// The spans of inline Markdown: `code` in the monospace font, `**bold**`
/// in the bold weight of `font`, and the text of `[text](link)`.
fn inline(markdown: &str, font: Font) -> Vec<Span<'static, ()>> {
    let mut spans = Vec::new();
    let mut plain = String::new();
    let mut bold = false;
    let mut rest = markdown;

    let flush = |plain: &mut String, spans: &mut Vec<Span<'static, ()>>, bold: bool| {
        if !plain.is_empty() {
            let face = if bold {
                Font {
                    weight: Weight::Bold,
                    ..font
                }
            } else {
                font
            };
            spans.push(span(std::mem::take(plain)).font(face));
        }
    };

    while let Some(character) = rest.chars().next() {
        if character == '`'
            && let Some(end) = rest[1..].find('`')
        {
            flush(&mut plain, &mut spans, bold);
            spans
                .push(span(rest[1..=end].to_owned()).font(oito2_iced_syntax::font::JETBRAINS_MONO));
            rest = &rest[end + 2..];
        } else if rest.starts_with("**") {
            flush(&mut plain, &mut spans, bold);
            bold = !bold;
            rest = &rest[2..];
        } else if character == '['
            && let Some(close) = rest.find("](")
            && let Some(end) = rest[close..].find(')')
        {
            let label = &rest[1..close];
            flush(&mut plain, &mut spans, bold);
            spans.extend(inline(label, font));
            rest = &rest[close + end + 1..];
        } else if character == '['
            && rest[1..].starts_with('`')
            && let Some(end) = rest.find("`]")
        {
            flush(&mut plain, &mut spans, bold);
            spans.push(span(rest[2..end].to_owned()).font(oito2_iced_syntax::font::JETBRAINS_MONO));
            rest = &rest[end + 2..];
        } else {
            plain.push(character);
            rest = &rest[character.len_utf8()..];
        }
    }

    flush(&mut plain, &mut spans, bold);
    spans
}

/// A block of Rust code with a copy button.
pub fn code<'a, Message: Clone + 'a>(ctx: &Context, code: &'a str) -> Element<'a, Message, Theme> {
    code_in(ctx, code, "rust")
}

/// A block of code in `language`, with a copy button.
pub fn code_in<'a, Message: Clone + 'a>(
    ctx: &Context,
    code: &'a str,
    language: &str,
) -> Element<'a, Message, Theme> {
    code_block(&ctx.l10n, code)
        .language(language)
        .width(Length::Fill)
        .into()
}

/// A note set apart from the prose.
pub fn note<'a, Message: 'a>(markdown: &'a str) -> Element<'a, Message, Theme> {
    info_box(InfoType::Information)
        .content(prose(markdown))
        .into()
}

/// A table with a `header` row and body `rows` of inline Markdown cells.
pub fn table<'a, Message: 'a>(
    header: &[&'a str],
    rows: &[Vec<&'a str>],
) -> Element<'a, Message, Theme> {
    let cell = |markdown: &'a str, head: bool| -> Element<'a, Message, Theme> {
        let content: Element<'a, Message, Theme> = if head {
            styled_text(markdown, typography::LABEL_LARGE).into()
        } else {
            prose(markdown)
        };

        container(content)
            .padding([spacing::SM, spacing::MD])
            .width(Length::FillPortion(1))
            .into()
    };

    let mut lines = Column::new().push(row(header.iter().map(|title| cell(title, true))));
    for cells in rows {
        lines = lines.push(row(cells.iter().map(|markdown| cell(markdown, false))));
    }

    border_container(lines)
        .radius(radius::MEDIUM)
        .width(Length::Fill)
        .into()
}

/// An example panel: `example` centered over the source of the
/// `// #region <name>` blocks of `source`, the text of the page's file.
pub fn example<'a, Message: Clone + 'a>(
    ctx: &Context,
    source: &'static str,
    regions: &[&str],
    example: impl Into<Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    let mut panel = column![
        container(example)
            .padding(spacing::XL)
            .center_x(Length::Fill)
    ]
    .spacing(spacing::MD)
    .padding(Padding {
        bottom: spacing::MD,
        ..Padding::ZERO
    });

    if let Some(code) = regions_of(source, regions) {
        panel = panel.push(
            container(
                code_block(&ctx.l10n, code)
                    .language("rust")
                    .width(Length::Fill),
            )
            .padding([0.0, spacing::MD]),
        );
    }

    border_container(panel).width(Length::Fill).into()
}

/// The source of the `regions` of `source`, dedented and separated by a
/// blank line; `None` when no region is found. The result is kept for
/// the life of the process, so views can borrow it.
pub fn regions_of(source: &'static str, regions: &[&str]) -> Option<&'static str> {
    thread_local! {
        static CACHE: RefCell<HashMap<(usize, String), Option<&'static str>>> =
            RefCell::new(HashMap::new());
    }

    let key = (source.as_ptr() as usize, regions.join("\u{0}"));

    CACHE.with(|cache| {
        if let Some(found) = cache.borrow().get(&key) {
            return *found;
        }

        let found: Vec<String> = regions
            .iter()
            .filter_map(|name| region(source, name))
            .collect();
        let joined = (!found.is_empty()).then(|| &*Box::leak(found.join("\n\n").into_boxed_str()));

        cache.borrow_mut().insert(key, joined);
        joined
    })
}

/// The lines between `// #region name` and the next `// #endregion` of
/// `source`, with their common indentation removed.
fn region(source: &str, name: &str) -> Option<String> {
    let marker = format!("// #region {name}");
    let mut lines = source.lines().skip_while(|line| line.trim() != marker);
    lines.next()?;

    let body: Vec<&str> = lines
        .take_while(|line| line.trim() != "// #endregion")
        .collect();
    let indent = body
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or_default();

    Some(
        body.iter()
            .map(|line| line.get(indent..).unwrap_or_default().trim_end())
            .collect::<Vec<_>>()
            .join("\n")
            .trim_matches('\n')
            .trim_end_matches(',')
            .to_owned(),
    )
}

/// The blocks of a Markdown document: `##` and deeper headings, prose,
/// lists, code, tables and notes. A leading `#` title is skipped, since
/// the page shows its own.
pub fn markdown<'a, Message: Clone + 'a>(
    ctx: &Context,
    document: &'a str,
) -> Vec<Element<'a, Message, Theme>> {
    let mut blocks = Vec::new();
    let mut lines = document.lines().peekable();
    let mut first = true;

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(level) = heading_level(trimmed) {
            let title: &'static str = Box::leak(
                trimmed[level..]
                    .trim()
                    .replace(['[', ']'], "")
                    .into_boxed_str(),
            );
            if !(first && level == 1) {
                blocks.push(heading_at(title, level));
            }
        } else if let Some(language) = trimmed.strip_prefix("```") {
            let start = offset_of(document, lines.peek().copied().unwrap_or(""));
            let mut end = start;
            for line in lines.by_ref() {
                if line.trim_start().starts_with("```") {
                    break;
                }
                end = offset_of(document, line) + line.len();
            }
            let code = document.get(start..end).unwrap_or_default();
            let language = if language.is_empty() {
                "plaintext"
            } else {
                language
            };
            blocks.push(code_in(ctx, code, language));
        } else if trimmed.starts_with('|') {
            let mut table_lines = vec![trimmed];
            while let Some(next) = lines.peek()
                && next.trim().starts_with('|')
            {
                table_lines.push(lines.next().unwrap_or_default().trim());
            }
            blocks.push(markdown_table(&table_lines));
        } else if let Some(quote) = trimmed.strip_prefix('>') {
            let start = offset_of(document, quote.trim_start());
            let mut end = start + quote.trim_start().len();
            while let Some(next) = lines.peek()
                && next.trim().starts_with('>')
            {
                let next = lines.next().unwrap_or_default();
                end = offset_of(document, next) + next.len();
            }
            let text = document.get(start..end).unwrap_or_default();
            blocks.push(note_lines(text));
        } else if list_item(trimmed).is_some() {
            let mut items = vec![trimmed];
            while let Some(next) = lines.peek()
                && (list_item(next.trim()).is_some()
                    || (!next.trim().is_empty() && next.starts_with("  ")))
            {
                let next = lines.next().unwrap_or_default();
                if list_item(next.trim()).is_some() {
                    items.push(next.trim());
                } else if let Some(last) = items.last_mut() {
                    let start = offset_of(document, last);
                    let end = offset_of(document, next) + next.len();
                    *last = document.get(start..end).unwrap_or(last);
                }
            }
            blocks.push(list(&items));
        } else {
            let start = offset_of(document, trimmed);
            let mut end = start + trimmed.len();
            while let Some(next) = lines.peek()
                && !next.trim().is_empty()
                && heading_level(next.trim()).is_none()
                && !next.trim().starts_with("```")
                && !next.trim().starts_with('|')
                && list_item(next.trim()).is_none()
            {
                let next = lines.next().unwrap_or_default();
                end = offset_of(document, next) + next.len();
            }
            blocks.push(prose_lines(document.get(start..end).unwrap_or_default()));
        }

        first = false;
    }

    blocks
}

/// The byte offset of `part`, a slice of `document`, inside it.
fn offset_of(document: &str, part: &str) -> usize {
    (part.as_ptr() as usize)
        .saturating_sub(document.as_ptr() as usize)
        .min(document.len())
}

/// The number of `#` of a heading line, 1 to 4.
fn heading_level(line: &str) -> Option<usize> {
    let level = line.chars().take_while(|c| *c == '#').count();
    ((1..=4).contains(&level) && line[level..].starts_with(' ')).then_some(level)
}

/// The text of a list item line and whether the list is numbered.
fn list_item(line: &str) -> Option<(&str, bool)> {
    if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
        return Some((item, false));
    }

    let digits = line.chars().take_while(char::is_ascii_digit).count();
    (digits > 0 && line[digits..].starts_with(". ")).then(|| (&line[digits + 2..], true))
}

/// Prose whose lines are joined by spaces.
fn prose_lines<'a, Message: 'a>(text: &'a str) -> Element<'a, Message, Theme> {
    let joined: &'static str = Box::leak(
        text.lines()
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(" ")
            .into_boxed_str(),
    );
    prose(joined)
}

/// A note whose `>` lines are joined by spaces.
fn note_lines<'a, Message: 'a>(text: &'a str) -> Element<'a, Message, Theme> {
    let joined: &'static str = Box::leak(
        text.lines()
            .map(|line| line.trim().trim_start_matches('>').trim())
            .collect::<Vec<_>>()
            .join(" ")
            .into_boxed_str(),
    );
    note(joined)
}

/// A bulleted or numbered list of item lines.
fn list<'a, Message: 'a>(items: &[&'a str]) -> Element<'a, Message, Theme> {
    column(items.iter().enumerate().map(|(index, line)| {
        let (item, numbered) = list_item(line).unwrap_or((line, false));
        let marker = if numbered {
            format!("{}.", index + 1)
        } else {
            "•".to_owned()
        };

        row![
            text(marker)
                .size(typography::BODY_MEDIUM.size)
                .width(Length::Fixed(20.0)),
            prose_lines(item),
        ]
        .into()
    }))
    .spacing(spacing::XS)
    .into()
}

/// A table from Markdown `|` lines; the separator line is skipped.
fn markdown_table<'a, Message: 'a>(lines: &[&'a str]) -> Element<'a, Message, Theme> {
    let cells = |line: &'a str| -> Vec<&'a str> {
        line.trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect()
    };

    let header = lines.first().map(|line| cells(line)).unwrap_or_default();
    let rows: Vec<Vec<&'a str>> = lines
        .iter()
        .skip(1)
        .filter(|line| !line.chars().all(|c| matches!(c, '|' | '-' | ':' | ' ')))
        .map(|line| cells(line))
        .collect();

    table(&header, &rows)
}

/// The API reference of the items at `paths`, such as
/// `oito2_iced::widget::copy::copy_button`, under an "API reference"
/// heading; paths missing from the generated reference are skipped.
pub fn api<'a, Message: Clone + 'a>(ctx: &Context, paths: &[&str]) -> Element<'a, Message, Theme> {
    let items = paths.iter().filter_map(|path| api::find(path));

    column![heading("API reference")]
        .extend(items.map(|item| api::view(ctx, item)))
        .spacing(spacing::LG)
        .into()
}

/// A label in the label medium style and the variant color.
pub fn caption<'a, Message: 'a>(label: impl text::IntoFragment<'a>) -> Element<'a, Message, Theme> {
    styled_text(label, typography::LABEL_MEDIUM)
        .class(TextClass::Variant)
        .into()
}

/// Text in the monospace font.
pub fn mono<'a, Message: 'a>(content: impl text::IntoFragment<'a>) -> Element<'a, Message, Theme> {
    text(content)
        .font(oito2_iced_syntax::font::JETBRAINS_MONO)
        .size(typography::BODY_MEDIUM.size)
        .into()
}

/// Text in the title small style.
pub fn title<'a, Message: 'a>(content: impl text::IntoFragment<'a>) -> Element<'a, Message, Theme> {
    text(content)
        .font(font::inter(Weight::Semibold))
        .size(typography::TITLE_SMALL.size)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "fn view() {\n    // #region preview\n    let a = 1;\n        let b = 2;\n    // #endregion\n    // #region other\n    let c = 3;\n    // #endregion\n}\n";

    #[test]
    fn regions_are_dedented_and_joined() {
        assert_eq!(
            region(SOURCE, "preview").as_deref(),
            Some("let a = 1;\n    let b = 2;")
        );
        assert_eq!(
            region("// #region x\n    row![a],\n// #endregion", "x").as_deref(),
            Some("row![a]")
        );
        assert_eq!(
            regions_of(SOURCE, &["preview", "other"]),
            Some("let a = 1;\n    let b = 2;\n\nlet c = 3;")
        );
        assert_eq!(regions_of(SOURCE, &["missing"]), None);
    }

    #[test]
    fn inline_markdown_splits_code_bold_and_links() {
        let spans = inline(
            "Use `copy_button` **now**, see [the guide](https://x).",
            Font::default(),
        );
        let texts: Vec<&str> = spans.iter().map(|span| span.text.as_ref()).collect();

        assert_eq!(
            texts,
            [
                "Use ",
                "copy_button",
                " ",
                "now",
                ", see ",
                "the guide",
                "."
            ]
        );
    }

    #[test]
    fn list_items_are_recognized() {
        assert_eq!(list_item("- one"), Some(("one", false)));
        assert_eq!(list_item("12. twelve"), Some(("twelve", true)));
        assert_eq!(list_item("plain"), None);
        assert_eq!(heading_level("## Title"), Some(2));
        assert_eq!(heading_level("#hashtag"), None);
    }
}
