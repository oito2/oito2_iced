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

//! The HeaderBar page: a window title bar with the application controls,
//! tabs in the middle and a search that takes the place of the title.

use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::menu;
use oito2_iced::widget::search::search_title_field;
use oito2_iced::widget::tab_bar::{TabBarStyle, tab, tab_bar};
use oito2_iced::window::{self, Labels, header_bar};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("header_bar.rs");

/// The fruits the search example filters.
const FRUITS: [&str; 11] = [
    "Apple",
    "Apricot",
    "Banana",
    "Blueberry",
    "Cherry",
    "Grape",
    "Mango",
    "Orange",
    "Peach",
    "Pear",
    "Plum",
];

/// The views of the tabs example.
const VIEWS: [&str; 3] = ["Albums", "Artists", "Playlists"];

/// The views of the subtle tabs example.
const FOLDERS: [&str; 2] = ["Inbox", "Archive"];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    last: String,
    menu_open: bool,
    view: usize,
    folder: usize,
    query: String,
    searching: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            last: "Nothing yet".to_owned(),
            menu_open: false,
            view: 0,
            folder: 0,
            query: String::new(),
            searching: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A control of the bar was used.
    Action(&'static str),
    /// A window button or the bar asked for a window action.
    Window(window::Action),
    /// The main menu opened or closed.
    MenuOpen(bool),
    /// A tab of the tabs example was chosen.
    View(usize),
    /// A tab of the subtle tabs example was chosen.
    Folder(usize),
    /// The search text changed.
    Query(String),
    /// The search opened or closed.
    Searching(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Action(action) => {
            state.last = action.to_owned();
            state.menu_open = false;
        }
        // #region update
        Message::Window(action) => state.last = format!("{action:?}"),
        // #endregion
        // #region search-update
        Message::Query(query) => state.query = query,
        Message::Searching(searching) => {
            state.searching = searching;
            if !searching {
                state.query.clear();
            }
        }
        // #endregion
        Message::MenuOpen(open) => state.menu_open = open,
        Message::View(view) => state.view = view,
        Message::Folder(folder) => state.folder = folder,
    }

    Task::none()
}

/// A small window: `bar` over `body`, up to 560 pixels wide.
fn frame<'a>(
    bar: impl Into<Element<'a, Message, Theme>>,
    body: impl Into<Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    container(
        border_container(column![
            bar.into(),
            container(body)
                .height(Length::Fixed(140.0))
                .width(Length::Fill),
        ])
        .radius(0.0)
        .width(Length::Fill)
        .clip(true),
    )
    .max_width(560.0)
    .width(Length::Fill)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        frame(
            // #region preview
            header_bar(Message::Window)
                .title("Documents")
                .start(
                    icon_button(lucide::ARROW_LEFT)
                        .tooltip("Back")
                        .on_press(Message::Action("Back")),
                )
                .end(
                    icon_button(lucide::FOLDER_PLUS)
                        .tooltip("New folder")
                        .on_press(Message::Action("New folder")),
                )
                .end(
                    menu::anchor(
                        icon_button(lucide::MENU)
                            .tooltip("Main menu")
                            .on_press(Message::MenuOpen(!state.menu_open)),
                        [
                            menu::item("Preferences").on_press(Message::Action("Preferences")),
                            menu::item("About Documents").on_press(Message::Action("About")),
                        ],
                    )
                    .open(state.menu_open)
                    .on_close(Message::MenuOpen(false)),
                )
                .labels(Labels::new(l10n)),
            // #endregion
            container(text(format!("Last action: {}", state.last))).center(Length::Fill),
        ),
    );

    let tabs = doc::example(
        ctx,
        SOURCE,
        &["tabs"],
        frame(
            // #region tabs
            header_bar(Message::Window)
                .title_element(
                    container(
                        tab_bar(
                            [
                                tab("Albums").icon(lucide::HEADPHONES),
                                tab("Artists").icon(lucide::USERS),
                                tab("Playlists").icon(lucide::LIST_MUSIC),
                            ],
                            state.view,
                        )
                        .on_select(Message::View),
                    )
                    .width(Length::Fixed(360.0)),
                )
                .labels(Labels::new(l10n)),
            // #endregion
            container(text(VIEWS.get(state.view).copied().unwrap_or_default()))
                .center(Length::Fill),
        ),
    );

    let subtle = doc::example(
        ctx,
        SOURCE,
        &["subtle"],
        frame(
            // #region subtle
            header_bar(Message::Window)
                .title_element(
                    container(
                        tab_bar([tab("Inbox"), tab("Archive")], state.folder)
                            .style(TabBarStyle::Subtle)
                            .on_select(Message::Folder),
                    )
                    .width(Length::Fixed(240.0)),
                )
                .labels(Labels::new(l10n)),
            // #endregion
            container(text(FOLDERS.get(state.folder).copied().unwrap_or_default()))
                .center(Length::Fill),
        ),
    );

    let query = state.query.to_lowercase();
    let matches = FRUITS
        .iter()
        .filter(|fruit| fruit.to_lowercase().contains(&query))
        .map(|fruit| text(*fruit).into());
    let search = doc::example(
        ctx,
        SOURCE,
        &["search", "search-update"],
        frame(
            // #region search
            header_bar(Message::Window)
                .title_element(
                    search_title_field(
                        l10n,
                        text("Fruits"),
                        &state.query,
                        state.searching,
                        Message::Searching,
                    )
                    .placeholder("Search fruits")
                    .on_input(Message::Query),
                )
                .labels(Labels::new(l10n)),
            // #endregion
            scrollable(
                column(matches)
                    .spacing(spacing::XS)
                    .padding(spacing::MD)
                    .width(Length::Fill),
            ),
        ),
    );

    doc::page(
        "HeaderBar",
        "The window title bar with the application controls in it: buttons and menus at both \
         ends, a title or tabs in the middle, and a search that takes the place of the title.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Put `header_bar` at the top of an undecorated window. It is a `TitleBar`, so \
                 it shows the window buttons, drags the window from its empty area, maximizes \
                 it on a double click and opens the window menu on a right click. `start` \
                 elements sit after the leading window buttons and `end` elements before the \
                 trailing ones; the middle shows `title`, or `title_element` for any element \
                 such as tabs.",
            ),
            doc::code(ctx, "use oito2_iced::window::{self, Labels, header_bar};"),
            doc::prose(
                "The bar publishes every request as a `window::Action` through the closure it \
                 is given; the application turns it into a window task with `window::perform` \
                 (or `window::perform_latest` for a single window), and keeps a `WindowState` \
                 from the window events for `state`, which shows the restore button while \
                 maximized and dims the bar while the window is not focused.",
            ),
            doc::code(
                ctx,
                "Message::Window(action) => window::perform_latest(action),\n\n\
                 header_bar(Message::Window)\n    .title(\"Documents\")\n    \
                 .end(new_folder_button)\n    .state(self.window)\n    \
                 .labels(Labels::new(&l10n))",
            ),
            doc::note(
                "The previews on this page only record the window actions in their body, so \
                 their window buttons don't affect this window.",
            ),
            doc::prose(
                "`control_style` draws the window buttons in the look of a desktop (GNOME by \
                 default; `WindowControlStyle::None` hides them), `button_layout` chooses \
                 which buttons show and on which side, and `labels` gives their tooltips the \
                 strings of the current language.",
            ),
            doc::heading("Examples"),
            doc::subheading("Tabs"),
            doc::prose(
                "`title_element` with a `tab_bar` puts view switcher tabs in the middle in \
                 place of the title, as in GNOME applications. The tabs stay centered.",
            ),
            tabs,
            doc::subheading("Subtle tabs"),
            doc::prose(
                "`TabBarStyle::Subtle` draws the selected tab with an accent tint instead of a \
                 pill.",
            ),
            subtle,
            doc::subheading("Search"),
            doc::prose(
                "`search_title_field` switches between the title and a search field, with a \
                 toggle button at its start. Opening the search focuses the field; `Escape` \
                 clears it and, on an empty field, closes the search, giving the focus back \
                 to the toggle. The application keeps the open state and the text, so it can \
                 also open the search from a shortcut such as `Ctrl+F`.",
            ),
            search,
            doc::prose(
                "To keep a title and show a search field elsewhere, `search` places any \
                 element, usually a `search_field`, in the middle in place of the title while \
                 the application shows its search.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab`",
                        "Moves the focus through the buttons and tabs of the bar.",
                    ],
                    vec![
                        "`Enter` / `Space` on the search toggle",
                        "Opens or closes the search.",
                    ],
                    vec![
                        "`Escape` in the search field",
                        "Clears the text; on an empty field, closes the search.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::window::header_bar",
                    "oito2_iced::window::title_bar",
                    "oito2_iced::window::TitleBar",
                    "oito2_iced::window::Action",
                    "oito2_iced::window::perform_latest",
                    "oito2_iced::window::Labels",
                    "oito2_iced::widget::search::search_title_field",
                ],
            ),
        ],
    )
}
