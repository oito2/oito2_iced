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

//! Markdown styling: code blocks use the code block container class.

use iced::widget::{container, markdown};

use super::Theme;
use super::container::Class;

impl markdown::Catalog for Theme {
    fn code_block<'a>() -> <Self as container::Catalog>::Class<'a> {
        Class::Custom(Box::new(code_block))
    }
}

/// A code block: the highest surface container with the medium radius.
pub fn code_block(theme: &Theme) -> container::Style {
    let roles = theme.roles();

    container::Style {
        background: Some(iced::Background::Color(roles.surface_container_highest)),
        text_color: Some(roles.syntax.foreground),
        border: iced::Border::default()
            .color(roles.outline_variant)
            .width(1.0)
            .rounded(crate::tokens::radius::MEDIUM),
        ..container::Style::default()
    }
}
