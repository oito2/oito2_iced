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

//! The layout page: expandable sections, scroll areas, splitters and
//! paned views.

use iced::widget::{column, container, row, text};
use iced::{Element, Length};

use oito2_iced::Theme;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::expandable::{expandable, expansion_panel};
use oito2_iced::widget::paned_view::paned_view;
use oito2_iced::widget::scroll_area::{Direction, scroll_area};
use oito2_iced::widget::splitter::{self, pane, splitter};

use super::{Gallery, Message, card, page};

/// The height of the examples that fill their area.
const AREA_HEIGHT: f32 = 220.0;

/// A labelled region filling a pane.
fn region<'a>(label: &'a str) -> Element<'a, Message, Theme> {
    container(text(label))
        .padding(spacing::MD)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// The layout page.
pub fn view(gallery: &Gallery) -> Element<'_, Message, Theme> {
    let sections = column![
        expandable(
            "Details",
            text("Shown while the section is open; the chevron turns."),
            gallery.expanded,
        )
        .collapsed(text("Closed: a summary takes its place.").class(TextClass::Variant))
        .on_toggle(Message::Expanded),
        expansion_panel(&gallery.panel)
            .section("General", text("Name, language and region"))
            .section("Network", text("Proxy and connection limits"))
            .section("Privacy", text("History and diagnostics"))
            .on_toggle(Message::Panel),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(420.0));

    let rows = column((0..30).map(|index| text(format!("Row {index}")).into()))
        .spacing(spacing::XS)
        .padding(spacing::MD);
    let items = row((0..20).map(|index| text(format!("Item {index}")).into()))
        .spacing(spacing::MD)
        .padding(spacing::MD);

    let scrolling = row![
        scroll_area(rows)
            .undershoot(true)
            .width(Length::FillPortion(2)),
        scroll_area(items)
            .direction(Direction::Horizontal)
            .undershoot(true)
            .width(Length::FillPortion(3)),
    ]
    .spacing(spacing::LG)
    .height(Length::Fixed(AREA_HEIGHT));

    let split = container(
        splitter(
            &gallery.split,
            vec![
                pane(region("Files")).fraction(0.25),
                pane(
                    splitter(
                        &gallery.nested_split,
                        vec![pane(region("Editor")), pane(region("Terminal"))],
                    )
                    .axis(splitter::Axis::Vertical)
                    .on_change(Message::NestedSplit),
                )
                .fraction(0.5),
                pane(region("Outline")).fraction(0.25).collapsible(true),
            ],
        )
        .on_change(Message::Split),
    )
    .height(Length::Fixed(AREA_HEIGHT));

    let paned = container(
        paned_view(region("Pane"), region("Page"), gallery.pane_size)
            .min_page_size(200.0)
            .on_resize(Message::PaneSize),
    )
    .height(Length::Fixed(AREA_HEIGHT));

    page(
        "Layout",
        "Sections open in place; dividers move with the mouse, or with the arrows, Home and End once focused.",
        [
            card("Expandable and expansion panel", sections),
            card("Scroll areas with undershoot", scrolling),
            card(
                "Splitter (Enter or a double click collapses the outline)",
                split,
            ),
            card("Paned view", paned),
        ],
    )
}
