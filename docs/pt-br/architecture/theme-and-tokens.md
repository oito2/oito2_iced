🌐 [English](../../en/architecture/theme-and-tokens.md) | **Português** | 🏠 [Índice](../index.md)

---

# Tema e tokens

Como o `oito2_iced` transforma uma cor de destaque ou um preset nas cores de cada widget, e onde ficam os números compartilhados. Código: `crates/oito2_iced/src/{tokens.rs,theme/,font.rs,density.rs}` e `crates/oito2_iced_themes/src/`.

## Tokens

O `tokens.rs` guarda os valores exportados pelo oito2_ui no `tokens.json`, comparados pelos testes de paridade:

| Módulo | Conteúdo |
|---|---|
| `tokens::radius` | `CHECK` 4, `SMALL` 6, `MEDIUM` 8, `LARGE` 12, `WINDOW` 14 |
| `tokens::spacing` | `XXS` 2, `XS` 4, `SM` 8, `MD` 12, `LG` 16, `PAGE` 20, `XL` 24, `XXL` 32, `XXXL` 48 |
| `tokens::sizes` | Altura do botão 34 (toque 40), barra de título 46, ícone 20, borda de foco 2, controle da janela 24, progresso 6, alvos mínimos 24 (ponteiro) e 48 (toque) |
| `tokens::typography` | Os 15 estilos de texto do Material (`DISPLAY_LARGE` … `LABEL_SMALL`), o peso de ênfase e a lista de fontes alternativas |
| `tokens::Accent` | As nove cores de destaque e suas cores |
| `tokens::Neutrals`, `tokens::Status` | As superfícies neutras no estilo GTK e as cores de status, claras e escuras |

## De uma cor de destaque aos papéis de cor

```text
cor de destaque + brilho + contraste
        │
        ▼
material-colors: SchemeTonalSpot (HCT)    ── paletas tonais a partir da semente
        │
        ▼
theme::roles: compõe os papéis do Material 3 sobre os neutros do OITO2,
              cores de chrome, semânticas e de sintaxe
        │
        ▼
contrast::ensure: ajusta cada papel de texto até a meta WCAG
              contra as superfícies em que aparece
        │
        ▼
ColorRoles  ──►  Theme
```

- `ColorRoles::base(brightness, accent, contrast)` monta os papéis dos temas base do OITO2; `ColorRoles::resolve(&Palette, contrast)` monta os papéis a partir das cores brutas de uma família (`Palette`), que é como os presets funcionam.
- `theme::contrast` reúne os auxiliares da WCAG: `ratio`, `foreground_for`, `min_ratio`, `ensure`, e as metas `TEXT_AA` (4.5), `NON_TEXT_AA` (3.0) e `TEXT_AAA` (7.0).
- Os testes de paridade comparam cada papel de cada tema base (nove cores de destaque × claro e escuro × contraste padrão e alto) com o `schemes.json`, e cada caso de contraste com o `foundation.json`.

## O `Theme`

O `Theme` guarda os `ColorRoles` resolvidos, um nome e uma densidade, atrás de um ponteiro compartilhado, para que cloná-lo seja barato.

| Construtor | Monta |
|---|---|
| `Theme::new(brightness, Accent)` | Um tema base a partir de uma das nove cores de destaque |
| `Theme::base(brightness, Color, Contrast)` | Um tema base a partir de qualquer cor de destaque e de um nível de contraste |
| `Theme::light()`, `Theme::dark()` | Os temas base com a cor de destaque padrão (azul) |
| `Theme::from_palette(name, &Palette, Contrast)` | Um tema a partir das cores brutas de uma família |
| `Theme::from_roles(name, ColorRoles)` | Um tema a partir de papéis montados à mão |

`with_density`, `with_name`, `brightness`, `contrast`, `density`, `roles`, `control_height`, `icon_size` e `focus_ring` o completam. O `Theme` implementa `iced::theme::Base`, então o Iced também consegue escolher sozinho um padrão claro ou escuro.

## Catálogos de estilo e classes

O `theme/` tem um submódulo por widget nativo do Iced: `button`, `checkbox`, `container`, `float`, `menu`, `pane_grid`, `pick_list`, `progress_bar`, `radio`, `rule`, `scrollable`, `slider`, `table`, `text`, `text_editor`, `text_input`, `toggler` e, atrás de features, `svg`, `qr_code` e `markdown`. Cada um implementa o `Catalog` do widget para o `Theme` com um enum `Class`:

```rust
pub enum Class<'a> {
    Filled,
    Tonal,
    Elevated,
    Outlined,
    Text,
    Danger,
    Chrome,
    Fab,
    Icon,
    Custom(StyleFn<'a, Theme>),
}
```

`impl From<StyleFn> for Class` mantém funcionando o builder `.style(|theme, status| …)` de cada widget do Iced; escolher uma classe com `.class(…)` exige a feature `advanced` do Iced, que o `oito2_iced` ativa. As camadas de estado seguem o Material: `HOVER_OPACITY` 0.08, `PRESSED_OPACITY` 0.10, contêineres desativados a 0.12 e conteúdo desativado a 0.38.

## Presets

`oito2_iced_themes::Preset` lista os 13 presets. Cada um tem sua paleta oficial (`palettes::catppuccin` lê o crate `catppuccin`, os outros estão escritos por extenso), transformada em `FamilyColors`, depois em uma `Palette` (`Palette::from_family`), depois em `ColorRoles` com as mesmas regras de contraste dos temas base. Os testes de paridade comparam os 13 presets × contraste padrão e alto com o `schemes.json`, além das 212 cores das paletas.

## Fontes

O `font.rs` embute oito instâncias estáticas da Inter (Regular, Medium, SemiBold, Bold e seus itálicos) na família `OITO2 Inter`; o `oito2_iced_syntax::font` embute quatro da JetBrains Mono (Regular, Bold e itálicos) em `OITO2 JetBrains Mono`. Instâncias estáticas são obrigatórias: a pilha de texto do Iced encontra uma face pelo peso exato, então os outros pesos de uma fonte variável caem para uma fonte do sistema. Os nomes de família próprios impedem que uma cópia da Inter instalada no sistema as substitua. As fontes estão sob a SIL Open Font License (`assets/fonts/OFL.txt`).

## Densidade

`density::set(Density)` guarda a densidade de ponteiro ou de toque em uma thread-local; os widgets leem `density::control_height()` e `density::icon_size()` enquanto fazem o layout. O app a chama no início do `view`, com a densidade do seu tema, para que layout e estilo concordem.
