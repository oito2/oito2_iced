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

//! The MasterDetailPage page: the adaptive layout, the narrow layout, a
//! fixed pane with a footer and an empty list.

use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{self, Icon, lucide};
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::master_detail::{detail_page, master_detail, master_tile};
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("master_detail_page.rs");

// #region data
/// The settings sections: icon, title and summary.
const SETTINGS: [(Icon, &str, &str); 5] = [
    (lucide::WIFI, "Network", "Wi-Fi, VPN and proxy"),
    (lucide::BLUETOOTH, "Bluetooth", "Devices and pairing"),
    (lucide::PALETTE, "Appearance", "Style, accent and wallpaper"),
    (lucide::BELL, "Notifications", "Banners and sounds"),
    (lucide::LOCK, "Privacy", "Location, camera and history"),
];
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    preview: Option<usize>,
    preview_pane: f32,
    narrow: Option<usize>,
    fixed: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            preview: None,
            preview_pane: 240.0,
            narrow: None,
            fixed: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A section of the preview was selected, or its page closed.
    Preview(Option<usize>),
    /// The pane of the preview was resized.
    PreviewPane(f32),
    /// A section of the narrow example was selected, or its page closed.
    Narrow(Option<usize>),
    /// A section of the fixed example was selected, or its page closed.
    Fixed(Option<usize>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Preview(selected) => state.preview = selected,
        Message::PreviewPane(size) => state.preview_pane = size,
        // #endregion
        Message::Narrow(selected) => state.narrow = selected,
        Message::Fixed(selected) => state.fixed = selected,
    }

    Task::none()
}

/// A bordered frame of `width` by `height` around an example.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: Length,
    height: f32,
) -> Element<'a, Message, Theme> {
    border_container(content)
        .width(width)
        .height(Length::Fixed(height))
        .clip(true)
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = ctx.l10n;
    let direction = ctx.direction;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["data", "preview", "update"],
        frame(
            // #region preview
            master_detail(
                SETTINGS.len(),
                state.preview,
                |index, selected, _width| {
                    let (glyph, title, _) = SETTINGS[index];
                    master_tile(selected, Message::Preview(Some(index)))
                        .leading(icon::icon(glyph))
                        .title(title)
                        .into()
                },
                move |index, back| {
                    let (glyph, title, summary) = SETTINGS[index];
                    detail_page(
                        column![
                            icon::icon(glyph).size(48.0),
                            styled_text(summary, typography::BODY_LARGE),
                        ]
                        .spacing(spacing::MD)
                        .padding(spacing::XL),
                    )
                    .title(title)
                    .back(back)
                    .localizations(&l10n)
                    .direction(direction)
                    .into()
                },
            )
            .on_back(Message::Preview(None))
            .pane_size(state.preview_pane)
            .on_resize(Message::PreviewPane)
            .direction(direction),
            // #endregion
            Length::Fill,
            340.0,
        ),
    );

    let narrow = doc::example(
        ctx,
        SOURCE,
        &["narrow"],
        frame(
            // #region narrow
            master_detail(
                SETTINGS.len(),
                state.narrow,
                |index, selected, _width| {
                    let (glyph, title, summary) = SETTINGS[index];
                    master_tile(selected, Message::Narrow(Some(index)))
                        .leading(icon::icon(glyph))
                        .title(title)
                        .subtitle(summary)
                        .into()
                },
                move |index, back| {
                    let (_, title, summary) = SETTINGS[index];
                    detail_page(container(text(summary)).center(Length::Fill))
                        .title(title)
                        .back(back)
                        .localizations(&l10n)
                        .direction(direction)
                        .into()
                },
            )
            .on_back(Message::Narrow(None))
            .direction(direction),
            // #endregion
            Length::Fixed(360.0),
            340.0,
        ),
    );

    let footer = doc::example(
        ctx,
        SOURCE,
        &["footer"],
        frame(
            // #region footer
            master_detail(
                SETTINGS.len(),
                state.fixed,
                |index, selected, _width| {
                    master_tile(selected, Message::Fixed(Some(index)))
                        .title(SETTINGS[index].1)
                        .into()
                },
                move |index, back| {
                    let (_, title, summary) = SETTINGS[index];
                    detail_page(container(text(summary)).center(Length::Fill))
                        .title(title)
                        .back(back)
                        .localizations(&l10n)
                        .into()
                },
            )
            .on_back(Message::Fixed(None))
            .resizable(false)
            .breakpoint(480.0)
            .pane_size(200.0)
            .min_page_size(200.0)
            .master_footer(|| {
                container(doc::caption(format!("{} sections", SETTINGS.len())))
                    .padding(spacing::MD)
                    .into()
            }),
            // #endregion
            Length::Fixed(520.0),
            300.0,
        ),
    );

    let empty = doc::example(
        ctx,
        SOURCE,
        &["empty"],
        frame(
            // #region empty
            master_detail(0, None, |_, _, _| text("").into(), |_, _| text("").into()).empty(|| {
                container(text("No settings match"))
                    .center(Length::Fill)
                    .into()
            }),
            // #endregion
            Length::Fixed(360.0),
            200.0,
        ),
    );

    doc::page(
        "MasterDetailPage",
        "A list of items beside the page of the selected one, as in settings applications, \
         that shows one side at a time when narrow.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `master_detail` the number of items, the selected one, a builder of the \
                 tile of an item and a builder of its page. The tile builder receives the \
                 index, whether it is selected and the pane width; `master_tile` is a list \
                 tile that publishes its press message and shows the selection. The page \
                 builder receives the index and the back message, which `detail_page` turns \
                 into the back button of its bar.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::master_detail::{detail_page, master_detail, \
                 master_tile};",
            ),
            doc::prose(
                "At or above `breakpoint` wide, 620 pixels by default, the list is a resizable \
                 pane beside the page, which shows the first item while nothing is selected. \
                 Below it, the list fills the area until an item is selected; the page then \
                 replaces it, and its back button publishes `on_back`, on which the \
                 application clears the selection. The selection and the pane width live in \
                 the application, so the selection survives switching between the layouts.",
            ),
            doc::heading("Examples"),
            doc::subheading("Narrow layout"),
            doc::prose(
                "In an area narrower than the breakpoint, the list shows alone while nothing \
                 is selected. Select an item to show its page, then use the back button of \
                 its bar to return to the list.",
            ),
            narrow,
            doc::subheading("Fixed pane and footer"),
            doc::prose(
                "With `resizable(false)` the pane keeps `pane_size`. `master_footer` sits \
                 below the list, and `master_bar` above it, such as a title bar with a search \
                 field.",
            ),
            footer,
            doc::subheading("Empty list"),
            doc::prose(
                "When there are no items, `empty` replaces the list. A `master_bar` and a \
                 `master_footer` stay around it, so a search that matches nothing can still \
                 be cleared.",
            ),
            empty,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab`",
                        "Moves the focus through the tiles and the divider.",
                    ],
                    vec!["`Enter` / `Space`", "Selects the focused tile."],
                    vec![
                        "Arrow keys, `Home` / `End`",
                        "Move the focused divider, when the pane is resizable.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::master_detail::master_detail",
                    "oito2_iced::widget::master_detail::MasterDetail",
                    "oito2_iced::widget::master_detail::master_tile",
                    "oito2_iced::widget::master_detail::detail_page",
                    "oito2_iced::widget::master_detail::DetailPage",
                    "oito2_iced::widget::master_detail::DEFAULT_BREAKPOINT",
                ],
            ),
        ],
    )
}
