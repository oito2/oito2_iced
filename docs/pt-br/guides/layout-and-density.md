🌐 [English](../../en/guides/layout-and-density.md) | **Português** | 🏠 [Índice](../index.md)

---

# Layout e densidade

Como dimensionar e espaçar um app para que ele se alinhe aos widgets do OITO2, com ponteiro ou em tela de toque, numa janela larga ou estreita.

## Densidade

Os controles do OITO2 têm 34 pixels de altura para ponteiro e 40 em tela de toque; caixas de seleção, botões de opção e interruptores têm alvos quadrados de 34 e 48 pixels. O Iced faz o layout dos widgets sem o tema, então a densidade fica em `oito2_iced::density`, e o app a define no início de cada `view`:

```rust
use oito2_iced::density;

fn view(&self) -> Element<'_, Message, Theme> {
    density::set(self.theme().density());
    // monta a view
}
```

| Função | Retorna |
|---|---|
| `density::set(Density)`, `density::get()` | Guarda e lê a densidade da thread atual |
| `density::control_height()` | 34 (ponteiro) ou 40 (toque) |
| `density::icon_size()` | O tamanho de ícone da densidade |
| `control_height_of(density)`, `icon_size_of(density)` | O mesmo, para uma densidade dada |

Escolha a densidade pela plataforma (`settings.platform.density()` no `oito2_iced_settings`), deixe o usuário escolhê-la e leve-a no tema com `theme.with_density(density)`, para que o tema e o layout concordem.

## Espaçamento, raios e tamanhos

Use os tokens em vez de números, para que layouts personalizados se alinhem aos widgets:

| Módulo | Valores |
|---|---|
| `tokens::spacing` | `XXS` 2, `XS` 4, `SM` 8, `MD` 12, `LG` 16, `PAGE` 20, `XL` 24, `XXL` 32, `XXXL` 48 |
| `tokens::radius` | `SMALL` 6, `MEDIUM` 8, `LARGE` 12, `WINDOW` 14 |
| `tokens::sizes` | `BUTTON_HEIGHT` 34, `TOUCH_BUTTON_HEIGHT` 40, `TITLE_BAR_HEIGHT` 46, `ICON_SIZE` 20, `MIN_POINTER_TARGET` 24, `MIN_TOUCH_TARGET` 48 |

```rust
use iced::widget::{column, container};
use oito2_iced::tokens::spacing;

let page = container(column![title, body].spacing(spacing::MD)).padding(spacing::PAGE);
```

## Superfícies

As superfícies da janela vêm das cores de chrome do tema, por meio das classes de container:

| Área | Classe |
|---|---|
| Barra lateral | `container::Class::Sidebar` |
| Área da barra de ferramentas principal | `container::Class::Mainbar` |
| Cartões e grupos | `container::Class::Card`, ou `containers::border_container` |
| Barra de título | desenhada por `window::title_bar` e `window::header_bar` |

`widget::containers` também tem `section` (um título sobre conteúdo agrupado), `tile` e `tile_list` (linhas de uma página de configurações) e `watermark`.

## Rolagem

`scroll_area::scroll_area(content)` é um `scrollable` com as barras de rolagem do OITO2; `.undershoot(true)` adiciona sombras suaves nas bordas que têm mais conteúdo além delas. `scroll_area::undershoot` as adiciona a qualquer scrollable montado à mão.

## Layouts adaptativos

Vários widgets mudam de layout conforme a própria largura:

| Widget | Largo | Estreito |
|---|---|---|
| `master_detail` | Lista e detalhe lado a lado, com um divisor redimensionável (a partir de 620 px, `DEFAULT_BREAKPOINT`) | Lista, depois detalhe com um botão de voltar |
| `navigation_page` | Trilho de navegação com rótulos ou estendido ao lado do conteúdo | Abaixo de 600 px (`COMPACT_BREAKPOINT`): um trilho compacto, ou uma barra de navegação inferior com a densidade de toque |
| `pagination` | Números de página com reticências | Forma compacta abaixo de 420 px (`COMPACT_WIDTH`) |

Para layouts próprios, use o widget `responsive` do Iced, que entrega o tamanho disponível a uma closure.

## Painéis redimensionáveis

- `splitter::splitter(&state, panes)` dispõe dois ou mais painéis lado a lado ou empilhados, com divisores arrastáveis e operáveis pelo teclado, tamanhos mínimos e máximos e painéis recolhíveis; os tamanhos ficam num `splitter::State` que o app mantém.
- `paned_view` é a forma de dois painéis com um lado fixo.
- O `pane_grid` do Iced também é estilizado, para ladrilhamento livre.

## Direita para a esquerda

Widgets que apontam para algum lado (sheets, splitters, menus, popovers, calendários, carrosséis, avaliações, indicadores de progresso e outros) aceitam `.direction(ReadingDirection::RightToLeft)` para se espelhar; veja [Localização](./localization.md#direção-de-leitura).
