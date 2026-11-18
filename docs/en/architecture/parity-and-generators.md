🌐 [Português](../../pt-br/architecture/parity-and-generators.md) | **English** | 🏠 [Index](../index.md)

---

# Parity and generators

How the design stays equal to oito2_ui, which files are generated, and what the CI runs. Code: `tools/parity`, `tools/icons`, `tools/fonts`, `.github/workflows/`.

## Parity fixtures

oito2_ui exports its design as JSON fixtures in `tools/parity/fixtures`. The parity tool copies them into the test fixtures of the crates:

```bash
cargo run -p oito2_iced_parity -- sync <path to oito2_ui>
cargo run -p oito2_iced_parity -- check <path to oito2_ui>
```

`sync` copies every fixture, then records the source commit in `tools/parity/source.json` and in the README of each fixture directory; `check` exits with status 1 when a fixture differs from the checkout. Fixtures are never edited by hand.

| Fixture | Copied to | Checked by the tests |
|---|---|---|
| `tokens.json` | `crates/oito2_iced/tests/fixtures/parity` | Radii, spacing, sizes, accents, neutrals, status colors, type scale |
| `schemes.json` | `crates/oito2_iced/…`, `crates/oito2_iced_themes/…` | Every color role of the base themes and of the 13 presets, standard and high contrast; the preset palettes |
| `foundation.json` | `crates/oito2_iced/…` | Contrast ratios, foreground choice and `ensure` |
| `l10n.json` | `crates/oito2_iced/…` | Every string in English and Portuguese, and the date and time formats |
| `format.json` | `crates/oito2_iced/…` | Byte sizes, compact numbers and relative times |
| `manifest.json` | `crates/oito2_iced/…` | Not compared: records the Flutter version and the package versions the fixtures were exported with |
| `syntax.json` | `crates/oito2_iced_syntax/…` | The color of each syntax role in every base theme |
| `qr.json` | `crates/oito2_iced_extras/…` | QR code matrices, module by module |

A design change therefore starts in oito2_ui, is exported there, and reaches this repository with `sync`.

### Known divergence: syntax tokens

oito2_ui highlights code with highlight.js; oito2_iced uses TextMate grammars (`syntect` with the `two-face` grammar set). The colors of each role are compared exactly, but token boundaries differ, so the tokenization is measured as character agreement with highlight.js (at least 60 % per language and 80 % overall) instead of compared exactly.

## Lucide icons

`crates/oito2_iced/src/icon/lucide.rs` holds every Lucide icon as an `Icon` constant (`folder-open` is `FOLDER_OPEN`). It is generated from a Lucide checkout at a release tag — the tag oito2_ui uses — and never edited by hand:

```bash
cargo run -p oito2_iced_icons -- generate <path to lucide>
cargo run -p oito2_iced_icons -- check <path to lucide>
```

`generate` writes the module, copies the Lucide license next to it (`LICENSE-LUCIDE`) and records the tag and the icon count in `tools/icons/source.json`; `check` exits with status 1 when the module differs. The linker drops the icons an app doesn't use. `icon::Freedesktop` maps freedesktop.org icon names to Lucide icons, as oito2_ui does.

## Fonts

`tools/fonts/make_instances.py` writes static instances of the Inter and JetBrains Mono variable fonts with fontTools, in the families `OITO2 Inter` and `OITO2 JetBrains Mono`. See [Theme and tokens](./theme-and-tokens.md#fonts) for why static instances are required.

## API reference of the showcase

The showcase shows an API reference at the end of the component pages, generated from the public items and doc comments of the workspace crates into `apps/showcase/src/generated/api_reference.rs` by the `oito2_iced_gen_docs` tool in `tools/gen_docs`. The CI runs it with `--check`, which fails when the file is out of date.

## CI workflows

| Workflow | Trigger | Jobs |
|---|---|---|
| `ci.yml` | Push to `main`, pull requests, manual | **checks**: `cargo fmt`, `cargo clippy` (`-D warnings`), `cargo doc` (`-D warnings`), `cargo deny`, `oito2_iced_gen_docs --check`; **test**: the workspace tests with `ICED_TEST_BACKEND=tiny-skia` on Ubuntu, Windows and macOS (Windows and macOS don't fail the workflow), uploading failing snapshots; **msrv**: `cargo +1.97 check` |
| `parity.yml` | Every Monday at 06:00 UTC, manual | Checks out `oito2/oito2_ui` `main` and runs `oito2_iced_parity check` |

Actions are pinned by commit SHA, permissions are read-only and every job has a timeout. There is no release workflow yet.
