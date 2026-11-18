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
//! The AngleSlider page: a round dial that picks an angle.

use iced::widget::row;
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::angle_slider::{angle_slider, angle_text};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("angle_slider.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    direction: f32,
    angle: f32,
    hue: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            direction: 90.0,
            angle: 45.0,
            hue: 200.0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The direction dial turned.
    Direction(f32),
    /// The rotation dial turned.
    Angle(f32),
    /// The hue dial turned.
    Hue(f32),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Direction(direction) => state.direction = direction,
        Message::Angle(angle) => state.angle = angle,
        Message::Hue(hue) => state.hue = hue,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        angle_slider(state.direction)
            .localizations(l10n)
            .on_change(Message::Direction),
        // #endregion
    );

    let steps = doc::example(
        ctx,
        SOURCE,
        &["steps"],
        row![
            // #region steps
            angle_slider(state.angle)
                .step(5.0)
                .markers([0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0])
                .localizations(l10n)
                .on_change(Message::Angle),
            doc::title(angle_text(state.angle, l10n)),
            // #endregion
        ]
        .spacing(spacing::XL)
        .align_y(Alignment::Center),
    );

    let compact = doc::example(
        ctx,
        SOURCE,
        &["compact"],
        // #region compact
        angle_slider(state.hue)
            .size(48.0)
            .show_value(false)
            .markers([])
            .on_change(Message::Hue),
        // #endregion
    );

    let read_only = doc::example(
        ctx,
        SOURCE,
        &["read-only"],
        // #region read-only
        angle_slider(135.0).localizations(l10n),
        // #endregion
    );

    doc::page(
        "AngleSlider",
        "A round dial that picks an angle in degrees, such as a rotation or a gradient \
         direction.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The dial is controlled: give `angle_slider` the value and update it in \
                 `on_change`. Angles run from 0 at the top, clockwise, to just under 360, \
                 and wrap around. Dragging or clicking points the dial at the pointer. \
                 `on_change_end` publishes the final angle when a drag or click ends, and \
                 after each key press. `localizations` writes the angle in the number \
                 format of the language.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::angle_slider::{angle_slider, angle_text};",
            ),
            doc::heading("Examples"),
            doc::subheading("Steps and markers"),
            doc::prose(
                "`step` is the increment the angle snaps to, 1 by default, and the turn of \
                 one arrow key; `page_step`, 15 by default, is the turn of Page Up and Page \
                 Down. `markers` draws ticks at the given angles, the four quarter turns by \
                 default. `angle_text` writes an angle as the dial does.",
            ),
            steps,
            doc::subheading("Size and value"),
            doc::prose(
                "`size` is the dial diameter, 96 by default. With `show_value(false)` the \
                 angle is not written in the middle, which suits a small dial.",
            ),
            compact,
            doc::subheading("Read-only"),
            doc::prose(
                "Without `on_change` the dial is read-only: it ignores input, Tab skips it \
                 and its hand greys out.",
            ),
            read_only,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Up` / `Right`", "Turns the dial clockwise by `step`."],
                    vec![
                        "`Down` / `Left`",
                        "Turns the dial counterclockwise by `step`.",
                    ],
                    vec!["`Page Up` / `Page Down`", "Turns the dial by `page_step`."],
                    vec!["`Home`", "Sets the angle to 0."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::angle_slider::angle_slider",
                    "oito2_iced::widget::angle_slider::AngleSlider",
                    "oito2_iced::widget::angle_slider::angle_text",
                ],
            ),
        ],
    )
}
