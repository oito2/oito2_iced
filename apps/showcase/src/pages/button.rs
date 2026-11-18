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
//! The Button page: the OITO2 text buttons, floating action buttons and
//! the button classes of the theme.

use iced::widget::{button as native, row, text};
use iced::{Alignment, Background, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button::{self, Class};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("button.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    presses: u32,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A button was pressed.
    Press,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Press => state.presses += 1,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        row![
            button::text("Cancel").on_press(Message::Press),
            button::filled("Save").on_press(Message::Press),
        ]
        .spacing(spacing::SM),
        // #endregion
    );

    let variants = doc::example(
        ctx,
        SOURCE,
        &["variants"],
        // #region variants
        row![
            button::filled("Filled").on_press(Message::Press),
            button::tonal("Tonal").on_press(Message::Press),
            button::elevated("Elevated").on_press(Message::Press),
            button::outlined("Outlined").on_press(Message::Press),
            button::text("Text").on_press(Message::Press),
            button::danger("Danger").on_press(Message::Press),
        ]
        .spacing(spacing::SM)
        .wrap(),
        // #endregion
    );

    let presses = match state.presses {
        0 => "Not pressed yet.".to_owned(),
        1 => "Pressed once.".to_owned(),
        presses => format!("Pressed {presses} times."),
    };

    let icons = doc::example(
        ctx,
        SOURCE,
        &["icons"],
        // #region icons
        row![
            button::filled("Download")
                .icon(lucide::DOWNLOAD)
                .on_press(Message::Press),
            button::tonal("Share")
                .icon(lucide::SHARE_2)
                .on_press(Message::Press),
            button::outlined("Upload")
                .icon(lucide::UPLOAD)
                .tooltip("Upload a file")
                .on_press(Message::Press),
        ]
        .spacing(spacing::SM)
        .wrap(),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        row![
            button::filled("Filled"),
            button::tonal("Tonal"),
            button::outlined("Outlined"),
            button::text("Text").on_press_maybe(None),
        ]
        .spacing(spacing::SM)
        .wrap(),
        // #endregion
    );

    let width = doc::example(
        ctx,
        SOURCE,
        &["width"],
        iced::widget::container(
            // #region width
            button::filled("Continue")
                .width(Length::Fill)
                .on_press(Message::Press),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    let content = doc::example(
        ctx,
        SOURCE,
        &["content"],
        // #region content
        button::button(
            row![text("Inbox"), text("12").size(12)]
                .spacing(spacing::SM)
                .align_y(Alignment::Center),
        )
        .class(Class::Outlined)
        .on_press(Message::Press),
        // #endregion
    );

    let fabs = doc::example(
        ctx,
        SOURCE,
        &["fabs"],
        // #region fabs
        row![
            button::fab(lucide::PENCIL)
                .tooltip("Compose")
                .on_press(Message::Press),
            button::fab(lucide::PLUS)
                .label("New item")
                .on_press(Message::Press),
        ]
        .spacing(spacing::MD)
        .align_y(Alignment::Center),
        // #endregion
    );

    let classes = doc::example(
        ctx,
        SOURCE,
        &["native"],
        // #region native
        row![
            native(text("Native tonal"))
                .class(Class::Tonal)
                .padding([spacing::SM, spacing::LG])
                .on_press(Message::Press),
            native(text("Chrome"))
                .class(Class::Chrome)
                .padding([spacing::SM, spacing::LG])
                .on_press(Message::Press),
        ]
        .spacing(spacing::SM),
        // #endregion
    );

    let custom = doc::example(
        ctx,
        SOURCE,
        &["custom"],
        // #region custom
        button::filled("Custom")
            .on_press(Message::Press)
            .class(Class::Custom(Box::new(|theme: &Theme, status| {
                let roles = theme.roles();
                let (background, text_color) = match status {
                    native::Status::Hovered | native::Status::Pressed => {
                        (roles.tertiary, roles.on_tertiary)
                    }
                    _ => (roles.tertiary_container, roles.on_tertiary_container),
                };

                native::Style {
                    background: Some(Background::Color(background)),
                    text_color,
                    border: iced::Border::default().rounded(radius::LARGE),
                    ..native::Style::default()
                }
            }))),
        // #endregion
    );

    doc::page(
        "Button",
        "The Material text buttons and the floating action button, with keyboard focus.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Each builder of the `button` module makes a button of one class with a \
                 label: `filled` for the main action of a view, `tonal`, `elevated` and \
                 `outlined` for secondary actions, `text` for low-emphasis actions and \
                 `danger` for destructive ones. A button is at least as tall as the control \
                 height of the density and grows with its label.",
            ),
            doc::code(ctx, "use oito2_iced::widget::button;"),
            doc::prose(
                "With `on_press` the button takes part in focus traversal, shows the focus \
                 ring while focused and publishes its message on a click, Enter or Space.",
            ),
            doc::heading("Examples"),
            doc::subheading("Variants"),
            variants,
            doc::caption(presses),
            doc::subheading("Icons and tooltips"),
            doc::prose(
                "`icon` places an icon before the label, at the icon size of the density. \
                 `tooltip` shows a label below the button while the pointer rests on it.",
            ),
            icons,
            doc::subheading("Disabled"),
            doc::prose(
                "Without a press message, or with `on_press_maybe(None)`, the button is \
                 disabled: it greys out and Tab skips it.",
            ),
            disabled,
            doc::subheading("Width"),
            doc::prose(
                "A button sizes to its label, at least 64 pixels wide. With `width` it \
                 takes that width and centers its content.",
            ),
            width,
            doc::subheading("Any content"),
            doc::prose(
                "`button::button` takes any content, laid out like a text button, in the \
                 filled class unless `class` says otherwise.",
            ),
            content,
            doc::subheading("Floating action buttons"),
            doc::prose(
                "`fab` is a 56-pixel square on the primary container with the large radius \
                 and a low shadow. `label` turns it into an extended button with the label \
                 after the icon.",
            ),
            fabs,
            doc::note(
                "Square icon buttons, including the back and close buttons, have their own \
                 page: IconButton.",
            ),
            doc::heading("Classes"),
            doc::prose(
                "The theme implements the button catalog of Iced, so `Class` styles the \
                 native `iced::widget::button` as well. Besides the classes of the \
                 builders, `Class::Chrome` is the fully rounded, transparent style of title \
                 bar buttons, and `Class::Icon` and `Class::IconSelected` style icon \
                 buttons.",
            ),
            classes,
            doc::table(
                &["Class", "Use"],
                &[
                    vec!["`Filled`", "Primary background: the main action of a view."],
                    vec!["`Tonal`", "Secondary container: secondary actions."],
                    vec![
                        "`Elevated`",
                        "Raised neutral surface with a shadow on hover.",
                    ],
                    vec!["`Outlined`", "Transparent with an outline."],
                    vec!["`Text`", "Transparent, primary text: low-emphasis actions."],
                    vec!["`Danger`", "Error background: destructive actions."],
                    vec![
                        "`Chrome`",
                        "Transparent and fully rounded: title bar buttons.",
                    ],
                    vec!["`Fab`", "Primary container, large radius and a low shadow."],
                    vec!["`Icon`", "Transparent with the variant content color."],
                    vec![
                        "`IconSelected`",
                        "Secondary container: a selected toggle icon.",
                    ],
                ],
            ),
            doc::subheading("Custom style"),
            doc::prose(
                "`Class::Custom` takes a closure of the theme and the status, for a style \
                 the catalog does not have.",
            ),
            custom,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the next enabled button."],
                    vec!["`Enter` / `Space`", "Publishes the press message."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::button::button",
                    "oito2_iced::widget::button::filled",
                    "oito2_iced::widget::button::tonal",
                    "oito2_iced::widget::button::elevated",
                    "oito2_iced::widget::button::outlined",
                    "oito2_iced::widget::button::text",
                    "oito2_iced::widget::button::danger",
                    "oito2_iced::widget::button::fab",
                    "oito2_iced::widget::button::Button",
                    "oito2_iced::widget::button::MIN_WIDTH",
                    "oito2_iced::widget::button::FAB_SIZE",
                    "oito2_iced::theme::button::Class",
                ],
            ),
        ],
    )
}
