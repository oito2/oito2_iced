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

//! The data page: list box, tree view, JSON tree, pagination, steps and a
//! table of contents.

use iced::widget::{column, container, responsive, row, scrollable, text};
use iced::{Element, Length};

use oito2_iced::Theme;
use oito2_iced::collections::{
    ListCollection, SelectionController, SelectionMode, SelectionOptions, TreeCollection, TreeNode,
};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::json_tree_view::{self, JsonEntry, JsonValue};
use oito2_iced::widget::list_box::list_box;
use oito2_iced::widget::pagination::pagination;
use oito2_iced::widget::steps::{step, steps};
use oito2_iced::widget::table_of_contents::{self as toc, Entry};
use oito2_iced::widget::tree_view::{TreeState, tree_view};

use super::{Gallery, Message, card, page};

/// The id of the scrollable document the table of contents follows.
pub const DOCUMENT: &str = "toc-document";

/// The card width under which the document goes below the table of
/// contents.
const COMPACT_TOC_WIDTH: f32 = 600.0;

/// The sections of the document, with the paragraphs under each.
const SECTIONS: [(&str, u16); 5] = [
    ("Introduction", 0),
    ("Installation", 0),
    ("From crates.io", 1),
    ("Usage", 0),
    ("License", 0),
];

/// The state of the data page.
#[derive(Debug)]
pub struct Data {
    pub(super) fruits: Option<SelectionController<String>>,
    pub(super) files: Option<TreeCollection<()>>,
    pub(super) files_state: TreeState,
    pub(super) json: TreeCollection<JsonEntry>,
    pub(super) json_state: TreeState,
    pub(super) page: u32,
    pub(super) page_size: u32,
    pub(super) page_size_open: bool,
    pub(super) step: usize,
    pub(super) entries: Vec<Entry>,
    pub(super) section: usize,
    pub(super) opened: Option<String>,
}

impl Default for Data {
    fn default() -> Self {
        let fruits = ListCollection::new(
            [
                "Apple", "Banana", "Cherry", "Grape", "Mango", "Orange", "Pear",
            ]
            .map(str::to_owned),
        )
        .ok()
        .map(|collection| {
            SelectionController::new(
                collection,
                SelectionOptions {
                    mode: SelectionMode::Multiple,
                    ..SelectionOptions::default()
                },
            )
        });
        let files = TreeCollection::new(vec![
            TreeNode::branch(
                "lib",
                vec![
                    TreeNode::leaf("lib/main.rs").with_label("main.rs"),
                    TreeNode::branch(
                        "lib/widget",
                        vec![
                            TreeNode::leaf("lib/widget/tree.rs").with_label("tree.rs"),
                            TreeNode::leaf("lib/widget/list.rs").with_label("list.rs"),
                        ],
                    )
                    .with_label("widget"),
                ],
            ),
            TreeNode::branch(
                "tests",
                vec![TreeNode::leaf("tests/tree.rs").with_label("tree.rs")],
            ),
            TreeNode::leaf("Cargo.toml"),
            TreeNode::leaf("README.md"),
        ])
        .ok();
        let json = json_tree_view::json_tree(&sample_json(), json_tree_view::DEFAULT_ROOT_LABEL);
        let json_state =
            TreeState::default().with_expanded(json_tree_view::expanded_to_depth(&json, 1));

        Self {
            fruits,
            files,
            files_state: TreeState::default().with_expanded(["lib".to_owned()]),
            json,
            json_state,
            page: 5,
            page_size: 20,
            page_size_open: false,
            step: 1,
            entries: SECTIONS
                .iter()
                .enumerate()
                .map(|(index, (title, depth))| {
                    toc::entry(*title, format!("toc-section-{index}")).depth(*depth)
                })
                .collect(),
            section: 0,
            opened: None,
        }
    }
}

/// The value the JSON tree shows.
fn sample_json() -> JsonValue {
    JsonValue::Object(vec![
        ("name".into(), JsonValue::String("oito2_iced".into())),
        ("version".into(), JsonValue::String("0.1.0".into())),
        ("stars".into(), JsonValue::Integer(42)),
        ("ratio".into(), JsonValue::Float(0.75)),
        ("published".into(), JsonValue::Bool(false)),
        ("homepage".into(), JsonValue::Null),
        (
            "keywords".into(),
            JsonValue::Array(vec![
                JsonValue::String("gui".into()),
                JsonValue::String("iced".into()),
                JsonValue::String("design-system".into()),
            ]),
        ),
        (
            "authors".into(),
            JsonValue::Array(vec![JsonValue::Object(vec![
                ("name".into(), JsonValue::String("OITO2".into())),
                ("role".into(), JsonValue::String("maintainer".into())),
            ])]),
        ),
    ])
}

/// The data page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let data = &gallery.data;

    let fruits: Element<'a, Message, Theme> = match &data.fruits {
        Some(fruits) => list_box(fruits)
            .on_change(Message::Fruits)
            .on_activate(|fruit: &String| Message::FruitActivated(fruit.clone()))
            .max_height(220.0)
            .width(Length::Fixed(280.0))
            .into(),
        None => text("").into(),
    };

    let fruits = column![
        fruits,
        text(match &data.opened {
            Some(fruit) => format!("Opened {fruit}"),
            None => "Double-click or press Enter to open".to_owned(),
        })
        .class(TextClass::Variant),
    ]
    .spacing(spacing::SM);

    let files: Element<'a, Message, Theme> = match &data.files {
        Some(files) => tree_view(files, &data.files_state)
            .on_event(Message::Files)
            .checkable(true)
            .localizations(l10n)
            .height(Length::Fixed(260.0))
            .width(Length::Fixed(320.0))
            .into(),
        None => text("").into(),
    };

    let json = json_tree_view::json_tree_view(&data.json, &data.json_state)
        .on_event(Message::Json)
        .localizations(l10n)
        .height(Length::Fixed(260.0))
        .width(Length::Fixed(360.0));

    let lists = row![fruits, files, json]
        .spacing(spacing::XL)
        .wrap()
        .vertical_spacing(spacing::LG);

    let pages = pagination(data.page, 10)
        .on_change(Message::DataPage)
        .page_sizes([10, 20, 50], data.page_size, Message::PageSize)
        .page_size_menu(data.page_size_open, Message::PageSizeOpen)
        .localizations(l10n);

    let flow = steps(
        [
            step("Account").description("Your e-mail"),
            step("Profile").description("Name and photo"),
            step("Plan"),
            step("Done"),
        ],
        data.step,
    )
    .on_select(Message::Step);

    let contents = responsive(move |size| {
        let document = scrollable(
            column(SECTIONS.iter().enumerate().map(|(index, (title, _))| {
                column![
                    container(oito2_iced::widget::styled_text(
                        *title,
                        typography::TITLE_MEDIUM
                    ))
                    .id(format!("toc-section-{index}")),
                    text(
                        "The table of contents marks the section whose heading reached the top \
                             of this panel. Pick an entry to scroll to its section."
                    )
                    .class(TextClass::Variant),
                    container(text("")).height(Length::Fixed(140.0)),
                ]
                .spacing(spacing::SM)
                .into()
            }))
            .spacing(spacing::MD)
            .padding(spacing::MD),
        )
        .id(DOCUMENT)
        .height(Length::Fixed(240.0));

        let entries = container(
            toc::table_of_contents(&data.entries, data.section).on_select(Message::TocPick),
        )
        .width(Length::Fixed(200.0));
        let tracker =
            toc::tracker(document, &data.entries, data.section).on_change(Message::Section);
        // The document goes under the entries when they would squeeze it.
        let contents: Element<'a, Message, Theme> = if size.width < COMPACT_TOC_WIDTH {
            column![entries, tracker].spacing(spacing::LG).into()
        } else {
            row![entries, tracker].spacing(spacing::LG).into()
        };
        contents
    })
    .height(Length::Shrink);

    page(
        "Data",
        "Lists, trees and navigation through long content, all usable with the keyboard.",
        [
            card("List box, tree and JSON", lists),
            card("Pagination", pages),
            card("Steps", flow),
            card("Table of contents", contents),
        ],
    )
}
