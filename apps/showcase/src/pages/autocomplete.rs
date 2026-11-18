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
//! The Autocomplete page: a text field that suggests the items of a
//! collection.

use iced::widget::{column, container};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::collections::ListCollection;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::autocomplete::autocomplete;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("autocomplete.rs");

/// A country: its code and its name.
type Country = (&'static str, &'static str);

// #region countries
/// The countries the fields suggest, valued by their code and labeled by
/// their name; `None` when two codes repeat.
fn countries() -> Option<ListCollection<Country>> {
    ListCollection::with_value_of(
        [
            ("ar", "Argentina"),
            ("bo", "Bolívia"),
            ("br", "Brasil"),
            ("cl", "Chile"),
            ("co", "Colômbia"),
            ("ec", "Equador"),
            ("py", "Paraguai"),
            ("pe", "Peru"),
            ("uy", "Uruguai"),
            ("ve", "Venezuela"),
        ],
        |(code, _)| (*code).to_owned(),
    )
    .map(|countries| countries.with_label_of(|(_, name)| (*name).to_owned()))
    .ok()
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    countries: Option<ListCollection<Country>>,
    country: String,
    picked: Option<&'static str>,
    browsed_text: String,
    browsed: Option<&'static str>,
    home: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            countries: countries(),
            country: String::new(),
            picked: None,
            browsed_text: String::new(),
            browsed: None,
            home: "Brasil".to_owned(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The text of the first field changed.
    Country(String),
    /// A country was picked in the first field.
    Picked(Country),
    /// The text of the browsing field changed.
    BrowsedText(String),
    /// A country was picked in the browsing field.
    Browsed(Country),
    /// The text of the field with an initial value changed.
    Home(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Country(text) => state.country = text,
        Message::Picked((_, name)) => state.picked = Some(name),
        Message::BrowsedText(text) => state.browsed_text = text,
        Message::Browsed((_, name)) => state.browsed = Some(name),
        Message::Home(text) => state.home = text,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let Some(countries) = &state.countries else {
        return doc::page("Autocomplete", "The countries repeat a code.", []);
    };

    let picked = |name: Option<&str>| doc::caption(format!("Picked: {}", name.unwrap_or("—")));

    let preview = doc::example(
        ctx,
        SOURCE,
        &["countries", "preview"],
        container(
            column![
                // #region preview
                autocomplete("Country", &state.country, countries)
                    .on_input(Message::Country)
                    .on_select(Message::Picked),
                // #endregion
                picked(state.picked),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(300.0)),
    );

    let show_all = doc::example(
        ctx,
        SOURCE,
        &["show-all"],
        container(
            column![
                // #region show-all
                autocomplete("Country", &state.browsed_text, countries)
                    .show_all_when_empty(true)
                    .on_input(Message::BrowsedText)
                    .on_select(Message::Browsed),
                // #endregion
                picked(state.browsed),
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(300.0)),
    );

    let initial = doc::example(
        ctx,
        SOURCE,
        &["initial"],
        container(
            // #region initial
            autocomplete("Country", &state.home, countries)
                .label("Home country")
                .helper("Where you live")
                .on_input(Message::Home),
            // #endregion
        )
        .width(Length::Fixed(300.0)),
    );

    doc::page(
        "Autocomplete",
        "A text field that suggests items of a collection as you type, and fills in the one \
         you pick.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `autocomplete` a placeholder, the current text and a `ListCollection` \
                 of the items to suggest. The suggestions are the enabled items whose label \
                 contains the typed text, ignoring case and accents, with the match \
                 highlighted: typing \"colo\" finds \"Colômbia\". Picking one publishes its \
                 label through `on_input` and the item through `on_select`.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::collections::ListCollection;\n\
                 use oito2_iced::widget::autocomplete::autocomplete;",
            ),
            doc::prose(
                "The list grows up to `options_max_height`, 240 by default, before it \
                 scrolls, and is as wide as the field unless `options_width` is set. \
                 `suggestions` replaces the matching items with a list of your own, and \
                 `on_submit` publishes a message on Enter while no suggestion shows. \
                 Without `on_input` the field is disabled.",
            ),
            doc::heading("Examples"),
            doc::subheading("Every option while empty"),
            doc::prose(
                "With `show_all_when_empty`, every enabled item shows while the field is \
                 empty, so the list can be browsed before typing.",
            ),
            show_all,
            doc::subheading("Initial value, label and helper"),
            doc::prose(
                "The text lives in the application, so a field starts filled when its text \
                 does. `label` and `helper` show above and below the field, as on a text \
                 field.",
            ),
            initial,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Up` / `Down`",
                        "Moves the highlight through the suggestions.",
                    ],
                    vec!["`Enter`", "Picks the highlighted suggestion."],
                    vec!["`Escape`", "Closes the suggestions until the text changes."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::autocomplete::autocomplete",
                    "oito2_iced::widget::autocomplete::Autocomplete",
                    "oito2_iced::widget::autocomplete::OPTIONS_MAX_HEIGHT",
                    "oito2_iced::collections::ListCollection",
                ],
            ),
        ],
    )
}
