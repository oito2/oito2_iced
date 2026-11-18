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
//! The Avatar page: sizes, shapes, statuses, fallbacks and groups.

use iced::widget::{column, row};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::avatar::{self, Shape, Status};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("avatar.rs");

// #region people
/// The people the avatars show.
const PEOPLE: [&str; 6] = [
    "Ana Souza",
    "Bruno Lima",
    "Carla Dias",
    "Diego Rocha",
    "Elisa Prado",
    "Fábio Nunes",
];
// #endregion

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {}
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let ana = avatar::avatar()
        .name("Ana Souza")
        .status(Status::Online)
        .size(56.0);
    let ana_description = ana.description(l10n);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            // #region preview
            avatar::avatar()
                .name("Ana Souza")
                .status(Status::Online)
                .size(56.0),
            // #endregion
            doc::caption(ana_description),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    );

    let statuses = doc::example(
        ctx,
        SOURCE,
        &["people", "statuses"],
        // #region statuses
        row(
            [Status::Online, Status::Away, Status::Busy, Status::Offline]
                .into_iter()
                .zip(PEOPLE)
                .enumerate()
                .map(|(index, (status, name))| {
                    avatar::avatar()
                        .name(name)
                        .status(status)
                        .size(32.0 + index as f32 * 8.0)
                        .into()
                }),
        )
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
        // #endregion
    );

    let fallbacks = doc::example(
        ctx,
        SOURCE,
        &["fallbacks"],
        row![
            // #region fallbacks
            avatar::avatar().name("Grupo oito2").shape(Shape::Square),
            avatar::avatar(),
            avatar::avatar().icon(lucide::USERS),
            avatar::avatar()
                .name("Helena Costa")
                .background(iced::Color::from_rgb8(0x9A, 0x3E, 0xA8)),
            // #endregion
        ]
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
    );

    let group = doc::example(
        ctx,
        SOURCE,
        &["people", "group"],
        // #region group
        avatar::group(PEOPLE.map(|name| avatar::avatar().name(name).size(32.0))),
        // #endregion
    );

    let more = avatar::more_description(PEOPLE.len() - avatar::DEFAULT_MAX, l10n);

    let squares = doc::example(
        ctx,
        SOURCE,
        &["squares"],
        // #region squares
        avatar::group(
            PEOPLE.map(|name| avatar::avatar().name(name).size(40.0).shape(Shape::Square)),
        )
        .max(3)
        .size(40.0)
        .overlap(0.2)
        .shape(Shape::Square),
        // #endregion
    );

    doc::page(
        "Avatar",
        "A picture of a person or thing, falling back to initials or an icon, with an \
         optional presence status.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`avatar` shows the initials of its `name`, the first letters of its first \
                 and last words, on a color picked from the name, so the same name always \
                 gets the same color. Without a name it shows its `icon`, a person by \
                 default, on a neutral surface. `size` sets the width and height, 40 pixels \
                 by default.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::avatar::{self, Shape, Status};",
            ),
            doc::note(
                "With the crate feature `image`, `image` sets a raster picture, drawn over \
                 the initials and cropped to cover the avatar; the initials show while it \
                 loads. This showcase builds without that feature.",
            ),
            doc::heading("Examples"),
            doc::subheading("Statuses and sizes"),
            doc::prose(
                "A `Status` adds a presence dot at the bottom end: `Online`, `Away`, `Busy` \
                 or `Offline`. The dot grows with the avatar.",
            ),
            statuses,
            doc::subheading("Shape and fallbacks"),
            doc::prose(
                "`Shape::Square` rounds the corners instead of making a circle. Without a \
                 name the avatar shows its `icon`; `background` replaces the color picked \
                 from the name.",
            ),
            fallbacks,
            doc::subheading("Group"),
            doc::prose(
                "`avatar::group` overlaps its avatars in a row, each with a ring in the \
                 background color, and shows at most `max`, four by default, followed by a \
                 \"+N\" bubble for the rest. Give it the `size` and `shape` of the avatars, \
                 32 pixels and a circle by default.",
            ),
            group,
            doc::prose("`overlap` is the part of each avatar the next one covers, 0.3 by default."),
            squares,
            doc::heading("Descriptions"),
            doc::prose(
                "Iced has no accessibility tree, so the avatar draws no text for assistive \
                 technologies. `Avatar::description` and `description` give the localized \
                 text that describes an avatar, such as the caption of the preview, and \
                 `more_description` the text of the bubble, for tooltips:",
            ),
            doc::caption(more),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::avatar::avatar",
                    "oito2_iced::widget::avatar::Avatar",
                    "oito2_iced::widget::avatar::group",
                    "oito2_iced::widget::avatar::AvatarGroup",
                    "oito2_iced::widget::avatar::Status",
                    "oito2_iced::widget::avatar::Shape",
                    "oito2_iced::widget::avatar::initials",
                    "oito2_iced::widget::avatar::color_for",
                    "oito2_iced::widget::avatar::description",
                    "oito2_iced::widget::avatar::more_description",
                ],
            ),
        ],
    )
}
