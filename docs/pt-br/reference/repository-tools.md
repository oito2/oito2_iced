🌐 [English](../../en/reference/repository-tools.md) | **Português** | 🏠 [Índice](../index.md)

---

# Ferramentas do repositório

Todos os comandos, ferramentas, variáveis de ambiente e workflows do repositório. Todos os comandos rodam a partir da raiz de um clone.

## Comandos do Cargo

| Comando | O que faz |
|---|---|
| `cargo run -p oito2_iced_showcase` | Compila e executa o showcase |
| `cargo build --release -p oito2_iced_showcase` | Binário otimizado do showcase em `target/release/` |
| `cargo fmt --all -- --check` | Confere a formatação |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Roda o lint em todos os crates, testes e ferramentas |
| `cargo clippy --workspace --all-features --locked --target x86_64-pc-windows-msvc -- -D warnings` | Roda o lint para Windows (requer `rustup target add x86_64-pc-windows-msvc`) |
| `cargo clippy --workspace --all-features --locked --target aarch64-apple-darwin -- -D warnings` | Roda o lint para macOS (requer `rustup target add aarch64-apple-darwin`) |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked` | Gera a documentação da API, falhando em caso de avisos |
| `cargo deny check` | Confere licenças, alertas de segurança, proibições e origens das dependências (`deny.toml`; requer `cargo-deny`) |
| `ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked` | Roda todos os testes com o renderizador por software |
| `cargo +1.97 check --workspace --all-features --locked` | Confere a versão mínima suportada do Rust (requer `rustup toolchain install 1.97`) |

Em máquinas com 16 GB de RAM ou menos, acrescente `-j 6` (ou menos jobs) aos comandos de compilação e de teste.

## Ferramentas

| Ferramenta | Comando | O que faz |
|---|---|---|
| `oito2_iced_parity` (`tools/parity`) | `cargo run -p oito2_iced_parity -- sync <path to oito2_ui>` | Copia as fixtures de paridade do oito2_ui para `crates/*/tests/fixtures/parity/` e registra o commit em `tools/parity/source.json` |
| | `cargo run -p oito2_iced_parity -- check <path to oito2_ui>` | Sai com status 1 quando alguma fixture difere |
| `oito2_iced_icons` (`tools/icons`) | `cargo run -p oito2_iced_icons -- generate <path to lucide>` | Escreve `crates/oito2_iced/src/icon/lucide.rs` e `LICENSE-LUCIDE` e registra a tag em `tools/icons/source.json` |
| | `cargo run -p oito2_iced_icons -- check <path to lucide>` | Sai com status 1 quando o módulo está desatualizado |
| `make_instances.py` (`tools/fonts`) | `python3 tools/fonts/make_instances.py <Variable.ttf> <Variable-Italic.ttf> <output dir>` | Instâncias estáticas da Inter (Regular, Medium, SemiBold, Bold e itálicos) na família `OITO2 Inter`; requer fontTools |
| | `python3 tools/fonts/make_instances.py --jetbrains-mono <Variable.ttf> <Variable-Italic.ttf> <output dir>` | Instâncias estáticas da JetBrains Mono (Regular, Bold e itálicos) em `OITO2 JetBrains Mono` |
| `oito2_iced_gen_docs` (`tools/gen_docs`) | `cargo run -p oito2_iced_gen_docs` | Gera a referência da API do showcase, `apps/showcase/src/generated/api_reference.rs` |
| | `cargo run -p oito2_iced_gen_docs --locked -- --check` | Falha quando a referência gerada está desatualizada (rodado pela CI) |

## Variáveis de ambiente

| Variável | Lida por | Efeito |
|---|---|---|
| `ICED_BACKEND=tiny-skia` | Iced, nos apps e no showcase | Usa o renderizador por software em vez do wgpu |
| `ICED_TEST_BACKEND=tiny-skia` | `iced_test`, nos testes | Renderiza os snapshots com o renderizador por software com que os goldens são feitos |
| `OITO2_UPDATE_GOLDENS=1` | `oito2_iced_test_utils` | Escreve os goldens ausentes e reescreve os alterados em vez de falhar |
| `WAYLAND_DISPLAY`, `WAYLAND_SOCKET` | winit e `oito2_iced_files::drop_zone::can_drop_files` | Quando definidas, o app roda no Wayland, onde arquivos soltos na janela não são informados |

## Arquivos de configuração

| Arquivo | Conteúdo |
|---|---|
| `Cargo.toml` | Membros do workspace, campos de pacote compartilhados, versões das dependências, lints, `[profile.dev]` com `debug = "line-tables-only"` e `opt-level = 1` para as dependências |
| `rust-toolchain.toml` | Rust 1.99.0 com `rustfmt` e `clippy` |
| `clippy.toml` | Permite `unwrap`, `expect` e `panic` dentro de funções `#[test]` |
| `deny.toml` | Licenças aceitas e os alertas ignorados, com o motivo |
| `tools/parity/source.json` | O commit do oito2_ui e a lista de fixtures |
| `tools/icons/source.json` | A tag do Lucide e a quantidade de ícones |

## Workflows

| Workflow | Gatilho | Jobs |
|---|---|---|
| `.github/workflows/ci.yml` | Push para `main`, pull requests, manual | Formatação, lint, documentação, dependências e a referência da API gerada; testes no Ubuntu, Windows e macOS (Windows e macOS em regime de melhor esforço); MSRV 1.97 |
| `.github/workflows/parity.yml` | Segundas-feiras às 06:00 UTC, manual | Compara as fixtures com o `main` de `oito2/oito2_ui` |
