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

//! The IconButton page: plain, disabled and toggle icon buttons, and the
//! back and close buttons.

use iced::widget::{column, row};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{Icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button::{back_button, close_button, icon_button};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("icon_button.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    styles: Vec<&'static str>,
    muted: bool,
    last: Option<&'static str>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            styles: vec!["Italic"],
            muted: false,
            last: None,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A plain button was pressed.
    Pressed(&'static str),
    /// A text style toggle was pressed.
    Style(&'static str),
    /// The mute toggle was pressed.
    Mute,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pressed(name) => state.last = Some(name),
        // #region toggle-update
        Message::Style(name) => {
            if let Some(index) = state.styles.iter().position(|style| *style == name) {
                state.styles.remove(index);
            } else {
                state.styles.push(name);
            }
        }
        // #endregion
        Message::Mute => state.muted = !state.muted,
    }

    Task::none()
}

// #region toggle
/// A toggle button for the text style `name`.
fn toggle<'a>(state: &State, name: &'static str, icon: Icon) -> Element<'a, Message, Theme> {
    icon_button(icon)
        .tooltip(name)
        .selected(state.styles.contains(&name))
        .on_press(Message::Style(name))
        .into()
}
// #endregion

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![
            // #region preview
            icon_button(lucide::SETTINGS)
                .tooltip("Settings")
                .on_press(Message::Pressed("Settings")),
            // #endregion
            doc::caption(state.last.map_or_else(
                || "Not pressed yet.".to_owned(),
                |name| format!("Pressed: {name}"),
            )),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let buttons = doc::example(
        ctx,
        SOURCE,
        &["buttons"],
        // #region buttons
        row![
            icon_button(lucide::SEARCH)
                .tooltip("Search")
                .on_press(Message::Pressed("Search")),
            icon_button::<Message>(lucide::TRASH).tooltip("Delete"),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    let toggles = doc::example(
        ctx,
        SOURCE,
        &["toggle", "toggles", "toggle-update"],
        // #region toggles
        row![
            toggle(state, "Italic", lucide::ITALIC),
            toggle(state, "Underline", lucide::UNDERLINE),
            toggle(state, "Strikethrough", lucide::STRIKETHROUGH),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    let selected_icon = doc::example(
        ctx,
        SOURCE,
        &["selected-icon"],
        // #region selected-icon
        icon_button(lucide::VOLUME_2)
            .selected_icon(lucide::VOLUME_X)
            .tooltip("Mute")
            .selected(state.muted)
            .on_press(Message::Mute),
        // #endregion
    );

    let back_close = doc::example(
        ctx,
        SOURCE,
        &["back-close"],
        // #region back-close
        row![
            back_button(l10n, ctx.direction).on_press(Message::Pressed("Back")),
            close_button(l10n).on_press(Message::Pressed("Close")),
            close_button::<Message>(l10n),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    doc::page(
        "IconButton",
        "A square button that shows an icon, as a plain button or as a toggle, as tall as the \
         control height of the density.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `icon_button` an icon, then a press message with `on_press`. The \
                 `tooltip` names the action, which an icon alone may not make clear, so give \
                 every icon button one. Without `on_press` the button is disabled and skipped \
                 by focus traversal.",
            ),
            doc::code(ctx, "use oito2_iced::widget::button::icon_button;"),
            doc::heading("Examples"),
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_press`, the button is dimmed and ignores presses, like the delete \
                 button here. `on_press_maybe` takes an `Option` to switch between both.",
            ),
            buttons,
            doc::subheading("Toggles"),
            doc::prose(
                "With `selected`, the button is a toggle: while selected it fills with the \
                 secondary container color. The application keeps the state and flips it in \
                 the press message.",
            ),
            toggles,
            doc::subheading("Selected icon"),
            doc::prose(
                "A toggle shows `selected_icon`, when given, instead of its icon while selected.",
            ),
            selected_icon,
            doc::subheading("Back and close"),
            doc::prose(
                "`back_button` shows a chevron pointing to the start of the reading direction, \
                 and `close_button` an \"x\", with the localized \"Back\" and \"Close\" \
                 tooltips. Both are icon buttons, disabled without `on_press`.",
            ),
            back_close,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the button."],
                    vec!["`Enter` / `Space`", "Presses the focused button."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::button::icon_button",
                    "oito2_iced::widget::button::IconButton",
                    "oito2_iced::widget::button::back_button",
                    "oito2_iced::widget::button::close_button",
                ],
            ),
        ],
    )
}
