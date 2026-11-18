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

//! The pages of the showcase, one module per page; see
//! [`crate::catalog`] for the shape every page follows.

pub mod about;
pub mod angle_slider;
pub mod animated_icon;
pub mod animations;
pub mod autocomplete;
pub mod avatar;
pub mod banner;
pub mod border_container;
pub mod button;
pub mod calendar;
pub mod carousel;
pub mod changelog;
pub mod check_button;
pub mod checkbox;
pub mod checkbox_list_tile;
pub mod choice_chip_bar;
pub mod code_block;
pub mod collections;
pub mod color_disk;
pub mod color_picker;
pub mod commands;
pub mod component_state;
pub mod composition;
pub mod context_menu_region;
pub mod copy_button;
pub mod date_time_entry;
pub mod diagonal_clip;
pub mod dialog;
pub mod dialog_title_bar;
pub mod download_trigger;
pub mod draggable;
pub mod editable_label;
pub mod expandable;
pub mod expansion_panel;
pub mod file_upload;
pub mod floating_panel;
pub mod focus_border;
pub mod format;
pub mod forms;
pub mod foundation;
pub mod freedesktop_icons;
pub mod getting_started;
pub mod header_bar;
pub mod highlighted_text;
pub mod hover_card;
pub mod icon_button;
pub mod image_cropper;
pub mod info_box;
pub mod json_tree_view;
pub mod list_box;
pub mod list_tile;
pub mod marquee;
pub mod master_detail_page;
pub mod material_theme;
pub mod navigation_menu;
pub mod navigation_page;
pub mod navigation_rail;
pub mod number_entry;
pub mod option_button;
pub mod page_indicator;
pub mod pagination;
pub mod paned_view;
pub mod password_field;
pub mod pin_entry;
pub mod placeholder_icon;
pub mod popover;
pub mod popup_menu_button;
pub mod progress_indicator;
pub mod qr_code;
pub mod radio;
pub mod radio_button;
pub mod rating;
pub mod refs;
pub mod scroll_area;
pub mod scroll_view_undershoot;
pub mod search_field;
pub mod section;
pub mod segment_group;
pub mod segmented_entry;
pub mod selectable_container;
pub mod sheet;
pub mod signature_pad;
pub mod slider;
pub mod split_button;
pub mod splitter;
pub mod steps;
pub mod styling;
pub mod swap;
pub mod switch;
pub mod switch_button;
pub mod tab_bar;
pub mod table_of_contents;
pub mod tags_field;
pub mod text_field;
pub mod tile;
pub mod timer_text;
pub mod title_bar;
pub mod toaster;
pub mod toggle_button;
pub mod toggle_group;
pub mod tooltip;
pub mod tour;
pub mod tree_view;
pub mod vertical_tabs;
pub mod watermark;

use iced::Element;

use oito2_iced::Theme;

use crate::doc::{self, Context};

/// A page that shows the Markdown `document` under `title` and
/// `description`.
fn document<'a, Message: Clone + 'a>(
    title: &'a str,
    description: &'a str,
    ctx: &Context,
    document: &'static str,
) -> Element<'a, Message, Theme> {
    doc::page(title, description, doc::markdown(ctx, document))
}
