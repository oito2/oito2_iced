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

//! The ListBox page: single and multiple selection, custom rows and
//! activation.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::collections::{
    ListCollection, SelectionController, SelectionMode, SelectionOptions,
};
use oito2_iced::icon::{self, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::list_box::{SelectionChange, list_box};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("list_box.rs");

/// The width of the example lists.
const WIDTH: f32 = 240.0;

// #region data
/// The fonts of the examples, grouped by family; Ubuntu is disabled.
fn fonts() -> Option<ListCollection<&'static str>> {
    ListCollection::new([
        "Inter",
        "Noto Sans",
        "Outfit",
        "Ubuntu",
        "JetBrains Mono",
        "Fira Code",
        "Noto Serif",
        "Merriweather",
    ])
    .ok()
    .map(|fonts| {
        fonts
            .with_group_of(|font| {
                Some(
                    match *font {
                        "JetBrains Mono" | "Fira Code" => "Monospace",
                        "Noto Serif" | "Merriweather" => "Serif",
                        _ => "Sans serif",
                    }
                    .to_owned(),
                )
            })
            .with_disabled(|font| *font == "Ubuntu")
    })
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    single: Option<SelectionController<&'static str>>,
    multiple: Option<SelectionController<&'static str>>,
    custom: Option<SelectionController<&'static str>>,
    activated: Option<&'static str>,
}

impl Default for State {
    fn default() -> Self {
        // #region single-data
        let single = fonts().map(|fonts| {
            SelectionController::with_values(fonts, SelectionOptions::default(), ["Inter"])
        });
        // #endregion
        // #region multiple-data
        let multiple = fonts().map(|fonts| {
            SelectionController::new(
                fonts,
                SelectionOptions {
                    mode: SelectionMode::Multiple,
                    ..SelectionOptions::default()
                },
            )
        });
        // #endregion
        let custom = fonts().map(|fonts| {
            SelectionController::with_values(fonts, SelectionOptions::default(), ["Outfit"])
        });

        Self {
            single,
            multiple,
            custom,
            activated: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The selection of the single-selection list changed.
    Single(SelectionChange),
    /// The selection of the multiple-selection list changed.
    Multiple(SelectionChange),
    /// The selection of the custom list changed.
    Custom(SelectionChange),
    /// A font was activated.
    Activated(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Single(change) => {
            if let Some(single) = &mut state.single {
                change.apply(single);
            }
        }
        // #endregion
        Message::Multiple(change) => {
            if let Some(multiple) = &mut state.multiple {
                change.apply(multiple);
            }
        }
        Message::Custom(change) => {
            if let Some(custom) = &mut state.custom {
                change.apply(custom);
            }
        }
        Message::Activated(font) => state.activated = Some(font),
    }

    Task::none()
}

/// The selected values of `controller`, as a caption.
fn selected<'a>(
    controller: Option<&SelectionController<&'static str>>,
) -> Element<'a, Message, Theme> {
    let values = controller
        .map(SelectionController::selected_values)
        .unwrap_or_default();

    doc::caption(format!("Selected: {}", values.join(", ")))
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let single = state.single.as_ref().map(|single| {
        // #region preview
        list_box(single)
            .on_change(Message::Single)
            .on_activate(|font| Message::Activated(font))
            .max_height(260.0)
            .width(Length::Fixed(WIDTH))
        // #endregion
    });
    let preview = doc::example(
        ctx,
        SOURCE,
        &["data", "single-data", "preview", "update"],
        column![single, selected(state.single.as_ref())]
            .spacing(spacing::SM)
            .align_x(Alignment::Center),
    );

    let multiple = state.multiple.as_ref().map(|multiple| {
        // #region multiple
        list_box(multiple)
            .on_change(Message::Multiple)
            .on_activate(|font| Message::Activated(font))
            .max_height(300.0)
            .width(Length::Fixed(WIDTH))
        // #endregion
    });
    let multiple = doc::example(
        ctx,
        SOURCE,
        &["multiple-data", "multiple"],
        column![multiple, selected(state.multiple.as_ref())]
            .spacing(spacing::SM)
            .align_x(Alignment::Center),
    );

    let custom = state.custom.as_ref().map(|custom| {
        // #region custom
        list_box(custom)
            .item_view(|font, selected| {
                row![
                    icon::icon(if selected {
                        lucide::CIRCLE_CHECK
                    } else {
                        lucide::TYPE
                    })
                    .size(16.0),
                    text(*font),
                ]
                .spacing(spacing::SM)
                .align_y(Alignment::Center)
                .into()
            })
            .on_change(Message::Custom)
            .max_height(240.0)
            .width(Length::Fixed(WIDTH))
        // #endregion
    });
    let custom = doc::example(ctx, SOURCE, &["custom"], column![custom]);

    let activated = doc::example(
        ctx,
        SOURCE,
        &["activated"],
        // #region activated
        text(format!(
            "Last activated: {}",
            state.activated.unwrap_or("none")
        )),
        // #endregion
    );

    doc::page(
        "ListBox",
        "A list of items that the user selects from, one or several, with the mouse or the \
         keyboard.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `list_box` a `SelectionController` over a `ListCollection` of the items. \
                 The controller keeps the selection in the application: the list publishes \
                 each gesture as a `SelectionChange` through `on_change`, and \
                 `SelectionChange::apply` applies it to the controller. Without `on_change` \
                 the list is read-only and skipped by Tab.",
            ),
            doc::prose(
                "Grouped items (`ListCollection::with_group_of`) show under their group \
                 headings, and disabled items (`with_disabled`) cannot be selected or \
                 activated. The list takes the focus as a whole and marks its active row. \
                 `max_height` bounds it, and it scrolls beyond. Every row is built, so it \
                 suits lists of up to a few hundred items.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::collections::{ListCollection, SelectionController, \
                 SelectionOptions};\nuse oito2_iced::widget::list_box::{SelectionChange, \
                 list_box};",
            ),
            doc::note(
                "To pick one value from a menu that takes less room, use `menu::select` over \
                 the same collection.",
            ),
            doc::heading("Examples"),
            doc::subheading("Multiple selection"),
            doc::prose(
                "With `SelectionMode::Multiple`, clicks follow the desktop conventions: \
                 `Shift`+click selects the range from the anchor, `Ctrl`+click (`Cmd`+click) \
                 toggles an item, and a plain click selects only it.",
            ),
            multiple,
            doc::subheading("Custom rows"),
            doc::prose(
                "`item_view` builds the content of each row from the item and whether it is \
                 selected; without it, the row shows the item label. The row still draws the \
                 selection fill and sets the text color around it.",
            ),
            custom,
            doc::subheading("Activation"),
            doc::prose(
                "`on_activate` publishes a message on `Enter` or a double click on an enabled \
                 item, for example to open it. The two lists at the top report here.",
            ),
            activated,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Down` / `Up`",
                        "Moves the active row; a single-selection list also selects it.",
                    ],
                    vec!["`Home` / `End`", "Moves to the first or the last row."],
                    vec![
                        "`Shift` + arrows, `Home`, `End`",
                        "Extends the selection, in a multiple-selection list.",
                    ],
                    vec![
                        "`Space`",
                        "Toggles the active row, in a multiple-selection list.",
                    ],
                    vec![
                        "`Ctrl+A` / `Cmd+A`",
                        "Selects all, in a multiple-selection list.",
                    ],
                    vec!["`Enter`", "Activates the active row."],
                    vec!["Letters", "Jump to the next row whose label matches."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::list_box::list_box",
                    "oito2_iced::widget::list_box::ListBox",
                    "oito2_iced::widget::list_box::SelectionChange",
                    "oito2_iced::collections::ListCollection",
                    "oito2_iced::collections::SelectionController",
                    "oito2_iced::collections::SelectionMode",
                    "oito2_iced::collections::SelectionOptions",
                ],
            ),
        ],
    )
}
