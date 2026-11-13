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

//! OITO2 widgets.

pub mod button;
pub mod chips;
pub mod containers;
pub mod controls;
pub mod copy;
pub mod editable_label;
mod focusable;
pub mod helpers;
pub mod highlighted_text;
pub mod info;
pub mod list_tile;
mod min_size;
pub mod number_entry;
pub mod password;
pub mod popover;
pub mod progress;
pub mod search;
pub mod segment_group;
pub mod selectable;
pub mod slider;
pub mod text_field;
pub mod toggle_group;
pub mod toggles;
mod tooltip;
mod typography;

pub use focusable::{Focusable, focusable};
pub use min_size::{MinSize, Minimum, min_size};
pub use popover::{Popover, popover};
pub use tooltip::{TOOLTIP_DELAY, tooltip};
pub use typography::styled_text;
