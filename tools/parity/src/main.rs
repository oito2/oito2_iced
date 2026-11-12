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

//! Copies the parity fixtures from a checkout of `oito2_ui` into the test
//! fixtures of the crates, or checks that they are still equal.
//!
//! ```sh
//! cargo run -p oito2_iced_parity -- sync <path to oito2_ui>
//! cargo run -p oito2_iced_parity -- check <path to oito2_ui>
//! ```
//!
//! `sync` copies every fixture, then records the source commit in
//! `tools/parity/source.json` and in the README of each fixture directory.
//! `check` exits with status 1 when a fixture differs from the checkout.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// Directory of the fixtures inside the `oito2_ui` checkout.
const SOURCE_DIRECTORY: &str = "tools/parity/fixtures";

/// Each crate fixture directory, relative to the workspace root, with the
/// fixtures it holds.
const TARGETS: &[(&str, &[&str])] = &[
    (
        "crates/oito2_iced/tests/fixtures/parity",
        &[
            "tokens.json",
            "schemes.json",
            "foundation.json",
            "l10n.json",
            "format.json",
            "manifest.json",
        ],
    ),
    (
        "crates/oito2_iced_themes/tests/fixtures/parity",
        &["schemes.json"],
    ),
];

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let result = match arguments.as_slice() {
        [command, source] if command == "sync" => sync(Path::new(source)),
        [command, source] if command == "check" => check(Path::new(source)),
        _ => Err("usage: oito2_iced_parity <sync|check> <path to oito2_ui>".to_owned()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// The workspace root: two levels above this tool.
fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .to_path_buf()
}

/// The commit checked out in `source`.
fn commit(source: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(source)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;

    if !output.status.success() {
        return Err(format!("{} is not a git checkout", source.display()));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Reads a file, naming it in the error.
fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))
}

/// Writes a file, naming it in the error.
fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<(), String> {
    fs::write(path, contents).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// Copies every fixture and records the source commit.
fn sync(source: &Path) -> Result<(), String> {
    let commit = commit(source)?;
    let root = workspace();
    let mut copied = Vec::new();

    for (directory, files) in TARGETS {
        let target = root.join(directory);

        fs::create_dir_all(&target)
            .map_err(|error| format!("cannot create {}: {error}", target.display()))?;

        for file in *files {
            let bytes = read(&source.join(SOURCE_DIRECTORY).join(file))?;
            write(&target.join(file), bytes)?;
            copied.push(serde_json::json!({ "file": file, "target": directory }));
        }

        write(&target.join("README.md"), readme(files, &commit))?;
    }

    let record = serde_json::json!({
        "repository": "oito2/oito2_ui",
        "commit": commit,
        "fixtures": copied,
    });
    let text = serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?;

    write(&root.join("tools/parity/source.json"), text + "\n")?;
    println!("copied {} fixtures from {commit}", copied.len());

    Ok(())
}

/// The README of a fixture directory.
fn readme(files: &[&str], commit: &str) -> String {
    let mut text = String::from(
        "# Parity fixtures\n\n         Copied from `oito2_ui/tools/parity/fixtures` (repository `oito2/oito2_ui`)\n         by `cargo run -p oito2_iced_parity -- sync <path to oito2_ui>`.\n\n         | File | Source commit |\n|---|---|\n",
    );

    for file in files {
        let _ = writeln!(text, "| `{file}` | `{commit}` |");
    }

    text.push_str("\nUpdate only by running the sync again, which records the new commit here.\n");
    text
}

/// Compares every fixture with the checkout.
fn check(source: &Path) -> Result<(), String> {
    let root = workspace();
    let mut differing = Vec::new();

    for (directory, files) in TARGETS {
        for file in *files {
            let expected = read(&source.join(SOURCE_DIRECTORY).join(file))?;
            let actual = read(&root.join(directory).join(file))?;

            if expected != actual {
                differing.push(format!("{directory}/{file}"));
            }
        }
    }

    if differing.is_empty() {
        println!("every fixture equals {}", source.display());
        Ok(())
    } else {
        Err(format!(
            "fixtures that differ from {}:\n  {}\nrun the sync and the tests",
            source.display(),
            differing.join("\n  ")
        ))
    }
}
