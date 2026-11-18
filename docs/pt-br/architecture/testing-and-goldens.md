🌐 [English](../../en/architecture/testing-and-goldens.md) | **Português** | 🏠 [Índice](../index.md)

---

# Testes e goldens

Como o workspace é testado: testes unitários, testes de comportamento em um simulador headless, imagens golden com tolerância, testes de paridade e as auditorias do showcase. Código: `crates/*/tests/all/`, `crates/oito2_iced_test_utils`, `apps/showcase/src/`.

## Organização: um binário de teste por crate

Os testes de integração ficam em `crates/<crate>/tests/all/main.rs`, que declara um módulo por área (`tests/all/<area>.rs`), com fixtures em `tests/fixtures/` e goldens em `tests/goldens/`. Um `tests/<name>.rs` no nível de cima seria um binário separado, linkado com todo o Iced; dezenas deles linkados em paralelo esgotam a memória de uma máquina de 16 GB, então eles não são permitidos. Pelo mesmo motivo o workspace compila os binários de debug com `debug = "line-tables-only"`, e as execuções locais limitam os jobs em paralelo (por exemplo `-j 6`).

## Tipos de teste

| Tipo | Onde | O quê |
|---|---|---|
| Unitário | Módulos `#[cfg(test)]` ao lado do código | Regras de cor, contraste, coleções, formatos, atalhos, datas, geometria dos overlays, cálculos de layout |
| Paridade | `tests/all/*parity.rs` (como `scheme_parity.rs`) e módulos por área | Cada valor das fixtures do oito2_ui (veja [Paridade e geradores](./parity-and-generators.md)) |
| Comportamento | `tests/all/<area>.rs` | Cliques, texto digitado, Tab, Enter, Espaço, setas e Escape no simulador `iced_test`, conferindo as mensagens publicadas |
| Golden | `tests/all/<area>.rs` com `golden!` | Snapshots comparados com imagens de referência |
| Doc tests | Exemplos do rustdoc | Os exemplos compilam (`no_run`) ou rodam |

## O simulador

O `iced_test` roda uma view sem interface gráfica: `click`, `tap_key`, `typewrite` e `snapshot`. `oito2_iced_test_utils::settings()` lhe passa as faces da Inter embutidas como fontes e como fonte padrão, para que o texto tenha a mesma medida em todo lugar. A navegação de foco é um widget (`focus::navigation`), não uma subscription, então o Tab funciona no simulador. O simulador não expõe o cursor do mouse; `oito2_iced_test_utils::mouse_interaction` monta uma interface de usuário headless para lê-lo.

## Imagens golden

- Arquivos: `crates/*/tests/goldens/<name>-<renderer>.png`; os testes rodam com `ICED_TEST_BACKEND=tiny-skia`, então a parte do renderizador é `tiny-skia`.
- Comparação: o `Snapshot::matches_image` do próprio Iced é exato no pixel, então o `oito2_iced_test_utils` lê os pixels do snapshot e os compara com tolerância — um pixel conta como diferente quando um canal difere em mais de 2, e no máximo 0,5 % dos pixels podem diferir.
- Um golden ausente faz o teste falhar. `OITO2_UPDATE_GOLDENS=1` escreve ou reescreve os goldens; olhe cada imagem nova ou alterada antes de aceitá-la.
- Uma comparação que falha escreve `<name>.actual.png` ao lado do golden; a CI os envia como o artefato `snapshots-<os>`.
- Os goldens nunca dependem do tempo: nada de spinners indeterminados, nada de `now()`, e as animações são desenhadas no estado final.
- Conjuntos típicos: claro e escuro, alto contraste, densidade compacta e de toque, inglês e português, e da direita para a esquerda onde o widget se espelha.

## Auditorias do showcase

Os testes do showcase montam cada página duas vezes (larga, clara, em inglês; compacta, escura, em português) e rodam duas auditorias:

- **Foco**: o Tab percorre cada página e todo controle interativo precisa receber o foco.
- **Cursor**: todo controle clicável visível precisa mostrar o cursor de mão (cursor de texto em superfícies editáveis, cursores de redimensionamento nas alças).

## Testes de regressão

Toda correção de comportamento observável ganha um teste que falha sem a correção e passa com ela.

## O que continua manual

Gerenciadores de janelas e GPUs reais: arrastar, clique duplo, menu do sistema, cursores de redimensionamento, renderização com wgpu e tiny-skia, e mudanças ao vivo da cor de destaque e do contraste em cada desktop são conferidos à mão em sistemas reais.
