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

//! Window control buttons drawn in the visual family of each desktop.

use iced::advanced::graphics::geometry::{self, Frame, LineCap, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::widget::{Row, Space};
use iced::{
    Alignment, Border, Color, Element, Event, Length, Point, Rectangle, Size, mouse, touch, window,
};

use super::{Action, WindowState};
use crate::Theme;
use crate::foundation::{WindowButton, WindowControlStyle};
use crate::l10n::Localizations;
use crate::tokens::{radius, sizes, spacing};

/// The action and glyph of a window control button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlType {
    /// Minimizes the window.
    Minimize,
    /// Maximizes the window.
    Maximize,
    /// Restores a maximized window.
    Restore,
    /// Closes the window.
    Close,
}

impl ControlType {
    /// The control of `button` for a window in `state`: the maximize
    /// button is a restore button while the window is maximized.
    pub fn of(button: WindowButton, state: WindowState) -> Self {
        match button {
            WindowButton::Minimize => Self::Minimize,
            WindowButton::Maximize if state.maximized => Self::Restore,
            WindowButton::Maximize => Self::Maximize,
            WindowButton::Close => Self::Close,
        }
    }

    /// The window action the control requests.
    pub fn action(self) -> Action {
        match self {
            Self::Minimize => Action::Minimize,
            Self::Maximize | Self::Restore => Action::ToggleMaximize,
            Self::Close => Action::Close,
        }
    }
}

/// The labels of the window control buttons, reported as their text to
/// widget operations (the simulator selectors among them).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Labels {
    /// Label of [`ControlType::Minimize`].
    pub minimize: String,
    /// Label of [`ControlType::Maximize`].
    pub maximize: String,
    /// Label of [`ControlType::Restore`].
    pub restore: String,
    /// Label of [`ControlType::Close`].
    pub close: String,
}

impl Labels {
    /// The labels of `localizations`.
    pub fn new(localizations: &Localizations) -> Self {
        Self {
            minimize: localizations.minimize_window.to_owned(),
            maximize: localizations.maximize_window.to_owned(),
            restore: localizations.restore_window.to_owned(),
            close: localizations.close_window.to_owned(),
        }
    }

    /// The label of `control`.
    pub fn get(&self, control: ControlType) -> &str {
        match control {
            ControlType::Minimize => &self.minimize,
            ControlType::Maximize => &self.maximize,
            ControlType::Restore => &self.restore,
            ControlType::Close => &self.close,
        }
    }
}

impl Default for Labels {
    /// The English labels.
    fn default() -> Self {
        Self::new(&Localizations::en())
    }
}

impl From<Localizations> for Labels {
    fn from(localizations: Localizations) -> Self {
        Self::new(&localizations)
    }
}

/// Close button background of the Windows style while hovered or pressed.
const WINDOWS_CLOSE_RED: Color = Color::from_rgb8(0xC4, 0x2B, 0x1C);
/// Close button background of the Breeze style while hovered or pressed.
const BREEZE_CLOSE_RED: Color = Color::from_rgb8(0xDA, 0x44, 0x53);
/// macOS close button.
const MACOS_RED: Color = Color::from_rgb8(0xFF, 0x5F, 0x57);
/// macOS minimize button.
const MACOS_YELLOW: Color = Color::from_rgb8(0xFE, 0xBC, 0x2E);
/// macOS maximize and restore button.
const MACOS_GREEN: Color = Color::from_rgb8(0x28, 0xC8, 0x40);
/// macOS buttons of an unfocused window.
const MACOS_INACTIVE: Color = Color::from_rgb8(0xC8, 0xC8, 0xC8);
/// macOS glyphs, shown while the button is hovered.
const MACOS_GLYPH: Color = Color::from_rgba8(0, 0, 0, 0xA6 as f32 / 255.0);
/// Side of the macOS buttons.
const MACOS_BUTTON_SIZE: f32 = 12.0;

/// The outline of a control background.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Shape {
    Circle,
    Rounded,
    Rectangle,
}

/// The interaction state a control is drawn in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Hovered,
    Pressed,
}

/// The tree state of a [`WindowControl`].
#[derive(Debug, Default)]
struct State {
    pressed: bool,
    drawn: Option<Status>,
}

/// A window control button drawn in the visual family of a
/// [`WindowControlStyle`]: a circle (GNOME, Breeze, COSMIC), a rounded
/// square (Xfwm), a full-height rectangle (Windows) or a colored dot
/// (macOS), with the glyph of its [`ControlType`].
///
/// The button stays out of keyboard focus traversal, as native window
/// buttons do, and reports its label as text to widget operations.
/// [`WindowControlStyle::None`] draws nothing and takes no space.
pub struct WindowControl<Message> {
    control: ControlType,
    style: WindowControlStyle,
    label: String,
    on_press: Option<Message>,
    foreground: Option<Color>,
    height: f32,
    active: bool,
    opacity: f32,
}

/// A window control button with the action and glyph of `control`, drawn
/// in `style` and labeled `label`.
pub fn window_control<Message>(
    control: ControlType,
    style: WindowControlStyle,
    label: impl Into<String>,
) -> WindowControl<Message> {
    WindowControl {
        control,
        style,
        label: label.into(),
        on_press: None,
        foreground: None,
        height: sizes::TITLE_BAR_HEIGHT,
        active: true,
        opacity: 1.0,
    }
}

impl<Message> WindowControl<Message> {
    /// The width a control of `style` takes.
    pub fn width_for(style: WindowControlStyle) -> f32 {
        match style {
            WindowControlStyle::Windows => sizes::WINDOWS_WINDOW_CONTROL_WIDTH,
            WindowControlStyle::None => 0.0,
            _ => sizes::WINDOW_CONTROL_SIZE,
        }
    }

    /// Publishes `message` when the button is clicked; without it the
    /// button is disabled.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// The glyph color; defaults to the `on_surface` role of the theme.
    #[must_use]
    pub fn foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// The title bar height, which the Windows buttons fill; the other
    /// styles are [`sizes::WINDOW_CONTROL_SIZE`] tall. Defaults to
    /// [`sizes::TITLE_BAR_HEIGHT`].
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Whether the window is focused; the macOS buttons of an unfocused
    /// window turn grey until hovered. Defaults to `true`.
    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Multiplies the alpha of every color the button draws.
    #[must_use]
    pub(crate) fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    /// The size of the box the control takes.
    fn box_size(&self) -> Size {
        match self.style {
            WindowControlStyle::None => Size::ZERO,
            WindowControlStyle::Windows => {
                Size::new(sizes::WINDOWS_WINDOW_CONTROL_WIDTH, self.height)
            }
            _ => Size::new(sizes::WINDOW_CONTROL_SIZE, sizes::WINDOW_CONTROL_SIZE),
        }
    }

    /// The size of the background drawn centered in the box.
    fn button_size(&self) -> Size {
        match self.style {
            WindowControlStyle::Windows => self.box_size(),
            WindowControlStyle::Macos => Size::new(MACOS_BUTTON_SIZE, MACOS_BUTTON_SIZE),
            _ => Size::new(sizes::WINDOW_CONTROL_SIZE, sizes::WINDOW_CONTROL_SIZE),
        }
    }

    /// The side of the square the glyph is drawn in, and its stroke width.
    fn glyph_metrics(&self) -> (f32, f32) {
        match self.style {
            WindowControlStyle::Windows => (10.0, 1.0),
            WindowControlStyle::Macos => (6.0, 1.2),
            _ => (8.0, 1.4),
        }
    }

    /// The background, glyph color and shape for `status`, tinting
    /// `foreground` for hover and press.
    fn appearance(&self, foreground: Color, status: Status) -> (Color, Color, Shape) {
        let hovered = status != Status::Idle;
        let pressed = status == Status::Pressed;
        let is_close = self.control == ControlType::Close;
        let tint = |idle: f32, hover: f32, press: f32| Color {
            a: match status {
                Status::Idle => idle,
                Status::Hovered => hover,
                Status::Pressed => press,
            },
            ..foreground
        };

        match self.style {
            WindowControlStyle::Gnome => (tint(0.10, 0.15, 0.20), foreground, Shape::Circle),
            WindowControlStyle::Cosmic => (tint(0.0, 0.10, 0.16), foreground, Shape::Circle),
            WindowControlStyle::Breeze if is_close && hovered => (
                BREEZE_CLOSE_RED.scale_alpha(if pressed { 0.8 } else { 1.0 }),
                Color::WHITE,
                Shape::Circle,
            ),
            WindowControlStyle::Breeze => (tint(0.0, 0.12, 0.18), foreground, Shape::Circle),
            WindowControlStyle::Xfwm => (tint(0.0, 0.10, 0.16), foreground, Shape::Rounded),
            WindowControlStyle::Windows if is_close && hovered => (
                WINDOWS_CLOSE_RED.scale_alpha(if pressed { 0.85 } else { 1.0 }),
                Color::WHITE,
                Shape::Rectangle,
            ),
            WindowControlStyle::Windows => (tint(0.0, 0.08, 0.12), foreground, Shape::Rectangle),
            WindowControlStyle::Macos => {
                let color = if !self.active && !hovered {
                    MACOS_INACTIVE
                } else {
                    match self.control {
                        ControlType::Close => MACOS_RED,
                        ControlType::Minimize => MACOS_YELLOW,
                        ControlType::Maximize | ControlType::Restore => MACOS_GREEN,
                    }
                };
                let glyph = if hovered {
                    MACOS_GLYPH
                } else {
                    Color::TRANSPARENT
                };
                (color, glyph, Shape::Circle)
            }
            WindowControlStyle::None => (Color::TRANSPARENT, foreground, Shape::Circle),
        }
    }

    /// The interaction state for `cursor` over `bounds`.
    fn status(&self, state: &State, bounds: Rectangle, cursor: mouse::Cursor) -> Status {
        if self.on_press.is_none() || !cursor.is_over(bounds) {
            Status::Idle
        } else if state.pressed {
            Status::Pressed
        } else {
            Status::Hovered
        }
    }

    /// The glyph paths, centered at `center`.
    fn glyph(&self, center: Point, side: f32) -> Vec<Path> {
        let half = side / 2.0;
        let (left, top, right, bottom) = (
            center.x - half,
            center.y - half,
            center.x + half,
            center.y + half,
        );
        let horizontal = || Path::line(Point::new(left, center.y), Point::new(right, center.y));
        let plus = self.style == WindowControlStyle::Macos;

        match self.control {
            ControlType::Minimize => vec![horizontal()],
            ControlType::Maximize | ControlType::Restore if plus => vec![
                horizontal(),
                Path::line(Point::new(center.x, top), Point::new(center.x, bottom)),
            ],
            ControlType::Maximize => vec![Path::rectangle(
                Point::new(left, top),
                Size::new(side, side),
            )],
            ControlType::Restore => {
                let inset = side * 0.25;
                let front_top = top + inset;
                let front_right = right - inset;
                vec![
                    Path::rectangle(
                        Point::new(left, front_top),
                        Size::new(front_right - left, bottom - front_top),
                    ),
                    Path::new(|builder| {
                        builder.move_to(Point::new(left + inset, front_top));
                        builder.line_to(Point::new(left + inset, top));
                        builder.line_to(Point::new(right, top));
                        builder.line_to(Point::new(right, bottom - inset));
                        builder.line_to(Point::new(front_right, bottom - inset));
                    }),
                ]
            }
            ControlType::Close => vec![
                Path::line(Point::new(left, top), Point::new(right, bottom)),
                Path::line(Point::new(right, top), Point::new(left, bottom)),
            ],
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for WindowControl<Message>
where
    Message: Clone,
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        let size = self.box_size();
        Size::new(Length::Fixed(size.width), Length::Fixed(size.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = self.box_size();
        layout::atomic(limits, size.width, size.height)
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if self.style != WindowControlStyle::None {
            operation.text(None, layout.bounds(), &self.label);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if self.style == WindowControlStyle::None {
            return;
        }

        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(bounds) {
                    state.pressed = self.on_press.is_some();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if cursor.is_over(bounds) {
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if state.pressed {
                    state.pressed = false;

                    if cursor.is_over(bounds)
                        && let Some(message) = &self.on_press
                    {
                        shell.publish(message.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = false;
            }
            _ => {}
        }

        let status = self.status(state, bounds, cursor);

        if let Event::Window(window::Event::RedrawRequested(_)) = event {
            state.drawn = Some(status);
        } else if state.drawn.is_some_and(|drawn| drawn != status) {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if self.on_press.is_some()
            && self.style != WindowControlStyle::None
            && cursor.is_over(layout.bounds())
        {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        if self.style == WindowControlStyle::None {
            return;
        }

        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();
        let foreground = self.foreground.unwrap_or(theme.roles().on_surface);
        let (background, glyph, shape) =
            self.appearance(foreground, self.status(state, bounds, cursor));
        let (background, glyph) = (
            background.scale_alpha(self.opacity),
            glyph.scale_alpha(self.opacity),
        );

        let button = self.button_size();
        let center = bounds.center();
        let button_bounds = Rectangle::new(
            Point::new(
                center.x - button.width / 2.0,
                center.y - button.height / 2.0,
            ),
            button,
        );

        if background.a > 0.0 {
            let radius = match shape {
                Shape::Circle => button.width.min(button.height) / 2.0,
                Shape::Rounded => radius::CHECK,
                Shape::Rectangle => 0.0,
            };

            renderer.fill_quad(
                Quad {
                    bounds: button_bounds,
                    border: Border::default().rounded(radius),
                    ..Quad::default()
                },
                background,
            );
        }

        if glyph.a > 0.0 {
            let (side, width) = self.glyph_metrics();
            let mut frame = Frame::new(renderer, bounds.size());
            let stroke = Stroke::default()
                .with_color(glyph)
                .with_width(width)
                .with_line_cap(LineCap::Square);

            for path in self.glyph(frame.center(), side) {
                frame.stroke(&path, stroke);
            }

            let geometry = frame.into_geometry();
            renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
                renderer.draw_geometry(geometry);
            });
        }
    }
}

impl<'a, Message, Renderer> From<WindowControl<Message>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + 'a,
{
    fn from(control: WindowControl<Message>) -> Self {
        Element::new(control)
    }
}

/// The window control buttons of one side of a [`WindowButtonLayout`],
/// in order, drawn in `style` for a window in `state`: the maximize button
/// is a restore button while the window is maximized, and macOS buttons
/// turn grey while it is not focused. Each button publishes the
/// [`Action`] of its [`ControlType`] through `on_action`.
///
/// The Windows buttons are [`sizes::TITLE_BAR_HEIGHT`] tall; inside a
/// [`title_bar`](super::title_bar) they fill the bar instead.
///
/// [`WindowButtonLayout`]: crate::foundation::WindowButtonLayout
pub fn controls<'a, Message>(
    style: WindowControlStyle,
    buttons: &[WindowButton],
    state: WindowState,
    labels: &Labels,
    on_action: impl Fn(Action) -> Message,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    row(
        style,
        buttons,
        state,
        labels,
        &on_action,
        sizes::TITLE_BAR_HEIGHT,
        1.0,
    )
}

/// The control buttons of `buttons` in a row, with the gap of `style`,
/// `height` tall for the Windows style and drawn at `opacity`.
pub(super) fn row<'a, Message>(
    style: WindowControlStyle,
    buttons: &[WindowButton],
    state: WindowState,
    labels: &Labels,
    on_action: &dyn Fn(Action) -> Message,
    height: f32,
    opacity: f32,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    if style == WindowControlStyle::None || buttons.is_empty() {
        return Space::new().into();
    }

    let gap = match style {
        WindowControlStyle::Windows | WindowControlStyle::Macos => 0.0,
        _ => spacing::SM,
    };

    Row::with_children(buttons.iter().map(|&button| {
        let control = ControlType::of(button, state);

        window_control(control, style, labels.get(control))
            .on_press(on_action(control.action()))
            .height(height)
            .active(state.focused)
            .opacity(opacity)
            .into()
    }))
    .spacing(gap)
    .align_y(Alignment::Center)
    .into()
}
