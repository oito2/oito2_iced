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

//! Code blocks: highlighted, read-only code on the container surface.
//!
//! A [`code_block`] shows code in JetBrains Mono, 13 pixels with a 1.45
//! line height, colored with the syntax colors of the theme, on the
//! `surface_container` color inside a 1-pixel divider border with the
//! medium radius and 16 pixels of padding. Long lines scroll sideways
//! instead of wrapping. Line numbers, in the comment color, are optional;
//! the copy button in the top end corner copies the whole code and can be
//! hidden.
//!
//! The code is tokenized once and kept in a cache shared by every block
//! (see [`highlight_cached`]), so building the view again costs one cache
//! lookup, which compares the code with the cached texts. The colors are
//! applied when the block is drawn, so a theme change recolors it without
//! tokenizing again.
//!
//! Register the fonts of [`JETBRAINS_MONO_FONTS`](crate::font::JETBRAINS_MONO_FONTS) with
//! the application, or the code falls back to a system font.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_syntax::code_block;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Copied,
//! }
//!
//! # fn view<'a>(localizations: &Localizations, code: &'a str) -> oito2_iced::Element<'a, Message> {
//! code_block::code_block(localizations, code)
//!     .language("rust")
//!     .line_numbers(true)
//!     .on_copied(Message::Copied)
//!     .into()
//! # }
//! ```

use std::cell::RefCell;
use std::sync::Arc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Paragraph as _, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Renderer as _, renderer};
use iced::widget::text::{LineHeight, Span, Wrapping};
use iced::widget::{Row, Stack, container, space};
use iced::{
    Alignment, Border, Element as IcedElement, Length, Padding, Pixels, Rectangle, Size, mouse,
};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::SyntaxColors;
use oito2_iced::tokens::{radius, sizes, spacing};
use oito2_iced::widget::copy;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::scroll_area::{self, Direction};
use oito2_iced::{Element, Theme};

use crate::font::JETBRAINS_MONO;
use crate::highlighter::{Run, font_of, highlight_cached};

/// The default size of the code text.
pub const TEXT_SIZE: f32 = 13.0;

/// The default line height of the code, relative to the text size.
pub const LINE_HEIGHT: f32 = 1.45;

type Renderer = iced::Renderer;

/// A code block showing `code`, with the localized tooltips of its copy
/// button.
///
/// Without a [language](CodeBlock::language) the code shows unhighlighted
/// in the foreground color.
pub fn code_block<'a, Message>(
    localizations: &Localizations,
    code: &'a str,
) -> CodeBlock<'a, Message> {
    CodeBlock {
        code,
        language: None,
        localizations: *localizations,
        line_numbers: false,
        copy_button: true,
        on_copied: None,
        text_size: TEXT_SIZE,
        line_height: LINE_HEIGHT,
        padding: Padding::new(spacing::LG),
        width: Length::Shrink,
        direction: ReadingDirection::LeftToRight,
    }
}

/// A code block. See [`code_block`].
pub struct CodeBlock<'a, Message> {
    code: &'a str,
    language: Option<String>,
    localizations: Localizations,
    line_numbers: bool,
    copy_button: bool,
    on_copied: Option<Message>,
    text_size: f32,
    line_height: f32,
    padding: Padding,
    width: Length,
    direction: ReadingDirection,
}

impl<'a, Message> CodeBlock<'a, Message> {
    /// Sets the language: a name, alias or extension such as `dart`, `yml`
    /// or `rs`, in any letter case (see [`language`](crate::language)).
    /// Plain text and unknown languages show unhighlighted.
    #[must_use]
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Shows a column of line numbers in the comment color before the
    /// code (off by default).
    #[must_use]
    pub fn line_numbers(mut self, line_numbers: bool) -> Self {
        self.line_numbers = line_numbers;
        self
    }

    /// Shows the copy button in the top end corner (on by default).
    #[must_use]
    pub fn copy_button(mut self, copy_button: bool) -> Self {
        self.copy_button = copy_button;
        self
    }

    /// Sets the message published after each copy.
    #[must_use]
    pub fn on_copied(mut self, message: Message) -> Self {
        self.on_copied = Some(message);
        self
    }

    /// Sets the size of the code text ([`TEXT_SIZE`] by default).
    #[must_use]
    pub fn text_size(mut self, size: impl Into<Pixels>) -> Self {
        self.text_size = size.into().0;
        self
    }

    /// Sets the line height relative to the text size ([`LINE_HEIGHT`] by
    /// default).
    #[must_use]
    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = line_height;
        self
    }

    /// Sets the space around the code (16 pixels by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width; by default the block is as wide as its code, up to
    /// the available width, beyond which it scrolls.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the reading direction: right to left puts the copy button in
    /// the top left corner and lays the line numbers out after the code.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<CodeBlock<'a, Message>> for Element<'a, Message> {
    fn from(block: CodeBlock<'a, Message>) -> Self {
        let size = block.text_size;
        let line_height = LineHeight::Relative(block.line_height);
        let rtl = block.direction == ReadingDirection::RightToLeft;
        let runs = highlight_cached(block.code, block.language.as_deref().unwrap_or("plaintext"));

        let mut items: Vec<Element<'a, Message>> = Vec::new();

        if block.line_numbers {
            let count = block.code.matches('\n').count() + 1;
            let numbers = (1..=count)
                .map(|line| line.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            items.push(
                iced::widget::text(numbers)
                    .font(JETBRAINS_MONO)
                    .size(size)
                    .line_height(line_height)
                    .wrapping(Wrapping::None)
                    .align_x(if rtl {
                        iced::alignment::Horizontal::Left
                    } else {
                        iced::alignment::Horizontal::Right
                    })
                    .style(|theme: &Theme| iced::widget::text::Style {
                        color: Some(theme.roles().syntax.comment),
                    })
                    .into(),
            );
            items.push(space().width(spacing::LG).into());
        }

        items.push(IcedElement::new(Code {
            code: block.code,
            runs,
            size,
            line_height,
        }));

        if block.copy_button {
            items.push(space().width(sizes::BUTTON_HEIGHT).into());
        }

        if rtl {
            items.reverse();
        }

        let content = scroll_area::scroll_area(Row::with_children(items).align_y(Alignment::Start))
            .direction(Direction::Horizontal)
            .padding(block.padding)
            .width(block.width)
            .height(Length::Shrink);

        let mut stack = Stack::new().push(content);

        if block.copy_button {
            let button = copy::copy_button(&block.localizations, block.code)
                .on_copied_maybe(block.on_copied);

            stack = stack.push(
                container(button)
                    .padding(spacing::XS)
                    .width(Length::Fill)
                    .align_x(if rtl {
                        iced::alignment::Horizontal::Left
                    } else {
                        iced::alignment::Horizontal::Right
                    }),
            );
        }

        container(stack)
            .width(block.width)
            .style(|theme: &Theme| {
                let roles = theme.roles();

                container::Style {
                    text_color: Some(roles.syntax.foreground),
                    background: Some(roles.surface_container.into()),
                    border: Border::default()
                        .color(roles.outline_variant)
                        .width(1.0)
                        .rounded(radius::MEDIUM),
                    ..container::Style::default()
                }
            })
            .into()
    }
}

/// The highlighted code of a [`CodeBlock`]: one paragraph of spans, one
/// per run, colored when drawn.
struct Code<'a> {
    code: &'a str,
    runs: Arc<[Run]>,
    size: f32,
    line_height: LineHeight,
}

/// The paragraph of a [`Code`] and what it was built from.
struct Built {
    runs: Arc<[Run]>,
    size: f32,
    line_height: LineHeight,
    /// The colors of the spans; `None` before the first draw, when the
    /// paragraph is only measured.
    colors: Option<SyntaxColors>,
    paragraph: <Renderer as text::Renderer>::Paragraph,
}

/// The state of a [`Code`]: its paragraph, built in the layout and built
/// again in the draw when the theme colors differ from those it has.
#[derive(Default)]
struct State {
    built: RefCell<Option<Built>>,
}

impl Code<'_> {
    /// Builds the paragraph of the code with `colors`, or without colors.
    fn build(&self, colors: Option<&SyntaxColors>) -> Built {
        let spans: Vec<Span<'_, (), iced::Font>> = self
            .runs
            .iter()
            .filter_map(|run| {
                let piece = self.code.get(run.range.clone())?;

                Some(
                    Span::new(piece)
                        .font_maybe(font_of(&run.token))
                        .color_maybe(colors.map(|colors| run.token.role.color_in(colors))),
                )
            })
            .collect();

        let paragraph = <Renderer as text::Renderer>::Paragraph::with_spans(text::Text {
            content: spans.as_slice(),
            bounds: Size::INFINITE,
            size: Pixels(self.size),
            line_height: self.line_height,
            font: JETBRAINS_MONO,
            align_x: text::Alignment::Left,
            align_y: iced::alignment::Vertical::Top,
            shaping: text::Shaping::Advanced,
            wrapping: Wrapping::None,
        });

        Built {
            runs: Arc::clone(&self.runs),
            size: self.size,
            line_height: self.line_height,
            colors: colors.copied(),
            paragraph,
        }
    }

    /// Whether `built` was built from the runs and the text settings of
    /// this code.
    fn matches(&self, built: &Built) -> bool {
        Arc::ptr_eq(&built.runs, &self.runs)
            && built.size == self.size
            && built.line_height == self.line_height
    }
}

impl<Message> Widget<Message, Theme, Renderer> for Code<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let built = tree.state.downcast_mut::<State>().built.get_mut();

        let stale = built.as_ref().is_none_or(|built| !self.matches(built));

        if stale {
            let colors = built.as_ref().and_then(|built| built.colors);
            *built = Some(self.build(colors.as_ref()));
        }

        let size = built
            .as_ref()
            .map_or(Size::ZERO, |built| built.paragraph.min_bounds());

        layout::Node::new(limits.resolve(Length::Shrink, Length::Shrink, size))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if !bounds.intersects(viewport) {
            return;
        }

        let colors = theme.roles().syntax;
        let state = tree.state.downcast_ref::<State>();
        let Ok(mut built) = state.built.try_borrow_mut() else {
            return;
        };

        let stale = built
            .as_ref()
            .is_none_or(|built| !self.matches(built) || built.colors != Some(colors));

        if stale {
            *built = Some(self.build(Some(&colors)));
        }

        if let Some(built) = built.as_ref() {
            renderer.with_layer(*viewport, |renderer| {
                renderer.fill_paragraph(
                    &built.paragraph,
                    bounds.position(),
                    colors.foreground,
                    *viewport,
                );
            });
        }
    }
}
