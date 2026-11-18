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

//! The TitleBar page: the title bar of an undecorated window in every
//! control style and button layout, and the inactive window state.

use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Length, Task, border};

use oito2_iced::Theme;
use oito2_iced::foundation::{WindowButtonLayout, WindowControlStyle};
use oito2_iced::icon::lucide;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::toggles;
use oito2_iced::window::{self, Labels, TitleBar, WindowState, title_bar};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("title_bar.rs");

/// The control styles shown on the page, with the desktops that use them.
const STYLES: [(&str, WindowControlStyle); 6] = [
    (
        "GNOME, Budgie, Cinnamon, Pantheon",
        WindowControlStyle::Gnome,
    ),
    ("KDE Plasma", WindowControlStyle::Breeze),
    ("Xfce", WindowControlStyle::Xfwm),
    ("COSMIC", WindowControlStyle::Cosmic),
    ("Windows", WindowControlStyle::Windows),
    ("macOS (drawn for preview)", WindowControlStyle::Macos),
];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    window: WindowState,
    last: Option<window::Action>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A title bar asked for a window action.
    Window(window::Action),
    /// The window active switch changed.
    Active(bool),
    /// A button of the bar was pressed.
    Pressed(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // The page records the actions instead of applying them to the
        // showcase window.
        Message::Window(action) => state.last = Some(action),
        Message::Active(active) => state.window.focused = active,
        Message::Pressed(name) => shell.toast(name, InfoType::Information),
    }

    Task::none()
}

/// A miniature window with `bar`, its top corners rounded, above an empty
/// page.
fn frame(bar: TitleBar<'_, Message>) -> Element<'_, Message, Theme> {
    let bar = bar.radius(border::top(radius::WINDOW - 1.0));

    border_container(column![bar, Space::new().height(48)])
        .radius(radius::WINDOW)
        .padding(1)
        .width(Length::Fixed(560.0))
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let labels = Labels::new(&ctx.l10n);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        frame(
            // #region preview
            title_bar("Documents", Space::new(), Message::Window)
                .labels(labels.clone())
                .state(state.window),
            // #endregion
        ),
    );

    let last = match state.last {
        Some(action) => format!("Last window action: {action:?}"),
        None => "Press, double-click or right-click the empty area of a bar.".to_owned(),
    };

    let styles = doc::example(
        ctx,
        SOURCE,
        &["styles"],
        column(STYLES.into_iter().map(|(name, style)| {
            frame(
                // #region styles
                title_bar(name, Space::new(), Message::Window)
                    .control_style(style)
                    .button_layout(WindowButtonLayout::default_for(style))
                    .labels(labels.clone())
                    .state(state.window),
                // #endregion
            )
        }))
        .spacing(spacing::MD),
    );

    let layouts = doc::example(
        ctx,
        SOURCE,
        &["layouts"],
        column(
            // #region layouts
            [
                (
                    "GNOME default: appmenu:close",
                    WindowButtonLayout::parse_gnome("appmenu:close"),
                ),
                (
                    "Left side: close,minimize,maximize:",
                    WindowButtonLayout::parse_gnome("close,minimize,maximize:"),
                ),
                (
                    "KDE: ButtonsOnLeft=X, ButtonsOnRight=IA",
                    WindowButtonLayout::parse_kde("X", "IA"),
                ),
                ("Xfce: O|HMC", WindowButtonLayout::parse_xfwm("O|HMC")),
            ]
            .into_iter()
            .map(|(name, layout)| {
                title_bar(name, Space::new(), Message::Window)
                    .button_layout(layout)
                    .labels(labels.clone())
            })
            // #endregion
            .map(frame),
        )
        .spacing(spacing::MD),
    );

    let actions = doc::example(
        ctx,
        SOURCE,
        &["actions"],
        frame(
            // #region actions
            title_bar(
                "Settings",
                icon_button(lucide::ARROW_LEFT)
                    .tooltip("Back")
                    .on_press(Message::Pressed("Back")),
                Message::Window,
            )
            .end(
                icon_button(lucide::SEARCH)
                    .tooltip("Search")
                    .on_press(Message::Pressed("Search")),
            )
            .center_title(false)
            .labels(labels.clone()),
            // #endregion
        ),
    );

    let undecorated = doc::example(
        ctx,
        SOURCE,
        &["undecorated"],
        frame(
            // #region undecorated
            title_bar("Undecorated", Space::new(), Message::Window)
                .control_style(WindowControlStyle::None),
            // #endregion
        ),
    );

    let inactive = doc::example(
        ctx,
        SOURCE,
        &["inactive"],
        column![
            row![
                text("Window active").width(Length::Fill),
                toggles::switch(state.window.focused).on_toggle(Message::Active),
            ]
            .align_y(Alignment::Center),
            frame(
                // #region inactive
                title_bar("Inactive preview", Space::new(), Message::Window)
                    .control_style(WindowControlStyle::Macos)
                    .state(WindowState {
                        focused: state.window.focused,
                        ..WindowState::default()
                    })
                    .labels(labels),
                // #endregion
            ),
        ]
        .spacing(spacing::SM)
        .width(Length::Fixed(560.0)),
    );

    doc::page(
        "TitleBar",
        "A title bar for an undecorated window, with a title, actions and window control \
         buttons drawn in the style of the user's desktop.",
        [
            preview,
            container(doc::caption(last)).center_x(Length::Fill).into(),
            doc::heading("Usage"),
            doc::prose(
                "`title_bar` shows a title, an element after the leading window buttons and \
                 the window control buttons on the sides of a `WindowButtonLayout`. Its empty \
                 area publishes `window::Action::Drag` on a press, `ToggleMaximize` on a double \
                 click and `ShowSystemMenu` on a right press; the buttons publish `Minimize`, \
                 `ToggleMaximize` and `Close`. The application turns each action into a window \
                 task with `window::perform`, or `window::perform_latest` for a single window.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::window::{self, title_bar};\n\n\
                 // In `view`:\n\
                 title_bar(\"My app\", Space::new(), Message::Window)\n    \
                     .state(self.window)\n\n\
                 // In `update`:\n\
                 Message::Window(action) => window::perform_latest(action),",
            ),
            doc::prose(
                "Open the window with `decorations: false` so the bar replaces the native \
                 one, and wrap the whole window in `window::resize_handles` to resize it from \
                 its edges. Keep a `WindowState` in the model and feed it the events of \
                 `iced::window::events`: `WindowState::update` applies the focus changes and \
                 tells when to query the maximized flag again with `WindowState::query`, whose \
                 result goes to `WindowState::apply`. While maximized, the maximize button \
                 turns into a restore button.",
            ),
            doc::prose(
                "`header_bar` builds the same bar for application controls, with `start` and \
                 `end` items and a search in place of the title; see the HeaderBar page. The \
                 control style and the button layout of the user's desktop come from the \
                 system settings; see the Foundation page.",
            ),
            doc::heading("Examples"),
            doc::subheading("Control styles"),
            doc::prose(
                "`control_style` draws the buttons in the look of a desktop. \
                 `WindowButtonLayout::default_for` gives each style its usual layout: leading \
                 close, minimize and maximize for macOS, and trailing minimize, maximize and \
                 close otherwise. The title is centered, except with the Windows style.",
            ),
            styles,
            doc::subheading("Button layouts"),
            doc::prose(
                "`WindowButtonLayout` lists the buttons before and after the title. It parses \
                 the formats of the desktops: `parse_gnome` for the GNOME `button-layout` \
                 setting, `parse_kde` for the KWin `ButtonsOnLeft` and `ButtonsOnRight` values, \
                 and `parse_xfwm` for the Xfce `button_layout`. Entries other than minimize, \
                 maximize and close are ignored.",
            ),
            layouts,
            doc::subheading("Leading and actions"),
            doc::prose(
                "The element given to `title_bar` goes after the leading window buttons, such \
                 as a back button, and `end` adds items before the trailing ones. \
                 `center_title(false)` starts the title after the leading items.",
            ),
            actions,
            doc::subheading("Without window buttons"),
            doc::prose(
                "`WindowControlStyle::None` shows the bar without window controls. `height` \
                 and `radius` change the size and the corners of the bar, and \
                 `leading_inset` keeps room for buttons drawn by the operating system.",
            ),
            undecorated,
            doc::subheading("Inactive window"),
            doc::prose(
                "While the `WindowState` says the window does not have the input focus, the \
                 bar content fades to `INACTIVE_OPACITY` (0.7) and macOS buttons turn grey.",
            ),
            inactive,
            doc::note(
                "The window buttons stay out of keyboard focus traversal, as native window \
                 buttons do. Each one is labeled Minimize, Maximize, Restore or Close from the \
                 `Labels` given to `labels`, English by default.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::window::title_bar",
                    "oito2_iced::window::TitleBar",
                    "oito2_iced::window::Action",
                    "oito2_iced::window::perform",
                    "oito2_iced::window::perform_latest",
                    "oito2_iced::window::resize_handles",
                    "oito2_iced::window::WindowState",
                    "oito2_iced::window::Labels",
                    "oito2_iced::window::controls",
                    "oito2_iced::window::window_control",
                    "oito2_iced::window::ControlType",
                    "oito2_iced::window::INACTIVE_OPACITY",
                ],
            ),
        ],
    )
}
