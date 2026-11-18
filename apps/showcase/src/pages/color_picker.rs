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
//! The ColorPicker page: the inline picker, its formats and opacity, and
//! the button that opens it in a popover.

use iced::widget::{column, row, text};
use iced::{Alignment, Color, Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::color_picker::{
    ColorFormat, color_picker, color_picker_button, from_hex, to_hex,
};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("color_picker.rs");

// #region swatches
/// The preset colors offered below the fields.
const SWATCHES: [Color; 8] = [
    Color::from_rgb8(0xE0, 0x1B, 0x24),
    Color::from_rgb8(0xFF, 0x78, 0x00),
    Color::from_rgb8(0xF6, 0xD3, 0x2D),
    Color::from_rgb8(0x2E, 0xC2, 0x7E),
    Color::from_rgb8(0x35, 0x84, 0xE4),
    Color::from_rgb8(0x91, 0x41, 0xAC),
    Color::from_rgb8(0x86, 0x5E, 0x3C),
    Color::from_rgb8(0x3D, 0x38, 0x46),
];
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    inline: Color,
    rgb: Color,
    button: Color,
    open: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            inline: Color::from_rgb8(0x35, 0x84, 0xE4),
            rgb: Color::from_rgb8(0xF6, 0xD3, 0x2D),
            button: Color::from_rgb8(0x2E, 0xC2, 0x7E),
            open: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The inline picker changed.
    Inline(Color),
    /// The RGB picker changed.
    Rgb(Color),
    /// The button picker changed.
    Button(Color),
    /// The popover of the button opened or closed.
    Open(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Inline(color) => state.inline = color,
        Message::Rgb(color) => state.rgb = color,
        Message::Button(color) => state.button = color,
        Message::Open(open) => state.open = open,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["swatches", "preview"],
        // #region preview
        color_picker(state.inline)
            .swatches(SWATCHES)
            .localizations(l10n)
            .direction(ctx.direction)
            .on_change(Message::Inline),
        // #endregion
    );

    let format = doc::example(
        ctx,
        SOURCE,
        &["format"],
        // #region format
        color_picker(state.rgb)
            .initial_format(ColorFormat::Rgb)
            .show_alpha(false)
            .localizations(l10n)
            .on_change(Message::Rgb),
        // #endregion
    );

    let button = doc::example(
        ctx,
        SOURCE,
        &["swatches", "button"],
        // #region button
        color_picker_button(state.button)
            .show_alpha(false)
            .swatches(SWATCHES)
            .open(state.open)
            .on_open_change(Message::Open)
            .localizations(l10n)
            .on_change(Message::Button),
        // #endregion
    );

    let parsed = from_hex("#9141AC").map_or_else(String::new, |color| to_hex(color, false));

    let hex = doc::example(
        ctx,
        SOURCE,
        &["hex"],
        column![
            row![
                text("Inline picker:"),
                // #region hex
                text(to_hex(state.inline, true)),
                // #endregion
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            doc::caption(format!("Parsed and formatted again: {parsed}")),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center),
    );

    doc::page(
        "ColorPicker",
        "Picks a color with a saturation and brightness area, hue and opacity sliders, text \
         fields and preset swatches.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`color_picker` is controlled: give it the color and update it in \
                 `on_change`, which publishes each new color; without it the picker is \
                 disabled. The fields apply their text on Enter or when they lose the \
                 focus, and text that is not a valid color is replaced by the current one. \
                 `swatches` adds preset colors below the fields, and `width` sets the \
                 picker width, 264 by default.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::color_picker::{color_picker, color_picker_button};",
            ),
            doc::note(
                "The picker keeps the color as HSV, so the hue survives while the color is \
                 gray, where the hue has no effect, and moving back out of gray returns to \
                 the same hue.",
            ),
            doc::heading("Examples"),
            doc::subheading("Format and opacity"),
            doc::prose(
                "The buttons below the fields switch between one hex field and RGB or HSL \
                 fields; `initial_format` chooses the one shown first. With \
                 `show_alpha(false)` the opacity slider hides, the hex field leaves out the \
                 alpha channel and edits keep the current opacity.",
            ),
            format,
            doc::subheading("In a popover"),
            doc::prose(
                "`color_picker_button` shows a swatch of the color and its hex value, and \
                 opens the picker in a popover. The open state lives in the application, \
                 through `open` and `on_open_change`. Without `on_change` the button is \
                 disabled.",
            ),
            button,
            doc::subheading("Hex text"),
            doc::prose(
                "`to_hex` formats a color as `#RRGGBB`, or `#RRGGBBAA` when it is \
                 translucent and asked to, and `from_hex` reads such text back. `Hsv` and \
                 `Hsl` convert colors to and from the hue-based models.",
            ),
            hex,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Arrows on the area",
                        "Move the saturation (across) or the brightness (up and down) by \
                         1 %, or 10 % with `Shift`.",
                    ],
                    vec![
                        "`Right` / `Left` on a slider",
                        "Raise or lower the value by 1 %, or 10 % with `Shift` (mirrored \
                         right to left).",
                    ],
                    vec![
                        "`Up` / `Down` on a slider",
                        "Raise or lower the value by 1 %.",
                    ],
                    vec!["`Enter` in a field", "Applies the typed value."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::color_picker::color_picker",
                    "oito2_iced::widget::color_picker::ColorPicker",
                    "oito2_iced::widget::color_picker::color_picker_button",
                    "oito2_iced::widget::color_picker::ColorPickerButton",
                    "oito2_iced::widget::color_picker::ColorFormat",
                    "oito2_iced::widget::color_picker::to_hex",
                    "oito2_iced::widget::color_picker::from_hex",
                    "oito2_iced::widget::color_picker::Hsv",
                    "oito2_iced::widget::color_picker::Hsl",
                ],
            ),
        ],
    )
}
