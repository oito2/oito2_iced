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

//! A popover: a panel anchored to an element, on the popover surface.
//!
//! [`popover`] is an [`anchored`](crate::overlay::anchored()) panel with
//! the popover style; every option of [`Anchored`] applies.

use iced::Element;
use iced::advanced::renderer;

use crate::Theme;
pub use crate::overlay::anchored::{Alignment, Anchored, Placement};

/// An `anchor` with a popover `panel` that opens next to it while open.
pub fn popover<'a, Message, Renderer>(
    anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
    panel: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Anchored<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    crate::overlay::anchored(anchor, panel)
}

/// A popover. See [`popover`].
pub type Popover<'a, Message, Renderer = iced::Renderer> = Anchored<'a, Message, Renderer>;
