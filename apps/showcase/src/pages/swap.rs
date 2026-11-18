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

//! The Swap page: each effect and a custom duration.

use std::time::Duration;

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::swap::{SwapEffect, swap};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("swap.rs");

/// The effects, with their names.
const EFFECTS: [(SwapEffect, &str); 4] = [
    (SwapEffect::Fade, "Fade"),
    (SwapEffect::Scale, "Scale"),
    (SwapEffect::Rotate, "Rotate"),
    (SwapEffect::Flip, "Flip"),
];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    playing: bool,
    swapped: bool,
    dark: bool,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview was pressed.
    Playing(bool),
    /// A button of the effects was pressed.
    Swapped(bool),
    /// The dark style button was pressed.
    Dark(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Playing(playing) => state.playing = playing,
        Message::Swapped(swapped) => state.swapped = swapped,
        Message::Dark(dark) => state.dark = dark,
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
        button::button(swap(state.playing, icon(lucide::PLAY), icon(lucide::PAUSE)))
            .class(ButtonClass::Tonal)
            .tooltip(if state.playing { "Pause" } else { "Play" })
            .on_press(Message::Playing(!state.playing)),
        // #endregion
    );

    let effects = doc::example(
        ctx,
        SOURCE,
        &["effects"],
        // #region effects
        row(EFFECTS.map(|(effect, name)| {
            column![
                button::button(
                    swap(state.swapped, icon(lucide::PLAY), icon(lucide::PAUSE)).effect(effect)
                )
                .class(ButtonClass::Tonal)
                .on_press(Message::Swapped(!state.swapped)),
                text(name),
            ]
            .spacing(spacing::XS)
            .align_x(Alignment::Center)
            .into()
        }))
        .spacing(spacing::MD)
        .wrap(),
        // #endregion
    );

    let duration = doc::example(
        ctx,
        SOURCE,
        &["duration"],
        // #region duration
        button::button(
            swap(state.dark, icon(lucide::SUN), icon(lucide::MOON))
                .effect(SwapEffect::Rotate)
                .duration(Duration::from_millis(500)),
        )
        .class(ButtonClass::Tonal)
        .tooltip("Dark style")
        .on_press(Message::Dark(!state.dark)),
        // #endregion
    );

    let label = doc::example(
        ctx,
        SOURCE,
        &["label"],
        // #region label
        button::button(swap(state.playing, text("Play"), text("Pause")).effect(SwapEffect::Scale))
            .on_press(Message::Playing(!state.playing)),
        // #endregion
    );

    doc::page(
        "Swap",
        "Animates between two elements, such as a play button that becomes pause.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`swap` shows `on`, its third argument, while `swapped` is true and `off` \
                 otherwise. Change `swapped` and the elements change with a transition. The \
                 swap only animates: put it inside a button, or another widget that handles \
                 the clicks.",
            ),
            doc::code(ctx, "use oito2_iced::widget::swap::{SwapEffect, swap};"),
            doc::heading("Examples"),
            doc::subheading("Effects"),
            doc::prose(
                "`effect` picks the transition: `SwapEffect::Fade`, the default; \
                 `SwapEffect::Scale` grows the new element in while the old one shrinks away; \
                 `SwapEffect::Rotate`; and `SwapEffect::Flip` flips them around the vertical \
                 axis. Click any icon to swap them all.",
            ),
            effects,
            doc::note(
                "Iced 0.14 draws an element with neither opacity nor rotation, so the fade and \
                 rotate effects show the element that is more visible at each moment, and the \
                 flip narrows and widens the elements around their vertical center line.",
            ),
            doc::subheading("Duration"),
            doc::prose(
                "`duration` sets the length of the transition, 200 milliseconds by default.",
            ),
            duration,
            doc::subheading("Any content"),
            doc::prose("Both sides take any element, such as the labels of a button."),
            label,
            doc::prose(
                "A swap shows its final state the first time it is drawn, and \
                 `animated(false)` changes at once, for reduced motion. Update the tooltip of \
                 the button around it with the state, as the preview does.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::swap::swap",
                    "oito2_iced::widget::swap::Swap",
                    "oito2_iced::widget::swap::SwapEffect",
                    "oito2_iced::widget::swap::DURATION",
                ],
            ),
        ],
    )
}
