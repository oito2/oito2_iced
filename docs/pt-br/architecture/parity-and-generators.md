🌐 [English](../../en/architecture/parity-and-generators.md) | **Português** | 🏠 [Índice](../index.md)

---

# Paridade e geradores

Como o design se mantém igual ao do oito2_ui, quais arquivos são gerados e o que a CI roda. Código: `tools/parity`, `tools/icons`, `tools/fonts`, `.github/workflows/`.

## Fixtures de paridade

O oito2_ui exporta seu design como fixtures JSON em `tools/parity/fixtures`. A ferramenta de paridade as copia para as fixtures de teste dos crates:

```bash
cargo run -p oito2_iced_parity -- sync <path to oito2_ui>
cargo run -p oito2_iced_parity -- check <path to oito2_ui>
```

O `sync` copia todas as fixtures e depois registra o commit de origem em `tools/parity/source.json` e no README de cada diretório de fixtures; o `check` sai com status 1 quando uma fixture difere do checkout. As fixtures nunca são editadas à mão.

| Fixture | Copiada para | Conferida pelos testes |
|---|---|---|
| `tokens.json` | `crates/oito2_iced/tests/fixtures/parity` | Raios, espaçamentos, tamanhos, cores de destaque, neutros, cores de status, escala tipográfica |
| `schemes.json` | `crates/oito2_iced/…`, `crates/oito2_iced_themes/…` | Cada papel de cor dos temas base e dos 13 presets, contraste padrão e alto; as paletas dos presets |
| `foundation.json` | `crates/oito2_iced/…` | Razões de contraste, escolha da cor do primeiro plano e `ensure` |
| `l10n.json` | `crates/oito2_iced/…` | Cada string em inglês e em português, e os formatos de data e hora |
| `format.json` | `crates/oito2_iced/…` | Tamanhos em bytes, números compactos e tempos relativos |
| `manifest.json` | `crates/oito2_iced/…` | Não é comparada: registra a versão do Flutter e as versões dos pacotes com que as fixtures foram exportadas |
| `syntax.json` | `crates/oito2_iced_syntax/…` | A cor de cada papel de sintaxe em cada tema base |
| `qr.json` | `crates/oito2_iced_extras/…` | Matrizes de QR code, módulo a módulo |

Uma mudança de design, portanto, começa no oito2_ui, é exportada lá e chega a este repositório com o `sync`.

### Divergência conhecida: tokens de sintaxe

O oito2_ui realça código com o highlight.js; o oito2_iced usa gramáticas TextMate (`syntect` com o conjunto de gramáticas `two-face`). As cores de cada papel são comparadas exatamente, mas os limites dos tokens diferem, então a tokenização é medida como concordância de caracteres com o highlight.js (pelo menos 60 % por linguagem e 80 % no total) em vez de comparada exatamente.

## Ícones Lucide

`crates/oito2_iced/src/icon/lucide.rs` guarda cada ícone Lucide como uma constante `Icon` (`folder-open` é `FOLDER_OPEN`). Ele é gerado a partir de um checkout do Lucide em uma tag de release — a tag que o oito2_ui usa — e nunca é editado à mão:

```bash
cargo run -p oito2_iced_icons -- generate <path to lucide>
cargo run -p oito2_iced_icons -- check <path to lucide>
```

O `generate` escreve o módulo, copia a licença do Lucide para o lado dele (`LICENSE-LUCIDE`) e registra a tag e a quantidade de ícones em `tools/icons/source.json`; o `check` sai com status 1 quando o módulo difere. O linker descarta os ícones que um app não usa. `icon::Freedesktop` mapeia nomes de ícones do freedesktop.org para ícones Lucide, como o oito2_ui faz.

## Fontes

`tools/fonts/make_instances.py` escreve instâncias estáticas das fontes variáveis Inter e JetBrains Mono com o fontTools, nas famílias `OITO2 Inter` e `OITO2 JetBrains Mono`. Veja em [Tema e tokens](./theme-and-tokens.md#fontes) por que instâncias estáticas são obrigatórias.

## Referência da API no showcase

O showcase mostra uma referência da API no fim das páginas de componentes, gerada a partir dos itens públicos e dos comentários de documentação dos crates do workspace em `apps/showcase/src/generated/api_reference.rs` pela ferramenta `oito2_iced_gen_docs`, em `tools/gen_docs`. A CI a roda com `--check`, que falha quando o arquivo está desatualizado.

## Workflows de CI

| Workflow | Gatilho | Jobs |
|---|---|---|
| `ci.yml` | Push para `main`, pull requests, manual | **checks**: `cargo fmt`, `cargo clippy` (`-D warnings`), `cargo doc` (`-D warnings`), `cargo deny`, `oito2_iced_gen_docs --check`; **test**: os testes do workspace com `ICED_TEST_BACKEND=tiny-skia` no Ubuntu, no Windows e no macOS (Windows e macOS não fazem o workflow falhar), enviando os snapshots que falharam; **msrv**: `cargo +1.97 check` |
| `parity.yml` | Toda segunda-feira às 06:00 UTC, manual | Faz checkout de `oito2/oito2_ui` `main` e roda `oito2_iced_parity check` |

As actions são fixadas por SHA de commit, as permissões são somente leitura e todo job tem um tempo limite. Ainda não há workflow de release.
