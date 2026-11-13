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

//! Tooltips in the OITO2 style.

use std::time::Duration;

use iced::widget::text::IntoFragment;
use iced::widget::{Tooltip, container};

use super::styled_text;
use crate::theme::container::Class;
use crate::tokens::{spacing, typography};
use crate::{Element, Theme};

/// How long the pointer rests on an element before its tooltip shows.
pub const TOOLTIP_DELAY: Duration = Duration::from_millis(500);

/// Shows `label` below `content` while the pointer rests on it: small text
/// on the inverse surface with the small radius, after [`TOOLTIP_DELAY`].
pub fn tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    label: impl IntoFragment<'a>,
) -> Tooltip<'a, Message, Theme> {
    let label = container(styled_text(label, typography::BODY_SMALL))
        .padding([spacing::XS, spacing::SM])
        .class(Class::Tooltip);

    Tooltip::new(content, label, iced::widget::tooltip::Position::Bottom)
        .gap(spacing::XS)
        .delay(TOOLTIP_DELAY)
}
