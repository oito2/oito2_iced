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

//! The NavigationMenu page: links, flyout panels and the line under the
//! current page.

use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::list_tile::list_tile;
use oito2_iced_extras::navigation_menu::{Event, MenuState, item, navigation_menu};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("navigation_menu.rs");

/// The pages of the preview menu.
const PAGES: [&str; 4] = ["Home", "Components", "Docs", "Blog"];
/// The sections of the links-only menu.
const SECTIONS: [&str; 3] = ["Overview", "Pricing", "Changelog"];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    menu: MenuState,
    sections: MenuState,
    selected: usize,
    section: usize,
    opened: Option<&'static str>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview menu opened or closed a panel.
    Menu(Event),
    /// The links-only menu opened or closed a panel.
    Sections(Event),
    /// A page of the preview menu was chosen.
    Page(usize),
    /// A section was chosen.
    Section(usize),
    /// A link of a panel was chosen.
    Link(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Menu(event) => state.menu.apply(event),
        Message::Sections(event) => state.sections.apply(event),
        Message::Page(index) => state.selected = index,
        // #region link-update
        Message::Link(link) => {
            state.opened = Some(link);
            state.menu.apply(Event::Closed);
        }
        // #endregion
        Message::Section(index) => state.section = index,
    }

    Task::none()
}

// #region links
/// A menu panel of `links`, each a title with a short description.
fn links<'a>(links: &[(&'static str, &'static str)]) -> Element<'a, Message, Theme> {
    container(
        row(links.iter().map(|&(title, description)| {
            container(
                list_tile()
                    .title(title)
                    .subtitle(description)
                    .on_press(Message::Link(title)),
            )
            .width(Length::Fixed(200.0))
            .into()
        }))
        .spacing(spacing::SM)
        .wrap(),
    )
    .width(Length::Fixed(420.0))
    .into()
}
// #endregion

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let current = match state.opened {
        Some(link) => format!(
            "Current page: {} · last link: {link}",
            PAGES[state.selected]
        ),
        None => format!("Current page: {}", PAGES[state.selected]),
    };

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "links", "link-update"],
        column![
            border_container(
                scrollable(
                    // #region preview
                    navigation_menu(
                        &state.menu,
                        [
                            item("Home").on_press(Message::Page(0)),
                            item("Components").panel(links(&[
                                ("Inputs", "Fields, pickers and toggles"),
                                ("Overlays", "Popovers, sheets and toasts"),
                                ("Data", "Trees, lists and pagination"),
                                ("Window", "Title bars and controls"),
                            ])),
                            item("Docs").panel(links(&[
                                ("Getting started", "Install and theme"),
                                ("Keyboard", "Focus and shortcuts"),
                            ])),
                            item("Blog").on_press(Message::Page(3)),
                        ],
                    )
                    .selected(Some(state.selected))
                    .on_event(Message::Menu),
                    // #endregion
                )
                .direction(Direction::Horizontal(
                    Scrollbar::new().width(0).scroller_width(0)
                )),
            )
            .padding([spacing::SM, spacing::MD])
            .width(Length::Fill),
            text(current).class(TextClass::Variant),
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(420.0)),
    );

    let links_only = doc::example(
        ctx,
        SOURCE,
        &["links-only"],
        container(
            // #region links-only
            navigation_menu(
                &state.sections,
                SECTIONS
                    .iter()
                    .enumerate()
                    .map(|(index, section)| item(*section).on_press(Message::Section(index))),
            )
            .selected(Some(state.section))
            .on_event(Message::Sections),
            // #endregion
        )
        .width(Length::Fixed(420.0)),
    );

    doc::page(
        "NavigationMenu",
        "A website-style header menu of links and triggers that open flyout panels, with a \
         line under the current page.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `navigation_menu` a `MenuState` and its items, in order. An `item` with \
                 `on_press` is a link; one with `panel` is a trigger that opens that content \
                 in a popover below it; an item with neither is disabled. `selected` marks the \
                 current page with the accent color and a line that slides under it; `None` \
                 marks none.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_extras::navigation_menu::{Event, MenuState, item, navigation_menu};",
            ),
            doc::prose(
                "The application keeps which panel is open in the `MenuState` and applies the \
                 `Event`s the menu publishes through `on_event`; without it the triggers are \
                 disabled. A panel opens when its trigger is pressed, or on hover after \
                 `hover_delay` (150 ms by default); while a panel is open, hovering another \
                 trigger switches to its panel at once. The panel closes shortly after the \
                 pointer leaves both the trigger and the panel, on a press outside, on Escape \
                 or on a second press of its trigger, and when the application applies \
                 `Event::Closed`, as after a link of the panel. Panels are at most \
                 `panel_max_width` pixels wide, 560 by default.",
            ),
            doc::note(
                "The menu lays its items out in one row at their natural width, so in narrow \
                 layouts place it in a horizontal `scrollable`, as here, or switch to a menu \
                 button.",
            ),
            doc::heading("Examples"),
            doc::subheading("Links only"),
            doc::prose(
                "Without panels the menu is a row of links with the sliding line under the \
                 selected one, such as the sections of a page.",
            ),
            links_only,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next item or panel content."],
                    vec![
                        "Enter, Space",
                        "Follows a link, or opens or closes the panel of a trigger.",
                    ],
                    vec![
                        "Down",
                        "Opens the panel of the focused trigger and moves the focus into it.",
                    ],
                    vec![
                        "Escape",
                        "Closes the open panel; the focus returns to its trigger.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_extras::navigation_menu::navigation_menu",
                    "oito2_iced_extras::navigation_menu::NavigationMenu",
                    "oito2_iced_extras::navigation_menu::item",
                    "oito2_iced_extras::navigation_menu::Item",
                    "oito2_iced_extras::navigation_menu::MenuState",
                    "oito2_iced_extras::navigation_menu::Event",
                    "oito2_iced_extras::navigation_menu::HOVER_DELAY",
                    "oito2_iced_extras::navigation_menu::PANEL_MAX_WIDTH",
                ],
            ),
        ],
    )
}
