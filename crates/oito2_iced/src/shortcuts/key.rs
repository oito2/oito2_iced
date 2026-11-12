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

//! Key chords, shortcuts and their text forms.

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use iced::keyboard::key::Named;
use iced::keyboard::{Key, Modifiers};

pub use oito2_foundation::OperatingSystem;

/// Whether `os` uses Command, rather than Control, as the primary shortcut
/// modifier: macOS and iOS.
#[must_use]
pub fn uses_command_key(os: OperatingSystem) -> bool {
    matches!(os, OperatingSystem::Macos | OperatingSystem::Ios)
}

/// One key press of a shortcut: a key with modifiers.
///
/// `primary` stands for the platform's main shortcut modifier: Command on
/// macOS and iOS, Control elsewhere. Shortcuts written with it, such as
/// `Mod+K`, read naturally on every platform.
///
/// Letter keys are stored as lower-case characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeyChord {
    /// The key, without modifiers.
    pub key: Key,
    /// Whether the platform's primary modifier is held.
    pub primary: bool,
    /// Whether Control is held.
    pub control: bool,
    /// Whether Shift is held.
    pub shift: bool,
    /// Whether Alt (Option on Apple keyboards) is held.
    pub alt: bool,
    /// Whether Meta (Command on Apple keyboards, Super elsewhere) is held.
    pub meta: bool,
}

/// The text names of the special keys: the parse name, the key, the text
/// label and the Apple label.
const SPECIAL_KEYS: [(&str, Named, &str, &str); 14] = [
    ("enter", Named::Enter, "Enter", "↩"),
    ("esc", Named::Escape, "Esc", "⎋"),
    ("backspace", Named::Backspace, "Backspace", "⌫"),
    ("delete", Named::Delete, "Delete", "⌦"),
    ("tab", Named::Tab, "Tab", "⇥"),
    ("space", Named::Space, "Space", "Space"),
    ("up", Named::ArrowUp, "↑", "↑"),
    ("down", Named::ArrowDown, "↓", "↓"),
    ("left", Named::ArrowLeft, "←", "←"),
    ("right", Named::ArrowRight, "→", "→"),
    ("pageup", Named::PageUp, "PgUp", "⇞"),
    ("pagedown", Named::PageDown, "PgDn", "⇟"),
    ("home", Named::Home, "Home", "↖"),
    ("end", Named::End, "End", "↘"),
];

/// The function keys F1 to F12.
const FUNCTION_KEYS: [Named; 12] = [
    Named::F1,
    Named::F2,
    Named::F3,
    Named::F4,
    Named::F5,
    Named::F6,
    Named::F7,
    Named::F8,
    Named::F9,
    Named::F10,
    Named::F11,
    Named::F12,
];

/// The modifier names accepted by [`Shortcut::parse`].
const MODIFIER_NAMES: [&str; 11] = [
    "mod", "primary", "ctrl", "control", "shift", "alt", "option", "meta", "cmd", "command",
    "super",
];

impl KeyChord {
    /// Creates a chord of `key` with no modifiers.
    #[must_use]
    pub fn new(key: Key) -> Self {
        Self {
            key,
            primary: false,
            control: false,
            shift: false,
            alt: false,
            meta: false,
        }
    }

    /// Creates a chord of the character key `text`, lower-cased.
    #[must_use]
    pub fn character(text: &str) -> Self {
        Self::new(normalized(Key::Character(text.into())))
    }

    /// Returns the chord with the primary modifier.
    #[must_use]
    pub fn with_primary(mut self) -> Self {
        self.primary = true;
        self
    }

    /// Returns the chord with Control.
    #[must_use]
    pub fn with_control(mut self) -> Self {
        self.control = true;
        self
    }

    /// Returns the chord with Shift.
    #[must_use]
    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Returns the chord with Alt.
    #[must_use]
    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Returns the chord with Meta.
    #[must_use]
    pub fn with_meta(mut self) -> Self {
        self.meta = true;
        self
    }

    /// Creates the chord of a pressed `key` with the held `modifiers`.
    ///
    /// The chord sets `control` and `meta` as pressed, never `primary`;
    /// [`matches`](Self::matches) resolves `primary` when comparing. Returns
    /// `None` when `key` is a modifier key on its own or is unidentified.
    #[must_use]
    pub fn from_event(key: &Key, modifiers: Modifiers) -> Option<Self> {
        if Self::is_modifier_key(key) || matches!(key, Key::Unidentified) {
            return None;
        }
        Some(Self {
            key: normalized(key.clone()),
            primary: false,
            control: modifiers.control(),
            shift: modifiers.shift(),
            alt: modifiers.alt(),
            meta: modifiers.logo(),
        })
    }

    /// Creates the chord of a key press event; `None` for any other event
    /// and for presses of modifier keys.
    #[must_use]
    pub fn from_keyboard_event(event: &iced::keyboard::Event) -> Option<Self> {
        match event {
            iced::keyboard::Event::KeyPressed { key, modifiers, .. } => {
                Self::from_event(key, *modifiers)
            }
            _ => None,
        }
    }

    /// Whether the press of `key` with `modifiers` is this chord on `os`.
    #[must_use]
    pub fn matches_event(&self, key: &Key, modifiers: Modifiers, os: OperatingSystem) -> bool {
        Self::from_event(key, modifiers).is_some_and(|pressed| self.matches(&pressed, os))
    }

    /// Whether `key` is a modifier key on its own: Control, Shift, Alt or
    /// Meta on either side, Caps Lock or Fn.
    #[must_use]
    pub fn is_modifier_key(key: &Key) -> bool {
        matches!(
            key,
            Key::Named(
                Named::Alt
                    | Named::AltGraph
                    | Named::CapsLock
                    | Named::Control
                    | Named::Fn
                    | Named::Hyper
                    | Named::Meta
                    | Named::Shift
                    | Named::Super
            )
        )
    }

    /// Whether the chord holds a modifier other than Shift, so it can't be
    /// typed as text.
    #[must_use]
    pub fn has_command_modifier(&self) -> bool {
        self.primary || self.control || self.alt || self.meta
    }

    /// Returns the chord with `primary` turned into Command (meta) or
    /// Control for `os`.
    #[must_use]
    pub fn resolve(&self, os: OperatingSystem) -> Self {
        if !self.primary {
            return self.clone();
        }
        let apple = uses_command_key(os);
        Self {
            key: self.key.clone(),
            primary: false,
            control: self.control || !apple,
            shift: self.shift,
            alt: self.alt,
            meta: self.meta || apple,
        }
    }

    /// Whether this chord, resolved for `os`, is `other` resolved for `os`.
    #[must_use]
    pub fn matches(&self, other: &Self, os: OperatingSystem) -> bool {
        self.resolve(os) == other.resolve(os)
    }

    /// Returns the display parts of the chord on `os`: the modifier names
    /// or symbols, in the platform order, then the key.
    #[must_use]
    pub fn parts(&self, os: OperatingSystem) -> Vec<String> {
        let chord = self.resolve(os);
        let apple = uses_command_key(os);
        let mut parts = Vec::new();
        let mut push = |held: bool, apple_label: &str, label: &str| {
            if held {
                parts.push(if apple { apple_label } else { label }.to_owned());
            }
        };
        push(chord.control, "⌃", "Ctrl");
        push(chord.alt, "⌥", "Alt");
        push(chord.shift, "⇧", "Shift");
        push(chord.meta, "⌘", "Super");
        parts.push(Self::key_label(&chord.key, os));
        parts
    }

    /// Returns the chord as text on `os`, such as `Ctrl+Shift+K` or `⇧⌘K`.
    #[must_use]
    pub fn label(&self, os: OperatingSystem) -> String {
        let separator = if uses_command_key(os) { "" } else { "+" };
        self.parts(os).join(separator)
    }

    /// Returns the display name of `key` on `os`: a text name or an Apple
    /// symbol for special keys, and the upper-cased key label for single
    /// characters.
    #[must_use]
    pub fn key_label(key: &Key, os: OperatingSystem) -> String {
        match key {
            Key::Named(named) => {
                match SPECIAL_KEYS
                    .iter()
                    .find(|(_, special, _, _)| special == named)
                {
                    Some((_, _, text, apple)) => {
                        (if uses_command_key(os) { apple } else { text }).to_string()
                    }
                    None => format!("{named:?}"),
                }
            }
            Key::Character(text) => {
                if text.chars().count() == 1 {
                    text.to_uppercase()
                } else {
                    text.to_string()
                }
            }
            Key::Unidentified => String::new(),
        }
    }
}

impl fmt::Display for KeyChord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label(OperatingSystem::Linux))
    }
}

/// Returns `key` as it is stored in a chord: characters lower-cased and a
/// space character read as the Space key.
fn normalized(key: Key) -> Key {
    match key {
        Key::Character(text) if text.as_str() == " " => Key::Named(Named::Space),
        Key::Character(text) => Key::Character(text.to_lowercase().into()),
        other => other,
    }
}

/// The error of parsing a [`Shortcut`] from text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseShortcutError {
    /// A key name is unknown; an empty text reports an empty name.
    UnknownKey(String),
    /// A modifier name is unknown.
    UnknownModifier(String),
}

impl fmt::Display for ParseShortcutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKey(name) => write!(f, "unknown key \"{name}\""),
            Self::UnknownModifier(name) => write!(f, "unknown modifier \"{name}\""),
        }
    }
}

impl std::error::Error for ParseShortcutError {}

/// A keyboard shortcut: one [`KeyChord`], or a sequence of chords pressed
/// one after the other, such as `G` then `H`. A shortcut always holds at
/// least one chord.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Shortcut {
    chords: Vec<KeyChord>,
}

impl Shortcut {
    /// Creates a single-chord shortcut.
    #[must_use]
    pub fn single(chord: KeyChord) -> Self {
        Self {
            chords: vec![chord],
        }
    }

    /// Creates a shortcut from `chords`; `None` when there are none.
    #[must_use]
    pub fn from_chords(chords: Vec<KeyChord>) -> Option<Self> {
        (!chords.is_empty()).then_some(Self { chords })
    }

    /// Creates a single-chord shortcut of `key` with the platform's primary
    /// modifier (Command on Apple platforms, Control elsewhere), plus Shift
    /// when `shift` and Alt when `alt`.
    #[must_use]
    pub fn primary(key: Key, shift: bool, alt: bool) -> Self {
        let mut chord = KeyChord::new(normalized(key)).with_primary();
        chord.shift = shift;
        chord.alt = alt;
        Self::single(chord)
    }

    /// Parses a shortcut such as `Mod+K`, `Ctrl+Shift+P`, `Alt+Enter` or the
    /// sequence `G H`: chords are separated by spaces and keys within a
    /// chord by `+`.
    ///
    /// Modifier names: `Mod`/`Primary` (Command on Apple platforms, Control
    /// elsewhere), `Ctrl`/`Control`, `Shift`, `Alt`/`Option`,
    /// `Meta`/`Cmd`/`Command`/`Super`. Keys: letters, digits, `F1` to `F12`,
    /// `Enter`/`Return`, `Esc`/`Escape`, `Space`, `Tab`, `Backspace`,
    /// `Delete`, `Up`, `Down`, `Left`, `Right`, `Home`, `End`, `PageUp`,
    /// `PageDown` and the punctuation `,` `.` `/` `;` `-` `=` `[` `]`.
    /// Names are case insensitive.
    ///
    /// # Errors
    ///
    /// Returns [`ParseShortcutError`] when a key or modifier name is
    /// unknown, including an empty `text`.
    pub fn parse(text: &str) -> Result<Self, ParseShortcutError> {
        let mut chords = Vec::new();
        for chord in text.split_whitespace() {
            let mut parts: Vec<&str> = chord.split('+').collect();
            let key_name = parts.pop().unwrap_or_default();
            let names: HashSet<String> = parts.iter().map(|part| part.to_lowercase()).collect();
            let key = key_named(key_name)
                .ok_or_else(|| ParseShortcutError::UnknownKey(key_name.to_owned()))?;
            if let Some(unknown) = parts
                .iter()
                .find(|part| !MODIFIER_NAMES.contains(&part.to_lowercase().as_str()))
            {
                return Err(ParseShortcutError::UnknownModifier(unknown.to_lowercase()));
            }
            let has = |candidates: &[&str]| candidates.iter().any(|name| names.contains(*name));
            chords.push(KeyChord {
                key,
                primary: has(&["mod", "primary"]),
                control: has(&["ctrl", "control"]),
                shift: has(&["shift"]),
                alt: has(&["alt", "option"]),
                meta: has(&["meta", "cmd", "command", "super"]),
            });
        }
        Self::from_chords(chords).ok_or_else(|| ParseShortcutError::UnknownKey(String::new()))
    }

    /// The chords, pressed in order.
    #[must_use]
    pub fn chords(&self) -> &[KeyChord] {
        &self.chords
    }

    /// Whether this shortcut has as many chords as `other` and each one,
    /// resolved for `os`, matches the chord of `other` at the same position.
    #[must_use]
    pub fn matches(&self, other: &Self, os: OperatingSystem) -> bool {
        self.chords.len() == other.chords.len() && self.starts_with(other, os)
    }

    /// Whether this shortcut begins with every chord of `prefix`, resolved
    /// for `os`.
    #[must_use]
    pub fn starts_with(&self, prefix: &Self, os: OperatingSystem) -> bool {
        prefix.chords.len() <= self.chords.len()
            && prefix
                .chords
                .iter()
                .zip(&self.chords)
                .all(|(wanted, own)| own.matches(wanted, os))
    }

    /// Returns the shortcut as text on `os`, such as `Ctrl+K` or `G H`.
    #[must_use]
    pub fn label(&self, os: OperatingSystem) -> String {
        self.chords
            .iter()
            .map(|chord| chord.label(os))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl FromStr for Shortcut {
    type Err = ParseShortcutError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label(OperatingSystem::Linux))
    }
}

/// Returns the key named `name` in the [`Shortcut::parse`] syntax, or
/// `None` when the name is unknown.
fn key_named(name: &str) -> Option<Key> {
    let lower = name.to_lowercase();
    let mut chars = lower.chars();
    if let (Some(only), None) = (chars.next(), chars.next())
        && (only.is_ascii_lowercase() || only.is_ascii_digit())
    {
        return Some(Key::Character(lower.as_str().into()));
    }
    if let Some(digits) = lower.strip_prefix('f')
        && (1..=2).contains(&digits.len())
        && digits.bytes().all(|b| b.is_ascii_digit())
        && let Ok(n) = digits.parse::<usize>()
        && (1..=12).contains(&n)
    {
        return Some(Key::Named(FUNCTION_KEYS[n - 1]));
    }
    let named = match lower.as_str() {
        "return" => Some(Named::Enter),
        "escape" => Some(Named::Escape),
        other => SPECIAL_KEYS
            .iter()
            .find(|(special, ..)| *special == other)
            .map(|(_, named, ..)| *named),
    };
    if let Some(named) = named {
        return Some(Key::Named(named));
    }
    matches!(
        lower.as_str(),
        "," | "." | "/" | ";" | "-" | "=" | "[" | "]"
    )
    .then(|| Key::Character(lower.as_str().into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use OperatingSystem::{Linux, Macos, Windows};

    fn shortcut(text: &str) -> Shortcut {
        Shortcut::parse(text).expect("valid shortcut")
    }

    fn key(text: &str) -> Key {
        Key::Character(text.into())
    }

    #[test]
    fn parses_chords_modifiers_and_sequences() {
        let palette = shortcut("Mod+Shift+P");
        assert_eq!(palette.chords().len(), 1);
        assert_eq!(palette.chords()[0].key, key("p"));
        assert!(palette.chords()[0].primary);
        assert!(palette.chords()[0].shift);
        let go_home = shortcut("G H");
        let keys: Vec<_> = go_home.chords().iter().map(|c| c.key.clone()).collect();
        assert_eq!(keys, [key("g"), key("h")]);
        assert_eq!(shortcut("F5").chords()[0].key, Key::Named(Named::F5));
        assert_eq!(shortcut("Ctrl+,").chords()[0].key, key(","));
        assert_eq!(
            Shortcut::parse("Hyper+K").unwrap_err(),
            ParseShortcutError::UnknownModifier("hyper".to_owned())
        );
        assert_eq!(
            Shortcut::parse("Ctrl+Pause").unwrap_err(),
            ParseShortcutError::UnknownKey("Pause".to_owned())
        );
    }

    #[test]
    fn parses_every_documented_key_name() {
        for name in [
            "A",
            "z",
            "0",
            "9",
            "F1",
            "f12",
            "Enter",
            "Return",
            "Esc",
            "Escape",
            "Space",
            "Tab",
            "Backspace",
            "Delete",
            "Up",
            "Down",
            "Left",
            "Right",
            "Home",
            "End",
            "PageUp",
            "PageDown",
            ",",
            ".",
            "/",
            ";",
            "-",
            "=",
            "[",
            "]",
        ] {
            assert!(Shortcut::parse(name).is_ok(), "{name}");
        }
        for name in ["F0", "F13", "ab", "", "+", "Ctrl++"] {
            assert!(Shortcut::parse(name).is_err(), "{name}");
        }
        assert_eq!(
            Shortcut::parse("   "),
            Err(ParseShortcutError::UnknownKey(String::new()))
        );
    }

    #[test]
    fn parses_modifier_synonyms_and_ignores_case() {
        let chord = &shortcut("CMD+option+Control+SHIFT+x").chords()[0].clone();
        assert!(chord.meta && chord.alt && chord.control && chord.shift && !chord.primary);
        assert!(shortcut("primary+k").chords()[0].primary);
        assert!(shortcut("Super+k").chords()[0].meta);
        assert_eq!("Mod+K".parse::<Shortcut>(), Ok(shortcut("Mod+K")));
    }

    #[test]
    fn resolves_the_primary_modifier_per_platform() {
        let wanted = shortcut("Mod+K");
        assert!(wanted.matches(&shortcut("Ctrl+K"), Linux));
        assert!(wanted.matches(&shortcut("Cmd+K"), Macos));
        assert!(!wanted.matches(&shortcut("Ctrl+K"), Macos));
        assert!(!wanted.matches(&shortcut("Cmd+K"), Linux));
    }

    #[test]
    fn sequences_match_by_length_and_prefix() {
        let go_home = shortcut("G H");
        assert!(go_home.matches(&shortcut("G H"), Linux));
        assert!(!go_home.matches(&shortcut("G"), Linux));
        assert!(go_home.starts_with(&shortcut("G"), Linux));
        assert!(!go_home.starts_with(&shortcut("H"), Linux));
        assert!(!shortcut("G").starts_with(&go_home, Linux));
    }

    #[test]
    fn labels_follow_the_platform_notation() {
        let wanted = shortcut("Mod+Shift+K");
        assert_eq!(wanted.label(Linux), "Ctrl+Shift+K");
        assert_eq!(wanted.label(Windows), "Ctrl+Shift+K");
        assert_eq!(wanted.label(Macos), "⇧⌘K");
        assert_eq!(shortcut("G H").label(Linux), "G H");
        assert_eq!(shortcut("Alt+Enter").label(Macos), "⌥↩");
        assert_eq!(
            shortcut("Ctrl+Alt+Meta+Delete").label(Linux),
            "Ctrl+Alt+Super+Delete"
        );
        assert_eq!(shortcut("Ctrl+Alt+Meta+Delete").label(Macos), "⌃⌥⌘⌦");
        assert_eq!(shortcut("PageUp").label(Linux), "PgUp");
        assert_eq!(shortcut("Up").label(Macos), "↑");
        assert_eq!(shortcut("F5").label(Linux), "F5");
        assert_eq!(shortcut("Mod+K").to_string(), "Ctrl+K");
        assert_eq!(shortcut("Space").label(Macos), "Space");
    }

    #[test]
    fn primary_constructor_builds_the_chord() {
        let built = Shortcut::primary(key("K"), true, false);
        assert_eq!(built, shortcut("Mod+Shift+K"));
        assert_eq!(
            Shortcut::primary(key("s"), false, true),
            shortcut("Mod+Alt+S")
        );
        assert!(Shortcut::from_chords(Vec::new()).is_none());
    }

    #[test]
    fn command_modifier_excludes_shift() {
        assert!(shortcut("Mod+K").chords()[0].has_command_modifier());
        assert!(shortcut("Alt+K").chords()[0].has_command_modifier());
        assert!(!shortcut("Shift+/").chords()[0].has_command_modifier());
        assert!(!shortcut("G").chords()[0].has_command_modifier());
    }

    #[test]
    fn builds_a_chord_from_an_event() {
        let chord = KeyChord::from_event(&key("K"), Modifiers::CTRL | Modifiers::SHIFT);
        let expected = KeyChord::character("k").with_control().with_shift();
        assert_eq!(chord, Some(expected));
        assert_eq!(
            KeyChord::from_event(&key("k"), Modifiers::LOGO).map(|c| c.meta),
            Some(true)
        );
        assert!(KeyChord::from_event(&Key::Named(Named::Control), Modifiers::CTRL).is_none());
        assert!(KeyChord::from_event(&Key::Named(Named::Shift), Modifiers::SHIFT).is_none());
        assert!(KeyChord::from_event(&Key::Unidentified, Modifiers::empty()).is_none());
        assert_eq!(
            KeyChord::from_event(&key(" "), Modifiers::empty()),
            Some(KeyChord::new(Key::Named(Named::Space)))
        );
    }

    #[test]
    fn matches_key_events_with_the_primary_modifier() {
        let wanted = &shortcut("Mod+K").chords()[0].clone();
        assert!(wanted.matches_event(&key("k"), Modifiers::CTRL, Linux));
        assert!(!wanted.matches_event(&key("k"), Modifiers::LOGO, Linux));
        assert!(wanted.matches_event(&key("k"), Modifiers::LOGO, Macos));
        assert!(!wanted.matches_event(&key("k"), Modifiers::CTRL, Macos));
        assert!(!wanted.matches_event(&key("k"), Modifiers::CTRL | Modifiers::SHIFT, Linux));
        assert!(!wanted.matches_event(&key("j"), Modifiers::CTRL, Linux));
        assert!(!wanted.matches_event(&Key::Named(Named::Control), Modifiers::CTRL, Linux));
    }

    #[test]
    fn matches_named_and_punctuation_events() {
        let enter = &shortcut("Alt+Enter").chords()[0].clone();
        assert!(enter.matches_event(&Key::Named(Named::Enter), Modifiers::ALT, Linux));
        let help = &shortcut("Shift+/").chords()[0].clone();
        assert!(help.matches_event(&key("/"), Modifiers::SHIFT, Linux));
    }

    #[test]
    fn builds_a_chord_from_a_keyboard_event() {
        use iced::keyboard::{Event, Location, key::Physical};
        let pressed = Event::KeyPressed {
            key: key("k"),
            modified_key: key("k"),
            physical_key: Physical::Unidentified(iced::keyboard::key::NativeCode::Unidentified),
            location: Location::Standard,
            modifiers: Modifiers::CTRL,
            text: None,
            repeat: false,
        };
        assert_eq!(
            KeyChord::from_keyboard_event(&pressed),
            Some(KeyChord::character("k").with_control())
        );
        assert!(KeyChord::from_keyboard_event(&Event::ModifiersChanged(Modifiers::CTRL)).is_none());
    }
}
