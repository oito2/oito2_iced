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

//! The Commands page: commands with keyboard shortcuts, their labels on
//! each system, the sequence matcher and the command palette.

use std::sync::LazyLock;

use iced::widget::{column, text};
use iced::{Alignment, Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::shortcuts::{Command, OperatingSystem, Shortcut};
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::command_palette::command_palette;

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("commands.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    count: u32,
    last: Option<&'static str>,
    open: bool,
    query: String,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// Opens the command palette of the showcase.
    OpenAppPalette,
    /// Counts one more.
    Count,
    /// Sets the count back to zero.
    Reset,
    /// Opens the palette of the page.
    Open,
    /// The query of the page's palette changed.
    Query(String),
    /// The page's palette closed.
    Close,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::OpenAppPalette => shell.request(Action::OpenPalette),
        Message::Count => {
            state.count += 1;
            state.last = Some("Count one more");
        }
        Message::Reset => {
            state.count = 0;
            state.last = Some("Reset the count");
        }
        // #region palette-update
        Message::Open => {
            state.open = true;
            state.query.clear();
        }
        Message::Query(query) => state.query = query,
        Message::Close => state.open = false,
        // #endregion
    }

    Task::none()
}

// #region commands
/// The commands of the page.
fn commands() -> Vec<Command<Message>> {
    let mut count =
        Command::new("demo.count", "Count one more", Message::Count).with_group("This page");
    let mut reset = Command::new("demo.reset", "Reset the count", Message::Reset)
        .with_group("This page")
        .with_description("Sets the count back to zero");

    if let Ok(shortcut) = Shortcut::parse("Mod+J") {
        count = count.with_shortcut(shortcut);
    }
    if let Ok(shortcut) = Shortcut::parse("R R") {
        reset = reset.with_shortcut(shortcut);
    }

    vec![count, reset]
}
// #endregion

/// The shortcuts of the labels example, with their labels on Linux and
/// macOS.
static LABELS: LazyLock<Vec<[String; 3]>> = LazyLock::new(|| {
    // #region labels
    ["Mod+K", "Mod+Shift+P", "Alt+Enter", "G H"]
        .map(|text| {
            [
                text.to_owned(),
                label(text, OperatingSystem::Linux),
                label(text, OperatingSystem::Macos),
            ]
        })
        .to_vec()
    // #endregion
});

/// The label of the shortcut `text` on `os`.
fn label(text: &str, os: OperatingSystem) -> String {
    Shortcut::parse(text).map_or_else(|error| error.to_string(), |shortcut| shortcut.label(os))
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        button::outlined("Open the command palette")
            .icon(lucide::COMMAND)
            .on_press(Message::OpenAppPalette),
        // #endregion
    );

    let rows: Vec<Vec<&str>> = LABELS
        .iter()
        .map(|row| row.iter().map(String::as_str).collect())
        .collect();
    let labels = doc::example(
        ctx,
        SOURCE,
        &["labels"],
        doc::table(&["Shortcut", "Linux and Windows", "macOS"], &rows),
    );

    let status = match state.last {
        Some(last) => format!("Count: {} · last command: {last}", state.count),
        None => format!("Count: {}", state.count),
    };

    let commands = commands();
    let commands: Vec<&Command<Message>> = commands.iter().collect();

    let palette = doc::example(
        ctx,
        SOURCE,
        &["commands", "palette", "palette-update"],
        // #region palette
        command_palette(
            column![
                text(status).class(TextClass::Variant),
                button::outlined("Search the commands of this page").on_press(Message::Open),
            ]
            .spacing(spacing::SM)
            .align_x(Alignment::Center),
            &ctx.l10n,
            &commands,
            &state.query,
            Message::Query,
            Message::Close,
        )
        .open(state.open),
        // #endregion
    );

    doc::page(
        "Commands",
        "Actions that run from keyboard shortcuts or a searchable command palette, with \
         shortcut labels in the notation of each system.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "A `Command` has a unique `id`, a `label` and the message it publishes, and \
                 optionally a `Shortcut`, a group, a description and keywords. A \
                 `CommandRegistry` holds the commands of a scope: `register` adds or replaces \
                 one by id, `search` finds them, `find_by_shortcut` finds the command of a \
                 shortcut and `conflicts` lists the pairs whose shortcuts clash, or where one \
                 is a prefix of the other. A disabled command neither reacts to its shortcut \
                 nor appears in searches.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::shortcuts::{Command, CommandRegistry, SequenceMatcher, Shortcut};",
            ),
            doc::prose(
                "The application runs the shortcuts: it listens to `iced::keyboard::listen()`, \
                 turns each key press into a `KeyChord` with `KeyChord::from_keyboard_event` and \
                 hands it to a `SequenceMatcher`, which returns the command to run once the \
                 chords of its shortcut are complete. Scopes are matched innermost first, and \
                 an inner command replaces an outer one with the same id.",
            ),
            doc::code(
                ctx,
                "Message::Key(event) => {\n    \
                     if let Some(chord) = KeyChord::from_keyboard_event(&event)\n        \
                         && let KeyMatch::Run(command) =\n            \
                             self.matcher.press(&[&self.commands], &chord, os, typing, Instant::now())\n    \
                     {\n        \
                         return self.update(command.action.clone());\n    \
                     }\n\
                 }",
            ),
            doc::note(
                "Shortcuts without Control, Alt, Command or the primary modifier, such as `R R` \
                 or `Shift+/`, don't run while a text input has the focus (`typing`), so they \
                 don't steal typing.",
            ),
            doc::heading("Examples"),
            doc::subheading("Shortcuts and their labels"),
            doc::prose(
                "`Shortcut::parse` reads chords separated by spaces and keys within a chord \
                 joined by `+`. `Mod` is the primary modifier: Command on macOS and iOS, \
                 Control elsewhere. A sequence such as `G H` waits up to the timeout of the \
                 `SequenceMatcher`, one second by default, between chords. `label` writes a \
                 shortcut in the notation of an `OperatingSystem`; Apple systems show the \
                 modifiers as symbols.",
            ),
            labels,
            doc::subheading("Command palette"),
            doc::prose(
                "`command_palette` wraps the root of the view. While `open`, it dims the view \
                 and shows a search field above the commands that match the query, each with \
                 its group, description and shortcut. Typing keeps the commands whose label, \
                 description, group and keywords contain every word, ignoring case and \
                 accents; those whose label starts with the first word come first \
                 (`matching_commands`). Choosing a command publishes the close message, then \
                 the action of the command. The application keeps whether the palette is open \
                 and its query.",
            ),
            palette,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Down, Up",
                        "In the palette, moves the highlight while the focus stays in the \
                         field.",
                    ],
                    vec![
                        "Enter",
                        "In the palette, closes it and runs the highlighted command.",
                    ],
                    vec![
                        "Escape",
                        "In the palette, closes it from an empty field or outside the field.",
                    ],
                    vec!["Tab", "Cycles the focus inside the palette."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::shortcuts::Command",
                    "oito2_iced::shortcuts::CommandRegistry",
                    "oito2_iced::shortcuts::Shortcut",
                    "oito2_iced::shortcuts::KeyChord",
                    "oito2_iced::shortcuts::SequenceMatcher",
                    "oito2_iced::shortcuts::KeyMatch",
                    "oito2_iced::shortcuts::uses_command_key",
                    "oito2_iced::widget::command_palette::command_palette",
                    "oito2_iced::widget::command_palette::matching_commands",
                ],
            ),
        ],
    )
}
