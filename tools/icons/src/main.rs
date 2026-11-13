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

//! Generates the Lucide icon module of `oito2_iced` from a Lucide checkout
//! at a release tag, or checks that the module is still up to date.
//!
//! ```sh
//! cargo run -p oito2_iced_icons -- generate <path to lucide>
//! cargo run -p oito2_iced_icons -- check <path to lucide>
//! ```
//!
//! `generate` writes one constant per icon of `<lucide>/icons` into
//! `crates/oito2_iced/src/icon/lucide.rs`, copies the Lucide license next to
//! it and records the tag in `tools/icons/source.json`. `check` exits with
//! status 1 when the module differs from what `generate` would write.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

/// The module written by `generate`, relative to the workspace root.
const MODULE: &str = "crates/oito2_iced/src/icon/lucide.rs";

/// The license copied next to the module, relative to the workspace root.
const LICENSE: &str = "crates/oito2_iced/src/icon/LICENSE-LUCIDE";

/// The record of the source tag, relative to the workspace root.
const SOURCE: &str = "tools/icons/source.json";

/// The root element every Lucide icon starts with, without white space.
const HEADER: &str = "<svgxmlns=\"http://www.w3.org/2000/svg\"width=\"24\"height=\"24\"\
    viewBox=\"002424\"fill=\"none\"stroke=\"currentColor\"stroke-width=\"2\"\
    stroke-linecap=\"round\"stroke-linejoin=\"round\">";

/// The GPL header of every source file of the workspace.
const GPL_HEADER: &str = "// Copyright (C) 2026  OITO2
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
";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    let result = match arguments.as_slice() {
        [command, source] if command == "generate" => generate(Path::new(source)),
        [command, source] if command == "check" => check(Path::new(source)),
        _ => Err("usage: oito2_iced_icons <generate|check> <path to lucide>".to_owned()),
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

/// The release tag checked out in `source`.
fn tag(source: &Path) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(source)
        .args(["describe", "--tags", "--exact-match"])
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "{} is not a git checkout of a Lucide release tag",
            source.display()
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

/// Reads a text file, naming it in the error.
fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("cannot read {}: {error}", path.display()))
}

/// Writes a file, naming it in the error.
fn write(path: &Path, contents: impl AsRef<[u8]>) -> Result<(), String> {
    fs::write(path, contents).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// One icon: its Lucide name and the elements inside its root element.
struct Icon {
    name: String,
    body: String,
}

/// Reads every icon of the checkout, sorted by name.
fn icons(source: &Path) -> Result<Vec<Icon>, String> {
    let directory = source.join("icons");
    let entries = fs::read_dir(&directory)
        .map_err(|error| format!("cannot list {}: {error}", directory.display()))?;

    let mut icons = Vec::new();

    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();

        if path.extension().and_then(|extension| extension.to_str()) != Some("svg") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("invalid file name {}", path.display()))?
            .to_owned();

        let body = body(&read(&path)?).ok_or_else(|| {
            format!(
                "{} does not start with the common Lucide root element",
                path.display()
            )
        })?;

        icons.push(Icon { name, body });
    }

    icons.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(icons)
}

/// The elements inside the root element of a Lucide icon, one after the
/// other without line breaks, or `None` when the root element differs from
/// the common one.
fn body(svg: &str) -> Option<String> {
    let end = svg.find('>')?;
    let root: String = svg[..=end].split_whitespace().collect();

    if root != HEADER {
        return None;
    }

    let inner = svg[end + 1..].trim().strip_suffix("</svg>")?;

    let elements: Vec<String> = inner
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.replace(" />", "/>"))
        .collect();

    Some(elements.join(""))
}

/// The Rust name of an icon constant: `folder-open` becomes `FOLDER_OPEN`.
fn constant(name: &str) -> String {
    name.to_ascii_uppercase().replace('-', "_")
}

/// The source of the generated module.
fn module(icons: &[Icon], tag: &str) -> String {
    let mut text = String::from(GPL_HEADER);

    let _ = write!(
        text,
        "//
// The icon artwork is Lucide {tag}: ISC License, Copyright (c) 2026 Lucide
// Icons and Contributors; the icons derived from Feather are also under the
// MIT License, Copyright (c) 2013-present Cole Bemis. The full notices are
// in LICENSE-LUCIDE.

//! Every Lucide {tag} icon, as an [`Icon`] constant named after the icon:
//! `folder-open` is [`FOLDER_OPEN`].
//!
//! This module is generated; regenerate it instead of editing it.

use super::Icon;

/// The Lucide release the icons come from.
pub const VERSION: &str = \"{tag}\";

/// Builds the icon `$name` from the elements of its root element.
macro_rules! lucide {{
    ($name:literal, $body:literal) => {{
        Icon::new(
            $name,
            concat!(
                r#\"<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\">\"#,
                $body,
                \"</svg>\"
            ),
        )
    }};
}}
"
    );

    for icon in icons {
        let _ = write!(
            text,
            "\n/// `{name}`.\npub const {constant}: Icon = lucide!(\"{name}\", r#\"{body}\"#);\n",
            name = icon.name,
            constant = constant(&icon.name),
            body = icon.body,
        );
    }

    text
}

/// Writes the module, the license and the source record.
fn generate(source: &Path) -> Result<(), String> {
    let tag = tag(source)?;
    let icons = icons(source)?;
    let root = workspace();

    write(&root.join(MODULE), module(&icons, &tag))?;
    write(&root.join(LICENSE), read(&source.join("LICENSE"))?)?;

    let record = serde_json::json!({
        "repository": "lucide-icons/lucide",
        "tag": tag,
        "icons": icons.len(),
    });
    let text = serde_json::to_string_pretty(&record).map_err(|error| error.to_string())?;
    write(&root.join(SOURCE), text + "\n")?;

    println!("generated {} icons from Lucide {tag}", icons.len());
    Ok(())
}

/// Compares the module and the license with the checkout.
fn check(source: &Path) -> Result<(), String> {
    let tag = tag(source)?;
    let root = workspace();

    let module_matches = read(&root.join(MODULE))? == module(&icons(source)?, &tag);
    let license_matches = read(&root.join(LICENSE))? == read(&source.join("LICENSE"))?;

    if module_matches && license_matches {
        println!("the icon module equals Lucide {tag}");
        Ok(())
    } else {
        Err(format!(
            "the icon module differs from Lucide {tag} at {}; run generate",
            source.display()
        ))
    }
}
