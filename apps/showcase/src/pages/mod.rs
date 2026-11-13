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

//! The component pages of the showcase: one page per family of widgets,
//! each with its examples and the state they change.

mod buttons;
mod containers;
mod entries;
mod groups;
mod icons;
mod layout;
mod navigation;
mod overlays;
mod progress;
mod toggles;

use std::fmt;

use oito2_iced::collections::ListCollection;
use oito2_iced::widget::splitter;

use iced::font::Weight;
use iced::widget::{column, container, text};
use iced::{Element, Length};

use oito2_iced::l10n::Localizations;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::{Theme, font};

/// A page of the showcase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Page {
    /// Theme, overlay, fonts and animation.
    #[default]
    Overview,
    /// Buttons.
    Buttons,
    /// Check boxes, radios, switches, toggle buttons and list tiles.
    Toggles,
    /// Segment and toggle groups, choice chips and sliders.
    Groups,
    /// Text fields and the fields built on them.
    Entries,
    /// Containers, sections, tiles, banners and info boxes.
    Containers,
    /// Expandable sections, scroll areas, splitters and paned views.
    Layout,
    /// Tab bars, navigation rails and pages, and master-detail pages.
    Navigation,
    /// Menus, popovers, hover cards, autocomplete, sheets, dialogs and toasts.
    Overlays,
    /// Progress indicators.
    Progress,
    /// Icons.
    Icons,
}

impl Page {
    /// Every page, in the order of the sidebar.
    pub const ALL: [Page; 11] = [
        Page::Overview,
        Page::Buttons,
        Page::Toggles,
        Page::Groups,
        Page::Entries,
        Page::Containers,
        Page::Layout,
        Page::Navigation,
        Page::Overlays,
        Page::Progress,
        Page::Icons,
    ];
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Page::Overview => "Overview",
            Page::Buttons => "Buttons",
            Page::Toggles => "Toggles",
            Page::Groups => "Groups",
            Page::Entries => "Entries",
            Page::Containers => "Containers",
            Page::Layout => "Layout",
            Page::Navigation => "Navigation",
            Page::Overlays => "Overlays",
            Page::Progress => "Progress",
            Page::Icons => "Icons",
        })
    }
}

/// The labels of the choice chips.
const CHIPS: [&str; 7] = [
    "All",
    "Images",
    "Videos",
    "Documents",
    "Audio",
    "Archives",
    "Code",
];

/// The state the component pages change.
#[derive(Debug)]
pub struct Gallery {
    presses: u32,
    favorite: bool,
    check: Option<bool>,
    size: u8,
    dark: bool,
    bluetooth: bool,
    layout: &'static str,
    styles: Vec<&'static str>,
    chips: Vec<bool>,
    volume: f32,
    selectable: bool,
    banner: bool,
    name: String,
    email: String,
    password: String,
    search: String,
    title_search: String,
    title_search_active: bool,
    copies: f64,
    price: f64,
    document_name: String,
    expanded: bool,
    panel: Vec<bool>,
    split: splitter::State,
    nested_split: splitter::State,
    pane_size: f32,
    tab: usize,
    subtle_tab: usize,
    vertical_tab: usize,
    rail: usize,
    destination: usize,
    device: Option<usize>,
    device_pane: f32,
    sort: &'static str,
    sort_open: bool,
    hidden_files: bool,
    fruits: Option<ListCollection<String>>,
    fruit: Option<String>,
    fruit_open: bool,
    split_open: bool,
    context_open: bool,
    popover_open: bool,
    cities: Option<ListCollection<String>>,
    city: String,
}

impl Default for Gallery {
    fn default() -> Self {
        Self {
            presses: 0,
            favorite: false,
            check: Some(true),
            size: 1,
            dark: false,
            bluetooth: true,
            layout: "grid",
            styles: vec!["b"],
            chips: CHIPS.iter().map(|label| *label == "Images").collect(),
            volume: 40.0,
            selectable: false,
            banner: false,
            name: String::new(),
            email: String::new(),
            password: String::new(),
            search: String::new(),
            title_search: String::new(),
            title_search_active: false,
            copies: 1.0,
            price: 1234.5,
            document_name: "Project notes".to_owned(),
            expanded: false,
            panel: vec![false, true, false],
            split: splitter::State::new(),
            nested_split: splitter::State::new(),
            pane_size: 240.0,
            tab: 0,
            subtle_tab: 0,
            vertical_tab: 0,
            rail: 0,
            destination: 0,
            device: Some(1),
            device_pane: 240.0,
            sort: overlays::SORTS[0],
            sort_open: false,
            hidden_files: false,
            fruits: collection(&["Apple", "Banana", "Cherry", "Grape", "Mango"]),
            fruit: None,
            fruit_open: false,
            split_open: false,
            context_open: false,
            popover_open: false,
            cities: collection(&[
                "Belo Horizonte",
                "Brasília",
                "Curitiba",
                "Juiz de Fora",
                "Salvador",
                "Santos",
                "São Paulo",
            ]),
            city: String::new(),
        }
    }
}

/// A change made on a component page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A button was pressed.
    Press,
    /// The favorite toggle button was pressed.
    ToggleFavorite,
    /// The check box changed.
    Check(Option<bool>),
    /// A size radio was chosen.
    Size(Option<u8>),
    /// The dark style switch changed.
    Dark(bool),
    /// The Bluetooth switch changed.
    Bluetooth(bool),
    /// A layout segment was chosen.
    Layout(&'static str),
    /// The text styles changed.
    Styles(Vec<&'static str>),
    /// A choice chip was pressed.
    Chip(usize),
    /// The volume slider moved.
    Volume(f32),
    /// The selectable container was toggled.
    Selectable(bool),
    /// The banner was pressed.
    Banner,
    /// The name field changed.
    Name(String),
    /// The email field changed.
    Email(String),
    /// The password field changed.
    Password(String),
    /// The search field changed.
    Search(String),
    /// The title search field changed.
    TitleSearch(String),
    /// The title search opened or closed.
    TitleSearchActive(bool),
    /// The copies entry changed.
    Copies(f64),
    /// The price entry changed.
    Price(f64),
    /// The document was renamed.
    Rename(String),
    /// The expandable section was opened or closed.
    Expanded(bool),
    /// A section of the expansion panel was opened or closed.
    Panel(usize, bool),
    /// The splitter moved.
    Split(splitter::State),
    /// The nested splitter moved.
    NestedSplit(splitter::State),
    /// The paned view divider moved.
    PaneSize(f32),
    /// A tab of the pill tab bar was chosen.
    Tab(usize),
    /// A tab of the subtle tab bar was chosen.
    SubtleTab(usize),
    /// A vertical tab was chosen.
    VerticalTab(usize),
    /// A rail destination was chosen.
    Rail(usize),
    /// A destination of the navigation page was chosen.
    Destination(usize),
    /// A device was chosen in the master-detail page, or the detail closed.
    Device(Option<usize>),
    /// The master pane of the master-detail page was resized.
    DevicePane(f32),
    /// A sort order was chosen.
    Sort(&'static str),
    /// The sort menu opened or closed.
    SortOpen(bool),
    /// The hidden files item changed.
    HiddenFiles(bool),
    /// A fruit was chosen.
    Fruit(String),
    /// The fruit select opened or closed.
    FruitOpen(bool),
    /// The split button menu opened or closed.
    SplitOpen(bool),
    /// The context menu opened or closed.
    ContextOpen(bool),
    /// The popover opened or closed.
    PopoverOpen(bool),
    /// The city field changed or a city was picked.
    City(String),
    /// A sheet should open: at the bottom when true, at the end otherwise.
    OpenSheet(bool),
    /// The dialog should open.
    OpenDialog,
    /// A toast should show.
    ShowToast,
    /// The command palette should open.
    OpenPalette,
}

/// A collection of the `labels`, valued by themselves; `None` when two
/// labels repeat.
fn collection(labels: &[&str]) -> Option<ListCollection<String>> {
    ListCollection::new(labels.iter().map(|label| (*label).to_owned())).ok()
}

impl Gallery {
    /// Applies a change.
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Press => self.presses += 1,
            Message::ToggleFavorite => self.favorite = !self.favorite,
            Message::Check(value) => self.check = value,
            Message::Size(size) => {
                if let Some(size) = size {
                    self.size = size;
                }
            }
            Message::Dark(dark) => self.dark = dark,
            Message::Bluetooth(on) => self.bluetooth = on,
            Message::Layout(layout) => self.layout = layout,
            Message::Styles(styles) => self.styles = styles,
            Message::Chip(index) => {
                if let Some(selected) = self.chips.get_mut(index) {
                    *selected = !*selected;
                }
            }
            Message::Volume(volume) => self.volume = volume,
            Message::Selectable(selected) => self.selectable = selected,
            Message::Banner => self.banner = !self.banner,
            Message::Name(name) => self.name = name,
            Message::Email(email) => self.email = email,
            Message::Password(password) => self.password = password,
            Message::Search(search) => self.search = search,
            Message::TitleSearch(search) => self.title_search = search,
            Message::TitleSearchActive(active) => {
                self.title_search_active = active;
                if !active {
                    self.title_search.clear();
                }
            }
            Message::Copies(copies) => self.copies = copies,
            Message::Price(price) => self.price = price,
            Message::Rename(name) => self.document_name = name,
            Message::Expanded(expanded) => self.expanded = expanded,
            Message::Panel(index, open) => {
                if let Some(section) = self.panel.get_mut(index) {
                    *section = open;
                }
            }
            Message::Split(state) => self.split = state,
            Message::NestedSplit(state) => self.nested_split = state,
            Message::PaneSize(size) => self.pane_size = size,
            Message::Tab(index) => self.tab = index,
            Message::SubtleTab(index) => self.subtle_tab = index,
            Message::VerticalTab(index) => self.vertical_tab = index,
            Message::Rail(index) => self.rail = index,
            Message::Destination(index) => self.destination = index,
            Message::Device(device) => self.device = device,
            Message::DevicePane(size) => self.device_pane = size,
            Message::Sort(sort) => self.sort = sort,
            Message::SortOpen(open) => self.sort_open = open,
            Message::HiddenFiles(shown) => self.hidden_files = shown,
            Message::Fruit(fruit) => self.fruit = Some(fruit),
            Message::FruitOpen(open) => self.fruit_open = open,
            Message::SplitOpen(open) => self.split_open = open,
            Message::ContextOpen(open) => self.context_open = open,
            Message::PopoverOpen(open) => self.popover_open = open,
            Message::City(city) => self.city = city,
            Message::OpenSheet(_)
            | Message::OpenDialog
            | Message::ShowToast
            | Message::OpenPalette => {}
        }
    }

    /// The content of a component page; `None` for the overview, which the
    /// application draws itself.
    pub fn view<'a>(
        &'a self,
        page: Page,
        localizations: &Localizations,
    ) -> Option<Element<'a, Message, Theme>> {
        match page {
            Page::Overview => None,
            Page::Buttons => Some(buttons::view(self, localizations)),
            Page::Toggles => Some(toggles::view(self)),
            Page::Groups => Some(groups::view(self, localizations)),
            Page::Entries => Some(entries::view(self, localizations)),
            Page::Containers => Some(containers::view(self)),
            Page::Layout => Some(layout::view(self)),
            Page::Navigation => Some(navigation::view(self, localizations)),
            Page::Overlays => Some(overlays::view(self)),
            Page::Progress => Some(progress::view(self)),
            Page::Icons => Some(icons::view()),
        }
    }
}

/// A titled card holding one example.
fn card<'a, Message: 'a>(
    title: &'a str,
    content: impl Into<Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    container(
        column![
            text(title).font(font::inter(Weight::Semibold)),
            content.into()
        ]
        .spacing(spacing::MD),
    )
    .class(ContainerClass::Card)
    .padding(spacing::LG)
    .width(Length::Fill)
    .into()
}

/// A page: its title, a one-line description and its cards.
fn page<'a, Message: 'a>(
    title: &'a str,
    description: &'a str,
    cards: impl IntoIterator<Item = Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    column![
        oito2_iced::widget::styled_text(title, typography::HEADLINE_SMALL),
        text(description).class(TextClass::Variant),
    ]
    .extend(cards)
    .spacing(spacing::LG)
    .into()
}
