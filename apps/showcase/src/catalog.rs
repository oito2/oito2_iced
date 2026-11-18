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

//! The catalog of the showcase: every page with its group and title, the
//! state of every page, and the messages that reach them.
//!
//! Each page is a module of [`crate::pages`] with a `State` (its
//! `Default` is the first view), a `Message`, an `update` that may start
//! a task or ask the application for an [`Action`] through the [`Shell`],
//! and a `view`.

use std::fmt;

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::widget::info::InfoType;

use crate::doc::Context;
use crate::pages;

/// A group of pages in the sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Group {
    /// Getting started, the changelog and the project.
    Overview,
    /// The patterns every component shares.
    Guides,
    /// One page per widget of `oito2_iced`.
    Components,
    /// The widgets of `oito2_iced_extras`.
    Extras,
    /// The widgets of `oito2_iced_files`.
    Files,
    /// Collections, formatting, commands and the freedesktop icon names.
    Utilities,
    /// The theme, its colors, fonts and tokens, and the platform rules.
    Theme,
}

impl Group {
    /// Every group, in the order of the sidebar.
    pub const ALL: [Group; 7] = [
        Group::Overview,
        Group::Guides,
        Group::Components,
        Group::Extras,
        Group::Files,
        Group::Utilities,
        Group::Theme,
    ];

    /// The name of the group.
    pub fn title(self) -> &'static str {
        match self {
            Group::Overview => "Overview",
            Group::Guides => "Guides",
            Group::Components => "Components",
            Group::Extras => "Extras",
            Group::Files => "Files",
            Group::Utilities => "Utilities",
            Group::Theme => "Theme",
        }
    }
}

/// Something a page asks the application to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Shows a toast; with `undo`, the toast has an Undo action.
    Toast {
        /// The title of the toast.
        title: String,
        /// The text under the title.
        description: Option<String>,
        /// The kind of message.
        kind: InfoType,
        /// Whether the toast offers to undo.
        undo: bool,
    },
    /// Opens the example dialog.
    OpenDialog,
    /// Opens the example sheet, at the bottom when `true`, at the end
    /// otherwise.
    OpenSheet(bool),
    /// Opens the command palette.
    OpenPalette,
    /// Opens the settings dialog.
    OpenSettings,
}

/// The actions a page update asks for, applied by the application after
/// the update.
#[derive(Debug, Default)]
pub struct Shell {
    actions: Vec<Action>,
}

impl Shell {
    /// Asks the application for `action`.
    pub fn request(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Shows a toast of `kind` titled `title`.
    pub fn toast(&mut self, title: impl Into<String>, kind: InfoType) {
        self.request(Action::Toast {
            title: title.into(),
            description: None,
            kind,
            undo: false,
        });
    }

    /// The actions asked for, in order.
    pub fn into_actions(self) -> Vec<Action> {
        self.actions
    }
}

/// Declares the pages: `module => Variant, "Title"` per group.
macro_rules! catalog {
    ($($group:ident { $($module:ident => $variant:ident, $title:literal;)* })*) => {
        /// A page of the showcase.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum PageId {
            $($(
                #[doc = $title]
                $variant,
            )*)*
        }

        impl PageId {
            /// Every page, in the order of the sidebar.
            pub const ALL: &[PageId] = &[$($(PageId::$variant,)*)*];

            /// The title of the page.
            pub fn title(self) -> &'static str {
                match self {
                    $($(PageId::$variant => $title,)*)*
                }
            }

            /// The group of the page.
            pub fn group(self) -> Group {
                match self {
                    $($(PageId::$variant => Group::$group,)*)*
                }
            }

            /// The module name of the page, also its identifier in commands.
            pub fn slug(self) -> &'static str {
                match self {
                    $($(PageId::$variant => stringify!($module),)*)*
                }
            }
        }

        /// The state of every page.
        #[derive(Debug, Default)]
        pub struct Pages {
            $($(
                #[doc = $title]
                pub $module: pages::$module::State,
            )*)*
        }

        /// A message of one page.
        #[derive(Debug, Clone)]
        pub enum Message {
            $($(
                #[doc = $title]
                $variant(pages::$module::Message),
            )*)*
        }

        impl Pages {
            /// Applies a message to its page.
            pub fn update(&mut self, message: Message, shell: &mut Shell) -> Task<Message> {
                match message {
                    $($(
                        Message::$variant(message) => {
                            pages::$module::update(&mut self.$module, message, shell)
                                .map(Message::$variant)
                        }
                    )*)*
                }
            }

            /// The content of `page`.
            pub fn view<'a>(&'a self, page: PageId, ctx: &Context) -> Element<'a, Message, Theme> {
                match page {
                    $($(
                        PageId::$variant => {
                            pages::$module::view(&self.$module, ctx).map(Message::$variant)
                        }
                    )*)*
                }
            }
        }
    };
}

catalog! {
    Overview {
        getting_started => GettingStarted, "Getting Started";
        changelog => Changelog, "Changelog";
        about => About, "About";
    }
    Guides {
        styling => Styling, "Styling";
        composition => Composition, "Composition";
        component_state => ComponentState, "Component State";
        animations => Animations, "Animations";
        forms => Forms, "Forms";
        refs => Refs, "Refs";
    }
    Components {
        angle_slider => AngleSlider, "AngleSlider";
        animated_icon => AnimatedIcon, "AnimatedIcon";
        autocomplete => Autocomplete, "Autocomplete";
        avatar => Avatar, "Avatar";
        banner => Banner, "Banner";
        border_container => BorderContainer, "BorderContainer";
        button => Button, "Button";
        calendar => Calendar, "Calendar";
        carousel => Carousel, "Carousel";
        check_button => CheckButton, "CheckButton";
        checkbox => Checkbox, "Checkbox";
        checkbox_list_tile => CheckboxListTile, "CheckboxListTile";
        choice_chip_bar => ChoiceChipBar, "ChoiceChipBar";
        code_block => CodeBlock, "CodeBlock";
        color_disk => ColorDisk, "ColorDisk";
        color_picker => ColorPicker, "ColorPicker";
        context_menu_region => ContextMenuRegion, "ContextMenuRegion";
        copy_button => CopyButton, "CopyButton";
        date_time_entry => DateTimeEntry, "DateTimeEntry";
        diagonal_clip => DiagonalClip, "DiagonalClip";
        dialog => Dialog, "Dialog";
        dialog_title_bar => DialogTitleBar, "DialogTitleBar";
        draggable => Draggable, "Draggable";
        editable_label => EditableLabel, "EditableLabel";
        expandable => Expandable, "Expandable";
        expansion_panel => ExpansionPanel, "ExpansionPanel";
        floating_panel => FloatingPanel, "FloatingPanel";
        focus_border => FocusBorder, "FocusBorder";
        header_bar => HeaderBar, "HeaderBar";
        highlighted_text => HighlightedText, "HighlightedText";
        hover_card => HoverCard, "HoverCard";
        icon_button => IconButton, "IconButton";
        info_box => InfoBox, "InfoBox";
        json_tree_view => JsonTreeView, "JsonTreeView";
        list_box => ListBox, "ListBox";
        list_tile => ListTile, "ListTile";
        marquee => Marquee, "Marquee";
        master_detail_page => MasterDetailPage, "MasterDetailPage";
        navigation_page => NavigationPage, "NavigationPage";
        navigation_rail => NavigationRail, "NavigationRail";
        number_entry => NumberEntry, "NumberEntry";
        option_button => OptionButton, "OptionButton";
        page_indicator => PageIndicator, "PageIndicator";
        pagination => Pagination, "Pagination";
        paned_view => PanedView, "PanedView";
        password_field => PasswordField, "PasswordField";
        pin_entry => PinEntry, "PinEntry";
        placeholder_icon => PlaceholderIcon, "PlaceholderIcon";
        popover => Popover, "Popover";
        popup_menu_button => PopupMenuButton, "PopupMenuButton";
        progress_indicator => ProgressIndicator, "ProgressIndicator";
        radio => Radio, "Radio";
        radio_button => RadioButton, "RadioButton";
        rating => Rating, "Rating";
        scroll_area => ScrollArea, "ScrollArea";
        scroll_view_undershoot => ScrollViewUndershoot, "ScrollViewUndershoot";
        search_field => SearchField, "SearchField";
        section => Section, "Section";
        segment_group => SegmentGroup, "SegmentGroup";
        segmented_entry => SegmentedEntry, "SegmentedEntry";
        selectable_container => SelectableContainer, "SelectableContainer";
        sheet => Sheet, "Sheet";
        slider => Slider, "Slider";
        split_button => SplitButton, "SplitButton";
        splitter => Splitter, "Splitter";
        steps => Steps, "Steps";
        swap => Swap, "Swap";
        switch => Switch, "Switch";
        switch_button => SwitchButton, "SwitchButton";
        tab_bar => TabBar, "TabBar";
        table_of_contents => TableOfContents, "TableOfContents";
        tags_field => TagsField, "TagsField";
        text_field => TextField, "TextField";
        tile => Tile, "Tile";
        timer_text => TimerText, "TimerText";
        title_bar => TitleBar, "TitleBar";
        toaster => Toaster, "Toaster";
        toggle_button => ToggleButton, "ToggleButton";
        toggle_group => ToggleGroup, "ToggleGroup";
        tooltip => Tooltip, "Tooltip";
        tour => Tour, "Tour";
        tree_view => TreeView, "TreeView";
        vertical_tabs => VerticalTabs, "VerticalTabs";
        watermark => Watermark, "Watermark";
    }
    Extras {
        image_cropper => ImageCropper, "ImageCropper";
        navigation_menu => NavigationMenu, "NavigationMenu";
        qr_code => QrCode, "QrCode";
        signature_pad => SignaturePad, "SignaturePad";
    }
    Files {
        download_trigger => DownloadTrigger, "DownloadTrigger";
        file_upload => FileUpload, "FileUpload";
    }
    Utilities {
        collections => Collections, "Collections";
        commands => Commands, "Commands";
        format => Format, "Format";
        freedesktop_icons => FreedesktopIcons, "FreedesktopIcons";
    }
    Theme {
        material_theme => MaterialTheme, "Material theme";
        foundation => Foundation, "Foundation";
    }
}

// The macro cannot mark one variant as the default, so the impl is written
// out.
#[allow(clippy::derivable_impls)]
impl Default for PageId {
    fn default() -> Self {
        PageId::GettingStarted
    }
}

impl fmt::Display for PageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.title())
    }
}
