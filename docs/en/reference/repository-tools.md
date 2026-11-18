🌐 [Português](../../pt-br/reference/repository-tools.md) | **English** | 🏠 [Index](../index.md)

---

# Repository tools

Every command, tool, environment variable and workflow of the repository. All commands run from the root of a clone.

## Cargo commands

| Command | Does |
|---|---|
| `cargo run -p oito2_iced_showcase` | Builds and runs the showcase |
| `cargo build --release -p oito2_iced_showcase` | Optimized showcase binary in `target/release/` |
| `cargo fmt --all -- --check` | Checks formatting |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Lints every crate, test and tool |
| `cargo clippy --workspace --all-features --locked --target x86_64-pc-windows-msvc -- -D warnings` | Lints for Windows (needs `rustup target add x86_64-pc-windows-msvc`) |
| `cargo clippy --workspace --all-features --locked --target aarch64-apple-darwin -- -D warnings` | Lints for macOS (needs `rustup target add aarch64-apple-darwin`) |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked` | Builds the API documentation, failing on warnings |
| `cargo deny check` | Checks licenses, advisories, bans and sources of the dependencies (`deny.toml`; needs `cargo-deny`) |
| `ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked` | Runs every test with the software renderer |
| `cargo +1.97 check --workspace --all-features --locked` | Checks the minimum supported Rust version (needs `rustup toolchain install 1.97`) |

On machines with 16 GB of RAM or less, add `-j 6` (or fewer jobs) to the build and test commands.

## Tools

| Tool | Command | Does |
|---|---|---|
| `oito2_iced_parity` (`tools/parity`) | `cargo run -p oito2_iced_parity -- sync <path to oito2_ui>` | Copies the parity fixtures of oito2_ui into `crates/*/tests/fixtures/parity/` and records the commit in `tools/parity/source.json` |
| | `cargo run -p oito2_iced_parity -- check <path to oito2_ui>` | Exits with status 1 when a fixture differs |
| `oito2_iced_icons` (`tools/icons`) | `cargo run -p oito2_iced_icons -- generate <path to lucide>` | Writes `crates/oito2_iced/src/icon/lucide.rs` and `LICENSE-LUCIDE`, records the tag in `tools/icons/source.json` |
| | `cargo run -p oito2_iced_icons -- check <path to lucide>` | Exits with status 1 when the module is out of date |
| `make_instances.py` (`tools/fonts`) | `python3 tools/fonts/make_instances.py <Variable.ttf> <Variable-Italic.ttf> <output dir>` | Static Inter instances (Regular, Medium, SemiBold, Bold and italics) in the family `OITO2 Inter`; needs fontTools |
| | `python3 tools/fonts/make_instances.py --jetbrains-mono <Variable.ttf> <Variable-Italic.ttf> <output dir>` | Static JetBrains Mono instances (Regular, Bold and italics) in `OITO2 JetBrains Mono` |
| `oito2_iced_gen_docs` (`tools/gen_docs`) | `cargo run -p oito2_iced_gen_docs` | Generates the API reference of the showcase, `apps/showcase/src/generated/api_reference.rs` |
| | `cargo run -p oito2_iced_gen_docs --locked -- --check` | Fails when the generated reference is out of date (run by the CI) |

## Environment variables

| Variable | Read by | Effect |
|---|---|---|
| `ICED_BACKEND=tiny-skia` | Iced, in apps and the showcase | Uses the software renderer instead of wgpu |
| `ICED_TEST_BACKEND=tiny-skia` | `iced_test`, in the tests | Renders snapshots with the software renderer the goldens are made with |
| `OITO2_UPDATE_GOLDENS=1` | `oito2_iced_test_utils` | Writes missing goldens and rewrites changed ones instead of failing |
| `WAYLAND_DISPLAY`, `WAYLAND_SOCKET` | winit and `oito2_iced_files::drop_zone::can_drop_files` | When set, the app runs on Wayland, where file drops are not reported |

## Configuration files

| File | Contents |
|---|---|
| `Cargo.toml` | Workspace members, shared package fields, dependency versions, lints, `[profile.dev]` with `debug = "line-tables-only"` and `opt-level = 1` for dependencies |
| `rust-toolchain.toml` | Rust 1.99.0 with `rustfmt` and `clippy` |
| `clippy.toml` | Allows `unwrap`, `expect` and `panic` inside `#[test]` functions |
| `deny.toml` | Accepted licenses and the advisories ignored with their reason |
| `tools/parity/source.json` | The oito2_ui commit and the fixture list |
| `tools/icons/source.json` | The Lucide tag and icon count |

## Workflows

| Workflow | Trigger | Jobs |
|---|---|---|
| `.github/workflows/ci.yml` | Push to `main`, pull requests, manual | Format, lint, docs, dependencies and the generated API reference; tests on Ubuntu, Windows and macOS (Windows and macOS best effort); MSRV 1.97 |
| `.github/workflows/parity.yml` | Mondays at 06:00 UTC, manual | Compares the fixtures with `oito2/oito2_ui` `main` |
