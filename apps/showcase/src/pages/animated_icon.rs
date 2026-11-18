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
//! The AnimatedIcon page: icons that draw themselves, their play modes,
//! use in a button and a custom icon.

use std::time::Duration;

use iced::widget::canvas::Path;
use iced::widget::{column, keyed_column, row, text};
use iced::{Alignment, Color, Element, Length, Point, Task};

use oito2_iced::Theme;
use oito2_iced::animation::ease_out;
use oito2_iced::icon::animated::{self, Canvas, Data, Mode};
use oito2_iced::icon::lucide;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::segment_group::{segment, segment_group};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("animated_icon.rs");

/// The animated icons of the catalog, with their names.
const ICONS: [(&str, Data); 10] = [
    ("OK", animated::OK),
    ("OK_FILLED", animated::OK_FILLED),
    ("HEART", animated::HEART),
    ("HEART_FILLED", animated::HEART_FILLED),
    ("STAR", animated::STAR),
    ("STAR_FILLED", animated::STAR_FILLED),
    ("STAR_HALF_FILLED", animated::STAR_HALF_FILLED),
    ("COMPASS", animated::COMPASS),
    ("COMPASS_FILLED", animated::COMPASS_FILLED),
    ("NO_NETWORK", animated::NO_NETWORK),
];

// #region custom-data
/// Draws a ring that grows from the center of the 24-unit grid.
fn paint_ring(canvas: &mut Canvas<'_>, progress: f32, color: Color) {
    let radius = 2.0 + 8.0 * ease_out(progress);

    canvas.stroke(&Path::circle(Point::new(12.0, 12.0), radius), color, 2.0);
}

/// A ring that grows in 900 milliseconds.
const RING: Data = Data::new(paint_ring).with_duration(Duration::from_millis(900));
// #endregion

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    mode: Mode,
    replays: u32,
    liked: bool,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The icons should play again from the start.
    Replay,
    /// A play mode was chosen.
    Mode(Mode),
    /// The like button was pressed.
    Like,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Replay => state.replays += 1,
        Message::Mode(mode) => state.mode = mode,
        Message::Like => state.liked = !state.liked,
    }

    Task::none()
}

/// `icon` in a keyed column whose key is `replays`, so a new count makes a
/// new icon state that plays from the start.
fn replayable<'a>(
    replays: u32,
    icon: impl Into<Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    keyed_column([(replays, icon.into())]).into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let replay = || {
        button::outlined("Replay")
            .icon(lucide::ROTATE_CCW)
            .on_press(Message::Replay)
    };

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            replayable(
                state.replays,
                // #region preview
                animated::icon(animated::OK).size(48.0),
                // #endregion
            ),
            replay(),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let modes = segment_group(
        [
            segment(Mode::Once, "Once"),
            segment(Mode::Repeat, "Repeat"),
            segment(Mode::Still, "Still"),
        ],
        Some(state.mode),
    )
    .on_change(Message::Mode);

    let icons = row(ICONS.map(|(name, data)| {
        column![
            replayable(
                state.replays,
                // #region gallery
                animated::icon(data).size(40.0).mode(state.mode),
                // #endregion
            ),
            text(name).size(11).class(TextClass::Variant),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center)
        .width(Length::Fixed(128.0))
        .into()
    }))
    .spacing(spacing::SM)
    .wrap()
    .vertical_spacing(spacing::MD);

    let gallery = doc::example(
        ctx,
        SOURCE,
        &["gallery"],
        column![
            row![modes, replay()]
                .spacing(spacing::MD)
                .align_y(Alignment::Center),
            icons,
        ]
        .spacing(spacing::LG)
        .align_x(Alignment::Center),
    );

    let liked = doc::example(
        ctx,
        SOURCE,
        &["button"],
        // #region button
        button::button(if state.liked {
            animated::icon(animated::HEART_FILLED)
        } else {
            animated::icon(animated::HEART).mode(Mode::Still)
        })
        .class(if state.liked {
            button::Class::IconSelected
        } else {
            button::Class::Icon
        })
        .padding([0.0, spacing::XS])
        .tooltip(if state.liked { "Unlike" } else { "Like" })
        .on_press(Message::Like),
        // #endregion
    );

    let color = doc::example(
        ctx,
        SOURCE,
        &["color"],
        replayable(
            state.replays,
            // #region color
            animated::icon(animated::COMPASS)
                .size(56.0)
                .color(Color::from_rgb8(0x35, 0x84, 0xE4))
                .duration(Duration::from_secs(3)),
            // #endregion
        ),
    );

    let custom = doc::example(
        ctx,
        SOURCE,
        &["custom-data", "custom"],
        replayable(
            state.replays,
            // #region custom
            animated::icon(RING).size(48.0),
            // #endregion
        ),
    );

    doc::page(
        "AnimatedIcon",
        "An icon that draws itself when it appears, plays in a loop, or shows its final \
         frame, in the stroke style of the Lucide icons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `animated::icon` one of the icons of the `animated` module. Like the \
                 static `icon`, it takes its size from the density and its color from its \
                 parent unless you set them, so it fits in buttons and list tiles. By \
                 default it plays once, when it first shows and again whenever its data or \
                 mode changes.",
            ),
            doc::code(ctx, "use oito2_iced::icon::animated::{self, Mode};"),
            doc::note(
                "A new widget state plays the animation from the start. The Replay buttons \
                 of this page put each icon in a `keyed_column` whose key changes on every \
                 replay.",
            ),
            doc::heading("Examples"),
            doc::subheading("Icons and modes"),
            doc::prose(
                "`mode` sets how the animation plays: `Mode::Once`, `Mode::Repeat` over and \
                 over, or `Mode::Still`, which shows the final frame. The filled variants \
                 cut their inner mark out of the fill, so the background shows through it. \
                 The icon asks for redraws only while it plays.",
            ),
            gallery,
            doc::table(
                &["Icon", "Animation"],
                &[
                    vec!["`OK`", "A circle that draws itself, then a check mark."],
                    vec![
                        "`OK_FILLED`",
                        "A filled circle that pops in, then a check mark cut out of it.",
                    ],
                    vec!["`HEART`", "A heart outline that draws itself, then beats."],
                    vec!["`HEART_FILLED`", "A filled heart that pops in."],
                    vec!["`STAR`", "A star outline that draws itself."],
                    vec!["`STAR_FILLED`", "A filled star that spins and pops in."],
                    vec![
                        "`STAR_HALF_FILLED`",
                        "A star outline whose left half fills.",
                    ],
                    vec![
                        "`COMPASS`",
                        "A compass whose needle, with a filled north half, swings and \
                         settles north.",
                    ],
                    vec![
                        "`COMPASS_FILLED`",
                        "A filled compass whose cut-out needle swings and settles north.",
                    ],
                    vec![
                        "`NO_NETWORK`",
                        "Wi-Fi waves that appear one by one, then a slash across them.",
                    ],
                ],
            ),
            doc::subheading("In a button"),
            doc::prose(
                "Swap the icon on a state change to play it as feedback. Here the outline \
                 heart is still, and the filled heart pops in when the button is selected. \
                 `icon_button` takes a static icon, so the example builds the button from \
                 `button::button` with the icon classes.",
            ),
            liked,
            doc::subheading("Color and duration"),
            doc::prose(
                "`color` and `size` override the parent color and the density icon size, \
                 and `duration` the length of one play, which otherwise comes from the icon \
                 data. `animated(false)` shows the final frame, for reduced motion.",
            ),
            color,
            doc::subheading("Custom icon"),
            doc::prose(
                "A `Data` pairs a `Painter` with the length of one play, 700 milliseconds \
                 by default. The painter draws one frame on a `Canvas` of 24 grid units, \
                 scaled to the icon size, for a progress from 0 to 1, in the icon color. \
                 `Canvas::stroke` takes its width in grid units, like the 2-unit strokes \
                 of the Lucide icons.",
            ),
            custom,
            doc::note(
                "The icon does not read a reduced motion preference by itself; set \
                 `animated(false)` from the preference of the application.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::icon::animated::icon",
                    "oito2_iced::icon::animated::AnimatedIcon",
                    "oito2_iced::icon::animated::Mode",
                    "oito2_iced::icon::animated::Data",
                    "oito2_iced::icon::animated::Painter",
                    "oito2_iced::icon::animated::Canvas",
                ],
            ),
        ],
    )
}
