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

//! Toasts: short notifications stacked over the application by a
//! [`toaster`].
//!
//! The application owns a [`Toasts`] queue: [`Toasts::push`] adds a
//! [`Toast`] and returns its [`ToastId`], [`Toasts::replace`] changes it and
//! [`Toasts::dismiss`] closes it. The [`toaster`] wraps the root of the view,
//! draws the visible toasts over it on the inverse surface and publishes
//! [`Event`]s that the application hands back to [`Toasts::update`].
//!
//! At most [`Toasts::with_max_visible`] toasts show at a time, the oldest
//! nearest the edge of the [`Placement`]; newer ones wait for room. A toast
//! with a duration closes by itself once its time runs out; the time stops
//! while the pointer is over the stack or the focus is in a toast. A toast
//! slides in when it appears and out when it closes. Escape closes the
//! focused toast when it is dismissible.
//!
//! The toaster reads the clock from the redraw events of the window and
//! asks for a redraw only when a toast is moving or its time runs out, so
//! the application needs no subscription.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::widget::toast::{self, Toast, Toasts};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//!     Undo,
//!     Toast(toast::Event),
//! }
//!
//! struct App {
//!     toasts: Toasts<Message>,
//! }
//!
//! impl App {
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::Save => {
//!                 self.toasts
//!                     .push(Toast::new("Saved").with_action("Undo", Message::Undo));
//!             }
//!             Message::Undo => {}
//!             Message::Toast(event) => {
//!                 if let Some(action) = self.toasts.update(event) {
//!                     self.update(action);
//!                 }
//!             }
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         let content = iced::widget::text("Content");
//!         toast::toaster(content, &self.toasts, &Localizations::en(), Message::Toast).into()
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::widget::{Column, container, text};
use iced::{
    Color, Event as IcedEvent, Length, Padding, Point, Rectangle, Size, Vector, mouse, window,
};

use super::{button, progress, styled_text};
use crate::animation::ease_out_cubic;
use crate::focus::operations::has_focus;
use crate::l10n::Localizations;
use crate::theme::{HOVER_OPACITY, PRESSED_OPACITY, contrast, state_layer};
use crate::tokens::typography::TextStyle;
use crate::tokens::{radius, spacing, typography};
use crate::widget::info::InfoType;
use crate::{Element, Theme};

/// How long a toast stays when [`Toast::new`] creates it.
pub const DEFAULT_DURATION: Duration = Duration::from_secs(4);

/// The largest number of toasts a [`Toasts`] queue shows at a time by
/// default.
pub const DEFAULT_MAX_VISIBLE: usize = 3;

/// The largest width of the toast stack by default, in logical pixels.
pub const DEFAULT_MAX_WIDTH: f32 = 420.0;

/// How long a toast takes to slide in or out.
pub const TRANSITION: Duration = Duration::from_millis(220);

/// The size of the type icon and of the loading spinner.
const LEADING_SIZE: f32 = 18.0;

/// The size of the glyph of the dismiss button.
#[cfg(feature = "svg")]
const DISMISS_ICON_SIZE: f32 = 16.0;

/// The opacity of the description text over the toast content color.
const DESCRIPTION_OPACITY: f32 = 0.8;

/// The elevation of a toast surface.
const ELEVATION: f32 = 6.0;

/// The part of its height a toast travels while it slides in or out.
const SLIDE: f32 = 0.3;

/// Where a [`toaster`] stacks its toasts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Placement {
    /// The top start corner.
    TopStart,
    /// The top center.
    Top,
    /// The top end corner.
    TopEnd,
    /// The bottom start corner.
    BottomStart,
    /// The bottom center.
    #[default]
    Bottom,
    /// The bottom end corner.
    BottomEnd,
}

impl Placement {
    /// Every placement, in declaration order.
    pub const ALL: [Placement; 6] = [
        Placement::TopStart,
        Placement::Top,
        Placement::TopEnd,
        Placement::BottomStart,
        Placement::Bottom,
        Placement::BottomEnd,
    ];

    /// Whether the stack sits at the top edge, so toasts enter from above
    /// and the oldest sits on top.
    pub fn is_top(self) -> bool {
        matches!(
            self,
            Placement::TopStart | Placement::Top | Placement::TopEnd
        )
    }

    /// The horizontal position of a toast `width` wide in `area`.
    fn x(self, area: Rectangle, width: f32) -> f32 {
        match self {
            Placement::TopStart | Placement::BottomStart => area.x,
            Placement::Top | Placement::Bottom => area.center_x() - width / 2.0,
            Placement::TopEnd | Placement::BottomEnd => area.x + area.width - width,
        }
    }
}

/// A short notification shown by a [`toaster`].
#[derive(Debug, Clone, PartialEq)]
pub struct Toast<Message> {
    /// The message, shown in bold.
    pub title: String,
    /// Optional detail text shown below the title.
    pub description: Option<String>,
    /// The kind of message, shown as a colored icon; none when `None`.
    pub kind: Option<InfoType>,
    /// The label of the action button and the message it publishes; the
    /// toast closes after the action.
    pub action: Option<(String, Message)>,
    /// How long the toast stays; it stays until dismissed when `None`. The
    /// time does not run while the pointer or the focus is on a toast.
    pub duration: Option<Duration>,
    /// Whether the toast shows a spinner instead of an icon and stays until
    /// it is replaced or dismissed.
    pub loading: bool,
    /// Whether the toast has a dismiss button and closes on Escape.
    pub dismissible: bool,
}

impl<Message> Toast<Message> {
    /// A dismissible toast showing `title` for [`DEFAULT_DURATION`].
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            kind: None,
            action: None,
            duration: Some(DEFAULT_DURATION),
            loading: false,
            dismissible: true,
        }
    }

    /// Returns the toast with `description` below the title.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the toast with the icon of `kind`.
    #[must_use]
    pub fn with_kind(mut self, kind: InfoType) -> Self {
        self.kind = Some(kind);
        self
    }

    /// Returns the toast with an action button labeled `label` that
    /// publishes `message` and closes the toast.
    #[must_use]
    pub fn with_action(mut self, label: impl Into<String>, message: Message) -> Self {
        self.action = Some((label.into(), message));
        self
    }

    /// Returns the toast staying for `duration`, or until dismissed with
    /// `None`.
    #[must_use]
    pub fn with_duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }

    /// Returns the toast with a spinner when `loading`; a loading toast
    /// stays until it is replaced or dismissed.
    #[must_use]
    pub fn with_loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Returns the toast with or without its dismiss button and Escape.
    #[must_use]
    pub fn with_dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// The time after which the toast closes by itself, if any.
    fn timeout(&self) -> Option<Duration> {
        self.duration.filter(|_| !self.loading)
    }
}

/// The identifier of a toast in a [`Toasts`] queue, returned by
/// [`Toasts::push`] to replace or dismiss the toast later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToastId(u64);

/// What a [`toaster`] reports to the application, to hand to
/// [`Toasts::update`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// The toast was dismissed: by its button, by Escape, or because its
    /// time ran out.
    Dismiss(ToastId),
    /// The action button of the toast was pressed.
    Action(ToastId),
    /// The toast finished sliding out and can be dropped.
    Removed(ToastId),
}

/// A toast in the queue with its closing state.
#[derive(Debug, Clone)]
struct Entry<Message> {
    id: ToastId,
    toast: Toast<Message>,
    closing: bool,
    revision: u64,
}

/// The queue of toasts of an application, shown by a [`toaster`].
#[derive(Debug, Clone)]
pub struct Toasts<Message> {
    entries: Vec<Entry<Message>>,
    next_id: u64,
    placement: Placement,
    max_visible: usize,
    max_width: f32,
}

impl<Message> Default for Toasts<Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Message> Toasts<Message> {
    /// An empty queue showing up to [`DEFAULT_MAX_VISIBLE`] toasts at the
    /// bottom center, at most [`DEFAULT_MAX_WIDTH`] wide.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 0,
            placement: Placement::default(),
            max_visible: DEFAULT_MAX_VISIBLE,
            max_width: DEFAULT_MAX_WIDTH,
        }
    }

    /// Returns the queue stacking its toasts at `placement`.
    #[must_use]
    pub fn with_placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Returns the queue showing at most `max_visible` toasts at a time (at
    /// least one).
    #[must_use]
    pub fn with_max_visible(mut self, max_visible: usize) -> Self {
        self.max_visible = max_visible.max(1);
        self
    }

    /// Returns the queue with a stack at most `max_width` wide.
    #[must_use]
    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width.max(0.0);
        self
    }

    /// Where the toasts stack.
    pub fn placement(&self) -> Placement {
        self.placement
    }

    /// Sets where the toasts stack.
    pub fn set_placement(&mut self, placement: Placement) {
        self.placement = placement;
    }

    /// Queues `toast`, shown at once when there is room, and returns its
    /// identifier.
    pub fn push(&mut self, toast: Toast<Message>) -> ToastId {
        let id = ToastId(self.next_id);
        self.next_id += 1;
        self.entries.push(Entry {
            id,
            toast,
            closing: false,
            revision: 0,
        });
        id
    }

    /// Replaces the content of the toast `id` and restarts its time.
    /// Returns false, changing nothing, when the toast is dismissed.
    pub fn replace(&mut self, id: ToastId, toast: Toast<Message>) -> bool {
        match self.entries.iter_mut().find(|e| e.id == id && !e.closing) {
            Some(entry) => {
                entry.toast = toast;
                entry.revision += 1;
                true
            }
            None => false,
        }
    }

    /// Dismisses the toast `id`: a shown toast slides out, a waiting one is
    /// dropped at once. Returns false when it is already dismissed.
    pub fn dismiss(&mut self, id: ToastId) -> bool {
        let Some(index) = self.entries.iter().position(|e| e.id == id && !e.closing) else {
            return false;
        };

        if index < self.max_visible {
            self.entries[index].closing = true;
        } else {
            self.entries.remove(index);
        }

        true
    }

    /// Dismisses every toast, shown or waiting.
    pub fn dismiss_all(&mut self) {
        let max_visible = self.max_visible;
        let mut index = 0;
        self.entries.retain_mut(|entry| {
            let shown = index < max_visible;
            index += 1;
            entry.closing = true;
            shown
        });
    }

    /// Whether the toast `id` is shown or waiting, and not dismissed.
    pub fn is_active(&self, id: ToastId) -> bool {
        self.entries.iter().any(|e| e.id == id && !e.closing)
    }

    /// The toast `id`, while it is in the queue.
    pub fn get(&self, id: ToastId) -> Option<&Toast<Message>> {
        self.entries.iter().find(|e| e.id == id).map(|e| &e.toast)
    }

    /// The number of toasts shown or waiting, not dismissed.
    pub fn active_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.closing).count()
    }

    /// The entries that have room on screen, oldest first.
    fn visible(&self) -> &[Entry<Message>] {
        &self.entries[..self.entries.len().min(self.max_visible)]
    }
}

impl<Message: Clone> Toasts<Message> {
    /// Applies an event of the [`toaster`]. Returns the action message of
    /// the toast on [`Event::Action`], for the application to run; the
    /// toast is dismissed.
    pub fn update(&mut self, event: Event) -> Option<Message> {
        match event {
            Event::Dismiss(id) => {
                self.dismiss(id);
                None
            }
            Event::Action(id) => {
                let action = self
                    .entries
                    .iter()
                    .find(|e| e.id == id && !e.closing)
                    .and_then(|e| e.toast.action.as_ref())
                    .map(|(_, message)| message.clone());
                self.dismiss(id);
                action
            }
            Event::Removed(id) => {
                self.entries.retain(|e| e.id != id);
                None
            }
        }
    }
}

/// The color of the type icon of `kind` on the inverse surface: the status
/// color of the opposite brightness, raised to the non-text contrast.
#[cfg(feature = "svg")]
fn kind_color(kind: InfoType, theme: &Theme) -> Color {
    let roles = theme.roles();
    use crate::tokens::Status;

    let status = if theme.brightness() == crate::theme::Brightness::Light {
        Status::DARK
    } else {
        Status::LIGHT
    };
    let color = match kind {
        InfoType::Information => status.info,
        InfoType::Success => status.success,
        InfoType::Warning => status.warning,
        InfoType::Danger => status.error,
        InfoType::Important => roles.inverse_primary,
    };

    contrast::ensure(color, &[roles.inverse_surface], contrast::NON_TEXT_AA)
}

/// The style of a transparent button with `foreground` content on the
/// inverse surface.
fn inverse_button(
    status: iced::widget::button::Status,
    foreground: Color,
) -> iced::widget::button::Style {
    use iced::widget::button::Status as S;

    let background = match status {
        S::Hovered => Some(state_layer(Color::TRANSPARENT, foreground, HOVER_OPACITY)),
        S::Pressed => Some(state_layer(Color::TRANSPARENT, foreground, PRESSED_OPACITY)),
        S::Active | S::Disabled => None,
    };
    iced::widget::button::Style {
        background: background.map(iced::Background::Color),
        text_color: foreground,
        border: iced::Border::default().rounded(radius::MEDIUM),
        ..iced::widget::button::Style::default()
    }
}

/// The title style: the medium body text in the emphasis weight.
const TITLE: TextStyle = TextStyle {
    weight: typography::EMPHASIS_WEIGHT,
    ..typography::BODY_MEDIUM
};

/// The view of one toast: the inverse surface with the leading icon or
/// spinner, the texts, the action and the dismiss button.
fn toast_view<'a, Message: Clone + 'a>(
    id: ToastId,
    toast: &Toast<Message>,
    localizations: &Localizations,
    on_event: &dyn Fn(Event) -> Message,
) -> Element<'a, Message> {
    let mut children: Vec<Element<'a, Message>> = Vec::new();

    if toast.loading {
        children.push(
            progress::circular_indeterminate()
                .size(LEADING_SIZE)
                .class(crate::theme::progress_bar::Class::Custom(Box::new(
                    |theme: &Theme| iced::widget::progress_bar::Style {
                        background: iced::Background::Color(Color::TRANSPARENT),
                        bar: iced::Background::Color(theme.roles().on_inverse_surface),
                        border: iced::Border::default(),
                    },
                )))
                .into(),
        );
    } else if let Some(kind) = toast.kind {
        #[cfg(feature = "svg")]
        children.push(
            container(crate::icon::icon(kind.icon()).size(LEADING_SIZE))
                .style(move |theme: &Theme| container::Style {
                    text_color: Some(kind_color(kind, theme)),
                    ..container::Style::default()
                })
                .into(),
        );
        #[cfg(not(feature = "svg"))]
        let _ = kind;
    }

    let flexible = children.len();

    let mut texts = Column::new().push(styled_text(toast.title.clone(), TITLE));

    if let Some(description) = &toast.description {
        texts = texts.push(
            styled_text(description.clone(), typography::BODY_MEDIUM).style(|theme: &Theme| {
                text::Style {
                    color: Some(
                        theme
                            .roles()
                            .on_inverse_surface
                            .scale_alpha(DESCRIPTION_OPACITY),
                    ),
                }
            }),
        );
    }

    children.push(
        container(texts)
            .padding(Padding::from([spacing::XS, 0.0]))
            .into(),
    );

    if let Some((label, _)) = &toast.action {
        children.push(
            button::text(label.clone())
                .class(button::Class::Custom(Box::new(|theme: &Theme, status| {
                    let roles = theme.roles();
                    let color = contrast::ensure(
                        roles.inverse_primary,
                        &[roles.inverse_surface],
                        contrast::TEXT_AA,
                    );
                    inverse_button(status, color)
                })))
                .on_press(on_event(Event::Action(id)))
                .into(),
        );
    }

    if toast.dismissible {
        children.push(dismiss_button(localizations, on_event(Event::Dismiss(id))));
    }

    let row = FitRow {
        children,
        flexible,
        spacing: spacing::SM,
    };

    container(Element::new(row))
        .padding(Padding {
            top: spacing::SM,
            right: spacing::SM,
            bottom: spacing::SM,
            left: spacing::LG,
        })
        .style(|theme: &Theme| {
            let roles = theme.roles();
            container::Style {
                background: Some(iced::Background::Color(roles.inverse_surface)),
                text_color: Some(roles.on_inverse_surface),
                border: iced::Border::default().rounded(radius::LARGE),
                shadow: crate::theme::container::elevation(roles.shadow, ELEVATION),
                ..container::Style::default()
            }
        })
        .into()
}

/// The dismiss button of a toast: a square of the control height with the
/// close glyph and the localized "Dismiss" tooltip, publishing `message`.
fn dismiss_button<'a, Message: Clone + 'a>(
    localizations: &Localizations,
    message: Message,
) -> Element<'a, Message> {
    #[cfg(feature = "svg")]
    let content: Element<'a, Message> =
        container(crate::icon::icon(crate::icon::lucide::X).size(DISMISS_ICON_SIZE))
            .center(Length::Fixed(crate::density::control_height()))
            .into();
    #[cfg(not(feature = "svg"))]
    let content: Element<'a, Message> = container(styled_text("\u{00D7}", typography::LABEL_LARGE))
        .center(Length::Fixed(crate::density::control_height()))
        .into();

    button::button(content)
        .padding(0.0)
        .class(button::Class::Custom(Box::new(|theme: &Theme, status| {
            inverse_button(status, theme.roles().on_inverse_surface)
        })))
        .tooltip(localizations.dismiss)
        .on_press(message)
        .into()
}

/// A shown toast of a [`Toaster`].
struct Item<'a, Message> {
    id: ToastId,
    revision: u64,
    closing: bool,
    timeout: Option<Duration>,
    dismissible: bool,
    element: Element<'a, Message>,
}

/// Wraps `content`, usually the whole view, and shows the visible toasts of
/// `toasts` over it, in the language of `localizations`. The toaster fills
/// the available space. The toaster
/// publishes `on_event` with the [`Event`]s to hand to
/// [`Toasts::update`].
pub fn toaster<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    toasts: &Toasts<Message>,
    localizations: &Localizations,
    on_event: impl Fn(Event) -> Message + 'a,
) -> Toaster<'a, Message> {
    let items = toasts
        .visible()
        .iter()
        .map(|entry| Item {
            id: entry.id,
            revision: entry.revision,
            closing: entry.closing,
            timeout: entry.toast.timeout(),
            dismissible: entry.toast.dismissible,
            element: toast_view(entry.id, &entry.toast, localizations, &on_event),
        })
        .collect();

    Toaster {
        content: content.into(),
        items,
        placement: toasts.placement,
        max_width: toasts.max_width,
        on_event: Box::new(on_event),
    }
}

/// The root element that shows toasts over the application. See
/// [`toaster`].
pub struct Toaster<'a, Message> {
    content: Element<'a, Message>,
    items: Vec<Item<'a, Message>>,
    placement: Placement,
    max_width: f32,
    on_event: Box<dyn Fn(Event) -> Message + 'a>,
}

/// The motion of a toast.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    /// Fully shown.
    Shown,
    /// Sliding in since the instant, or waiting for the next frame.
    Entering(Option<Instant>),
    /// Sliding out since the instant, or waiting for the next frame.
    Leaving(Option<Instant>),
    /// Slid out; the removal was reported when `true`.
    Gone(bool),
}

impl Phase {
    /// The visible part of the toast at `now`, from 0 to 1.
    fn visibility(self, now: Option<Instant>) -> f32 {
        let progress = |start: Instant| {
            let elapsed = now.map_or(Duration::ZERO, |now| now.saturating_duration_since(start));
            (elapsed.as_secs_f32() / TRANSITION.as_secs_f32()).clamp(0.0, 1.0)
        };

        match self {
            Phase::Shown | Phase::Leaving(None) => 1.0,
            Phase::Entering(None) | Phase::Gone(_) => 0.0,
            Phase::Entering(Some(start)) => ease_out_cubic(progress(start)),
            Phase::Leaving(Some(start)) => ease_out_cubic(1.0 - progress(start)),
        }
    }
}

/// The motion and the time of a toast.
#[derive(Debug, Clone, Copy)]
struct Track {
    phase: Phase,
    revision: u64,
    closing: bool,
    remaining: Option<Duration>,
    since: Option<Instant>,
    expired: bool,
}

impl Track {
    /// The track of `item`, shown at once when `initial`.
    fn new<Message>(item: &Item<'_, Message>, initial: bool) -> Self {
        let phase = match (initial, item.closing) {
            (true, true) => Phase::Gone(false),
            (true, false) => Phase::Shown,
            (false, _) => Phase::Entering(None),
        };

        Self {
            phase,
            revision: item.revision,
            closing: item.closing,
            remaining: None,
            since: None,
            expired: false,
        }
    }

    /// Follows the current properties of `item`: starts sliding out when
    /// it closes and restarts the time when its content changed.
    fn follow<Message>(&mut self, item: &Item<'_, Message>) {
        if item.closing && !self.closing {
            self.closing = true;
            self.phase = match self.phase {
                Phase::Entering(None) => Phase::Gone(false),
                Phase::Gone(reported) => Phase::Gone(reported),
                _ => Phase::Leaving(None),
            };
        }

        if item.revision != self.revision {
            self.revision = item.revision;
            self.remaining = None;
            self.since = None;
            self.expired = false;
        }
    }
}

/// The state of a toaster.
#[derive(Debug, Default)]
struct State {
    now: Option<Instant>,
    ids: Vec<ToastId>,
    tracks: HashMap<ToastId, Track>,
    hovered: bool,
    focused: bool,
}

impl State {
    /// Follows the shown `items`, creating the tracks of new toasts and
    /// dropping the tracks of the toasts that left.
    fn sync<Message>(&mut self, items: &[Item<'_, Message>], initial: bool) {
        for item in items {
            self.tracks
                .entry(item.id)
                .and_modify(|track| track.follow(item))
                .or_insert_with(|| Track::new(item, initial));
        }

        self.tracks
            .retain(|id, _| items.iter().any(|item| item.id == *id));
        self.ids = items.iter().map(|item| item.id).collect();
    }

    /// The phase of the toast `id`.
    fn phase(&self, id: ToastId) -> Phase {
        self.tracks
            .get(&id)
            .map_or(Phase::Shown, |track| track.phase)
    }
}

impl<'a, Message: Clone + 'a> Toaster<'a, Message> {
    /// Advances the motion and the time of the toasts to `now` and asks for
    /// the next redraw.
    fn tick(&self, state: &mut State, now: Instant, shell: &mut Shell<'_, Message>) {
        state.now = Some(now);
        let paused = state.hovered || state.focused;
        let mut moving = false;
        let mut deadline: Option<Instant> = None;

        for item in &self.items {
            let Some(track) = state.tracks.get_mut(&item.id) else {
                continue;
            };

            let before = track.phase;
            track.phase = match track.phase {
                Phase::Entering(None) => Phase::Entering(Some(now)),
                Phase::Leaving(None) => Phase::Leaving(Some(now)),
                Phase::Entering(Some(start))
                    if now.saturating_duration_since(start) >= TRANSITION =>
                {
                    Phase::Shown
                }
                Phase::Leaving(Some(start))
                    if now.saturating_duration_since(start) >= TRANSITION =>
                {
                    Phase::Gone(false)
                }
                phase => phase,
            };

            if let Phase::Gone(false) = track.phase {
                track.phase = Phase::Gone(true);
                shell.publish((self.on_event)(Event::Removed(item.id)));
            }

            moving |= track.phase != before
                || matches!(track.phase, Phase::Entering(_) | Phase::Leaving(_));

            if track.closing || track.expired {
                continue;
            }

            let Some(timeout) = item.timeout else {
                continue;
            };

            let remaining = *track.remaining.get_or_insert(timeout);

            if paused {
                if let Some(since) = track.since.take() {
                    track.remaining =
                        Some(remaining.saturating_sub(now.saturating_duration_since(since)));
                }
            } else {
                let since = *track.since.get_or_insert(now);

                if now.saturating_duration_since(since) >= remaining {
                    track.expired = true;
                    shell.publish((self.on_event)(Event::Dismiss(item.id)));
                } else {
                    let end = since + remaining;
                    deadline = Some(deadline.map_or(end, |d| d.min(end)));
                }
            }
        }

        if moving {
            shell.invalidate_layout();
            shell.request_redraw();
        } else if let Some(deadline) = deadline {
            shell.request_redraw_at(deadline);
        }
    }

    /// Whether `cursor` is over a toast slot of `layout`.
    fn over_toasts(layout: Layout<'_>, cursor: mouse::Cursor) -> bool {
        layout
            .children()
            .skip(1)
            .any(|slot| cursor.is_over(slot.bounds()))
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for Toaster<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let mut state = State::default();
        state.sync(&self.items, true);
        tree::State::new(state)
    }

    fn children(&self) -> Vec<Tree> {
        std::iter::once(Tree::new(&self.content))
            .chain(self.items.iter().map(|item| Tree::new(&item.element)))
            .collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();
        let old_ids = std::mem::take(&mut state.ids);
        state.sync(&self.items, false);

        let mut old: Vec<Option<Tree>> = std::mem::take(&mut tree.children)
            .into_iter()
            .map(Some)
            .collect();

        let mut content = old
            .first_mut()
            .and_then(Option::take)
            .unwrap_or_else(|| Tree::new(&self.content));
        content.diff(&self.content);

        let mut children = vec![content];

        for item in &self.items {
            let reused = old_ids
                .iter()
                .position(|id| *id == item.id)
                .and_then(|index| old.get_mut(index + 1))
                .and_then(Option::take);

            let child = match reused {
                Some(mut child) => {
                    child.diff(&item.element);
                    child
                }
                None => Tree::new(&item.element),
            };

            children.push(child);
        }

        tree.children = children;
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
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_ref::<State>();
        let Some((content_tree, toast_trees)) = children.split_first_mut() else {
            return layout::Node::new(Size::ZERO);
        };

        let content = self
            .content
            .as_widget_mut()
            .layout(content_tree, renderer, limits);
        let size = limits.resolve(Length::Fill, Length::Fill, content.size());
        let area = Rectangle::new(Point::ORIGIN, size).shrink(spacing::MD);
        let max_width = self.max_width.min(area.width).max(0.0);
        let top = self.placement.is_top();

        let mut nodes = vec![content];
        let mut offset = 0.0;

        for (item, child_tree) in self.items.iter_mut().zip(toast_trees.iter_mut()) {
            let toast_limits =
                layout::Limits::new(Size::ZERO, Size::new(max_width, area.height.max(0.0)));
            let node = item
                .element
                .as_widget_mut()
                .layout(child_tree, renderer, &toast_limits);
            let toast = node.size();
            let full = toast.height + 2.0 * spacing::XS;
            let visibility = state.phase(item.id).visibility(state.now);
            let height = full * visibility;
            let slide = SLIDE * full * (1.0 - visibility);

            let (slot_y, toast_y) = if top {
                (area.y + offset, height - spacing::XS - toast.height - slide)
            } else {
                (area.y + area.height - offset - height, spacing::XS + slide)
            };
            offset += height;

            let slot = layout::Node::with_children(
                Size::new(toast.width, height),
                vec![node.move_to(Point::new(0.0, toast_y))],
            )
            .move_to(Point::new(self.placement.x(area, toast.width), slot_y));

            nodes.push(slot);
        }

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            let mut layouts = layout.children();
            let mut trees = tree.children.iter_mut();

            if let (Some(content_layout), Some(content_tree)) = (layouts.next(), trees.next()) {
                self.content.as_widget_mut().operate(
                    content_tree,
                    content_layout,
                    renderer,
                    operation,
                );
            }

            for ((item, child_tree), slot) in self.items.iter_mut().zip(trees).zip(layouts) {
                if let Some(toast_layout) = slot.children().next() {
                    item.element.as_widget_mut().operate(
                        child_tree,
                        toast_layout,
                        renderer,
                        operation,
                    );
                }
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
        let over_toasts = Self::over_toasts(layout, cursor);

        {
            let Tree {
                state, children, ..
            } = &mut *tree;
            let state = state.downcast_mut::<State>();

            let slots: Vec<Layout<'_>> = layout.children().skip(1).collect();

            for index in (0..self.items.len()).rev() {
                let (Some(item), Some(child_tree), Some(toast_layout)) = (
                    self.items.get_mut(index),
                    children.get_mut(index + 1),
                    slots.get(index).and_then(|slot| slot.children().next()),
                ) else {
                    continue;
                };

                item.element.as_widget_mut().update(
                    child_tree,
                    event,
                    toast_layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                );

                if shell.is_event_captured() {
                    break;
                }
            }

            let mut focused = false;

            for ((item, child_tree), slot) in self
                .items
                .iter_mut()
                .zip(children.iter_mut().skip(1))
                .zip(layout.children().skip(1))
            {
                let Some(toast_layout) = slot.children().next() else {
                    continue;
                };

                let has = crate::focus::run(
                    &mut item.element,
                    child_tree,
                    toast_layout,
                    renderer,
                    has_focus(),
                )
                .unwrap_or(false);

                if has
                    && !shell.is_event_captured()
                    && item.dismissible
                    && !item.closing
                    && is_escape(event)
                {
                    shell.publish((self.on_event)(Event::Dismiss(item.id)));
                    shell.capture_event();
                }

                focused |= has;
            }

            if focused != state.focused || over_toasts != state.hovered {
                state.focused = focused;
                state.hovered = over_toasts;
                shell.request_redraw();
            }

            if let IcedEvent::Window(window::Event::RedrawRequested(now)) = event {
                self.tick(state, *now, shell);
            }
        }

        if shell.is_event_captured() {
            return;
        }

        let content_cursor = if over_toasts {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };

        if let (Some(content_layout), Some(content_tree)) =
            (layout.children().next(), tree.children.first_mut())
        {
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

        let state = tree.state.downcast_ref::<State>();
        let over_toasts = Self::over_toasts(layout, cursor);
        let mut layouts = layout.children();

        if let (Some(content_layout), Some(content_tree)) = (layouts.next(), tree.children.first())
        {
            let content_cursor = if over_toasts {
                mouse::Cursor::Unavailable
            } else {
                cursor
            };

            self.content.as_widget().draw(
                content_tree,
                renderer,
                theme,
                style,
                content_layout,
                content_cursor,
                viewport,
            );
        }

        if self.items.is_empty() {
            return;
        }

        renderer.with_layer(*viewport, |renderer| {
            for ((item, child_tree), slot) in self
                .items
                .iter()
                .zip(tree.children.iter().skip(1))
                .zip(layouts)
            {
                let Some(toast_layout) = slot.children().next() else {
                    continue;
                };

                let draw = |renderer: &mut iced::Renderer| {
                    item.element.as_widget().draw(
                        child_tree,
                        renderer,
                        theme,
                        style,
                        toast_layout,
                        cursor,
                        viewport,
                    );
                };

                if state.phase(item.id) == Phase::Shown {
                    draw(renderer);
                } else if let Some(clip) = slot.bounds().intersection(viewport) {
                    renderer.with_layer(clip, draw);
                }
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
        let content_layout = layouts.next();

        for ((item, child_tree), slot) in self
            .items
            .iter()
            .zip(tree.children.iter().skip(1))
            .zip(layouts)
        {
            if cursor.is_over(slot.bounds())
                && let Some(toast_layout) = slot.children().next()
            {
                return item.element.as_widget().mouse_interaction(
                    child_tree,
                    toast_layout,
                    cursor,
                    viewport,
                    renderer,
                );
            }
        }

        match (content_layout, tree.children.first()) {
            (Some(content_layout), Some(content_tree)) => self
                .content
                .as_widget()
                .mouse_interaction(content_tree, content_layout, cursor, viewport, renderer),
            _ => mouse::Interaction::default(),
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
        let mut layouts = layout.children();
        let (content_tree, toast_trees) = tree.children.split_first_mut()?;
        let content_layout = layouts.next()?;

        let mut overlays = Vec::new();

        if let Some(overlay) = self.content.as_widget_mut().overlay(
            content_tree,
            content_layout,
            renderer,
            viewport,
            translation,
        ) {
            overlays.push(overlay);
        }

        for ((item, child_tree), slot) in self
            .items
            .iter_mut()
            .zip(toast_trees.iter_mut())
            .zip(layouts)
        {
            if let Some(toast_layout) = slot.children().next()
                && let Some(overlay) = item.element.as_widget_mut().overlay(
                    child_tree,
                    toast_layout,
                    renderer,
                    viewport,
                    translation,
                )
            {
                overlays.push(overlay);
            }
        }

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

impl<'a, Message: Clone + 'a> From<Toaster<'a, Message>> for Element<'a, Message> {
    fn from(toaster: Toaster<'a, Message>) -> Self {
        Element::new(toaster)
    }
}

/// Returns whether `event` is a press of Escape.
fn is_escape(event: &IcedEvent) -> bool {
    matches!(
        event,
        IcedEvent::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::Escape),
            ..
        })
    )
}

/// A row that lays out every child but the flexible one first, gives the
/// flexible child the width left, centers the children vertically and
/// shrinks to its content.
struct FitRow<'a, Message> {
    children: Vec<Element<'a, Message>>,
    flexible: usize,
    spacing: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for FitRow<'_, Message> {
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
        let max = limits.max();
        let count = self.children.len();
        let mut used = self.spacing * count.saturating_sub(1) as f32;
        let mut nodes = vec![layout::Node::default(); count];

        let order = (0..count)
            .filter(|index| *index != self.flexible)
            .chain((self.flexible < count).then_some(self.flexible));

        for index in order {
            let child_limits = layout::Limits::new(
                Size::ZERO,
                Size::new((max.width - used).max(0.0), max.height),
            );
            let node = self.children[index].as_widget_mut().layout(
                &mut tree.children[index],
                renderer,
                &child_limits,
            );
            used += node.size().width;
            nodes[index] = node;
        }

        let height = nodes
            .iter()
            .map(|node| node.size().height)
            .fold(0.0, f32::max);
        let mut x = 0.0;

        for node in &mut nodes {
            let size = node.size();
            node.move_to_mut(Point::new(x, (height - size.height) / 2.0));
            x += size.width + self.spacing;
        }

        let width = (x - self.spacing).max(0.0);

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
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((child, state), child_layout) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                child
                    .as_widget_mut()
                    .operate(state, child_layout, renderer, operation);
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
        for ((child, state), child_layout) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state,
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
        for ((child, state), child_layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
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
            .map(|((child, state), child_layout)| {
                child
                    .as_widget()
                    .mouse_interaction(state, child_layout, cursor, viewport, renderer)
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

    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Undo,
    }

    #[test]
    fn push_returns_distinct_ids_and_dismiss_closes_shown_toasts() {
        let mut toasts = Toasts::<Message>::new().with_max_visible(1);
        let first = toasts.push(Toast::new("One"));
        let second = toasts.push(Toast::new("Two"));

        assert_ne!(first, second);
        assert_eq!(toasts.active_count(), 2);

        assert!(toasts.dismiss(first));
        assert!(!toasts.is_active(first));
        assert!(
            toasts.get(first).is_some(),
            "a shown toast waits for its exit"
        );
        assert!(!toasts.dismiss(first));

        assert!(toasts.dismiss(second));
        assert!(
            toasts.get(second).is_none(),
            "a waiting toast leaves at once"
        );

        toasts.update(Event::Removed(first));
        assert!(toasts.get(first).is_none());
    }

    #[test]
    fn the_action_event_returns_the_message_and_dismisses() {
        let mut toasts = Toasts::new();
        let id = toasts.push(Toast::new("Deleted").with_action("Undo", Message::Undo));

        assert_eq!(toasts.update(Event::Action(id)), Some(Message::Undo));
        assert!(!toasts.is_active(id));
        assert_eq!(toasts.update(Event::Action(id)), None);
    }

    #[test]
    fn replace_changes_only_active_toasts() {
        let mut toasts = Toasts::<Message>::new();
        let id = toasts.push(Toast::new("Saving").with_loading(true));

        assert!(toasts.replace(id, Toast::new("Saved")));
        assert_eq!(toasts.get(id).map(|t| t.title.as_str()), Some("Saved"));

        toasts.dismiss(id);
        assert!(!toasts.replace(id, Toast::new("Again")));
    }

    #[test]
    fn dismiss_all_keeps_only_the_shown_toasts_for_their_exit() {
        let mut toasts = Toasts::<Message>::new().with_max_visible(2);
        for title in ["A", "B", "C"] {
            toasts.push(Toast::new(title));
        }

        toasts.dismiss_all();

        assert_eq!(toasts.active_count(), 0);
        assert_eq!(toasts.visible().len(), 2);
    }

    #[test]
    fn loading_toasts_have_no_timeout() {
        assert_eq!(Toast::<Message>::new("A").timeout(), Some(DEFAULT_DURATION));
        assert_eq!(
            Toast::<Message>::new("A").with_loading(true).timeout(),
            None
        );
        assert_eq!(
            Toast::<Message>::new("A").with_duration(None).timeout(),
            None
        );
    }

    #[test]
    fn the_visibility_follows_the_phase() {
        let start = Instant::now();
        let half = Some(start + TRANSITION / 2);

        assert!((Phase::Shown.visibility(None) - 1.0).abs() < f32::EPSILON);
        assert!(Phase::Gone(true).visibility(None).abs() < f32::EPSILON);

        let entering = Phase::Entering(Some(start)).visibility(half);
        let leaving = Phase::Leaving(Some(start)).visibility(half);
        assert!(entering > 0.5 && entering < 1.0);
        assert!(leaving > 0.0 && leaving < 1.0);

        let done = Some(start + TRANSITION);
        assert!((Phase::Entering(Some(start)).visibility(done) - 1.0).abs() < f32::EPSILON);
        assert!(Phase::Leaving(Some(start)).visibility(done).abs() < f32::EPSILON);
    }

    #[test]
    fn placements_split_into_top_and_bottom() {
        let top: Vec<_> = Placement::ALL.into_iter().filter(|p| p.is_top()).collect();
        assert_eq!(
            top,
            vec![Placement::TopStart, Placement::Top, Placement::TopEnd]
        );
    }
}
