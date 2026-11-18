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

//! The NumberEntry page: whole numbers, decimal steps and a read-only
//! entry.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::widget::number_entry::number_entry;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("number_entry.rs");

/// The width of the example entries.
const WIDTH: f32 = 200.0;

/// The state of the page.
#[derive(Debug)]
pub struct State {
    copies: f64,
    scale: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            copies: 1.0,
            scale: 1.5,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The number of copies changed.
    Copies(f64),
    /// The scale changed.
    Scale(f64),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Copies(copies) => state.copies = copies,
        // #endregion
        Message::Scale(scale) => state.scale = scale,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview", "update"],
        container(
            // #region preview
            number_entry(l10n, state.copies)
                .range(1.0, 99.0)
                .label("Copies")
                .on_change(Message::Copies),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let decimals = doc::example(
        ctx,
        SOURCE,
        &["decimals"],
        container(
            // #region decimals
            number_entry(l10n, state.scale)
                .range(0.25, 4.0)
                .step(0.25)
                .decimals(2)
                .label("Scale")
                .on_change(Message::Scale),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let read_only = doc::example(
        ctx,
        SOURCE,
        &["read-only"],
        container(
            // #region read-only
            number_entry::<Message>(l10n, 42.0).label("Read-only"),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    doc::page(
        "NumberEntry",
        "A number field with decrease and increase buttons, stepped with the keyboard, the \
         mouse wheel or by dragging the buttons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `number_entry` the strings of the current language and the value, and \
                 update the value from `on_change`. The entry shows it in the number format of \
                 the locale, aligned to the end. Typed text is read when the field loses the \
                 focus or on `Enter`: it is rounded, kept within `range` and published, and \
                 text that isn't a number is replaced by the current value. The text being \
                 typed lives in the entry, the value in the application.",
            ),
            doc::code(ctx, "use oito2_iced::widget::number_entry::number_entry;"),
            doc::prose(
                "The buttons change the value by `step`, 1 by default, and turn off at the \
                 bounds; their tooltips, \"Decrease\" and \"Increase\", come from the strings \
                 of the language. While the field has the focus the mouse wheel steps too, \
                 and dragging sideways on the buttons changes the value one step per 4 \
                 pixels.",
            ),
            doc::heading("Examples"),
            doc::subheading("Decimals and step"),
            doc::prose(
                "`decimals` sets the decimal places shown and kept, 0 by default, and `step` \
                 the change of the buttons, the arrow keys and the wheel. `page_step`, ten \
                 steps by default, is the change of `Page Up` and `Page Down`.",
            ),
            decimals,
            doc::subheading("Read-only"),
            doc::prose(
                "Without `on_change` the entry is read-only: the text can't be edited and the \
                 buttons, the arrow keys and the wheel do nothing.",
            ),
            read_only,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Up` / `Down`", "Raises or lowers the value by the step."],
                    vec![
                        "`Page Up` / `Page Down`",
                        "Raises or lowers the value by the page step.",
                    ],
                    vec![
                        "`Home` / `End`",
                        "Sets the value to a bound, when it is finite.",
                    ],
                    vec!["`Enter`", "Applies the typed text."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::number_entry::number_entry",
                    "oito2_iced::widget::number_entry::NumberEntry",
                ],
            ),
        ],
    )
}
