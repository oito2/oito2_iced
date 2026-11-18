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

pub mod angle_slider;
pub mod autocomplete;
pub mod avatar;
pub mod button;
pub mod calendar;
pub mod carousel;
pub mod chips;
pub mod color_picker;
pub mod command_palette;
pub mod containers;
pub mod context_menu;
pub mod controls;
pub mod copy;
pub mod diagonal_clip;
pub mod dialog;
pub mod draggable;
pub mod editable_label;
pub mod entries;
pub mod expandable;
pub mod floating_panel;
pub mod helpers;
pub mod highlighted_text;
pub mod hover_card;
pub mod info;
pub mod json_tree_view;
pub mod list_box;
pub mod list_tile;
pub mod marquee;
pub mod master_detail;
pub mod menu;
pub mod navigation_page;
pub mod navigation_rail;
pub mod number_entry;
pub mod page_indicator;
pub mod pagination;
pub mod paned_view;
pub mod password;
pub mod pin_entry;
pub mod popover;
pub mod progress;
pub mod rating;
pub mod scroll_area;
pub mod search;
pub mod segment_group;
pub mod selectable;
pub mod sheet;
pub mod slider;
pub mod split_button;
pub mod splitter;
pub mod steps;
pub mod swap;
pub mod tab_bar;
pub mod table_of_contents;
pub mod tags_field;
pub mod text_field;
pub mod timer;
pub mod toast;
pub mod toggle_group;
pub mod toggles;
pub mod tour;
pub mod tree_view;

mod focusable;
mod min_size;
pub(crate) mod resize_handle;
mod tooltip;
mod typography;

pub use focusable::{Focusable, focusable};
pub use min_size::{MinSize, Minimum, min_size};
pub use popover::{Popover, popover};
pub use tooltip::{TOOLTIP_DELAY, tooltip};
pub use typography::styled_text;
