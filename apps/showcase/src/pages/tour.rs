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

//! The Tour page: a guided tour over a small toolbar. The application
//! hosts the tour over the whole window.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::overlay::Placement;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button::{self, icon_button};
use oito2_iced::widget::search::search_field;
use oito2_iced::widget::tour::{self, Step, Tour};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("tour.rs");

// #region steps
/// The id of the search field, a tour target.
const SEARCH: &str = "tour-page-search";
/// The id of the Create button, a tour target that waits for a tap.
const CREATE: &str = "tour-page-create";
/// The id of the settings button, a tour target.
const SETTINGS: &str = "tour-page-settings";

/// The steps of the tour.
fn steps() -> Vec<Step> {
    vec![
        Step::new("Welcome").description("This short tour shows the toolbar."),
        Step::new("Search")
            .description("Find any file by name.")
            .target(SEARCH),
        Step::new("Create")
            .description("Press the button to go on.")
            .target(CREATE)
            .wait_for_tap(true),
        Step::new("Settings")
            .description("Change the theme and the language here.")
            .target(SETTINGS)
            .placement(Placement::Start),
    ]
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    /// The tour, hosted by the application over the whole window.
    pub tour: Tour,
    result: Option<bool>,
    query: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tour: Tour::new(steps()),
            result: None,
            query: String::new(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The tour moved or ended; published by the host at the root.
    Event(tour::Event),
    /// Starts the tour at the step.
    Start(usize),
    /// The search text changed.
    Query(String),
    /// A toolbar button was pressed.
    Pressed,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Start(index) => state.tour.go_to(index),
        Message::Event(event) => {
            if let Some(completed) = state.tour.apply(event) {
                state.result = Some(completed);
            }
        }
        // #endregion
        Message::Query(query) => state.query = query,
        Message::Pressed => {}
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let result = match state.result {
        None => "Result: not started",
        Some(true) => "Result: completed",
        Some(false) => "Result: skipped",
    };

    let preview = doc::example(
        ctx,
        SOURCE,
        &["steps", "preview", "update"],
        // #region preview
        column![
            row![
                container(
                    search_field(&ctx.l10n, &state.query)
                        .placeholder("Search files")
                        .on_input(Message::Query)
                )
                .width(Length::Fixed(220.0))
                .id(SEARCH),
                container(
                    button::filled("Create")
                        .icon(lucide::PLUS)
                        .on_press(Message::Pressed)
                )
                .id(CREATE),
                container(
                    icon_button(lucide::SETTINGS)
                        .tooltip("Settings")
                        .on_press(Message::Pressed)
                )
                .id(SETTINGS),
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            button::outlined("Start the tour").on_press(Message::Start(0)),
            text(result).class(TextClass::Variant),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
        // #endregion
    );

    let go_to = doc::example(
        ctx,
        SOURCE,
        &["go-to"],
        row(
            // #region go-to
            state.tour.steps().iter().enumerate().map(|(index, step)| {
                button::outlined(format!("Start at {}", step.title()))
                    .on_press(Message::Start(index))
                    .into()
            }),
            // #endregion
        )
        .spacing(spacing::SM)
        .wrap(),
    );

    doc::page(
        "Tour",
        "A guided tour that highlights parts of the window one step at a time, with a card \
         explaining each.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "A `Tour` is a plain value the application owns: its `Step`s and the index of \
                 the current one. Give each highlighted element an `Id`, such as a `container` \
                 with `id`, and wrap the root of the view in `tour::host`, which publishes the \
                 `tour::Event`s for `Tour::apply`. `start` runs the tour; `apply` returns \
                 `Some(true)` when it was completed and `Some(false)` when it was skipped.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::tour::{self, Step, Tour};\n\n\
                 // In `view`, around the whole window:\n\
                 tour::host(content, &self.tour)\n    \
                     .on_event(Message::Tour)\n    \
                     .localizations(&l10n)",
            ),
            doc::prose(
                "Each step dims the window around its `target`, scrolls the innermost \
                 scrollable that holds it so the target sits in its middle, and shows a card \
                 with the `title`, the `description`, its position and back, next and skip \
                 buttons. Pointer input outside the spotlight is blocked; input inside reaches \
                 the target. The spotlight moves from one target to the next over \
                 `MOVE_DURATION`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Steps without a target"),
            doc::prose(
                "A step without a `target`, such as the Welcome step above, shows its card in \
                 the center of the window; so does a step whose target is not in the view.",
            ),
            doc::subheading("Waiting for a tap"),
            doc::prose(
                "With `wait_for_tap(true)`, the step has no next button: a release inside the \
                 target reaches it, then the tour moves on, as in the Create step.",
            ),
            doc::subheading("Placement"),
            doc::prose(
                "`placement` picks the side of the target where the card opens, below by \
                 default, as for `popover`. `spotlight_padding` sets the space around the \
                 target inside the spotlight, 6 pixels by default.",
            ),
            doc::subheading("Moving between steps"),
            doc::prose(
                "The application moves the tour from code: `start` and `go_to` take a step \
                 index, and `next`, `previous` and `skip` move from the current one. `index`, \
                 `step` and `is_active` describe where it is.",
            ),
            go_to,
            doc::heading("Keyboard"),
            doc::prose(
                "The card takes the focus on each step, and Tab cycles inside it. When the \
                 tour ends, the focus returns to the widget that had it before.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Right",
                        "Moves to the next step (the previous one in right-to-left layouts).",
                    ],
                    vec![
                        "Left",
                        "Moves to the previous step (the next one in right-to-left layouts).",
                    ],
                    vec!["Escape", "Skips the tour."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::tour::Tour",
                    "oito2_iced::widget::tour::Step",
                    "oito2_iced::widget::tour::Event",
                    "oito2_iced::widget::tour::host",
                    "oito2_iced::widget::tour::Host",
                    "oito2_iced::widget::tour::MOVE_DURATION",
                    "oito2_iced::widget::tour::SPOTLIGHT_PADDING",
                ],
            ),
        ],
    )
}
