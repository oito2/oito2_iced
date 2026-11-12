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

//! The title bar and the header bar of an undecorated window.

use iced::widget::text::{self, LineHeight, Wrapping};
use iced::widget::{Row, Space, column, container, mouse_area};
use iced::{Alignment, Length};

use super::control::{self, Labels};
use super::{Action, WindowState};
use crate::foundation::{WindowButtonLayout, WindowControlStyle};
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{sizes, spacing, typography};
use crate::{Element, Theme};

/// Opacity of the title bar content while the window is not focused.
pub const INACTIVE_OPACITY: f32 = 0.7;

/// Thickness of the line along the bottom edge of the title bar.
const BORDER_WIDTH: f32 = 1.0;

/// The middle of a title bar.
enum Title<'a, Message> {
    Text(text::Fragment<'a>),
    Element(Element<'a, Message>),
}

/// The title bar of an undecorated window: a title, items after the
/// leading window buttons and before the trailing ones, and the window
/// control buttons on the sides given by a [`WindowButtonLayout`].
///
/// A press on the empty area of the bar drags the window, a double click
/// toggles maximization and a right press opens the window menu. The title
/// is centered, except with the Windows control style; while the window is
/// not focused the title and the buttons are drawn at
/// [`INACTIVE_OPACITY`]. A line in the `outline_variant` role runs along
/// the bottom edge.
///
/// Built by [`title_bar`] and [`header_bar`]; turns into an [`Element`].
pub struct TitleBar<'a, Message> {
    title: Option<Title<'a, Message>>,
    search: Option<Element<'a, Message>>,
    start: Vec<Element<'a, Message>>,
    end: Vec<Element<'a, Message>>,
    style: WindowControlStyle,
    layout: Option<WindowButtonLayout>,
    state: WindowState,
    labels: Labels,
    center_title: Option<bool>,
    leading_inset: f32,
    height: f32,
    on_action: Box<dyn Fn(Action) -> Message + 'a>,
}

/// A title bar showing `title`, with `start` after the leading window
/// buttons, publishing the window actions through `on_action`.
///
/// The bar defaults to the GNOME control style, the default button layout
/// of the style and a focused, restored window; see [`TitleBar`].
pub fn title_bar<'a, Message>(
    title: impl text::IntoFragment<'a>,
    start: impl Into<Element<'a, Message>>,
    on_action: impl Fn(Action) -> Message + 'a,
) -> TitleBar<'a, Message> {
    TitleBar::new(on_action).title(title).start(start)
}

/// A header bar: a [`TitleBar`] that holds the application controls, as
/// in GNOME applications.
///
/// [`TitleBar::start`] items sit after the leading window buttons, such as
/// a back button or a sidebar toggle, and [`TitleBar::end`] items before
/// the trailing ones, such as a search or a menu button. The middle shows
/// [`TitleBar::title`] or [`TitleBar::title_element`], or the
/// [`TitleBar::search`] element in place of either while the application
/// shows its search.
pub fn header_bar<'a, Message>(
    on_action: impl Fn(Action) -> Message + 'a,
) -> TitleBar<'a, Message> {
    TitleBar::new(on_action)
}

impl<'a, Message> TitleBar<'a, Message> {
    /// An empty bar publishing the window actions through `on_action`.
    fn new(on_action: impl Fn(Action) -> Message + 'a) -> Self {
        Self {
            title: None,
            search: None,
            start: Vec::new(),
            end: Vec::new(),
            style: WindowControlStyle::Gnome,
            layout: None,
            state: WindowState::default(),
            labels: Labels::default(),
            center_title: None,
            leading_inset: 0.0,
            height: sizes::TITLE_BAR_HEIGHT,
            on_action: Box::new(on_action),
        }
    }

    /// Shows `title` in the middle, in the title medium style.
    #[must_use]
    pub fn title(mut self, title: impl text::IntoFragment<'a>) -> Self {
        self.title = Some(Title::Text(title.into_fragment()));
        self
    }

    /// Shows `element` in the middle in place of a text title, such as tabs.
    #[must_use]
    pub fn title_element(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.title = Some(Title::Element(element.into()));
        self
    }

    /// Shows `element`, usually a search field, in the middle in place of
    /// the title.
    #[must_use]
    pub fn search(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.search = Some(element.into());
        self
    }

    /// Adds `element` after the leading window buttons and the previous
    /// start items.
    #[must_use]
    pub fn start(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.start.push(element.into());
        self
    }

    /// Adds `element` after the previous end items, before the trailing
    /// window buttons.
    #[must_use]
    pub fn end(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.end.push(element.into());
        self
    }

    /// The visual family of the window buttons; [`WindowControlStyle::None`]
    /// shows no buttons. Defaults to [`WindowControlStyle::Gnome`].
    #[must_use]
    pub fn control_style(mut self, style: WindowControlStyle) -> Self {
        self.style = style;
        self
    }

    /// Which window buttons appear and on which side; defaults to
    /// [`WindowButtonLayout::default_for`] the control style.
    #[must_use]
    pub fn button_layout(mut self, layout: WindowButtonLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// The state of the window: maximized shows the restore button and an
    /// unfocused window dims the bar.
    #[must_use]
    pub fn state(mut self, state: WindowState) -> Self {
        self.state = state;
        self
    }

    /// The labels of the window buttons; defaults to English.
    #[must_use]
    pub fn labels(mut self, labels: Labels) -> Self {
        self.labels = labels;
        self
    }

    /// Whether the title is centered in the bar; defaults to centered for
    /// every control style except [`WindowControlStyle::Windows`].
    #[must_use]
    pub fn center_title(mut self, center: bool) -> Self {
        self.center_title = Some(center);
        self
    }

    /// Empty space kept at the leading edge, for window buttons drawn by
    /// the operating system.
    #[must_use]
    pub fn leading_inset(mut self, inset: f32) -> Self {
        self.leading_inset = inset;
        self
    }

    /// The bar height, bottom line included; defaults to
    /// [`sizes::TITLE_BAR_HEIGHT`].
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<'a, Message> From<TitleBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: TitleBar<'a, Message>) -> Self {
        let TitleBar {
            title,
            search,
            start,
            end,
            style,
            layout,
            state,
            labels,
            center_title,
            leading_inset,
            height,
            on_action,
        } = bar;

        let layout = layout.unwrap_or_else(|| WindowButtonLayout::default_for(style));
        let opacity = if state.focused { 1.0 } else { INACTIVE_OPACITY };
        let content_height = (height - BORDER_WIDTH).max(0.0);
        let shows_controls = style != WindowControlStyle::None;
        let edge = if style == WindowControlStyle::Windows {
            0.0
        } else {
            spacing::SM
        };
        let controls = |buttons| {
            control::row(
                style,
                buttons,
                state,
                &labels,
                &*on_action,
                content_height,
                opacity,
            )
        };

        let mut leading: Vec<Element<'a, Message>> =
            vec![Space::new().width(leading_inset + edge).into()];
        if shows_controls && !layout.leading.is_empty() {
            leading.push(controls(&layout.leading));
        }
        for item in start {
            leading.push(Space::new().width(spacing::XS).into());
            leading.push(item);
        }

        let has_end = !end.is_empty();
        let mut trailing: Vec<Element<'a, Message>> = Vec::new();
        for item in end {
            trailing.push(item);
            trailing.push(Space::new().width(spacing::XS).into());
        }
        if shows_controls && !layout.trailing.is_empty() {
            if has_end {
                trailing.push(Space::new().width(spacing::XS).into());
            }
            trailing.push(controls(&layout.trailing));
        }
        trailing.push(Space::new().width(edge).into());

        let leading = Row::with_children(leading).align_y(Alignment::Center);
        let trailing = Row::with_children(trailing).align_y(Alignment::Center);

        let middle: Option<Element<'a, Message>> = match (search, title) {
            (Some(search), _) => Some(search),
            (None, Some(Title::Element(element))) => Some(element),
            (None, Some(Title::Text(fragment))) => Some(title_text(fragment, opacity)),
            (None, None) => None,
        };

        let centered = center_title.unwrap_or(style != WindowControlStyle::Windows);
        let content: Element<'a, Message> = match middle {
            Some(middle) if centered => Row::new()
                .push(container(leading).width(Length::Fill))
                .push(container(middle).padding([0.0, spacing::LG]))
                .push(
                    container(trailing)
                        .width(Length::Fill)
                        .align_x(Alignment::End),
                )
                .align_y(Alignment::Center)
                .into(),
            Some(middle) => Row::new()
                .push(leading)
                .push(
                    container(middle)
                        .padding([0.0, spacing::LG])
                        .width(Length::Fill),
                )
                .push(trailing)
                .align_y(Alignment::Center)
                .into(),
            None => Row::new()
                .push(leading)
                .push(Space::new().width(Length::Fill))
                .push(trailing)
                .align_y(Alignment::Center)
                .into(),
        };

        let bar = column![
            container(content)
                .width(Length::Fill)
                .height(content_height)
                .align_y(Alignment::Center)
                .clip(true),
            container(Space::new().width(Length::Fill).height(BORDER_WIDTH)).style(
                |theme: &Theme| container::Style {
                    background: Some(theme.roles().outline_variant.into()),
                    ..container::Style::default()
                }
            ),
        ];

        mouse_area(
            container(bar)
                .class(ContainerClass::TitleBar)
                .width(Length::Fill)
                .height(height),
        )
        .on_press(on_action(Action::Drag))
        .on_double_click(on_action(Action::ToggleMaximize))
        .on_right_press(on_action(Action::ShowSystemMenu))
        .into()
    }
}

/// A one-line title in the title medium style, in the `on_surface` role
/// drawn at `opacity`.
fn title_text<'a, Message>(fragment: text::Fragment<'a>, opacity: f32) -> Element<'a, Message>
where
    Message: 'a,
{
    let style = typography::TITLE_MEDIUM;

    iced::widget::text(fragment)
        .size(style.size)
        .line_height(LineHeight::Absolute(style.line_height.into()))
        .font(crate::font::inter(style.weight))
        .wrapping(Wrapping::None)
        .style(move |theme: &Theme| text::Style {
            color: Some(theme.roles().on_surface.scale_alpha(opacity)),
        })
        .into()
}
