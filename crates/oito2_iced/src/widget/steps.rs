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

//! Step indicators for multi-step flows.
//!
//! [`steps`] shows a list of [`Step`]s in order: those before the current
//! one completed (a check mark), the current one outlined in the primary
//! color, a step marked with [`Step::error`] in the error color with a
//! cross, and the rest upcoming with their number. Lines join the steps,
//! filled up to the current one. The content of the current step shows
//! below the row, or under the current step when the steps run in a
//! [column](Axis::Vertical).
//!
//! The current step lives in the application: with [`Steps::on_select`] a
//! step is clicked, or focused with Tab and activated with Enter or Space,
//! to move to it. With [`Steps::linear`] (the default) only the completed
//! steps and the current one can be reached. [`description`] gives the
//! localized text that describes a step, such as "Profile, Step 2 of 3,
//! Current".
//!
//! In a row, each step header takes at most an equal share of the width,
//! up to [`MAX_HEADER_WIDTH`], and long titles wrap.
//!
//! ```no_run
//! use oito2_iced::widget::steps;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Step(usize),
//! }
//!
//! # fn view<'a>(current: usize) -> oito2_iced::Element<'a, Message> {
//! steps::steps(
//!     [
//!         steps::step("Account").description("Your e-mail"),
//!         steps::step("Profile"),
//!         steps::step("Done"),
//!     ],
//!     current,
//! )
//! .on_select(Message::Step)
//! .into()
//! # }
//! ```

use std::borrow::Cow;
use std::rc::Rc;

use iced::advanced::Renderer as _;
use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::text::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget};
use iced::font::Weight;
use iced::widget::button::{Status as ButtonStatus, Style as ButtonStyle};
use iced::widget::{Button, Column, Row, Space, container, responsive};
use iced::{
    Alignment, Background, Border, Color, Element as IcedElement, Length, Padding, Pixels, Point,
    Rectangle, Size, mouse,
};

use super::{focusable, styled_text};
use crate::l10n::Localizations;
use crate::theme::button::Class;
use crate::theme::{HOVER_OPACITY, PRESSED_OPACITY};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// The diameter of the circle of a step.
pub const INDICATOR_SIZE: f32 = 28.0;

/// The largest width of a step header in a row.
pub const MAX_HEADER_WIDTH: f32 = 160.0;

/// The side of the check mark and the cross in a circle (a literal in the
/// OITO2 design).
const GLYPH_SIZE: f32 = 16.0;

/// The distance from the top of a row to the line between two steps (a
/// literal in the OITO2 design).
const CONNECTOR_TOP: f32 = 14.0;

/// The thickness of the line between two steps (a literal in the OITO2
/// design).
const CONNECTOR_WIDTH: f32 = 2.0;

/// The space above the title of a step in a column (a literal in the OITO2
/// design).
const VERTICAL_TITLE_TOP: f32 = 4.0;

/// The width of the outline of the current step's circle and of the other
/// circles (literals in the OITO2 design).
const OUTLINE_WIDTHS: (f32, f32) = (2.0, 1.5);

/// Whether the steps run in a row or a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Axis {
    /// In a row, with the content of the current step below.
    #[default]
    Horizontal,
    /// In a column, with the content under the current step.
    Vertical,
}

/// The display state of a step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StepState {
    /// After the current step.
    Upcoming,
    /// The current step.
    Current,
    /// Before the current step.
    Completed,
    /// Marked with [`Step::error`].
    Error,
}

/// The state of the step at `index` when `current` is the current step:
/// an error first, then completed, current or upcoming by its position.
#[must_use]
pub fn state_of(index: usize, current: usize, has_error: bool) -> StepState {
    if has_error {
        StepState::Error
    } else if index < current {
        StepState::Completed
    } else if index == current {
        StepState::Current
    } else {
        StepState::Upcoming
    }
}

/// The localized description of a step: its title, its position and its
/// state, such as "Profile, Step 2 of 3, Current". `index` counts from 0.
#[must_use]
pub fn description(
    title: &str,
    index: usize,
    count: usize,
    state: StepState,
    localizations: &Localizations,
) -> String {
    let position = localizations.step_of(
        i64::try_from(index + 1).unwrap_or(i64::MAX),
        i64::try_from(count).unwrap_or(i64::MAX),
    );
    let state = match state {
        StepState::Completed => Some(localizations.step_completed),
        StepState::Current => Some(localizations.step_current),
        StepState::Error => Some(localizations.step_error),
        StepState::Upcoming => None,
    };

    [Some(title), Some(position.as_str()), state]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(", ")
}

/// One step of a [`Steps`]. See [`step`].
pub struct Step<'a, Message> {
    title: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    content: Option<Element<'a, Message>>,
    error: bool,
}

/// A step named `title`.
pub fn step<'a, Message>(title: impl Into<Cow<'a, str>>) -> Step<'a, Message> {
    Step {
        title: title.into(),
        description: None,
        content: None,
        error: false,
    }
}

impl<'a, Message> Step<'a, Message> {
    /// Sets a short second line under the title.
    #[must_use]
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the content shown while the step is current.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets whether the step shows the error state (false by default).
    #[must_use]
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }
}

/// A step indicator. See [`steps`].
pub struct Steps<'a, Message> {
    steps: Vec<Step<'a, Message>>,
    current: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    axis: Axis,
    linear: bool,
}

/// A step indicator showing `steps` in order with `current` as the current
/// step; `current` equal to the number of steps marks all of them done.
/// Without [`Steps::on_select`] the steps can't be clicked or focused.
pub fn steps<'a, Message>(
    steps: impl IntoIterator<Item = Step<'a, Message>>,
    current: usize,
) -> Steps<'a, Message> {
    Steps {
        steps: steps.into_iter().collect(),
        current,
        on_select: None,
        axis: Axis::Horizontal,
        linear: true,
    }
}

impl<'a, Message> Steps<'a, Message> {
    /// Sets the function that turns the index of a clicked step that can be
    /// reached into a message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or makes the steps read-only with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets whether the steps run in a row or a column (a row by default).
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Sets whether only the completed steps and the current one can be
    /// reached (true by default); otherwise any step can.
    #[must_use]
    pub fn linear(mut self, linear: bool) -> Self {
        self.linear = linear;
        self
    }
}

/// What a step header shows and the message it publishes.
struct Header<'a, Message> {
    index: usize,
    title: Cow<'a, str>,
    description: Option<Cow<'a, str>>,
    state: StepState,
    message: Option<Message>,
}

impl<'a, Message: Clone + 'a> Header<'a, Message> {
    /// The clickable title and description, with the circle above them in
    /// a row.
    fn view(&self, vertical: bool) -> Element<'a, Message> {
        let state = self.state;
        let align = if vertical {
            Alignment::Start
        } else {
            Alignment::Center
        };

        let title = styled_text(self.title.clone(), typography::TITLE_SMALL)
            .align_x(align)
            .style(move |theme: &Theme| {
                let roles = theme.roles();
                let color = match state {
                    StepState::Upcoming => roles.on_surface_variant,
                    StepState::Error => roles.semantic.error_text,
                    StepState::Current | StepState::Completed => roles.on_surface,
                };

                iced::widget::text::Style { color: Some(color) }
            });

        let mut text = Column::new().push(title).align_x(align);

        if let Some(description) = &self.description {
            text = text.push(
                styled_text(description.clone(), typography::BODY_SMALL)
                    .align_x(align)
                    .style(|theme: &Theme| iced::widget::text::Style {
                        color: Some(theme.roles().on_surface_variant),
                    }),
            );
        }

        let content: Element<'a, Message> = if vertical {
            container(text)
                .padding(Padding {
                    top: VERTICAL_TITLE_TOP,
                    ..Padding::ZERO
                })
                .into()
        } else {
            Column::new()
                .push(Indicator {
                    index: self.index,
                    state,
                })
                .push(text)
                .spacing(spacing::XS)
                .align_x(Alignment::Center)
                .into()
        };

        let button = Button::new(content)
            .padding(spacing::XXS)
            .on_press_maybe(self.message.clone())
            .class(Class::Custom(Box::new(header_style)));

        let mut wrapper = focusable(button)
            .radius(radius::SMALL)
            .enabled(self.message.is_some());

        if let Some(message) = &self.message {
            wrapper = wrapper.on_activate(message.clone());
        }

        wrapper.into()
    }
}

/// The style of a step header: transparent, with the state layer of the
/// surface text while hovered or pressed.
fn header_style(theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let roles = theme.roles();
    let background = match status {
        ButtonStatus::Hovered => Some(roles.on_surface.scale_alpha(HOVER_OPACITY)),
        ButtonStatus::Pressed => Some(roles.on_surface.scale_alpha(PRESSED_OPACITY)),
        ButtonStatus::Active | ButtonStatus::Disabled => None,
    };

    ButtonStyle {
        background: background.map(Background::Color),
        text_color: roles.on_surface,
        border: Border::default().rounded(radius::SMALL),
        ..ButtonStyle::default()
    }
}

/// The line between two steps, filled once the step after it is reached.
fn connector<'a, Message: 'a>(done: bool, vertical: bool) -> Element<'a, Message> {
    let (width, height) = if vertical {
        (Length::Fixed(CONNECTOR_WIDTH), Length::Fill)
    } else {
        (Length::Fill, Length::Fixed(CONNECTOR_WIDTH))
    };

    container(Space::new())
        .width(width)
        .height(height)
        .style(move |theme: &Theme| {
            let roles = theme.roles();
            let color = if done {
                roles.selected_fill()
            } else {
                roles.outline_variant
            };

            container::Style {
                background: Some(Background::Color(color)),
                border: Border::default().rounded(CONNECTOR_WIDTH / 2.0),
                ..container::Style::default()
            }
        })
        .into()
}

impl<'a, Message: Clone + 'a> From<Steps<'a, Message>> for Element<'a, Message> {
    fn from(steps: Steps<'a, Message>) -> Self {
        let count = steps.steps.len();
        let current = steps.current;
        let linear = steps.linear;
        let on_select = steps.on_select;

        let mut headers = Vec::with_capacity(count);
        let mut contents = Vec::with_capacity(count);

        for (index, step) in steps.steps.into_iter().enumerate() {
            let reachable = !linear || index <= current;

            headers.push(Header {
                index,
                title: step.title,
                description: step.description,
                state: state_of(index, current, step.error),
                message: on_select
                    .as_ref()
                    .filter(|_| reachable)
                    .map(|on_select| on_select(index)),
            });
            contents.push(step.content);
        }

        match steps.axis {
            Axis::Horizontal => horizontal(headers, contents, current),
            Axis::Vertical => vertical(headers, contents, current),
        }
    }
}

/// The steps in a row, with the content of the current step below.
fn horizontal<'a, Message: Clone + 'a>(
    headers: Vec<Header<'a, Message>>,
    mut contents: Vec<Option<Element<'a, Message>>>,
    current: usize,
) -> Element<'a, Message> {
    let headers = Rc::new(headers);
    let count = headers.len().max(1) as f32;

    let row = responsive(move |size| {
        let header_width = MAX_HEADER_WIDTH.min(size.width / count);
        let mut row = Row::new().align_y(Alignment::Start);

        for header in headers.iter() {
            if header.index > 0 {
                row = row.push(
                    container(connector(header.index <= current, false))
                        .padding(Padding {
                            top: CONNECTOR_TOP,
                            right: spacing::SM,
                            bottom: 0.0,
                            left: spacing::SM,
                        })
                        .width(Length::Fill),
                );
            }

            row = row.push(container(header.view(false)).max_width(header_width));
        }

        row.into()
    })
    .height(Length::Shrink);

    let mut column = Column::new().push(row).spacing(spacing::LG);

    if let Some(content) = contents.get_mut(current).and_then(Option::take) {
        column = column.push(content);
    }

    column.into()
}

/// The steps in a column, with the content under the current step.
fn vertical<'a, Message: Clone + 'a>(
    headers: Vec<Header<'a, Message>>,
    contents: Vec<Option<Element<'a, Message>>>,
    current: usize,
) -> Element<'a, Message> {
    let count = headers.len();

    let rows = headers.into_iter().zip(contents).map(|(header, content)| {
        let mut track = Column::new()
            .push(Indicator {
                index: header.index,
                state: header.state,
            })
            .width(Length::Fixed(INDICATOR_SIZE))
            .height(Length::Fill)
            .align_x(Alignment::Center);

        if header.index + 1 < count {
            track = track.push(
                container(connector(header.index < current, true))
                    .padding([spacing::XS, 0.0])
                    .height(Length::Fill),
            );
        }

        let mut body = Column::new()
            .push(header.view(true))
            .spacing(spacing::SM)
            .width(Length::Fill);

        if header.index == current
            && let Some(content) = content
        {
            body = body.push(content);
        }

        Row::new()
            .push(track)
            .push(container(body).padding(Padding {
                bottom: spacing::LG,
                ..Padding::ZERO
            }))
            .spacing(spacing::MD)
            .into()
    });

    Column::with_children(rows).into()
}

/// The circle of a step: its number, a check mark when completed or a
/// cross on error.
struct Indicator {
    index: usize,
    state: StepState,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Indicator {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(INDICATOR_SIZE), Length::Fixed(INDICATOR_SIZE))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, INDICATOR_SIZE, INDICATOR_SIZE)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let roles = theme.roles();
        let bounds = layout.bounds();
        let fill = roles.selected_fill();
        let on_fill = if fill == roles.primary {
            roles.on_primary
        } else {
            roles.on_indicator()
        };

        let (background, border, foreground) = match self.state {
            StepState::Completed => (Some(fill), fill, on_fill),
            StepState::Current => (
                None,
                roles.semantic.primary_text,
                roles.semantic.primary_text,
            ),
            StepState::Error => (None, roles.semantic.error_text, roles.semantic.error_text),
            StepState::Upcoming => (None, roles.outline, roles.on_surface_variant),
        };
        let width = if self.state == StepState::Current {
            OUTLINE_WIDTHS.0
        } else {
            OUTLINE_WIDTHS.1
        };

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border::default()
                    .rounded(INDICATOR_SIZE / 2.0)
                    .width(width)
                    .color(border),
                ..Quad::default()
            },
            background.unwrap_or(Color::TRANSPARENT),
        );

        match self.state {
            StepState::Completed | StepState::Error => {
                draw_glyph(renderer, bounds, self.state == StepState::Error, foreground);
            }
            StepState::Current | StepState::Upcoming => {
                let style = typography::LABEL_MEDIUM;

                renderer.fill_text(
                    text::Text {
                        content: (self.index + 1).to_string(),
                        bounds: Size::new(bounds.width + 1.0, bounds.height),
                        size: Pixels(style.size),
                        line_height: text::LineHeight::Absolute(Pixels(style.line_height)),
                        font: crate::font::inter(Weight::Semibold),
                        align_x: text::Alignment::Center,
                        align_y: iced::alignment::Vertical::Center,
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                    },
                    bounds.center(),
                    foreground,
                    bounds,
                );
            }
        }
    }
}

impl<'a, Message: 'a> From<Indicator> for Element<'a, Message> {
    fn from(indicator: Indicator) -> Self {
        IcedElement::new(indicator)
    }
}

/// Draws the check mark, or the cross when `cross` is set, centered in
/// `bounds`, on the 24-unit grid of the icons.
fn draw_glyph<R: geometry::Renderer>(
    renderer: &mut R,
    bounds: Rectangle,
    cross: bool,
    color: Color,
) {
    let unit = GLYPH_SIZE / 24.0;
    let origin = Point::new(
        bounds.center_x() - GLYPH_SIZE / 2.0,
        bounds.center_y() - GLYPH_SIZE / 2.0,
    );
    let mut frame = Frame::new(renderer, Size::new(GLYPH_SIZE, GLYPH_SIZE));
    let at = |x: f32, y: f32| Point::new(x * unit, y * unit);
    let path = Path::new(|builder| {
        if cross {
            builder.move_to(at(18.0, 6.0));
            builder.line_to(at(6.0, 18.0));
            builder.move_to(at(6.0, 6.0));
            builder.line_to(at(18.0, 18.0));
        } else {
            builder.move_to(at(20.0, 6.0));
            builder.line_to(at(9.0, 17.0));
            builder.line_to(at(4.0, 12.0));
        }
    });

    frame.stroke(
        &path,
        Stroke::default()
            .with_color(color)
            .with_width(2.0 * unit)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
    );

    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_wins_over_the_position() {
        assert_eq!(state_of(0, 1, false), StepState::Completed);
        assert_eq!(state_of(1, 1, false), StepState::Current);
        assert_eq!(state_of(2, 1, false), StepState::Upcoming);
        assert_eq!(state_of(0, 1, true), StepState::Error);
    }
}
