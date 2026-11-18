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

//! Reading of attributes: `cfg` evaluation, `doc(hidden)`, `#[path]`
//! and doc comments.

use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, ExprLit, Lit, Meta, Token};

/// Whether every `#[cfg]` of `attrs` holds for a Linux build with every
/// feature enabled and without `test`. A `cfg` that does not parse
/// counts as enabled.
pub fn enabled(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("cfg"))
        .all(|attr| attr.parse_args::<Meta>().map_or(true, |meta| holds(&meta)))
}

/// Evaluates one `cfg` predicate.
fn holds(meta: &Meta) -> bool {
    match meta {
        Meta::Path(path) => ["doc", "debug_assertions", "unix"]
            .iter()
            .any(|name| path.is_ident(name)),
        Meta::NameValue(pair) => {
            let value = string(&pair.value).unwrap_or_default();
            if pair.path.is_ident("feature") {
                true
            } else if pair.path.is_ident("target_os") {
                value == "linux"
            } else if pair.path.is_ident("target_family") {
                value == "unix"
            } else {
                false
            }
        }
        Meta::List(list) => {
            let Ok(args) = list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
            else {
                return true;
            };
            if list.path.is_ident("not") {
                !args.iter().all(holds)
            } else if list.path.is_ident("any") {
                args.iter().any(holds)
            } else {
                args.iter().all(holds)
            }
        }
    }
}

/// Whether `attrs` hold `#[doc(hidden)]`.
pub fn hidden(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("doc")
            && matches!(attr.meta, Meta::List(_))
            && attr
                .parse_args::<syn::Ident>()
                .is_ok_and(|ident| ident == "hidden")
    })
}

/// Whether the item with `attrs` is compiled and shown in the docs.
pub fn visible(attrs: &[Attribute]) -> bool {
    enabled(attrs) && !hidden(attrs)
}

/// The value of `#[path = "..."]`, if any.
pub fn path(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| match &attr.meta {
        Meta::NameValue(pair) if pair.path.is_ident("path") => string(&pair.value),
        _ => None,
    })
}

/// Whether `attrs` hold `#[macro_export]`.
pub fn macro_export(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("macro_export"))
}

/// The string of a string literal expression.
fn string(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit), ..
        }) => Some(lit.value()),
        _ => None,
    }
}

/// The doc comment of `attrs`: every `///`, `//!` and `#[doc = "..."]`
/// line in order, without the indentation all of them share, without the
/// blank lines around it and without the hidden `# ` lines of Rust code
/// blocks.
pub fn docs(attrs: &[Attribute]) -> String {
    let mut lines = Vec::new();
    for attr in attrs {
        if let Meta::NameValue(pair) = &attr.meta
            && pair.path.is_ident("doc")
            && let Some(value) = string(&pair.value)
        {
            lines.extend(value.split('\n').map(str::to_owned));
        }
    }

    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    let lines: Vec<&str> = lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or("").trim_end())
        .collect();

    hide_doctest_lines(&lines).trim_matches('\n').to_owned()
}

/// Joins `lines`, leaving out the lines that rustdoc hides in Rust code
/// blocks (`#` alone or followed by a space) and turning `##` into `#`.
fn hide_doctest_lines(lines: &[&str]) -> String {
    let mut out = Vec::new();
    let mut fence: Option<(char, bool)> = None;
    for line in lines {
        let trimmed = line.trim_start();
        let marker = ['`', '~']
            .into_iter()
            .find(|mark| trimmed.starts_with(&mark.to_string().repeat(3)));
        match (fence, marker) {
            (Some((open, _)), Some(mark)) if open == mark => {
                fence = None;
            }
            (None, Some(mark)) => {
                let info = trimmed.trim_start_matches(mark).trim();
                fence = Some((mark, is_rust(info)));
            }
            (Some((_, true)), _) => {
                if trimmed == "#" || trimmed.starts_with("# ") {
                    continue;
                }
                if let Some(rest) = trimmed.strip_prefix("##") {
                    let indent = &line[..line.len() - trimmed.len()];
                    out.push(format!("{indent}#{rest}"));
                    continue;
                }
            }
            _ => {}
        }
        out.push((*line).to_owned());
    }
    out.join("\n")
}

/// Whether a code block with the info string `info` holds Rust code.
fn is_rust(info: &str) -> bool {
    info.split([',', ' ']).all(|token| {
        matches!(
            token.trim(),
            "" | "rust" | "no_run" | "ignore" | "should_panic" | "compile_fail" | "test_harness"
        ) || token.trim().starts_with("edition")
    })
}

/// The first paragraph of `docs`, on one line.
pub fn summary(docs: &str) -> String {
    docs.lines()
        .map(str::trim)
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attrs(source: &str) -> Vec<Attribute> {
        syn::parse_str::<syn::ItemFn>(&format!("{source}\nfn f() {{}}"))
            .expect("the attributes parse")
            .attrs
    }

    #[test]
    fn joins_doc_lines_and_hides_doctest_lines() {
        let attrs = attrs(
            "/// First line\n/// continues.\n///\n/// ```\n/// # use foo;\n/// let x = 1;\n/// ## not hidden\n/// ```\n///\n/// ```text\n/// # kept\n/// ```\n#[doc = \" Last.\"]",
        );
        assert_eq!(
            docs(&attrs),
            "First line\ncontinues.\n\n```\nlet x = 1;\n# not hidden\n```\n\n```text\n# kept\n```\nLast."
        );
        assert_eq!(summary(&docs(&attrs)), "First line continues.");
    }

    #[test]
    fn evaluates_cfg() {
        assert!(enabled(&attrs("#[cfg(feature = \"svg\")]")));
        assert!(!enabled(&attrs("#[cfg(not(feature = \"svg\"))]")));
        assert!(!enabled(&attrs("#[cfg(test)]")));
        assert!(!enabled(&attrs("#[cfg(all(test, feature = \"svg\"))]")));
        assert!(enabled(&attrs("#[cfg(target_os = \"linux\")]")));
        assert!(!enabled(&attrs("#[cfg(windows)]")));
        assert!(hidden(&attrs("#[doc(hidden)]")));
        assert!(!hidden(&attrs("/// Shown.")));
    }
}
