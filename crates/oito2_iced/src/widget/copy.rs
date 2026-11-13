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

//! Copy buttons and copy fields.
//!
//! A [`copy_button`] is an icon button that writes its text to the standard
//! clipboard on a click, Enter or Space. After a copy its icon turns into a
//! check mark in the success tone and its tooltip into the localized
//! "Copied" for the feedback duration, then both reset; a copy during the
//! feedback restarts it. The keyboard focus stays on the button across the
//! change. The feedback is timed by the redraw events of the window, so a
//! button that was never pressed draws the same every time.
//!
//! Iced writes to the clipboard without reporting whether the platform
//! accepted the text, so the button always shows the copied state; there
//! is no failure state.
//!
//! A [`copy_field`] is a read-only text field showing a value, optionally
//! hidden behind bullets, with a copy button at its end. Its text can be
//! focused, selected and copied with the keyboard; edits are discarded.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::widget::copy;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Copied,
//! }
//!
//! # fn view<'a>(localizations: &Localizations, link: &'a str) -> oito2_iced::Element<'a, Message> {
//! copy::copy_field(localizations, link)
//!     .label("Share link")
//!     .on_copied(Message::Copied)
//!     .into()
//! # }
//! ```

#[cfg(feature = "svg")]
pub use enabled::*;

#[cfg(feature = "svg")]
mod enabled {
    use std::borrow::Cow;
    use std::marker::PhantomData;
    use std::time::Duration;

    use iced::advanced::clipboard::Kind;
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::widget::operation::{self, Operation};
    use iced::advanced::widget::{Id, Tree, Widget, tree};
    use iced::advanced::{Clipboard, Shell, overlay, renderer};
    use iced::time::Instant;
    use iced::widget::container;
    use iced::widget::text::IntoFragment;
    use iced::{Event, Length, Padding, Rectangle, Size, Vector, mouse, window};

    use crate::icon::{icon, lucide};
    use crate::l10n::Localizations;
    use crate::theme::button::Class;
    use crate::widget::{Minimum, min_size};
    use crate::widget::{button, text_field};
    use crate::{Element, Theme};

    /// How long the check mark and the "Copied" tooltip show by default.
    pub const FEEDBACK_DURATION: Duration = Duration::from_millis(1500);

    type Renderer = iced::Renderer;

    /// An icon button that copies `text` to the clipboard, with the
    /// localized "Copy" tooltip.
    pub fn copy_button<'a, Message>(
        localizations: &Localizations,
        text: impl Into<String>,
    ) -> CopyButton<'a, Message> {
        CopyButton {
            text: text.into(),
            tooltip: Cow::Borrowed(localizations.material.copy),
            copied: localizations.copied,
            on_copied: None,
            feedback: FEEDBACK_DURATION,
            id: None,
        }
    }

    /// A copy button. See [`copy_button`].
    pub struct CopyButton<'a, Message> {
        text: String,
        tooltip: Cow<'a, str>,
        copied: &'static str,
        on_copied: Option<Message>,
        feedback: Duration,
        id: Option<Id>,
    }

    impl<'a, Message> CopyButton<'a, Message> {
        /// Sets the tooltip shown before a copy, instead of the localized
        /// "Copy".
        #[must_use]
        pub fn tooltip(mut self, label: impl IntoFragment<'a>) -> Self {
            self.tooltip = label.into_fragment();
            self
        }

        /// Sets the message published after each copy.
        #[must_use]
        pub fn on_copied(mut self, message: Message) -> Self {
            self.on_copied = Some(message);
            self
        }

        /// Sets the message published after each copy, or none.
        #[must_use]
        pub fn on_copied_maybe(mut self, message: Option<Message>) -> Self {
            self.on_copied = message;
            self
        }

        /// Sets how long the check mark and the "Copied" tooltip show
        /// ([`FEEDBACK_DURATION`] by default).
        #[must_use]
        pub fn feedback_duration(mut self, duration: Duration) -> Self {
            self.feedback = duration;
            self
        }

        /// Sets the widget [`Id`] of the button.
        #[must_use]
        pub fn id(mut self, id: impl Into<Id>) -> Self {
            self.id = Some(id.into());
            self
        }
    }

    /// The message of the inner icon buttons: a press.
    #[derive(Debug, Clone, Copy)]
    struct Press;

    /// An icon button that publishes [`Press`], with `glyph` in the color
    /// given by `color` (the label color of the button when `None`).
    fn press_button<'a>(
        glyph: crate::icon::Icon,
        color: Option<fn(&Theme) -> iced::Color>,
        tooltip: Cow<'a, str>,
        id: Option<Id>,
    ) -> Element<'a, Press> {
        let mut glyph = container(icon(glyph));

        if let Some(color) = color {
            glyph = glyph.class(crate::theme::container::Class::Custom(Box::new(
                move |theme: &Theme| container::Style::default().color(color(theme)),
            )));
        }

        let mut button = button::button(min_size(
            glyph,
            Minimum::Control(0.0),
            Minimum::Control(0.0),
        ))
        .class(Class::Icon)
        .padding(Padding::ZERO)
        .tooltip(tooltip)
        .on_press(Press);

        if let Some(id) = id {
            button = button.id(id);
        }

        button.into()
    }

    impl<'a, Message: Clone + 'a> From<CopyButton<'a, Message>> for Element<'a, Message> {
        fn from(button: CopyButton<'a, Message>) -> Self {
            let idle = press_button(lucide::COPY, None, button.tooltip, button.id.clone());
            let copied = press_button(
                lucide::CHECK,
                Some(|theme: &Theme| theme.roles().semantic.success),
                Cow::Borrowed(button.copied),
                button.id,
            );

            Element::new(Copier {
                children: [idle, copied],
                text: button.text,
                on_copied: button.on_copied,
                feedback: button.feedback,
            })
        }
    }

    /// The widget behind [`CopyButton`]: the idle button and the copied
    /// button, laid out on top of each other, with only the one of the
    /// current state drawn and operated.
    struct Copier<'a, Message> {
        children: [Element<'a, Press>; 2],
        text: String,
        on_copied: Option<Message>,
        feedback: Duration,
    }

    /// The copied state of a [`Copier`].
    #[derive(Debug, Clone, Copy, Default)]
    struct State {
        /// Whether the copied button shows.
        copied: bool,
        /// When the copied button started to show; `None` until the first
        /// redraw after a copy.
        since: Option<Instant>,
    }

    impl State {
        /// The index of the child of the current state.
        fn active(self) -> usize {
            usize::from(self.copied)
        }
    }

    impl<Message: Clone> Copier<'_, Message> {
        /// Shows the child of the `copied` state, moving the keyboard focus
        /// from the other child to it.
        fn show(
            &mut self,
            state: &mut State,
            trees: &mut [Tree],
            layout: Layout<'_>,
            renderer: &Renderer,
            copied: bool,
        ) {
            let from = state.active();
            let to = usize::from(copied);

            state.copied = copied;
            state.since = None;

            if from == to {
                return;
            }

            let layouts: Vec<Layout<'_>> = layout.children().collect();
            let (Some(from_layout), Some(to_layout)) = (layouts.get(from), layouts.get(to)) else {
                return;
            };

            let mut unfocus = SetFocus {
                focus: false,
                found: false,
            };
            self.children[from].as_widget_mut().operate(
                &mut trees[from],
                *from_layout,
                renderer,
                &mut unfocus,
            );

            if unfocus.found {
                self.children[to].as_widget_mut().operate(
                    &mut trees[to],
                    *to_layout,
                    renderer,
                    &mut SetFocus {
                        focus: true,
                        found: false,
                    },
                );
            }
        }
    }

    /// An operation that focuses or unfocuses every focusable widget it
    /// visits and records whether any of them had the focus.
    struct SetFocus {
        focus: bool,
        found: bool,
    }

    impl<T> Operation<T> for SetFocus {
        fn focusable(
            &mut self,
            _id: Option<&Id>,
            _bounds: Rectangle,
            state: &mut dyn operation::Focusable,
        ) {
            self.found |= state.is_focused();

            if self.focus {
                state.focus();
            } else {
                state.unfocus();
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    impl<Message: Clone> Widget<Message, Theme, Renderer> for Copier<'_, Message> {
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
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            let nodes: Vec<layout::Node> = self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .map(|(child, tree)| child.as_widget_mut().layout(tree, renderer, limits))
                .collect();

            let size = nodes.iter().fold(Size::ZERO, |size, node| {
                Size::new(
                    size.width.max(node.size().width),
                    size.height.max(node.size().height),
                )
            });

            layout::Node::with_children(size, nodes)
        }

        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn Operation,
        ) {
            let active = tree.state.downcast_ref::<State>().active();

            if let Some(child_layout) = layout.children().nth(active) {
                self.children[active].as_widget_mut().operate(
                    &mut tree.children[active],
                    child_layout,
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
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) {
            let mut state = *tree.state.downcast_ref::<State>();

            if let Event::Window(window::Event::RedrawRequested(now)) = event
                && state.copied
            {
                let until = *state.since.get_or_insert(*now) + self.feedback;

                if *now >= until {
                    self.show(&mut state, &mut tree.children, layout, renderer, false);
                    shell.request_redraw();
                } else {
                    shell.request_redraw_at(until);
                }
            }

            let active = state.active();
            let mut pressed = false;

            for (index, ((child, child_tree), child_layout)) in self
                .children
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .enumerate()
            {
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

                if index == active {
                    apply(shell, &local);
                    drop(local);
                    pressed |= !messages.is_empty();
                } else {
                    shell.request_redraw_at(local.redraw_request());
                }
            }

            if pressed {
                clipboard.write(Kind::Standard, self.text.clone());
                self.show(&mut state, &mut tree.children, layout, renderer, true);

                if let Some(message) = &self.on_copied {
                    shell.publish(message.clone());
                }

                shell.request_redraw();
            }

            *tree.state.downcast_mut::<State>() = state;
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            let active = tree.state.downcast_ref::<State>().active();

            if let Some(child_layout) = layout.children().nth(active) {
                self.children[active].as_widget().draw(
                    &tree.children[active],
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
            renderer: &Renderer,
        ) -> mouse::Interaction {
            let active = tree.state.downcast_ref::<State>().active();

            layout
                .children()
                .nth(active)
                .map(|child_layout| {
                    self.children[active].as_widget().mouse_interaction(
                        &tree.children[active],
                        child_layout,
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
            renderer: &Renderer,
            viewport: &Rectangle,
            translation: Vector,
        ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
            let active = tree.state.downcast_ref::<State>().active();
            let child_layout = layout.children().nth(active)?;

            self.children[active]
                .as_widget_mut()
                .overlay(
                    &mut tree.children[active],
                    child_layout,
                    renderer,
                    viewport,
                    translation,
                )
                .map(silent)
        }
    }

    /// Applies the requests of `local` (event capture, redraw, layout and
    /// widget invalidation, input method) to `shell`, without its messages.
    fn apply<Inner, Message>(shell: &mut Shell<'_, Message>, local: &Shell<'_, Inner>) {
        if local.is_event_captured() {
            shell.capture_event();
        }

        shell.request_redraw_at(local.redraw_request());

        if local.is_layout_invalid() {
            shell.invalidate_layout();
        }

        if local.are_widgets_invalid() {
            shell.invalidate_widgets();
        }

        shell.request_input_method(local.input_method());
    }

    /// Wraps an overlay of an inner message type, such as a tooltip, in one
    /// that discards its messages.
    fn silent<'b, Inner: 'b, Message: 'b>(
        content: overlay::Element<'b, Inner, Theme, Renderer>,
    ) -> overlay::Element<'b, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(Silent {
            content,
            _message: PhantomData,
        }))
    }

    /// An overlay that forwards everything to its content and discards the
    /// messages the content publishes.
    struct Silent<'b, Inner, Message> {
        content: overlay::Element<'b, Inner, Theme, Renderer>,
        _message: PhantomData<fn() -> Message>,
    }

    impl<'b, Inner: 'b, Message: 'b> overlay::Overlay<Message, Theme, Renderer>
        for Silent<'b, Inner, Message>
    {
        fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
            self.content.as_overlay_mut().layout(renderer, bounds)
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
        ) {
            self.content
                .as_overlay()
                .draw(renderer, theme, style, layout, cursor);
        }

        fn operate(
            &mut self,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn Operation,
        ) {
            self.content
                .as_overlay_mut()
                .operate(layout, renderer, operation);
        }

        fn update(
            &mut self,
            event: &Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
        ) {
            let mut messages = Vec::new();
            let mut local = Shell::new(&mut messages);

            self.content
                .as_overlay_mut()
                .update(event, layout, cursor, renderer, clipboard, &mut local);

            apply(shell, &local);
        }

        fn mouse_interaction(
            &self,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content
                .as_overlay()
                .mouse_interaction(layout, cursor, renderer)
        }

        fn overlay<'c>(
            &'c mut self,
            layout: Layout<'c>,
            renderer: &Renderer,
        ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
            self.content
                .as_overlay_mut()
                .overlay(layout, renderer)
                .map(silent)
        }

        fn index(&self) -> f32 {
            self.content.as_overlay().index()
        }
    }

    /// A read-only text field showing `value` with a copy button at its
    /// end that copies it.
    pub fn copy_field<'a, Message>(
        localizations: &Localizations,
        value: &'a str,
    ) -> CopyField<'a, Message> {
        CopyField {
            value,
            localizations: *localizations,
            label: None,
            obscure: false,
            on_copied: None,
            id: None,
            width: Length::Fill,
        }
    }

    /// A copy field. See [`copy_field`].
    pub struct CopyField<'a, Message> {
        value: &'a str,
        localizations: Localizations,
        label: Option<Cow<'a, str>>,
        obscure: bool,
        on_copied: Option<Message>,
        id: Option<Id>,
        width: Length,
    }

    impl<'a, Message> CopyField<'a, Message> {
        /// Shows `label` above the field.
        #[must_use]
        pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
            self.label = Some(label.into_fragment());
            self
        }

        /// Shows the value as bullets, such as for a secret; the copy button
        /// still copies the value.
        #[must_use]
        pub fn obscure(mut self, obscure: bool) -> Self {
            self.obscure = obscure;
            self
        }

        /// Sets the message published after each copy.
        #[must_use]
        pub fn on_copied(mut self, message: Message) -> Self {
            self.on_copied = Some(message);
            self
        }

        /// Sets the widget [`Id`] of the input.
        #[must_use]
        pub fn id(mut self, id: impl Into<Id>) -> Self {
            self.id = Some(id.into());
            self
        }

        /// Sets the width (the available width by default).
        #[must_use]
        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.width = width.into();
            self
        }
    }

    impl<'a, Message: Clone + 'a> From<CopyField<'a, Message>> for Element<'a, Message> {
        fn from(field: CopyField<'a, Message>) -> Self {
            let copy = copy_button::<Option<Message>>(&field.localizations, field.value)
                .on_copied_maybe(field.on_copied.map(Some));

            let mut input = text_field::text_field::<Option<Message>>("", field.value)
                .on_input(|_| None)
                .secure(field.obscure)
                .trailing(copy)
                .width(field.width);

            if let Some(label) = field.label {
                input = input.label(label);
            }

            if let Some(id) = field.id {
                input = input.id(id);
            }

            Element::new(ReadOnly {
                content: input.into(),
            })
        }
    }

    /// The widget behind [`CopyField`]: publishes the messages its content
    /// wraps in `Some` and discards the edits, which its content publishes
    /// as `None`, rebuilding the view so the field shows its value again.
    struct ReadOnly<'a, Message> {
        content: Element<'a, Option<Message>>,
    }

    impl<Message> Widget<Message, Theme, Renderer> for ReadOnly<'_, Message> {
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
            renderer: &Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        }

        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn Operation,
        ) {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        }

        fn update(
            &mut self,
            tree: &mut Tree,
            event: &Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) {
            let mut messages = Vec::new();
            let mut local = Shell::new(&mut messages);

            self.content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                &mut local,
                viewport,
            );

            apply(shell, &local);
            drop(local);

            for message in messages {
                match message {
                    Some(message) => shell.publish(message),
                    None => shell.invalidate_widgets(),
                }
            }
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }

        fn mouse_interaction(
            &self,
            tree: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }

        fn overlay<'b>(
            &'b mut self,
            tree: &'b mut Tree,
            layout: Layout<'b>,
            renderer: &Renderer,
            viewport: &Rectangle,
            translation: Vector,
        ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
            self.content
                .as_widget_mut()
                .overlay(
                    &mut tree.children[0],
                    layout,
                    renderer,
                    viewport,
                    translation,
                )
                .map(silent)
        }
    }
}
