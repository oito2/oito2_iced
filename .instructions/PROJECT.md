# Project Rules

Rules defined for this project. They take precedence over `AGENTS.md` and over the standards in `.instructions/` — use them to add a rule or to change a default. Perci creates this file once and never rewrites it.

<!-- One concise bullet per rule, under a heading for its topic. When a rule replaces a default, say which. -->

## Session Start

- Load the project memory with Sync82 (`load_project_context`, marker `.sync82.json`) before any task; update it at the end of the session.
- The development plan lives in `/srv/workspace/MyProjects/DevNotes/oito2_iced/oito2-iced.md` (decisions D1–D24, phases F0–F12). Read it before starting a phase and keep its status notes current; phase results go to the same folder (e.g. `f0-results.md`).

## Developer Notes

- Personal notes, plans and test procedures live in `/srv/workspace/MyProjects/DevNotes/oito2_iced/` (outside the repository) and may be written in pt-BR (exception to the English-only documentation rule of `AGENTS.md`, which applies to files inside the repository).
- `.references/` (Iced 0.14.0, iced_aw v0.14.1, libcosmic, mundy, material-colors clones and links to the sibling repositories), `.sync82.json` and the whole `.claude/` folder stay out of git.
- `.references/iced` is the primary source for the Iced API: the API changed between 0.12, 0.13 and 0.14, so verify every Iced call against it. Code from the references (MIT, MPL, Apache) is never copied into this GPL project.

## Design Source of Truth

- `oito2_ui` (Dart) defines tokens, colors, strings and behaviour; this project never invents a color, radius or height. A design change lands there first and reaches `tools/parity/fixtures`.
- The parity fixtures reach `crates/*/tests/fixtures/parity/` only through `cargo run -p oito2_iced_parity -- sync <path to oito2_ui>`, which records the source commit in `tools/parity/source.json` and in each fixture README; `parity.yml` runs `check` against `oito2_ui@main` every week.

## Source Files

- Every hand-written `.rs` file starts with the GPL-3.0 header "Copyright (C) 2026  OITO2" (two spaces, uppercase), as in `crates/oito2_iced/src/lib.rs`.
- Toolchain 1.99.0 (`rust-toolchain.toml`), edition 2024, MSRV 1.97 (set by `material-colors` 0.5). Lints come from the workspace: `unsafe_code = "forbid"`, clippy `unwrap_used`, `panic` and `todo` denied; `clippy.toml` relaxes them inside `#[test]` functions only, so test helpers use `assert!` instead of `panic!`. Every public item has rustdoc (`#![deny(missing_docs)]`).
- Public type names carry no `Oito2` prefix. Style classes are enums with the OITO2 variants plus `Custom(StyleFn)`, and implement `From<StyleFn>` so the `style` builder of each Iced widget keeps working.
- The `oito2_iced` crate depends on `iced` with `default-features = false` and only `advanced`, `x11`, `wayland` and `thread-pool`; applications choose the renderers. Catalogs of feature-gated Iced widgets sit behind crate features that forward to Iced (`svg`, on by default; `qr_code`; `markdown`).

## Testing & Verification

- Run before reporting: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked`, `cargo deny check`, `ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked`, `cargo +1.97 check --workspace --all-features --locked`. Code that differs per platform is also checked with `cargo clippy --target x86_64-pc-windows-msvc` and `--target aarch64-apple-darwin`.
- Behaviour tests use the `iced_test` simulator; the focus traversal lives in `focus::navigation` (a widget, not a subscription) so the simulator exercises it.
- Fonts are static instances made by `tools/fonts/make_instances.py` from the `oito2_ui` variable fonts; a variable font is never embedded, because the text stack only matches faces by exact weight.
- Test helpers live in the `oito2_iced_test_utils` crate (`publish = false`): simulator settings with the embedded fonts, snapshot pixels and the `golden!` macro. Goldens live in `crates/*/tests/goldens/<name>-<renderer>.png`, compared with 0.5 % tolerance; `OITO2_UPDATE_GOLDENS=1` rewrites them, and a missing golden fails. Look at every new or changed golden before accepting it.
- Never automate clicks or input on the user's real desktop; manual desktop checks (LiveCDs, window managers) are the user's.

## Git

- Commit flow: write the commit message to `/srv/workspace/MyProjects/DevNotes/oito2_iced/commit-msg.txt` and hand the user `git add -A; git commit -F <that file>; git push`. Tags and publishing are the user's too.

## Agent Conduct

- Genuine choices go through the selectable-question tool with a **[RECOMENDADO]** option.
- When a session permission rule denies a command, hand the user the command instead of working around it.
