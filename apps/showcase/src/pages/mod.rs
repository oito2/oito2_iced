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
mod groups;
mod icons;
mod progress;
mod toggles;

use std::fmt;

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
    /// Containers, sections, tiles, banners and info boxes.
    Containers,
    /// Progress indicators.
    Progress,
    /// Icons.
    Icons,
}

impl Page {
    /// Every page, in the order of the sidebar.
    pub const ALL: [Page; 7] = [
        Page::Overview,
        Page::Buttons,
        Page::Toggles,
        Page::Groups,
        Page::Containers,
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
            Page::Containers => "Containers",
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
            Page::Containers => Some(containers::view(self)),
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
