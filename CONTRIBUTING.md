# Contributing to oito2_iced

🌐 **Language:** English · [Português](docs/pt-br/contribuindo.md)

Thanks for your interest in oito2_iced. This guide covers the development environment, the checks every change must pass, the rules for tests and golden images, the generators, and how changes reach the repository.

## Table of Contents

- [Before you start](#before-you-start)
- [Development environment](#development-environment)
- [Repository layout](#repository-layout)
- [Checks](#checks)
- [Tests and golden images](#tests-and-golden-images)
- [Parity fixtures](#parity-fixtures)
- [Lucide icons](#lucide-icons)
- [Fonts](#fonts)
- [Code conventions](#code-conventions)
- [Documentation and changelogs](#documentation-and-changelogs)
- [Pull requests](#pull-requests)
- [Release process](#release-process)

## Before you start

- **Design changes start in [oito2_ui](https://github.com/oito2/oito2_ui).** Colors, radii, heights, strings and behavior are defined there and reach this repository through the parity fixtures; oito2_iced never invents a color, a radius or a height.
- Open an issue before a large change, so the approach can be agreed first.
- By contributing you agree that your work is licensed under the GPL-3.0-or-later, like the rest of the project, and you follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Development environment

1. Install [rustup](https://rustup.rs). The repository pins Rust **1.99.0** with `rustfmt` and `clippy` in `rust-toolchain.toml`; rustup installs it on the first `cargo` call.
2. On Linux, install the development files the build needs (the CI installs the same ones):

   ```bash
   sudo apt install libxkbcommon-dev libglib2.0-dev
   ```

3. Install [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) for the dependency check, and the minimum supported Rust version for the MSRV check:

   ```bash
   cargo install --locked cargo-deny
   rustup toolchain install 1.97 --profile minimal
   ```

4. Clone and run the showcase:

   ```bash
   git clone https://github.com/oito2/oito2_iced.git
   cd oito2_iced
   cargo run -p oito2_iced_showcase
   ```

The [prerequisites page](docs/en/getting-started/prerequisites.md) has the details per system.

## Repository layout

| Path | Contents |
|---|---|
| `crates/oito2_iced` | Core crate: theme, tokens, fonts, icons, focus, overlay, widgets, window chrome |
| `crates/oito2_iced_settings`, `_themes`, `_syntax`, `_extras`, `_files` | Satellite crates |
| `crates/oito2_iced_test_utils` | Test helpers (not published): simulator settings, snapshot pixels, the `golden!` macro |
| `apps/showcase` | The showcase app (`oito2_iced_showcase`) |
| `examples/desktop` | `oito2_iced_desktop_example`: the code of the desktop [recipes](docs/en/guides/recipes/index.md) (tray icon, notifications, autostart) |
| `tools/parity` | `oito2_iced_parity`: copies and checks the parity fixtures |
| `tools/icons` | `oito2_iced_icons`: generates the Lucide icon module |
| `tools/fonts` | `make_instances.py`: the static font instances |
| `docs/en`, `docs/pt-br` | This documentation, in English and Brazilian Portuguese |

## Checks

Run every check before opening a pull request; the CI runs the same ones on Linux, and the tests also on Windows and macOS.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo deny check
cargo run -p oito2_iced_gen_docs --locked -- --check
ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked
cargo +1.97 check --workspace --all-features --locked
```

Code that differs per platform is also linted for Windows and macOS (add the targets once with `rustup target add x86_64-pc-windows-msvc aarch64-apple-darwin`):

```bash
cargo clippy --workspace --all-features --locked --target x86_64-pc-windows-msvc -- -D warnings
cargo clippy --workspace --all-features --locked --target aarch64-apple-darwin -- -D warnings
```

Linking the test binaries of a crate that depends on all of Iced takes a lot of memory. On a machine with 16 GB of RAM or less, limit the parallel jobs, for example with `-j 6`.

## Tests and golden images

- **One test binary per crate.** Integration tests live in `crates/<crate>/tests/all/main.rs`, which declares one module per area (`tests/all/<area>.rs`). Never add a top-level `tests/<name>.rs`: each one is a separate binary linked with all of Iced.
- **Behavior tests** use the `iced_test` simulator: clicks, typed text, Tab and Enter. Keyboard traversal runs through the `focus::navigation` widget, so the simulator exercises it.
- **Golden images** live in `crates/*/tests/goldens/<name>-<renderer>.png` and are compared with a 0.5 % tolerance by the `golden!` macro of `oito2_iced_test_utils`. Always run the tests with `ICED_TEST_BACKEND=tiny-skia`, the renderer the goldens are made with.
- A **missing golden fails** the test. Create or refresh goldens with `OITO2_UPDATE_GOLDENS=1`, then **look at every new or changed image** before committing it.
- A failing golden writes `<name>.actual.png` next to it (ignored by Git); the CI uploads these files as an artifact.
- **Fonts in tests** are the embedded static instances (family `OITO2 Inter`), so a copy of Inter installed on the system never changes the result.
- Every fix of observable behavior gets a regression test that fails without the fix.

## Parity fixtures

The fixtures exported by oito2_ui reach `crates/*/tests/fixtures/parity/` only through the parity tool, which also records the source commit in `tools/parity/source.json` and in the README of each fixture directory:

```bash
cargo run -p oito2_iced_parity -- sync <path to oito2_ui>
cargo run -p oito2_iced_parity -- check <path to oito2_ui>
```

`check` exits with status 1 when a fixture differs from the checkout. The `Parity` workflow (`.github/workflows/parity.yml`) runs it every Monday against `oito2_ui` `main`.

## Lucide icons

`crates/oito2_iced/src/icon/lucide.rs` is generated and never edited by hand. Generate it from a Lucide checkout at the release tag oito2_ui uses (recorded in `tools/icons/source.json`):

```bash
cargo run -p oito2_iced_icons -- generate <path to lucide>
cargo run -p oito2_iced_icons -- check <path to lucide>
```

## Fonts

The embedded fonts are static instances of the variable fonts, made by `tools/fonts/make_instances.py` (requires `fontTools`):

```bash
python3 tools/fonts/make_instances.py <Variable.ttf> <Variable-Italic.ttf> <output dir>
python3 tools/fonts/make_instances.py --jetbrains-mono <Variable.ttf> <Variable-Italic.ttf> <output dir>
```

Never embed a variable font: the Iced text stack matches faces by exact weight, so the other weights would fall back to a system font.

## Code conventions

- Every hand-written `.rs` file starts with the GPL-3.0 header `Copyright (C) 2026  OITO2`, as in `crates/oito2_iced/src/lib.rs`.
- Comments are in English and describe what the code does.
- Every public item has rustdoc (`#![deny(missing_docs)]`).
- The workspace lints forbid `unsafe` code and deny `unwrap`, `panic!` and `todo!`; `clippy.toml` relaxes them only inside `#[test]` functions, so test helpers use `assert!`.
- Public type names carry no `Oito2` prefix. Style classes are enums with the OITO2 variants plus `Custom(StyleFn)`, and implement `From<StyleFn>`.
- Verify every Iced call against Iced **0.14**: the API changed a lot between 0.12, 0.13 and 0.14. Code from other projects is never copied into this GPL project.

## Documentation and changelogs

- A change to a public API or to user-visible behavior updates the documentation in the same pull request: rustdoc, the showcase page, and `docs/en` and `docs/pt-br` (both trees keep the same files).
- Each published crate has a `CHANGELOG.md` in the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format; add user-facing changes under `[Unreleased]` of the crate they touch.

## Pull requests

1. Fork the repository and create a branch from `main`.
2. Make the change with its tests, documentation and changelog entry.
3. Run every check of the [Checks](#checks) section.
4. Open a pull request and fill in the template.

## Release process

Publishing is planned for a later release. Today every crate is at version `0.0.0` with `publish = false`, there is no release workflow, and nothing is published to crates.io; apps use the crates as Git dependencies. When releases start, this section will describe the tags, the crates.io publication and the release notes taken from the changelogs.
