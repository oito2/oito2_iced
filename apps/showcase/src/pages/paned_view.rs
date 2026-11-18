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

//! The PanedView page: pane sides, stacking, limits, a fixed divider and
//! size reports.

use iced::widget::{container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::paned_view::{Side, paned_view};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("paned_view.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    preview: f32,
    end: f32,
    stacked: f32,
    limits: f32,
    reported: f32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            preview: 200.0,
            end: 180.0,
            stacked: 100.0,
            limits: 240.0,
            reported: 200.0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The pane of the preview was resized.
    Preview(f32),
    /// The pane at the end was resized.
    End(f32),
    /// The stacked pane was resized.
    Stacked(f32),
    /// The pane of the limits example was resized.
    Limits(f32),
    /// The pane of the reporting example was resized.
    Reported(f32),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Preview(size) => state.preview = size,
        // #endregion
        Message::End(size) => state.end = size,
        Message::Stacked(size) => state.stacked = size,
        Message::Limits(size) => state.limits = size,
        Message::Reported(size) => state.reported = size,
    }

    Task::none()
}

// #region region
/// A region showing `label` in its center.
fn region<'a>(label: impl text::IntoFragment<'a>) -> container::Container<'a, Message, Theme> {
    container(text(label)).center(Length::Fill)
}
// #endregion

/// A bordered frame of `width` by `height` around an example.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: Length,
    height: f32,
) -> Element<'a, Message, Theme> {
    border_container(content)
        .width(width)
        .height(Length::Fixed(height))
        .clip(true)
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["region", "preview", "update"],
        frame(
            // #region preview
            paned_view(
                region("Pane").class(ContainerClass::Mainbar),
                region("Page"),
                state.preview,
            )
            .min_pane_size(120.0)
            .min_page_size(160.0)
            .on_resize(Message::Preview),
            // #endregion
            Length::Fixed(520.0),
            240.0,
        ),
    );

    let end = doc::example(
        ctx,
        SOURCE,
        &["end"],
        frame(
            // #region end
            paned_view(region("Details"), region("Page"), state.end)
                .side(Side::End)
                .min_pane_size(120.0)
                .min_page_size(160.0)
                .direction(ctx.direction)
                .on_resize(Message::End),
            // #endregion
            Length::Fixed(520.0),
            240.0,
        ),
    );

    let stacked = doc::example(
        ctx,
        SOURCE,
        &["stacked"],
        frame(
            // #region stacked
            paned_view(region("Terminal"), region("Editor"), state.stacked)
                .side(Side::Bottom)
                .min_pane_size(60.0)
                .min_page_size(100.0)
                .on_resize(Message::Stacked),
            // #endregion
            Length::Fixed(420.0),
            280.0,
        ),
    );

    let limits = doc::example(
        ctx,
        SOURCE,
        &["limits"],
        frame(
            // #region limits
            paned_view(
                region("Pane · drag the divider").class(ContainerClass::Mainbar),
                region("Page"),
                state.limits,
            )
            .min_pane_size(160.0)
            .min_page_size(160.0)
            .on_resize(Message::Limits),
            // #endregion
            Length::Fill,
            260.0,
        ),
    );

    let fixed = doc::example(
        ctx,
        SOURCE,
        &["fixed"],
        frame(
            // #region fixed
            paned_view::<Message>(region("Pane"), region("Page"), 160.0)
                .min_pane_size(120.0)
                .min_page_size(160.0)
                .resizable(false),
            // #endregion
            Length::Fixed(520.0),
            200.0,
        ),
    );

    let callback = doc::example(
        ctx,
        SOURCE,
        &["callback"],
        frame(
            // #region callback
            paned_view(
                region(format!("{} px", state.reported.round())),
                region("Page"),
                state.reported,
            )
            .min_pane_size(120.0)
            .min_page_size(160.0)
            .keyboard_step(40.0)
            .on_resize(Message::Reported),
            // #endregion
            Length::Fixed(520.0),
            200.0,
        ),
    );

    doc::page(
        "PanedView",
        "Splits an area between a side pane and a page, with a divider that resizes the pane \
         with the pointer or the keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `paned_view` a pane, a page and the pane size: the pane is that many \
                 pixels wide, or tall when stacked, and the page takes the rest. The size \
                 lives in the application: `on_resize` publishes the next size while the \
                 divider moves; without it the divider stays still. Place the view where its \
                 size is bounded, such as the body of a window.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::paned_view::{Side, paned_view};",
            ),
            doc::note(
                "For one resizable side pane, use a paned view. For three or more panes, \
                 nested panes or collapsible panes, use `splitter`. `master_detail` builds on \
                 a paned view for list and detail layouts.",
            ),
            doc::heading("Examples"),
            doc::subheading("Pane at the end"),
            doc::prose(
                "`side` places the pane. `Side::Start` and `Side::End` follow the reading \
                 `direction`: the end is on the right in left-to-right text and on the left in \
                 right-to-left text.",
            ),
            end,
            doc::subheading("Stacked"),
            doc::prose(
                "With `Side::Top` or `Side::Bottom`, the pane and the page are stacked and the \
                 sizes are heights.",
            ),
            stacked,
            doc::subheading("Size limits"),
            doc::prose(
                "The pane never shrinks below `min_pane_size`, 180 pixels by default, and \
                 never leaves less than `min_page_size`, 320 by default, for the page. In a \
                 narrow window the pane gives way to keep the page size; when even the \
                 smallest pane does not fit, the pane keeps its minimum and the end of the \
                 view is clipped instead of overflowing.",
            ),
            limits,
            doc::subheading("Fixed pane"),
            doc::prose(
                "With `resizable(false)`, the divider is a plain line that cannot be dragged \
                 or focused.",
            ),
            fixed,
            doc::subheading("Reacting to resizes"),
            doc::prose(
                "Since the application keeps the size, it can show it, save it and restore it \
                 later. `keyboard_step` sets how far one arrow key press moves the divider, 16 \
                 pixels by default.",
            ),
            callback,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the divider."],
                    vec![
                        "`Left` / `Right`",
                        "Moves a side-by-side divider by the keyboard step.",
                    ],
                    vec![
                        "`Up` / `Down`",
                        "Moves a stacked divider by the keyboard step.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::paned_view::paned_view",
                    "oito2_iced::widget::paned_view::PanedView",
                    "oito2_iced::widget::paned_view::Side",
                    "oito2_iced::widget::paned_view::DEFAULT_PANE_SIZE",
                    "oito2_iced::widget::paned_view::DEFAULT_MIN_PANE_SIZE",
                    "oito2_iced::widget::paned_view::DEFAULT_MIN_PAGE_SIZE",
                ],
            ),
        ],
    )
}
