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

//! Check boxes, radios and switches drawn in the OITO2 style.
//!
//! Each control is a square target (34 pixels for a pointer, 48 for touch,
//! following the [density]) with the painted control
//! centered in it. A control takes part in focus traversal while it has a
//! change callback, shows the focus ring around the painted shape while
//! focused, and changes on a click and on Enter or Space. A change animates
//! the control for [`TOGGLE_DURATION`]; a control that did not change since
//! it was created is drawn in its final state.
//!
//! The value lives in the application: each control takes the current
//! value and a closure that turns the next value into a message.
//!
//! ```no_run
//! use oito2_iced::widget::toggles;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Hidden(Option<bool>),
//!     Size(Option<u8>),
//!     Wifi(bool),
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! iced::widget::row![
//!     toggles::checkbox(true).on_change(Message::Hidden),
//!     toggles::radio(1, Some(2)).on_change(Message::Size),
//!     toggles::switch(false).on_toggle(Message::Wifi),
//! ]
//! .into()
//! # }
//! ```

use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::{Border, Color, Event, Length, Point, Rectangle, Size, Vector, mouse, touch, window};

use crate::animation::{Animation, EASE_IN_OUT_CUBIC};
use crate::density::{self, Density};
use crate::theme::{Brightness, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, state_layer};
use crate::tokens::{radius, sizes, spacing};
use crate::widget::helpers::{ReadingDirection, activate_radio, next_check_value};
use crate::{Element, Theme};

/// Duration of the selection animation of every control.
pub const TOGGLE_DURATION: Duration = Duration::from_millis(150);

/// Side of the check box and of the radio ring for a pointer.
pub const BOX_SIZE: f32 = 20.0;

/// Side of the check box and of the radio ring on a touch screen.
pub const TOUCH_BOX_SIZE: f32 = 24.0;

/// Width and height of the switch track for a pointer.
pub const SWITCH_TRACK: (f32, f32) = (42.0, 24.0);

/// Width and height of the switch track on a touch screen.
pub const TOUCH_SWITCH_TRACK: (f32, f32) = (52.0, 30.0);

/// Opacity of the tone blended over a control while it is pressed.
const PRESSED_OVERLAY: f32 = 0.16;

/// Width of the outline of unselected controls.
const OUTLINE_WIDTH: f32 = 1.5;

/// Space between a control and its focus ring.
const RING_GAP: f32 = 2.0;

/// The painted control and its options.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Kind {
    /// A check box; `mixed` draws a dash instead of the check mark.
    Checkbox { mixed: bool },
    /// A radio ring.
    Radio,
    /// A switch; the thumb travels toward the end of `direction`.
    Switch {
        on_off_shapes: bool,
        direction: ReadingDirection,
    },
}

/// A painted check box, radio or switch.
///
/// An interactive toggle takes the focus and publishes its activation
/// message; a passive one only draws itself, tracking hover and press for
/// its look, and leaves the input to the widget around it.
pub(crate) struct Toggle<Message> {
    pub(crate) kind: Kind,
    pub(crate) selected: bool,
    pub(crate) enabled: bool,
    pub(crate) on_activate: Option<Message>,
    pub(crate) id: Option<Id>,
    pub(crate) interactive: bool,
}

impl<Message> Toggle<Message> {
    /// The same toggle without focus or input handling of its own, for a
    /// row or tile that handles the input for it.
    pub(crate) fn passive(self) -> Self {
        Self {
            interactive: false,
            on_activate: None,
            id: None,
            ..self
        }
    }

    /// Whether the control sits before its label by default: check boxes
    /// and radios do, switches follow the label.
    pub(crate) fn leads(&self) -> bool {
        !matches!(self.kind, Kind::Switch { .. })
    }
}

/// The look of a toggle in its current interaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Visual {
    hovered: bool,
    pressed: bool,
    focused: bool,
}

/// The tree state of a [`Toggle`].
#[derive(Debug)]
struct State {
    focused: bool,
    pressed: bool,
    touch: bool,
    animation: Animation<bool>,
    now: Option<Instant>,
    drawn: Option<Visual>,
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

/// The colors a toggle draws with.
#[derive(Debug, Clone, Copy)]
struct Colors {
    fill: Color,
    on_fill: Color,
    outline: Color,
    surface: Color,
    track: Color,
    on_surface: Color,
    off_thumb: Color,
}

impl Colors {
    /// The colors of `theme`: the selected fill and its content color, the
    /// outline, the unselected box and track fills, the tone of the hover
    /// and press overlays and the thumb of an off switch.
    fn of(theme: &Theme) -> Self {
        let roles = theme.roles();
        let fill = roles.selected_fill();
        let on_fill = if fill == roles.primary {
            roles.on_primary
        } else {
            roles.on_indicator()
        };

        Self {
            fill,
            on_fill,
            outline: roles.outline,
            surface: roles.surface_container_lowest,
            track: roles.surface_container_highest,
            on_surface: roles.on_surface,
            off_thumb: match roles.brightness {
                Brightness::Dark => roles.on_surface_variant,
                Brightness::Light => roles.surface_container_lowest,
            },
        }
    }
}

/// The color `t` of the way from `from` to `to`.
fn mix(from: Color, to: Color, t: f32) -> Color {
    Color {
        r: from.r + (to.r - from.r) * t,
        g: from.g + (to.g - from.g) * t,
        b: from.b + (to.b - from.b) * t,
        a: from.a + (to.a - from.a) * t,
    }
}

/// The side of the square target of a check box or radio, which is also
/// the height of a switch.
fn target(touch: bool) -> f32 {
    if touch {
        sizes::MIN_TOUCH_TARGET
    } else {
        sizes::BUTTON_HEIGHT
    }
}

/// The side of the check box or radio ring.
fn box_size(touch: bool) -> f32 {
    if touch { TOUCH_BOX_SIZE } else { BOX_SIZE }
}

/// The switch track size.
fn track_size(touch: bool) -> Size {
    let (width, height) = if touch {
        TOUCH_SWITCH_TRACK
    } else {
        SWITCH_TRACK
    };

    Size::new(width, height)
}

/// A `size` rectangle centered at `center`.
fn centered(center: Point, size: Size) -> Rectangle {
    Rectangle::new(
        Point::new(center.x - size.width / 2.0, center.y - size.height / 2.0),
        size,
    )
}

/// The check mark polyline inside a box of side `side` at the origin.
fn check_points(side: f32) -> [Point; 3] {
    [
        Point::new(side * 0.24, side * 0.52),
        Point::new(side * 0.42, side * 0.70),
        Point::new(side * 0.77, side * 0.32),
    ]
}

/// The leading `t` fraction of the polyline through `points`, measured
/// along its length.
fn partial_polyline(points: &[Point], t: f32) -> Vec<Point> {
    let lengths: Vec<f32> = points
        .windows(2)
        .map(|pair| pair[0].distance(pair[1]))
        .collect();
    let mut remaining = lengths.iter().sum::<f32>() * t.clamp(0.0, 1.0);
    let mut result = Vec::with_capacity(points.len());

    if let Some(first) = points.first() {
        result.push(*first);
    }

    for (pair, length) in points.windows(2).zip(lengths) {
        if remaining >= length {
            result.push(pair[1]);
            remaining -= length;
        } else {
            if length > 0.0 && remaining > 0.0 {
                let fraction = remaining / length;
                result.push(pair[0] + (pair[1] - pair[0]) * fraction);
            }
            break;
        }
    }

    result
}

/// Fills `bounds` with `color` and the given corner `radius` and border.
fn quad<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    radius: f32,
    color: Color,
    border: Option<(Color, f32)>,
) {
    let border = match border {
        Some((border_color, width)) => Border::default()
            .color(border_color)
            .width(width)
            .rounded(radius),
        None => Border::default().rounded(radius),
    };

    renderer.fill_quad(
        Quad {
            bounds,
            border,
            ..Quad::default()
        },
        color,
    );
}

impl<Message> Toggle<Message> {
    /// The interaction state for `cursor` over `bounds`.
    fn visual(&self, state: &State, bounds: Rectangle, cursor: mouse::Cursor) -> Visual {
        Visual {
            hovered: self.enabled && cursor.is_over(bounds),
            pressed: self.enabled && state.pressed,
            focused: self.enabled && self.interactive && state.focused,
        }
    }

    /// The selection progress, from 0 (off) to 1 (on), at the time of the
    /// last redraw.
    fn progress(&self, state: &State) -> f32 {
        match state.now {
            Some(now) => state.animation.interpolate(0.0, 1.0, now),
            None if self.selected => 1.0,
            None => 0.0,
        }
    }
}

/// Draws a check box of side `side` centered in `bounds`, with a dash
/// instead of the check mark when `mixed`.
#[allow(clippy::too_many_arguments)]
fn draw_checkbox<Renderer: geometry::Renderer>(
    renderer: &mut Renderer,
    theme: &Theme,
    bounds: Rectangle,
    colors: &Colors,
    (overlay, opacity): (f32, f32),
    t: f32,
    (mixed, side): (bool, f32),
    focused: bool,
) {
    let rect = centered(bounds.center(), Size::new(side, side));

    quad(
        renderer,
        rect,
        radius::CHECK,
        state_layer(colors.surface, colors.on_surface, overlay).scale_alpha(opacity),
        Some((colors.outline.scale_alpha(opacity), OUTLINE_WIDTH)),
    );

    if t > 0.0 {
        let fill = state_layer(colors.fill, colors.on_fill, overlay);
        quad(
            renderer,
            rect,
            radius::CHECK,
            fill.scale_alpha(t * opacity),
            None,
        );

        let mark = colors.on_fill.scale_alpha(opacity);

        if mixed {
            let bar = centered(rect.center(), Size::new(side * 0.5 * t, side * 0.1));
            quad(renderer, bar, side * 0.05, mark, None);
        } else {
            let origin = rect.position() - bounds.position();
            let points: Vec<Point> = check_points(side)
                .iter()
                .map(|point| *point + origin)
                .collect();
            let points = partial_polyline(&points, t);

            if points.len() > 1 {
                let mut frame = Frame::new(renderer, bounds.size());
                let path = Path::new(|builder| {
                    builder.move_to(points[0]);
                    for point in &points[1..] {
                        builder.line_to(*point);
                    }
                });

                frame.stroke(
                    &path,
                    Stroke::default()
                        .with_color(mark)
                        .with_width(side * 0.11)
                        .with_line_cap(LineCap::Round)
                        .with_line_join(LineJoin::Round),
                );

                let geometry = frame.into_geometry();
                renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
                    renderer.draw_geometry(geometry)
                });
            }
        }
    }

    if focused {
        crate::focus::draw_ring(
            renderer,
            theme,
            rect.expand(RING_GAP),
            radius::CHECK + RING_GAP,
        );
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Toggle<Message>
where
    Message: Clone,
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            focused: false,
            pressed: false,
            touch: false,
            animation: Animation::new(self.selected)
                .duration(TOGGLE_DURATION)
                .easing(EASE_IN_OUT_CUBIC),
            now: None,
            drawn: None,
        })
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let touch = density::get() == Density::Touch;
        tree.state.downcast_mut::<State>().touch = touch;

        let height = target(touch);
        let width = match self.kind {
            Kind::Switch { .. } => track_size(touch).width + spacing::SM,
            Kind::Checkbox { .. } | Kind::Radio => height,
        };

        layout::atomic(limits, width, height)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if self.interactive && self.enabled {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), layout.bounds());
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
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();
        let redraw_at = match event {
            Event::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };

        if state.animation.value() != self.selected {
            let at = redraw_at.unwrap_or_else(Instant::now);
            state.animation.go_mut(self.selected, at);
            state.now = Some(at);
            shell.request_redraw();
        }

        if let Some(now) = redraw_at {
            state.now = Some(now);

            if state.animation.is_animating(now) {
                shell.request_redraw();
            }
        }

        let over = cursor.is_over(bounds);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if over =>
            {
                if self.enabled {
                    state.pressed = true;

                    if self.interactive {
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(_))
            | Event::Touch(touch::Event::FingerPressed { .. })
                if !over && state.focused =>
            {
                state.focused = false;
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
                if state.pressed =>
            {
                state.pressed = false;

                if self.interactive {
                    if over && let Some(message) = &self.on_activate {
                        shell.publish(message.clone());
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = false;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused
                    && self.interactive
                    && self.enabled
                    && !shell.is_event_captured()
                    && modifiers.is_empty()
                    && matches!(
                        key,
                        keyboard::Key::Named(key::Named::Enter | key::Named::Space)
                    ) =>
            {
                if let Some(message) = &self.on_activate {
                    shell.publish(message.clone());
                }

                shell.capture_event();
            }
            _ => {}
        }

        let visual = self.visual(state, bounds, cursor);

        if redraw_at.is_some() {
            state.drawn = Some(visual);
        } else if state.drawn.is_some_and(|drawn| drawn != visual) {
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
        if self.interactive && self.enabled && cursor.is_over(layout.bounds()) {
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
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<State>();
        let visual = self.visual(state, bounds, cursor);
        let colors = Colors::of(theme);
        let t = self.progress(state);
        let overlay = if visual.pressed {
            PRESSED_OVERLAY
        } else if visual.hovered {
            HOVER_OPACITY
        } else {
            0.0
        };
        let opacity = if self.enabled {
            1.0
        } else {
            DISABLED_CONTENT_OPACITY
        };

        match self.kind {
            Kind::Checkbox { mixed } => draw_checkbox(
                renderer,
                theme,
                bounds,
                &colors,
                (overlay, opacity),
                t,
                (mixed, box_size(state.touch)),
                visual.focused,
            ),
            Kind::Radio => draw_radio(
                renderer,
                theme,
                bounds,
                &colors,
                (overlay, opacity),
                t,
                box_size(state.touch),
                visual.focused,
            ),
            Kind::Switch {
                on_off_shapes,
                direction,
            } => draw_switch(
                renderer,
                theme,
                bounds,
                &colors,
                (overlay, opacity),
                t,
                track_size(state.touch),
                (on_off_shapes, direction),
                visual.focused,
            ),
        }
    }
}

/// Draws a radio ring of side `side` centered in `bounds`.
#[allow(clippy::too_many_arguments)]
fn draw_radio<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    theme: &Theme,
    bounds: Rectangle,
    colors: &Colors,
    (overlay, opacity): (f32, f32),
    t: f32,
    side: f32,
    focused: bool,
) {
    let circle = centered(bounds.center(), Size::new(side, side));
    let round = side / 2.0;

    quad(
        renderer,
        circle,
        round,
        state_layer(colors.surface, colors.on_surface, overlay).scale_alpha(opacity),
        Some((colors.outline.scale_alpha(opacity), OUTLINE_WIDTH)),
    );

    if t > 0.0 {
        let fill = state_layer(colors.fill, colors.on_fill, overlay);
        quad(renderer, circle, round, fill.scale_alpha(t * opacity), None);

        let dot = side * 0.2 * t;
        quad(
            renderer,
            centered(circle.center(), Size::new(dot * 2.0, dot * 2.0)),
            dot,
            colors.on_fill.scale_alpha(opacity),
            None,
        );
    }

    if focused {
        crate::focus::draw_ring(renderer, theme, circle.expand(RING_GAP), round + RING_GAP);
    }
}

/// Draws a switch with a `track` sized track centered in `bounds`.
#[allow(clippy::too_many_arguments)]
fn draw_switch<Renderer: renderer::Renderer>(
    renderer: &mut Renderer,
    theme: &Theme,
    bounds: Rectangle,
    colors: &Colors,
    (overlay, opacity): (f32, f32),
    t: f32,
    track: Size,
    (on_off_shapes, direction): (bool, ReadingDirection),
    focused: bool,
) {
    let rect = centered(bounds.center(), track);
    let round = track.height / 2.0;
    let off = state_layer(colors.track, colors.on_surface, overlay);
    let on = state_layer(colors.fill, colors.on_fill, overlay);
    let border = (t < 1.0).then(|| {
        (
            colors.outline.scale_alpha((1.0 - t) * opacity),
            OUTLINE_WIDTH,
        )
    });

    quad(
        renderer,
        rect,
        round,
        mix(off, on, t).scale_alpha(opacity),
        border,
    );

    let inset = track.height * 0.125;
    let thumb = track.height / 2.0 - inset;
    let start = rect.x + inset + thumb;
    let end = rect.x + rect.width - inset - thumb;
    let ltr = direction == ReadingDirection::LeftToRight;
    let x = if ltr {
        start + (end - start) * t
    } else {
        end - (end - start) * t
    };
    let y = rect.center_y();

    if on_off_shapes {
        let mark = track.height * 0.36;
        let i_x = if ltr { start } else { end };
        quad(
            renderer,
            centered(Point::new(i_x, y), Size::new(track.height * 0.08, mark)),
            1.0,
            colors.on_fill.scale_alpha(t * opacity),
            None,
        );

        let o_x = if ltr { end } else { start };
        quad(
            renderer,
            centered(Point::new(o_x, y), Size::new(mark, mark)),
            mark / 2.0,
            Color::TRANSPARENT,
            Some((
                colors.outline.scale_alpha((1.0 - t) * opacity),
                track.height * 0.07,
            )),
        );
    }

    quad(
        renderer,
        centered(Point::new(x, y), Size::new(thumb * 2.0, thumb * 2.0)),
        thumb,
        mix(colors.off_thumb, colors.on_fill, t).scale_alpha(opacity),
        None,
    );

    if focused {
        crate::focus::draw_ring(renderer, theme, rect.expand(RING_GAP), round + RING_GAP);
    }
}

impl<'a, Message, Renderer> From<Toggle<Message>> for iced::Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: geometry::Renderer + 'a,
{
    fn from(toggle: Toggle<Message>) -> Self {
        iced::Element::new(toggle)
    }
}

/// A check box. Built by [`checkbox`].
pub struct Checkbox<'a, Message> {
    value: Option<bool>,
    tristate: bool,
    on_change: Option<Box<dyn Fn(Option<bool>) -> Message + 'a>>,
    enabled: bool,
    id: Option<Id>,
}

/// A check box showing `value`: checked (`true` or `Some(true)`), unchecked
/// (`false` or `Some(false)`) or mixed (`None`, a dash).
///
/// The box has a 1.5-pixel outline while unchecked and fills with the
/// selected fill of the theme when checked, drawing the check mark as it
/// fills. It is 20 pixels for a pointer and 24 on a touch screen.
pub fn checkbox<'a, Message>(value: impl Into<Option<bool>>) -> Checkbox<'a, Message> {
    Checkbox {
        value: value.into(),
        tristate: false,
        on_change: None,
        enabled: true,
        id: None,
    }
}

impl<'a, Message> Checkbox<'a, Message> {
    /// Sets the closure that turns the next value into the message
    /// published on a click, Enter or Space. Without it the box is disabled.
    ///
    /// The next value is `Some(true)` after unchecked, `Some(false)` after
    /// checked, or `None` (mixed) after checked when [`Checkbox::tristate`],
    /// and `Some(false)` after mixed.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Option<bool>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Makes a click cycle unchecked → checked → mixed.
    #[must_use]
    pub fn tristate(mut self, tristate: bool) -> Self {
        self.tristate = tristate;
        self
    }

    /// Sets whether the box responds to input (true by default); a disabled
    /// box is dimmed and skipped by focus traversal.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the widget [`Id`], used to focus the box programmatically and to
    /// find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The painted toggle with its activation message.
    pub(crate) fn into_toggle(self) -> Toggle<Message> {
        let enabled = self.enabled && self.on_change.is_some();
        let next = next_check_value(self.value, self.tristate);

        Toggle {
            kind: Kind::Checkbox {
                mixed: self.value.is_none(),
            },
            selected: self.value != Some(false),
            enabled,
            on_activate: self.on_change.filter(|_| enabled).map(|f| f(next)),
            id: self.id,
            interactive: true,
        }
    }
}

impl<'a, Message: Clone + 'a> From<Checkbox<'a, Message>> for Element<'a, Message> {
    fn from(checkbox: Checkbox<'a, Message>) -> Self {
        checkbox.into_toggle().into()
    }
}

/// A radio button. Built by [`radio`].
pub struct Radio<'a, T, Message> {
    value: T,
    selected: Option<T>,
    toggleable: bool,
    on_change: Option<Box<dyn Fn(Option<T>) -> Message + 'a>>,
    enabled: bool,
    id: Option<Id>,
}

/// A radio button for `value` in a group whose current selection is
/// `selected`; it is selected when both are equal.
///
/// The ring has a 1.5-pixel outline while unselected and fills with the
/// selected fill of the theme when selected, with a dot in its content
/// color. It is 20 pixels for a pointer and 24 on a touch screen.
pub fn radio<'a, T, Message>(value: T, selected: Option<T>) -> Radio<'a, T, Message> {
    Radio {
        value,
        selected,
        toggleable: false,
        on_change: None,
        enabled: true,
        id: None,
    }
}

impl<'a, T: PartialEq, Message> Radio<'a, T, Message> {
    /// Sets the closure that turns the new selection into the message
    /// published on a click, Enter or Space: `Some(value)` to select this
    /// radio, or `None` to clear a [`Radio::toggleable`] group. Without it
    /// the radio is disabled.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Option<T>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Makes activating the selected radio clear the group.
    #[must_use]
    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    /// Sets whether the radio responds to input (true by default); a
    /// disabled radio is dimmed and skipped by focus traversal.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the widget [`Id`].
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The painted toggle with its activation message.
    pub(crate) fn into_toggle(self) -> Toggle<Message> {
        let enabled = self.enabled && self.on_change.is_some();
        let selected = self.selected.as_ref() == Some(&self.value);
        let on_activate = match self.on_change.filter(|_| enabled) {
            Some(on_change) => {
                activate_radio(self.value, self.selected.as_ref(), self.toggleable).map(on_change)
            }
            None => None,
        };

        Toggle {
            kind: Kind::Radio,
            selected,
            enabled,
            on_activate,
            id: self.id,
            interactive: true,
        }
    }
}

impl<'a, T: PartialEq, Message: Clone + 'a> From<Radio<'a, T, Message>> for Element<'a, Message> {
    fn from(radio: Radio<'a, T, Message>) -> Self {
        radio.into_toggle().into()
    }
}

/// A switch. Built by [`switch`].
pub struct Switch<'a, Message> {
    value: bool,
    on_toggle: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    on_off_shapes: bool,
    direction: ReadingDirection,
    enabled: bool,
    id: Option<Id>,
}

/// A switch showing `value`.
///
/// The track is the highest surface container with an outline while off
/// and the selected fill of the theme while on; the thumb slides toward
/// the end of the reading direction. The track is 42 × 24 pixels for a
/// pointer and 52 × 30 on a touch screen, in a target 8 pixels wider.
pub fn switch<'a, Message>(value: bool) -> Switch<'a, Message> {
    Switch {
        value,
        on_toggle: None,
        on_off_shapes: false,
        direction: ReadingDirection::LeftToRight,
        enabled: true,
        id: None,
    }
}

impl<'a, Message> Switch<'a, Message> {
    /// Sets the closure that turns the new value into the message published
    /// on a click, Enter or Space. Without it the switch is disabled.
    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_toggle = Some(Box::new(on_toggle));
        self
    }

    /// Marks the on state with an "I" and the off state with an "O" on the
    /// track, so the state does not rely on color alone.
    #[must_use]
    pub fn on_off_shapes(mut self, on_off_shapes: bool) -> Self {
        self.on_off_shapes = on_off_shapes;
        self
    }

    /// Sets the reading direction; the thumb of an on switch sits at its
    /// end. Left to right by default.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the switch responds to input (true by default); a
    /// disabled switch is dimmed and skipped by focus traversal.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the widget [`Id`].
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// The painted toggle with its activation message.
    pub(crate) fn into_toggle(self) -> Toggle<Message> {
        let enabled = self.enabled && self.on_toggle.is_some();
        let next = !self.value;

        Toggle {
            kind: Kind::Switch {
                on_off_shapes: self.on_off_shapes,
                direction: self.direction,
            },
            selected: self.value,
            enabled,
            on_activate: self.on_toggle.filter(|_| enabled).map(|f| f(next)),
            id: self.id,
            interactive: true,
        }
    }
}

impl<'a, Message: Clone + 'a> From<Switch<'a, Message>> for Element<'a, Message> {
    fn from(switch: Switch<'a, Message>) -> Self {
        switch.into_toggle().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Check(Option<bool>),
        Pick(Option<u8>),
        Switch(bool),
    }

    #[test]
    fn a_check_box_publishes_its_next_value() {
        let toggle = checkbox(false).on_change(Message::Check).into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Check(Some(true))));
        assert!(!toggle.selected);

        let toggle = checkbox(true)
            .tristate(true)
            .on_change(Message::Check)
            .into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Check(None)));

        let toggle = checkbox(None).on_change(Message::Check).into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Check(Some(false))));
        assert!(toggle.selected);
        assert_eq!(toggle.kind, Kind::Checkbox { mixed: true });
    }

    #[test]
    fn controls_without_a_callback_or_disabled_have_no_message() {
        let toggle = checkbox::<Message>(false).into_toggle();
        assert!(!toggle.enabled);
        assert_eq!(toggle.on_activate, None);

        let toggle = switch(false)
            .on_toggle(Message::Switch)
            .enabled(false)
            .into_toggle();
        assert!(!toggle.enabled);
        assert_eq!(toggle.on_activate, None);
    }

    #[test]
    fn a_radio_selects_and_clears_when_toggleable() {
        let toggle = radio(1, Some(2)).on_change(Message::Pick).into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Pick(Some(1))));
        assert!(!toggle.selected);

        let toggle = radio(1, Some(1)).on_change(Message::Pick).into_toggle();
        assert!(toggle.selected && toggle.enabled);
        assert_eq!(toggle.on_activate, None);

        let toggle = radio(1, Some(1))
            .toggleable(true)
            .on_change(Message::Pick)
            .into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Pick(None)));
    }

    #[test]
    fn a_switch_publishes_the_opposite_value() {
        let toggle = switch(true).on_toggle(Message::Switch).into_toggle();
        assert_eq!(toggle.on_activate, Some(Message::Switch(false)));
        assert!(!toggle.leads());
        assert!(checkbox::<Message>(true).into_toggle().leads());
    }

    #[test]
    fn the_check_mark_draws_progressively() {
        let points = [
            Point::new(0.0, 0.0),
            Point::new(3.0, 0.0),
            Point::new(3.0, 4.0),
        ];

        assert_eq!(partial_polyline(&points, 0.0), vec![Point::ORIGIN]);
        assert_eq!(
            partial_polyline(&points, 0.5),
            vec![Point::ORIGIN, Point::new(3.0, 0.0), Point::new(3.0, 0.5)]
        );
        assert_eq!(partial_polyline(&points, 1.0), points.to_vec());
    }

    #[test]
    fn mix_interpolates_every_channel() {
        let color = mix(Color::BLACK, Color::WHITE, 0.25);
        assert!((color.r - 0.25).abs() < 1e-6);
        assert!((color.a - 1.0).abs() < 1e-6);
    }
}
