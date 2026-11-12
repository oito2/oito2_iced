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
//! This is the F0 prototype: the theme implements [`iced::theme::Base`] and
//! the catalogs of `button`, `checkbox`, `container`, `text` and
//! `text_input`; [`widget::focusable`] gives keyboard focus to any element;
//! [`focus::navigation`] moves the focus with Tab and Shift+Tab;
//! [`widget::popover()`] opens an anchored panel; [`window`] draws the title
//! bar and the resize handles of an undecorated window.

#![deny(missing_docs)]

pub mod animation;
pub mod focus;
pub mod font;
pub mod theme;
pub mod tokens;
pub mod widget;
pub mod window;

pub use theme::Theme;

/// An [`iced::Element`] styled by the OITO2 [`Theme`].
pub type Element<'a, Message, Renderer = iced::Renderer> =
    iced::Element<'a, Message, Theme, Renderer>;
