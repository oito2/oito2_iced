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

//! Carousels: a row of pages shown one at a time, with previous and next
//! buttons over them and a [page indicator](super::page_indicator) below.
//!
//! The current page lives in the application, in a [`Controller`] that
//! also holds the automatic scrolling settings. The carousel publishes the
//! page the user asks for through [`Carousel::on_page_change`]; the
//! application passes it to [`Controller::go_to`], and the pages slide to
//! it. Without [`Carousel::on_page_change`] the carousel only shows the
//! current page.
//!
//! Pages change by the buttons, the dots, the left and right arrow keys
//! (swapped in [right-to-left](Carousel::direction) text) while the focus
//! is in the carousel (it takes the focus as a whole and shows the focus
//! ring), or a swipe, also with a mouse, that snaps to the nearest page.
//! The buttons and arrow keys wrap around when [`Carousel::looping`] is
//! true, jumping straight to the other end.
//!
//! With [`Controller::with_auto_scroll`] the carousel moves to the next
//! page every [`Controller::with_auto_scroll_interval`], wrapping to the
//! first, while no pointer hovers it and nothing inside it has the focus.
//! It never scrolls by itself, and changes pages without sliding, when
//! [`Carousel::animated`] is false.
//!
//! [`PageController`] holds the number of pages and the selected page of
//! other paged views.
//!
//! ```no_run
//! use oito2_iced::widget::carousel::{self, Controller};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Page(usize),
//! }
//!
//! struct App {
//!     carousel: Controller,
//! }
//!
//! impl App {
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::Page(page) => self.carousel.go_to(page),
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         carousel::carousel(
//!             &self.carousel,
//!             ["One", "Two", "Three"].map(|page| iced::widget::text(page).into()),
//!         )
//!         .height(160.0)
//!         .on_page_change(Message::Page)
//!         .into()
//!     }
//! }
//! ```

use std::borrow::Cow;
use std::rc::Rc;
use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard;
use iced::time::Instant;
use iced::{
    Element as IcedElement, Event, Length, Point, Rectangle, Size, Vector, mouse, touch, window,
};

use super::helpers::ReadingDirection;
use super::page_indicator::{self, PageIndicatorLayout};
use crate::animation::ease_in_out_cubic;
use crate::l10n::Localizations;
use crate::tokens::{radius, spacing};
use crate::{Element, Theme};

/// The default height of the pages.
pub const HEIGHT: f32 = 240.0;

/// The default time each page stays before the next one with automatic
/// scrolling.
pub const AUTO_SCROLL_INTERVAL: Duration = Duration::from_secs(4);

/// The default length of a page change.
pub const ANIMATION_DURATION: Duration = Duration::from_millis(400);

/// The opacity of the background of the previous and next buttons (a
/// literal in the OITO2 design).
#[cfg(feature = "svg")]
const CONTROL_OPACITY: f32 = 0.85;

/// How far a mouse moves before a press becomes a swipe (the precise
/// pointer slop of the OITO2 design).
const MOUSE_SLOP: f32 = 1.0;

/// How far a finger moves before a touch becomes a swipe (the touch slop
/// of the OITO2 design).
const TOUCH_SLOP: f32 = 18.0;

/// The current page of a [`Carousel`] and its automatic scrolling, owned
/// by the application.
#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    page: usize,
    animate: bool,
    auto_scroll: bool,
    auto_scroll_interval: Duration,
    animation_duration: Duration,
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

impl Controller {
    /// A controller on the first page, without automatic scrolling.
    pub fn new() -> Self {
        Self {
            page: 0,
            animate: true,
            auto_scroll: false,
            auto_scroll_interval: AUTO_SCROLL_INTERVAL,
            animation_duration: ANIMATION_DURATION,
        }
    }

    /// Starts on `page`.
    #[must_use]
    pub fn with_initial_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    /// Sets whether the carousel advances by itself (false by default).
    #[must_use]
    pub fn with_auto_scroll(mut self, auto_scroll: bool) -> Self {
        self.auto_scroll = auto_scroll;
        self
    }

    /// Sets the time each page stays before the next one with automatic
    /// scrolling ([`AUTO_SCROLL_INTERVAL`] by default).
    #[must_use]
    pub fn with_auto_scroll_interval(mut self, interval: Duration) -> Self {
        self.auto_scroll_interval = interval;
        self
    }

    /// Sets the length of a page change ([`ANIMATION_DURATION`] by
    /// default).
    #[must_use]
    pub fn with_animation_duration(mut self, duration: Duration) -> Self {
        self.animation_duration = duration;
        self
    }

    /// The current page.
    pub fn page(&self) -> usize {
        self.page
    }

    /// Whether the carousel advances by itself.
    pub fn auto_scroll(&self) -> bool {
        self.auto_scroll
    }

    /// The time each page stays before the next one with automatic
    /// scrolling.
    pub fn auto_scroll_interval(&self) -> Duration {
        self.auto_scroll_interval
    }

    /// The length of a page change.
    pub fn animation_duration(&self) -> Duration {
        self.animation_duration
    }

    /// Slides to `page`.
    pub fn go_to(&mut self, page: usize) {
        self.page = page;
        self.animate = true;
    }

    /// Shows `page` at once, without sliding.
    pub fn jump_to(&mut self, page: usize) {
        self.page = page;
        self.animate = false;
    }
}

/// The number of pages and the selected page of a paged view, such as a
/// master-detail or a navigation page, owned by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PageController {
    length: usize,
    index: Option<usize>,
}

impl PageController {
    /// A controller of `length` pages with `index` selected; an index
    /// beyond the pages selects none.
    pub fn new(length: usize, index: Option<usize>) -> Self {
        Self {
            length,
            index: index.filter(|index| *index < length),
        }
    }

    /// The number of pages.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Changes the number of pages, and clears the selection when the
    /// selected page is beyond the new length.
    pub fn set_length(&mut self, length: usize) {
        self.length = length;
        self.index = self.index.filter(|index| *index < length);
    }

    /// The selected page, if any.
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    /// Selects `index`, or clears the selection with `None`. An index
    /// beyond the pages leaves the selection as it was and returns false.
    pub fn set_index(&mut self, index: Option<usize>) -> bool {
        if index.is_some_and(|index| index >= self.length) {
            return false;
        }

        self.index = index;
        true
    }

    /// Whether a page is selected.
    pub fn has_selection(&self) -> bool {
        self.index.is_some()
    }
}

/// A row of pages shown one at a time. See [`carousel`].
pub struct Carousel<'a, Message> {
    controller: Controller,
    pages: Vec<Element<'a, Message>>,
    width: Length,
    height: f32,
    navigation_controls: bool,
    page_indicator: bool,
    looping: bool,
    on_page_change: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    direction: ReadingDirection,
    animated: bool,
    id: Option<Id>,
    previous_label: Cow<'a, str>,
    next_label: Cow<'a, str>,
}

/// A carousel of `pages` on the page of `controller`, as wide as its
/// parent and [`HEIGHT`] tall, with previous and next buttons and a page
/// indicator. Without [`Carousel::on_page_change`] the page does not
/// change.
pub fn carousel<'a, Message>(
    controller: &Controller,
    pages: impl IntoIterator<Item = Element<'a, Message>>,
) -> Carousel<'a, Message> {
    let strings = Localizations::en();

    Carousel {
        controller: controller.clone(),
        pages: pages.into_iter().collect(),
        width: Length::Fill,
        height: HEIGHT,
        navigation_controls: true,
        page_indicator: true,
        looping: true,
        on_page_change: None,
        direction: ReadingDirection::default(),
        animated: true,
        id: None,
        previous_label: Cow::Borrowed(strings.material.previous_page),
        next_label: Cow::Borrowed(strings.material.next_page),
    }
}

impl<'a, Message> Carousel<'a, Message> {
    /// Sets the function that turns the page the user asks for into a
    /// message.
    #[must_use]
    pub fn on_page_change(mut self, on_page_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_page_change = Some(Rc::new(on_page_change));
        self
    }

    /// Sets the width ([`Length::Fill`] by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the pages ([`HEIGHT`] by default).
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// Sets whether previous and next buttons sit over the pages (true by
    /// default). The buttons need the `svg` feature.
    #[must_use]
    pub fn navigation_controls(mut self, navigation_controls: bool) -> Self {
        self.navigation_controls = navigation_controls;
        self
    }

    /// Sets whether a page indicator sits below the pages (true by
    /// default); it shows only with more than one page.
    #[must_use]
    pub fn page_indicator(mut self, page_indicator: bool) -> Self {
        self.page_indicator = page_indicator;
        self
    }

    /// Sets whether going past the last page returns to the first, and
    /// back (true by default).
    #[must_use]
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    /// Sets the reading direction: right to left puts the previous button
    /// at the right and swaps the left and right arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether pages slide and scroll by themselves (true by
    /// default); false changes pages at once and turns automatic scrolling
    /// off, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the tooltips of the previous and next buttons from
    /// `localizations`.
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.previous_label = Cow::Borrowed(localizations.material.previous_page);
        self.next_label = Cow::Borrowed(localizations.material.next_page);
        self
    }

    /// Sets the widget [`Id`], used to focus the carousel programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// The page `step` pages from `page` among `count`, wrapping when
/// `looping` and clamping otherwise, with whether it wrapped; `None` when
/// it stays on `page`.
fn step(page: usize, count: usize, step: isize, looping: bool) -> Option<(usize, bool)> {
    if count == 0 {
        return None;
    }

    let count = count as isize;
    let target = page as isize + step;
    let (target, wraps) = if looping {
        (target.rem_euclid(count), target < 0 || target >= count)
    } else {
        (target.clamp(0, count - 1), false)
    };
    let target = target as usize;

    (target != page).then_some((target, wraps))
}

/// The previous or next button.
#[cfg(feature = "svg")]
fn control<'a, Message: Clone + 'a>(
    forward: bool,
    direction: ReadingDirection,
    label: Cow<'a, str>,
    message: Option<Message>,
) -> Element<'a, Message> {
    use super::button::icon_button;
    use super::helpers::Side;
    use crate::icon::lucide;
    use iced::widget::container;

    let chevron = match direction.chevron(forward) {
        Side::Left => lucide::CHEVRON_LEFT,
        Side::Right => lucide::CHEVRON_RIGHT,
    };

    container(icon_button(chevron).tooltip(label).on_press_maybe(message))
        .style(|theme: &Theme| container::Style {
            background: Some(
                theme
                    .roles()
                    .surface_container_highest
                    .scale_alpha(CONTROL_OPACITY)
                    .into(),
            ),
            border: iced::Border::default().rounded(radius::MEDIUM),
            ..container::Style::default()
        })
        .into()
}

impl<'a, Message: Clone + 'a> From<Carousel<'a, Message>> for Element<'a, Message> {
    fn from(carousel: Carousel<'a, Message>) -> Self {
        let count = carousel.pages.len();
        let page = carousel.controller.page.min(count.saturating_sub(1));
        let enabled = carousel.on_page_change.is_some();
        let request = |target: Option<(usize, bool)>| {
            carousel
                .on_page_change
                .as_ref()
                .zip(target)
                .map(|(on_page_change, (target, _))| on_page_change(target))
        };

        let targets = [
            step(page, count, -1, carousel.looping),
            step(page, count, 1, carousel.looping),
        ];

        let mut children = carousel.pages;

        #[cfg(feature = "svg")]
        let controls = carousel.navigation_controls;
        #[cfg(not(feature = "svg"))]
        let controls = false;

        #[cfg(feature = "svg")]
        if controls {
            children.push(control(
                false,
                carousel.direction,
                carousel.previous_label,
                request(targets[0]),
            ));
            children.push(control(
                true,
                carousel.direction,
                carousel.next_label,
                request(targets[1]),
            ));
        }
        #[cfg(not(feature = "svg"))]
        let _ = (&request, carousel.previous_label, carousel.next_label);

        let indicator = carousel.page_indicator && count > 1;

        if indicator {
            let on_page_change = carousel.on_page_change.clone();

            children.push(
                page_indicator::page_indicator(count, page)
                    .layout(PageIndicatorLayout::Bounded)
                    .direction(carousel.direction)
                    .animated(carousel.animated)
                    .on_select_maybe(
                        on_page_change.map(|on_page_change| move |page| on_page_change(page)),
                    )
                    .into(),
            );
        }

        IcedElement::new(Inner {
            page,
            count,
            controls,
            children,
            targets,
            width: carousel.width,
            height: carousel.height,
            looping: carousel.looping,
            on_page_change: carousel.on_page_change.filter(|_| enabled),
            direction: carousel.direction,
            animated: carousel.animated,
            jump: !carousel.controller.animate,
            auto_scroll: carousel
                .controller
                .auto_scroll
                .then_some(carousel.controller.auto_scroll_interval),
            duration: carousel.controller.animation_duration,
            id: carousel.id,
        })
    }
}

/// The carousel widget: the pages, then the previous and next buttons when
/// present, then the indicator when present.
struct Inner<'a, Message> {
    page: usize,
    count: usize,
    controls: bool,
    children: Vec<Element<'a, Message>>,
    /// The pages the previous and next buttons go to, with whether they
    /// wrap.
    targets: [Option<(usize, bool)>; 2],
    width: Length,
    height: f32,
    looping: bool,
    on_page_change: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    direction: ReadingDirection,
    animated: bool,
    /// Whether the last page change of the controller asked not to slide.
    jump: bool,
    auto_scroll: Option<Duration>,
    duration: Duration,
    id: Option<Id>,
}

/// A slide between two page positions.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Slide {
    from: f32,
    to: f32,
    /// The first frame of the slide; `None` until it is drawn.
    started: Option<Instant>,
}

/// A pointer press on the pages, which becomes a swipe past the slop.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Swipe {
    origin: Point,
    /// The page position at the press.
    start: f32,
    /// The page position under the pointer.
    position: f32,
    swiping: bool,
    touch: bool,
}

/// The tree state of a carousel.
#[derive(Debug, Clone, Default)]
struct State {
    focused: bool,
    /// Whether a widget inside the carousel has the focus.
    focus_inside: bool,
    /// Whether the last input came from the keyboard, which shows the
    /// ring.
    keyboard: bool,
    hovered: bool,
    page: usize,
    slide: Option<Slide>,
    swipe: Option<Swipe>,
    /// The page the carousel asked for by wrapping around, which shows
    /// without sliding.
    pending_jump: Option<usize>,
    /// When automatic scrolling moves next.
    next_tick: Option<Instant>,
    now: Option<Instant>,
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

impl State {
    /// The page position shown: fractional while sliding or swiping.
    fn position(&self, duration: Duration) -> f32 {
        if let Some(swipe) = self.swipe.filter(|swipe| swipe.swiping) {
            return swipe.position;
        }

        match self.slide {
            Some(slide) => {
                let progress = match (slide.started, self.now) {
                    (Some(started), Some(now)) if !duration.is_zero() => {
                        now.saturating_duration_since(started).as_secs_f32()
                            / duration.as_secs_f32()
                    }
                    (Some(_), _) => 1.0,
                    (None, _) => 0.0,
                };

                slide.from + (slide.to - slide.from) * ease_in_out_cubic(progress.clamp(0.0, 1.0))
            }
            None => self.page as f32,
        }
    }
}

impl<Message> Inner<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_page_change.is_some() && self.count > 0
    }

    /// The index of the first control child.
    fn controls_start(&self) -> usize {
        self.count
    }

    /// The area of the pages inside `bounds`.
    fn pages_area(&self, bounds: Rectangle) -> Rectangle {
        Rectangle {
            height: self.height.min(bounds.height),
            ..bounds
        }
    }

    /// The side later pages sit on: 1 for the right, -1 for the left.
    fn sign(&self) -> f32 {
        match self.direction {
            ReadingDirection::LeftToRight => 1.0,
            ReadingDirection::RightToLeft => -1.0,
        }
    }

    /// The translation of the pages for the shown `position`.
    fn offset(&self, area: Rectangle, position: f32) -> Vector {
        Vector::new(
            self.sign() * (self.page as f32 - position) * area.width,
            0.0,
        )
    }

    /// Asks for `target`, remembering a wrap so it shows without sliding.
    fn request(&self, state: &mut State, shell: &mut Shell<'_, Message>, target: (usize, bool)) {
        if let Some(on_page_change) = &self.on_page_change {
            if target.1 {
                state.pending_jump = Some(target.0);
            }
            shell.publish(on_page_change(target.0));
        }
    }

    /// Whether a widget inside the carousel has the focus.
    fn has_focus_inside(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
    ) -> bool {
        let page = self.page;

        self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
            .filter(|(index, _)| *index >= self.count || *index == page)
            .any(|(_, ((child, tree), layout))| {
                crate::focus::run(
                    child,
                    tree,
                    layout,
                    renderer,
                    crate::focus::operations::has_focus(),
                )
                .unwrap_or(false)
            })
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Inner<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            page: self.page,
            ..State::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);

        let state = tree.state.downcast_mut::<State>();

        if state.page != self.page {
            let jump = !self.animated || self.jump || state.pending_jump == Some(self.page);

            state.slide = (!jump).then(|| Slide {
                from: state.position(self.duration),
                to: self.page as f32,
                started: None,
            });
            state.swipe = None;
            state.page = self.page;
            state.pending_jump = None;
            state.next_tick = None;
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let area = limits.width(self.width).resolve(
            self.width,
            Length::Fixed(self.height),
            Size::new(0.0, self.height),
        );
        let width = area.width;
        let height = self.height;
        let pages = layout::Limits::new(Size::new(width, height), Size::new(width, height));
        let loose = layout::Limits::new(Size::ZERO, Size::new(width, height));
        let page = self.page;
        let count = self.count;
        let controls = self.controls;
        let rtl = self.direction == ReadingDirection::RightToLeft;
        let sign = self.sign();
        let mut total = height;

        let nodes = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .enumerate()
            .map(|(index, (child, tree))| {
                if index < count {
                    let x = sign * (index as f32 - page as f32) * width;

                    child
                        .as_widget_mut()
                        .layout(tree, renderer, &pages)
                        .move_to(Point::new(x, 0.0))
                } else if controls && index < count + 2 {
                    let node = child.as_widget_mut().layout(tree, renderer, &loose);
                    let size = node.size();
                    let start = (index == count) != rtl;
                    let x = if start {
                        spacing::SM
                    } else {
                        width - spacing::SM - size.width
                    };

                    node.move_to(Point::new(x, (height - size.height) / 2.0))
                } else {
                    let limits = layout::Limits::new(Size::ZERO, Size::new(width, f32::INFINITY));
                    let node = child.as_widget_mut().layout(tree, renderer, &limits);
                    let size = node.size();

                    total = height + spacing::SM + size.height;

                    node.move_to(Point::new(
                        ((width - size.width) / 2.0).max(0.0),
                        height + spacing::SM,
                    ))
                }
            })
            .collect();

        layout::Node::with_children(Size::new(width, total), nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), layout.bounds());
        }

        let page = self.page;
        let count = self.count;

        operation.traverse(&mut |operation| {
            for (index, ((child, tree), layout)) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .enumerate()
            {
                if index >= count || index == page {
                    child
                        .as_widget_mut()
                        .operate(tree, layout, renderer, operation);
                }
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
        let bounds = layout.bounds();
        let area = self.pages_area(bounds);
        let position = tree.state.downcast_ref::<State>().position(self.duration);
        let offset = self.offset(area, position);
        let page_cursor = if cursor.is_over(area) {
            cursor - offset
        } else {
            mouse::Cursor::Unavailable
        };
        let controls_start = self.controls_start();
        let mut pressed = [false; 2];

        for (index, ((child, child_tree), child_layout)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
        {
            if index < self.count {
                child.as_widget_mut().update(
                    child_tree,
                    event,
                    child_layout,
                    page_cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );
            } else {
                let mut messages = Vec::new();
                let mut local = Shell::new(&mut messages);

                child.as_widget_mut().update(
                    child_tree,
                    event,
                    child_layout,
                    cursor,
                    renderer,
                    clipboard,
                    &mut local,
                    viewport,
                );

                let published = !local.is_empty();

                shell.merge(local, std::convert::identity);

                if self.controls && published && index < controls_start + 2 {
                    pressed[index - controls_start] = true;
                }
            }
        }

        if matches!(
            event,
            Event::Keyboard(_) | Event::Window(window::Event::RedrawRequested(_))
        ) {
            let inside = self.has_focus_inside(tree, layout, renderer);

            tree.state.downcast_mut::<State>().focus_inside = inside;
        }

        let state = tree.state.downcast_mut::<State>();

        for (index, target) in self.targets.iter().enumerate() {
            if pressed[index]
                && let Some((target, true)) = target
            {
                state.pending_jump = Some(*target);
            }
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            if let Some(slide) = &mut state.slide {
                let started = *slide.started.get_or_insert(*now);

                if now.saturating_duration_since(started) >= self.duration {
                    state.slide = None;
                }
                shell.request_redraw();
            }

            let scrolling = self.enabled()
                && self.animated
                && self.count > 1
                && !state.hovered
                && !state.focused
                && !state.focus_inside
                && state.swipe.is_none();

            match self.auto_scroll {
                Some(interval) if scrolling => {
                    let tick = *state.next_tick.get_or_insert(*now + interval);

                    if *now >= tick {
                        state.next_tick = Some(*now + interval);

                        if let Some(target) = step(self.page, self.count, 1, true) {
                            self.request(state, shell, target);
                        }
                    }

                    if let Some(tick) = state.next_tick {
                        shell.request_redraw_at(tick);
                    }
                }
                _ => state.next_tick = None,
            }
        } else if state.slide.is_some_and(|slide| slide.started.is_none()) {
            shell.request_redraw();
        }

        if !self.enabled() {
            state.focused = false;
            state.swipe = None;
            return;
        }

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let hovered = cursor.is_over(bounds);

                if hovered != state.hovered {
                    state.hovered = hovered;
                    state.next_tick = None;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) if state.hovered => {
                state.hovered = false;
                state.next_tick = None;
                shell.request_redraw();
            }
            _ => {}
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                state.keyboard = false;

                if let Some(origin) = cursor.position_over(area)
                    && !shell.is_event_captured()
                {
                    state.swipe = Some(Swipe {
                        origin,
                        start: position,
                        position,
                        swiping: false,
                        touch: matches!(event, Event::Touch(_)),
                    });
                } else if state.focused && !cursor.is_over(bounds) {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let (Some(swipe), Some(point)) = (&mut state.swipe, cursor.position()) else {
                    return;
                };
                let dx = point.x - swipe.origin.x;
                let slop = if swipe.touch { TOUCH_SLOP } else { MOUSE_SLOP };

                if !swipe.swiping && dx.abs() > slop {
                    swipe.swiping = true;
                }

                if swipe.swiping && area.width > 0.0 {
                    swipe.position = (swipe.start - self.sign() * dx / area.width)
                        .clamp(0.0, self.count.saturating_sub(1) as f32);
                    state.slide = None;
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if let Some(swipe) = state.swipe.take()
                    && swipe.swiping
                {
                    let target = (swipe.position.round() as usize).min(self.count - 1);

                    state.slide = Some(Slide {
                        from: swipe.position,
                        to: target as f32,
                        started: None,
                    });

                    if target != self.page {
                        self.request(state, shell, (target, false));
                    }

                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                state.keyboard = true;

                if (state.focused || state.focus_inside)
                    && !shell.is_event_captured()
                    && modifiers.is_empty()
                {
                    let (back, forward) = self.direction.arrows();
                    let delta = match key {
                        keyboard::Key::Named(named) if *named == forward => Some(1),
                        keyboard::Key::Named(named) if *named == back => Some(-1),
                        _ => None,
                    };

                    if let Some(delta) = delta {
                        if let Some(target) = step(self.page, self.count, delta, self.looping) {
                            self.request(state, shell, target);
                        }
                        shell.capture_event();
                    }
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
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let area = self.pages_area(bounds);
        let position = state.position(self.duration);
        let offset = self.offset(area, position);
        let page_cursor = if cursor.is_over(area) {
            cursor - offset
        } else {
            mouse::Cursor::Unavailable
        };
        let page_viewport = Rectangle {
            x: area.x - offset.x,
            ..area
        };

        renderer.with_layer(area, |renderer| {
            renderer.with_translation(offset, |renderer| {
                for (index, (child, child_layout)) in
                    self.children.iter().zip(layout.children()).enumerate()
                {
                    if index >= self.count {
                        break;
                    }

                    if child_layout.bounds().intersects(&page_viewport) {
                        child.as_widget().draw(
                            &tree.children[index],
                            renderer,
                            theme,
                            style,
                            child_layout,
                            page_cursor,
                            &page_viewport,
                        );
                    }
                }
            });
        });

        // A layer of their own keeps the buttons, the indicator and the ring
        // above the layer of the pages.
        renderer.with_layer(*viewport, |renderer| {
            for (index, (child, child_layout)) in self
                .children
                .iter()
                .zip(layout.children())
                .enumerate()
                .skip(self.count)
            {
                child.as_widget().draw(
                    &tree.children[index],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            }

            if self.enabled() && state.keyboard && (state.focused || state.focus_inside) {
                crate::focus::draw_ring(renderer, theme, bounds, radius::LARGE);
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
        let state = tree.state.downcast_ref::<State>();
        let area = self.pages_area(layout.bounds());
        let offset = self.offset(area, state.position(self.duration));

        for (index, (child, child_layout)) in self
            .children
            .iter()
            .zip(layout.children())
            .enumerate()
            .skip(self.count)
        {
            let interaction = child.as_widget().mouse_interaction(
                &tree.children[index],
                child_layout,
                cursor,
                viewport,
                renderer,
            );

            if interaction != mouse::Interaction::None {
                return interaction;
            }
        }

        if !cursor.is_over(area) {
            return mouse::Interaction::None;
        }

        self.children
            .iter()
            .zip(layout.children())
            .enumerate()
            .take(self.count)
            .map(|(index, (child, child_layout))| {
                child.as_widget().mouse_interaction(
                    &tree.children[index],
                    child_layout,
                    cursor - offset,
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
        let page = self.page;
        let count = self.count;
        let overlays: Vec<_> = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
            .filter(|(index, _)| *index >= count || *index == page)
            .filter_map(|(_, ((child, tree), layout))| {
                child
                    .as_widget_mut()
                    .overlay(tree, layout, renderer, viewport, translation)
            })
            .collect();

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_wrap_or_clamp() {
        assert_eq!(step(0, 3, -1, true), Some((2, true)));
        assert_eq!(step(2, 3, 1, true), Some((0, true)));
        assert_eq!(step(1, 3, 1, true), Some((2, false)));
        assert_eq!(step(0, 3, -1, false), None);
        assert_eq!(step(2, 3, 1, false), None);
        assert_eq!(step(0, 0, 1, true), None);
        assert_eq!(step(0, 1, 1, true), None);
    }

    #[test]
    fn a_page_controller_keeps_the_selection_in_range() {
        let mut pages = PageController::new(3, Some(2));

        assert!(pages.has_selection());
        assert!(!pages.set_index(Some(3)));
        assert_eq!(pages.index(), Some(2));

        pages.set_length(2);
        assert_eq!(pages.index(), None);
        assert_eq!(PageController::new(2, Some(5)).index(), None);
    }

    #[test]
    fn go_to_slides_and_jump_to_does_not() {
        let mut controller = Controller::new().with_initial_page(1);

        controller.jump_to(2);
        assert!(!controller.animate);
        controller.go_to(0);
        assert!(controller.animate);
        assert_eq!(controller.page(), 0);
    }
}
