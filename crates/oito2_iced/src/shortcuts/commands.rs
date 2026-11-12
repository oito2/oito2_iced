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

//! Commands: actions with optional shortcuts, a registry and a matcher
//! that turns key presses into the command to run.
//!
//! A command carries a message `M` instead of a callback: when a shortcut
//! or a palette entry selects it, the application sends that message to its
//! `update` function.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use super::key::{KeyChord, OperatingSystem, Shortcut};
use crate::collections::fold_text;

/// An action the user can run by a keyboard shortcut or from a command
/// palette.
#[derive(Debug, Clone, PartialEq)]
pub struct Command<M> {
    /// A unique identifier; a command of an inner scope overrides one of an
    /// outer scope with the same id.
    pub id: String,
    /// The name shown in a command palette.
    pub label: String,
    /// The message produced when the command runs.
    pub action: M,
    /// The keyboard shortcut, if any.
    pub shortcut: Option<Shortcut>,
    /// A longer explanation shown in a command palette.
    pub description: Option<String>,
    /// The palette group, such as "Navigation" or "View".
    pub group: Option<String>,
    /// Extra words the palette search matches.
    pub keywords: Vec<String>,
    /// Whether the command can run now; disabled commands neither react to
    /// their shortcut nor appear in searches.
    pub enabled: bool,
}

impl<M> Command<M> {
    /// Creates an enabled command without a shortcut.
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>, action: M) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            action,
            shortcut: None,
            description: None,
            group: None,
            keywords: Vec::new(),
            enabled: true,
        }
    }

    /// Returns the command with `shortcut`.
    #[must_use]
    pub fn with_shortcut(mut self, shortcut: Shortcut) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    /// Returns the command with a `description`.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the command in the palette `group`.
    #[must_use]
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Returns the command with extra search `keywords`.
    #[must_use]
    pub fn with_keywords<S: Into<String>>(mut self, keywords: impl IntoIterator<Item = S>) -> Self {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }

    /// Returns the command with its enabled state set to `enabled`.
    #[must_use]
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Returns the pairs of `commands` whose shortcuts clash on `os`: the same
/// shortcut, or one that is a prefix of another (the longer one could never
/// run). Commands without a shortcut never clash.
#[must_use]
pub fn conflicts<M>(
    commands: &[Command<M>],
    os: OperatingSystem,
) -> Vec<(&Command<M>, &Command<M>)> {
    let mut result = Vec::new();
    for (i, first) in commands.iter().enumerate() {
        for second in &commands[i + 1..] {
            if let (Some(a), Some(b)) = (&first.shortcut, &second.shortcut)
                && (a.starts_with(b, os) || b.starts_with(a, os))
            {
                result.push((first, second));
            }
        }
    }
    result
}

/// The commands of one scope, such as the application or a panel.
///
/// Scopes nest: functions that take a slice of scopes read it innermost
/// first, and a command replaces the commands of the same id in the scopes
/// after it.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandRegistry<M> {
    commands: Vec<Command<M>>,
}

impl<M> Default for CommandRegistry<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M> CommandRegistry<M> {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Registers `command`. A command with the same id is replaced in place
    /// and returned.
    pub fn register(&mut self, command: Command<M>) -> Option<Command<M>> {
        match self.commands.iter_mut().find(|c| c.id == command.id) {
            Some(existing) => Some(std::mem::replace(existing, command)),
            None => {
                self.commands.push(command);
                None
            }
        }
    }

    /// Removes and returns the command with `id`.
    pub fn unregister(&mut self, id: &str) -> Option<Command<M>> {
        let index = self.commands.iter().position(|c| c.id == id)?;
        Some(self.commands.remove(index))
    }

    /// Returns the command with `id`.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Command<M>> {
        self.commands.iter().find(|c| c.id == id)
    }

    /// Mutable access to the command with `id`, such as to enable or
    /// disable it.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Command<M>> {
        self.commands.iter_mut().find(|c| c.id == id)
    }

    /// The registered commands, in registration order.
    #[must_use]
    pub fn commands(&self) -> &[Command<M>] {
        &self.commands
    }

    /// Returns the enabled commands, in registration order.
    #[must_use]
    pub fn enabled(&self) -> Vec<&Command<M>> {
        self.commands.iter().filter(|c| c.enabled).collect()
    }

    /// Returns the first enabled command whose shortcut is `shortcut` on
    /// `os`.
    #[must_use]
    pub fn find_by_shortcut(
        &self,
        shortcut: &Shortcut,
        os: OperatingSystem,
    ) -> Option<&Command<M>> {
        self.commands.iter().find(|command| {
            command.enabled
                && command
                    .shortcut
                    .as_ref()
                    .is_some_and(|own| own.matches(shortcut, os))
        })
    }

    /// Returns the enabled command whose shortcut is the single chord
    /// `chord` on `os`.
    #[must_use]
    pub fn find_by_chord(&self, chord: &KeyChord, os: OperatingSystem) -> Option<&Command<M>> {
        self.find_by_shortcut(&Shortcut::single(chord.clone()), os)
    }

    /// Returns the pairs of commands of this registry whose shortcuts clash
    /// on `os`; see [`conflicts`].
    #[must_use]
    pub fn conflicts(&self, os: OperatingSystem) -> Vec<(&Command<M>, &Command<M>)> {
        conflicts(&self.commands, os)
    }

    /// Returns the enabled commands matching `query`, best first.
    ///
    /// The query is folded (case and accents ignored) and split at spaces;
    /// every word must appear in the label, keywords, group or description.
    /// Commands whose label starts with the whole query come first, then
    /// those whose label contains it, then the rest; commands of equal rank
    /// keep their registration order. An empty query returns every enabled
    /// command.
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&Command<M>> {
        let folded = fold_text(query.trim());
        let words: Vec<&str> = folded.split_whitespace().collect();
        let mut ranked: Vec<(u8, &Command<M>)> = Vec::new();
        for command in self.commands.iter().filter(|c| c.enabled) {
            let label = fold_text(&command.label);
            let mut extra = command.keywords.join(" ");
            for text in [&command.group, &command.description].into_iter().flatten() {
                extra.push(' ');
                extra.push_str(text);
            }
            let extra = fold_text(&extra);
            if !words
                .iter()
                .all(|word| label.contains(word) || extra.contains(word))
            {
                continue;
            }
            let rank = if folded.is_empty() || label.starts_with(&folded) {
                0
            } else if label.contains(&folded) {
                1
            } else {
                2
            };
            ranked.push((rank, command));
        }
        ranked.sort_by_key(|(rank, _)| *rank);
        ranked.into_iter().map(|(_, command)| command).collect()
    }
}

/// Returns the enabled commands available across `scopes`, innermost scope
/// first. A command of an inner scope replaces the commands of the same id
/// of the scopes after it.
#[must_use]
pub fn available_in<'a, M>(scopes: &[&'a CommandRegistry<M>]) -> Vec<&'a Command<M>> {
    let mut result = Vec::new();
    let mut ids: HashSet<&str> = HashSet::new();
    for scope in scopes {
        for command in &scope.commands {
            if command.enabled && ids.insert(command.id.as_str()) {
                result.push(command);
            }
        }
    }
    result
}

/// The outcome of [`SequenceMatcher::press`].
#[derive(Debug)]
pub enum KeyMatch<'a, M> {
    /// The press completed the shortcut of this command; run it.
    Run(&'a Command<M>),
    /// The press continues the shortcut of some command; the press is
    /// consumed and the matcher waits for the next chord.
    Pending,
    /// The press matches no shortcut and should be handled elsewhere.
    NoMatch,
}

impl<M> Clone for KeyMatch<'_, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M> Copy for KeyMatch<'_, M> {}

/// Matches a stream of key presses against the shortcuts of command scopes,
/// including multi-chord sequences such as `G H`.
///
/// The matcher keeps the chords of a sequence in progress. It reads the
/// clock from the `now` argument of [`press`](Self::press), so it needs no
/// timer: a sequence is dropped when more than the timeout passed since its
/// last chord.
#[derive(Debug, Clone)]
pub struct SequenceMatcher {
    pending: Vec<KeyChord>,
    last_press: Option<Instant>,
    timeout: Duration,
}

impl Default for SequenceMatcher {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
    }
}

impl SequenceMatcher {
    /// Creates a matcher that waits up to `timeout` between the chords of a
    /// sequence.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        Self {
            pending: Vec::new(),
            last_press: None,
            timeout,
        }
    }

    /// The chords of the sequence in progress.
    #[must_use]
    pub fn pending(&self) -> &[KeyChord] {
        &self.pending
    }

    /// Drops the sequence in progress.
    pub fn reset(&mut self) {
        self.pending.clear();
        self.last_press = None;
    }

    /// Matches the press of `chord` at time `now` against `scopes`,
    /// innermost first. `typing` tells that a text input has focus: then
    /// only shortcuts that start with a Control, Alt, Command or primary
    /// chord can run, so plain keys keep typing.
    ///
    /// Commands that are disabled, have no shortcut, or whose id appears in
    /// an inner scope (even disabled there) are not candidates. A full match
    /// wins over a longer shortcut it begins; the first candidate in scope
    /// order wins. When a sequence in progress does not continue with
    /// `chord`, it is dropped and `chord` is tried as the start of a new
    /// one.
    pub fn press<'a, M>(
        &mut self,
        scopes: &[&'a CommandRegistry<M>],
        chord: &KeyChord,
        os: OperatingSystem,
        typing: bool,
        now: Instant,
    ) -> KeyMatch<'a, M> {
        if let Some(last) = self.last_press
            && now.saturating_duration_since(last) > self.timeout
        {
            self.reset();
        }
        let result = self.try_match(scopes, chord, os, typing, now);
        if matches!(result, KeyMatch::NoMatch) && !self.pending.is_empty() {
            self.reset();
            return self.try_match(scopes, chord, os, typing, now);
        }
        result
    }

    /// Matches the pending chords plus `chord`; keeps `chord` pending when
    /// it continues a longer shortcut.
    fn try_match<'a, M>(
        &mut self,
        scopes: &[&'a CommandRegistry<M>],
        chord: &KeyChord,
        os: OperatingSystem,
        typing: bool,
        now: Instant,
    ) -> KeyMatch<'a, M> {
        let mut chords = self.pending.clone();
        chords.push(chord.clone());
        let Some(sequence) = Shortcut::from_chords(chords) else {
            return KeyMatch::NoMatch;
        };
        let mut overridden: HashSet<&str> = HashSet::new();
        let mut candidates: Vec<(&'a Command<M>, &'a Shortcut)> = Vec::new();
        for scope in scopes {
            for command in &scope.commands {
                if let Some(shortcut) = &command.shortcut
                    && command.enabled
                    && !overridden.contains(command.id.as_str())
                    && (!typing || shortcut.chords()[0].has_command_modifier())
                {
                    candidates.push((command, shortcut));
                }
            }
            overridden.extend(scope.commands.iter().map(|c| c.id.as_str()));
        }
        if let Some((command, _)) = candidates
            .iter()
            .find(|(_, shortcut)| shortcut.matches(&sequence, os))
        {
            self.reset();
            return KeyMatch::Run(command);
        }
        if candidates
            .iter()
            .any(|(_, shortcut)| shortcut.starts_with(&sequence, os))
        {
            self.pending.push(chord.clone());
            self.last_press = Some(now);
            return KeyMatch::Pending;
        }
        KeyMatch::NoMatch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use OperatingSystem::{Linux, Macos};
    use iced::keyboard::Key;

    fn shortcut(text: &str) -> Shortcut {
        Shortcut::parse(text).expect("valid shortcut")
    }

    fn chord(text: &str) -> KeyChord {
        shortcut(text).chords()[0].clone()
    }

    fn command(id: &'static str, text: &str) -> Command<&'static str> {
        Command::new(id, id, id).with_shortcut(shortcut(text))
    }

    /// A registry with a palette command, a two-key home sequence and a
    /// disabled command.
    fn registry() -> CommandRegistry<&'static str> {
        let mut registry = CommandRegistry::new();
        registry.register(command("palette", "Mod+K"));
        registry.register(command("home", "G H"));
        registry.register(command("off", "Mod+D").with_enabled(false));
        registry
    }

    fn run<'a>(
        matcher: &mut SequenceMatcher,
        scopes: &[&'a CommandRegistry<&'static str>],
        text: &str,
        typing: bool,
        now: Instant,
    ) -> Option<&'a str> {
        match matcher.press(scopes, &chord(text), Linux, typing, now) {
            KeyMatch::Run(command) => Some(command.action),
            KeyMatch::Pending | KeyMatch::NoMatch => None,
        }
    }

    #[test]
    fn runs_a_command_from_its_shortcut() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        assert_eq!(
            run(&mut matcher, &[&registry], "Ctrl+K", false, now),
            Some("palette")
        );
        assert_eq!(run(&mut matcher, &[&registry], "Ctrl+D", false, now), None);
    }

    #[test]
    fn the_primary_modifier_follows_the_system() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        let ctrl_k = chord("Ctrl+K");
        let cmd_k = chord("Cmd+K");
        assert!(matches!(
            matcher.press(&[&registry], &ctrl_k, Macos, false, now),
            KeyMatch::NoMatch
        ));
        assert!(matches!(
            matcher.press(&[&registry], &cmd_k, Macos, false, now),
            KeyMatch::Run(_)
        ));
    }

    #[test]
    fn runs_a_sequence_pressed_in_time() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        assert!(matches!(
            matcher.press(&[&registry], &chord("G"), Linux, false, now),
            KeyMatch::Pending
        ));
        assert_eq!(matcher.pending().len(), 1);
        let later = now + Duration::from_millis(900);
        assert_eq!(
            run(&mut matcher, &[&registry], "H", false, later),
            Some("home")
        );
        assert!(matcher.pending().is_empty());
    }

    #[test]
    fn drops_a_sequence_after_the_timeout() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        matcher.press(&[&registry], &chord("G"), Linux, false, now);
        let later = now + Duration::from_secs(2);
        assert_eq!(run(&mut matcher, &[&registry], "H", false, later), None);
        assert!(matcher.pending().is_empty());
    }

    #[test]
    fn a_broken_sequence_retries_the_press_as_a_new_start() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        matcher.press(&[&registry], &chord("G"), Linux, false, now);
        assert!(matches!(
            matcher.press(&[&registry], &chord("G"), Linux, false, now),
            KeyMatch::Pending
        ));
        assert_eq!(
            run(&mut matcher, &[&registry], "H", false, now),
            Some("home")
        );
        assert_eq!(
            run(&mut matcher, &[&registry], "Ctrl+K", false, now),
            Some("palette")
        );
    }

    #[test]
    fn plain_key_shortcuts_skip_text_fields() {
        let registry = registry();
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        assert!(matches!(
            matcher.press(&[&registry], &chord("G"), Linux, true, now),
            KeyMatch::NoMatch
        ));
        assert_eq!(run(&mut matcher, &[&registry], "H", true, now), None);
        assert_eq!(
            run(&mut matcher, &[&registry], "Ctrl+K", true, now),
            Some("palette")
        );
    }

    #[test]
    fn an_inner_command_hides_the_outer_shortcut_of_its_id() {
        let mut outer = CommandRegistry::new();
        outer.register(command("save", "Ctrl+Shift+S"));
        let mut inner = CommandRegistry::new();
        inner.register(command("save", "Ctrl+S").with_enabled(false));
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        assert_eq!(
            run(&mut matcher, &[&inner, &outer], "Ctrl+Shift+S", false, now),
            None
        );
        assert_eq!(
            run(&mut matcher, &[&inner, &outer], "Ctrl+S", false, now),
            None
        );
        assert_eq!(
            run(&mut matcher, &[&outer], "Ctrl+Shift+S", false, now),
            Some("save")
        );
    }

    #[test]
    fn inner_scopes_win_on_the_same_shortcut() {
        let mut outer = CommandRegistry::new();
        outer.register(command("outer", "Ctrl+S"));
        let mut inner = CommandRegistry::new();
        inner.register(command("inner", "Ctrl+S"));
        let mut matcher = SequenceMatcher::default();
        let now = Instant::now();
        assert_eq!(
            run(&mut matcher, &[&inner, &outer], "Ctrl+S", false, now),
            Some("inner")
        );
    }

    #[test]
    fn available_commands_let_inner_scopes_override_by_id() {
        let mut outer = CommandRegistry::new();
        outer.register(Command::new("save", "Save all", 1));
        outer.register(Command::new("quit", "Quit", 2));
        let mut inner = CommandRegistry::new();
        inner.register(Command::new("save", "Save file", 3));
        let labels: Vec<_> = available_in(&[&inner, &outer])
            .iter()
            .map(|c| c.label.as_str())
            .collect();
        assert_eq!(labels, ["Save file", "Quit"]);
    }

    #[test]
    fn available_commands_leave_out_disabled_ones() {
        let mut scope = CommandRegistry::new();
        scope.register(Command::new("a", "A", 1).with_enabled(false));
        scope.register(Command::new("b", "B", 2));
        assert_eq!(available_in(&[&scope]).len(), 1);
        assert_eq!(scope.enabled().len(), 1);
    }

    #[test]
    fn reports_clashing_and_prefix_shortcuts() {
        let commands = vec![
            command("a", "G"),
            command("b", "G H"),
            command("c", "Ctrl+K"),
            command("d", "Mod+K"),
            Command::new("e", "E", "e"),
        ];
        let ids = |os| -> Vec<String> {
            conflicts(&commands, os)
                .iter()
                .map(|(a, b)| format!("{}{}", a.id, b.id))
                .collect()
        };
        assert_eq!(ids(Linux), ["ab", "cd"]);
        assert_eq!(ids(Macos), ["ab"]);
    }

    #[test]
    fn registers_replaces_and_unregisters_by_id() {
        let mut registry = CommandRegistry::new();
        assert!(registry.register(Command::new("a", "First", 1)).is_none());
        registry.register(Command::new("b", "B", 2));
        let replaced = registry.register(Command::new("a", "Second", 3));
        assert_eq!(replaced.map(|c| c.label), Some("First".to_owned()));
        assert_eq!(registry.commands().len(), 2);
        assert_eq!(registry.commands()[0].label, "Second");
        assert_eq!(registry.get("b").map(|c| c.action), Some(2));
        if let Some(command) = registry.get_mut("b") {
            command.enabled = false;
        }
        assert_eq!(registry.enabled().len(), 1);
        assert!(registry.unregister("a").is_some());
        assert!(registry.unregister("a").is_none());
    }

    #[test]
    fn looks_commands_up_by_shortcut() {
        let registry = registry();
        let found = registry.find_by_shortcut(&shortcut("Ctrl+K"), Linux);
        assert_eq!(found.map(|c| c.id.as_str()), Some("palette"));
        assert!(
            registry
                .find_by_shortcut(&shortcut("Cmd+K"), Linux)
                .is_none()
        );
        assert!(
            registry
                .find_by_shortcut(&shortcut("Ctrl+D"), Linux)
                .is_none()
        );
        let by_chord = registry.find_by_chord(&chord("Cmd+K"), Macos);
        assert_eq!(by_chord.map(|c| c.id.as_str()), Some("palette"));
        assert!(
            registry
                .find_by_chord(&KeyChord::new(Key::Unidentified), Linux)
                .is_none()
        );
    }

    fn palette() -> CommandRegistry<&'static str> {
        let mut registry = CommandRegistry::new();
        registry.register(
            Command::new("theme", "Toggle dark style", "theme")
                .with_group("View")
                .with_shortcut(shortcut("Mod+Shift+L")),
        );
        registry.register(
            Command::new("settings", "Open settings", "settings").with_keywords(["preferences"]),
        );
        registry.register(Command::new("search", "Search pages", "search"));
        registry
    }

    fn found(registry: &CommandRegistry<&'static str>, query: &str) -> Vec<&'static str> {
        registry.search(query).iter().map(|c| c.action).collect()
    }

    #[test]
    fn search_lists_everything_for_an_empty_query() {
        assert_eq!(found(&palette(), ""), ["theme", "settings", "search"]);
        assert_eq!(found(&palette(), "  "), ["theme", "settings", "search"]);
    }

    #[test]
    fn search_filters_by_label_keywords_and_group() {
        let registry = palette();
        assert_eq!(found(&registry, "prefer"), ["settings"]);
        assert_eq!(found(&registry, "VIEW"), ["theme"]);
        assert_eq!(found(&registry, "open set"), ["settings"]);
        assert!(found(&registry, "zzz").is_empty());
    }

    #[test]
    fn search_ranks_label_prefixes_first_and_keeps_order_among_equals() {
        let mut registry = CommandRegistry::new();
        registry.register(Command::new("a", "Reopen file", "a"));
        registry.register(Command::new("b", "Open file", "b"));
        registry.register(Command::new("c", "Other", "c").with_keywords(["open"]));
        registry.register(Command::new("d", "Open folder", "d"));
        assert_eq!(found(&registry, "open"), ["b", "d", "a", "c"]);
        let mut many = CommandRegistry::new();
        for i in 0..60 {
            many.register(Command::new(format!("c{i}"), format!("Command {i:02}"), i));
        }
        let labels: Vec<_> = many
            .search("mand")
            .iter()
            .map(|c| c.label.clone())
            .collect();
        let mut sorted = labels.clone();
        sorted.sort();
        assert_eq!(labels, sorted);
    }

    #[test]
    fn search_ignores_disabled_commands_and_accents() {
        let mut registry = CommandRegistry::new();
        registry.register(Command::new("a", "Ação rápida", "a"));
        registry.register(Command::new("b", "Ação lenta", "b").with_enabled(false));
        assert_eq!(found(&registry, "acao"), ["a"]);
    }
}
