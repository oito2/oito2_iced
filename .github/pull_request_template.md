## Summary

<!-- What does this change do, and why? -->

Closes #

## How it was tested

<!-- Tests added or run, goldens reviewed, systems and desktops checked by hand. -->

## Checklist

- [ ] `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` pass
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked` and `cargo deny check` pass
- [ ] `ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked` passes, with tests added or updated
- [ ] Every new or changed golden image was looked at
- [ ] `cargo run -p oito2_iced_gen_docs --locked -- --check` passes (or the reference was regenerated)
- [ ] `cargo +1.97 check --workspace --all-features --locked` passes
- [ ] `CHANGELOG.md` of each affected crate updated under `[Unreleased]`, for user-facing changes
- [ ] Showcase page and documentation (`docs/en` and `docs/pt-br`) updated
- [ ] New source files carry the GPL-3.0 header
