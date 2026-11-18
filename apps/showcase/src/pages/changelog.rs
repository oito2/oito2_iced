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

//! The Changelog page: the changelog of every published crate.

use iced::widget::column;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The changelog of every published crate, in the order of the page.
const CHANGELOGS: [(&str, &str); 6] = [
    (
        "oito2_iced",
        include_str!("../../../../crates/oito2_iced/CHANGELOG.md"),
    ),
    (
        "oito2_iced_settings",
        include_str!("../../../../crates/oito2_iced_settings/CHANGELOG.md"),
    ),
    (
        "oito2_iced_themes",
        include_str!("../../../../crates/oito2_iced_themes/CHANGELOG.md"),
    ),
    (
        "oito2_iced_syntax",
        include_str!("../../../../crates/oito2_iced_syntax/CHANGELOG.md"),
    ),
    (
        "oito2_iced_extras",
        include_str!("../../../../crates/oito2_iced_extras/CHANGELOG.md"),
    ),
    (
        "oito2_iced_files",
        include_str!("../../../../crates/oito2_iced_files/CHANGELOG.md"),
    ),
];

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
    doc::page(
        "Changelog",
        "The changes of every crate, newest first.",
        CHANGELOGS.iter().map(|(name, changelog)| {
            column![doc::heading(name)]
                .extend(doc::markdown(ctx, changelog))
                .spacing(spacing::MD)
                .into()
        }),
    )
}
