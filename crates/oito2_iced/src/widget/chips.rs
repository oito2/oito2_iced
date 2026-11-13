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

//! Choice chip bars: a bar of chips for picking one or more filters.
//!
//! A selected chip shows the secondary container fill and, unless
//! [`ChoiceChipBar::show_checkmark`] turns it off, a check mark before its
//! label. Chips have the medium radius, or are pills with
//! [`ChoiceChipBar::pill`]. Each chip takes part in focus traversal and is
//! activated by a click, Enter or Space.
//!
//! In the [`BarStyle::Row`] style the chips scroll horizontally: the mouse
//! wheel scrolls the row, the row follows the keyboard focus, and arrow
//! buttons and a soft edge shadow appear only on the sides with hidden
//! chips. In the [`BarStyle::Wrap`] style the chips wrap onto more rows.
//!
//! The selection lives in the application: the bar shows `selected` and
//! publishes the index of an activated chip through
//! [`ChoiceChipBar::on_select`].
//!
//! ```no_run
//! use oito2_iced::widget::chips;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Filter(usize),
//! }
//!
//! # fn view<'a>(selected: &[bool]) -> oito2_iced::Element<'a, Message> {
//! chips::choice_chip_bar(["All", "Images", "Videos", "Documents"], selected)
//!     .on_select(Message::Filter)
//!     .into()
//! # }
//! ```

use std::borrow::Cow;
use std::f32::consts::FRAC_PI_2;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::operation::{Focusable, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::gradient::Linear;
use iced::widget::button::{Catalog as _, Status, Style};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::text::IntoFragment;
use iced::widget::{Button, Row, container, row, scrollable};
use iced::{
    Alignment, Background, Border, Color, Element as IcedElement, Event, Gradient, Length, Padding,
    Point, Rectangle, Size, Vector, keyboard, mouse, window,
};

use super::min_size::{Minimum, min_size};
use super::{focusable, styled_text};
use crate::l10n::Localizations;
use crate::theme::button::Class;
use crate::theme::{
    DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY,
    TRANSPARENT, state_layer,
};
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme, density};

/// How a [`ChoiceChipBar`] lays out chips that don't fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarStyle {
    /// One scrollable row with arrow buttons at the edges that hide chips.
    #[default]
    Row,
    /// Several rows that wrap.
    Wrap,
}

/// The distance one arrow button press scrolls, by default.
pub const SCROLL_STEP: f32 = 200.0;

/// The width of the soft shadow at an edge that hides chips.
const UNDERSHOOT_EXTENT: f32 = spacing::MD;

/// The side of the check mark of a selected chip.
const CHECKMARK_SIZE: f32 = sizes::ICON_SIZE - 2.0;

/// The width of a chip border.
const BORDER_WIDTH: f32 = 1.0;

/// The distance one line of a mouse wheel scrolls.
const WHEEL_LINE: f32 = 60.0;

/// A bar of choice chips. See [`choice_chip_bar`].
pub struct ChoiceChipBar<'a, Message> {
    labels: Vec<Cow<'a, str>>,
    selected: Vec<bool>,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    style: BarStyle,
    spacing: f32,
    scroll_step: f32,
    previous_label: Cow<'a, str>,
    next_label: Cow<'a, str>,
    show_checkmark: bool,
    pill: bool,
}

/// A bar with one chip per label; the chip at index `i` is selected when
/// `selected[i]` is true. Without [`ChoiceChipBar::on_select`] the bar is
/// disabled.
pub fn choice_chip_bar<'a, Message, L>(
    labels: impl IntoIterator<Item = L>,
    selected: &[bool],
) -> ChoiceChipBar<'a, Message>
where
    L: IntoFragment<'a>,
{
    let localizations = Localizations::en();

    ChoiceChipBar {
        labels: labels
            .into_iter()
            .map(IntoFragment::into_fragment)
            .collect(),
        selected: selected.to_vec(),
        on_select: None,
        style: BarStyle::Row,
        spacing: spacing::SM,
        scroll_step: SCROLL_STEP,
        previous_label: Cow::Borrowed(localizations.scroll_back),
        next_label: Cow::Borrowed(localizations.scroll_forward),
        show_checkmark: true,
        pill: false,
    }
}

impl<'a, Message> ChoiceChipBar<'a, Message> {
    /// Sets the function that turns the index of an activated chip into a
    /// message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or disables the bar with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets how chips that don't fit are laid out ([`BarStyle::Row`] by
    /// default).
    #[must_use]
    pub fn style(mut self, style: BarStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the space between chips, and between rows in the wrap style.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the distance one arrow button press scrolls
    /// ([`SCROLL_STEP`] by default).
    #[must_use]
    pub fn scroll_step(mut self, step: f32) -> Self {
        self.scroll_step = step;
        self
    }

    /// Takes the tooltips of the arrow buttons from `localizations`
    /// (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.previous_label = Cow::Borrowed(localizations.scroll_back);
        self.next_label = Cow::Borrowed(localizations.scroll_forward);
        self
    }

    /// Sets the tooltip of the backward arrow button.
    #[must_use]
    pub fn previous_label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.previous_label = label.into_fragment();
        self
    }

    /// Sets the tooltip of the forward arrow button.
    #[must_use]
    pub fn next_label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.next_label = label.into_fragment();
        self
    }

    /// Sets whether a selected chip shows a check mark (true by default);
    /// without it only the fill marks the selection.
    #[must_use]
    pub fn show_checkmark(mut self, show: bool) -> Self {
        self.show_checkmark = show;
        self
    }

    /// Makes the chips pills instead of rounded rectangles.
    #[must_use]
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }
}

/// The style of a chip: the surface with an outline, or the secondary
/// container when selected, with the state layer of the pointer.
fn chip_style(theme: &Theme, status: Status, selected: bool, pill: bool) -> Style {
    let roles = theme.roles();
    let corner = if pill { f32::MAX } else { radius::MEDIUM };

    if status == Status::Disabled {
        let background = if selected {
            roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY)
        } else {
            TRANSPARENT
        };
        let border = if selected {
            TRANSPARENT
        } else {
            roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY)
        };

        return Style {
            background: (background != TRANSPARENT).then_some(Background::Color(background)),
            text_color: roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
            border: Border::default()
                .color(border)
                .width(BORDER_WIDTH)
                .rounded(corner),
            ..Style::default()
        };
    }

    let (background, layer, border) = if selected {
        (
            roles.secondary_container,
            roles.on_secondary_container,
            TRANSPARENT,
        )
    } else {
        (roles.surface, roles.on_surface_variant, roles.outline)
    };

    let background = match status {
        Status::Hovered => state_layer(background, layer, HOVER_OPACITY),
        Status::Pressed => state_layer(background, layer, PRESSED_OPACITY),
        Status::Active | Status::Disabled => background,
    };

    Style {
        background: Some(Background::Color(background)),
        text_color: roles.on_surface,
        border: Border::default()
            .color(border)
            .width(BORDER_WIDTH)
            .rounded(corner),
        ..Style::default()
    }
}

/// Builds one chip: the optional check mark and the label, focusable and
/// at least as tall as the control height.
fn chip<'a, Message: Clone + 'a>(
    label: Cow<'a, str>,
    selected: bool,
    show_checkmark: bool,
    pill: bool,
    message: Option<Message>,
) -> Element<'a, Message> {
    let enabled = message.is_some();
    let label = container(styled_text(label, typography::LABEL_LARGE)).padding([0.0, spacing::SM]);

    let content: Element<'a, Message> = if selected && show_checkmark {
        row![CheckMark { enabled }, label]
            .align_y(Alignment::Center)
            .into()
    } else {
        label.into()
    };

    let button = Button::new(min_size(content, Minimum::None, Minimum::Control(0.0)))
        .padding(Padding::from([0.0, spacing::XS + BORDER_WIDTH]))
        .on_press_maybe(message.clone())
        .class(Class::Custom(Box::new(move |theme, status| {
            chip_style(theme, status, selected, pill)
        })));

    let mut wrapper = focusable(button)
        .radius(if pill { f32::MAX } else { radius::MEDIUM })
        .enabled(enabled);

    if let Some(message) = message {
        wrapper = wrapper.on_activate(message);
    }

    wrapper.into()
}

impl<'a, Message> From<ChoiceChipBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: ChoiceChipBar<'a, Message>) -> Self {
        let chips: Vec<Element<'a, Message>> = bar
            .labels
            .into_iter()
            .enumerate()
            .map(|(index, label)| {
                let selected = bar.selected.get(index).copied().unwrap_or(false);
                let message = bar.on_select.as_ref().map(|on_select| on_select(index));

                chip(label, selected, bar.show_checkmark, bar.pill, message)
            })
            .collect();

        let row = Row::with_children(chips).spacing(bar.spacing);

        match bar.style {
            BarStyle::Wrap => row.wrap().vertical_spacing(bar.spacing).into(),
            BarStyle::Row => {
                let content = scrollable(
                    container(row.align_y(Alignment::Center))
                        .padding([sizes::FOCUS_BORDER_WIDTH, sizes::FOCUS_BORDER_WIDTH]),
                )
                .direction(Direction::Horizontal(Scrollbar::hidden()))
                .width(Length::Fill);

                IcedElement::new(Scroller {
                    children: vec![
                        content.into(),
                        arrow(false, bar.previous_label),
                        arrow(true, bar.next_label),
                    ],
                    step: bar.scroll_step,
                    arrows: cfg!(feature = "svg"),
                })
            }
        }
    }
}

/// The content of an arrow button: a chevron toward the start or the end
/// of the row, with its tooltip.
#[cfg(feature = "svg")]
fn arrow<'a, Message: 'a>(forward: bool, label: Cow<'a, str>) -> Element<'a, Message> {
    use crate::icon::{icon, lucide};

    let chevron = if forward {
        lucide::CHEVRON_RIGHT
    } else {
        lucide::CHEVRON_LEFT
    };

    super::tooltip(
        min_size(icon(chevron), Minimum::Control(0.0), Minimum::Control(0.0)),
        label,
    )
    .into()
}

/// Without icons, the arrow buttons are left out.
#[cfg(not(feature = "svg"))]
fn arrow<'a, Message: 'a>(_forward: bool, _label: Cow<'a, str>) -> Element<'a, Message> {
    iced::widget::Space::new().into()
}

/// The check mark before the label of a selected chip.
struct CheckMark {
    enabled: bool,
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for CheckMark
where
    Renderer: geometry::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(CHECKMARK_SIZE), Length::Fixed(CHECKMARK_SIZE))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, CHECKMARK_SIZE, CHECKMARK_SIZE)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let roles = theme.roles();
        let color = if self.enabled {
            roles.on_secondary_container
        } else {
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
        };

        let bounds = layout.bounds();
        let size = bounds.width;
        let mut frame = Frame::new(renderer, bounds.size());
        let point = |x: f32, y: f32| Point::new(size * x, size * y);
        let path = Path::new(|builder| {
            builder.move_to(point(0.15, 0.45));
            builder.line_to(point(0.4, 0.7));
            builder.line_to(point(0.85, 0.25));
        });

        frame.stroke(
            &path,
            Stroke::default()
                .with_color(color)
                .with_width(2.0 * size / 24.0)
                .with_line_cap(LineCap::Butt)
                .with_line_join(LineJoin::Miter),
        );

        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message, Renderer> From<CheckMark> for IcedElement<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer + 'a,
{
    fn from(mark: CheckMark) -> Self {
        IcedElement::new(mark)
    }
}

/// A horizontal scroll view with arrow buttons on the sides that hide
/// content and a soft shadow at those edges. Its children are the
/// scrollable, the backward arrow and the forward arrow.
struct Scroller<'a, Message> {
    children: Vec<Element<'a, Message>>,
    step: f32,
    arrows: bool,
}

/// The scroll state of a [`Scroller`], read back from its scrollable.
#[derive(Debug, Clone, Copy, Default)]
struct ScrollerState {
    /// How far the row is scrolled.
    offset: f32,
    can_go_back: bool,
    can_go_forward: bool,
    /// The arrow being pressed: `true` for the forward one.
    pressed: Option<bool>,
    /// The bounds of the focused chip the row last scrolled to.
    revealed: Option<Rectangle>,
}

/// What a [`Probe`] finds in the scrollable: its bounds, the bounds of its
/// content, its translation and the bounds of the focused chip.
#[derive(Debug, Clone, Copy, Default)]
struct Found {
    viewport: Rectangle,
    content: Rectangle,
    translation: Vector,
    focused: Option<Rectangle>,
}

/// An operation that scrolls the first scrollable it meets by `delta`
/// pixels and reports what it found.
struct Probe {
    delta: f32,
    found: Option<Found>,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        if self.found.is_some() {
            return;
        }

        if self.delta != 0.0 {
            state.scroll_by(
                AbsoluteOffset {
                    x: self.delta,
                    y: 0.0,
                },
                bounds,
                content_bounds,
            );
        }

        self.found = Some(Found {
            viewport: bounds,
            content: content_bounds,
            translation,
            focused: None,
        });
    }

    fn focusable(&mut self, _id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
        if state.is_focused()
            && let Some(found) = &mut self.found
        {
            found.focused = Some(bounds);
        }
    }
}

/// The horizontal scroll that brings `target` fully into `viewport` when
/// the content is shifted by `translation`; zero when it already shows.
fn reveal_delta(viewport: Rectangle, translation: f32, target: Rectangle) -> f32 {
    let margin = sizes::FOCUS_BORDER_WIDTH;
    let left = target.x - translation - margin;
    let right = target.x + target.width - translation + margin;

    if left < viewport.x {
        left - viewport.x
    } else if right > viewport.x + viewport.width {
        right - (viewport.x + viewport.width)
    } else {
        0.0
    }
}

impl<Message> Scroller<'_, Message> {
    /// Scrolls the row by `delta` and returns what the scrollable reports
    /// afterwards.
    fn scroll(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        delta: f32,
    ) -> Option<Found> {
        let content_layout = layout.children().next()?;
        let content = self.children.first_mut()?;

        if delta != 0.0 {
            let mut probe = Probe { delta, found: None };
            content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                &mut probe,
            );
        }

        let mut probe = Probe {
            delta: 0.0,
            found: None,
        };
        content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            &mut probe,
        );

        probe.found
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Scroller<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ScrollerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ScrollerState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let [content, back, forward] = self.children.as_mut_slice() else {
            return layout::Node::new(Size::ZERO);
        };
        let [content_tree, back_tree, forward_tree] = tree.children.as_mut_slice() else {
            return layout::Node::new(Size::ZERO);
        };
        let state = tree.state.downcast_mut::<ScrollerState>();

        let limits = limits.width(Length::Fill);
        let full = content
            .as_widget_mut()
            .layout(content_tree, renderer, &limits);
        let width = full.size().width;
        let content_width = full
            .children()
            .first()
            .map_or(0.0, |inner| inner.size().width);
        let arrow = density::control_height();

        let (can_go_back, can_go_forward) = if !self.arrows || content_width <= width + 0.5 {
            (false, false)
        } else {
            let can_go_back = state.offset > 0.5;
            let viewport = width - arrow - if can_go_back { arrow } else { 0.0 };

            (can_go_back, content_width - state.offset > viewport + 0.5)
        };

        state.can_go_back = can_go_back;
        state.can_go_forward = can_go_forward;

        let start = if can_go_back { arrow } else { 0.0 };
        let end = if can_go_forward { arrow } else { 0.0 };

        let content_node = if can_go_back || can_go_forward {
            content.as_widget_mut().layout(
                content_tree,
                renderer,
                &limits.max_width((width - start - end).max(0.0)),
            )
        } else {
            full
        };

        let arrow_limits = layout::Limits::new(Size::ZERO, Size::new(arrow, arrow));
        let back_node = back
            .as_widget_mut()
            .layout(back_tree, renderer, &arrow_limits);
        let forward_node = forward
            .as_widget_mut()
            .layout(forward_tree, renderer, &arrow_limits);

        let height = if can_go_back || can_go_forward {
            content_node.size().height.max(arrow)
        } else {
            content_node.size().height
        };

        let middle = |node: layout::Node, x: f32| {
            let y = (height - node.size().height) / 2.0;
            node.move_to(Point::new(x, y))
        };

        let back_node = middle(back_node, 0.0);
        let forward_node = middle(forward_node, width - arrow);
        let content_node = middle(content_node, start);

        layout::Node::with_children(
            Size::new(width, height),
            vec![content_node, back_node, forward_node],
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some(content), Some(content_layout)) =
            (self.children.first_mut(), layout.children().next())
        {
            content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
        }
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
        let (can_go_back, can_go_forward) = {
            let state = tree.state.downcast_ref::<ScrollerState>();
            (state.can_go_back, state.can_go_forward)
        };
        let visible = [true, can_go_back, can_go_forward];

        for (((child, child_tree), child_layout), visible) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .zip(visible)
        {
            if visible {
                child.as_widget_mut().update(
                    child_tree,
                    event,
                    child_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            }
        }

        let mut children = layout.children();
        let (Some(content_layout), Some(back_layout), Some(forward_layout)) =
            (children.next(), children.next(), children.next())
        else {
            return;
        };
        let over_back = can_go_back && cursor.is_over(back_layout.bounds());
        let over_forward = can_go_forward && cursor.is_over(forward_layout.bounds());

        let mut delta = 0.0;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
                if over_back || over_forward =>
            {
                tree.state.downcast_mut::<ScrollerState>().pressed = Some(over_forward);
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                let state = tree.state.downcast_mut::<ScrollerState>();

                if let Some(forward) = state.pressed.take() {
                    if (forward && over_forward) || (!forward && over_back) {
                        delta = if forward { self.step } else { -self.step };
                    }
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta: wheel })
                if !shell.is_event_captured() && cursor.is_over(content_layout.bounds()) =>
            {
                let vertical = match *wheel {
                    mouse::ScrollDelta::Lines { y, .. } => y * WHEEL_LINE,
                    mouse::ScrollDelta::Pixels { y, .. } => y,
                };

                if vertical != 0.0 {
                    delta = -vertical;
                    shell.capture_event();
                }
            }
            _ => {}
        }

        let reveal = matches!(
            event,
            Event::Window(window::Event::RedrawRequested(_))
                | Event::Keyboard(
                    keyboard::Event::KeyPressed { .. } | keyboard::Event::KeyReleased { .. }
                )
        );

        let Some(mut found) = self.scroll(tree, layout, renderer, delta) else {
            return;
        };

        if reveal {
            let state = tree.state.downcast_mut::<ScrollerState>();

            if found.focused != state.revealed {
                state.revealed = found.focused;

                if let Some(focused) = found.focused {
                    let delta = reveal_delta(found.viewport, found.translation.x, focused);

                    if delta != 0.0
                        && let Some(after) = self.scroll(tree, layout, renderer, delta)
                    {
                        found = after;
                    }
                }
            }
        }

        let state = tree.state.downcast_mut::<ScrollerState>();
        let offset = found.translation.x;
        let scrolled = (offset - state.offset).abs() > 0.01;
        state.offset = offset;

        let overflows = found.content.width > found.viewport.width + 0.5;
        let back = self.arrows && overflows && offset > 0.5;
        let forward =
            self.arrows && overflows && found.content.width - offset > found.viewport.width + 0.5;

        if back != state.can_go_back || forward != state.can_go_forward {
            shell.invalidate_layout();
            shell.request_redraw();
        } else if scrolled {
            shell.request_redraw();
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

        let state = tree.state.downcast_ref::<ScrollerState>();
        let mut children = layout.children();
        let (Some(content_layout), Some(back_layout), Some(forward_layout)) =
            (children.next(), children.next(), children.next())
        else {
            return;
        };

        if let Some(content) = self.children.first() {
            content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        }

        let bounds = content_layout.bounds();
        let shadow = theme.roles().shadow;

        for (visible, at_start) in [(state.can_go_back, true), (state.can_go_forward, false)] {
            if !visible {
                continue;
            }

            let (x, stops) = if at_start {
                (bounds.x, [shadow.scale_alpha(0.12), Color::TRANSPARENT])
            } else {
                (
                    bounds.x + bounds.width - UNDERSHOOT_EXTENT,
                    [Color::TRANSPARENT, shadow.scale_alpha(0.12)],
                )
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x,
                        y: bounds.y,
                        width: UNDERSHOOT_EXTENT,
                        height: bounds.height,
                    },
                    ..renderer::Quad::default()
                },
                Background::Gradient(Gradient::Linear(
                    Linear::new(FRAC_PI_2)
                        .add_stop(0.0, stops[0])
                        .add_stop(1.0, stops[1]),
                )),
            );
        }

        for (index, (visible, arrow_layout, forward)) in [
            (state.can_go_back, back_layout, false),
            (state.can_go_forward, forward_layout, true),
        ]
        .into_iter()
        .enumerate()
        {
            if !visible {
                continue;
            }

            let arrow_bounds = arrow_layout.bounds();
            let status = if state.pressed == Some(forward) {
                Status::Pressed
            } else if cursor.is_over(arrow_bounds) {
                Status::Hovered
            } else {
                Status::Active
            };
            let arrow_style = theme.style(&Class::Icon, status);

            if let Some(background) = arrow_style.background {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: arrow_bounds,
                        border: arrow_style.border,
                        ..renderer::Quad::default()
                    },
                    background,
                );
            }

            if let Some(arrow) = self.children.get(index + 1) {
                arrow.as_widget().draw(
                    &tree.children[index + 1],
                    renderer,
                    theme,
                    &renderer::Style {
                        text_color: arrow_style.text_color,
                    },
                    arrow_layout,
                    cursor,
                    viewport,
                );
            }
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
        let state = tree.state.downcast_ref::<ScrollerState>();
        let mut children = layout.children();
        let (Some(content_layout), Some(back_layout), Some(forward_layout)) =
            (children.next(), children.next(), children.next())
        else {
            return mouse::Interaction::default();
        };

        if (state.can_go_back && cursor.is_over(back_layout.bounds()))
            || (state.can_go_forward && cursor.is_over(forward_layout.bounds()))
        {
            return mouse::Interaction::Pointer;
        }

        self.children
            .first()
            .map_or_else(Default::default, |content| {
                content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hidden_target_scrolls_into_the_viewport() {
        let viewport = Rectangle::new(Point::new(10.0, 0.0), Size::new(100.0, 30.0));
        let chip = |x: f32| Rectangle::new(Point::new(x, 0.0), Size::new(40.0, 30.0));

        assert_eq!(reveal_delta(viewport, 0.0, chip(20.0)), 0.0);
        assert_eq!(reveal_delta(viewport, 0.0, chip(100.0)), 32.0);
        assert_eq!(reveal_delta(viewport, 50.0, chip(40.0)), -22.0);
    }
}
