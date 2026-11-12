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

//! Client-side window chrome for undecorated windows: a title bar and a
//! header bar that drag, maximize and open the system menu, window control
//! buttons in the visual family of each desktop, and resize handles on the
//! edges.
//!
//! The chrome publishes [`Action`] values through the `on_action` mapping
//! of each builder; the application turns them into window tasks with
//! [`perform`] (or [`perform_latest`] for a single-window application).
//! The chrome follows the [`WindowState`] the application keeps from the
//! window events.

mod control;
mod state;
mod title_bar;

pub use control::{ControlType, Labels, WindowControl, controls, window_control};
pub use state::{Placement, WindowState};
pub use title_bar::{INACTIVE_OPACITY, TitleBar, header_bar, title_bar};

use iced::widget::{Space, column, mouse_area, row, stack};
use iced::window::{self, Direction};
use iced::{Element, Length, Task, mouse};

use crate::Theme;

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
