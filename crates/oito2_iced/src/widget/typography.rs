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

//! Text in a style of the type scale.

use iced::widget::text::{IntoFragment, LineHeight, Text};

use crate::Theme;
use crate::tokens::typography::TextStyle;

/// `content` in the size, line height and Inter weight of `style`, such as
/// [`LABEL_LARGE`](crate::tokens::typography::LABEL_LARGE) for button
/// labels. Letter spacing is not applied: Iced has no tracking.
pub fn styled_text<'a, Renderer>(
    content: impl IntoFragment<'a>,
    style: TextStyle,
) -> Text<'a, Theme, Renderer>
where
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    Text::new(content)
        .size(style.size)
        .line_height(LineHeight::Absolute(style.line_height.into()))
        .font(crate::font::inter(style.weight))
}
