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

//! A guided tour that spotlights targets one step at a time.
//!
//! A [`Tour`] is a plain value the application owns: its [`Step`]s and the
//! index of the current one. The application starts it with
//! [`Tour::start`] and applies the [`Event`]s the tour publishes with
//! [`Tour::apply`], which reports when the tour ends and whether it was
//! completed or skipped.
//!
//! [`host`] wraps the root of the view. While the tour runs, it dims
//! everything but a rounded spotlight around the target of the current
//! step (an element with an [`Id`], such as a
//! [`container`](iced::widget::container())), scrolls the innermost
//! scrollable that holds the target so the target sits in its middle, and
//! shows a card next to the target with the step title, its description,
//! its position, and back, next and skip buttons; the card is centered when
//! the step has no target or the target isn't in the view. The spotlight
//! moves from one target to the next over [`MOVE_DURATION`].
//!
//! Pointer input outside the spotlight is blocked; input inside it reaches
//! the target, and a release inside the target of a [`Step::wait_for_tap`]
//! step publishes [`Event::Next`] after the target has received it. The
//! card takes the focus on each step; while it has the focus, Escape skips
//! the tour, the arrow keys move between steps and Tab cycles inside it.
//! When the tour ends, the focus returns to the widget that had it before.
//!
//! ```no_run
//! use iced::widget::{button, column, container};
//! use oito2_iced::widget::tour::{self, Step, Tour};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Start,
//!     Tour(tour::Event),
//! }
//!
//! fn new_tour() -> Tour {
//!     Tour::new(vec![
//!         Step::new("Welcome").description("This short tour shows the toolbar."),
//!         Step::new("Search")
//!             .description("Find any file by name.")
//!             .target("search"),
//!     ])
//! }
//!
//! fn update(tour: &mut Tour, message: Message) {
//!     match message {
//!         Message::Start => tour.start(),
//!         Message::Tour(event) => {
//!             if let Some(completed) = tour.apply(event) {
//!                 println!("finished, completed: {completed}");
//!             }
//!         }
//!     }
//! }
//!
//! fn view(tour: &Tour) -> oito2_iced::Element<'_, Message> {
//!     tour::host(
//!         column![
//!             container("Search files").id("search"),
//!             button("Start the tour").on_press(Message::Start),
//!         ],
//!         tour,
//!     )
//!     .on_event(Message::Tour)
//!     .into()
//! }
//! ```

use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, Path, fill};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::focusable::{Count, count};
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::operation::{self, Focusable, Operation, TextInput};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::widget::text::Alignment as TextAlignment;
use iced::widget::{Column, Row, Space, container, text};
use iced::{
    Alignment, Element as IcedElement, Event as IcedEvent, Length, Padding, Point, Rectangle, Size,
    Vector, mouse, touch, window,
};

use super::button;
use super::typography::styled_text;
use crate::animation::ease_out_cubic;
use crate::focus::operations::{focus_first, focus_last, has_focus};
use crate::l10n::Localizations;
use crate::overlay::anchored::{self, Geometry, Placement, Side, place};
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{radius, spacing, typography};
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// How long the spotlight takes to move from one target to the next.
pub const MOVE_DURATION: Duration = Duration::from_millis(250);

/// The default space around the target inside the spotlight.
pub const SPOTLIGHT_PADDING: f32 = 6.0;

/// The largest width of the step card.
pub const CARD_MAX_WIDTH: f32 = 320.0;

/// The opacity of the scrim that dims the view around the spotlight.
pub const SCRIM_OPACITY: f32 = 0.5;

/// The space between the target and the tip of the card arrow.
const CARD_GAP: f32 = 2.0;

/// The base width and the height of the card arrow.
const ARROW: Size = Size::new(14.0, 7.0);

/// The size of the skip icon.
#[cfg(feature = "svg")]
const SKIP_ICON_SIZE: f32 = 16.0;

/// One step of a [`Tour`].
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    title: String,
    description: Option<String>,
    target: Option<Id>,
    placement: Placement,
    wait_for_tap: bool,
    spotlight_padding: f32,
}

impl Step {
    /// A step with `title` as the heading of its card, without a target:
    /// its card shows in the middle of the view.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            target: None,
            placement: Placement::Bottom,
            wait_for_tap: false,
            spotlight_padding: SPOTLIGHT_PADDING,
        }
    }

    /// Sets the body text of the card, below the title.
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the [`Id`] of the element the step points at. The card is
    /// centered while no element of the view has this id.
    #[must_use]
    pub fn target(mut self, target: impl Into<Id>) -> Self {
        self.target = Some(target.into());
        self
    }

    /// Sets the side of the target where the card opens (below by
    /// default).
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Makes the step wait for a tap on the target instead of offering a
    /// next button: the tap reaches the target, then the tour moves on.
    /// While the target isn't in the view, the next button shows instead.
    #[must_use]
    pub fn wait_for_tap(mut self, wait: bool) -> Self {
        self.wait_for_tap = wait;
        self
    }

    /// Sets the space around the target inside the spotlight
    /// ([`SPOTLIGHT_PADDING`] by default).
    #[must_use]
    pub fn spotlight_padding(mut self, padding: f32) -> Self {
        self.spotlight_padding = padding;
        self
    }

    /// The heading of the card.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// The body text of the card, if any.
    pub fn description_text(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// The [`Id`] of the target, if any.
    pub fn target_id(&self) -> Option<&Id> {
        self.target.as_ref()
    }

    /// The side of the target where the card opens.
    pub fn placement_of(&self) -> Placement {
        self.placement
    }

    /// Whether the step waits for a tap on its target.
    pub fn waits_for_tap(&self) -> bool {
        self.wait_for_tap
    }

    /// The space around the target inside the spotlight.
    pub fn padding(&self) -> f32 {
        self.spotlight_padding
    }
}

/// A change the tour asks the application to apply with [`Tour::apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// Move to the next step, or complete the tour after the last one.
    Next,
    /// Move to the previous step.
    Previous,
    /// End the tour without completing it.
    Skip,
}

/// The steps of a guided tour and the current position, owned by the
/// application. See the [module documentation](self).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Tour {
    steps: Vec<Step>,
    index: Option<usize>,
    completed: bool,
}

impl Tour {
    /// A tour through `steps`, in order, that isn't running yet.
    pub fn new(steps: Vec<Step>) -> Self {
        Self {
            steps,
            index: None,
            completed: false,
        }
    }

    /// The steps, in order.
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    /// The index of the current step, or `None` when the tour isn't
    /// running.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// The current step, or `None` when the tour isn't running.
    pub fn step(&self) -> Option<&Step> {
        self.index.and_then(|index| self.steps.get(index))
    }

    /// Whether the tour is running.
    pub fn is_active(&self) -> bool {
        self.index.is_some()
    }

    /// Whether the last run reached the end, rather than being skipped;
    /// false while a run is in progress.
    pub fn completed(&self) -> bool {
        self.completed
    }

    /// Starts the tour, or restarts it, at the first step; does nothing
    /// without steps.
    pub fn start(&mut self) {
        self.go_to(0);
    }

    /// Shows the step at `index`, starting the tour when it isn't running;
    /// does nothing when `index` is out of range.
    pub fn go_to(&mut self, index: usize) {
        if index < self.steps.len() {
            self.index = Some(index);
            self.completed = false;
        }
    }

    /// Moves to the next step, or completes the tour after the last one;
    /// does nothing when the tour isn't running. Returns `Some(true)` when
    /// the tour was completed.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<bool> {
        let index = self.index?;

        if index + 1 < self.steps.len() {
            self.go_to(index + 1);
            None
        } else {
            self.finish(true)
        }
    }

    /// Moves to the previous step; does nothing on the first step or when
    /// the tour isn't running.
    pub fn previous(&mut self) {
        if let Some(index) = self.index
            && index > 0
        {
            self.go_to(index - 1);
        }
    }

    /// Ends the tour without completing it; does nothing when it isn't
    /// running. Returns `Some(false)` when the tour was running.
    pub fn skip(&mut self) -> Option<bool> {
        self.finish(false)
    }

    /// Applies `event`; returns `Some(completed)` when it ended the tour,
    /// with true when the tour was completed and false when it was skipped.
    pub fn apply(&mut self, event: Event) -> Option<bool> {
        match event {
            Event::Next => self.next(),
            Event::Previous => {
                self.previous();
                None
            }
            Event::Skip => self.skip(),
        }
    }

    /// Stops the running tour, recording whether it was `completed`.
    fn finish(&mut self, completed: bool) -> Option<bool> {
        self.index.take()?;
        self.completed = completed;

        Some(completed)
    }
}

/// The tour layer over the root of a view. See [`host`].
pub struct Host<'a, Message> {
    content: Element<'a, Message>,
    tour: &'a Tour,
    on_event: Option<Box<dyn Fn(Event) -> Message + 'a>>,
    direction: ReadingDirection,
    animated: bool,
    localizations: Localizations,
}

/// Wraps `content`, the root of the view, with the layer that runs `tour`.
/// See the [module documentation](self).
pub fn host<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    tour: &'a Tour,
) -> Host<'a, Message> {
    Host {
        content: content.into(),
        tour,
        on_event: None,
        direction: ReadingDirection::LeftToRight,
        animated: true,
        localizations: Localizations::en(),
    }
}

impl<'a, Message> Host<'a, Message> {
    /// Sets the function that turns the tour [`Event`]s into messages.
    /// Without it the card buttons are disabled and the keys do nothing.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(Event) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(on_event));
        self
    }

    /// Sets the reading direction: it maps the start and end placements to
    /// sides, mirrors the card and swaps the arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the spotlight moves between targets (true by default);
    /// false moves it at once, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the strings of the buttons and of the step position.
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }
}

/// The messages of the card buttons.
struct Messages<Message> {
    next: Option<Message>,
    previous: Option<Message>,
    skip: Option<Message>,
}

/// The step the layer shows.
struct Active<'a, Message> {
    index: usize,
    target: Option<Id>,
    placement: Placement,
    wait_for_tap: bool,
    padding: f32,
    /// The card with the next button, then, for a step that waits for a
    /// tap, the card without it.
    cards: Vec<Element<'a, Message>>,
    messages: Messages<Message>,
}

/// The widget behind [`Host`].
struct Layer<'a, Message> {
    content: Element<'a, Message>,
    active: Option<Active<'a, Message>>,
    direction: ReadingDirection,
    animated: bool,
}

impl<'a, Message: Clone + 'a> From<Host<'a, Message>> for Element<'a, Message> {
    fn from(host: Host<'a, Message>) -> Self {
        let Host {
            content,
            tour,
            on_event,
            direction,
            animated,
            localizations,
        } = host;

        let active = tour.index.and_then(|index| {
            let step = tour.steps.get(index)?;
            let message = |event: Event| on_event.as_ref().map(|on_event| on_event(event));
            let messages = Messages {
                next: message(Event::Next),
                previous: message(Event::Previous),
                skip: message(Event::Skip),
            };
            let card = |next: bool| {
                card(
                    step,
                    index,
                    tour.steps.len(),
                    next,
                    &messages,
                    &localizations,
                    direction,
                )
            };
            let mut cards = vec![card(true)];

            if step.wait_for_tap {
                cards.push(card(false));
            }

            Some(Active {
                index,
                target: step.target.clone(),
                placement: step.placement,
                wait_for_tap: step.wait_for_tap,
                padding: step.spotlight_padding,
                cards,
                messages,
            })
        });

        IcedElement::new(Layer {
            content,
            active,
            direction,
            animated,
        })
    }
}

/// Builds the content of the card of `step`, the step at `index` of
/// `total`, with the next button when `next` is true.
fn card<'a, Message: Clone + 'a>(
    step: &'a Step,
    index: usize,
    total: usize,
    next: bool,
    messages: &Messages<Message>,
    localizations: &Localizations,
    direction: ReadingDirection,
) -> Element<'a, Message> {
    let rtl = direction == ReadingDirection::RightToLeft;
    let align = if rtl {
        TextAlignment::Right
    } else {
        TextAlignment::Left
    };
    let mirrored = |mut children: Vec<Element<'a, Message>>| {
        if rtl {
            children.reverse();
        }
        children
    };

    let title = styled_text(step.title.as_str(), typography::TITLE_MEDIUM)
        .width(Length::Fill)
        .align_x(align);
    let header = Row::with_children(mirrored(vec![
        title.into(),
        skip_button(messages.skip.clone(), localizations),
    ]))
    .align_y(Alignment::Center);

    let mut column = Column::new().spacing(spacing::XS).push(header);

    if let Some(description) = &step.description {
        let end = Padding {
            left: if rtl { spacing::SM } else { 0.0 },
            right: if rtl { 0.0 } else { spacing::SM },
            ..Padding::ZERO
        };

        column = column.push(
            container(
                styled_text(description.as_str(), typography::BODY_MEDIUM)
                    .width(Length::Fill)
                    .align_x(align),
            )
            .padding(end),
        );
    }

    let progress = styled_text(
        localizations.tour_progress(index as i64 + 1, total as i64),
        typography::BODY_SMALL,
    )
    .width(Length::Fill)
    .align_x(align)
    .style(|theme: &Theme| text::Style {
        color: Some(theme.roles().on_surface_variant),
    });
    let mut footer: Vec<Element<'a, Message>> = vec![progress.into()];

    if index > 0 {
        footer.push(
            button::text(localizations.tour_back)
                .on_press_maybe(messages.previous.clone())
                .into(),
        );
    }
    if next {
        let label = if index + 1 == total {
            localizations.tour_done
        } else {
            localizations.tour_next
        };

        footer.push(
            button::filled(label)
                .on_press_maybe(messages.next.clone())
                .into(),
        );
    }

    column = column.push(Space::new().height(spacing::XS)).push(
        Row::with_children(mirrored(footer))
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
    );

    let padding = Padding {
        top: spacing::SM,
        right: if rtl { spacing::LG } else { spacing::SM },
        bottom: spacing::MD,
        left: if rtl { spacing::SM } else { spacing::LG },
    };

    container(column)
        .padding(padding)
        .width(Length::Fill)
        .into()
}

/// The skip button of the card header: an "x" icon button with the skip
/// tooltip, or a text button without icons.
fn skip_button<'a, Message: Clone + 'a>(
    message: Option<Message>,
    localizations: &Localizations,
) -> Element<'a, Message> {
    #[cfg(feature = "svg")]
    {
        button::button(crate::icon::icon(crate::icon::lucide::X).size(SKIP_ICON_SIZE))
            .class(button::Class::Icon)
            .padding(0)
            .width(crate::density::control_height())
            .tooltip(localizations.tour_skip)
            .on_press_maybe(message)
            .into()
    }
    #[cfg(not(feature = "svg"))]
    {
        button::text(localizations.tour_skip)
            .on_press_maybe(message)
            .into()
    }
}

/// A running move of the spotlight.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Move {
    /// The spotlight when the move began, relative to the layer.
    from: Rectangle,
    /// The first frame of the move; `None` until a frame arrives.
    started: Option<Instant>,
}

/// The tree state of the layer.
#[derive(Debug, Clone, Default)]
struct State {
    /// The index of the step shown.
    shown: Option<usize>,
    /// The spotlight of the current target, relative to the layer.
    target: Option<Rectangle>,
    /// The card in use: 0 with the next button, 1 without it.
    card: usize,
    moving: Option<Move>,
    now: Option<Instant>,
    /// Whether the focus moves into the card at the next event.
    focus_pending: bool,
    /// Whether the content focus is recorded before it is cleared.
    starting: bool,
    /// Whether the target scrolls into view at the next event.
    scroll_pending: bool,
    /// The index of the content widget that had the focus before the tour.
    saved_focus: Option<usize>,
    /// The index of the content widget that gets the focus back at the
    /// next event.
    restore: Option<usize>,
}

impl State {
    /// The linear progress of the spotlight move, from 0 to 1.
    fn progress(&self) -> f32 {
        match self.moving {
            Some(Move {
                started: Some(started),
                ..
            }) => match self.now {
                Some(now) => (now.saturating_duration_since(started).as_secs_f32()
                    / MOVE_DURATION.as_secs_f32())
                .clamp(0.0, 1.0),
                None => 0.0,
            },
            Some(_) => 0.0,
            None => 1.0,
        }
    }

    /// The spotlight drawn now, relative to the layer: between the
    /// spotlight the move began from and the current target.
    fn displayed(&self) -> Option<Rectangle> {
        let target = self.target?;

        match self.moving {
            Some(Move { from, .. }) => Some(lerp(from, target, ease_out_cubic(self.progress()))),
            None => Some(target),
        }
    }
}

/// The rectangle at `t` of the way from `from` to `to`.
fn lerp(from: Rectangle, to: Rectangle, t: f32) -> Rectangle {
    let mix = |a: f32, b: f32| a + (b - a) * t;

    Rectangle {
        x: mix(from.x, to.x),
        y: mix(from.y, to.y),
        width: mix(from.width, to.width),
        height: mix(from.height, to.height),
    }
}

/// Whether `point` lies inside `rectangle` with corners rounded by
/// `radius`.
fn rounded_contains(rectangle: Rectangle, radius: f32, point: Point) -> bool {
    if !rectangle.contains(point) {
        return false;
    }

    let radius = radius
        .min(rectangle.width / 2.0)
        .min(rectangle.height / 2.0)
        .max(0.0);
    let left = rectangle.x + radius;
    let right = rectangle.x + rectangle.width - radius;
    let top = rectangle.y + radius;
    let bottom = rectangle.y + rectangle.height - radius;
    let dx = if point.x < left {
        left - point.x
    } else if point.x > right {
        point.x - right
    } else {
        0.0
    };
    let dy = if point.y < top {
        top - point.y
    } else if point.y > bottom {
        point.y - bottom
    } else {
        0.0
    };

    dx * dx + dy * dy <= radius * radius
}

/// A scrollable met while looking for the target.
#[derive(Debug, Clone, Copy, PartialEq)]
struct ScrollFrame {
    /// The position of the scrollable among the scrollables of the
    /// content, in traversal order.
    index: usize,
    bounds: Rectangle,
    content: Rectangle,
    translation: Vector,
}

/// Where the target was found.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Found {
    /// The bounds of the target on screen.
    bounds: Rectangle,
    /// The innermost scrollable that holds the target, with the bounds of
    /// the target in its content.
    scroll: Option<(ScrollFrame, Rectangle)>,
}

/// An operation that finds the widget with the `target` id, its bounds on
/// screen after the scroll translations of its scrollables, and the
/// innermost scrollable that holds it.
struct Locate<'a> {
    target: &'a Id,
    /// The scrollable reported last, whose children come next.
    pending: Option<ScrollFrame>,
    /// The scrollables that hold the widgets being visited.
    stack: Vec<ScrollFrame>,
    scrollables: usize,
    found: Option<Found>,
}

impl<'a> Locate<'a> {
    fn new(target: &'a Id) -> Self {
        Self {
            target,
            pending: None,
            stack: Vec::new(),
            scrollables: 0,
            found: None,
        }
    }

    fn visit(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if self.found.is_some() || id != Some(self.target) {
            return;
        }

        let shift = self
            .stack
            .iter()
            .fold(Vector::ZERO, |shift, frame| shift + frame.translation);

        self.found = Some(Found {
            bounds: Rectangle::new(bounds.position() - shift, bounds.size()),
            scroll: self.stack.last().map(|frame| (*frame, bounds)),
        });
    }
}

impl Operation for Locate<'_> {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if self.found.is_some() {
            return;
        }

        let pushed = self.pending.take();

        if let Some(frame) = pushed {
            self.stack.push(frame);
        }

        operate(self);

        if pushed.is_some() {
            self.stack.pop();
        }
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        self.visit(id, bounds);
    }

    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn Focusable) {
        self.visit(id, bounds);
    }

    fn text_input(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn TextInput) {
        self.visit(id, bounds);
    }

    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, _text: &str) {
        self.visit(id, bounds);
    }

    fn custom(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn std::any::Any) {
        self.visit(id, bounds);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.visit(id, bounds);
        self.pending = Some(ScrollFrame {
            index: self.scrollables,
            bounds,
            content: content_bounds,
            translation,
        });
        self.scrollables += 1;
    }
}

/// The scroll offset that centers `target` in the scrollable of `frame`,
/// along each axis the content overflows, clamped to the content.
fn centering_offset(frame: &ScrollFrame, target: Rectangle) -> AbsoluteOffset<Option<f32>> {
    let axis = |extent: f32, content_start: f32, content_extent: f32, center: f32| {
        let max = content_extent - extent;

        (max > 0.0).then(|| (center - content_start - extent / 2.0).clamp(0.0, max))
    };

    AbsoluteOffset {
        x: axis(
            frame.bounds.width,
            frame.content.x,
            frame.content.width,
            target.center_x(),
        ),
        y: axis(
            frame.bounds.height,
            frame.content.y,
            frame.content.height,
            target.center_y(),
        ),
    }
}

/// An operation that scrolls the scrollable at `index`, in traversal
/// order, to `offset`.
struct ScrollAt {
    index: usize,
    offset: AbsoluteOffset<Option<f32>>,
    seen: usize,
}

impl Operation for ScrollAt {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if self.seen <= self.index {
            operate(self);
        }
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        if self.seen == self.index {
            state.scroll_to(self.offset);
        }
        self.seen += 1;
    }
}

/// An operation that focuses the focusable widget at `target`, in
/// traversal order, and unfocuses every other one.
struct FocusAt {
    target: usize,
    current: usize,
}

impl Operation for FocusAt {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        if self.current == self.target {
            state.focus();
        } else {
            state.unfocus();
        }
        self.current += 1;
    }
}

/// Finds the widget with the `target` id in `content`, laid out at
/// `layout`.
fn locate<Message>(
    content: &mut Element<'_, Message>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    target: &Id,
) -> Option<Found> {
    let mut locate = Locate::new(target);

    content
        .as_widget_mut()
        .operate(tree, layout, renderer, &mut locate);

    locate.found
}

/// The spotlight around the target of `active` in `content`, relative to
/// `origin`.
fn spotlight<Message>(
    active: &Active<'_, Message>,
    content: &mut Element<'_, Message>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    origin: Point,
) -> Option<Rectangle> {
    let target = active.target.as_ref()?;
    let found = locate(content, tree, layout, renderer, target)?;
    let bounds = found.bounds;

    Some(
        Rectangle::new(
            Point::new(bounds.x - origin.x, bounds.y - origin.y),
            bounds.size(),
        )
        .expand(active.padding),
    )
}

/// The placement options of the card next to a target.
fn geometry(placement: Placement, direction: ReadingDirection) -> Geometry {
    Geometry {
        side: Side::of(placement, direction),
        alignment: anchored::Alignment::Center,
        direction,
        gap: CARD_GAP,
        margin: spacing::SM,
        arrow: Some(ARROW),
        radius: radius::MEDIUM,
    }
}

/// Moves the focus to the next focusable widget of `card`, or the previous
/// one when not `forward`, wrapping around at the ends.
fn cycle<Message>(
    card: &mut Element<'_, Message>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    forward: bool,
) {
    let Some(Count { focused, total }) = crate::focus::run(card, tree, layout, renderer, count())
    else {
        return;
    };

    let wrap = match (focused, forward) {
        (None, _) => true,
        (Some(focused), true) => focused + 1 >= total,
        (Some(focused), false) => focused == 0,
    };

    let _ = match (wrap, forward) {
        (true, true) => crate::focus::run(card, tree, layout, renderer, focus_first::<()>()),
        (true, false) => crate::focus::run(card, tree, layout, renderer, focus_last::<()>()),
        (false, true) => crate::focus::run(
            card,
            tree,
            layout,
            renderer,
            operation::focusable::focus_next::<()>(),
        ),
        (false, false) => crate::focus::run(
            card,
            tree,
            layout,
            renderer,
            operation::focusable::focus_previous::<()>(),
        ),
    };
}

impl<Message> Layer<'_, Message> {
    /// Updates the state for the step about to be shown.
    fn sync(&self, state: &mut State) {
        let index = self.active.as_ref().map(|active| active.index);

        if index == state.shown {
            return;
        }

        match (state.shown, index) {
            (_, None) => {
                state.restore = state.saved_focus.take();
                state.target = None;
                state.moving = None;
                state.focus_pending = false;
                state.scroll_pending = false;
                state.starting = false;
            }
            (None, Some(_)) => {
                state.restore = None;
                state.moving = None;
                state.focus_pending = true;
                state.scroll_pending = true;
                state.starting = true;
            }
            (Some(_), Some(_)) => {
                state.moving = match (self.animated, state.displayed()) {
                    (true, Some(from)) => Some(Move {
                        from,
                        started: None,
                    }),
                    _ => None,
                };
                state.focus_pending = true;
                state.scroll_pending = true;
            }
        }

        state.shown = index;
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Layer<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let active = self.active.is_some();

        tree::State::new(State {
            shown: self.active.as_ref().map(|active| active.index),
            focus_pending: active,
            scroll_pending: active,
            starting: active,
            ..State::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        std::iter::once(Tree::new(&self.content))
            .chain(
                self.active
                    .iter()
                    .flat_map(|active| active.cards.iter().map(Tree::new)),
            )
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let children: Vec<&Element<'_, Message>> = std::iter::once(&self.content)
            .chain(self.active.iter().flat_map(|active| active.cards.iter()))
            .collect();

        tree.diff_children(&children);
        self.sync(tree.state.downcast_mut::<State>());
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Tree {
            state: tree_state,
            children,
            ..
        } = tree;
        let Some((content_tree, card_trees)) = children.split_first_mut() else {
            return layout::Node::new(Size::ZERO);
        };
        let content = self
            .content
            .as_widget_mut()
            .layout(content_tree, renderer, limits);
        let window = content.size();

        let Some(active) = self.active.as_mut() else {
            return layout::Node::with_children(window, vec![content]);
        };

        let state = tree_state.downcast_mut::<State>();

        state.target = spotlight(
            active,
            &mut self.content,
            content_tree,
            Layout::new(&content),
            renderer,
            Point::ORIGIN,
        );
        state.card =
            usize::from(active.wait_for_tap && state.target.is_some() && active.cards.len() > 1);

        let (Some(card), Some(card_tree)) = (
            active.cards.get_mut(state.card),
            card_trees.get_mut(state.card),
        ) else {
            return layout::Node::with_children(window, vec![content]);
        };

        let geometry = geometry(active.placement, self.direction);
        let margin = geometry.margin;
        let width = CARD_MAX_WIDTH.min(window.width - 2.0 * margin).max(0.0);
        let mut layout_card = |height: f32| {
            card.as_widget_mut().layout(
                card_tree,
                renderer,
                &layout::Limits::new(Size::new(width, 0.0), Size::new(width, height.max(0.0))),
            )
        };

        let node = match state.target {
            Some(anchor) => {
                let node = layout_card(window.height - 2.0 * margin);
                let side = geometry.side_for(node.size(), anchor, window);
                let space = geometry.space(side, anchor, window);
                let node = if side.is_vertical() && node.size().height > space {
                    layout_card(space)
                } else {
                    node
                };
                let placed = place(anchor, node.size(), window, side, &geometry);

                node.move_to(placed.position)
            }
            None => {
                let node = layout_card(window.height);
                let size = node.size();

                node.move_to(Point::new(
                    (window.width - size.width) / 2.0,
                    (window.height - size.height) / 2.0,
                ))
            }
        };

        layout::Node::with_children(window, vec![content, node])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut layouts = layout.children();
        let Some(content_layout) = layouts.next() else {
            return;
        };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            operation,
        );

        let card = tree.state.downcast_ref::<State>().card;

        if let (Some(active), Some(card_layout)) = (self.active.as_mut(), layouts.next())
            && let (Some(element), Some(card_tree)) =
                (active.cards.get_mut(card), tree.children.get_mut(1 + card))
        {
            element
                .as_widget_mut()
                .operate(card_tree, card_layout, renderer, operation);
        }
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
        let origin = layout.position();
        let mut layouts = layout.children();
        let Some(content_layout) = layouts.next() else {
            return;
        };
        let Tree {
            state: tree_state,
            children,
            ..
        } = tree;
        let Some((content_tree, card_trees)) = children.split_first_mut() else {
            return;
        };
        let state = tree_state.downcast_mut::<State>();

        let (Some(active), Some(card_layout)) = (self.active.as_mut(), layouts.next()) else {
            if let Some(target) = state.restore.take() {
                let _ = crate::focus::run(
                    &mut self.content,
                    content_tree,
                    content_layout,
                    renderer,
                    FocusAt { target, current: 0 },
                );
                shell.request_redraw();
            }

            self.content.as_widget_mut().update(
                content_tree,
                event,
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
            return;
        };
        let (Some(card), Some(card_tree)) = (
            active.cards.get_mut(state.card),
            card_trees.get_mut(state.card),
        ) else {
            return;
        };

        if let IcedEvent::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if let Some(moving) = state.moving.as_mut() {
                moving.started.get_or_insert(*now);
            }
            if state.progress() >= 1.0 {
                state.moving = None;
            }
        }
        if state.moving.is_some() {
            shell.request_redraw();
        }

        if state.scroll_pending {
            state.scroll_pending = false;

            if let Some(target) = &active.target
                && let Some(Found {
                    scroll: Some((frame, bounds)),
                    ..
                }) = locate(
                    &mut self.content,
                    content_tree,
                    content_layout,
                    renderer,
                    target,
                )
            {
                let mut scroll = ScrollAt {
                    index: frame.index,
                    offset: centering_offset(&frame, bounds),
                    seen: 0,
                };

                self.content.as_widget_mut().operate(
                    content_tree,
                    content_layout,
                    renderer,
                    &mut scroll,
                );
                shell.request_redraw();
            }
        }

        if state.focus_pending {
            state.focus_pending = false;

            if state.starting {
                state.starting = false;
                state.saved_focus = crate::focus::run(
                    &mut self.content,
                    content_tree,
                    content_layout,
                    renderer,
                    count(),
                )
                .and_then(|count| count.focused);
                let _ = crate::focus::run(
                    &mut self.content,
                    content_tree,
                    content_layout,
                    renderer,
                    operation::focusable::unfocus::<()>(),
                );
            }

            // The next button takes the focus, or the skip button while
            // the step waits for a tap.
            let _ = if state.card == 0 {
                crate::focus::run(card, card_tree, card_layout, renderer, focus_last::<()>())
            } else {
                crate::focus::run(card, card_tree, card_layout, renderer, focus_first::<()>())
            };
            shell.request_redraw();
        }

        let card_bounds = card_layout.bounds();

        card.as_widget_mut().update(
            card_tree,
            event,
            card_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        let focused =
            crate::focus::run(card, card_tree, card_layout, renderer, has_focus()) == Some(true);

        if !shell.is_event_captured()
            && focused
            && let IcedEvent::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event
        {
            let rtl = self.direction == ReadingDirection::RightToLeft;
            let message = match key {
                keyboard::Key::Named(key::Named::Escape) => Some(active.messages.skip.clone()),
                keyboard::Key::Named(key::Named::ArrowRight) if modifiers.is_empty() => {
                    Some(if rtl {
                        (active.index > 0)
                            .then(|| active.messages.previous.clone())
                            .flatten()
                    } else {
                        active.messages.next.clone()
                    })
                }
                keyboard::Key::Named(key::Named::ArrowLeft) if modifiers.is_empty() => {
                    Some(if rtl {
                        active.messages.next.clone()
                    } else {
                        (active.index > 0)
                            .then(|| active.messages.previous.clone())
                            .flatten()
                    })
                }
                keyboard::Key::Named(key::Named::Tab) => {
                    cycle(card, card_tree, card_layout, renderer, !modifiers.shift());
                    shell.capture_event();
                    shell.request_redraw();
                    None
                }
                _ => None,
            };

            if let Some(message) = message {
                if let Some(message) = message {
                    shell.publish(message);
                }
                shell.capture_event();
            }
        }

        // Pointer input reaches the content only inside the spotlight.
        let hole = state
            .displayed()
            .map(|hole| hole + Vector::new(origin.x, origin.y));
        let inside = |point: Point| {
            !card_bounds.contains(point)
                && hole.is_some_and(|hole| rounded_contains(hole, radius::MEDIUM, point))
        };
        let content_cursor = match cursor.position() {
            Some(point) if inside(point) => cursor,
            _ => mouse::Cursor::Unavailable,
        };
        let forward = match event {
            IcedEvent::Window(_) | IcedEvent::Mouse(_) => true,
            IcedEvent::Touch(
                touch::Event::FingerPressed { position, .. }
                | touch::Event::FingerMoved { position, .. }
                | touch::Event::FingerLifted { position, .. }
                | touch::Event::FingerLost { position, .. },
            ) => inside(*position),
            IcedEvent::Keyboard(_) | IcedEvent::InputMethod(_) => {
                !focused && !shell.is_event_captured()
            }
        };

        if forward {
            self.content.as_widget_mut().update(
                content_tree,
                event,
                content_layout,
                content_cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        // A release inside the target of a waiting step moves the tour on,
        // after the target has received it.
        let released = match event {
            IcedEvent::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                content_cursor.position()
            }
            IcedEvent::Touch(touch::Event::FingerLifted { position, .. }) => Some(*position),
            _ => None,
        };

        if active.wait_for_tap
            && let Some(point) = released
            && let Some(target) = state.target
            && (target + Vector::new(origin.x, origin.y)).contains(point)
            && let Some(message) = active.messages.next.clone()
        {
            shell.publish(message);
        }

        // The target moves when the content scrolls or changes.
        let target = spotlight(
            active,
            &mut self.content,
            content_tree,
            content_layout,
            renderer,
            origin,
        );

        if target != state.target {
            state.target = target;
            shell.invalidate_layout();
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
        use geometry::Renderer as _;
        use iced::advanced::Renderer as _;

        let mut layouts = layout.children();
        let Some(content_layout) = layouts.next() else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let origin = Vector::new(bounds.x, bounds.y);

        let (Some(active), Some(card_layout)) = (self.active.as_ref(), layouts.next()) else {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
            return;
        };

        let card_bounds = card_layout.bounds();
        let hole = state.displayed().map(|hole| hole + origin);
        let content_cursor = match cursor.position() {
            Some(point)
                if !card_bounds.contains(point)
                    && hole.is_some_and(|hole| rounded_contains(hole, radius::MEDIUM, point)) =>
            {
                cursor
            }
            _ => mouse::Cursor::Unavailable,
        };

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            content_layout,
            content_cursor,
            viewport,
        );

        renderer.with_layer(bounds, |renderer| {
            let mut frame = Frame::new(renderer, bounds.size());
            let path = Path::new(|builder| {
                builder.rectangle(Point::ORIGIN, bounds.size());

                if let Some(hole) = state.displayed() {
                    builder.rounded_rectangle(hole.position(), hole.size(), radius::MEDIUM.into());
                }
            });

            frame.fill(
                &path,
                fill::Fill {
                    style: geometry::Style::Solid(theme.roles().scrim.scale_alpha(SCRIM_OPACITY)),
                    rule: fill::Rule::EvenOdd,
                },
            );

            let scrim = frame.into_geometry();

            renderer.with_translation(origin, |renderer| renderer.draw_geometry(scrim));
        });

        // The card goes on a layer of its own, above the scrim.
        renderer.with_layer(bounds, |renderer| {
            let surface = <Theme as container::Catalog>::style(theme, &ContainerClass::Menu);

            container::draw_background(renderer, &surface, card_bounds);

            if let Some(target) = state.target {
                let anchor = target + origin;
                let geometry = geometry(active.placement, self.direction);
                let placed = geometry.placed_at(anchor, card_bounds);

                anchored::draw_arrow(renderer, card_bounds, placed, ARROW, &surface);
            }

            if let (Some(card), Some(card_tree)) = (
                active.cards.get(state.card),
                tree.children.get(1 + state.card),
            ) {
                let style = renderer::Style {
                    text_color: surface.text_color.unwrap_or(style.text_color),
                };

                card.as_widget().draw(
                    card_tree,
                    renderer,
                    theme,
                    &style,
                    card_layout,
                    cursor,
                    &bounds,
                );
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let mut layouts = layout.children();
        let Some(content_layout) = layouts.next() else {
            return mouse::Interaction::None;
        };
        let content = |cursor| {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                content_layout,
                cursor,
                viewport,
                renderer,
            )
        };

        let (Some(active), Some(card_layout)) = (self.active.as_ref(), layouts.next()) else {
            return content(cursor);
        };
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let Some(point) = cursor.position() else {
            return mouse::Interaction::None;
        };

        if card_layout.bounds().contains(point) {
            let interaction = match (
                active.cards.get(state.card),
                tree.children.get(1 + state.card),
            ) {
                (Some(card), Some(card_tree)) => card.as_widget().mouse_interaction(
                    card_tree,
                    card_layout,
                    cursor,
                    &bounds,
                    renderer,
                ),
                _ => mouse::Interaction::None,
            };

            return if interaction == mouse::Interaction::None {
                mouse::Interaction::Idle
            } else {
                interaction
            };
        }

        let inside = state
            .displayed()
            .map(|hole| hole + Vector::new(bounds.x, bounds.y))
            .is_some_and(|hole| rounded_contains(hole, radius::MEDIUM, point));

        if inside {
            content(cursor)
        } else {
            mouse::Interaction::Idle
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
        let bounds = layout.bounds();
        let mut layouts = layout.children();
        let content_layout = layouts.next()?;
        let card = tree.state.downcast_ref::<State>().card;
        let (content_tree, card_trees) = tree.children.split_first_mut()?;

        let content_overlay = self.content.as_widget_mut().overlay(
            content_tree,
            content_layout,
            renderer,
            viewport,
            translation,
        );

        let card_overlay = match (self.active.as_mut(), layouts.next()) {
            (Some(active), Some(card_layout)) => {
                match (active.cards.get_mut(card), card_trees.get_mut(card)) {
                    (Some(element), Some(card_tree)) => element.as_widget_mut().overlay(
                        card_tree,
                        card_layout,
                        renderer,
                        &bounds,
                        translation,
                    ),
                    _ => None,
                }
            }
            _ => None,
        };

        let overlays: Vec<_> = content_overlay.into_iter().chain(card_overlay).collect();

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tour() -> Tour {
        Tour::new(vec![Step::new("A"), Step::new("B"), Step::new("C")])
    }

    #[test]
    fn a_new_tour_is_idle() {
        let tour = tour();

        assert!(!tour.is_active());
        assert_eq!(tour.index(), None);
        assert!(tour.step().is_none());
        assert!(!tour.completed());
    }

    #[test]
    fn next_and_previous_move_and_the_last_next_completes() {
        let mut tour = tour();

        tour.start();
        assert_eq!(tour.index(), Some(0));
        tour.previous();
        assert_eq!(tour.index(), Some(0));
        assert_eq!(tour.apply(Event::Next), None);
        assert_eq!(tour.step().map(Step::title), Some("B"));
        assert_eq!(tour.apply(Event::Previous), None);
        assert_eq!(tour.index(), Some(0));
        tour.go_to(2);
        assert_eq!(tour.apply(Event::Next), Some(true));
        assert!(!tour.is_active());
        assert!(tour.completed());
        assert_eq!(tour.next(), None);
    }

    #[test]
    fn skip_ends_without_completing_and_start_resets() {
        let mut tour = tour();

        assert_eq!(tour.skip(), None);
        tour.start();
        assert_eq!(tour.apply(Event::Skip), Some(false));
        assert!(!tour.completed());
        tour.go_to(2);
        let _ = tour.next();
        assert!(tour.completed());
        tour.start();
        assert!(!tour.completed());
    }

    #[test]
    fn out_of_range_steps_are_ignored() {
        let mut tour = tour();

        tour.go_to(3);
        assert!(!tour.is_active());

        let mut empty = Tour::new(Vec::new());
        empty.start();
        assert!(!empty.is_active());
    }

    #[test]
    fn steps_keep_their_options() {
        let step = Step::new("Search")
            .description("Find files")
            .target("search")
            .placement(Placement::Start)
            .wait_for_tap(true)
            .spotlight_padding(10.0);

        assert_eq!(step.title(), "Search");
        assert_eq!(step.description_text(), Some("Find files"));
        assert_eq!(step.target_id(), Some(&Id::new("search")));
        assert_eq!(step.placement_of(), Placement::Start);
        assert!(step.waits_for_tap());
        assert_eq!(step.padding(), 10.0);
        assert_eq!(Step::new("x").padding(), SPOTLIGHT_PADDING);
    }

    #[test]
    fn the_spotlight_eases_from_the_previous_target() {
        let from = Rectangle::new(Point::new(0.0, 0.0), Size::new(10.0, 10.0));
        let to = Rectangle::new(Point::new(100.0, 50.0), Size::new(30.0, 20.0));
        let start = Instant::now();
        let mut state = State {
            target: Some(to),
            moving: Some(Move {
                from,
                started: Some(start),
            }),
            now: Some(start),
            ..State::default()
        };

        assert_eq!(state.displayed(), Some(from));

        state.now = Some(start + MOVE_DURATION / 2);
        let half = state.displayed().unwrap_or(from);
        let eased = ease_out_cubic(0.5);
        assert!((half.x - 100.0 * eased).abs() < 0.01);
        assert!(half.x > 50.0, "ease-out runs ahead of linear");

        state.now = Some(start + MOVE_DURATION);
        assert_eq!(state.displayed(), Some(to));
    }

    #[test]
    fn the_hole_has_rounded_corners() {
        let hole = Rectangle::new(Point::new(0.0, 0.0), Size::new(40.0, 20.0));

        assert!(rounded_contains(hole, 8.0, Point::new(20.0, 10.0)));
        assert!(rounded_contains(hole, 8.0, Point::new(0.5, 10.0)));
        assert!(!rounded_contains(hole, 8.0, Point::new(0.5, 0.5)));
        assert!(rounded_contains(hole, 8.0, Point::new(3.0, 3.0)));
        assert!(!rounded_contains(hole, 8.0, Point::new(41.0, 10.0)));
    }

    #[test]
    fn the_target_is_centered_in_its_scrollable() {
        let frame = ScrollFrame {
            index: 0,
            bounds: Rectangle::new(Point::new(0.0, 0.0), Size::new(200.0, 100.0)),
            content: Rectangle::new(Point::new(0.0, 0.0), Size::new(200.0, 1000.0)),
            translation: Vector::ZERO,
        };
        let middle = Rectangle::new(Point::new(0.0, 500.0), Size::new(50.0, 20.0));
        let top = Rectangle::new(Point::new(0.0, 10.0), Size::new(50.0, 20.0));
        let bottom = Rectangle::new(Point::new(0.0, 990.0), Size::new(50.0, 10.0));

        assert_eq!(centering_offset(&frame, middle).y, Some(460.0));
        assert_eq!(centering_offset(&frame, middle).x, None);
        assert_eq!(centering_offset(&frame, top).y, Some(0.0));
        assert_eq!(centering_offset(&frame, bottom).y, Some(900.0));
    }
}
