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

//! Client-side window chrome for undecorated windows: a title bar that
//! drags, maximizes and opens the system menu, window control buttons, and
//! resize handles on the edges.
//!
//! The chrome publishes [`Action`] values through the `on_action` mapping
//! of each builder; the application turns them into window tasks with
//! [`perform`] (or [`perform_latest`] for a single-window application).

use iced::widget::{Space, button, column, container, mouse_area, row, stack, text};
use iced::window::{self, Direction};
use iced::{Alignment, Element, Length, Task, mouse};

use crate::Theme;
use crate::theme::button::Class as ButtonClass;
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{sizes, spacing};

/// Thickness of the edge resize handles, in logical pixels.
pub const RESIZE_BORDER: f32 = 6.0;
/// Side of the corner resize handles, in logical pixels.
pub const RESIZE_CORNER: f32 = 12.0;

/// A request from the window chrome.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    /// Moves the window with the pointer.
    Drag,
    /// Resizes the window with the pointer, from an edge or a corner.
    Resize(Direction),
    /// Maximizes or restores the window.
    ToggleMaximize,
    /// Minimizes the window.
    Minimize,
    /// Closes the window.
    Close,
    /// Opens the window menu of the desktop.
    ShowSystemMenu,
}

/// The window task for an [`Action`] on the window `id`.
pub fn perform<T>(id: window::Id, action: Action) -> Task<T> {
    match action {
        Action::Drag => window::drag(id),
        Action::Resize(direction) => window::drag_resize(id, direction),
        Action::ToggleMaximize => window::toggle_maximize(id),
        Action::Minimize => window::minimize(id, true),
        Action::Close => window::close(id),
        Action::ShowSystemMenu => window::show_system_menu(id),
    }
}

/// The window task for an [`Action`] on the most recently opened window.
pub fn perform_latest<T>(action: Action) -> Task<T>
where
    T: Send + 'static,
{
    window::latest().and_then(move |id| perform(id, action))
}

/// A title bar: the `title` centered, `start` items on the left and the
/// window controls on the right. A press on the bar drags the window, a
/// double click toggles maximization and a right press opens the system
/// menu.
pub fn title_bar<'a, Message>(
    title: impl text::IntoFragment<'a>,
    start: impl Into<Element<'a, Message, Theme>>,
    on_action: impl Fn(Action) -> Message + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let bar = stack![
        container(text(title).font(crate::font::inter(iced::font::Weight::Bold)))
            .center(Length::Fill),
        row![
            start.into(),
            Space::new().width(Length::Fill),
            controls(&on_action)
        ]
        .align_y(Alignment::Center)
        .padding([0.0, spacing::SM]),
    ];

    mouse_area(
        container(bar)
            .class(ContainerClass::TitleBar)
            .width(Length::Fill)
            .height(sizes::TITLE_BAR_HEIGHT),
    )
    .on_press(on_action(Action::Drag))
    .on_double_click(on_action(Action::ToggleMaximize))
    .on_right_press(on_action(Action::ShowSystemMenu))
    .into()
}

/// The minimize, maximize and close buttons.
pub fn controls<'a, Message>(on_action: &impl Fn(Action) -> Message) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let control = |label: &'static str, action: Action| {
        button(container(text(label).size(14)).center(Length::Fill))
            .class(ButtonClass::Chrome)
            .padding(0)
            .width(sizes::WINDOW_CONTROL_SIZE)
            .height(sizes::WINDOW_CONTROL_SIZE)
            .on_press(on_action(action))
    };

    row![
        control("\u{2013}", Action::Minimize),
        control("\u{25A1}", Action::ToggleMaximize),
        control("\u{2715}", Action::Close),
    ]
    .spacing(spacing::MD)
    .align_y(Alignment::Center)
    .into()
}

/// Wraps the whole window `content` with eight invisible handles on the
/// edges and corners that resize the window.
pub fn resize_handles<'a, Message>(
    content: impl Into<Element<'a, Message, Theme>>,
    on_action: impl Fn(Action) -> Message + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let handle = |direction: Direction, width: Length, height: Length| {
        let interaction = match direction {
            Direction::North | Direction::South => mouse::Interaction::ResizingVertically,
            Direction::East | Direction::West => mouse::Interaction::ResizingHorizontally,
            Direction::NorthWest | Direction::SouthEast => {
                mouse::Interaction::ResizingDiagonallyDown
            }
            Direction::NorthEast | Direction::SouthWest => mouse::Interaction::ResizingDiagonallyUp,
        };

        mouse_area(Space::new().width(width).height(height))
            .interaction(interaction)
            .on_press(on_action(Action::Resize(direction)))
    };

    let corner = Length::Fixed(RESIZE_CORNER);
    let edge = Length::Fixed(RESIZE_BORDER);

    let top = row![
        handle(Direction::NorthWest, corner, corner),
        column![
            handle(Direction::North, Length::Fill, edge),
            Space::new().height(RESIZE_CORNER - RESIZE_BORDER)
        ]
        .width(Length::Fill),
        handle(Direction::NorthEast, corner, corner),
    ];

    let middle = row![
        handle(Direction::West, edge, Length::Fill),
        Space::new().width(Length::Fill).height(Length::Fill),
        handle(Direction::East, edge, Length::Fill),
    ]
    .height(Length::Fill);

    let bottom = row![
        handle(Direction::SouthWest, corner, corner),
        column![
            Space::new().height(RESIZE_CORNER - RESIZE_BORDER),
            handle(Direction::South, Length::Fill, edge)
        ]
        .width(Length::Fill),
        handle(Direction::SouthEast, corner, corner),
    ];

    stack![content.into(), column![top, middle, bottom]].into()
}
