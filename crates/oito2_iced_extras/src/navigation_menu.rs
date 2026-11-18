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

//! A website-style header menu: links and triggers in a row, a line that
//! slides under the selected item, and flyout panels below the triggers.
//!
//! An [`item`] with [`Item::on_press`] is a link; one with [`Item::panel`]
//! is a trigger that opens its panel, a
//! [popover](mod@oito2_iced::widget::popover) at most
//! [`NavigationMenu::panel_max_width`] wide, below it; an item with neither
//! is disabled. The selected item, which marks the current page,
//! takes the text-safe accent color and a line that slides under it.
//!
//! The application keeps which panel is open in a [`MenuState`] and
//! applies the [`Event`]s the menu publishes. A panel opens when its
//! trigger is pressed, or on hover after [`NavigationMenu::hover_delay`];
//! while a panel is open, hovering another trigger switches to its panel at
//! once. The panel closes shortly after the pointer leaves both the trigger
//! and the panel, on a press outside, on Escape or on a second press of its
//! trigger. Down on a focused trigger opens its panel; a panel opened by a
//! press or Down moves the focus to its first focusable element, and the
//! focus returns to the trigger when it closes with the focus inside.
//! Without [`NavigationMenu::on_event`] the triggers are disabled.
//!
//! The items keep their natural width in one row; in narrow layouts place
//! the menu in a horizontal scrollable.
//!
//! ```no_run
//! use oito2_iced_extras::navigation_menu::{self, Event, MenuState};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Menu(Event),
//!     Page(usize),
//! }
//!
//! # fn view<'a>(menu: &MenuState, page: usize) -> oito2_iced::Element<'a, Message> {
//! navigation_menu::navigation_menu(
//!     menu,
//!     [
//!         navigation_menu::item("Home").on_press(Message::Page(0)),
//!         navigation_menu::item("Docs").panel(iced::widget::text("Getting started")),
//!         navigation_menu::item("Blog").on_press(Message::Page(2)),
//!     ],
//! )
//! .selected(Some(page))
//! .on_event(Message::Menu)
//! .into()
//! # }
//! ```

use std::f32::consts::PI;
use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation, Outcome};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::widget::button::{Catalog as _, Status, Style};
use iced::widget::row;
use iced::widget::text::{Fragment, IntoFragment};
use iced::{
    Alignment, Border, Element as IcedElement, Event as IcedEvent, Length, Point, Rectangle, Size,
    Vector, mouse, window,
};
use oito2_iced::animation::{Animation, Easing, ease_out_cubic};
use oito2_iced::theme::button::Class;
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::{Minimum, min_size, popover, styled_text};
use oito2_iced::{Element, Theme};

/// The default wait before a hovered trigger opens its panel.
pub const HOVER_DELAY: Duration = Duration::from_millis(150);

/// The wait before the open panel closes once the pointer leaves its
/// trigger and the panel.
pub const LEAVE_DELAY: Duration = Duration::from_millis(250);

/// The default largest panel width.
pub const PANEL_MAX_WIDTH: f32 = 560.0;

/// The time the line under the selected item takes to slide.
pub const SLIDE_DURATION: Duration = Duration::from_millis(220);

/// The time a trigger's chevron takes to turn.
pub const CHEVRON_DURATION: Duration = Duration::from_millis(150);

/// The thickness of the line under the selected item.
pub const LINE_HEIGHT: f32 = 2.0;

/// The size of a trigger's chevron.
const CHEVRON_SIZE: f32 = 16.0;

/// The smallest width of an item, padding included.
const MIN_ITEM_WIDTH: f32 = 64.0;

/// A change of the open panel, published for the application to apply to
/// its [`MenuState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// The panel of the item at `index` opens, closing any other; with
    /// `focus`, its first focusable element takes the focus.
    Opened {
        /// The index of the trigger item.
        index: usize,
        /// Whether the focus moves into the panel (a press or Down, not a
        /// hover).
        focus: bool,
    },
    /// The open panel closes.
    Closed,
}

/// Which panel of a navigation menu is open, kept by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MenuState {
    open: Option<usize>,
    focus: bool,
}

impl MenuState {
    /// The index of the item whose panel is open.
    #[must_use]
    pub fn open(&self) -> Option<usize> {
        self.open
    }

    /// Applies an event published by the menu.
    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Opened { index, focus } => {
                self.open = Some(index);
                self.focus = focus;
            }
            Event::Closed => {
                self.open = None;
                self.focus = false;
            }
        }
    }
}

/// What an item does.
enum Kind<'a, Message> {
    Disabled,
    Link(Message),
    Panel(Element<'a, Message>),
}

/// An entry of a [`NavigationMenu`]. See [`item`].
pub struct Item<'a, Message> {
    label: Fragment<'a>,
    kind: Kind<'a, Message>,
}

/// Creates an item showing `label`; without [`Item::on_press`] or
/// [`Item::panel`] it is disabled.
pub fn item<'a, Message>(label: impl IntoFragment<'a>) -> Item<'a, Message> {
    Item {
        label: label.into_fragment(),
        kind: Kind::Disabled,
    }
}

impl<'a, Message> Item<'a, Message> {
    /// Makes the item a link that publishes `message` when pressed,
    /// replacing any panel.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.kind = Kind::Link(message);
        self
    }

    /// Makes the item a trigger that opens `content` in a panel below it,
    /// replacing any press message.
    #[must_use]
    pub fn panel(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.kind = Kind::Panel(content.into());
        self
    }
}

/// A navigation menu. See [`navigation_menu`].
pub struct NavigationMenu<'a, Message> {
    state: MenuState,
    items: Vec<Item<'a, Message>>,
    selected: Option<usize>,
    on_event: Option<Box<dyn Fn(Event) -> Message + 'a>>,
    hover_delay: Duration,
    panel_max_width: f32,
    direction: ReadingDirection,
    animated: bool,
}

/// Creates a navigation menu of `items`, in order, with the open panel of
/// `state`.
pub fn navigation_menu<'a, Message>(
    state: &MenuState,
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> NavigationMenu<'a, Message> {
    NavigationMenu {
        state: *state,
        items: items.into_iter().collect(),
        selected: None,
        on_event: None,
        hover_delay: HOVER_DELAY,
        panel_max_width: PANEL_MAX_WIDTH,
        direction: ReadingDirection::LeftToRight,
        animated: true,
    }
}

impl<'a, Message: Clone + 'a> NavigationMenu<'a, Message> {
    /// Sets the index of the item that marks the current page, or `None`
    /// for none.
    #[must_use]
    pub fn selected(mut self, index: Option<usize>) -> Self {
        self.selected = index;
        self
    }

    /// Sets the message that publishes the panel changes. Without it the
    /// triggers are disabled.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(Event) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(on_event));
        self
    }

    /// Sets the wait before a hovered trigger opens its panel;
    /// [`HOVER_DELAY`] by default.
    #[must_use]
    pub fn hover_delay(mut self, delay: Duration) -> Self {
        self.hover_delay = delay;
        self
    }

    /// Sets the largest panel width; [`PANEL_MAX_WIDTH`] by default.
    #[must_use]
    pub fn panel_max_width(mut self, width: f32) -> Self {
        self.panel_max_width = width;
        self
    }

    /// Sets the reading direction: right to left places the first item on
    /// the right, the chevrons before the labels and the panels at the
    /// right edge of their triggers.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the line and the chevrons animate (on by default).
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Builds the button of the item at `index`, inside its popover when it
    /// has a panel.
    fn trigger(&self, index: usize, item: Item<'a, Message>) -> (Element<'a, Message>, bool) {
        let selected = self.selected == Some(index);
        let open = self.state.open == Some(index);
        let label = styled_text(item.label, typography::LABEL_LARGE);
        let has_panel = matches!(item.kind, Kind::Panel(_));

        let content: Element<'a, Message> = if has_panel {
            let chevron = IcedElement::new(Chevron {
                open,
                animated: self.animated,
            });
            let children: [Element<'a, Message>; 2] = match self.direction {
                ReadingDirection::LeftToRight => [label.into(), chevron],
                ReadingDirection::RightToLeft => [chevron, label.into()],
            };
            row(children)
                .spacing(spacing::XS)
                .align_y(Alignment::Center)
                .into()
        } else {
            label.into()
        };
        let content = min_size(
            content,
            Minimum::Fixed(MIN_ITEM_WIDTH - 2.0 * spacing::MD),
            Minimum::None,
        );
        let button = button::button(content)
            .class(Class::Custom(Box::new(move |theme: &Theme, status| {
                trigger_style(theme, status, selected)
            })))
            .padding([0.0, spacing::MD]);

        match item.kind {
            Kind::Disabled => (button.into(), false),
            Kind::Link(message) => (button.on_press(message).into(), false),
            Kind::Panel(panel) => {
                let event = |event| self.on_event.as_ref().map(|on_event| on_event(event));
                let toggle = event(if open {
                    Event::Closed
                } else {
                    Event::Opened { index, focus: true }
                });
                let enabled = toggle.is_some();
                let mut popover = popover(button.on_press_maybe(toggle), panel)
                    .open(open)
                    .focus_on_open(self.state.focus)
                    .max_width(self.panel_max_width)
                    .padding(spacing::LG)
                    .direction(self.direction);

                if let Some(close) = event(Event::Closed) {
                    popover = popover.on_dismiss(close);
                }

                (popover.into(), enabled)
            }
        }
    }
}

/// The colors of a trigger: the accent when `selected`, the surface text
/// otherwise, transparent with a state layer and the medium radius.
fn trigger_style(theme: &Theme, status: Status, selected: bool) -> Style {
    let class = if selected { Class::Text } else { Class::Chrome };
    let style = theme.style(&class, status);

    Style {
        border: Border::default().rounded(radius::MEDIUM),
        ..style
    }
}

impl<'a, Message: Clone + 'a> From<NavigationMenu<'a, Message>> for Element<'a, Message> {
    fn from(mut menu: NavigationMenu<'a, Message>) -> Self {
        let items = std::mem::take(&mut menu.items);
        let (children, triggers) = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| menu.trigger(index, item))
            .unzip();

        IcedElement::new(Menu {
            children,
            triggers,
            selected: menu.selected,
            open: menu.state.open,
            on_event: menu.on_event,
            hover_delay: menu.hover_delay,
            direction: menu.direction,
            animated: menu.animated,
        })
    }
}

/// The row of items, the line under the selected one and the hover timing.
struct Menu<'a, Message> {
    children: Vec<Element<'a, Message>>,
    /// Whether each item is an enabled panel trigger.
    triggers: Vec<bool>,
    selected: Option<usize>,
    open: Option<usize>,
    on_event: Option<Box<dyn Fn(Event) -> Message + 'a>>,
    hover_delay: Duration,
    direction: ReadingDirection,
    animated: bool,
}

/// Where the pointer is, for the hover timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Region {
    #[default]
    Outside,
    Trigger(usize),
    Panel,
}

/// A panel change that waits for its delay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pending {
    Open(usize),
    Close,
}

/// A pending change and the first redraw after it was scheduled.
#[derive(Debug, Clone, Copy)]
struct Timer {
    pending: Pending,
    delay: Duration,
    started: Option<Instant>,
}

/// The line moving from `from` to the current target.
#[derive(Debug, Clone, Copy)]
struct Slide {
    from: Rectangle,
    started: Option<Instant>,
}

/// The tree state of a [`Menu`].
#[derive(Debug, Default)]
struct State {
    region: Region,
    /// Whether the pointer is over an open panel, written by the overlay.
    over_panel: bool,
    timer: Option<Timer>,
    /// The line under the selected item, relative to the menu.
    line: Option<Rectangle>,
    slide: Option<Slide>,
    now: Option<Instant>,
}

impl State {
    /// The line as drawn now: between the start of a slide and the target.
    fn shown_line(&self) -> Option<Rectangle> {
        let line = self.line?;
        let Some(slide) = self.slide else {
            return Some(line);
        };
        let (Some(started), Some(now)) = (slide.started, self.now) else {
            return Some(slide.from);
        };
        let t = (now.saturating_duration_since(started).as_secs_f32()
            / SLIDE_DURATION.as_secs_f32())
        .clamp(0.0, 1.0);
        let t = ease_out_cubic(t);
        let mix = |a: f32, b: f32| a + (b - a) * t;

        Some(Rectangle {
            x: mix(slide.from.x, line.x),
            y: mix(slide.from.y, line.y),
            width: mix(slide.from.width, line.width),
            height: mix(slide.from.height, line.height),
        })
    }

    fn sliding(&self) -> bool {
        match (self.slide, self.now) {
            (Some(Slide { started: None, .. }), _) => true,
            (
                Some(Slide {
                    started: Some(started),
                    ..
                }),
                Some(now),
            ) => now.saturating_duration_since(started) < SLIDE_DURATION,
            _ => false,
        }
    }
}

impl<Message> Menu<'_, Message> {
    fn publish(&self, shell: &mut Shell<'_, Message>, event: Event) {
        if let Some(on_event) = &self.on_event {
            shell.publish(on_event(event));
        }
    }

    /// Schedules `pending` after `delay`, replacing any pending change.
    fn schedule(
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        pending: Pending,
        delay: Duration,
    ) {
        state.timer = Some(Timer {
            pending,
            delay,
            started: None,
        });
        shell.request_redraw();
    }

    /// Follows the pointer into `region`: opens or switches panels and
    /// schedules the delayed changes.
    fn enter(&self, state: &mut State, shell: &mut Shell<'_, Message>, region: Region) {
        if region == state.region {
            return;
        }

        let left = state.region;
        state.region = region;

        match region {
            Region::Trigger(index) => {
                state.timer = None;
                match self.open {
                    Some(open) if open != index => {
                        self.publish(
                            shell,
                            Event::Opened {
                                index,
                                focus: false,
                            },
                        );
                    }
                    Some(_) => {}
                    None => Self::schedule(state, shell, Pending::Open(index), self.hover_delay),
                }
            }
            Region::Panel => state.timer = None,
            Region::Outside if left != Region::Outside => {
                Self::schedule(state, shell, Pending::Close, LEAVE_DELAY);
            }
            Region::Outside => {}
        }
    }

    /// Runs the pending change once its delay has passed at `now`.
    fn tick(&self, state: &mut State, shell: &mut Shell<'_, Message>, now: Instant) {
        let Some(timer) = state.timer.as_mut() else {
            return;
        };
        let started = *timer.started.get_or_insert(now);

        if now.saturating_duration_since(started) < timer.delay {
            shell.request_redraw();
            return;
        }

        let pending = timer.pending;
        state.timer = None;

        match pending {
            Pending::Open(index) if self.open != Some(index) => {
                self.publish(
                    shell,
                    Event::Opened {
                        index,
                        focus: false,
                    },
                );
            }
            Pending::Close if self.open.is_some() => self.publish(shell, Event::Closed),
            _ => {}
        }
    }

    /// The panel trigger under `cursor`.
    fn hovered_trigger(&self, layout: Layout<'_>, cursor: mouse::Cursor) -> Option<usize> {
        layout
            .children()
            .zip(&self.triggers)
            .position(|(child, &trigger)| trigger && cursor.is_over(child.bounds()))
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Menu<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let child_limits =
            layout::Limits::new(Size::ZERO, Size::new(f32::INFINITY, limits.max().height));
        let mut nodes: Vec<layout::Node> = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(child, child_tree)| {
                child
                    .as_widget_mut()
                    .layout(child_tree, renderer, &child_limits)
            })
            .collect();

        let gaps = spacing::XS * nodes.len().saturating_sub(1) as f32;
        let width = nodes.iter().map(|node| node.size().width).sum::<f32>() + gaps;
        let height = nodes
            .iter()
            .map(|node| node.size().height)
            .fold(0.0, f32::max);

        let mut x = 0.0;
        for node in &mut nodes {
            let size = node.size();
            let left = match self.direction {
                ReadingDirection::LeftToRight => x,
                ReadingDirection::RightToLeft => width - x - size.width,
            };
            node.move_to_mut(Point::new(left, (height - size.height) / 2.0));
            x += size.width + spacing::XS;
        }

        let state = tree.state.downcast_mut::<State>();
        let target = self
            .selected
            .and_then(|index| nodes.get(index))
            .map(|node| {
                let bounds = node.bounds();
                Rectangle {
                    x: bounds.x + spacing::SM,
                    y: bounds.y + bounds.height - LINE_HEIGHT,
                    width: (bounds.width - spacing::SM * 2.0).max(0.0),
                    height: LINE_HEIGHT,
                }
            });

        if target != state.line {
            state.slide = match (state.shown_line(), target) {
                (Some(from), Some(_)) if self.animated => Some(Slide {
                    from,
                    started: None,
                }),
                _ => None,
            };
            state.line = target;
        }

        layout::Node::with_children(
            limits.resolve(Length::Shrink, Length::Shrink, Size::new(width, height)),
            nodes,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.traverse(&mut |operation| {
            for ((child, child_tree), child_layout) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(child_tree, child_layout, renderer, operation);
            }
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &IcedEvent,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        for ((child, child_tree), child_layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
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

        if let IcedEvent::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowDown),
            ..
        }) = event
            && !shell.is_event_captured()
        {
            let focused = self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .zip(&self.triggers)
                .position(|(((child, child_tree), child_layout), &trigger)| {
                    trigger && has_focus(child, child_tree, child_layout, renderer)
                });

            if let Some(index) = focused {
                if self.open != Some(index) {
                    self.publish(shell, Event::Opened { index, focus: true });
                }
                shell.capture_event();
            }
        }

        let state = tree.state.downcast_mut::<State>();

        match event {
            IcedEvent::Mouse(mouse::Event::CursorMoved { .. } | mouse::Event::CursorLeft) => {
                let region = if self.open.is_some() && state.over_panel {
                    Region::Panel
                } else {
                    self.hovered_trigger(layout, cursor)
                        .map_or(Region::Outside, Region::Trigger)
                };
                self.enter(state, shell, region);
            }
            IcedEvent::Mouse(mouse::Event::ButtonPressed(_))
                if self.hovered_trigger(layout, cursor).is_some() =>
            {
                state.timer = None;
            }
            IcedEvent::Window(window::Event::RedrawRequested(now)) => {
                state.now = Some(*now);
                if let Some(slide) = state.slide.as_mut() {
                    slide.started.get_or_insert(*now);
                }
                if state.sliding() {
                    shell.request_redraw();
                } else {
                    state.slide = None;
                }
                self.tick(state, shell, *now);
            }
            _ => {}
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
        for ((child, child_tree), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                child_tree,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }

        let state = tree.state.downcast_ref::<State>();
        let line = if self.animated {
            state.shown_line()
        } else {
            state.line
        };

        if let Some(line) = line {
            let origin = layout.position();
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: origin.x + line.x,
                        y: origin.y + line.y,
                        ..line
                    },
                    border: Border::default().rounded(LINE_HEIGHT / 2.0),
                    ..renderer::Quad::default()
                },
                theme.roles().semantic.primary_text,
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
        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, child_tree), child_layout)| {
                child.as_widget().mouse_interaction(
                    child_tree,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        let overlays: Vec<_> = self
            .children
            .iter_mut()
            .zip(children)
            .zip(layout.children())
            .filter_map(|((child, child_tree), child_layout)| {
                child.as_widget_mut().overlay(
                    child_tree,
                    child_layout,
                    renderer,
                    viewport,
                    translation,
                )
            })
            .collect();

        if overlays.is_empty() {
            state.over_panel = false;
            return None;
        }

        Some(overlay::Element::new(Box::new(PanelWatch {
            inner: overlay::Group::with_children(overlays).overlay(),
            state,
        })))
    }
}

/// Whether a focusable element inside `element` has the focus.
fn has_focus<Message>(
    element: &mut Element<'_, Message>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> bool {
    let mut operation: Box<dyn Operation<bool>> =
        Box::new(oito2_iced::focus::operations::has_focus());

    loop {
        element.as_widget_mut().operate(
            tree,
            layout,
            renderer,
            &mut operation::black_box(operation.as_mut()),
        );

        match operation.finish() {
            Outcome::Chain(next) => operation = next,
            Outcome::Some(found) => return found,
            Outcome::None => return false,
        }
    }
}

/// The open panels of a [`Menu`]: records whether the pointer is over one.
struct PanelWatch<'b, Message> {
    inner: overlay::Element<'b, Message, Theme, iced::Renderer>,
    state: &'b mut State,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for PanelWatch<'_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        self.inner.as_overlay_mut().layout(renderer, bounds)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.inner
            .as_overlay()
            .draw(renderer, theme, style, layout, cursor);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.inner
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &IcedEvent,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        if let IcedEvent::Mouse(_) = event {
            self.state.over_panel = self
                .inner
                .as_overlay()
                .mouse_interaction(layout, cursor, renderer)
                != mouse::Interaction::None;
        }

        self.inner
            .as_overlay_mut()
            .update(event, layout, cursor, renderer, clipboard, shell);
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.inner
            .as_overlay()
            .mouse_interaction(layout, cursor, renderer)
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
        self.inner.as_overlay_mut().overlay(layout, renderer)
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}

/// A chevron pointing down that turns to point up while the panel is open,
/// in the text color of the button around it.
struct Chevron {
    open: bool,
    animated: bool,
}

/// The tree state of a [`Chevron`].
#[derive(Debug)]
struct ChevronState {
    turn: Animation<bool>,
    now: Option<Instant>,
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Chevron
where
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ChevronState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ChevronState {
            turn: Animation::new(self.open)
                .duration(CHEVRON_DURATION)
                .easing(Easing::Linear),
            now: None,
        })
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(CHEVRON_SIZE), Length::Fixed(CHEVRON_SIZE))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, CHEVRON_SIZE, CHEVRON_SIZE)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &IcedEvent,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<ChevronState>();
        let redraw_at = match event {
            IcedEvent::Window(window::Event::RedrawRequested(now)) => Some(*now),
            _ => None,
        };

        if state.turn.value() != self.open {
            let at = redraw_at.unwrap_or_else(Instant::now);
            state.turn.go_mut(self.open, at);
            state.now = Some(at);
            shell.request_redraw();
        }

        if let Some(now) = redraw_at {
            state.now = Some(now);

            if state.turn.is_animating(now) {
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<ChevronState>();
        let turn = match state.now {
            Some(now) if self.animated && state.turn.value() == self.open => {
                state.turn.interpolate(0.0, 1.0, now)
            }
            _ => f32::from(u8::from(self.open)),
        };

        let bounds = layout.bounds();
        let unit = bounds.width / 24.0;
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let path = Path::new(|builder| {
            builder.move_to(Point::new(6.0 * unit, 9.0 * unit));
            builder.line_to(Point::new(12.0 * unit, 15.0 * unit));
            builder.line_to(Point::new(18.0 * unit, 9.0 * unit));
        });

        frame.with_save(|frame| {
            frame.translate(Vector::new(center.x, center.y));
            frame.rotate(turn * PI);
            frame.translate(Vector::new(-center.x, -center.y));
            frame.stroke(
                &path,
                Stroke::default()
                    .with_color(style.text_color)
                    .with_width(2.0 * unit)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(LineJoin::Round),
            );
        });

        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_state_follows_the_events() {
        let mut state = MenuState::default();
        state.apply(Event::Opened {
            index: 1,
            focus: true,
        });
        assert_eq!(state.open(), Some(1));
        assert!(state.focus);
        state.apply(Event::Opened {
            index: 2,
            focus: false,
        });
        assert_eq!(state.open(), Some(2));
        assert!(!state.focus);
        state.apply(Event::Closed);
        assert_eq!(state, MenuState::default());
    }
}
