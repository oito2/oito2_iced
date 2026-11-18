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

//! The Marquee page: horizontal, reversed, vertical and sharp-edged
//! marquees, which play on request.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{self, lucide};
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::marquee::{Axis, marquee};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("marquee.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    playing: bool,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The marquees started or stopped.
    Playing(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Playing(playing) => state.playing = playing,
    }

    Task::none()
}

/// The rounded surface container frame of `width` around a marquee.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: f32,
) -> Element<'a, Message, Theme> {
    container(content)
        .padding([spacing::SM, 0.0])
        .width(Length::Fixed(width))
        .style(|theme: &Theme| {
            container::Style::default()
                .background(theme.roles().surface_container)
                .border(iced::Border::default().rounded(radius::MEDIUM))
        })
        .into()
}

/// The button that starts and stops the marquees.
fn play<'a>(playing: bool) -> Element<'a, Message, Theme> {
    if playing {
        button::outlined("Pause the marquees")
            .icon(lucide::PAUSE)
            .on_press(Message::Playing(false))
            .into()
    } else {
        button::outlined("Play the marquees")
            .icon(lucide::PLAY)
            .on_press(Message::Playing(true))
            .into()
    }
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let playing = state.playing;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            frame(
                // #region preview
                marquee(text(
                    "oito2_iced 0.1.0 · Linux, Windows and macOS · GPL-3.0-or-later · Lucide icons",
                ))
                .fade_color(|theme| theme.roles().surface_container)
                // #endregion
                .animated(playing),
                360.0,
            ),
            play(playing),
        ]
        .spacing(spacing::MD)
        .align_x(iced::Alignment::Center),
    );

    let reverse = doc::example(
        ctx,
        SOURCE,
        &["reverse"],
        frame(
            // #region reverse
            marquee(
                row([
                    lucide::MONITOR,
                    lucide::SMARTPHONE,
                    lucide::TABLET,
                    lucide::GLOBE,
                    lucide::LAPTOP,
                    lucide::TV,
                    lucide::WATCH,
                    lucide::SERVER,
                    lucide::CPU,
                    lucide::HARD_DRIVE,
                ]
                .map(|glyph| icon::icon(glyph).into()))
                .spacing(spacing::LG),
            )
            .reverse(true)
            .speed(60.0)
            .fade_color(|theme| theme.roles().surface_container)
            // #endregion
            .animated(playing),
            360.0,
        ),
    );

    let vertical = doc::example(
        ctx,
        SOURCE,
        &["vertical"],
        frame(
            // #region vertical
            marquee(column(
                [
                    "Build 128 passed",
                    "Deploy to Pages done",
                    "3 new issues",
                    "Release 0.1.0 tagged",
                    "Goldens updated",
                ]
                .map(|line| text(line).into()),
            ))
            .axis(Axis::Vertical)
            .speed(20.0)
            .height(80)
            .fade_color(|theme| theme.roles().surface_container)
            // #endregion
            .animated(playing),
            220.0,
        ),
    );

    let edges = doc::example(
        ctx,
        SOURCE,
        &["edges"],
        frame(
            // #region edges
            marquee(text(
                "Breaking: a marquee without faded edges keeps moving even under the pointer.",
            ))
            .fade_edges(false)
            .pause_on_hover(false)
            .gap(64.0)
            // #endregion
            .animated(playing),
            360.0,
        ),
    );

    doc::page(
        "Marquee",
        "Scrolls content that is longer than its space continuously, such as a news ticker or \
         a long song title.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Wrap the content in `marquee`. When the content is longer than the marquee, \
                 it moves toward the start of the reading direction at `speed` pixels per \
                 second, 40 by default, followed by a copy after `gap` pixels, 32 by default; \
                 content that fits stays still. The marquee pauses while the pointer is over \
                 it or the focus is inside it, and stays at its start with `animated(false)`, \
                 for reduced motion.",
            ),
            doc::code(ctx, "use oito2_iced::widget::marquee::{Axis, marquee};"),
            doc::note(
                "The marquees on this page stay still until the Play button above starts \
                 them, through `animated`. The copy is the same content drawn a second time, \
                 and only the copy at the start edge receives the pointer, so plain content, \
                 such as text or icons, fits best.",
            ),
            doc::prose(
                "The ends fade into `fade_color`, the surface role by default: give it the \
                 color of the background behind the marquee, as these examples do with the \
                 surface container of their frame.",
            ),
            doc::heading("Examples"),
            doc::subheading("Reversed and faster"),
            doc::prose(
                "`reverse` moves the content toward the end instead, and `speed` sets how \
                 fast.",
            ),
            reverse,
            doc::subheading("Vertical"),
            doc::prose(
                "With `axis(Axis::Vertical)` the content moves up, or down when reversed. A \
                 vertical marquee fills the height of its parent, so give it a bounded \
                 `height`.",
            ),
            vertical,
            doc::subheading("Edges and pausing"),
            doc::prose(
                "By default the content fades over `fade_extent` pixels, 24, at both ends; \
                 `fade_edges(false)` cuts it sharply instead. `pause_on_hover(false)` keeps it \
                 moving under the pointer and the focus.",
            ),
            edges,
            doc::note(
                "Keep `pause_on_hover` on for content people need to read: the motion then \
                 stops under the pointer.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::marquee::marquee",
                    "oito2_iced::widget::marquee::Marquee",
                    "oito2_iced::widget::marquee::Axis",
                    "oito2_iced::widget::marquee::SPEED",
                    "oito2_iced::widget::marquee::GAP",
                    "oito2_iced::widget::marquee::FADE_EXTENT",
                ],
            ),
        ],
    )
}
