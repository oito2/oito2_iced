🌐 [English](../../en/architecture/overlay.md) | **Português** | 🏠 [Índice](../index.md)

---

# Overlay

Como os popups abrem ao lado da sua âncora e como as superfícies que cobrem a janela ficam sobre a view. Código: `crates/oito2_iced/src/overlay/anchored.rs` e os hosts em `crates/oito2_iced/src/widget/{toast,dialog,sheet,tour,command_palette}.rs`.

## Por que uma base própria

O widget `float` do Iced não tem posicionamento nem fechamento e é desenhado abaixo de outros overlays, e o `pick_list` não consegue receber o foco. O oito2_iced implementa `iced::advanced::overlay::Overlay` uma vez, em `overlay::anchored`, e constrói todos os popups sobre ele.

## O painel ancorado

`overlay::anchored(anchor, panel)` mantém a âncora no layout e, enquanto `open(true)`, mostra o painel em um overlay:

| Aspecto | Comportamento |
|---|---|
| Posicionamento | `Placement::{Top, Bottom, Start, End}`; início e fim seguem a direção de leitura (`direction`) |
| Alinhamento | `Alignment::{Start, Center, End}` ao longo da âncora |
| Inversão | Quando o painel não cabe no lado preferido, ele abre no lado oposto se esse lado tiver mais espaço |
| Contenção | O painel se desloca para ficar dentro da janela, a `margin` de distância das bordas; `max_width` e `max_height` o limitam |
| Largura | `match_anchor_width` o deixa tão largo quanto a âncora; `fit_anchor_width`, pelo menos tão largo |
| Seta | `arrow(true)` desenha um indicador apontando para a âncora, no lado em que o painel ficou |
| Superfície | Popover por padrão; `.menu()` e `.tooltip()` trocam para as superfícies de menu e de tooltip |
| Fechamento | Um clique do mouse fora do painel e da âncora, ou o Escape, publica `on_dismiss`; o app fecha o painel renderizando `open(false)` |
| Aninhamento | Um painel dentro de outro abre acima dele; um clique dentro do painel interno não fecha o externo |

O overlay informa o índice 2.0, acima dos overlays do próprio Iced. A geometria (`Geometry::side_for`, `placed_at`) é pura e testada sem janela; `overlay::place` é reutilizado pelo cartão do tour.

Dois detalhes aprendidos a duras penas:

- O Iced recria um overlay para desenhá-lo sem chamar `layout` de novo, então o lado e a seta são derivados dos limites no `draw`.
- O Iced considera o ponteiro sobre um overlay filho só quando o `mouse_interaction` dele não é `None`; um painel com conteúdo inerte reivindica o ponteiro na própria área, para que um clique dentro de um painel aninhado não feche o painel pai.

## Construídos sobre ele

| Widget | Módulo |
|---|---|
| Popover | `widget::popover` |
| Hover card | `widget::hover_card` |
| Tooltip | `widget::tooltip` |
| Menus, botão de menu popup, select | `widget::menu` |
| Menu de contexto | `widget::context_menu` |
| Split button | `widget::split_button` |
| Autocomplete e sugestões de tags | `widget::autocomplete`, `widget::tags_field` |
| Seletores de data e de cor | `widget::calendar`, `widget::entries`, `widget::color_picker` |
| Painéis do menu de navegação | `oito2_iced_extras::navigation_menu` |

## Hosts na raiz

Superfícies que cobrem a janela não se ancoram a um elemento; elas envolvem a raiz da view com um `stack`:

| Host | Superfície | Controlado pelo app |
|---|---|---|
| `toast::toaster(view, &toasts, …)` | Pilha de toasts na superfície inversa, em uma de seis posições | A fila `Toasts` |
| `dialog::host(view, Option<Dialog>)` | Diálogo centralizado sobre um scrim (0.54) | Se um diálogo é mostrado |
| `sheet::host(view, Option<Sheet>)` | Sheet lateral ou inferior, modal ou não, com pontos de encaixe | Se está aberto, sua altura de repouso |
| `tour::host(view, &tour)` | Scrim com um holofote sobre o alvo e um cartão | O valor `Tour` |
| `command_palette::command_palette(view, …)` | Lista de comandos com busca | Se está aberta, a consulta |

Enquanto um host modal está aberto, a view abaixo não recebe entrada e o Tab circula dentro da superfície.
