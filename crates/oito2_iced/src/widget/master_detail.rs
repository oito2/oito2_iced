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

//! Master-detail pages: a list of items (the master) next to the page of
//! the selected item (the detail), as in settings applications.
//!
//! At or above the breakpoint width the master is a resizable pane beside
//! the detail, which shows the first item while nothing is selected. Below
//! it, the master fills the area until an item is selected; the detail then
//! replaces it and its bar shows a back button that publishes the back
//! message, after which the application clears the selection. The switch
//! between master and detail is immediate.
//!
//! The selection and the pane width live in the application.
//!
//! ```no_run
//! use oito2_iced::widget::master_detail::{self, detail_page, master_tile};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Select(usize),
//!     Back,
//!     Resized(f32),
//! }
//!
//! # fn view<'a>(selected: Option<usize>, pane: f32) -> oito2_iced::Element<'a, Message> {
//! master_detail::master_detail(
//!     3,
//!     selected,
//!     |index, selected, _width| {
//!         master_tile(selected, Message::Select(index))
//!             .title(format!("Item {index}"))
//!             .into()
//!     },
//!     |index, back| {
//!         detail_page(iced::widget::text(format!("Body {index}")))
//!             .title(format!("Item {index}"))
//!             .back(back)
//!             .into()
//!     },
//! )
//! .on_back(Message::Back)
//! .pane_size(pane)
//! .on_resize(Message::Resized)
//! .into()
//! # }
//! ```

use std::rc::Rc;

use iced::widget::text::{self, IntoFragment, LineHeight, Wrapping};
use iced::widget::{Column, Row, Space, column, container, responsive, scrollable, stack};
use iced::{Alignment, Color, Length, Padding};

use super::helpers::ReadingDirection;
use super::list_tile::{ListTile, list_tile};
use super::paned_view::{self, clamp_pane_size, paned_view};
use crate::l10n::Localizations;
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{sizes, spacing, typography};
use crate::{Element, Theme};

/// The width at which the master and the detail sit side by side by
/// default.
pub const DEFAULT_BREAKPOINT: f32 = 620.0;

/// The space between master tiles by default.
pub const DEFAULT_TILE_SPACING: f32 = spacing::XS;

/// The space around the master list by default.
pub const DEFAULT_LIST_PADDING: f32 = spacing::SM;

/// Thickness of the line along the bottom edge of a detail page bar.
const BAR_BORDER: f32 = 1.0;

/// Builds the master tile of an item from its index, whether its page is
/// shown, and the width of the master pane.
type TileBuilder<'a, Message> = dyn Fn(usize, bool, f32) -> Element<'a, Message> + 'a;

/// Builds the detail page of an item from its index and the message of
/// its back button, given in the narrow layout only.
type PageBuilder<'a, Message> = dyn Fn(usize, Option<Message>) -> Element<'a, Message> + 'a;

/// Builds an element on each layout.
type Builder<'a, Message> = dyn Fn() -> Element<'a, Message> + 'a;

/// A master list next to the detail page of the selected item. Built by
/// [`master_detail`].
pub struct MasterDetail<'a, Message> {
    length: usize,
    selected: Option<usize>,
    tile: Box<TileBuilder<'a, Message>>,
    page: Box<PageBuilder<'a, Message>>,
    empty: Option<Box<Builder<'a, Message>>>,
    master_bar: Option<Box<Builder<'a, Message>>>,
    master_footer: Option<Box<Builder<'a, Message>>>,
    on_back: Option<Message>,
    on_resize: Option<Rc<dyn Fn(f32) -> Message + 'a>>,
    breakpoint: f32,
    pane_size: f32,
    min_pane_size: f32,
    min_page_size: f32,
    resizable: bool,
    tile_spacing: f32,
    list_padding: Padding,
    pane_color: Option<Color>,
    direction: ReadingDirection,
}

/// A master-detail page of `length` items with `selected` shown, building
/// the master tiles with `tile` (index, selected, pane width) and the
/// detail pages with `page` (index, back message).
pub fn master_detail<'a, Message>(
    length: usize,
    selected: Option<usize>,
    tile: impl Fn(usize, bool, f32) -> Element<'a, Message> + 'a,
    page: impl Fn(usize, Option<Message>) -> Element<'a, Message> + 'a,
) -> MasterDetail<'a, Message> {
    MasterDetail {
        length,
        selected,
        tile: Box::new(tile),
        page: Box::new(page),
        empty: None,
        master_bar: None,
        master_footer: None,
        on_back: None,
        on_resize: None,
        breakpoint: DEFAULT_BREAKPOINT,
        pane_size: paned_view::DEFAULT_PANE_SIZE,
        min_pane_size: paned_view::DEFAULT_MIN_PANE_SIZE,
        min_page_size: paned_view::DEFAULT_MIN_PAGE_SIZE,
        resizable: true,
        tile_spacing: DEFAULT_TILE_SPACING,
        list_padding: Padding::new(DEFAULT_LIST_PADDING),
        pane_color: None,
        direction: ReadingDirection::default(),
    }
}

impl<'a, Message> MasterDetail<'a, Message> {
    /// Builds the content shown in place of the list when there are no
    /// items; the master bar and footer stay around it.
    #[must_use]
    pub fn empty(mut self, empty: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.empty = Some(Box::new(empty));
        self
    }

    /// Builds a bar above the master list, such as a title bar with a
    /// search field.
    #[must_use]
    pub fn master_bar(mut self, bar: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.master_bar = Some(Box::new(bar));
        self
    }

    /// Builds an element below the master list.
    #[must_use]
    pub fn master_footer(mut self, footer: impl Fn() -> Element<'a, Message> + 'a) -> Self {
        self.master_footer = Some(Box::new(footer));
        self
    }

    /// The message of the back button of the narrow layout; the
    /// application clears the selection on it. Without it the detail page
    /// has no back button.
    #[must_use]
    pub fn on_back(mut self, message: Message) -> Self {
        self.on_back = Some(message);
        self
    }

    /// Publishes the new master pane width while it is resized; without it
    /// the pane cannot be resized.
    #[must_use]
    pub fn on_resize(mut self, on_resize: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_resize = Some(Rc::new(on_resize));
        self
    }

    /// The width at which the master and the detail are shown side by
    /// side; defaults to [`DEFAULT_BREAKPOINT`].
    #[must_use]
    pub fn breakpoint(mut self, breakpoint: f32) -> Self {
        self.breakpoint = breakpoint;
        self
    }

    /// The master pane width; defaults to
    /// [`paned_view::DEFAULT_PANE_SIZE`].
    #[must_use]
    pub fn pane_size(mut self, size: f32) -> Self {
        self.pane_size = size;
        self
    }

    /// The smallest master pane width; defaults to
    /// [`paned_view::DEFAULT_MIN_PANE_SIZE`].
    #[must_use]
    pub fn min_pane_size(mut self, size: f32) -> Self {
        self.min_pane_size = size.max(0.0);
        self
    }

    /// The smallest width left for the detail page; defaults to
    /// [`paned_view::DEFAULT_MIN_PAGE_SIZE`].
    #[must_use]
    pub fn min_page_size(mut self, size: f32) -> Self {
        self.min_page_size = size.max(0.0);
        self
    }

    /// Whether the master pane can be resized (true by default).
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// The space between master tiles; defaults to
    /// [`DEFAULT_TILE_SPACING`].
    #[must_use]
    pub fn tile_spacing(mut self, spacing: f32) -> Self {
        self.tile_spacing = spacing;
        self
    }

    /// The space around the master list; defaults to
    /// [`DEFAULT_LIST_PADDING`] on every side.
    #[must_use]
    pub fn list_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.list_padding = padding.into();
        self
    }

    /// The master pane background; defaults to the chrome main bar color.
    #[must_use]
    pub fn pane_color(mut self, color: Color) -> Self {
        self.pane_color = Some(color);
        self
    }

    /// The reading direction: the master is on the right in right-to-left
    /// text.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

/// The layout a master-detail page takes at a width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shown {
    /// The empty content, with the master bar and footer when given.
    Empty,
    /// The master beside the detail of the item.
    Both(usize),
    /// The master alone.
    Master,
    /// The detail of the item alone.
    Detail(usize),
}

/// What a page of `length` items with `selected` shows at `width`.
fn shown(length: usize, selected: Option<usize>, width: f32, breakpoint: f32) -> Shown {
    let selected = selected.filter(|index| *index < length);

    if length == 0 {
        Shown::Empty
    } else if width >= breakpoint {
        Shown::Both(selected.unwrap_or(0))
    } else if let Some(index) = selected {
        Shown::Detail(index)
    } else {
        Shown::Master
    }
}

impl<'a, Message: Clone + 'a> MasterDetail<'a, Message> {
    /// The master pane at `width` with the tile at `selected` highlighted,
    /// showing `content` in place of the list when given.
    fn master(
        &self,
        width: f32,
        selected: Option<usize>,
        content: Option<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        let content = content.unwrap_or_else(|| {
            let tiles =
                (0..self.length).map(|index| (self.tile)(index, Some(index) == selected, width));

            scrollable(
                Column::with_children(tiles)
                    .spacing(self.tile_spacing)
                    .padding(self.list_padding)
                    .width(Length::Fill),
            )
            .height(Length::Fill)
            .into()
        });

        let pane = column![
            self.master_bar.as_ref().map(|bar| bar()),
            container(content).width(Length::Fill).height(Length::Fill),
            self.master_footer.as_ref().map(|footer| footer()),
        ];

        let pane = container(pane).width(Length::Fill).height(Length::Fill);

        match self.pane_color {
            Some(color) => pane
                .style(move |theme: &Theme| container::Style {
                    background: Some(color.into()),
                    text_color: Some(theme.roles().on_surface),
                    ..container::Style::default()
                })
                .into(),
            None => pane.class(ContainerClass::Mainbar).into(),
        }
    }

    /// The page at `width`.
    fn view(&self, width: f32) -> Element<'a, Message> {
        match shown(self.length, self.selected, width, self.breakpoint) {
            Shown::Empty => {
                let empty = self
                    .empty
                    .as_ref()
                    .map_or_else(|| Space::new().into(), |empty| empty());

                if self.master_bar.is_none() && self.master_footer.is_none() {
                    empty
                } else {
                    self.master(width, None, Some(empty))
                }
            }
            Shown::Both(index) => {
                let pane_width = clamp_pane_size(
                    self.pane_size,
                    self.min_pane_size,
                    self.min_page_size,
                    width,
                );
                let mut view = paned_view(
                    self.master(pane_width, Some(index), None),
                    (self.page)(index, None),
                    self.pane_size,
                )
                .min_pane_size(self.min_pane_size)
                .min_page_size(self.min_page_size)
                .resizable(self.resizable)
                .direction(self.direction);

                if let Some(on_resize) = &self.on_resize {
                    let on_resize = Rc::clone(on_resize);
                    view = view.on_resize(move |size| on_resize(size));
                }

                view.into()
            }
            Shown::Master => self.master(width, None, None),
            Shown::Detail(index) => (self.page)(index, self.on_back.clone()),
        }
    }
}

impl<'a, Message: Clone + 'a> From<MasterDetail<'a, Message>> for Element<'a, Message> {
    fn from(page: MasterDetail<'a, Message>) -> Self {
        responsive(move |size| page.view(size.width)).into()
    }
}

/// A master tile: a list tile highlighted while `selected` that publishes
/// `on_press` on a click, Enter or Space. Fill it with
/// [`ListTile::title`], [`ListTile::subtitle`], [`ListTile::leading`] and
/// [`ListTile::trailing`].
pub fn master_tile<'a, Message>(selected: bool, on_press: Message) -> ListTile<'a, Message> {
    list_tile().selected(selected).on_press(on_press)
}

/// A detail page: a bar above the body on the page background. Built by
/// [`detail_page`].
pub struct DetailPage<'a, Message> {
    body: Element<'a, Message>,
    title: Option<text::Fragment<'a>>,
    actions: Vec<Element<'a, Message>>,
    bar: Option<Element<'a, Message>>,
    floating_action_button: Option<Element<'a, Message>>,
    background: Option<Color>,
    back: Option<Message>,
    localizations: Localizations,
    direction: ReadingDirection,
}

/// A detail page showing `body`.
///
/// The default bar is [`sizes::TITLE_BAR_HEIGHT`] tall on the title bar
/// color, with a line in the `outline_variant` role along its bottom edge,
/// the centered title in the title medium style, the back button at the
/// start and the actions at the end. It does not drag the window. The page
/// has no bar without a title, actions or back button.
pub fn detail_page<'a, Message>(body: impl Into<Element<'a, Message>>) -> DetailPage<'a, Message> {
    DetailPage {
        body: body.into(),
        title: None,
        actions: Vec::new(),
        bar: None,
        floating_action_button: None,
        background: None,
        back: None,
        localizations: Localizations::en(),
        direction: ReadingDirection::default(),
    }
}

impl<'a, Message> DetailPage<'a, Message> {
    /// The title of the default bar.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Adds an action at the end of the default bar.
    #[must_use]
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Replaces the default bar.
    #[must_use]
    pub fn bar(mut self, bar: impl Into<Element<'a, Message>>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    /// A floating action button at the bottom end corner.
    #[must_use]
    pub fn floating_action_button(mut self, button: impl Into<Element<'a, Message>>) -> Self {
        self.floating_action_button = Some(button.into());
        self
    }

    /// The background; defaults to the page background role.
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// The message of the back button; `None` shows no back button. The
    /// page builder of a [`MasterDetail`] receives it.
    #[must_use]
    pub fn back(mut self, message: Option<Message>) -> Self {
        self.back = message;
        self
    }

    /// The strings of the back button tooltip; English by default.
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// The reading direction of the back chevron and the button corner.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

/// The back button of a detail page bar, publishing `message`.
#[cfg(feature = "svg")]
fn back_button<'a, Message: Clone + 'a>(
    message: Message,
    localizations: &Localizations,
    direction: ReadingDirection,
) -> Element<'a, Message> {
    super::button::back_button(localizations, direction)
        .on_press(message)
        .into()
}

/// The back button of a detail page bar, publishing `message`.
#[cfg(not(feature = "svg"))]
fn back_button<'a, Message: Clone + 'a>(
    message: Message,
    localizations: &Localizations,
    _direction: ReadingDirection,
) -> Element<'a, Message> {
    super::button::text(localizations.material.back)
        .on_press(message)
        .into()
}

/// The default bar of a detail page.
fn detail_bar<'a, Message: Clone + 'a>(
    title: Option<text::Fragment<'a>>,
    actions: Vec<Element<'a, Message>>,
    back: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let edge = || Space::new().width(spacing::SM);

    let mut leading = Row::new().push(edge()).align_y(Alignment::Center);
    if let Some(back) = back {
        leading = leading.push(Space::new().width(spacing::XS)).push(back);
    }

    let mut trailing = Row::new().align_y(Alignment::Center);
    for action in actions {
        trailing = trailing.push(action).push(Space::new().width(spacing::XS));
    }
    trailing = trailing.push(edge());

    let style = typography::TITLE_MEDIUM;
    let title = title.map(|title| {
        container(
            iced::widget::text(title)
                .size(style.size)
                .line_height(LineHeight::Absolute(style.line_height.into()))
                .font(crate::font::inter(style.weight))
                .wrapping(Wrapping::None)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.roles().on_surface),
                }),
        )
        .padding([0.0, spacing::LG])
    });

    let content = Row::new()
        .push(container(leading).width(Length::Fill))
        .push(title)
        .push(
            container(trailing)
                .width(Length::Fill)
                .align_x(Alignment::End),
        )
        .align_y(Alignment::Center);

    container(column![
        container(content)
            .width(Length::Fill)
            .height(sizes::TITLE_BAR_HEIGHT - BAR_BORDER)
            .align_y(Alignment::Center)
            .clip(true),
        container(Space::new().width(Length::Fill).height(BAR_BORDER)).style(|theme: &Theme| {
            container::Style {
                background: Some(theme.roles().outline_variant.into()),
                ..container::Style::default()
            }
        }),
    ])
    .class(ContainerClass::TitleBar)
    .width(Length::Fill)
    .height(sizes::TITLE_BAR_HEIGHT)
    .into()
}

impl<'a, Message: Clone + 'a> From<DetailPage<'a, Message>> for Element<'a, Message> {
    fn from(page: DetailPage<'a, Message>) -> Self {
        let DetailPage {
            body,
            title,
            actions,
            bar,
            floating_action_button,
            background,
            back,
            localizations,
            direction,
        } = page;

        let bar = bar.or_else(|| {
            (title.is_some() || !actions.is_empty() || back.is_some()).then(|| {
                let back = back.map(|message| back_button(message, &localizations, direction));
                detail_bar(title, actions, back)
            })
        });

        let content = column![
            bar,
            container(body).width(Length::Fill).height(Length::Fill),
        ];

        let content: Element<'a, Message> = match floating_action_button {
            Some(button) => stack![
                content,
                container(button)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .padding(spacing::LG)
                    .align_x(match direction {
                        ReadingDirection::LeftToRight => Alignment::End,
                        ReadingDirection::RightToLeft => Alignment::Start,
                    })
                    .align_y(Alignment::End),
            ]
            .into(),
            None => content.into(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |theme: &Theme| {
                let roles = theme.roles();
                container::Style {
                    background: Some(background.unwrap_or(roles.background).into()),
                    text_color: Some(roles.on_surface),
                    ..container::Style::default()
                }
            })
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_layouts_show_both_and_default_to_the_first_item() {
        assert_eq!(shown(3, None, 800.0, 620.0), Shown::Both(0));
        assert_eq!(shown(3, Some(2), 620.0, 620.0), Shown::Both(2));
        assert_eq!(shown(3, Some(7), 800.0, 620.0), Shown::Both(0));
    }

    #[test]
    fn narrow_layouts_show_the_master_or_the_detail() {
        assert_eq!(shown(3, None, 400.0, 620.0), Shown::Master);
        assert_eq!(shown(3, Some(1), 400.0, 620.0), Shown::Detail(1));
    }

    #[test]
    fn no_items_show_the_empty_content() {
        assert_eq!(shown(0, Some(0), 800.0, 620.0), Shown::Empty);
    }
}
