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
//! The DialogTitleBar page: the title bar of a dialog with a plain title,
//! an icon, an animated icon, the window control styles and a custom close
//! label.

use iced::widget::{column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::foundation::{WindowButton, WindowButtonLayout, WindowControlStyle};
use oito2_iced::icon::{animated, icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::dialog::{self, TitleBar};
use oito2_iced::window::Labels;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("dialog_title_bar.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    closes: u32,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A close button was pressed.
    Close,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Close => state.closes += 1,
    }

    Task::none()
}

// #region dialog
/// A dialog drawn in place with `bar` at its top.
fn sample<'a>(bar: TitleBar<'a, Message>) -> Element<'a, Message, Theme> {
    container(
        dialog::dialog(text(
            "The close button follows the window control style and side, and publishes \
             the dismiss message.",
        ))
        .title_bar(bar)
        .on_dismiss(Message::Close),
    )
    .width(Length::Fixed(420.0))
    .into()
}
// #endregion

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["dialog", "preview"],
        // #region preview
        sample(
            dialog::title_bar()
                .title("Preferences")
                .labels(Labels::new(l10n)),
        ),
        // #endregion
    );

    let closes = match state.closes {
        0 => "Not closed yet.".to_owned(),
        1 => "Closed once.".to_owned(),
        closes => format!("Closed {closes} times."),
    };

    let with_icon = doc::example(
        ctx,
        SOURCE,
        &["icon"],
        // #region icon
        sample(
            dialog::title_bar()
                .title("Settings")
                .leading(icon(lucide::SETTINGS)),
        ),
        // #endregion
    );

    let with_animated = doc::example(
        ctx,
        SOURCE,
        &["animated"],
        // #region animated
        sample(
            dialog::title_bar()
                .title("Saved")
                .leading(animated::icon(animated::OK)),
        ),
        // #endregion
    );

    let styles = doc::example(
        ctx,
        SOURCE,
        &["styles"],
        column(
            [
                WindowControlStyle::Breeze,
                WindowControlStyle::Windows,
                WindowControlStyle::Cosmic,
            ]
            .map(|style| {
                // #region styles
                sample(
                    dialog::title_bar()
                        .title(format!("{style:?}"))
                        .control_style(style),
                )
                // #endregion
            }),
        )
        .spacing(spacing::LG),
    );

    let leading = doc::example(
        ctx,
        SOURCE,
        &["layout"],
        // #region layout
        sample(
            dialog::title_bar()
                .title("Close at the start")
                .button_layout(&WindowButtonLayout::new(
                    vec![WindowButton::Close],
                    Vec::new(),
                )),
        ),
        // #endregion
    );

    let labels = doc::example(
        ctx,
        SOURCE,
        &["labels"],
        // #region labels
        sample(dialog::title_bar().title("Export").labels(Labels {
            close: "Cancel export".to_owned(),
            ..Labels::new(l10n)
        })),
        // #endregion
    );

    doc::page(
        "DialogTitleBar",
        "The title bar of a dialog: a title and a close button that dismisses the dialog.",
        [
            preview,
            doc::caption(closes),
            doc::heading("Usage"),
            doc::prose(
                "Give `dialog::title_bar` to the `title_bar` of a dialog. Its close button \
                 publishes the dismiss message of the dialog, or the message of `on_close` \
                 when it is set. The bar is the title bar height, transparent, has no \
                 bottom border and does not move the window when dragged. The title is \
                 centered, except with the Windows style.",
            ),
            doc::code(ctx, "use oito2_iced::widget::dialog;"),
            doc::prose(
                "The close button follows the window control style of `control_style`, \
                 GNOME by default, with the GNOME style in place of none and of macOS, and \
                 sits on the side of the close button in `button_layout`. Pass the style \
                 and the button layout of the desktop, from the system settings, so the \
                 dialog matches the window.",
            ),
            doc::note(
                "For the title bar of the application window, use `title_bar` of the `window` module.",
            ),
            doc::heading("Examples"),
            doc::subheading("With an icon"),
            doc::prose(
                "`leading` shows an element before the title, such as an icon that tells \
                 what the dialog is about.",
            ),
            with_icon,
            doc::subheading("With an animated icon"),
            doc::prose("The leading element can also be an animated icon."),
            with_animated,
            doc::subheading("Control styles"),
            doc::prose("`control_style` draws the close button of a desktop."),
            styles,
            doc::subheading("Button layout"),
            doc::prose(
                "With `button_layout`, the close button sits at the start when the leading \
                 buttons of the layout include it.",
            ),
            leading,
            doc::subheading("Close label"),
            doc::prose(
                "`labels` sets the labels of the window buttons, the close label being the \
                 tooltip of the close button. `Labels::new` takes the strings of the \
                 language; without `labels` they are English.",
            ),
            labels,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::dialog::title_bar",
                    "oito2_iced::widget::dialog::TitleBar",
                    "oito2_iced::window::Labels",
                ],
            ),
        ],
    )
}
