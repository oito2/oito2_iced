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

//! Tree view: keyboard navigation, pointer selection and double clicks,
//! multiple selection, check boxes, filtering, renaming, lazy loading,
//! scrolling of the virtualized rows, the read-only state and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use std::time::Duration;

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::{self, Key, Modifiers, key::Named};
use iced::widget::column;
use iced::{Event, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::collections::{SelectionMode, TreeCollection, TreeNode};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::tree_view::{self, TreeEvent, TreeState};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tree(TreeEvent),
    Activate(String),
    Rename(String, String),
    Load(String),
    After,
}

/// The tree of the examples: `src` with a file and a folder of two files,
/// a readme and a disabled license.
fn files() -> TreeCollection<()> {
    TreeCollection::new(vec![
        TreeNode::branch(
            "src",
            vec![
                TreeNode::leaf("src/main.dart").with_label("main.dart"),
                TreeNode::branch(
                    "src/widgets",
                    vec![
                        TreeNode::leaf("src/widgets/a.dart").with_label("a.dart"),
                        TreeNode::leaf("src/widgets/b.dart").with_label("b.dart"),
                    ],
                )
                .with_label("widgets"),
            ],
        ),
        TreeNode::leaf("README.md"),
        TreeNode::leaf("LICENSE").with_disabled(true),
    ])
    .expect("unique values")
}

/// An app with one tree view and a button after it.
struct Files {
    tree: TreeCollection<()>,
    state: TreeState,
    query: String,
    checkable: bool,
    activate: bool,
    rename: bool,
    load: bool,
    read_only: bool,
    direction: ReadingDirection,
    height: f32,
}

impl Files {
    fn new(mode: SelectionMode) -> Self {
        Self {
            tree: files(),
            state: TreeState::new(mode),
            query: String::new(),
            checkable: false,
            activate: false,
            rename: false,
            load: false,
            read_only: false,
            direction: ReadingDirection::LeftToRight,
            height: 300.0,
        }
    }

    fn with_tree(tree: TreeCollection<()>) -> Self {
        Self {
            tree,
            ..Self::new(SelectionMode::Single)
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut tree = tree_view::tree_view(&self.tree, &self.state)
            .query(&self.query)
            .checkable(self.checkable)
            .direction(self.direction)
            .height(self.height)
            .id("tree");

        if !self.read_only {
            tree = tree.on_event(Message::Tree);
        }
        if self.activate {
            tree = tree.on_activate(Message::Activate);
        }
        if self.rename {
            tree = tree.on_rename(Message::Rename);
        }
        if self.load {
            tree = tree.on_load_children(Message::Load);
        }

        focus::navigation(column![
            tree,
            button::filled("After").on_press(Message::After).id("after"),
        ])
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tree(event) => self.state.apply(&self.tree, event),
            Message::Load(value) => self.state.start_loading(&value),
            Message::Activate(_) | Message::Rename(..) | Message::After => {}
        }
    }
}

/// The app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the
/// next events.
struct Harness {
    model: Files,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    messages: Vec<Message>,
    size: Size,
}

impl Harness {
    fn new(model: Files) -> Self {
        let settings = support::settings();
        let _fonts = simulator::Simulator::<()>::with_settings(
            support::settings(),
            iced::widget::Space::new(),
        );
        let backend = std::env::var("ICED_TEST_BACKEND").ok();
        let renderer =
            iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
                settings.default_font,
                settings.default_text_size,
                backend.as_deref(),
            ))
            .expect("headless renderer");

        Self {
            model,
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            messages: Vec::new(),
            size: Size::new(300.0, 400.0),
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let _ = ui.update(
            &events,
            self.cursor,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut messages,
        );
        self.cache = Some(ui.into_cache());

        for message in messages {
            self.messages.push(message.clone());
            self.model.update(message);
        }
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let mut find = selector.find();
        ui.operate(&self.renderer, &mut operation::black_box(&mut find));
        self.cache = Some(ui.into_cache());

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn text(&mut self, text: &str) -> Option<Rectangle> {
        self.find(text).map(|target| target.bounds())
    }

    fn bounds(&mut self, id: &'static str) -> Rectangle {
        self.find(selector::id(id)).expect("widget exists").bounds()
    }

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn click_text(&mut self, text: &str) {
        let bounds = self.text(text).expect("text is shown");
        self.click_at(bounds.center());
    }

    fn click_with(&mut self, text: &str, modifiers: Modifiers) {
        self.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            modifiers,
        ))]);
        self.click_text(text);
        self.send([Event::Keyboard(keyboard::Event::ModifiersChanged(
            Modifiers::empty(),
        ))]);
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn key_with(&mut self, key: Key, modifiers: Modifiers) {
        self.send([Event::Keyboard(keyboard::Event::KeyPressed {
            key: key.clone(),
            modified_key: key,
            physical_key: keyboard::key::Physical::Unidentified(
                keyboard::key::NativeCode::Unidentified,
            ),
            location: keyboard::Location::Standard,
            modifiers,
            repeat: false,
            text: None,
        })]);
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    fn selected(&self) -> Vec<String> {
        let mut selected: Vec<String> = self.model.state.selected().iter().cloned().collect();
        selected.sort();
        selected
    }

    fn rows(&mut self, texts: &[&str]) -> Vec<bool> {
        texts.iter().map(|text| self.text(text).is_some()).collect()
    }
}

fn sorted(values: &[&str]) -> Vec<String> {
    let mut values: Vec<String> = values.iter().map(|v| (*v).to_owned()).collect();
    values.sort();
    values
}

#[test]
fn the_arrow_keys_move_open_and_close_branches() {
    let mut ui = Harness::new(Files::new(SelectionMode::Single));

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(ui.bounds("tree")));
    ui.key(Named::Home);
    assert_eq!(ui.selected(), ["src"]);

    ui.key(Named::ArrowRight);
    assert!(ui.model.state.is_expanded("src"));
    assert!(ui.text("main.dart").is_some());

    ui.key(Named::ArrowRight);
    assert_eq!(ui.selected(), ["src/main.dart"]);
    ui.key(Named::ArrowLeft);
    assert_eq!(ui.selected(), ["src"]);
    ui.key(Named::ArrowLeft);
    assert!(!ui.model.state.is_expanded("src"));

    ui.key(Named::End);
    assert_eq!(
        ui.selected(),
        ["README.md"],
        "the disabled LICENSE is skipped"
    );
    ui.type_text("s");
    assert_eq!(ui.selected(), ["src"]);
}

#[test]
fn enter_activates_and_a_double_click_on_a_branch_toggles_it() {
    let mut ui = Harness::new(Files {
        activate: true,
        ..Files::new(SelectionMode::Single)
    });
    ui.click_text("README.md");
    ui.key(Named::Enter);
    assert_eq!(
        ui.messages.last(),
        Some(&Message::Activate("README.md".into()))
    );

    let mut ui = Harness::new(Files::new(SelectionMode::Single));
    let src = ui.text("src").expect("src").center();
    ui.cursor = mouse::Cursor::Available(src);
    ui.send(simulator::click());
    ui.send(simulator::click());

    assert!(ui.model.state.is_expanded("src"));
    assert_eq!(ui.selected(), ["src"]);
}

#[test]
fn two_slow_clicks_are_not_a_double_click() {
    let mut ui = Harness::new(Files {
        activate: true,
        ..Files::new(SelectionMode::Single)
    });
    let readme = ui.text("README.md").expect("readme").center();

    ui.click_at(readme);
    std::thread::sleep(Duration::from_millis(350));
    ui.click_at(readme);
    assert!(
        !ui.messages
            .iter()
            .any(|m| matches!(m, Message::Activate(_)))
    );

    ui.click_at(readme);
    assert_eq!(
        ui.messages.last(),
        Some(&Message::Activate("README.md".into()))
    );
}

#[test]
fn shift_click_works_after_the_anchor_branch_closes() {
    let mut ui = Harness::new(Files::new(SelectionMode::Multiple));
    let tree = files();
    ui.model.state.expand_all(&tree);

    ui.click_text("a.dart");
    assert_eq!(ui.selected(), ["src/widgets/a.dart"]);

    ui.model.state.set_expanded("src/widgets", false);
    ui.click_with("README.md", Modifiers::SHIFT);

    assert_eq!(ui.selected(), ["README.md"]);
}

#[test]
fn check_boxes_cascade_and_show_the_mixed_state() {
    let mut ui = Harness::new(Files {
        checkable: true,
        ..Files::new(SelectionMode::Single)
    });
    let tree = files();
    ui.model.state.expand_all(&tree);

    ui.click_text("widgets");
    ui.key(Named::Space);
    assert_eq!(
        sorted(
            &ui.model
                .state
                .checked()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        ),
        sorted(&["src/widgets/a.dart", "src/widgets/b.dart"])
    );
    assert_eq!(ui.model.state.check_state_of(&tree, "src"), None);
    assert_eq!(
        ui.model.state.check_state_of(&tree, "src/widgets"),
        Some(true)
    );

    // A click on the check box of a leaf toggles it.
    let row = ui.text("main.dart").expect("main.dart");
    ui.click_at(Point::new(row.x - spacing::XS - 10.0, row.center_y()));
    assert_eq!(ui.model.state.check_state_of(&tree, "src"), Some(true));
}

#[test]
fn multiple_selection_with_shift_control_and_control_a() {
    let mut ui = Harness::new(Files::new(SelectionMode::Multiple));

    ui.click_text("src");
    ui.key_with(Key::Named(Named::ArrowDown), Modifiers::SHIFT);
    assert_eq!(ui.selected(), sorted(&["src", "README.md"]));

    // Control+click toggles one node without touching the others.
    ui.click_with("src", Modifiers::CTRL);
    assert_eq!(ui.selected(), ["README.md"]);

    // Control+A selects every enabled shown node, the open branch's
    // children included.
    ui.model.state.set_expanded("src", true);
    ui.key_with(Key::Character("a".into()), Modifiers::CTRL);
    assert_eq!(
        ui.selected(),
        sorted(&["src", "src/main.dart", "src/widgets", "README.md"])
    );
}

#[test]
fn a_query_shows_matches_with_their_ancestors() {
    let mut ui = Harness::new(Files {
        query: "b.da".into(),
        ..Files::new(SelectionMode::Single)
    });

    assert!(ui.text("README.md").is_none());
    assert!(ui.text("a.dart").is_none());
    assert!(ui.text("main.dart").is_none());
    assert!(ui.find(selector::id("tree")).is_some());

    // Branches stay open while filtering: the arrow does nothing.
    ui.key(Named::Tab);
    ui.key(Named::Home);
    ui.key(Named::ArrowLeft);
    assert!(
        !ui.messages
            .iter()
            .any(|m| matches!(m, Message::Tree(TreeEvent::Expand { .. })))
    );
}

#[test]
fn f2_renames_the_active_node() {
    let mut ui = Harness::new(Files {
        rename: true,
        ..Files::new(SelectionMode::Single)
    });

    ui.click_text("README.md");
    ui.key(Named::F2);
    assert!(ui.focused().is_some(), "the field takes the focus");
    ui.type_text("READ.md");
    ui.key(Named::Enter);

    assert_eq!(
        ui.messages.last(),
        Some(&Message::Rename("README.md".into(), "READ.md".into()))
    );
    assert!(ui.text("READ.md").is_none(), "the field is gone");
    assert_eq!(
        ui.focused(),
        Some(ui.bounds("tree")),
        "the tree has the focus back"
    );
}

#[test]
fn escape_cancels_a_rename() {
    let mut ui = Harness::new(Files {
        rename: true,
        ..Files::new(SelectionMode::Single)
    });

    ui.click_text("README.md");
    ui.key(Named::F2);
    ui.type_text("READ.md");
    assert!(ui.text("READ.md").is_some());
    ui.key(Named::Escape);

    assert!(!ui.messages.iter().any(|m| matches!(m, Message::Rename(..))));
    assert!(ui.text("READ.md").is_none());
    assert!(ui.text("README.md").is_some());
}

#[test]
fn leaving_the_field_confirms_a_rename() {
    let mut ui = Harness::new(Files {
        rename: true,
        ..Files::new(SelectionMode::Single)
    });

    ui.click_text("README.md");
    ui.key(Named::F2);
    ui.type_text("  NOTES.md ");
    ui.click_text("src");

    assert!(
        ui.messages
            .contains(&Message::Rename("README.md".into(), "NOTES.md".into()))
    );
    assert_eq!(ui.selected(), ["src"]);
}

#[test]
fn a_rename_ends_when_its_row_goes_away() {
    let mut ui = Harness::new(Files {
        rename: true,
        ..Files::new(SelectionMode::Single)
    });

    ui.click_text("README.md");
    ui.key(Named::F2);
    assert!(ui.text("README.md").is_some(), "the field shows the label");

    ui.model.query = "main".into();
    ui.send([]);
    ui.model.query.clear();
    ui.send([]);

    assert!(!ui.messages.iter().any(|m| matches!(m, Message::Rename(..))));
    ui.click_text("README.md");
    ui.key(Named::Home);
    assert_eq!(ui.selected(), ["src"]);
}

#[test]
fn the_arrows_follow_the_reading_direction_in_rtl() {
    let mut ui = Harness::new(Files {
        direction: ReadingDirection::RightToLeft,
        ..Files::new(SelectionMode::Single)
    });

    ui.key(Named::Tab);
    ui.key(Named::ArrowLeft);
    assert!(ui.model.state.is_expanded("src"));
    ui.key(Named::ArrowRight);
    assert!(!ui.model.state.is_expanded("src"));
}

#[test]
fn typing_the_same_letter_cycles_through_its_matches() {
    let tree = TreeCollection::new(vec![
        TreeNode::leaf("alpha"),
        TreeNode::leaf("apple"),
        TreeNode::leaf("beta"),
    ])
    .expect("unique values");
    let mut ui = Harness::new(Files::with_tree(tree));

    ui.key(Named::Tab);
    let mut seen = Vec::new();
    for _ in 0..3 {
        ui.type_text("a");
        seen.extend(ui.selected());
    }

    assert_eq!(seen, ["apple", "alpha", "apple"]);
}

#[test]
fn an_empty_branch_loads_its_children_once() {
    let remote = || {
        TreeCollection::new(vec![TreeNode::branch("remote", Vec::new())]).expect("unique values")
    };
    let mut ui = Harness::new(Files {
        load: true,
        ..Files::with_tree(remote())
    });
    let row = ui.text("remote").expect("remote");
    let arrow = Point::new(row.x - spacing::XS - 10.0, row.center_y());

    ui.click_at(arrow);
    assert!(ui.model.state.is_expanded("remote"));
    assert!(ui.model.state.is_loading("remote"));

    ui.model.tree = remote()
        .insert(
            Some("remote"),
            vec![TreeNode::leaf("remote/file.txt")],
            None,
        )
        .expect("inserted");
    ui.model.state.finish_loading("remote", true);
    assert!(ui.text("remote/file.txt").is_some());

    ui.click_at(arrow);
    ui.click_at(arrow);
    let loads = ui
        .messages
        .iter()
        .filter(|m| matches!(m, Message::Load(_)))
        .count();
    assert_eq!(loads, 1);
}

#[test]
fn a_failed_load_closes_the_branch_and_reopening_retries() {
    let tree =
        TreeCollection::new(vec![TreeNode::branch("remote", Vec::new())]).expect("unique values");
    let mut ui = Harness::new(Files {
        load: true,
        ..Files::with_tree(tree)
    });
    let row = ui.text("remote").expect("remote");
    let arrow = Point::new(row.x - spacing::XS - 10.0, row.center_y());

    ui.click_at(arrow);
    ui.model.state.finish_loading("remote", false);
    assert!(!ui.model.state.is_expanded("remote"));

    ui.click_at(arrow);
    let loads = ui
        .messages
        .iter()
        .filter(|m| matches!(m, Message::Load(_)))
        .count();
    assert_eq!(loads, 2);
}

fn numbered(count: usize) -> TreeCollection<()> {
    TreeCollection::new(
        (0..count)
            .map(|i| TreeNode::leaf(format!("n{i}")))
            .collect(),
    )
    .expect("unique values")
}

#[test]
fn the_keyboard_keeps_the_active_row_fully_in_view() {
    let mut ui = Harness::new(Files {
        height: 200.0,
        ..Files::with_tree(numbered(30))
    });
    let view = ui.bounds("tree");

    ui.key(Named::Tab);
    for _ in 0..12 {
        ui.key(Named::ArrowDown);
    }
    assert_eq!(ui.selected(), ["n12"]);

    let n11 = ui.text("n11").expect("n11");
    let n12 = ui.text("n12").expect("n12");
    let height = n12.center_y() - n11.center_y();
    assert!(n12.center_y() + height / 2.0 + spacing::XS <= view.y + view.height + 0.01);
    assert!(ui.text("n0").is_none(), "rows out of view are not laid out");

    for _ in 0..12 {
        ui.key(Named::ArrowUp);
    }
    let n0 = ui.text("n0").expect("n0");
    assert!(n0.y > view.y);
}

#[test]
fn only_the_rows_in_view_are_laid_out_and_the_wheel_scrolls() {
    let mut ui = Harness::new(Files {
        height: 200.0,
        ..Files::with_tree(numbered(10_000))
    });
    let view = ui.bounds("tree");

    assert_eq!(
        ui.rows(&["n0", "n5", "n9", "n100"]),
        [true, true, false, false]
    );

    ui.cursor = mouse::Cursor::Available(view.center());
    ui.send([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Pixels { x: 0.0, y: -3400.0 },
    })]);

    assert_eq!(ui.rows(&["n0", "n100", "n102"]), [false, true, true]);

    ui.key(Named::Tab);
    ui.key(Named::End);
    assert_eq!(ui.selected(), ["n9999"]);
    assert!(ui.text("n9999").is_some());
}

#[test]
fn a_read_only_tree_ignores_input_and_is_skipped() {
    let mut ui = Harness::new(Files {
        read_only: true,
        ..Files::new(SelectionMode::Single)
    });

    ui.click_text("README.md");
    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(ui.bounds("after")));
    assert!(ui.messages.is_empty());
}

#[test]
fn disabled_nodes_cannot_be_selected() {
    let mut ui = Harness::new(Files::new(SelectionMode::Single));

    ui.click_text("LICENSE");
    assert!(ui.selected().is_empty());
}

#[test]
fn rows_and_disclosure_arrows_show_the_click_cursor() {
    // Building a harness loads the fonts the text is measured with.
    let _fonts = Harness::new(Files::new(SelectionMode::Single));
    let tree = files();
    let state = TreeState::default();
    let view = || -> Element<'_, Message> {
        tree_view::tree_view(&tree, &state)
            .on_event(Message::Tree)
            .into()
    };

    let size = Size::new(300.0, 300.0);
    let control = oito2_iced::density::control_height();
    let readme = Point::new(120.0, spacing::XS + control * 1.5);
    let arrow = Point::new(
        spacing::XS + spacing::XS + 10.0,
        spacing::XS + control * 0.5,
    );
    let license = Point::new(120.0, spacing::XS + control * 2.5);

    assert_eq!(
        support::mouse_interaction(view(), size, readme),
        mouse::Interaction::Pointer
    );
    assert_eq!(
        support::mouse_interaction(view(), size, arrow),
        mouse::Interaction::Pointer
    );
    assert_ne!(
        support::mouse_interaction(view(), size, license),
        mouse::Interaction::Pointer
    );
}

fn scene_state(tree: &TreeCollection<()>) -> TreeState {
    let mut state = TreeState::new(SelectionMode::Single)
        .with_selected(["lib/src/app.dart"])
        .with_checked(["lib/main.dart", "lib/src/app.dart"]);
    state.expand_all(tree);
    state
}

fn scene_tree() -> TreeCollection<()> {
    TreeCollection::from_paths(
        [
            "lib/main.dart",
            "lib/src/app.dart",
            "lib/src/home.dart",
            "test/app_test.dart",
            "README.md",
        ],
        "/",
    )
}

fn snapshot(theme: &Theme, name: &str, checkable: bool, direction: ReadingDirection) {
    let tree = scene_tree();
    let state = scene_state(&tree);
    let view: Element<'_, Message> = tree_view::tree_view(&tree, &state)
        .on_event(Message::Tree)
        .checkable(checkable)
        .direction(direction)
        .into();
    let mut ui = simulator::Simulator::with_size(support::settings(), (260.0, 300.0), view);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn tree_view_light() {
    snapshot(
        &Theme::light(),
        "tree_view_light",
        true,
        ReadingDirection::LeftToRight,
    );
}

#[test]
fn tree_view_dark() {
    snapshot(
        &Theme::dark(),
        "tree_view_dark",
        true,
        ReadingDirection::LeftToRight,
    );
}

#[test]
fn tree_view_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(
        &theme,
        "tree_view_light_high_contrast",
        false,
        ReadingDirection::LeftToRight,
    );
}

#[test]
fn tree_view_rtl() {
    snapshot(
        &Theme::light(),
        "tree_view_rtl",
        true,
        ReadingDirection::RightToLeft,
    );
}

#[test]
fn tree_view_focused_and_filtered() {
    let tree = scene_tree();
    let state = scene_state(&tree);
    let view: Element<'_, Message> = focus::navigation(
        tree_view::tree_view(&tree, &state)
            .on_event(Message::Tree)
            .query("app"),
    )
    .into();
    let mut ui = simulator::Simulator::with_size(support::settings(), (260.0, 200.0), view);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Home);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "tree_view_focused_and_filtered");
}

#[test]
fn tree_view_scrolled() {
    let tree = numbered(200);
    let state = TreeState::default();
    let view: Element<'_, Message> = tree_view::tree_view(&tree, &state)
        .on_event(Message::Tree)
        .into();
    let mut ui = simulator::Simulator::with_size(support::settings(), (200.0, 160.0), view);

    ui.point_at(Point::new(100.0, 80.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: -2.0 },
    })]);
    let snapshot = ui.snapshot(&Theme::dark()).expect("snapshot");

    support::golden!(&snapshot, "tree_view_scrolled");
}

#[test]
fn tree_view_renaming_and_loading() {
    let tree = TreeCollection::new(vec![
        TreeNode::<()>::branch("remote", Vec::new()),
        TreeNode::leaf("notes.txt"),
    ])
    .expect("unique values");
    let mut state = TreeState::default().with_expanded(["remote"]);
    state.start_loading("remote");
    let view: Element<'_, Message> = focus::navigation(
        tree_view::tree_view(&tree, &state)
            .on_event(Message::Tree)
            .on_rename(Message::Rename)
            .animated(false),
    )
    .into();
    let mut ui = simulator::Simulator::with_size(support::settings(), (220.0, 100.0), view);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::End);
    ui.tap_key(Named::F2);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "tree_view_renaming_and_loading");
}
