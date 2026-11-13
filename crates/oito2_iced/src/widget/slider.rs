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

//! Sliders: a value picked along a thick rounded track with a ring thumb,
//! with an optional label and value text above the track and labeled
//! [marks](Mark) along it.
//!
//! The track is inset by the thumb radius from both ends, so the thumb
//! stays inside the slider at the minimum and the maximum. A click or a
//! drag on the track picks the value under the pointer; while focused, the
//! arrow keys move the value by one division (or by 5 % of the range on a
//! continuous slider), Page Up and Page Down by a tenth of the range, and
//! Home and End to the minimum and the maximum. A halo shows around the
//! thumb while the pointer is over the slider, while it is dragged and while
//! it has the keyboard focus.
//!
//! The value lives in the application: the slider shows `value` and
//! publishes new values through [`Slider::on_change`]; without it the
//! slider is disabled.
//!
//! ```no_run
//! use oito2_iced::widget::slider;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Volume(f32),
//! }
//!
//! # fn view<'a>(volume: f32) -> oito2_iced::Element<'a, Message> {
//! slider::slider(volume)
//!     .range(0.0, 100.0)
//!     .divisions(10)
//!     .label("Volume")
//!     .value_text(|value| format!("{value:.0}%"))
//!     .marks([slider::mark(0.0).label("0%"), slider::mark(100.0).label("100%")])
//!     .on_change(Message::Volume)
//!     .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Paragraph as _};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::widget::text::IntoFragment;
use iced::widget::{Space, column, container, row};
use iced::{
    Alignment, Border, Color, Element as IcedElement, Event, Length, Padding, Pixels, Point,
    Rectangle, Shadow, Size, Vector, mouse, touch,
};

use super::styled_text;
use super::toggle_group::ITEM_TEXT;
use crate::theme::DISABLED_CONTENT_OPACITY;
use crate::theme::text::Class as TextClass;
use crate::tokens::{spacing, typography};
use crate::{Element, Theme};

/// The radius of the thumb, which is also the inset of the track from the
/// slider ends.
pub const THUMB_RADIUS: f32 = 10.0;

/// The thickness of the track.
pub const TRACK_HEIGHT: f32 = 6.0;

/// The default length of a vertical slider.
pub const VERTICAL_LENGTH: f32 = 192.0;

/// How much thicker the active part of the track is than the rest.
const ACTIVE_TRACK_EXTRA: f32 = 2.0;

/// The radius of the halo around the thumb.
const HALO_RADIUS: f32 = 16.0;

/// The width of the ring of the thumb.
const RING_WIDTH: f32 = 2.0;

/// The side of a mark dot.
const MARK_SIZE: f32 = 4.0;

/// The height of the marks of a horizontal slider with labels.
const LABELED_MARKS_HEIGHT: f32 = 24.0;

/// The part of the range an arrow key moves a continuous slider.
const CONTINUOUS_STEP: f32 = 0.05;

/// The part of the range Page Up and Page Down move the value.
const PAGE_STEP: f32 = 0.1;

/// A labeled point on the track of a [`Slider`]. Built by [`mark`].
#[derive(Debug, Clone, PartialEq)]
pub struct Mark<'a> {
    value: f32,
    label: Option<Cow<'a, str>>,
}

/// A mark at `value`.
pub fn mark<'a>(value: f32) -> Mark<'a> {
    Mark { value, label: None }
}

impl<'a> Mark<'a> {
    /// Shows `label` under the mark (beside it on a vertical slider).
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }
}

/// A slider for one value. See [`slider`].
pub struct Slider<'a, Message> {
    value: f32,
    min: f32,
    max: f32,
    divisions: Option<u32>,
    on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_change_start: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_change_end: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    label: Option<Cow<'a, str>>,
    value_text: Option<Box<dyn Fn(f32) -> String + 'a>>,
    show_value_indicator: bool,
    marks: Vec<Mark<'a>>,
    vertical: bool,
    length: f32,
    id: Option<Id>,
}

/// A slider showing `value`, between 0 and 1 unless [`Slider::range`]
/// says otherwise. Without [`Slider::on_change`] the slider is disabled.
pub fn slider<'a, Message>(value: f32) -> Slider<'a, Message> {
    Slider {
        value,
        min: 0.0,
        max: 1.0,
        divisions: None,
        on_change: None,
        on_change_start: None,
        on_change_end: None,
        label: None,
        value_text: None,
        show_value_indicator: false,
        marks: Vec::new(),
        vertical: false,
        length: VERTICAL_LENGTH,
        id: None,
    }
}

impl<'a, Message> Slider<'a, Message> {
    /// Sets the smallest and the largest value.
    #[must_use]
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Splits the range into `divisions` equal steps; the value snaps to
    /// them. A slider without divisions is continuous.
    #[must_use]
    pub fn divisions(mut self, divisions: u32) -> Self {
        self.divisions = (divisions > 0).then_some(divisions);
        self
    }

    /// Sets the function that turns a new value into a message, called
    /// while the value changes.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or disables the slider with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(f32) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(f32) -> Message + 'a>);
        self
    }

    /// Sets the function called with the value when a change starts: a
    /// press on the track or a key press.
    #[must_use]
    pub fn on_change_start(mut self, on_change_start: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change_start = Some(Box::new(on_change_start));
        self
    }

    /// Sets the function called with the value when a change ends: the
    /// release of a drag or a key press.
    #[must_use]
    pub fn on_change_end(mut self, on_change_end: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_change_end = Some(Box::new(on_change_end));
        self
    }

    /// Shows `label` above the track.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Formats the value; the formatted value shows above the track and,
    /// with [`Slider::show_value_indicator`], above the thumb while
    /// dragging.
    #[must_use]
    pub fn value_text(mut self, format: impl Fn(f32) -> String + 'a) -> Self {
        self.value_text = Some(Box::new(format));
        self
    }

    /// Sets whether the formatted value shows above the thumb while it is
    /// dragged (false by default).
    #[must_use]
    pub fn show_value_indicator(mut self, show: bool) -> Self {
        self.show_value_indicator = show;
        self
    }

    /// Sets the labeled points along the track.
    #[must_use]
    pub fn marks(mut self, marks: impl IntoIterator<Item = Mark<'a>>) -> Self {
        self.marks = marks.into_iter().collect();
        self
    }

    /// Runs the track up instead of across, with the minimum at the
    /// bottom.
    #[must_use]
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Sets the height of a vertical slider ([`VERTICAL_LENGTH`] by
    /// default).
    #[must_use]
    pub fn length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }

    /// Sets the widget [`Id`] of the track, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// The position of `value` along the range, from 0 to 1.
fn fraction(value: f32, min: f32, max: f32) -> f32 {
    if max <= min {
        0.0
    } else {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }
}

/// Snaps the position `t` to the nearest of `divisions` equal steps.
fn discretize(t: f32, divisions: Option<u32>) -> f32 {
    match divisions {
        Some(divisions) => {
            let divisions = divisions as f32;
            (t * divisions).round() / divisions
        }
        None => t,
    }
}

/// The position of the value at `t` after the keyboard change `key`,
/// snapped to the divisions.
fn stepped(t: f32, divisions: Option<u32>, key: Step) -> f32 {
    let unit = divisions.map_or(CONTINUOUS_STEP, |divisions| 1.0 / divisions as f32);
    let page = discretize(PAGE_STEP, divisions).max(unit);

    let next = match key {
        Step::Increase => t + unit,
        Step::Decrease => t - unit,
        Step::PageUp => t + page,
        Step::PageDown => t - page,
        Step::Home => 0.0,
        Step::End => 1.0,
    };

    discretize(next.clamp(0.0, 1.0), divisions)
}

/// A keyboard change of the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Increase,
    Decrease,
    PageUp,
    PageDown,
    Home,
    End,
}

impl<'a, Message> From<Slider<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(slider: Slider<'a, Message>) -> Self {
        let value_text = slider
            .value_text
            .as_ref()
            .map(|format| format(slider.value));
        let indicator = if slider.show_value_indicator {
            value_text.clone()
        } else {
            None
        };

        let track: Element<'a, Message> = IcedElement::new(Track {
            value: slider.value,
            min: slider.min,
            max: slider.max,
            divisions: slider.divisions,
            on_change: slider.on_change,
            on_change_start: slider.on_change_start,
            on_change_end: slider.on_change_end,
            indicator,
            vertical: slider.vertical,
            length: slider.length,
            id: slider.id,
        });

        let body: Element<'a, Message> = if slider.marks.is_empty() {
            track
        } else {
            let marks: Element<'a, Message> = IcedElement::new(Marks::new(
                slider.marks,
                slider.value,
                slider.min,
                slider.max,
                slider.vertical,
                slider.length,
            ));

            if slider.vertical {
                row![track, marks].spacing(spacing::XS).into()
            } else {
                column![track, marks].spacing(spacing::XS).into()
            }
        };

        if slider.label.is_none() && value_text.is_none() {
            return body;
        }

        let mut header = row![].spacing(spacing::SM).align_y(Alignment::Center);

        if let Some(label) = slider.label {
            let label = styled_text(label, ITEM_TEXT);
            header = if slider.vertical {
                header.push(label)
            } else {
                header.push(label.width(Length::Fill))
            };
        }

        if let Some(value) = value_text {
            header = header.push(styled_text(value, ITEM_TEXT));
        }

        let content = column![header, body].spacing(spacing::XS);

        if slider.vertical {
            content.into()
        } else {
            container(content.width(Length::Fill))
                .width(Length::Fill)
                .into()
        }
    }
}

/// The track and thumb of a [`Slider`], which take the pointer and the
/// keyboard.
struct Track<'a, Message> {
    value: f32,
    min: f32,
    max: f32,
    divisions: Option<u32>,
    on_change: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_change_start: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_change_end: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    indicator: Option<String>,
    vertical: bool,
    length: f32,
    id: Option<Id>,
}

/// The interaction state of a [`Track`].
#[derive(Debug, Clone, Copy, Default)]
struct TrackState {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the halo.
    keyboard: bool,
    /// The value last published while dragging, when dragging.
    dragging: Option<f32>,
}

impl operation::Focusable for TrackState {
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

impl<Message> Track<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_change.is_some() && self.max > self.min
    }

    /// The value at the position `t` along the range; on a slider with
    /// divisions, the value of the nearest division.
    fn value_at(&self, t: f32) -> f32 {
        match self.divisions {
            Some(divisions) => {
                let divisions = divisions as f32;
                self.min + (self.max - self.min) * (t * divisions).round() / divisions
            }
            None => self.min + (self.max - self.min) * t,
        }
    }

    /// The position along the range of the point `position` on the track.
    fn locate(&self, bounds: Rectangle, position: Point) -> f32 {
        let t = if self.vertical {
            let extent = bounds.height - 2.0 * THUMB_RADIUS;
            (bounds.y + bounds.height - THUMB_RADIUS - position.y) / extent
        } else {
            let extent = bounds.width - 2.0 * THUMB_RADIUS;
            (position.x - bounds.x - THUMB_RADIUS) / extent
        };

        discretize(t.clamp(0.0, 1.0), self.divisions)
    }

    /// The center of the thumb at the position `t`.
    fn thumb_center(&self, bounds: Rectangle, t: f32) -> Point {
        if self.vertical {
            let extent = bounds.height - 2.0 * THUMB_RADIUS;
            Point::new(
                bounds.center_x(),
                bounds.y + bounds.height - THUMB_RADIUS - extent * t,
            )
        } else {
            let extent = bounds.width - 2.0 * THUMB_RADIUS;
            Point::new(bounds.x + THUMB_RADIUS + extent * t, bounds.center_y())
        }
    }

    /// Publishes a change to `value`, between the start and end messages
    /// when `whole`.
    fn publish(&self, shell: &mut Shell<'_, Message>, value: f32, whole: bool) {
        if whole && let Some(start) = &self.on_change_start {
            shell.publish(start(self.value));
        }

        if let Some(on_change) = &self.on_change {
            shell.publish(on_change(value));
        }

        if whole && let Some(end) = &self.on_change_end {
            shell.publish(end(value));
        }
    }

    /// The key that changes the value, if `key` is one.
    fn step_of(&self, key: &keyboard::Key) -> Option<Step> {
        let keyboard::Key::Named(named) = key else {
            return None;
        };

        Some(match named {
            key::Named::ArrowUp | key::Named::ArrowRight => Step::Increase,
            key::Named::ArrowDown | key::Named::ArrowLeft => Step::Decrease,
            key::Named::PageUp => Step::PageUp,
            key::Named::PageDown => Step::PageDown,
            key::Named::Home => Step::Home,
            key::Named::End => Step::End,
            _ => return None,
        })
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Track<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TrackState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TrackState::default())
    }

    fn size(&self) -> Size<Length> {
        if self.vertical {
            Size::new(
                Length::Fixed(2.0 * THUMB_RADIUS),
                Length::Fixed(self.length),
            )
        } else {
            Size::new(Length::Fill, Length::Fixed(2.0 * THUMB_RADIUS))
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = self.size();

        layout::atomic(limits, size.width, size.height)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<TrackState>();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        } else {
            state.focused = false;
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
        let state = tree.state.downcast_mut::<TrackState>();
        let bounds = layout.bounds();

        if !self.enabled() {
            *state = TrackState::default();
            return;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position_over(bounds) {
                    let value = self.value_at(self.locate(bounds, position));

                    if let Some(start) = &self.on_change_start {
                        shell.publish(start(self.value));
                    }
                    if value != self.value {
                        self.publish(shell, value, false);
                    }

                    state.dragging = Some(value);
                    state.focused = true;
                    state.keyboard = false;
                    shell.capture_event();
                    shell.request_redraw();
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if let Some(last) = state.dragging {
                    if let Some(position) = cursor.land().position() {
                        let value = self.value_at(self.locate(bounds, position));

                        if value != last {
                            self.publish(shell, value, false);
                            state.dragging = Some(value);
                        }
                    }

                    shell.capture_event();
                    shell.request_redraw();
                } else if cursor.is_over(bounds) {
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if let Some(last) = state.dragging.take() {
                    if let Some(end) = &self.on_change_end {
                        shell.publish(end(last));
                    }
                    shell.request_redraw();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused && !shell.is_event_captured() && modifiers.is_empty() =>
            {
                if let Some(step) = self.step_of(key) {
                    let t = fraction(self.value, self.min, self.max);
                    let value = self.value_at(stepped(t, self.divisions, step));

                    if value != self.value {
                        self.publish(shell, value, true);
                    }

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
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let state = tree.state.downcast_ref::<TrackState>();
        let roles = theme.roles();
        let bounds = layout.bounds();
        let enabled = self.enabled();
        let indicator = roles.indicator();

        let (active, inactive, ring) = if enabled {
            (indicator, roles.surface_container_highest, indicator)
        } else {
            (
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
                roles.on_surface.scale_alpha(0.12),
                blend(
                    roles.surface,
                    roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
                ),
            )
        };

        let t = fraction(self.value, self.min, self.max);
        let center = self.thumb_center(bounds, t);
        let start = self.thumb_center(bounds, 0.0);
        let end = self.thumb_center(bounds, 1.0);

        let bar = |from: Point, to: Point, thickness: f32| {
            let (x0, x1) = (from.x.min(to.x), from.x.max(to.x));
            let (y0, y1) = (from.y.min(to.y), from.y.max(to.y));

            if self.vertical {
                Rectangle::new(
                    Point::new(x0 - thickness / 2.0, y0),
                    Size::new(thickness, y1 - y0),
                )
            } else {
                Rectangle::new(
                    Point::new(x0, y0 - thickness / 2.0),
                    Size::new(x1 - x0, thickness),
                )
            }
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: bar(center, end, TRACK_HEIGHT),
                border: Border::default().rounded(TRACK_HEIGHT / 2.0),
                ..renderer::Quad::default()
            },
            inactive,
        );

        let active_height = TRACK_HEIGHT + ACTIVE_TRACK_EXTRA;
        renderer.fill_quad(
            renderer::Quad {
                bounds: bar(start, center, active_height),
                border: Border::default().rounded(active_height / 2.0),
                ..renderer::Quad::default()
            },
            active,
        );

        let hovered = cursor.is_over(bounds);
        let halo =
            enabled && (hovered || state.dragging.is_some() || (state.focused && state.keyboard));

        let circle = |radius: f32| Rectangle {
            x: center.x - radius,
            y: center.y - radius,
            width: 2.0 * radius,
            height: 2.0 * radius,
        };

        if halo {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: circle(HALO_RADIUS),
                    border: Border::default().rounded(HALO_RADIUS),
                    ..renderer::Quad::default()
                },
                indicator.scale_alpha(0.12),
            );
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds: circle(THUMB_RADIUS),
                border: Border::default()
                    .color(ring)
                    .width(RING_WIDTH)
                    .rounded(THUMB_RADIUS),
                shadow: Shadow {
                    color: roles.shadow.scale_alpha(0.2),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
                ..renderer::Quad::default()
            },
            roles.surface,
        );

        if let (Some(label), Some(_)) = (&self.indicator, state.dragging) {
            draw_indicator(renderer, theme, label, center, *viewport);
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.enabled() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

/// The color of `top` painted over the opaque `bottom`.
fn blend(bottom: Color, top: Color) -> Color {
    let mix = |b: f32, t: f32| b + (t - b) * top.a;

    Color::from_rgb(
        mix(bottom.r, top.r),
        mix(bottom.g, top.g),
        mix(bottom.b, top.b),
    )
}

/// Draws the value indicator: `label` in a bubble on the inverse surface
/// above the thumb centered at `center`, over the other widgets.
fn draw_indicator<Renderer>(
    renderer: &mut Renderer,
    theme: &Theme,
    label: &str,
    center: Point,
    viewport: Rectangle,
) where
    Renderer: text::Renderer<Font = iced::Font>,
{
    let roles = theme.roles();
    let style = typography::LABEL_MEDIUM;
    let line_height = text::LineHeight::Absolute(Pixels(style.line_height));
    let font = crate::font::inter(style.weight);

    let paragraph = Renderer::Paragraph::with_text(text::Text {
        content: label,
        bounds: Size::INFINITE,
        size: Pixels(style.size),
        line_height,
        font,
        align_x: text::Alignment::Center,
        align_y: iced::alignment::Vertical::Center,
        shaping: text::Shaping::Advanced,
        wrapping: text::Wrapping::None,
    });
    let text_size = paragraph.min_bounds();

    let padding = Padding::from([spacing::XS, spacing::SM]);
    let width = text_size.width + padding.x();
    let height = text_size.height + padding.y();
    let bubble = Rectangle {
        x: center.x - width / 2.0,
        y: center.y - HALO_RADIUS - spacing::XS - height,
        width,
        height,
    };

    renderer.with_layer(viewport.union(&bubble), |renderer| {
        renderer.fill_quad(
            renderer::Quad {
                bounds: bubble,
                border: Border::default().rounded(height / 2.0),
                ..renderer::Quad::default()
            },
            roles.inverse_surface,
        );

        renderer.fill_text(
            text::Text {
                content: label.to_owned(),
                bounds: Size::new(text_size.width + 1.0, text_size.height),
                size: Pixels(style.size),
                line_height,
                font,
                align_x: text::Alignment::Center,
                align_y: iced::alignment::Vertical::Center,
                shaping: text::Shaping::Advanced,
                wrapping: text::Wrapping::None,
            },
            bubble.center(),
            roles.on_inverse_surface,
            bubble,
        );
    });
}

/// The marks of a [`Slider`]: a dot at each value, in the accent once the
/// value reaches it, with its label under it (beside it when vertical).
struct Marks<'a, Message> {
    values: Vec<f32>,
    labels: Vec<Element<'a, Message>>,
    value: f32,
    min: f32,
    max: f32,
    vertical: bool,
    length: f32,
    labeled: bool,
}

impl<'a, Message: 'a> Marks<'a, Message> {
    fn new(
        marks: Vec<Mark<'a>>,
        value: f32,
        min: f32,
        max: f32,
        vertical: bool,
        length: f32,
    ) -> Self {
        let labeled = marks.iter().any(|mark| mark.label.is_some());
        let values = marks.iter().map(|mark| mark.value).collect();
        let labels = marks
            .into_iter()
            .map(|mark| match mark.label {
                Some(label) => styled_text(label, typography::BODY_SMALL)
                    .class(TextClass::Variant)
                    .into(),
                None => Space::new().into(),
            })
            .collect();

        Self {
            values,
            labels,
            value,
            min,
            max,
            vertical,
            length,
            labeled,
        }
    }

    /// The position of the mark at `index` along the track, from the start
    /// of a horizontal slider or the top of a vertical one.
    fn along(&self, index: usize, extent: f32) -> f32 {
        let t = fraction(self.values[index], self.min, self.max);
        let along = THUMB_RADIUS + (extent - 2.0 * THUMB_RADIUS) * t;

        if self.vertical { extent - along } else { along }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Marks<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        self.labels.iter().map(Tree::new).collect()
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.traverse(&mut |operation| {
            for ((label, tree), label_layout) in self
                .labels
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                label
                    .as_widget_mut()
                    .operate(tree, label_layout, renderer, operation);
            }
        });
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.labels);
    }

    fn size(&self) -> Size<Length> {
        if self.vertical {
            Size::new(Length::Shrink, Length::Fixed(self.length))
        } else {
            Size::new(Length::Fill, Length::Shrink)
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let loose = layout::Limits::new(Size::ZERO, Size::INFINITE);
        let mut nodes: Vec<layout::Node> = self
            .labels
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(label, tree)| label.as_widget_mut().layout(tree, renderer, &loose))
            .collect();

        if self.vertical {
            let height = self.length;
            let widest = nodes
                .iter()
                .map(|node| node.size().width)
                .fold(0.0, f32::max);
            let width = if widest == 0.0 {
                MARK_SIZE
            } else {
                MARK_SIZE + spacing::XS + widest.ceil()
            };

            for (index, node) in nodes.iter_mut().enumerate() {
                let y = self.along(index, height) - node.size().height / 2.0;
                *node = node.clone().move_to(Point::new(MARK_SIZE + spacing::XS, y));
            }

            let size = limits.resolve(
                Length::Shrink,
                Length::Fixed(height),
                Size::new(width, height),
            );
            layout::Node::with_children(size, nodes)
        } else {
            let width = limits.max().width;
            let height = if self.labeled {
                LABELED_MARKS_HEIGHT
            } else {
                MARK_SIZE
            };

            for (index, node) in nodes.iter_mut().enumerate() {
                let x = self.along(index, width) - node.size().width / 2.0;
                *node = node
                    .clone()
                    .move_to(Point::new(x, MARK_SIZE + spacing::XXS));
            }

            layout::Node::with_children(Size::new(width, height), nodes)
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let roles = theme.roles();
        let bounds = layout.bounds();

        for (index, mark) in self.values.iter().enumerate() {
            let reached = *mark <= self.value;
            let color = if reached {
                roles.semantic.primary_text
            } else {
                roles.outline_variant
            };

            let dot = if self.vertical {
                let y = bounds.y + self.along(index, bounds.height);
                Rectangle::new(
                    Point::new(bounds.x, y - MARK_SIZE / 2.0),
                    Size::new(MARK_SIZE, MARK_SIZE),
                )
            } else {
                let x = bounds.x + self.along(index, bounds.width);
                Rectangle::new(
                    Point::new(x - MARK_SIZE / 2.0, bounds.y),
                    Size::new(MARK_SIZE, MARK_SIZE),
                )
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: dot,
                    border: Border::default().rounded(MARK_SIZE / 2.0),
                    ..renderer::Quad::default()
                },
                color,
            );
        }

        for ((label, tree), label_layout) in self
            .labels
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            label
                .as_widget()
                .draw(tree, renderer, theme, style, label_layout, cursor, viewport);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_snap_to_divisions() {
        assert_eq!(discretize(0.44, Some(10)), 0.4);
        assert_eq!(discretize(0.46, Some(10)), 0.5);
        assert_eq!(discretize(0.46, None), 0.46);
    }

    #[test]
    fn keys_move_by_a_division_a_page_or_to_the_ends() {
        let close = |a: f32, b: f32| (a - b).abs() < 1e-5;

        assert!(close(stepped(0.4, Some(10), Step::Increase), 0.5));
        assert!(close(stepped(0.4, Some(10), Step::Decrease), 0.3));
        assert!(close(stepped(0.5, None, Step::Increase), 0.55));
        assert!(close(stepped(0.5, Some(4), Step::PageUp), 0.75));
        assert!(close(stepped(0.5, None, Step::PageDown), 0.4));
        assert!(close(stepped(0.98, None, Step::Increase), 1.0));
        assert!(close(stepped(0.3, Some(10), Step::Home), 0.0));
        assert!(close(stepped(0.3, None, Step::End), 1.0));
    }

    #[test]
    fn fractions_stay_in_range() {
        assert_eq!(fraction(150.0, 0.0, 100.0), 1.0);
        assert_eq!(fraction(25.0, 0.0, 100.0), 0.25);
        assert_eq!(fraction(5.0, 5.0, 5.0), 0.0);
    }
}
