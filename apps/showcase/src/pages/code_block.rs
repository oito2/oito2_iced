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
//! The CodeBlock page: code highlighted in the syntax colors of the theme,
//! and the highlighter of the Iced text editor.

use iced::widget::{column, container, text, text_editor};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::chips::{BarStyle, choice_chip_bar};
use oito2_iced::widget::controls::switch_button;
use oito2_iced::widget::toggles::switch;
use oito2_iced_syntax::code_block::code_block;
use oito2_iced_syntax::highlighter::{self, Highlighter, Settings};
use oito2_iced_syntax::{font, language};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("code_block.rs");

/// The samples of the languages example: the language and its code.
const SAMPLES: [(&str, &str); 8] = [
    (
        "rust",
        "/// A counter that never goes below zero.\n\
         #[derive(Debug, Default)]\n\
         pub struct Counter(u32);\n\
         \n\
         impl Counter {\n    \
             pub fn decrement(&mut self) {\n        \
                 self.0 = self.0.saturating_sub(1); // stays at 0\n    \
             }\n\
         }",
    ),
    (
        "dart",
        "import 'package:flutter/widgets.dart';\n\
         \n\
         /// A counter that never goes below zero.\n\
         class Counter extends ValueNotifier<int> {\n  \
           Counter([super.value = 0]);\n\
         \n  \
           void decrement() {\n    \
             if (value > 0) value--;\n  \
           }\n\
         }",
    ),
    (
        "python",
        "from dataclasses import dataclass\n\
         \n\
         @dataclass\n\
         class Point:\n    \
             x: float = 0.0\n    \
             y: float = 0.0\n\
         \n    \
             def distance(self, other: \"Point\") -> float:\n        \
                 return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2) ** 0.5\n\
         \n\
         print(Point(3, 4).distance(Point()))  # 5.0",
    ),
    (
        "yaml",
        "name: oito2_iced\n\
         version: 0.1.0\n\
         features:\n  \
           - svg     # on by default\n  \
           - qr_code\n\
         enabled: true\n\
         retries: 3",
    ),
    (
        "bash",
        "#!/usr/bin/env bash\n\
         set -euo pipefail\n\
         \n\
         for file in \"$HOME\"/Pictures/*.png; do\n  \
           echo \"Optimizing ${file##*/}\"\n  \
           optipng -quiet \"$file\" || exit 1\n\
         done",
    ),
    (
        "html",
        "<!doctype html>\n\
         <html lang=\"en\">\n  \
           <body class=\"page\">\n    \
             <a href=\"https://github.com/oito2/oito2_iced\">Showcase</a>\n  \
           </body>\n\
         </html>",
    ),
    (
        "sql",
        "SELECT name, COUNT(*) AS total\n\
         FROM downloads\n\
         WHERE created_at >= '2026-01-01'\n\
         GROUP BY name\n\
         ORDER BY total DESC\n\
         LIMIT 10;",
    ),
    (
        "diff",
        "@@ -1,3 +1,3 @@\n \
         [dependencies]\n\
         -iced = \"0.13\"\n\
         +iced = \"0.14\"",
    ),
];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    sample: usize,
    line_numbers: bool,
    copies: u32,
    editor: text_editor::Content,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sample: 0,
            line_numbers: true,
            copies: 0,
            editor: text_editor::Content::with_text(
                "fn main() {\n    let answer = 6 * 7;\n    println!(\"The answer is {answer}\");\n}\n",
            ),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A sample language was chosen.
    Sample(usize),
    /// The line numbers switch changed.
    LineNumbers(bool),
    /// A code block copied its code.
    Copied,
    /// The highlighted editor was edited.
    Edit(text_editor::Action),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Sample(index) => state.sample = index,
        Message::LineNumbers(on) => state.line_numbers = on,
        Message::Copied => state.copies += 1,
        Message::Edit(action) => state.editor.perform(action),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            code_block(l10n, "fn main() { println!(\"Hello, OITO2!\"); }").language("rust"),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let (language, code) = SAMPLES.get(state.sample).copied().unwrap_or(SAMPLES[0]);
    let selected = SAMPLES.map(|(name, _)| name == language);

    let languages = doc::example(
        ctx,
        SOURCE,
        &["languages"],
        column![
            choice_chip_bar(SAMPLES.map(|(name, _)| name), &selected)
                .style(BarStyle::Wrap)
                .on_select(Message::Sample),
            switch_button(
                switch(state.line_numbers).on_toggle(Message::LineNumbers),
                "Line numbers",
            ),
            // #region languages
            code_block(l10n, code)
                .language(language)
                .line_numbers(state.line_numbers)
                .on_copied(Message::Copied)
                .width(Length::Fill),
            // #endregion
        ]
        .spacing(spacing::MD),
    );

    let copies = match state.copies {
        0 => "Not copied yet.".to_owned(),
        1 => "Copied once.".to_owned(),
        copies => format!("Copied {copies} times."),
    };

    let plain = doc::example(
        ctx,
        SOURCE,
        &["plain"],
        container(
            // #region plain
            code_block(l10n, "Build finished in 12.4s\n3 warnings, 0 errors").copy_button(false),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let style = doc::example(
        ctx,
        SOURCE,
        &["style"],
        container(
            // #region style
            code_block(l10n, "{\"name\": \"oito2_iced\", \"private\": true}")
                .language("json")
                .text_size(16)
                .line_height(1.6)
                .padding(spacing::SM),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    let editor = doc::example(
        ctx,
        SOURCE,
        &["editor"],
        // #region editor
        text_editor(&state.editor)
            .font(font::JETBRAINS_MONO)
            .size(13)
            .height(140)
            .on_action(Message::Edit)
            .highlight_with::<Highlighter>(Settings::new("rust"), highlighter::to_format),
        // #endregion
    );

    let more: Element<'a, Message, Theme> = text(language::LANGUAGES.join(", ")).into();

    doc::page(
        "CodeBlock",
        "Shows source code highlighted with the syntax colors of the theme, with optional \
         line numbers and a copy button.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`code_block` of the `oito2_iced_syntax` crate takes the strings of the \
                 current language, for the copy tooltip, and the code; `language` names its \
                 grammar, by name, alias such as `yml` or `sh`, or file extension such as \
                 `rs`, in any letter case. An unknown language, or none, shows plain text. \
                 The block scrolls sideways when a line is too long.",
            ),
            doc::code(ctx, "use oito2_iced_syntax::code_block::code_block;"),
            doc::prose(
                "The colors come from the syntax colors of the theme, applied when the \
                 block is drawn, so a theme change recolors it without tokenizing again. \
                 The code uses the embedded JetBrains Mono font: register \
                 `font::JETBRAINS_MONO_FONTS` with the application, or the code falls back \
                 to a system font.",
            ),
            doc::note(
                "The grammars are the TextMate grammars of `bat`, run by `syntect`, and load \
                 the first time a language is looked up. The tokens of a code are kept in a \
                 cache shared by every block, so building the view again costs one cache \
                 lookup.",
            ),
            doc::heading("Examples"),
            doc::subheading("Languages and line numbers"),
            doc::prose(
                "`line_numbers` adds a column of line numbers in the comment color, and \
                 `on_copied` publishes a message after the copy button copies the code.",
            ),
            languages,
            doc::caption(copies),
            doc::subheading("Plain text, without the copy button"),
            doc::prose(
                "Without a language the code keeps the foreground color. \
                 `copy_button(false)` removes the copy button from the top end corner, for \
                 text that is not meant to be copied.",
            ),
            plain,
            doc::subheading("Text size, line height and padding"),
            doc::prose(
                "`text_size` replaces the 13-pixel size and `line_height` the 1.45 line \
                 height; `padding` is the space around the code, 16 pixels by default.",
            ),
            style,
            doc::subheading("Highlighted text editor"),
            doc::prose(
                "`Highlighter` highlights the Iced `text_editor` line by line in the same \
                 colors, keeping the parser state of every line so an edit tokenizes again \
                 only from the changed line down. Its `Settings` carry the language, and \
                 `to_format` turns each token into its color and face.",
            ),
            editor,
            doc::subheading("Languages"),
            doc::prose(
                "`language::LANGUAGES` lists the languages by name, and `language::supports` \
                 tells whether a name finds a grammar:",
            ),
            more,
            doc::api(
                ctx,
                &[
                    "oito2_iced_syntax::code_block::code_block",
                    "oito2_iced_syntax::code_block::CodeBlock",
                    "oito2_iced_syntax::highlighter::Highlighter",
                    "oito2_iced_syntax::highlighter::Settings",
                    "oito2_iced_syntax::highlighter::to_format",
                    "oito2_iced_syntax::highlighter::highlight",
                    "oito2_iced_syntax::language::LANGUAGES",
                    "oito2_iced_syntax::language::supports",
                    "oito2_iced_syntax::font::JETBRAINS_MONO",
                ],
            ),
        ],
    )
}
