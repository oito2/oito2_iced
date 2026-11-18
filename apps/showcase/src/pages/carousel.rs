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
//! The Carousel page: buttons, dots, keys and swipes, automatic scrolling
//! and a carousel that stops at its ends.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Border, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{Icon, icon, lucide};
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::carousel::{Controller, carousel};
use oito2_iced::widget::styled_text;
use oito2_iced::widget::toggles::switch;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("carousel.rs");

/// The id of the carousels, which take the focus as a whole.
pub const CAROUSEL: &str = "showcase-carousel";

/// The slides: an icon and a title.
const SLIDES: [(Icon, &str); 4] = [
    (lucide::PALETTE, "Themes"),
    (lucide::KEYBOARD, "Shortcuts"),
    (lucide::LANGUAGES, "Languages"),
    (lucide::ACCESSIBILITY, "Access"),
];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    preview: Controller,
    auto: Controller,
    stopping: Controller,
}

impl Default for State {
    fn default() -> Self {
        Self {
            preview: Controller::new(),
            auto: Controller::new(),
            stopping: Controller::new(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The preview carousel asked for a page.
    Preview(usize),
    /// The automatic carousel asked for a page.
    Auto(usize),
    /// Automatic scrolling was turned on or off.
    AutoScroll(bool),
    /// The carousel that stops at its ends asked for a page.
    Stopping(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Preview(page) => state.preview.go_to(page),
        Message::Auto(page) => state.auto.go_to(page),
        Message::AutoScroll(on) => {
            // #region auto-toggle
            state.auto = Controller::new()
                .with_initial_page(state.auto.page())
                .with_auto_scroll(on);
            // #endregion
        }
        Message::Stopping(page) => state.stopping.go_to(page),
    }

    Task::none()
}

/// The colored slides, each with an icon and a title.
fn slides<'a>() -> Vec<Element<'a, Message, Theme>> {
    SLIDES
        .iter()
        .enumerate()
        .map(|(index, (glyph, title))| {
            container(
                column![
                    icon(*glyph).size(48.0),
                    styled_text(*title, typography::HEADLINE_SMALL),
                ]
                .spacing(spacing::SM)
                .align_x(Alignment::Center),
            )
            .center(Length::Fill)
            .padding([0.0, spacing::XS])
            .style(move |theme: &Theme| {
                let roles = theme.roles();
                let (background, foreground) = match index {
                    0 => (roles.primary_container, roles.on_primary_container),
                    1 => (roles.secondary_container, roles.on_secondary_container),
                    2 => (roles.tertiary_container, roles.on_tertiary_container),
                    _ => (roles.surface_container_highest, roles.on_surface),
                };

                container::Style {
                    background: Some(background.into()),
                    text_color: Some(foreground),
                    border: Border::default().rounded(radius::LARGE),
                    ..container::Style::default()
                }
            })
            .into()
        })
        .collect()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            carousel(&state.preview, slides())
                .localizations(l10n)
                .direction(ctx.direction)
                .on_page_change(Message::Preview)
                .id(CAROUSEL),
            // #endregion
        )
        .width(Length::Fixed(560.0)),
    );

    let auto = doc::example(
        ctx,
        SOURCE,
        &["auto-toggle", "auto"],
        column![
            row![
                switch(state.auto.auto_scroll()).on_toggle(Message::AutoScroll),
                text("Scroll automatically"),
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            container(
                // #region auto
                carousel(&state.auto, slides())
                    .height(180.0)
                    .localizations(l10n)
                    .on_page_change(Message::Auto)
                    .id(CAROUSEL),
                // #endregion
            )
            .width(Length::Fixed(560.0)),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let stopping = doc::example(
        ctx,
        SOURCE,
        &["no-loop"],
        container(
            column![
                // #region no-loop
                carousel(&state.stopping, slides())
                    .looping(false)
                    .page_indicator(false)
                    .height(160.0)
                    .localizations(l10n)
                    .on_page_change(Message::Stopping)
                    .id(CAROUSEL),
                text(format!(
                    "Page {} of {}",
                    state.stopping.page() + 1,
                    SLIDES.len()
                )),
                // #endregion
            ]
            .spacing(spacing::SM)
            .align_x(Alignment::Center),
        )
        .width(Length::Fixed(560.0)),
    );

    doc::page(
        "Carousel",
        "A row of pages shown one at a time, with previous and next buttons, a page \
         indicator and optional automatic scrolling.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `carousel` a `Controller` and its pages; they are `height` pixels \
                 tall, 240 by default, and as wide as the carousel. Pages change with the \
                 previous and next buttons, the dots of the page indicator below, the \
                 arrow keys, or a swipe, also with a mouse. Past the last page, the buttons \
                 and keys return to the first, and back, jumping straight there.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::carousel::{Controller, carousel};",
            ),
            doc::prose(
                "The current page lives in the `Controller`, owned by the application. The \
                 carousel publishes the page the user asks for through `on_page_change`; \
                 pass it to `Controller::go_to` and the pages slide to it. \
                 `with_initial_page` sets the first page, and `jump_to` changes the page \
                 without sliding.",
            ),
            doc::heading("Examples"),
            doc::subheading("Automatic scrolling"),
            doc::prose(
                "A controller `with_auto_scroll(true)` moves to the next page every \
                 `with_auto_scroll_interval`, 4 seconds by default, wrapping to the first. \
                 It pauses while a pointer hovers the carousel or anything inside it has \
                 the focus, and it never scrolls by itself with `animated(false)`. Turn it \
                 on with the switch.",
            ),
            auto,
            doc::subheading("Stopping at the ends"),
            doc::prose(
                "With `looping(false)`, the previous button is disabled on the first page \
                 and the next button on the last. Here the caption reads the page from the \
                 controller, and `page_indicator(false)` hides the dots; \
                 `navigation_controls(false)` would hide the buttons.",
            ),
            stopping,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab`",
                        "Moves the focus to the carousel and through its buttons and dots.",
                    ],
                    vec![
                        "`Right`",
                        "Shows the next page; the previous one in right-to-left text.",
                    ],
                    vec![
                        "`Left`",
                        "Shows the previous page; the next one in right-to-left text.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::carousel::carousel",
                    "oito2_iced::widget::carousel::Carousel",
                    "oito2_iced::widget::carousel::Controller",
                    "oito2_iced::widget::carousel::HEIGHT",
                    "oito2_iced::widget::carousel::AUTO_SCROLL_INTERVAL",
                ],
            ),
        ],
    )
}
