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

//! The Splitter page: two panes, three nested and collapsible panes, and
//! a pane that starts collapsed.

use iced::widget::{container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::splitter::{self, Axis, pane};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("splitter.rs");

/// The state of the page: the sizes of each splitter.
#[derive(Debug, Default)]
pub struct State {
    preview: splitter::State,
    nested: splitter::State,
    inner: splitter::State,
    collapsed: splitter::State,
}

/// A splitter of the page.
#[derive(Debug, Clone, Copy)]
pub enum Split {
    /// The preview.
    Preview,
    /// The outer splitter of the nested example.
    Nested,
    /// The inner splitter of the nested example.
    Inner,
    /// The splitter that starts collapsed.
    Collapsed,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A divider of a splitter moved, or a pane collapsed or was restored.
    Split(Split, splitter::State),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    let Message::Split(split, sizes) = message;
    match split {
        Split::Preview => state.preview = sizes,
        Split::Nested => state.nested = sizes,
        Split::Inner => state.inner = sizes,
        Split::Collapsed => state.collapsed = sizes,
    }

    Task::none()
}

/// A pane on the `class` surface showing `label` in the middle.
fn region<'a>(label: &'a str, class: ContainerClass<'a>) -> Element<'a, Message, Theme> {
    container(text(label))
        .class(class)
        .center(Length::Fill)
        .into()
}

/// A bordered frame of `width` × `height` around an example.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: f32,
    height: f32,
) -> Element<'a, Message, Theme> {
    border_container(content)
        .width(width)
        .height(height)
        .padding(1.0)
        .clip(true)
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        frame(
            // #region preview
            splitter::splitter(
                &state.preview,
                vec![
                    pane(region("Start", ContainerClass::Sidebar))
                        .fraction(0.4)
                        .min_size(120.0),
                    pane(region("End", ContainerClass::Transparent)).min_size(120.0),
                ],
            )
            .on_change(|sizes| Message::Split(Split::Preview, sizes)),
            // #endregion
            520.0,
            220.0,
        ),
    );

    let nested = doc::example(
        ctx,
        SOURCE,
        &["nested"],
        frame(
            // #region nested
            splitter::splitter(
                &state.nested,
                vec![
                    pane(region("Sidebar", ContainerClass::Sidebar))
                        .fraction(0.25)
                        .min_size(120.0)
                        .collapsible(true),
                    pane(
                        splitter::splitter(
                            &state.inner,
                            vec![
                                pane(region("Editor", ContainerClass::Transparent)),
                                pane(region("Terminal", ContainerClass::Mainbar))
                                    .fraction(0.3)
                                    .collapsible(true),
                            ],
                        )
                        .axis(Axis::Vertical)
                        .on_change(|sizes| Message::Split(Split::Inner, sizes)),
                    )
                    .min_size(200.0),
                    pane(region("Outline", ContainerClass::Sidebar))
                        .fraction(0.25)
                        .min_size(120.0)
                        .max_size(260.0)
                        .collapsible(true),
                ],
            )
            .on_change(|sizes| Message::Split(Split::Nested, sizes)),
            // #endregion
            640.0,
            320.0,
        ),
    );

    let collapsed = doc::example(
        ctx,
        SOURCE,
        &["collapsed"],
        frame(
            // #region collapsed
            splitter::splitter(
                &state.collapsed,
                vec![
                    pane(region("Filters", ContainerClass::Sidebar))
                        .min_size(160.0)
                        .collapsible(true)
                        .collapsed_size(32.0)
                        .collapsed(true),
                    pane(region("Results", ContainerClass::Transparent)).min_size(200.0),
                ],
            )
            .on_change(|sizes| Message::Split(Split::Collapsed, sizes)),
            // #endregion
            520.0,
            200.0,
        ),
    );

    doc::page(
        "Splitter",
        "Panes side by side or stacked, with a draggable divider between each two, that can \
         nest and collapse.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List two or more `pane`s, from the start, or from the top with \
                 `axis(Axis::Vertical)`. A pane starts at its `fraction` of the space, and the \
                 panes without one share what is left equally. `min_size` and `max_size` \
                 bound each pane in pixels, 48 pixels at least by default, and a divider keeps \
                 both of its panes within their limits.",
            ),
            doc::code(ctx, "use oito2_iced::widget::splitter::{self, Axis, pane};"),
            doc::prose(
                "The sizes live in the application as a `splitter::State`: the splitter shows \
                 the state it is given and publishes the next one through `on_change`. A new \
                 state lays the panes out from their fractions; afterwards the sizes keep \
                 their proportions when the space changes. Without `on_change` the dividers \
                 stay where they are.",
            ),
            doc::note("For a single resizable side pane, a paned view is simpler."),
            doc::heading("Examples"),
            doc::subheading("Nested and collapsible"),
            doc::prose(
                "A pane can hold another splitter on the other axis. A `collapsible` pane \
                 collapses to its collapsed size, zero by default, and back to its previous \
                 size with a double click on the divider beside it, or Enter or Space while \
                 the divider has the focus. A divider between two collapsible panes collapses \
                 the one before it.",
            ),
            nested,
            doc::subheading("Starting collapsed"),
            doc::prose(
                "`collapsed(true)` starts a collapsible pane collapsed. A non-zero \
                 `collapsed_size` keeps a strip of the pane visible.",
            ),
            collapsed,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next divider."],
                    vec![
                        "Left, Right",
                        "Move a divider between side-by-side panes by the keyboard step, 16 \
                         pixels by default (`keyboard_step`).",
                    ],
                    vec![
                        "Up, Down",
                        "Move a divider between stacked panes by the keyboard step.",
                    ],
                    vec![
                        "Home, End",
                        "Take the pane before the divider to its smallest or largest size, as \
                         far as the limits of both panes allow.",
                    ],
                    vec![
                        "Enter, Space",
                        "Collapse or restore the collapsible pane next to the divider.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::splitter::splitter",
                    "oito2_iced::widget::splitter::Splitter",
                    "oito2_iced::widget::splitter::pane",
                    "oito2_iced::widget::splitter::Pane",
                    "oito2_iced::widget::splitter::State",
                    "oito2_iced::widget::splitter::Axis",
                    "oito2_iced::widget::splitter::DEFAULT_MIN_SIZE",
                    "oito2_iced::widget::splitter::DEFAULT_KEYBOARD_STEP",
                ],
            ),
        ],
    )
}
