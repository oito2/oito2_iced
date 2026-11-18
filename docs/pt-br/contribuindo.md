# Como contribuir com o oito2_iced

🌐 **Idioma:** [English](../../CONTRIBUTING.md) · Português

Obrigado pelo seu interesse no oito2_iced. Este guia cobre o ambiente de desenvolvimento, as verificações que toda mudança precisa passar, as regras para testes e imagens golden, os geradores e como as mudanças chegam ao repositório.

## Índice

- [Antes de começar](#antes-de-começar)
- [Ambiente de desenvolvimento](#ambiente-de-desenvolvimento)
- [Estrutura do repositório](#estrutura-do-repositório)
- [Verificações](#verificações)
- [Testes e imagens golden](#testes-e-imagens-golden)
- [Fixtures de paridade](#fixtures-de-paridade)
- [Ícones Lucide](#ícones-lucide)
- [Fontes](#fontes)
- [Convenções de código](#convenções-de-código)
- [Documentação e changelogs](#documentação-e-changelogs)
- [Pull requests](#pull-requests)
- [Processo de lançamento](#processo-de-lançamento)

## Antes de começar

- **Mudanças de design começam no [oito2_ui](https://github.com/oito2/oito2_ui).** Cores, raios, alturas, strings e comportamento são definidos lá e chegam a este repositório pelas fixtures de paridade; o oito2_iced nunca inventa uma cor, um raio ou uma altura.
- Abra uma issue antes de uma mudança grande, para que a abordagem seja combinada primeiro.
- Ao contribuir, você concorda que seu trabalho seja licenciado sob a GPL-3.0-or-later, como o resto do projeto, e segue o [Código de Conduta](codigo-de-conduta.md).

## Ambiente de desenvolvimento

1. Instale o [rustup](https://rustup.rs). O repositório fixa o Rust **1.99.0** com `rustfmt` e `clippy` em `rust-toolchain.toml`; o rustup o instala na primeira chamada ao `cargo`.
2. No Linux, instale os arquivos de desenvolvimento de que a compilação precisa (a CI instala os mesmos):

   ```bash
   sudo apt install libxkbcommon-dev libglib2.0-dev
   ```

3. Instale o [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) para a verificação de dependências, e a versão mínima suportada do Rust para a verificação de MSRV:

   ```bash
   cargo install --locked cargo-deny
   rustup toolchain install 1.97 --profile minimal
   ```

4. Clone o repositório e rode o showcase:

   ```bash
   git clone https://github.com/oito2/oito2_iced.git
   cd oito2_iced
   cargo run -p oito2_iced_showcase
   ```

A [página de pré-requisitos](getting-started/prerequisites.md) traz os detalhes de cada sistema.

## Estrutura do repositório

| Caminho | Conteúdo |
|---|---|
| `crates/oito2_iced` | Crate principal: tema, tokens, fontes, ícones, foco, overlay, widgets, chrome da janela |
| `crates/oito2_iced_settings`, `_themes`, `_syntax`, `_extras`, `_files` | Crates satélites |
| `crates/oito2_iced_test_utils` | Auxiliares de teste (não publicado): configurações do simulador, pixels de snapshot, a macro `golden!` |
| `apps/showcase` | O app showcase (`oito2_iced_showcase`) |
| `examples/desktop` | `oito2_iced_desktop_example`: o código das [receitas](guides/recipes/index.md) de desktop (ícone na bandeja, notificações, início automático) |
| `tools/parity` | `oito2_iced_parity`: copia e confere as fixtures de paridade |
| `tools/icons` | `oito2_iced_icons`: gera o módulo de ícones Lucide |
| `tools/fonts` | `make_instances.py`: as instâncias estáticas das fontes |
| `docs/en`, `docs/pt-br` | Esta documentação, em inglês e em português do Brasil |

## Verificações

Rode todas as verificações antes de abrir um pull request; a CI roda as mesmas no Linux, e os testes também no Windows e no macOS.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --all-features --locked
cargo deny check
cargo run -p oito2_iced_gen_docs --locked -- --check
ICED_TEST_BACKEND=tiny-skia cargo test --workspace --all-features --locked
cargo +1.97 check --workspace --all-features --locked
```

O código que muda conforme a plataforma também passa pelo lint para Windows e macOS (adicione os targets uma vez com `rustup target add x86_64-pc-windows-msvc aarch64-apple-darwin`):

```bash
cargo clippy --workspace --all-features --locked --target x86_64-pc-windows-msvc -- -D warnings
cargo clippy --workspace --all-features --locked --target aarch64-apple-darwin -- -D warnings
```

Linkar os binários de teste de um crate que depende de todo o Iced consome muita memória. Numa máquina com 16 GB de RAM ou menos, limite os jobs em paralelo, por exemplo com `-j 6`.

## Testes e imagens golden

- **Um binário de teste por crate.** Os testes de integração ficam em `crates/<crate>/tests/all/main.rs`, que declara um módulo por área (`tests/all/<area>.rs`). Nunca adicione um `tests/<name>.rs` no nível de cima: cada um é um binário separado, linkado com todo o Iced.
- **Testes de comportamento** usam o simulador `iced_test`: cliques, texto digitado, Tab e Enter. A navegação pelo teclado passa pelo widget `focus::navigation`, então o simulador a exercita.
- **Imagens golden** ficam em `crates/*/tests/goldens/<name>-<renderer>.png` e são comparadas com tolerância de 0,5 % pela macro `golden!` do `oito2_iced_test_utils`. Sempre rode os testes com `ICED_TEST_BACKEND=tiny-skia`, o renderizador com que os goldens são feitos.
- Um **golden ausente faz o teste falhar**. Crie ou atualize goldens com `OITO2_UPDATE_GOLDENS=1` e então **olhe cada imagem nova ou alterada** antes de fazer o commit.
- Um golden que falha grava `<name>.actual.png` ao lado dele (ignorado pelo Git); a CI envia esses arquivos como artefato.
- **As fontes nos testes** são as instâncias estáticas embutidas (família `OITO2 Inter`), então uma cópia do Inter instalada no sistema nunca muda o resultado.
- Toda correção de comportamento observável ganha um teste de regressão que falha sem a correção.

## Fixtures de paridade

As fixtures exportadas pelo oito2_ui só chegam a `crates/*/tests/fixtures/parity/` pela ferramenta de paridade, que também registra o commit de origem em `tools/parity/source.json` e no README de cada diretório de fixtures:

```bash
cargo run -p oito2_iced_parity -- sync <path to oito2_ui>
cargo run -p oito2_iced_parity -- check <path to oito2_ui>
```

O `check` sai com status 1 quando uma fixture difere do checkout. O workflow `Parity` (`.github/workflows/parity.yml`) o roda toda segunda-feira contra o `main` do `oito2_ui`.

## Ícones Lucide

`crates/oito2_iced/src/icon/lucide.rs` é gerado e nunca editado à mão. Gere-o a partir de um checkout do Lucide na tag de release que o oito2_ui usa (registrada em `tools/icons/source.json`):

```bash
cargo run -p oito2_iced_icons -- generate <path to lucide>
cargo run -p oito2_iced_icons -- check <path to lucide>
```

## Fontes

As fontes embutidas são instâncias estáticas das fontes variáveis, feitas por `tools/fonts/make_instances.py` (requer `fontTools`):

```bash
python3 tools/fonts/make_instances.py <Variable.ttf> <Variable-Italic.ttf> <output dir>
python3 tools/fonts/make_instances.py --jetbrains-mono <Variable.ttf> <Variable-Italic.ttf> <output dir>
```

Nunca embuta uma fonte variável: a pilha de texto do Iced casa as faces pelo peso exato, então os outros pesos cairiam numa fonte do sistema.

## Convenções de código

- Todo arquivo `.rs` escrito à mão começa com o cabeçalho GPL-3.0 `Copyright (C) 2026  OITO2`, como em `crates/oito2_iced/src/lib.rs`.
- Os comentários são em inglês e descrevem o que o código faz.
- Todo item público tem rustdoc (`#![deny(missing_docs)]`).
- Os lints do workspace proíbem código `unsafe` e negam `unwrap`, `panic!` e `todo!`; o `clippy.toml` os relaxa só dentro de funções `#[test]`, então os auxiliares de teste usam `assert!`.
- Os nomes de tipos públicos não levam o prefixo `Oito2`. As classes de estilo são enums com as variantes OITO2 mais `Custom(StyleFn)`, e implementam `From<StyleFn>`.
- Confira cada chamada ao Iced contra o Iced **0.14**: a API mudou muito entre a 0.12, a 0.13 e a 0.14. Código de outros projetos nunca é copiado para este projeto GPL.

## Documentação e changelogs

- Uma mudança numa API pública ou num comportamento visível ao usuário atualiza a documentação no mesmo pull request: rustdoc, a página do showcase, e `docs/en` e `docs/pt-br` (as duas árvores mantêm os mesmos arquivos).
- Cada crate publicado tem um `CHANGELOG.md` no formato [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); adicione as mudanças visíveis ao usuário em `[Unreleased]` do crate que elas afetam.

## Pull requests

1. Faça um fork do repositório e crie um branch a partir do `main`.
2. Faça a mudança com seus testes, documentação e entrada no changelog.
3. Rode todas as verificações da seção [Verificações](#verificações).
4. Abra um pull request e preencha o modelo.

## Processo de lançamento

A publicação está prevista para uma versão futura. Hoje todo crate está na versão `0.0.0` com `publish = false`, não há workflow de release e nada é publicado no crates.io; os apps usam os crates como dependências Git. Quando os lançamentos começarem, esta seção vai descrever as tags, a publicação no crates.io e as notas de release tiradas dos changelogs.
