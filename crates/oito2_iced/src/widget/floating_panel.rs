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

//! A non-modal window inside the app that the user moves and resizes.
//!
//! [`floating_panel`] fills its parent, usually a layer of a
//! [`stack`](iced::widget::stack()) over the application content, to know
//! its bounds, but takes pointer input only on the panel itself. The
//! panel is a window title bar over the content: dragging the title bar
//! moves the panel and a double click maximizes or restores it; with
//! [`FloatingPanel::resizable`] its right and bottom edges and its corner
//! resize it. The title bar buttons minimize the panel to its title bar,
//! maximize it to the whole area and, with [`FloatingPanel::on_close`],
//! close it. With the focus on the title bar the arrow keys move the
//! panel by [`FloatingPanel::keyboard_step`] and Shift with the arrows
//! resizes it. The title bar keeps inside the area and the size stays
//! above [`FloatingPanel::min_size`].
//!
//! The rectangle and the minimized and maximized flags live in the
//! application as a [`PanelState`]: every move, resize, minimize or
//! maximize publishes the next state through [`FloatingPanel::on_change`],
//! including each step of a drag, and the application stores it. Without
//! [`FloatingPanel::on_change`] the panel stays still and its title bar is
//! skipped by Tab. A minimized panel keeps its content in the widget
//! tree, so the content keeps its state.
//!
//! Panels in the same stack keep the order of its layers: a press on a
//! panel does not bring it to the front, so the application reorders the
//! layers when it wants that.
//!
//! ```no_run
//! use iced::Length;
//! use iced::widget::{container, stack, text};
//! use oito2_iced::widget::floating_panel::{self, PanelState};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Panel(PanelState),
//!     Close,
//! }
//!
//! # fn view<'a>(panel: PanelState) -> oito2_iced::Element<'a, Message> {
//! stack![
//!     container(text("Page content")).center(Length::Fill),
//!     floating_panel::floating_panel(panel, "Inspector", text("Opacity: 100%"))
//!         .on_change(Message::Panel)
//!         .on_close(Message::Close),
//! ]
//! .into()
//! # }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::text;
use iced::{
    Border, Color, Element as IcedElement, Event, Length, Point, Rectangle, Size, Vector, border,
    mouse, touch,
};

use crate::foundation::{WindowButton, WindowButtonLayout, WindowControlStyle};
use crate::theme::container::elevation;
use crate::tokens::{radius, sizes};
use crate::window::{self, Action, Labels, RESIZE_BORDER, RESIZE_CORNER, WindowState};
use crate::{Element, Theme};

/// The default distance one arrow key press moves or resizes the panel.
pub const KEYBOARD_STEP: f32 = 16.0;

/// The default smallest size of a panel.
pub const MIN_SIZE: Size = Size::new(220.0, 140.0);

/// The default rectangle of a panel: 360 × 280 at (40, 40).
pub const DEFAULT_RECT: Rectangle = Rectangle {
    x: 40.0,
    y: 40.0,
    width: 360.0,
    height: 280.0,
};

/// The elevation of the panel shadow.
const ELEVATION: f32 = 8.0;

/// The width of the panel border.
const BORDER_WIDTH: f32 = 1.0;

/// The height of the title bar.
const BAR_HEIGHT: f32 = sizes::TITLE_BAR_HEIGHT;

/// The rectangle and the minimized and maximized flags of a floating
/// panel, kept by the application.
///
/// The rectangle is in the coordinates of the area the panel covers. The
/// methods return the next state of an interaction; the panel calls them
/// and publishes the result through [`FloatingPanel::on_change`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanelState {
    /// The position and size of the restored panel.
    pub rect: Rectangle,
    /// Whether only the title bar shows.
    pub minimized: bool,
    /// Whether the panel covers the whole area.
    pub maximized: bool,
}

impl Default for PanelState {
    /// A restored panel at [`DEFAULT_RECT`].
    fn default() -> Self {
        Self::new(DEFAULT_RECT)
    }
}

impl PanelState {
    /// A restored panel at `rect`.
    pub fn new(rect: Rectangle) -> Self {
        Self {
            rect,
            minimized: false,
            maximized: false,
        }
    }

    /// `rect` with the title bar inside an area of `bounds` and the size
    /// at least `min_size`: the width is at most the area width, the left
    /// edge keeps the panel inside the area and the top edge keeps the
    /// title bar inside it.
    pub fn clamp(rect: Rectangle, bounds: Size, min_size: Size) -> Rectangle {
        let width = rect.width.max(min_size.width);
        let height = rect.height.max(min_size.height);
        let max_left = (bounds.width - width).max(0.0);
        let max_top = (bounds.height - BAR_HEIGHT).max(0.0);

        Rectangle {
            x: rect.x.max(0.0).min(max_left),
            y: rect.y.max(0.0).min(max_top),
            width: width.min(bounds.width).max(0.0),
            height,
        }
    }

    /// The rectangle the panel shows in an area of `bounds`: the whole
    /// area while maximized, the clamped rectangle otherwise, only the
    /// title bar and the border tall while minimized.
    pub fn placed(self, bounds: Size, min_size: Size) -> Rectangle {
        if self.maximized {
            return Rectangle::with_size(bounds);
        }

        let rect = Self::clamp(self.rect, bounds, min_size);

        if self.minimized {
            Rectangle {
                height: BAR_HEIGHT + 2.0 * BORDER_WIDTH,
                ..rect
            }
        } else {
            rect
        }
    }

    /// The state with the rectangle moved by `delta` and clamped; a
    /// maximized panel does not move.
    #[must_use]
    pub fn moved(self, delta: Vector, bounds: Size, min_size: Size) -> Self {
        if self.maximized {
            return self;
        }

        Self {
            rect: Self::clamp(self.rect + delta, bounds, min_size),
            ..self
        }
    }

    /// The state with the rectangle grown by `delta` and clamped; a
    /// maximized panel keeps its size.
    #[must_use]
    pub fn resized(self, delta: Vector, bounds: Size, min_size: Size) -> Self {
        if self.maximized {
            return self;
        }

        let rect = Rectangle {
            width: self.rect.width + delta.x,
            height: self.rect.height + delta.y,
            ..self.rect
        };

        Self {
            rect: Self::clamp(rect, bounds, min_size),
            ..self
        }
    }

    /// The state minimized to the title bar, or restored; minimizing
    /// leaves the maximized state.
    #[must_use]
    pub fn toggle_minimized(self) -> Self {
        let minimized = !self.minimized;

        Self {
            minimized,
            maximized: self.maximized && !minimized,
            ..self
        }
    }

    /// The state maximized to the whole area, or restored; maximizing
    /// leaves the minimized state.
    #[must_use]
    pub fn toggle_maximized(self) -> Self {
        let maximized = !self.maximized;

        Self {
            maximized,
            minimized: self.minimized && !maximized,
            ..self
        }
    }
}

/// A non-modal window inside the app. See [`floating_panel`].
pub struct FloatingPanel<'a, Message> {
    state: PanelState,
    title: text::Fragment<'a>,
    content: Element<'a, Message>,
    on_change: Option<Box<dyn Fn(PanelState) -> Message + 'a>>,
    on_close: Option<Message>,
    min_size: Size,
    resizable: bool,
    control_style: WindowControlStyle,
    labels: Labels,
    keyboard_step: f32,
    id: Option<Id>,
}

/// A panel titled `title` showing `content`, placed and sized by `state`.
/// Without [`FloatingPanel::on_change`] it stays still.
pub fn floating_panel<'a, Message>(
    state: PanelState,
    title: impl text::IntoFragment<'a>,
    content: impl Into<Element<'a, Message>>,
) -> FloatingPanel<'a, Message> {
    FloatingPanel {
        state,
        title: title.into_fragment(),
        content: content.into(),
        on_change: None,
        on_close: None,
        min_size: MIN_SIZE,
        resizable: true,
        control_style: WindowControlStyle::Gnome,
        labels: Labels::default(),
        keyboard_step: KEYBOARD_STEP,
        id: None,
    }
}

impl<'a, Message> FloatingPanel<'a, Message> {
    /// Sets the function that turns the next state into a message, which
    /// lets the user move, resize, minimize and maximize the panel.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(PanelState) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the message of the close button; without it the title bar
    /// shows no close button.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Sets the smallest size the panel can be resized to ([`MIN_SIZE`] by
    /// default).
    #[must_use]
    pub fn min_size(mut self, min_size: Size) -> Self {
        self.min_size = min_size;
        self
    }

    /// Sets whether the edges and the corner resize the panel (`true` by
    /// default).
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Sets the look of the title bar buttons
    /// ([`WindowControlStyle::Gnome`] by default).
    #[must_use]
    pub fn control_style(mut self, style: WindowControlStyle) -> Self {
        self.control_style = style;
        self
    }

    /// Sets the labels of the title bar buttons (English by default).
    #[must_use]
    pub fn labels(mut self, labels: Labels) -> Self {
        self.labels = labels;
        self
    }

    /// Sets how far one arrow key press moves or resizes the panel, in
    /// logical pixels ([`KEYBOARD_STEP`] by default).
    #[must_use]
    pub fn keyboard_step(mut self, step: f32) -> Self {
        self.keyboard_step = step;
        self
    }

    /// Sets the widget [`Id`] of the title bar, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// An edge or the corner that resizes the panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grip {
    Right,
    Bottom,
    Corner,
}

impl Grip {
    /// The grip of `rect` under `point`, the corner first.
    fn at(rect: Rectangle, point: Point) -> Option<Self> {
        let right = rect.x + rect.width;
        let bottom = rect.y + rect.height;
        let inside = |x0: f32, y0: f32, x1: f32, y1: f32| {
            point.x >= x0 && point.x <= x1 && point.y >= y0 && point.y <= y1
        };

        if inside(right - RESIZE_CORNER, bottom - RESIZE_CORNER, right, bottom) {
            Some(Self::Corner)
        } else if inside(
            rect.x,
            bottom - RESIZE_BORDER,
            right - RESIZE_BORDER,
            bottom,
        ) {
            Some(Self::Bottom)
        } else if inside(
            right - RESIZE_BORDER,
            rect.y + BAR_HEIGHT,
            right,
            bottom - RESIZE_BORDER,
        ) {
            Some(Self::Right)
        } else {
            None
        }
    }

    /// The part of a pointer offset that resizes through this grip.
    fn delta(self, offset: Vector) -> Vector {
        match self {
            Self::Right => Vector::new(offset.x, 0.0),
            Self::Bottom => Vector::new(0.0, offset.y),
            Self::Corner => offset,
        }
    }

    /// The cursor over this grip.
    fn interaction(self) -> mouse::Interaction {
        match self {
            Self::Right => mouse::Interaction::ResizingHorizontally,
            Self::Bottom => mouse::Interaction::ResizingVertically,
            Self::Corner => mouse::Interaction::ResizingDiagonallyDown,
        }
    }
}

/// A pointer gesture in progress.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Gesture {
    /// Moving by the title bar.
    Move { origin: Point, start: PanelState },
    /// Resizing by a grip.
    Resize {
        origin: Point,
        start: PanelState,
        grip: Grip,
    },
}

/// The tree state of a [`Panel`].
#[derive(Debug, Clone, Copy, Default)]
struct State {
    focused: bool,
    gesture: Option<Gesture>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// The widget of a [`FloatingPanel`], with its title bar built.
struct Panel<'a, Message> {
    state: PanelState,
    bar: IcedElement<'a, Action, Theme, iced::Renderer>,
    content: Element<'a, Message>,
    on_change: Option<Box<dyn Fn(PanelState) -> Message + 'a>>,
    on_close: Option<Message>,
    min_size: Size,
    resizable: bool,
    keyboard_step: f32,
    id: Option<Id>,
}

impl<Message: Clone> Panel<'_, Message> {
    /// The panel rectangle inside the area `bounds`, in window coordinates.
    fn rect(&self, bounds: Rectangle) -> Rectangle {
        self.state.placed(bounds.size(), self.min_size) + Vector::new(bounds.x, bounds.y)
    }

    /// Whether the edges resize the panel now.
    fn can_resize(&self) -> bool {
        self.on_change.is_some() && self.resizable && !self.state.minimized && !self.state.maximized
    }

    /// Publishes `next` when it differs from the current state.
    fn change(&self, next: PanelState, shell: &mut Shell<'_, Message>) {
        if let Some(on_change) = &self.on_change
            && next != self.state
        {
            shell.publish(on_change(next));
        }
    }

    /// Applies a title bar action.
    fn act(
        &self,
        action: Action,
        state: &mut State,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) {
        match action {
            Action::Drag if self.on_change.is_some() => {
                if let Some(origin) = cursor.position() {
                    state.gesture = Some(Gesture::Move {
                        origin,
                        start: self.state,
                    });
                }
            }
            Action::Minimize => self.change(self.state.toggle_minimized(), shell),
            Action::ToggleMaximize => self.change(self.state.toggle_maximized(), shell),
            Action::Close => {
                if let Some(message) = &self.on_close {
                    shell.publish(message.clone());
                }
            }
            _ => {}
        }
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Panel<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.bar), Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        if tree.children.len() == 2 {
            tree.children[0].diff(&self.bar);
            tree.children[1].diff(&self.content);
        } else {
            tree.children = self.children();
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        let rect = self.state.placed(size, self.min_size);
        // The content keeps the size of the restored panel while the panel
        // is minimized.
        let body = PanelState {
            minimized: false,
            ..self.state
        }
        .placed(size, self.min_size);

        let bar = self
            .bar
            .as_widget_mut()
            .layout(
                &mut tree.children[0],
                renderer,
                &layout::Limits::new(Size::ZERO, Size::new(rect.width, BAR_HEIGHT)),
            )
            .move_to(Point::new(rect.x, rect.y));
        let content = self
            .content
            .as_widget_mut()
            .layout(
                &mut tree.children[1],
                renderer,
                &layout::Limits::new(
                    Size::ZERO,
                    Size::new(body.width, (body.height - BAR_HEIGHT).max(0.0)),
                ),
            )
            .move_to(Point::new(body.x, body.y + BAR_HEIGHT));

        layout::Node::with_children(size, vec![bar, content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut children = layout.children();
        let (Some(bar_layout), Some(content_layout)) = (children.next(), children.next()) else {
            return;
        };
        let state = tree.state.downcast_mut::<State>();

        if self.on_change.is_some() {
            operation.focusable(self.id.as_ref(), bar_layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), bar_layout.bounds());
        }

        let minimized = self.state.minimized;

        operation.traverse(&mut |operation| {
            self.bar.as_widget_mut().operate(
                &mut tree.children[0],
                bar_layout,
                renderer,
                operation,
            );

            if !minimized {
                self.content.as_widget_mut().operate(
                    &mut tree.children[1],
                    content_layout,
                    renderer,
                    operation,
                );
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let (Some(bar_layout), Some(content_layout)) = (children.next(), children.next()) else {
            return;
        };
        let bounds = layout.bounds();
        let rect = self.rect(bounds);

        {
            let state = tree.state.downcast_mut::<State>();

            if self.on_change.is_none() {
                state.focused = false;
                state.gesture = None;
            }

            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    let over = cursor.position_over(rect);

                    if over.is_none_or(|point| !bar_layout.bounds().contains(point))
                        && state.focused
                    {
                        state.focused = false;
                        shell.request_redraw();
                    }

                    if let Some(point) = over
                        && self.can_resize()
                        && !shell.is_event_captured()
                        && let Some(grip) = Grip::at(rect, point)
                    {
                        state.gesture = Some(Gesture::Resize {
                            origin: point,
                            start: self.state,
                            grip,
                        });
                        shell.capture_event();
                        return;
                    }
                }
                Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Touch(touch::Event::FingerMoved { .. }) => {
                    if let (Some(gesture), Some(position)) = (state.gesture, cursor.position()) {
                        let next = match gesture {
                            Gesture::Move { origin, start } => {
                                start.moved(position - origin, bounds.size(), self.min_size)
                            }
                            Gesture::Resize {
                                origin,
                                start,
                                grip,
                            } => start.resized(
                                grip.delta(position - origin),
                                bounds.size(),
                                self.min_size,
                            ),
                        };

                        self.change(next, shell);
                        shell.capture_event();
                        return;
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                | Event::Mouse(mouse::Event::CursorLeft)
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. }) => {
                    if let Some(gesture) = state.gesture.take()
                        && matches!(gesture, Gesture::Resize { .. })
                    {
                        shell.capture_event();
                        return;
                    }
                }
                _ => {}
            }
        }

        let mut actions = Vec::new();
        let mut local = Shell::new(&mut actions);

        self.bar.as_widget_mut().update(
            &mut tree.children[0],
            event,
            bar_layout,
            cursor,
            renderer,
            clipboard,
            &mut local,
            viewport,
        );

        if local.is_event_captured() {
            shell.capture_event();
        }
        shell.request_redraw_at(local.redraw_request());
        if local.is_layout_invalid() {
            shell.invalidate_layout();
        }
        if local.are_widgets_invalid() {
            shell.invalidate_widgets();
        }

        {
            let state = tree.state.downcast_mut::<State>();

            for action in actions {
                self.act(action, state, cursor, shell);
            }
        }

        if !self.state.minimized {
            self.content.as_widget_mut().update(
                &mut tree.children[1],
                event,
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Mouse(mouse::Event::WheelScrolled { .. })
            | Event::Touch(touch::Event::FingerPressed { .. })
                if cursor.is_over(rect) =>
            {
                shell.capture_event();
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused && !shell.is_event_captured() =>
            {
                let step = self.keyboard_step;
                let delta = match key {
                    keyboard::Key::Named(key::Named::ArrowLeft) => Vector::new(-step, 0.0),
                    keyboard::Key::Named(key::Named::ArrowRight) => Vector::new(step, 0.0),
                    keyboard::Key::Named(key::Named::ArrowUp) => Vector::new(0.0, -step),
                    keyboard::Key::Named(key::Named::ArrowDown) => Vector::new(0.0, step),
                    _ => return,
                };
                let next = if modifiers.shift() {
                    if self.resizable {
                        self.state.resized(delta, bounds.size(), self.min_size)
                    } else {
                        self.state
                    }
                } else {
                    self.state.moved(delta, bounds.size(), self.min_size)
                };

                self.change(next, shell);
                shell.capture_event();
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let mut children = layout.children();
        let (Some(bar_layout), Some(content_layout)) = (children.next(), children.next()) else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let rect = self.rect(layout.bounds());
        let corner = if self.state.maximized {
            0.0
        } else {
            radius::WINDOW
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: rect,
                border: Border::default().rounded(corner),
                shadow: elevation(roles.shadow, ELEVATION),
                ..renderer::Quad::default()
            },
            roles.chrome.dialog,
        );

        let style = renderer::Style {
            text_color: roles.on_surface,
        };

        renderer.with_layer(rect, |renderer| {
            self.bar.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                &style,
                bar_layout,
                cursor,
                viewport,
            );

            if !self.state.minimized {
                self.content.as_widget().draw(
                    &tree.children[1],
                    renderer,
                    theme,
                    &style,
                    content_layout,
                    cursor,
                    viewport,
                );
            }
        });

        renderer.fill_quad(
            renderer::Quad {
                bounds: rect,
                border: Border::default()
                    .color(roles.outline_variant)
                    .width(BORDER_WIDTH)
                    .rounded(corner),
                ..renderer::Quad::default()
            },
            Color::TRANSPARENT,
        );

        if state.focused && self.on_change.is_some() {
            let width = sizes::FOCUS_BORDER_WIDTH;

            renderer.fill_quad(
                renderer::Quad {
                    bounds: bar_layout.bounds().expand(width),
                    border: Border {
                        color: theme.focus_ring(),
                        width,
                        radius: border::top(corner + width),
                    },
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let (Some(bar_layout), Some(content_layout)) = (children.next(), children.next()) else {
            return mouse::Interaction::None;
        };
        let state = tree.state.downcast_ref::<State>();

        if let Some(Gesture::Resize { grip, .. }) = state.gesture {
            return grip.interaction();
        }

        let rect = self.rect(layout.bounds());
        let Some(point) = cursor.position_over(rect) else {
            return mouse::Interaction::None;
        };

        if self.can_resize()
            && let Some(grip) = Grip::at(rect, point)
        {
            return grip.interaction();
        }

        let bar = self.bar.as_widget().mouse_interaction(
            &tree.children[0],
            bar_layout,
            cursor,
            viewport,
            renderer,
        );
        let inner = if bar != mouse::Interaction::None || self.state.minimized {
            bar
        } else {
            self.content.as_widget().mouse_interaction(
                &tree.children[1],
                content_layout,
                cursor,
                viewport,
                renderer,
            )
        };

        match inner {
            mouse::Interaction::None => mouse::Interaction::Idle,
            other => other,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        if self.state.minimized {
            return None;
        }

        let content_layout = layout.children().nth(1)?;

        self.content.as_widget_mut().overlay(
            &mut tree.children[1],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> From<FloatingPanel<'a, Message>> for Element<'a, Message> {
    fn from(panel: FloatingPanel<'a, Message>) -> Self {
        let FloatingPanel {
            state,
            title,
            content,
            on_change,
            on_close,
            min_size,
            resizable,
            control_style,
            labels,
            keyboard_step,
            id,
        } = panel;

        let mut trailing = vec![WindowButton::Minimize, WindowButton::Maximize];
        if on_close.is_some() {
            trailing.push(WindowButton::Close);
        }

        let bar = window::header_bar(|action| action)
            .title(title)
            .control_style(control_style)
            .button_layout(WindowButtonLayout {
                leading: Vec::new(),
                trailing,
            })
            .state(WindowState {
                maximized: state.maximized,
                ..WindowState::default()
            })
            .labels(labels)
            .radius(if state.maximized {
                border::Radius::default()
            } else if state.minimized {
                border::radius(radius::WINDOW)
            } else {
                border::top(radius::WINDOW)
            })
            .into();

        IcedElement::new(Panel {
            state,
            bar,
            content,
            on_change,
            on_close,
            min_size,
            resizable,
            keyboard_step,
            id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOUNDS: Size = Size::new(1000.0, 800.0);

    fn panel() -> PanelState {
        PanelState::new(Rectangle::new(
            Point::new(50.0, 50.0),
            Size::new(300.0, 200.0),
        ))
    }

    #[test]
    fn defaults_match_the_design() {
        let state = PanelState::default();

        assert_eq!(
            state.rect,
            Rectangle::new(Point::new(40.0, 40.0), Size::new(360.0, 280.0))
        );
        assert!(!state.minimized && !state.maximized);
        assert_eq!(MIN_SIZE, Size::new(220.0, 140.0));
        assert_eq!(KEYBOARD_STEP, 16.0);
    }

    #[test]
    fn clamping_keeps_the_title_bar_inside_and_the_minimum_size() {
        let area = Size::new(400.0, 300.0);
        let rect = PanelState::clamp(
            Rectangle::new(Point::new(-20.0, 500.0), Size::new(100.0, 50.0)),
            area,
            MIN_SIZE,
        );

        assert_eq!(
            rect,
            Rectangle::new(Point::new(0.0, 300.0 - BAR_HEIGHT), MIN_SIZE)
        );

        let wide = PanelState::clamp(
            Rectangle::new(Point::new(300.0, 0.0), Size::new(900.0, 200.0)),
            area,
            MIN_SIZE,
        );

        assert_eq!(wide, Rectangle::new(Point::ORIGIN, Size::new(400.0, 200.0)));
    }

    #[test]
    fn moving_and_resizing_step_and_clamp() {
        let moved = panel().moved(Vector::new(16.0, 0.0), BOUNDS, MIN_SIZE);

        assert_eq!(moved.rect.x, 66.0);

        let grown = panel().resized(Vector::new(0.0, 16.0), BOUNDS, MIN_SIZE);

        assert_eq!(grown.rect.height, 216.0);

        let shrunk = panel().resized(Vector::new(-500.0, -500.0), BOUNDS, MIN_SIZE);

        assert_eq!(shrunk.rect.size(), MIN_SIZE);
    }

    #[test]
    fn a_maximized_panel_neither_moves_nor_resizes() {
        let maximized = panel().toggle_maximized();

        assert_eq!(
            maximized.moved(Vector::new(10.0, 10.0), BOUNDS, MIN_SIZE),
            maximized
        );
        assert_eq!(
            maximized.resized(Vector::new(10.0, 10.0), BOUNDS, MIN_SIZE),
            maximized
        );
        assert_eq!(
            maximized.placed(BOUNDS, MIN_SIZE),
            Rectangle::with_size(BOUNDS)
        );
    }

    #[test]
    fn minimizing_and_maximizing_exclude_each_other() {
        let minimized = panel().toggle_maximized().toggle_minimized();

        assert!(minimized.minimized && !minimized.maximized);
        assert_eq!(minimized.placed(BOUNDS, MIN_SIZE).height, BAR_HEIGHT + 2.0);
        assert_eq!(minimized.placed(BOUNDS, MIN_SIZE).width, 300.0);

        let maximized = minimized.toggle_maximized();

        assert!(maximized.maximized && !maximized.minimized);
        assert_eq!(maximized.toggle_maximized().rect, panel().rect);
    }

    #[test]
    fn grips_cover_the_right_and_bottom_edges_and_the_corner() {
        let rect = panel().rect;

        assert_eq!(Grip::at(rect, Point::new(347.0, 247.0)), Some(Grip::Corner));
        assert_eq!(Grip::at(rect, Point::new(200.0, 248.0)), Some(Grip::Bottom));
        assert_eq!(Grip::at(rect, Point::new(348.0, 150.0)), Some(Grip::Right));
        assert_eq!(Grip::at(rect, Point::new(348.0, 60.0)), None);
        assert_eq!(Grip::at(rect, Point::new(200.0, 150.0)), None);
    }
}
