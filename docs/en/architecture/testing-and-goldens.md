🌐 [Português](../../pt-br/architecture/testing-and-goldens.md) | **English** | 🏠 [Index](../index.md)

---

# Testing and goldens

How the workspace is tested: unit tests, behavior tests in a headless simulator, golden images with a tolerance, parity tests, and the audits of the showcase. Code: `crates/*/tests/all/`, `crates/oito2_iced_test_utils`, `apps/showcase/src/`.

## Layout: one test binary per crate

Integration tests live in `crates/<crate>/tests/all/main.rs`, which declares one module per area (`tests/all/<area>.rs`), with fixtures in `tests/fixtures/` and goldens in `tests/goldens/`. A top-level `tests/<name>.rs` would be a separate binary linked with all of Iced; dozens of them linked in parallel exhaust the memory of a 16 GB machine, so they are not allowed. For the same reason the workspace builds debug binaries with `debug = "line-tables-only"`, and local runs limit parallel jobs (for example `-j 6`).

## Kinds of tests

| Kind | Where | What |
|---|---|---|
| Unit | `#[cfg(test)]` modules next to the code | Color rules, contrast, collections, formats, shortcuts, dates, geometry of overlays, layout math |
| Parity | `tests/all/*parity.rs` (such as `scheme_parity.rs`) and per-area modules | Every value of the oito2_ui fixtures (see [Parity and generators](./parity-and-generators.md)) |
| Behavior | `tests/all/<area>.rs` | Clicks, typed text, Tab, Enter, Space, arrows and Escape in the `iced_test` simulator, checking the published messages |
| Golden | `tests/all/<area>.rs` with `golden!` | Snapshots compared with reference images |
| Doc tests | Rustdoc examples | Examples compile (`no_run`) or run |

## The simulator

`iced_test` runs a view headlessly: `click`, `tap_key`, `typewrite` and `snapshot`. `oito2_iced_test_utils::settings()` gives it the embedded Inter faces as fonts and the default font, so text measures the same everywhere. Focus traversal is a widget (`focus::navigation`), not a subscription, so Tab works in the simulator. The simulator doesn't expose the mouse cursor; `oito2_iced_test_utils::mouse_interaction` builds a headless user interface to read it.

## Golden images

- Files: `crates/*/tests/goldens/<name>-<renderer>.png`; the tests run with `ICED_TEST_BACKEND=tiny-skia`, so the renderer part is `tiny-skia`.
- Comparison: Iced's own `Snapshot::matches_image` is pixel-exact, so `oito2_iced_test_utils` reads the snapshot's pixels and compares them with a tolerance — a pixel counts as different when a channel differs by more than 2, and at most 0.5 % of the pixels may differ.
- A missing golden fails the test. `OITO2_UPDATE_GOLDENS=1` writes or rewrites the goldens; look at every new or changed image before accepting it.
- A failing comparison writes `<name>.actual.png` next to the golden; the CI uploads these as the `snapshots-<os>` artifact.
- Goldens never depend on time: no indeterminate spinners, no `now()`, and animations are drawn in their final state.
- Typical sets: light and dark, high contrast, compact and touch density, English and Portuguese, and right-to-left where the widget mirrors.

## Showcase audits

The showcase tests build every page twice (wide, light, English; compact, dark, Portuguese) and run two audits:

- **Focus**: Tab walks every page and every interactive control must receive the focus.
- **Cursor**: every visible clickable control must show the hand cursor (text cursor for editable surfaces, resize cursors on handles).

## Regression tests

Every fix of observable behavior gets a test that fails without the fix and passes with it.

## What stays manual

Real window managers and GPUs: drag, double click, system menu, resize cursors, wgpu and tiny-skia rendering, and live changes of accent and contrast on each desktop are checked by hand on live systems.
