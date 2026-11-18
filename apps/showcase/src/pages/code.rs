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

//! The code page: highlighted code blocks and a highlighted text editor.

use iced::widget::{column, row, text, text_editor};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::segment_group;
use oito2_iced::widget::toggles;
use oito2_iced_syntax::{code_block, font, highlighter, language};

use super::{Gallery, Message, card, page};

/// The sample languages of the code block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sample {
    /// A Rust function.
    Rust,
    /// A Dart function.
    Dart,
    /// A SQL query with a long line.
    Sql,
    /// A YAML document.
    Yaml,
}

impl Sample {
    /// The language name of the sample.
    fn language(self) -> &'static str {
        match self {
            Sample::Rust => "rust",
            Sample::Dart => "dart",
            Sample::Sql => "sql",
            Sample::Yaml => "yaml",
        }
    }

    /// The code of the sample.
    fn code(self) -> &'static str {
        match self {
            Sample::Rust => concat!(
                "/// Greets `name` a few times.\n",
                "fn greet(name: &str, times: usize) -> String {\n",
                "    let text = format!(\"Hello, {name}!\"); // 42\n",
                "    vec![text; times].join(\" \")\n",
                "}",
            ),
            Sample::Dart => concat!(
                "@override\n",
                "String greet(String name, {int times = 2}) {\n",
                "  final text = 'Hello, $name!'; // 42\n",
                "  return List.filled(times, text).join(' ');\n",
                "}",
            ),
            Sample::Sql => concat!(
                "SELECT id, name, email, created_at, updated_at FROM users ",
                "WHERE active = TRUE AND created_at > '2026-01-01' ORDER BY name\n",
                "LIMIT 10;",
            ),
            Sample::Yaml => concat!(
                "name: oito2_iced\n",
                "version: 0.1.0\n",
                "features:\n",
                "  - svg     # on by default\n",
                "  - qr_code\n",
                "enabled: true",
            ),
        }
    }
}

/// The state of the code page.
#[derive(Debug)]
pub struct Code {
    pub(super) sample: Sample,
    pub(super) line_numbers: bool,
    pub(super) copies: u32,
    pub(super) editor: text_editor::Content,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            sample: Sample::Rust,
            line_numbers: true,
            copies: 0,
            editor: text_editor::Content::with_text(
                "fn main() {\n    let answer = 6 * 7;\n    println!(\"The answer is {answer}\");\n}\n",
            ),
        }
    }
}

/// The code page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let code = &gallery.code;

    let samples = segment_group::segment_group(
        [
            segment_group::segment(Sample::Rust, "Rust"),
            segment_group::segment(Sample::Dart, "Dart"),
            segment_group::segment(Sample::Sql, "SQL"),
            segment_group::segment(Sample::Yaml, "YAML"),
        ],
        Some(code.sample),
    )
    .on_change(Message::CodeSample);

    let options = row![
        samples,
        row![
            toggles::switch(code.line_numbers).on_toggle(Message::LineNumbers),
            text("Line numbers"),
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center),
    ]
    .spacing(spacing::LG)
    .align_y(Alignment::Center)
    .wrap();

    let copies = match code.copies {
        0 => "Not copied yet".to_owned(),
        1 => "Copied once".to_owned(),
        n => format!("Copied {n} times"),
    };

    let block = card(
        "Code block",
        column![
            options,
            code_block::code_block(l10n, code.sample.code())
                .language(code.sample.language())
                .line_numbers(code.line_numbers)
                .on_copied(Message::CodeCopied)
                .width(Length::Fill),
            text(copies).class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    );

    let plain = card(
        "Plain text",
        code_block::code_block(l10n, "No language: the text keeps the foreground color.")
            .copy_button(false)
            .width(Length::Fill),
    );

    let editor = card(
        "Highlighted editor",
        column![
            text_editor(&code.editor)
                .font(font::JETBRAINS_MONO)
                .size(code_block::TEXT_SIZE)
                .height(140)
                .on_action(Message::CodeEdit)
                .highlight_with::<highlighter::Highlighter>(
                    highlighter::Settings::new("rust"),
                    highlighter::to_format,
                ),
            text(format!(
                "{} languages: {}",
                language::LANGUAGES.len(),
                language::LANGUAGES.join(", ")
            ))
            .class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    );

    page(
        "Code",
        "Code blocks and a text editor highlighted in the theme's syntax colors.",
        [block, plain, editor],
    )
}
