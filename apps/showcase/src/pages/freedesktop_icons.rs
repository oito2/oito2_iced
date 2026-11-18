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

//! The FreedesktopIcons page: icons named after the freedesktop.org Icon
//! Naming Specification, drawn with Lucide icons, and the Lucide icons
//! themselves.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{Freedesktop, Icon, icon, lucide, placeholder};
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::highlighted_text::highlighted_text;
use oito2_iced::widget::search::search_field;
use oito2_iced::widget::tooltip;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("freedesktop_icons.rs");

/// The names of the lookup example.
const LOOKUPS: [&str; 4] = [
    "dialog-warning",
    "edit-copy-symbolic",
    "folder-documents",
    "unknown-name",
];

/// A sample of the Lucide icons.
const LUCIDE: [Icon; 24] = [
    lucide::HOUSE,
    lucide::SEARCH,
    lucide::SETTINGS,
    lucide::BELL,
    lucide::USER,
    lucide::MAIL,
    lucide::CALENDAR,
    lucide::CLOCK,
    lucide::FOLDER,
    lucide::FILE,
    lucide::DOWNLOAD,
    lucide::UPLOAD,
    lucide::TRASH,
    lucide::PENCIL,
    lucide::COPY,
    lucide::SHARE,
    lucide::HEART,
    lucide::STAR,
    lucide::LOCK,
    lucide::EYE,
    lucide::SUN,
    lucide::MOON,
    lucide::WIFI,
    lucide::BLUETOOTH,
];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    query: String,
    lucide: String,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The search of the freedesktop names changed.
    Query(String),
    /// The search of the Lucide sample changed.
    Lucide(String),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Query(query) => state.query = query,
        Message::Lucide(query) => state.lucide = query,
    }

    Task::none()
}

/// An icon above its name with the matches of `query` highlighted.
fn cell<'a>(glyph: Icon, name: &'static str, query: &str) -> Element<'a, Message, Theme> {
    column![
        icon(glyph),
        highlighted_text(name, query).style(typography::BODY_SMALL),
    ]
    .spacing(spacing::XS)
    .align_x(Alignment::Center)
    .width(Length::Fixed(140.0))
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        row![
            icon(Freedesktop::DocumentOpen.icon()),
            icon(Freedesktop::DocumentSave.icon()),
            icon(Freedesktop::EditCopy.icon()),
            icon(Freedesktop::UserTrash.icon()),
            icon(Freedesktop::DialogWarning.icon()),
        ]
        .spacing(spacing::LG),
        // #endregion
    );

    let lookup = doc::example(
        ctx,
        SOURCE,
        &["lookup"],
        column(
            // #region lookup
            LOOKUPS.map(|name| {
                let glyph = Freedesktop::by_name(name)
                    .map_or(lucide::CIRCLE_QUESTION_MARK, Freedesktop::icon);

                row![icon(glyph), text(name)]
                    .spacing(spacing::SM)
                    .align_y(Alignment::Center)
                    .into()
            }),
            // #endregion
        )
        .spacing(spacing::SM)
        .width(Length::Fixed(320.0)),
    );

    // #region search-data
    let words: Vec<String> = state
        .query
        .to_lowercase()
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    let names: Vec<Freedesktop> = Freedesktop::ALL
        .into_iter()
        .filter(|named| {
            words
                .iter()
                .all(|word| named.name().contains(word.as_str()))
        })
        .collect();
    // #endregion

    let search = doc::example(
        ctx,
        SOURCE,
        &["search-data", "search"],
        column![
            container(
                search_field(&ctx.l10n, &state.query)
                    .placeholder("document-open")
                    .on_input(Message::Query)
            )
            .width(Length::Fixed(300.0)),
            doc::caption(format!("{} icons", names.len())),
            // #region search
            row(names.into_iter().map(|named| {
                tooltip(
                    cell(named.icon(), named.name(), &state.query),
                    format!("Freedesktop::{named:?}"),
                )
                .into()
            }))
            .spacing(spacing::SM)
            .wrap()
            .vertical_spacing(spacing::MD),
            // #endregion
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let sample: Vec<Icon> = LUCIDE
        .into_iter()
        .filter(|glyph| glyph.name().contains(state.lucide.trim()))
        .collect();

    let lucide_sample = doc::example(
        ctx,
        SOURCE,
        &["lucide"],
        column![
            container(
                search_field(&ctx.l10n, &state.lucide)
                    .placeholder("folder")
                    .on_input(Message::Lucide)
            )
            .width(Length::Fixed(300.0)),
            row(
                // #region lucide
                sample
                    .into_iter()
                    .map(|glyph| cell(glyph, glyph.name(), &state.lucide)),
                // #endregion
            )
            .spacing(spacing::SM)
            .wrap()
            .vertical_spacing(spacing::MD),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let sizes = doc::example(
        ctx,
        SOURCE,
        &["sizes"],
        // #region sizes
        row![
            icon(lucide::STAR).size(16.0),
            icon(lucide::STAR),
            icon(lucide::STAR).size(24.0),
            icon(lucide::STAR).size(32.0),
            icon(lucide::STAR).size(48.0),
            placeholder().size(48.0),
        ]
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
        // #endregion
    );

    doc::page(
        "FreedesktopIcons",
        "Icons named after the freedesktop.org Icon Naming Specification, drawn with the \
         Lucide icons of oito2_iced.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Each `Freedesktop` value has its specification name, from `name`, such as \
                 `document-open`, and the Lucide `Icon` that draws it, from `icon`. Draw it with \
                 the `icon` widget, which takes the text color of its parent.",
            ),
            doc::code(ctx, "use oito2_iced::icon::{Freedesktop, icon};"),
            doc::note(
                "Use these names when an icon comes from a Linux integration, such as a \
                 `.desktop` file, a notification or a tray item. For icons chosen in code, \
                 `oito2_iced::icon::lucide` has a constant for every Lucide icon.",
            ),
            doc::heading("Examples"),
            doc::subheading("Lookup by name"),
            doc::prose(
                "`Freedesktop::by_name` turns a specification name into an icon, or `None` when \
                 nothing matches. It ignores trailing `-symbolic`, `-rtl` and `-ltr` suffixes, \
                 and, as the specification does, falls back to the name without its last \
                 dash-separated part: `folder-documents` to `folder`.",
            ),
            lookup,
            doc::subheading("All names"),
            text(format!(
                "The {} names, filtered by every word typed in the search field. Rest the \
                 pointer on an icon to see its variant.",
                Freedesktop::ALL.len()
            ))
            .size(typography::BODY_MEDIUM.size)
            .into(),
            search,
            doc::subheading("Lucide icons"),
            doc::prose(
                "Every Lucide icon is a constant of `lucide`, named after the icon: \
                 `folder-open` is `lucide::FOLDER_OPEN`. Below, a sample of them filtered by \
                 name.",
            ),
            lucide_sample,
            doc::subheading("Sizes and placeholder"),
            doc::prose(
                "`size` sets the side of an icon, 20 pixels by default. `placeholder` stands \
                 in for a missing picture.",
            ),
            sizes,
            doc::api(
                ctx,
                &[
                    "oito2_iced::icon::Freedesktop",
                    "oito2_iced::icon::Icon",
                    "oito2_iced::icon::icon",
                    "oito2_iced::icon::Glyph",
                    "oito2_iced::icon::placeholder",
                    "oito2_iced::icon::lucide",
                ],
            ),
        ],
    )
}
