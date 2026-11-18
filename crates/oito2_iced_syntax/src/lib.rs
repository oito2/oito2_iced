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

//! Syntax highlighting for the OITO2 Iced design system.
//!
//! The crate highlights code with the TextMate grammars of `bat` (through
//! `syntect` and `two-face`) in the [`SyntaxColors`] of the OITO2 theme:
//!
//! - [`role`]: the token roles and the scopes that map to them;
//! - [`language`]: the languages and how a name finds a grammar;
//! - [`highlighter`]: runs of tokens, and a highlighter for the Iced text
//!   editor;
//! - [`code_block`]: the code block widget;
//! - [`font`]: the embedded JetBrains Mono font.
//!
//! [`SyntaxColors`]: oito2_iced::theme::SyntaxColors
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_syntax::code_block;
//!
//! # fn view<'a>(localizations: &Localizations) -> oito2_iced::Element<'a, ()> {
//! code_block::code_block(localizations, "fn main() {}")
//!     .language("rust")
//!     .into()
//! # }
//! ```

#![deny(missing_docs)]

pub mod code_block;
pub mod font;
pub mod highlighter;
pub mod language;
pub mod role;
