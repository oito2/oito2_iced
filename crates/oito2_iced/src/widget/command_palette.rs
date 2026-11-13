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

//! Command palettes: a modal search field above the [commands] of the
//! application, each with its shortcut.
//!
//! [`command_palette`] wraps the root of the view. While open, it dims the
//! view behind a scrim, which receives no input, and shows a panel with a
//! search field and the commands that match the query (see
//! [`matching_commands`]). The focus stays in the panel: Tab cycles through
//! it. The arrow keys move the highlight while the focus stays in the field;
//! Enter or a click publishes the close message and then the action of the
//! command; Escape on an empty field, Escape outside the field and a click
//! on the scrim publish the close message.
//!
//! The application keeps whether the palette is open and its query, and
//! opens it with a shortcut through
//! [`SequenceMatcher`](crate::shortcuts::SequenceMatcher).
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::shortcuts::Command;
//! use oito2_iced::widget::command_palette;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Query(String),
//!     Close,
//!     NewFile,
//! }
//!
//! # fn view<'a>(open: bool, query: &str) -> oito2_iced::Element<'a, Message> {
//! let commands = [Command::new("file.new", "New file", Message::NewFile)];
//! let commands: Vec<&Command<Message>> = commands.iter().collect();
//!
//! command_palette::command_palette(
//!     iced::widget::text("Content"),
//!     &Localizations::en(),
//!     &commands,
//!     query,
//!     Message::Query,
//!     Message::Close,
//! )
//! .open(open)
//! .into()
//! # }
//! ```
//!
//! [commands]: crate::shortcuts::Command

#![cfg_attr(not(feature = "svg"), allow(dead_code))]

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Event, Length, Rectangle, Size, Vector, mouse};

use crate::collections::fold_text;
use crate::focus::operations::{focus_first, focus_last, has_focus};
use crate::shortcuts::Command;
use crate::{Element, Theme};

#[cfg(feature = "svg")]
pub use self::palette::{CommandPalette, command_palette};

/// The largest width of the palette panel, in logical pixels.
pub const MAX_WIDTH: f32 = 560.0;

/// The largest height of the palette panel, in logical pixels.
pub const MAX_HEIGHT: f32 = 440.0;

/// Returns the enabled `commands` that contain every word of `query` in
/// their label, description, group or keywords, ignoring case and accents,
/// best match first.
///
/// Commands whose label starts with the first word come first, then those
/// whose label contains every word, then the rest; commands of equal rank
/// keep their order. An empty query returns every enabled command.
pub fn matching_commands<'c, M>(commands: &[&'c Command<M>], query: &str) -> Vec<&'c Command<M>> {
    let folded = fold_text(query);
    let words: Vec<&str> = folded.split_whitespace().collect();
    let mut ranked: Vec<(u8, &'c Command<M>)> = Vec::new();

    for command in commands.iter().copied().filter(|c| c.enabled) {
        let label = fold_text(&command.label);
        let mut text = command.label.clone();

        for part in [&command.description, &command.group]
            .into_iter()
            .flatten()
            .chain(&command.keywords)
        {
            text.push(' ');
            text.push_str(part);
        }

        let text = fold_text(&text);

        if !words.iter().all(|word| text.contains(word)) {
            continue;
        }

        let rank = match words.first() {
            None => 1,
            Some(first) if label.starts_with(first) => 0,
            Some(_) if words.iter().all(|word| label.contains(word)) => 1,
            Some(_) => 2,
        };

        ranked.push((rank, command));
    }

    ranked.sort_by_key(|(rank, _)| *rank);
    ranked.into_iter().map(|(_, command)| command).collect()
}

/// Returns the highlight after moving `step` rows from `current` among
/// `count` rows, wrapping around at both ends.
fn step_highlight(current: usize, step: isize, count: usize) -> usize {
    if count == 0 {
        return 0;
    }

    let count = count as isize;
    let current = current.min(count as usize - 1) as isize;

    (current + step).rem_euclid(count) as usize
}

/// Returns the step of an arrow key press: one row down or up.
fn arrow_step(event: &Event) -> Option<isize> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(named),
            modifiers,
            ..
        }) if modifiers.is_empty() => match named {
            key::Named::ArrowDown => Some(1),
            key::Named::ArrowUp => Some(-1),
            _ => None,
        },
        _ => None,
    }
}

/// Returns whether `event` is a press of `named`, and whether Shift is
/// held.
fn pressed(event: &Event, named: key::Named) -> Option<bool> {
    match event {
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key),
            modifiers,
            ..
        }) if *key == named => Some(modifiers.shift()),
        _ => None,
    }
}

#[cfg(feature = "svg")]
mod palette {
    use std::rc::Rc;

    use iced::advanced::widget::Id;
    use iced::widget::text::{self, LineHeight};
    use iced::widget::{Column, Row, Space, Stack, container, mouse_area, opaque, scrollable};
    use iced::{Alignment, Background, Border, Length, Padding};

    use super::{
        MAX_HEIGHT, MAX_WIDTH, Modal, Reveal, arrow_step, matching_commands, step_highlight,
    };
    use crate::density::{self, Density};
    use crate::l10n::Localizations;
    use crate::shortcuts::{Command, OperatingSystem, Shortcut, uses_command_key};
    use crate::theme::text::Class as TextClass;
    use crate::tokens::{radius, spacing, typography};
    use crate::widget::highlighted_text::highlighted_text;
    use crate::widget::list_tile::row_style;
    use crate::widget::search::host::{Effects, Host, Msg, Reconcile, Signal, Target};
    use crate::widget::search::search_field;
    use crate::widget::{Minimum, focusable, min_size, styled_text};
    use crate::{Element, Theme};

    /// The opacity of the scrim over the view behind the palette.
    const SCRIM_OPACITY: f32 = 138.0 / 255.0;

    /// The opacity of the primary color behind the highlighted command.
    const HIGHLIGHT_OPACITY: f32 = 0.12;

    /// The space between the palette panel and the window edges.
    const INSET: Padding = Padding {
        top: spacing::XL,
        right: 40.0,
        bottom: spacing::XL,
        left: 40.0,
    };

    /// The smallest width of a key cap of a shortcut label.
    const KEY_CAP_MIN_WIDTH: f32 = 20.0;

    /// The widget id of the highlighted command row.
    const HIGHLIGHT_ID: &str = "oito2-command-palette-highlight";

    /// A command as the palette shows it.
    struct Entry<Message> {
        label: String,
        subtitle: Option<String>,
        shortcut: Option<Shortcut>,
        action: Message,
    }

    impl<Message: Clone> Entry<Message> {
        fn new(command: &Command<Message>) -> Self {
            let subtitle = [&command.group, &command.description]
                .into_iter()
                .flatten()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(" \u{b7} ");

            Self {
                label: command.label.clone(),
                subtitle: (!subtitle.is_empty()).then_some(subtitle),
                shortcut: command.shortcut.clone(),
                action: command.action.clone(),
            }
        }
    }

    /// The own events of the palette panel.
    #[derive(Debug, Clone, Copy)]
    enum Own {
        /// Runs the command at the index.
        Run(usize),
    }

    /// The state of the palette panel: the highlighted row, the query it
    /// belongs to and whether the field got the first focus.
    #[derive(Debug, Default)]
    struct PanelState {
        highlight: usize,
        query: Option<String>,
    }

    /// A command palette over the view. See [`command_palette`].
    pub struct CommandPalette<'a, Message> {
        content: Element<'a, Message>,
        localizations: Localizations,
        entries: Vec<Entry<Message>>,
        query: String,
        on_query: Rc<dyn Fn(String) -> Message + 'a>,
        on_close: Message,
        open: bool,
        os: OperatingSystem,
    }

    /// Wraps `content`, usually the whole view, with a command palette over
    /// the enabled `commands` that match `query`, in the language of
    /// `localizations`. The search field publishes `on_query` with the new
    /// text; the palette publishes `on_close` when it closes, followed by
    /// the action of the command it runs. It shows only while
    /// [`CommandPalette::open`] is true. The wrapper fills the available
    /// space.
    pub fn command_palette<'a, Message: Clone + 'a>(
        content: impl Into<Element<'a, Message>>,
        localizations: &Localizations,
        commands: &[&Command<Message>],
        query: &str,
        on_query: impl Fn(String) -> Message + 'a,
        on_close: Message,
    ) -> CommandPalette<'a, Message> {
        CommandPalette {
            content: content.into(),
            localizations: *localizations,
            entries: matching_commands(commands, query)
                .into_iter()
                .map(Entry::new)
                .collect(),
            query: query.to_owned(),
            on_query: Rc::new(on_query),
            on_close,
            open: false,
            os: OperatingSystem::current(),
        }
    }

    impl<Message> CommandPalette<'_, Message> {
        /// Shows the palette when `open` (closed by default).
        #[must_use]
        pub fn open(mut self, open: bool) -> Self {
            self.open = open;
            self
        }

        /// Sets the system whose notation the shortcut labels use (the
        /// current one by default).
        #[must_use]
        pub fn os(mut self, os: OperatingSystem) -> Self {
            self.os = os;
            self
        }
    }

    /// The label of `shortcut` on `os`: one key cap per chord on Apple
    /// systems, one per key elsewhere.
    fn shortcut_label<'a, M: 'a>(shortcut: &Shortcut, os: OperatingSystem) -> Element<'a, M> {
        let apple = uses_command_key(os);
        let cap = |label: String| -> Element<'a, M> {
            container(min_size(
                styled_text(label, typography::LABEL_SMALL).style(|theme: &Theme| text::Style {
                    color: Some(theme.roles().on_surface_variant),
                }),
                Minimum::Fixed(KEY_CAP_MIN_WIDTH - 2.0 * spacing::XS),
                Minimum::None,
            ))
            .padding(Padding::from([1.0, spacing::XS]))
            .style(|theme: &Theme| {
                let roles = theme.roles();
                container::Style {
                    background: Some(Background::Color(roles.surface_container_high)),
                    border: Border::default()
                        .color(roles.outline_variant)
                        .width(1.0)
                        .rounded(radius::SMALL),
                    ..container::Style::default()
                }
            })
            .into()
        };

        let mut chords = Row::new().spacing(spacing::SM).align_y(Alignment::Center);

        for chord in shortcut.chords() {
            let parts = if apple {
                vec![chord.label(os)]
            } else {
                chord.parts(os)
            };

            chords = chords.push(
                Row::with_children(parts.into_iter().map(cap))
                    .spacing(spacing::XXS)
                    .align_y(Alignment::Center),
            );
        }

        chords.into()
    }

    /// The row of the command `entry` at `index`, highlighted when
    /// `highlighted`.
    fn command_row<'a, M: Clone + 'a>(
        index: usize,
        entry: &Entry<M>,
        query: &str,
        highlighted: bool,
        os: OperatingSystem,
    ) -> Element<'a, Msg<M, Own>> {
        let foreground = move |theme: &Theme| {
            let roles = theme.roles();
            if highlighted {
                roles.indicator()
            } else {
                roles.on_surface
            }
        };
        let secondary = move |theme: &Theme| {
            let roles = theme.roles();
            if highlighted {
                roles.indicator()
            } else {
                roles.on_surface_variant
            }
        };

        let mut texts = Column::new().width(Length::Fill).push(
            highlighted_text(entry.label.clone(), query)
                .style(typography::BODY_LARGE)
                .class(TextClass::Custom(Box::new(move |theme: &Theme| {
                    text::Style {
                        color: Some(foreground(theme)),
                    }
                }))),
        );

        if let Some(subtitle) = &entry.subtitle {
            texts = texts.push(
                styled_text(subtitle.clone(), typography::BODY_MEDIUM)
                    .line_height(LineHeight::Absolute(
                        typography::BODY_MEDIUM.line_height.into(),
                    ))
                    .style(move |theme: &Theme| text::Style {
                        color: Some(secondary(theme)),
                    }),
            );
        }

        let mut content = Row::new()
            .push(texts)
            .spacing(spacing::LG)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        if let Some(shortcut) = &entry.shortcut {
            content = content.push(shortcut_label(shortcut, os));
        }

        let density = density::get();
        let horizontal = match density {
            Density::Pointer => spacing::MD,
            Density::Touch => spacing::LG,
        };
        let min_height = crate::widget::list_tile::min_height(density, entry.subtitle.is_some());
        let content = min_size(
            content,
            Minimum::None,
            Minimum::Fixed((min_height - 2.0 * spacing::XS).max(0.0)),
        );

        let message = Msg::Own(Own::Run(index));
        let native = iced::widget::Button::new(content)
            .padding(Padding::from([spacing::XS, horizontal]))
            .width(Length::Fill)
            .on_press(message.clone())
            .class(crate::theme::button::Class::Custom(Box::new(
                move |theme, status| {
                    let roles = theme.roles();
                    let base = highlighted.then(|| roles.primary.scale_alpha(HIGHLIGHT_OPACITY));
                    row_style(theme, status, base, secondary(theme), true, radius::MEDIUM)
                },
            )));

        let row: Element<'a, Msg<M, Own>> = focusable(native)
            .radius(radius::MEDIUM)
            .on_activate(message)
            .into();

        if highlighted {
            container(row).id(Id::new(HIGHLIGHT_ID)).into()
        } else {
            row
        }
    }

    /// The panel of the palette: the search field above the matching
    /// commands, or the "no commands" note.
    fn panel<'a, M: Clone + 'a>(
        localizations: &Localizations,
        entries: &Rc<Vec<Entry<M>>>,
        query: &str,
        highlight: usize,
        on_query: &Rc<dyn Fn(String) -> M + 'a>,
        on_close: &M,
        os: OperatingSystem,
    ) -> Element<'a, Msg<M, Own>> {
        let on_query = Rc::clone(on_query);
        let field = search_field::<Msg<M, Own>>(localizations, query)
            .placeholder(localizations.command_palette_hint)
            .on_input(move |text| Msg::App(on_query(text)))
            .on_escape(Msg::App(on_close.clone()));
        let field = if entries.is_empty() {
            field
        } else {
            field.on_submit(Msg::Own(Own::Run(highlight)))
        };

        let mut column = Column::new()
            .push(container(field).padding(spacing::MD))
            .width(Length::Fill);

        if entries.is_empty() {
            column = column.push(
                container(
                    styled_text(localizations.no_commands_found, typography::BODY_MEDIUM).style(
                        |theme: &Theme| text::Style {
                            color: Some(theme.roles().on_surface_variant),
                        },
                    ),
                )
                .padding(spacing::XL),
            );
        } else {
            let rows =
                Column::with_children(entries.iter().enumerate().map(|(index, entry)| {
                    command_row(index, entry, query, index == highlight, os)
                }))
                .padding(Padding {
                    bottom: spacing::SM,
                    ..Padding::ZERO
                })
                .width(Length::Fill);

            let list = scrollable(rows).width(Length::Fill).height(Length::Shrink);

            column = column.push(Element::new(Reveal {
                content: list.into(),
                target: Id::new(HIGHLIGHT_ID),
                key: (highlight, entries.len()),
                first: highlight == 0,
                last: highlight + 1 == entries.len(),
            }));
        }

        container(column)
            .width(Length::Fill)
            .max_width(MAX_WIDTH)
            .max_height(MAX_HEIGHT)
            .class(crate::theme::container::Class::Dialog)
            .clip(true)
            .into()
    }

    impl<'a, Message: Clone + 'a> From<CommandPalette<'a, Message>> for Element<'a, Message> {
        fn from(palette: CommandPalette<'a, Message>) -> Self {
            if !palette.open {
                return Element::new(Modal::new(palette.content, None, None));
            }

            let entries = Rc::new(palette.entries);
            let localizations = palette.localizations;
            let query = palette.query;
            let on_query = palette.on_query;
            let on_close = palette.on_close;
            let os = palette.os;

            let view = {
                let entries = Rc::clone(&entries);
                let query = query.clone();
                let on_close = on_close.clone();
                move |state: &PanelState| {
                    let highlight = state.highlight.min(entries.len().saturating_sub(1));
                    panel(
                        &localizations,
                        &entries,
                        &query,
                        highlight,
                        &on_query,
                        &on_close,
                        os,
                    )
                }
            };

            let handler = {
                let entries = Rc::clone(&entries);
                let on_close = on_close.clone();
                move |state: &mut PanelState,
                      signal: Signal<'_, Own>,
                      effects: &mut Effects<Message>| {
                    match signal {
                        Signal::Own(Own::Run(index)) => {
                            if let Some(entry) = entries.get(index) {
                                effects.publish(on_close.clone());
                                effects.publish(entry.action.clone());
                            }
                        }
                        Signal::Before(event, _) => {
                            if let Some(step) = arrow_step(event)
                                && !entries.is_empty()
                            {
                                state.highlight =
                                    step_highlight(state.highlight, step, entries.len());
                                effects.rebuild();
                                effects.capture();
                            }
                        }
                        _ => {}
                    }
                }
            };

            let host = Host::new(view, handler).reconcile(move |state: &mut PanelState, _| {
                let first = state.query.is_none();
                let changed = state.query.as_deref() != Some(query.as_str());

                if changed {
                    state.query = Some(query.clone());
                    state.highlight = 0;
                }

                Reconcile {
                    rebuild: changed,
                    focus: first.then_some(Target::Input),
                }
            });

            let scrim = mouse_area(
                container(Space::new())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .style(|theme: &Theme| container::Style {
                        background: Some(Background::Color(
                            theme.roles().scrim.scale_alpha(SCRIM_OPACITY),
                        )),
                        ..container::Style::default()
                    }),
            )
            .on_press(on_close.clone());

            let positioned = container(
                Column::new()
                    .push(Space::new().height(Length::FillPortion(1)))
                    .push(opaque(Element::new(host)))
                    .push(Space::new().height(Length::FillPortion(4)))
                    .align_x(Alignment::Center)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .padding(INSET)
            .width(Length::Fill)
            .height(Length::Fill);

            let layer = Stack::new()
                .push(scrim)
                .push(positioned)
                .width(Length::Fill)
                .height(Length::Fill);

            Element::new(Modal::new(
                palette.content,
                Some(layer.into()),
                Some(on_close),
            ))
        }
    }
}

/// A root wrapper that, while it has a layer, draws the layer over the
/// content and hands it every event: the content receives no input, Tab
/// and Shift+Tab cycle the focus inside the layer, and an Escape the layer
/// does not handle publishes the escape message.
struct Modal<'a, Message> {
    content: Element<'a, Message>,
    layer: Option<Element<'a, Message>>,
    on_escape: Option<Message>,
}

impl<'a, Message> Modal<'a, Message> {
    #[cfg_attr(not(feature = "svg"), allow(dead_code))]
    fn new(
        content: Element<'a, Message>,
        layer: Option<Element<'a, Message>>,
        on_escape: Option<Message>,
    ) -> Self {
        Self {
            content,
            layer,
            on_escape,
        }
    }

    /// The elements of the wrapper, content first.
    fn elements(&self) -> Vec<&Element<'a, Message>> {
        std::iter::once(&self.content).chain(&self.layer).collect()
    }
}

/// Whether the modal showed its layer at the last layout.
#[derive(Debug, Default)]
struct ModalState {
    open: bool,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Modal<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ModalState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ModalState::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.elements().into_iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        let elements: Vec<&Element<'_, Message>> = self.elements();
        tree.diff_children(&elements);
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
        let state = state.downcast_mut::<ModalState>();
        let mut trees = children.iter_mut();
        let Some(content_tree) = trees.next() else {
            return layout::Node::new(Size::ZERO);
        };

        let content = self
            .content
            .as_widget_mut()
            .layout(content_tree, renderer, limits);
        let size = limits.resolve(Length::Fill, Length::Fill, content.size());
        let open = self.layer.is_some();

        if open && !state.open {
            let _ = crate::focus::run(
                &mut self.content,
                content_tree,
                Layout::new(&content),
                renderer,
                operation::focusable::unfocus::<()>(),
            );
        }

        state.open = open;

        let mut nodes = vec![content];

        if let (Some(layer), Some(layer_tree)) = (self.layer.as_mut(), trees.next()) {
            nodes.push(layer.as_widget_mut().layout(
                layer_tree,
                renderer,
                &layout::Limits::new(Size::ZERO, size),
            ));
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
        let mut layouts = layout.children();
        let content_layout = layouts.next();
        let layer_layout = layouts.next();

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| match (&mut self.layer, layer_layout) {
            (Some(layer), Some(layer_layout)) => {
                if let Some(layer_tree) = tree.children.get_mut(1) {
                    layer
                        .as_widget_mut()
                        .operate(layer_tree, layer_layout, renderer, operation);
                }
            }
            _ => {
                if let (Some(content_layout), Some(content_tree)) =
                    (content_layout, tree.children.first_mut())
                {
                    self.content.as_widget_mut().operate(
                        content_tree,
                        content_layout,
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
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let mut layouts = layout.children();
        let content_layout = layouts.next();

        // Behind the layer the content still receives the window events,
        // such as redraws, without the cursor, so it keeps its look.
        if self.layer.is_some()
            && matches!(event, Event::Window(_))
            && let (Some(content_layout), Some(content_tree)) =
                (content_layout, tree.children.first_mut())
        {
            self.content.as_widget_mut().update(
                content_tree,
                event,
                content_layout,
                mouse::Cursor::Unavailable,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let (Some(layer), Some(layer_layout), Some(layer_tree)) = (
            self.layer.as_mut(),
            layouts.next(),
            tree.children.get_mut(1),
        ) else {
            if let (Some(content_layout), Some(content_tree)) =
                (content_layout, tree.children.first_mut())
            {
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
            }
            return;
        };

        layer.as_widget_mut().update(
            layer_tree,
            event,
            layer_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        if let Some(backward) = pressed(event, key::Named::Tab) {
            let moved = if backward {
                crate::focus::run(
                    layer,
                    layer_tree,
                    layer_layout,
                    renderer,
                    operation::focusable::focus_previous::<()>(),
                );
                crate::focus::run(layer, layer_tree, layer_layout, renderer, has_focus())
            } else {
                crate::focus::run(
                    layer,
                    layer_tree,
                    layer_layout,
                    renderer,
                    operation::focusable::focus_next::<()>(),
                );
                crate::focus::run(layer, layer_tree, layer_layout, renderer, has_focus())
            };

            if moved != Some(true) {
                if backward {
                    crate::focus::run(
                        layer,
                        layer_tree,
                        layer_layout,
                        renderer,
                        focus_last::<()>(),
                    );
                } else {
                    crate::focus::run(
                        layer,
                        layer_tree,
                        layer_layout,
                        renderer,
                        focus_first::<()>(),
                    );
                }
            }

            shell.capture_event();
            shell.request_redraw();
        } else if pressed(event, key::Named::Escape).is_some()
            && let Some(message) = &self.on_escape
        {
            shell.publish(message.clone());
            shell.capture_event();
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

        let mut layouts = layout.children();
        let open = self.layer.is_some();

        if let (Some(content_layout), Some(content_tree)) = (layouts.next(), tree.children.first())
        {
            let content_cursor = if open {
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

        if let (Some(layer), Some(layer_layout), Some(layer_tree)) =
            (&self.layer, layouts.next(), tree.children.get(1))
        {
            renderer.with_layer(*viewport, |renderer| {
                layer.as_widget().draw(
                    layer_tree,
                    renderer,
                    theme,
                    style,
                    layer_layout,
                    cursor,
                    viewport,
                );
            });
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
        let mut layouts = layout.children();
        let content_layout = layouts.next();

        match (&self.layer, layouts.next(), tree.children.get(1)) {
            (Some(layer), Some(layer_layout), Some(layer_tree)) => layer
                .as_widget()
                .mouse_interaction(layer_tree, layer_layout, cursor, viewport, renderer),
            _ => match (content_layout, tree.children.first()) {
                (Some(content_layout), Some(content_tree)) => self
                    .content
                    .as_widget()
                    .mouse_interaction(content_tree, content_layout, cursor, viewport, renderer),
                _ => mouse::Interaction::default(),
            },
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
        let content_layout = layouts.next()?;
        let (content_tree, rest) = tree.children.split_first_mut()?;

        match (&mut self.layer, layouts.next(), rest.first_mut()) {
            (Some(layer), Some(layer_layout), Some(layer_tree)) => layer.as_widget_mut().overlay(
                layer_tree,
                layer_layout,
                renderer,
                viewport,
                translation,
            ),
            _ => self.content.as_widget_mut().overlay(
                content_tree,
                content_layout,
                renderer,
                viewport,
                translation,
            ),
        }
    }
}

/// Wraps the scrollable list of the palette and, whenever the highlighted
/// row changes, scrolls the list so the row is visible: the first row
/// scrolls to the top, the last to the bottom, and any other row by the
/// least distance.
struct Reveal<'a, Message> {
    content: Element<'a, Message>,
    target: iced::advanced::widget::Id,
    key: (usize, usize),
    first: bool,
    last: bool,
}

/// The highlight that the list was last scrolled to.
#[derive(Debug, Default)]
struct RevealState {
    revealed: Option<(usize, usize)>,
}

/// Finds the first scrollable of a composition and the bounds of the
/// container with the target id.
struct FindRow {
    target: iced::advanced::widget::Id,
    scroll: Option<(Rectangle, Rectangle, Vector)>,
    row: Option<Rectangle>,
}

impl Operation for FindRow {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn container(&mut self, id: Option<&iced::advanced::widget::Id>, bounds: Rectangle) {
        if id == Some(&self.target) {
            self.row = Some(bounds);
        }
    }

    fn scrollable(
        &mut self,
        _id: Option<&iced::advanced::widget::Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn operation::Scrollable,
    ) {
        self.scroll
            .get_or_insert((bounds, content_bounds, translation));
    }
}

/// Scrolls the first scrollable of a composition to a vertical offset.
struct ScrollTo {
    offset: f32,
    done: bool,
}

impl Operation for ScrollTo {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&iced::advanced::widget::Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _translation: Vector,
        state: &mut dyn operation::Scrollable,
    ) {
        if !self.done {
            self.done = true;
            state.scroll_to(operation::scrollable::AbsoluteOffset {
                x: None,
                y: Some(self.offset),
            });
        }
    }
}

/// The offset that shows the row at `row` (relative to the list content)
/// inside a viewport `height` tall scrolled to `scrolled`, or `None` when
/// the row is already visible.
fn reveal_offset(row_top: f32, row_height: f32, scrolled: f32, height: f32) -> Option<f32> {
    if row_top < scrolled {
        Some(row_top)
    } else if row_top + row_height > scrolled + height {
        Some(row_top + row_height - height)
    } else {
        None
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Reveal<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<RevealState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(RevealState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
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
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<RevealState>();
        let Some(child) = children.first_mut() else {
            return layout::Node::new(Size::ZERO);
        };

        let node = self.content.as_widget_mut().layout(child, renderer, limits);

        if state.revealed != Some(self.key) {
            state.revealed = Some(self.key);

            let mut find = FindRow {
                target: self.target.clone(),
                scroll: None,
                row: None,
            };
            self.content
                .as_widget_mut()
                .operate(child, Layout::new(&node), renderer, &mut find);

            if let (Some((bounds, content, translation)), Some(row)) = (find.scroll, find.row) {
                let overflow = (content.height - bounds.height).max(0.0);
                let offset = if self.first {
                    Some(0.0)
                } else if self.last {
                    Some(overflow)
                } else {
                    reveal_offset(row.y - content.y, row.height, translation.y, bounds.height)
                };

                if let Some(offset) = offset {
                    let mut scroll = ScrollTo {
                        offset: offset.clamp(0.0, overflow),
                        done: false,
                    };
                    self.content.as_widget_mut().operate(
                        child,
                        Layout::new(&node),
                        renderer,
                        &mut scroll,
                    );
                }
            }
        }

        layout::Node::with_children(node.size(), vec![node])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().operate(
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                content_layout,
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
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
        layout
            .children()
            .next()
            .map(|content_layout| {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
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
        let content_layout = layout.children().next()?;

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn commands() -> Vec<Command<u8>> {
        vec![
            Command::new("open", "Open file", 0).with_group("File"),
            Command::new("settings", "Preferences", 1).with_keywords(["settings", "options"]),
            Command::new("reopen", "Reopen closed tab", 2),
            Command::new("hidden", "Open hidden", 3).with_enabled(false),
            Command::new("accent", "Ação rápida", 4).with_description("Executa algo"),
        ]
    }

    fn labels(query: &str) -> Vec<String> {
        let commands = commands();
        let refs: Vec<&Command<u8>> = commands.iter().collect();
        matching_commands(&refs, query)
            .into_iter()
            .map(|c| c.label.clone())
            .collect()
    }

    #[test]
    fn an_empty_query_keeps_every_enabled_command_in_order() {
        assert_eq!(
            labels(""),
            [
                "Open file",
                "Preferences",
                "Reopen closed tab",
                "Ação rápida"
            ]
        );
    }

    #[test]
    fn label_prefixes_rank_first() {
        assert_eq!(labels("open"), ["Open file", "Reopen closed tab"]);
        assert_eq!(labels("tab reopen"), ["Reopen closed tab"]);
    }

    #[test]
    fn keywords_group_description_and_accents_match() {
        assert_eq!(labels("options"), ["Preferences"]);
        assert_eq!(labels("file"), ["Open file"]);
        assert_eq!(labels("acao executa"), ["Ação rápida"]);
        assert!(labels("missing").is_empty());
    }

    #[test]
    fn words_found_only_outside_the_label_rank_last() {
        let commands = [
            Command::new("a", "Close window", 0).with_keywords(["file"]),
            Command::new("b", "File close", 1),
        ];
        let refs: Vec<&Command<u8>> = commands.iter().collect();
        let found: Vec<u8> = matching_commands(&refs, "close file")
            .into_iter()
            .map(|c| c.action)
            .collect();

        assert_eq!(found, [0, 1], "both labels start with or contain the words");

        let found: Vec<u8> = matching_commands(&refs, "file")
            .into_iter()
            .map(|c| c.action)
            .collect();
        assert_eq!(found, [1, 0]);
    }

    #[test]
    fn the_highlight_wraps_around() {
        assert_eq!(step_highlight(0, -1, 3), 2);
        assert_eq!(step_highlight(2, 1, 3), 0);
        assert_eq!(step_highlight(1, 1, 3), 2);
        assert_eq!(step_highlight(9, 1, 3), 0);
        assert_eq!(step_highlight(0, 1, 0), 0);
    }

    #[test]
    fn reveal_scrolls_by_the_least_distance() {
        assert_eq!(reveal_offset(0.0, 40.0, 0.0, 100.0), None);
        assert_eq!(reveal_offset(120.0, 40.0, 0.0, 100.0), Some(60.0));
        assert_eq!(reveal_offset(20.0, 40.0, 50.0, 100.0), Some(20.0));
    }
}
