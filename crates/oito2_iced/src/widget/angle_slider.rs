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

//! Angle slider: a round dial that picks an angle in degrees, from 0 at
//! the top clockwise to just under 360, such as a rotation or a gradient
//! direction.
//!
//! A press or a drag points the dial at the pointer, in
//! [step](AngleSlider::step) increments. While focused, the arrow keys
//! turn it by the step (up and right clockwise), Page Up and Page Down by
//! the [page step](AngleSlider::page_step), and Home goes back to 0; the
//! value wraps around. [Markers](AngleSlider::markers) draw ticks at their
//! angles, and the angle shows in the middle of the dial unless
//! [`AngleSlider::show_value`] turns it off.
//!
//! The angle lives in the application: the dial shows `value` and
//! publishes new angles through [`AngleSlider::on_change`]; without it the
//! dial is read-only and skipped by Tab.
//!
//! ```no_run
//! use oito2_iced::widget::angle_slider;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Rotate(f32),
//! }
//!
//! # fn view<'a>(angle: f32) -> oito2_iced::Element<'a, Message> {
//! angle_slider::angle_slider(angle)
//!     .step(5.0)
//!     .on_change(Message::Rotate)
//!     .into()
//! # }
//! ```

use std::f32::consts::FRAC_PI_2;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::text;
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::{Element as IcedElement, Event, Length, Pixels, Point, Rectangle, Size, mouse, touch};

use crate::l10n::Localizations;
use crate::theme::DISABLED_CONTENT_OPACITY;
use crate::tokens::typography;
use crate::{Element, Theme};

/// The default diameter of the dial.
pub const DIAL_SIZE: f32 = 96.0;

/// The default step of the angle, in degrees.
pub const DEFAULT_STEP: f32 = 1.0;

/// The default turn of Page Up and Page Down, in degrees.
pub const DEFAULT_PAGE_STEP: f32 = 15.0;

/// The default angles with a tick.
pub const DEFAULT_MARKERS: [f32; 4] = [0.0, 90.0, 180.0, 270.0];

/// The width of the dial border (a literal in the OITO2 design, as are the
/// other dial measures below).
const BORDER_WIDTH: f32 = 1.0;

/// The distance of the ticks from the dial edge.
const TICK_INSET: f32 = 7.0;

/// The radius of a tick.
const TICK_RADIUS: f32 = 1.5;

/// The distance of the knob center from the dial edge.
const KNOB_INSET: f32 = 12.0;

/// The radius of the knob.
const KNOB_RADIUS: f32 = 6.0;

/// The width of the hand.
const HAND_WIDTH: f32 = 2.0;

/// The radius of the dot in the middle of a dial without the value text.
const HUB_RADIUS: f32 = 3.0;

/// The part of the radius the hand leaves free in the middle while the
/// value shows there.
const HOLLOW: f32 = 0.45;

/// The distance from the center within which the pointer direction is
/// ignored.
const DEAD_ZONE: f32 = 2.0;

/// A round dial picking an angle. See [`angle_slider`].
pub struct AngleSlider<'a, Message> {
    value: f32,
    on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_change_end: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    step: f32,
    page_step: f32,
    markers: Vec<f32>,
    size: f32,
    show_value: bool,
    localizations: Localizations,
    id: Option<Id>,
}

/// A dial showing the angle `value`, in degrees. Without
/// [`AngleSlider::on_change`] it is read-only.
pub fn angle_slider<'a, Message>(value: f32) -> AngleSlider<'a, Message> {
    AngleSlider {
        value,
        on_change: None,
        on_change_end: None,
        step: DEFAULT_STEP,
        page_step: DEFAULT_PAGE_STEP,
        markers: DEFAULT_MARKERS.to_vec(),
        size: DIAL_SIZE,
        show_value: true,
        localizations: Localizations::en(),
        id: None,
    }
}

impl<'a, Message> AngleSlider<'a, Message> {
    /// Sets the function that turns a new angle into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or makes the dial read-only with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(f32) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(f32) -> Message + 'a>);
        self
    }

    /// Sets the function called with the final angle when a press or a
    /// drag ends, and after each key press.
    #[must_use]
    pub fn on_change_end(mut self, on_change_end: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change_end = Some(Box::new(on_change_end));
        self
    }

    /// Sets the increment the angle snaps to and the turn of one arrow key
    /// press ([`DEFAULT_STEP`] by default); a step that is not positive
    /// keeps the default.
    #[must_use]
    pub fn step(mut self, step: f32) -> Self {
        self.step = if step > 0.0 { step } else { DEFAULT_STEP };
        self
    }

    /// Sets the turn of one Page Up or Page Down press
    /// ([`DEFAULT_PAGE_STEP`] by default).
    #[must_use]
    pub fn page_step(mut self, page_step: f32) -> Self {
        self.page_step = page_step;
        self
    }

    /// Sets the angles that get a tick ([`DEFAULT_MARKERS`] by default).
    #[must_use]
    pub fn markers(mut self, markers: impl IntoIterator<Item = f32>) -> Self {
        self.markers = markers.into_iter().collect();
        self
    }

    /// Sets the diameter of the dial ([`DIAL_SIZE`] by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    /// Sets whether the angle shows in the middle of the dial (true by
    /// default).
    #[must_use]
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Formats the angle with the number conventions of `localizations`
    /// (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the widget [`Id`], used to focus the dial programmatically and
    /// to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// `degrees` with a degree sign in the number format of `localizations`,
/// such as "90°" or "22,5°": whole angles without decimals, other angles
/// with one.
pub fn angle_text(degrees: f32, localizations: &Localizations) -> String {
    let number = if degrees.fract() == 0.0 {
        format!("{degrees:.0}")
    } else {
        format!("{degrees:.1}")
    };

    if localizations.locale_name == "pt_BR" {
        format!("{}°", number.replace('.', ","))
    } else {
        format!("{number}°")
    }
}

/// `degrees` wrapped into the range 0 to under 360.
fn wrap(degrees: f32) -> f32 {
    let wrapped = degrees.rem_euclid(360.0);

    if wrapped >= 360.0 { 0.0 } else { wrapped }
}

/// The tree state of an [`AngleSlider`].
#[derive(Debug, Clone, Copy, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the ring.
    keyboard: bool,
    /// Whether the pointer is turning the dial.
    dragging: bool,
    /// The angle last published, which the application may not show yet.
    pending: Option<f32>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<Message> AngleSlider<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_change.is_some()
    }

    /// `degrees` rounded to the nearest step and wrapped.
    fn snap(&self, degrees: f32) -> f32 {
        wrap((degrees / self.step).round() * self.step)
    }

    /// Snaps `degrees` and publishes it when it differs from the angle last
    /// published, or from the value when none is pending.
    fn set(&self, state: &mut State, shell: &mut Shell<'_, Message>, degrees: f32) {
        let next = self.snap(degrees);

        if next != state.pending.unwrap_or(self.value)
            && let Some(on_change) = &self.on_change
        {
            state.pending = Some(next);
            shell.publish(on_change(next));
        }
    }

    /// Publishes the final angle of a change and clears the pending one.
    fn end(&self, state: &mut State, shell: &mut Shell<'_, Message>) {
        if let Some(on_change_end) = &self.on_change_end {
            shell.publish(on_change_end(state.pending.unwrap_or(self.value)));
        }

        state.pending = None;
    }

    /// Turns the dial with `bounds` toward `position`, unless it is too
    /// close to the center to tell a direction.
    fn point_at(
        &self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        bounds: Rectangle,
        position: Point,
    ) {
        let center = bounds.center();
        let (dx, dy) = (position.x - center.x, position.y - center.y);

        if dx.hypot(dy) >= DEAD_ZONE {
            self.set(state, shell, dx.atan2(-dy).to_degrees());
        }
    }

    /// The turn a key gives, or `None` for Home and for keys the dial
    /// ignores; Home is reported as `Some(None)`.
    fn turn_of(&self, key: &keyboard::Key) -> Option<Option<f32>> {
        let keyboard::Key::Named(named) = key else {
            return None;
        };

        Some(match named {
            key::Named::ArrowUp | key::Named::ArrowRight => Some(self.step),
            key::Named::ArrowDown | key::Named::ArrowLeft => Some(-self.step),
            key::Named::PageUp => Some(self.page_step),
            key::Named::PageDown => Some(-self.page_step),
            key::Named::Home => None,
            _ => return None,
        })
    }

    fn label(&self) -> String {
        angle_text(self.value, &self.localizations)
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for AngleSlider<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = Length::Fixed(self.size);

        layout::atomic(limits, side, side)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), bounds, state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), bounds);
        }

        if self.show_value {
            operation.text(None, bounds, &self.label());
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if !self.enabled() {
            *state = State::default();
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position_over(bounds) {
                    state.focused = true;
                    state.keyboard = false;
                    state.dragging = true;
                    self.point_at(state, shell, bounds, position);
                    shell.capture_event();
                    shell.request_redraw();
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.dragging {
                    if let Some(position) = cursor.land().position() {
                        self.point_at(state, shell, bounds, position);
                    }

                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if state.dragging {
                    state.dragging = false;
                    self.end(state, shell);
                    shell.request_redraw();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused && !shell.is_event_captured() && modifiers.is_empty() =>
            {
                if let Some(turn) = self.turn_of(key) {
                    let target = turn.map_or(0.0, |turn| self.value + turn);

                    self.set(state, shell, target);
                    self.end(state, shell);
                    state.keyboard = true;
                    shell.capture_event();
                    shell.request_redraw();
                }
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
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::text::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let bounds = layout.bounds();
        let thumb = if self.enabled() {
            roles.selected_fill()
        } else {
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
        };

        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let radius = bounds.width.min(bounds.height) / 2.0;
        let at = |degrees: f32, distance: f32| {
            let angle = degrees.to_radians() - FRAC_PI_2;

            Point::new(
                center.x + distance * angle.cos(),
                center.y + distance * angle.sin(),
            )
        };

        frame.fill(&Path::circle(center, radius), roles.outline);
        frame.fill(
            &Path::circle(center, (radius - BORDER_WIDTH).max(0.0)),
            roles.surface_container_high,
        );

        for marker in &self.markers {
            frame.fill(
                &Path::circle(at(*marker, radius - TICK_INSET), TICK_RADIUS),
                roles.on_surface_variant,
            );
        }

        let inner = if self.show_value {
            radius * HOLLOW
        } else {
            0.0
        };
        let outer = radius - KNOB_INSET;
        let cap = HAND_WIDTH / 2.0;

        if outer - cap > inner + cap {
            frame.stroke(
                &Path::line(at(self.value, inner + cap), at(self.value, outer - cap)),
                Stroke::default()
                    .with_color(thumb)
                    .with_width(HAND_WIDTH)
                    .with_line_cap(LineCap::Round),
            );
        }

        if !self.show_value {
            frame.fill(&Path::circle(center, HUB_RADIUS), thumb);
        }

        frame.fill(&Path::circle(at(self.value, outer), KNOB_RADIUS), thumb);

        draw_frame(renderer, frame, bounds.position());

        if self.show_value {
            let text_style = typography::LABEL_LARGE;

            renderer.fill_text(
                text::Text {
                    content: self.label(),
                    bounds: bounds.size(),
                    size: Pixels(text_style.size),
                    line_height: text::LineHeight::Absolute(Pixels(text_style.line_height)),
                    font: crate::font::inter(text_style.weight),
                    align_x: text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                },
                bounds.center(),
                roles.on_surface,
                bounds,
            );
        }

        if state.focused && state.keyboard && self.enabled() {
            crate::focus::draw_ring(renderer, theme, bounds, radius);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if !self.enabled() {
            mouse::Interaction::default()
        } else if tree.state.downcast_ref::<State>().dragging {
            mouse::Interaction::Grabbing
        } else if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: 'a> From<AngleSlider<'a, Message>> for Element<'a, Message> {
    fn from(slider: AngleSlider<'a, Message>) -> Self {
        IcedElement::new(slider)
    }
}

/// Draws the geometry of `frame` with its origin at `origin`.
fn draw_frame<Renderer>(renderer: &mut Renderer, frame: Frame<Renderer>, origin: Point)
where
    Renderer: geometry::Renderer,
{
    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angles_wrap_into_a_turn() {
        assert_eq!(wrap(-1.0), 359.0);
        assert_eq!(wrap(374.0), 14.0);
        assert_eq!(wrap(360.0), 0.0);
        assert_eq!(wrap(90.0), 90.0);
    }

    #[test]
    fn angles_read_in_the_locale() {
        assert_eq!(angle_text(90.0, &Localizations::en()), "90°");
        assert_eq!(angle_text(22.5, &Localizations::en()), "22.5°");
        assert_eq!(angle_text(22.5, &Localizations::pt_br()), "22,5°");
    }
}
