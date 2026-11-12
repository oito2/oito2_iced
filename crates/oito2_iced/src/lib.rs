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

//! OITO2 design system for [Iced](https://iced.rs): a Material 3 theme with
//! the OITO2 color roles, styled widgets, keyboard focus for every control,
//! an anchored overlay and client-side window chrome.
//!
//! - [`Theme`] resolves every color role of the OITO2 base themes (each
//!   accent, light and dark, standard and high contrast) and implements the
//!   style catalog of every native Iced widget; [`theme`] holds the classes.
//! - [`tokens`] holds radii, spacing, sizes, accents, palettes and the type
//!   scale; [`font`] the embedded Inter and JetBrains Mono fonts.
//! - [`focus`] gives keyboard focus to any element, moves it with Tab, the
//!   arrow keys and custom operations, and draws the focus ring.
//! - [`overlay`] holds the anchored panel behind popovers, menus and
//!   tooltips; [`widget`] the OITO2 widgets; [`window`] the title bar,
//!   window controls and resize handles of an undecorated window;
//!   [`animation`] the Material motion.
//! - [`l10n`], [`format`](mod@format), [`collections`] and [`shortcuts`] work without a
//!   user interface: strings in English and Brazilian Portuguese, byte sizes
//!   and relative times, list, selection and tree models, and key chords.
//! - [`foundation`] re-exports the platform rules shared by the OITO2
//!   interfaces: desktop environment, window buttons and system settings.

#![deny(missing_docs)]

pub mod animation;
pub mod collections;
pub mod focus;
pub mod font;
pub mod format;
pub mod l10n;
pub mod overlay;
pub mod shortcuts;
pub mod theme;
pub mod tokens;
pub mod widget;
pub mod window;

pub use oito2_foundation as foundation;
pub use theme::Theme;

/// An [`iced::Element`] styled by the OITO2 [`Theme`].
pub type Element<'a, Message, Renderer = iced::Renderer> =
    iced::Element<'a, Message, Theme, Renderer>;
