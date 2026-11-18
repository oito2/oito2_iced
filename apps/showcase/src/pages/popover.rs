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

//! The Popover page: placements, an arrow, a short form, the anchor width
//! and nesting.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::controls::check_button;
use oito2_iced::widget::popover::{Alignment as PanelAlignment, Placement, popover};
use oito2_iced::widget::text_field::text_field;
use oito2_iced::widget::toggles::checkbox;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("popover.rs");

/// The placements of the placement example, with their names.
const PLACEMENTS: [(Placement, &str); 4] = [
    (Placement::Top, "Top"),
    (Placement::Bottom, "Bottom"),
    (Placement::Start, "Start"),
    (Placement::End, "End"),
];

/// A popover of the page.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    /// The popover of the preview.
    Details,
    /// The popover of a placement, by index.
    Placement(usize),
    /// The rename form.
    Rename,
    /// The popover as wide as its anchor.
    Wide,
    /// The outer nested popover.
    Outer,
    /// The inner nested popover.
    Inner,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    open: Vec<Panel>,
    arrow: bool,
    name: String,
    draft: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open: Vec::new(),
            arrow: true,
            name: "Notes".to_owned(),
            draft: "Notes".to_owned(),
        }
    }
}

impl State {
    /// Whether `panel` is open.
    fn is_open(&self, panel: Panel) -> bool {
        self.open.contains(&panel)
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A popover opened or closed.
    Toggle(Panel, bool),
    /// The arrow option changed.
    Arrow(bool),
    /// The new name changed.
    Draft(String),
    /// The new name was saved.
    Save,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Toggle(panel, open) => {
            state.open.retain(|shown| *shown != panel);
            if open {
                state.open.push(panel);
            } else if panel == Panel::Outer {
                state.open.retain(|shown| *shown != Panel::Inner);
            }
        }
        // #endregion
        Message::Arrow(arrow) => state.arrow = arrow,
        // #region form-update
        Message::Draft(draft) => state.draft = draft,
        Message::Save => {
            state.name = state.draft.clone();
            state.open.retain(|shown| *shown != Panel::Rename);
        } // #endregion
    }

    Task::none()
}

// #region anchor
/// An outlined button labeled `label` that opens and closes `panel`.
fn anchor<'a>(state: &State, label: &'a str, panel: Panel) -> button::Button<'a, Message> {
    button::outlined(label).on_press(Message::Toggle(panel, !state.is_open(panel)))
}
// #endregion

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        // #region preview
        popover(
            button::outlined("Details").on_press(Message::Toggle(
                Panel::Details,
                !state.is_open(Panel::Details),
            )),
            text("Popovers hold short, related content."),
        )
        .open(state.is_open(Panel::Details))
        .on_dismiss(Message::Toggle(Panel::Details, false)),
        // #endregion
    );

    let placements = row(PLACEMENTS
        .iter()
        .enumerate()
        .map(|(index, (placement, name))| {
            let panel = Panel::Placement(index);
            // #region placement
            popover(
                anchor(state, name, panel),
                text(format!("Opened on the {} side.", name.to_lowercase())),
            )
            .placement(*placement)
            .alignment(PanelAlignment::Center)
            .arrow(state.arrow)
            .direction(ctx.direction)
            .open(state.is_open(panel))
            .on_dismiss(Message::Toggle(panel, false))
            // #endregion
            .into()
        }))
    .spacing(spacing::SM);
    let placement = doc::example(
        ctx,
        SOURCE,
        &["anchor", "placement"],
        column![
            check_button(
                checkbox(state.arrow).on_change(|arrow| Message::Arrow(arrow.unwrap_or_default())),
                "Show the arrow",
            ),
            placements,
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let form = doc::example(
        ctx,
        SOURCE,
        &["anchor", "form", "form-update"],
        column![
            // #region form
            popover(
                anchor(state, "Rename", Panel::Rename),
                column![
                    text_field("New name", &state.draft)
                        .on_input(Message::Draft)
                        .on_submit(Message::Save),
                    button::filled("Save").on_press(Message::Save),
                ]
                .spacing(spacing::SM)
                .align_x(Alignment::End)
                .width(Length::Fixed(260.0)),
            )
            .open(state.is_open(Panel::Rename))
            .on_dismiss(Message::Toggle(Panel::Rename, false)),
            // #endregion
            doc::caption(format!("Name: {}", state.name)),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let wide = doc::example(
        ctx,
        SOURCE,
        &["match-width"],
        // #region match-width
        popover(
            anchor(state, "As wide as its anchor", Panel::Wide).width(Length::Fixed(240.0)),
            text("This panel takes the button width."),
        )
        .match_anchor_width(true)
        .open(state.is_open(Panel::Wide))
        .on_dismiss(Message::Toggle(Panel::Wide, false)),
        // #endregion
    );

    let nested = doc::example(
        ctx,
        SOURCE,
        &["nested"],
        // #region nested
        popover(
            anchor(state, "Outer", Panel::Outer),
            popover(
                anchor(state, "Open inner", Panel::Inner),
                text("The outer panel stays open."),
            )
            .placement(Placement::End)
            .open(state.is_open(Panel::Inner))
            .on_dismiss(Message::Toggle(Panel::Inner, false)),
        )
        .open(state.is_open(Panel::Outer))
        .on_dismiss(Message::Toggle(Panel::Outer, false)),
        // #endregion
    );

    doc::page(
        "Popover",
        "A non-modal panel anchored to an element, such as a picker or a short form.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `popover` the anchor and the panel content. The application keeps \
                 whether the panel is open and passes it to `open`: a button in the anchor \
                 typically toggles it, and `on_dismiss`, published on `Escape` and, unless \
                 `close_on_click_outside(false)`, on a press outside the anchor and the panel, \
                 closes it. Actions inside the panel close it by publishing a message that \
                 sets the state to closed.",
            ),
            doc::code(ctx, "use oito2_iced::widget::popover::popover;"),
            doc::prose(
                "The panel is on the popover surface, with a medium padding (`padding`), at \
                 most `max_width` wide, 360 by default, and at most `max_height` tall. `gap` \
                 sets the distance to the anchor and `margin` the space kept free at the \
                 edges of the window.",
            ),
            doc::note(
                "For content that shows while the pointer rests on an element, use \
                 `hover_card`; for a list of actions, a popup or context menu.",
            ),
            doc::heading("Examples"),
            doc::subheading("Placement and arrow"),
            doc::prose(
                "`placement` picks the preferred side of the anchor, below by default; `Start` \
                 and `End` follow the reading `direction`, and the panel moves to the opposite \
                 side when the preferred one has too little room. `alignment` lines it up \
                 with the start, center or end of the anchor edge, and `arrow` points it at \
                 the anchor.",
            ),
            placement,
            doc::subheading("A short form"),
            doc::prose(
                "When the panel opens, its first focusable element takes the focus unless \
                 `focus_on_open(false)`, and `Tab` cycles through the elements of the panel. \
                 When it closes with the focus inside, the focus returns to the first \
                 focusable element of the anchor.",
            ),
            form,
            doc::subheading("As wide as the anchor"),
            doc::prose(
                "With `match_anchor_width`, a panel above or below the anchor takes the \
                 anchor's width.",
            ),
            wide,
            doc::subheading("Nested"),
            doc::prose(
                "A popover opened from inside another one keeps the outer one open: a press in \
                 the inner panel does not count as a press outside the outer one.",
            ),
            nested,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Escape`", "Closes the panel."],
                    vec!["`Tab`", "Moves the focus through the panel content."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::popover::popover",
                    "oito2_iced::widget::popover::Popover",
                    "oito2_iced::widget::popover::Placement",
                    "oito2_iced::widget::popover::Alignment",
                    "oito2_iced::widget::popover::MAX_WIDTH",
                ],
            ),
        ],
    )
}
