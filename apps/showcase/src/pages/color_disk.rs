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
//! The ColorDisk page: round color swatches in a group navigated with the
//! arrow keys, larger, and disabled.

use iced::widget::row;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::focus::edge_interceptor;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::controls::color_disk;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("color_disk.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    accent: Accent,
    large: Accent,
}

impl Default for State {
    fn default() -> Self {
        Self {
            accent: Accent::Blue,
            large: Accent::Green,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// An accent of the preview was chosen.
    Accent(Accent),
    /// An accent of the larger disks was chosen.
    Large(Accent),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Accent(accent) => state.accent = accent,
        Message::Large(accent) => state.large = accent,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        edge_interceptor(
            row(Accent::ALL.map(|accent| {
                color_disk(accent.color(), accent.name())
                    .selected(state.accent == accent)
                    .on_press(Message::Accent(accent))
                    .into()
            }))
            .spacing(spacing::XS),
        ),
        // #endregion
    );

    let size = doc::example(
        ctx,
        SOURCE,
        &["size"],
        // #region size
        edge_interceptor(
            row(
                [Accent::Green, Accent::Orange, Accent::Purple].map(|accent| {
                    color_disk(accent.color(), accent.name())
                        .size(32.0)
                        .selected(state.large == accent)
                        .on_press(Message::Large(accent))
                        .into()
                }),
            )
            .spacing(spacing::SM),
        ),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        color_disk::<Message>(Accent::Slate.color(), "slate").selected(true),
        // #endregion
    );

    doc::page(
        "ColorDisk",
        "A round color swatch that can be selected, as in accent color pickers.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give each `color_disk` its color, a label that names the color and shows \
                 as its tooltip, whether it is `selected`, and an `on_press` message that \
                 selects it. A ring in the disk's own color surrounds the selected disk; a \
                 fainter one shows while the pointer hovers a disk or the keyboard focus is \
                 on it. Without `on_press` the disk is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::focus::edge_interceptor;\n\
                 use oito2_iced::widget::controls::color_disk;",
            ),
            doc::note(
                "Wrap a group of disks in an `edge_interceptor`: the arrow keys then move \
                 the focus between the disks and stop at the edges of the group, while Tab \
                 still leaves it.",
            ),
            doc::heading("Examples"),
            doc::subheading("Size"),
            doc::prose("`size` sets the disk diameter, 20 by default, without its ring."),
            size,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_press`, the disk shows its color and selection but ignores \
                 clicks, and Tab skips it.",
            ),
            disabled,
            doc::subheading("Contrast"),
            doc::prose(
                "When the color is close to the surface, the ring takes the nearest \
                 lightness of its hue with 3:1 contrast, and the disk gets a thin outline. \
                 `color_disk_ring` and `color_disk_blends` give the ring color and whether \
                 the disk needs the outline, for widgets of your own.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next disk."],
                    vec![
                        "Arrow keys",
                        "Inside an `edge_interceptor`, move the focus between the disks and \
                         stop at the edges of the group.",
                    ],
                    vec!["`Space` / `Enter`", "Selects the focused disk."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::color_disk",
                    "oito2_iced::widget::controls::ColorDisk",
                    "oito2_iced::widget::controls::COLOR_DISK_SIZE",
                    "oito2_iced::widget::controls::color_disk_ring",
                    "oito2_iced::widget::controls::color_disk_blends",
                    "oito2_iced::focus::edge_interceptor",
                ],
            ),
        ],
    )
}
