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

//! XDG autostart entries: a desktop entry file in the autostart directory
//! makes a freedesktop.org session (GNOME, KDE Plasma, Xfce, Cinnamon, MATE,
//! LXQt) start the application at login. Removing the file turns it off.
//!
//! Only the standard library is used; every function takes the directory,
//! so the tests run on a temporary directory.

use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The autostart directory: `$XDG_CONFIG_HOME/autostart` when
/// `xdg_config_home` is an absolute path, `$HOME/.config/autostart` when it
/// is unset, empty or relative, and `None` when neither gives an absolute
/// path.
pub fn directory(xdg_config_home: Option<OsString>, home: Option<OsString>) -> Option<PathBuf> {
    let config = xdg_config_home
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| {
            home.map(PathBuf::from)
                .filter(|path| path.is_absolute())
                .map(|home| home.join(".config"))
        })?;

    Some(config.join("autostart"))
}

/// The autostart [`directory`] read from the `XDG_CONFIG_HOME` and `HOME`
/// variables of the process environment.
pub fn directory_from_env() -> Option<PathBuf> {
    directory(
        std::env::var_os("XDG_CONFIG_HOME"),
        std::env::var_os("HOME"),
    )
}

/// An application to start at login.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The file name without the `.desktop` extension, usually the
    /// application id in reverse DNS notation.
    pub id: String,
    /// The name the session shows for the entry.
    pub name: String,
    /// The program to run.
    pub program: PathBuf,
    /// The arguments passed to the program.
    pub args: Vec<String>,
}

impl Entry {
    /// The file name of the entry: its id with the `.desktop` extension.
    pub fn file_name(&self) -> String {
        format!("{}.desktop", self.id)
    }

    /// The path of the entry inside the autostart directory `directory`.
    pub fn path(&self, directory: &Path) -> PathBuf {
        directory.join(self.file_name())
    }

    /// The contents of the desktop entry file.
    pub fn contents(&self) -> String {
        let exec = std::iter::once(self.program.to_string_lossy().into_owned())
            .chain(self.args.iter().cloned())
            .map(|argument| exec_argument(&argument))
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "[Desktop Entry]\nType=Application\nName={}\nExec={}\nTerminal=false\n",
            escape_string(&self.name),
            escape_string(&exec),
        )
    }

    /// Whether the entry file exists in `directory`.
    pub fn is_enabled(&self, directory: &Path) -> bool {
        self.path(directory).is_file()
    }

    /// Writes the entry file to `directory`, creating the directory when it
    /// is missing, and returns the path of the file.
    pub fn enable(&self, directory: &Path) -> io::Result<PathBuf> {
        fs::create_dir_all(directory)?;
        let path = self.path(directory);
        fs::write(&path, self.contents())?;
        Ok(path)
    }

    /// Removes the entry file from `directory`; a missing file is not an
    /// error.
    pub fn disable(&self, directory: &Path) -> io::Result<()> {
        match fs::remove_file(self.path(directory)) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error),
            _ => Ok(()),
        }
    }
}

/// One argument of the `Exec` key: `%` doubled, and the argument wrapped in
/// double quotes, with `"`, `` ` ``, `$` and `\` escaped by a backslash,
/// when it holds a reserved character.
fn exec_argument(argument: &str) -> String {
    const RESERVED: &[char] = &[
        ' ', '\t', '\n', '"', '\'', '\\', '>', '<', '~', '|', '&', ';', '$', '*', '?', '#', '(',
        ')', '`',
    ];

    let argument = argument.replace('%', "%%");
    if !argument.contains(RESERVED) {
        return argument;
    }

    let mut quoted = String::with_capacity(argument.len() + 2);
    quoted.push('"');
    for character in argument.chars() {
        if matches!(character, '"' | '`' | '$' | '\\') {
            quoted.push('\\');
        }
        quoted.push(character);
    }
    quoted.push('"');
    quoted
}

/// A value of the string type: backslash, newline, tab and carriage return
/// written as their escape sequences.
fn escape_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            other => escaped.push(other),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A new empty directory under the system temporary directory, unique to
    /// the test `name` and to this process.
    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "oito2_iced_autostart-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create the temporary directory");
        dir
    }

    fn entry(program: &str) -> Entry {
        Entry {
            id: "io.github.oito2.Example".into(),
            name: "OITO2 Example".into(),
            program: PathBuf::from(program),
            args: vec!["--minimized".into()],
        }
    }

    /// An absolute path on every platform: `name` in the temporary
    /// directory.
    fn absolute(name: &str) -> PathBuf {
        std::env::temp_dir().join(name)
    }

    #[test]
    fn directory_prefers_an_absolute_xdg_config_home() {
        assert_eq!(
            directory(Some(absolute("cfg").into()), Some(absolute("home").into())),
            Some(absolute("cfg").join("autostart"))
        );
    }

    #[test]
    fn directory_falls_back_to_home() {
        let home = absolute("home");
        let expected = Some(home.join(".config").join("autostart"));
        assert_eq!(directory(None, Some(home.clone().into())), expected);
        assert_eq!(
            directory(Some("".into()), Some(home.clone().into())),
            expected
        );
        assert_eq!(
            directory(Some("relative".into()), Some(home.into())),
            expected
        );
    }

    #[test]
    fn directory_is_none_without_an_absolute_path() {
        assert_eq!(directory(None, None), None);
        assert_eq!(directory(None, Some("home".into())), None);
    }

    #[test]
    fn contents_quote_the_exec_arguments() {
        let contents = entry("/opt/My App/bin/app").contents();
        assert_eq!(
            contents,
            "[Desktop Entry]\nType=Application\nName=OITO2 Example\n\
             Exec=\"/opt/My App/bin/app\" --minimized\nTerminal=false\n"
        );
    }

    #[test]
    fn exec_arguments_escape_reserved_characters() {
        assert_eq!(exec_argument("/usr/bin/app"), "/usr/bin/app");
        assert_eq!(exec_argument("100%"), "100%%");
        assert_eq!(exec_argument("a$b"), "\"a\\$b\"");
        assert_eq!(exec_argument("say \"hi\""), "\"say \\\"hi\\\"\"");
        assert_eq!(escape_string(&exec_argument("a\\b")), "\"a\\\\\\\\b\"");
    }

    #[test]
    fn enable_and_disable_write_and_remove_the_file() {
        let dir = temp_dir("roundtrip");
        let autostart = dir.join("autostart");
        let entry = entry("/usr/bin/app");

        assert!(!entry.is_enabled(&autostart));

        let path = entry.enable(&autostart).expect("enable");
        assert_eq!(path, autostart.join("io.github.oito2.Example.desktop"));
        assert!(entry.is_enabled(&autostart));
        assert_eq!(
            fs::read_to_string(&path).expect("read the entry"),
            entry.contents()
        );

        entry.disable(&autostart).expect("disable");
        assert!(!entry.is_enabled(&autostart));
        entry.disable(&autostart).expect("disable a missing entry");

        fs::remove_dir_all(&dir).expect("remove the temporary directory");
    }
}
