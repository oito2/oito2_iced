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

//! The Tooltip page: labels that show while the pointer rests on an
//! element.

use std::time::Duration;

use iced::widget::tooltip::Position;
use iced::widget::{Tooltip, container, row};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::{TOOLTIP_DELAY, styled_text, tooltip};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tooltip.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A button was pressed.
    Pressed(&'static str),
}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pressed(name) => shell.toast(name, InfoType::Information),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        tooltip(
            icon(lucide::INFO).size(24.0),
            "Rest the pointer here to see a tooltip",
        ),
        // #endregion
    );

    let buttons = doc::example(
        ctx,
        SOURCE,
        &["buttons"],
        // #region buttons
        row![
            icon_button(lucide::SAVE)
                .tooltip("Save")
                .on_press(Message::Pressed("Save")),
            icon_button(lucide::SHARE)
                .tooltip("Share")
                .on_press(Message::Pressed("Share")),
            icon_button(lucide::TRASH)
                .tooltip("Delete")
                .on_press(Message::Pressed("Delete")),
        ]
        .spacing(spacing::SM),
        // #endregion
    );

    let timing = doc::example(
        ctx,
        SOURCE,
        &["timing"],
        // #region timing
        tooltip(icon(lucide::ZAP).size(24.0), "Shows at once")
            .delay(Duration::ZERO)
            .gap(spacing::MD),
        // #endregion
    );

    let positions = doc::example(
        ctx,
        SOURCE,
        &["positions"],
        row(
            // #region positions
            [
                (Position::Top, "Top"),
                (Position::Left, "Left"),
                (Position::Right, "Right"),
                (Position::FollowCursor, "Follows the cursor"),
            ]
            .into_iter()
            .map(|(position, label)| {
                let label = container(styled_text(label, typography::BODY_SMALL))
                    .padding([spacing::XS, spacing::SM])
                    .class(ContainerClass::Tooltip);

                Tooltip::new(
                    icon(lucide::CIRCLE_QUESTION_MARK).size(24.0),
                    label,
                    position,
                )
                .gap(spacing::XS)
                .delay(TOOLTIP_DELAY)
                .into()
            }),
            // #endregion
        )
        .spacing(spacing::XL)
        .align_y(Alignment::Center),
    );

    doc::page(
        "Tooltip",
        "A short label that shows below an element while the pointer rests on it.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`tooltip` wraps any element and shows its label below it after the pointer \
                 rests on it for `TOOLTIP_DELAY` (500 ms): small text on the inverse surface \
                 with the small radius. It returns the Iced `Tooltip`, so its builders still \
                 apply.",
            ),
            doc::code(ctx, "use oito2_iced::widget::tooltip;"),
            doc::heading("Examples"),
            doc::subheading("Buttons"),
            doc::prose(
                "The buttons of the catalog take a `tooltip` builder, which wraps them in the \
                 same tooltip. Give one to every button that shows only an icon.",
            ),
            buttons,
            doc::subheading("Delay and gap"),
            doc::prose(
                "`delay` changes how long the pointer rests before the tooltip shows, and \
                 `gap` the space between the element and the tooltip.",
            ),
            timing,
            doc::subheading("Other positions"),
            doc::prose(
                "`tooltip` always shows below. For another position, build an Iced `Tooltip` \
                 with a label in the `Tooltip` container class, as `tooltip` does.",
            ),
            positions,
            doc::note(
                "Iced shows a tooltip only while the pointer rests on its element, not when \
                 the element takes the keyboard focus, so a tooltip never holds the only \
                 description of an action that the keyboard reaches.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tooltip",
                    "oito2_iced::widget::TOOLTIP_DELAY",
                    "oito2_iced::theme::container::Class",
                ],
            ),
        ],
    )
}
