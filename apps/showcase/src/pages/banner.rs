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
//! The Banner page: tiles, selectable cards, a watermark and tiles that
//! change on hover, carry a tint and open a dialog.

use iced::widget::{column, container, row, svg};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{Icon, icon, lucide};
use oito2_iced::theme::svg::Class as SvgClass;
use oito2_iced::tokens::{Accent, spacing, typography};
use oito2_iced::widget::containers::watermark;
use oito2_iced::widget::info::{banner, banner_tile};
use oito2_iced::widget::styled_text;

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("banner.rs");

// #region apps
/// The sample apps: a title, a description and an icon.
const APPS: [(&str, &str, Icon); 6] = [
    ("Weather", "Forecasts for your places", lucide::SUN),
    ("Music", "Play your library", lucide::HEADPHONES),
    ("Photos", "Browse and edit pictures", lucide::IMAGE),
    ("Maps", "Find places and routes", lucide::MAP),
    ("Mail", "Read and send messages", lucide::MAIL),
    ("Notes", "Write things down", lucide::NOTEBOOK_PEN),
];
// #endregion

/// The selectable apps: a title and a subtitle.
const CHOICES: [(&str, &str); 3] = [
    ("Text Editor", "Edit plain text files"),
    ("Files", "Access and organize files"),
    ("Calculator", "Perform calculations"),
];

/// The longer text a hovered tile shows.
const DETAILS: &str = "Shows more on hover: the icon steps aside and the description grows \
                       to several lines. A click opens a dialog.";

/// The state of the page.
#[derive(Debug)]
pub struct State {
    selected: usize,
    hovered: Option<usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected: 1,
            hovered: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A banner without a reaction of its own was pressed.
    Pressed,
    /// A selectable banner was pressed.
    Select(usize),
    /// The pointer entered (true) or left (false) a hover tile.
    Hover(usize, bool),
    /// A hover tile was pressed.
    Open,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pressed => {}
        Message::Select(index) => state.selected = index,
        Message::Hover(index, true) => state.hovered = Some(index),
        Message::Hover(index, false) => {
            if state.hovered == Some(index) {
                state.hovered = None;
            }
        }
        Message::Open => shell.request(Action::OpenDialog),
    }

    Task::none()
}

/// A large app icon in the primary color.
fn app_icon<'a>(icon: Icon) -> Element<'a, Message, Theme> {
    svg(icon.handle())
        .width(48)
        .height(48)
        .class(SvgClass::Primary)
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            banner_tile("Weather")
                .subtitle("Forecasts for your places")
                .icon(app_icon(lucide::SUN))
                .width(Length::Fill)
                .on_press(Message::Pressed),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    let selectable = doc::example(
        ctx,
        SOURCE,
        &["selectable"],
        // #region selectable
        row(CHOICES
            .iter()
            .enumerate()
            .map(|(index, (title, subtitle))| {
                banner(column![
                    styled_text(*title, typography::TITLE_MEDIUM),
                    styled_text(*subtitle, typography::BODY_MEDIUM),
                ])
                .selected(state.selected == index)
                .width(Length::Fixed(240.0))
                .on_press(Message::Select(index))
                .into()
            }))
        .spacing(spacing::MD)
        .wrap()
        .vertical_spacing(spacing::MD),
        // #endregion
    );

    let watermarked = doc::example(
        ctx,
        SOURCE,
        &["apps", "watermark"],
        // #region watermark
        row(APPS.iter().take(2).map(|(title, description, mark)| {
            banner(watermark(
                icon(*mark).size(96.0),
                column![
                    styled_text(*title, typography::TITLE_MEDIUM),
                    styled_text(*description, typography::BODY_MEDIUM),
                ]
                .spacing(spacing::XS)
                .padding(spacing::LG)
                .width(Length::Fill)
                .height(Length::Fixed(140.0)),
            ))
            .padding(0)
            .width(Length::Fixed(320.0))
            .on_press(Message::Pressed)
            .into()
        }))
        .spacing(spacing::MD)
        .wrap()
        .vertical_spacing(spacing::MD),
        // #endregion
    );

    let tiles = row(APPS
        .iter()
        .enumerate()
        .map(|(index, (title, description, mark))| {
            let tint = match index % 3 {
                1 => Some(Accent::Purple.color()),
                2 => Some(Accent::Teal.color()),
                _ => None,
            };
            let hovered = state.hovered == Some(index);

            // #region hover-tile
            let mut tile = banner_tile(*title)
                .subtitle(if hovered { DETAILS } else { *description })
                .width(Length::Fixed(320.0))
                .on_hover(move |inside| Message::Hover(index, inside))
                .on_press(Message::Open);

            if !hovered {
                tile = tile.icon(match tint {
                    Some(color) => icon(*mark).size(48.0).color(color).into(),
                    None => app_icon(*mark),
                });
            }

            if let Some(tint) = tint {
                tile = tile.tint(tint);
            }
            // #endregion

            tile.into()
        }))
    .spacing(spacing::MD)
    .wrap()
    .vertical_spacing(spacing::MD);

    let hover = doc::example(ctx, SOURCE, &["apps", "hover-tile"], tiles);

    doc::page(
        "Banner",
        "A card-like clickable surface for promoting content, such as an app in a store \
         grid.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`banner` draws its content on a bordered surface with large rounded \
                 corners and 16 pixels of padding. With `on_press` it is a button whose \
                 border strengthens on hover. `banner_tile` lays out an `icon` before a \
                 title and an optional `subtitle`.",
            ),
            doc::code(ctx, "use oito2_iced::widget::info::{banner, banner_tile};"),
            doc::note(
                "For a choice without a border at rest, such as an option in a picker, use \
                 a selectable container.",
            ),
            doc::heading("Examples"),
            doc::subheading("Selectable"),
            doc::prose(
                "With `selected`, the banner takes a tinted background and a 2-pixel border \
                 in the indicator color. Keep the selection in your state and change it in \
                 the press message.",
            ),
            selectable,
            doc::subheading("With a watermark"),
            doc::prose(
                "A banner takes any content. Here a `watermark` fills it, with \
                 `padding(0)` on the banner so the watermark reaches its edges.",
            ),
            watermarked,
            doc::subheading("Hover, tint and a dialog"),
            doc::prose(
                "`on_hover` publishes `true` when the pointer enters the banner and `false` \
                 when it leaves, so the content can change on hover: these tiles hide their \
                 icon and show a longer description. `tint` washes the background lightly \
                 with a color, and `color` replaces the background. A click opens a \
                 dialog.",
            ),
            hover,
            doc::note(
                "Content that only appears on hover cannot be reached without a pointer, so \
                 keep it available another way, as the dialog of these tiles does.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next banner."],
                    vec!["`Enter` / `Space`", "Publishes the press message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::info::banner",
                    "oito2_iced::widget::info::banner_tile",
                    "oito2_iced::widget::info::Banner",
                ],
            ),
        ],
    )
}
