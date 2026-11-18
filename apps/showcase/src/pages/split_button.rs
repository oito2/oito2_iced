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

//! The SplitButton page: every variant and a disabled split button.

use iced::widget::row;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::menu;
use oito2_iced::widget::split_button::{Variant, split_button};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("split_button.rs");

/// The variants, with their names.
const VARIANTS: [(Variant, &str); 3] = [
    (Variant::Elevated, "Elevated"),
    (Variant::Filled, "Filled"),
    (Variant::Outlined, "Outlined"),
];

/// The split button whose menu is open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    /// The preview.
    Preview,
    /// The split button of a variant.
    Variant(Variant),
}

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    open: Option<Menu>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A menu opened, or closed when `false`.
    Open(Menu, bool),
    /// An action was chosen.
    Action(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Open(menu, true) => state.open = Some(menu),
        Message::Open(menu, false) => {
            if state.open == Some(menu) {
                state.open = None;
            }
        }
        Message::Action(action) => shell.toast(action, InfoType::Success),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let open = |menu: Menu| state.open == Some(menu);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        split_button(
            "Save",
            [
                menu::item("Save as…").on_press(Message::Action("Saved as")),
                menu::item("Save a copy").on_press(Message::Action("Copy saved")),
            ],
        )
        .on_press(Message::Action("Saved"))
        .open(open(Menu::Preview))
        .on_open_change(|open| Message::Open(Menu::Preview, open)),
        // #endregion
    );

    let variants = doc::example(
        ctx,
        SOURCE,
        &["variants"],
        // #region variants
        row(VARIANTS.map(|(variant, name)| {
            split_button(
                name,
                [menu::item("Save as…").on_press(Message::Action("Saved as"))],
            )
            .variant(variant)
            .on_press(Message::Action("Saved"))
            .open(open(Menu::Variant(variant)))
            .on_open_change(move |open| Message::Open(Menu::Variant(variant), open))
            .into()
        }))
        .spacing(spacing::SM)
        .wrap(),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        split_button::<Message>("Save", []),
        // #endregion
    );

    doc::page(
        "SplitButton",
        "A button with a main action and an attached arrow that opens a menu of related \
         actions.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The main part shows the label and publishes the message of `on_press`. The \
                 arrow part opens a menu of `menu` items below itself and closes it when \
                 pressed again; the open state lives in the application, through `open` and \
                 `on_open_change`. Without `on_press` both parts are disabled, and without \
                 `on_open_change` the arrow is.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::split_button::{Variant, split_button};",
            ),
            doc::note(
                "Use a split button when one action is the usual choice and a few variants of \
                 it are needed less often. When no action is the default, a popup menu button \
                 is clearer.",
            ),
            doc::heading("Examples"),
            doc::subheading("Variants"),
            doc::prose(
                "`variant` picks the look of both parts: `Variant::Elevated`, the default, a \
                 neutral button; `Variant::Filled`, a primary button; and \
                 `Variant::Outlined`.",
            ),
            variants,
            doc::subheading("Disabled"),
            doc::prose("Without `on_press`, neither part responds."),
            disabled,
            doc::note(
                "The two parts share one rounded shape, split by a one-pixel gap, and each is \
                 a focus stop of its own. `direction` places the rounded corners, the menu and \
                 its items for right to left.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Tab",
                        "Moves the focus to the main part, then to the arrow.",
                    ],
                    vec!["Enter, Space", "Activates the focused part."],
                    vec!["Up, Down", "Move between the items of the open menu."],
                    vec!["Escape", "Closes the menu."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::split_button::split_button",
                    "oito2_iced::widget::split_button::SplitButton",
                    "oito2_iced::widget::split_button::Variant",
                ],
            ),
        ],
    )
}
