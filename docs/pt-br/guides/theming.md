🌐 [English](../../en/guides/theming.md) | **Português** | 🏠 [Índice](../index.md)

---

# Temas

Como escolher as cores de um app: destaques, claro e escuro, alto contraste, presets, as configurações do desktop e classes por widget.

## Entregue o tema ao Iced

Retorne um `oito2_iced::Theme` da função de tema da aplicação. As views o usam como tipo de tema dos seus elementos (`Element<'_, Message, Theme>`, ou o alias `oito2_iced::Element<'_, Message>`).

```rust
use oito2_iced::Theme;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::Accent;

fn theme(state: &App) -> Theme {
    if state.dark {
        Theme::new(Brightness::Dark, Accent::Green)
    } else {
        Theme::new(Brightness::Light, Accent::Green)
    }
}
```

| Construtor | Use para |
|---|---|
| `Theme::light()`, `Theme::dark()` | Os temas base do OITO2 com o destaque azul |
| `Theme::new(brightness, Accent)` | Um dos nove destaques: azul, verde-azulado, verde, amarelo, laranja, vermelho, rosa, roxo, ardósia |
| `Theme::base(brightness, color, Contrast)` | Qualquer cor de destaque, com contraste padrão ou alto |
| `Preset::theme(Contrast)` | Um preset nomeado do `oito2_iced_themes` |
| `oito2_iced_settings::theme(&settings, accent, fallback)` | O destaque do desktop, a preferência por claro ou escuro e o contraste |
| `Theme::from_palette`, `Theme::from_roles` | Sua própria família de cores ou seus próprios papéis |

## Acompanhe o desktop

Com o `oito2_iced_settings`, mantenha as `SystemSettings` no modelo, atualize-as a partir de `subscription()` e construa o tema a partir delas. Passar `Some(color)` como destaque substitui o destaque do desktop; o brilho de fallback vale quando o desktop não declara preferência.

```rust
fn theme(&self) -> Theme {
    oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
}
```

Veja [Integração com o desktop](./desktop-integration.md) para a subscription.

## Alto contraste

`Contrast::High` aumenta o contraste do texto e dos contornos em todas as superfícies. `oito2_iced_settings::contrast(&settings)` o retorna quando o desktop pede alto contraste; passe-o para `Theme::base` ou para um preset.

## Presets

```rust
use oito2_iced::theme::Contrast;
use oito2_iced_themes::Preset;

let theme = Preset::Dracula.theme(Contrast::Standard);
```

| Família | Presets |
|---|---|
| Everforest | `EverforestLight`, `EverforestDark` |
| Rosé Pine | `RosePine`, `RosePineMoon`, `RosePineDawn` |
| Nord | `NordDark`, `NordLight` |
| Dracula | `Dracula` (escuro), `Alucard` (claro) |
| Catppuccin | `CatppuccinLatte`, `CatppuccinFrappe`, `CatppuccinMacchiato`, `CatppuccinMocha` |

`Preset::ALL` lista todos eles, `label()` dá um nome de exibição, `name()` e `from_name()` um identificador estável para arquivos de configuração, e `Preset::with_brightness(brightness)` os presets de um mesmo brilho.

## Leia as cores

`theme.roles()` retorna os `ColorRoles`: todos os papéis do Material 3 (`primary`, `surface_container_high`, `outline_variant`…) e mais:

| Grupo | Cores |
|---|---|
| `roles.chrome` | `background`, `sidebar`, `mainbar`, `title_bar`, `dialog`: as superfícies da janela |
| `roles.semantic` | `primary_text`, `error_text`, `success`, `warning`, `info`, `link`: tons seguros para texto |
| `roles.syntax` | As cores de código usadas pelo `oito2_iced_syntax` |

```rust
use iced::widget::text;
use oito2_iced::Theme;

let saved = text("Saved").style(|theme: &Theme| text::Style {
    color: Some(theme.roles().semantic.success),
});
```

## Classes

Cada widget nativo do Iced escolhe uma classe do catálogo do OITO2 com `.class(…)`; builders do OITO2 como `button::filled` já escolhem uma.

| Catálogo | Classes |
|---|---|
| `theme::button::Class` | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`, `Danger`, `Chrome`, `Fab`, `Icon` |
| `theme::container::Class` | `Transparent`, `Background`, `TitleBar`, `Sidebar`, `Mainbar`, `Card`, `Dialog`, `Popover`, `Menu`, `Tooltip`, `Inverse` |
| `theme::text::Class` | `Default`, `Variant`, `Primary`, `Error` |

```rust
use iced::widget::{container, text};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;

let sidebar = container(text("Library").class(TextClass::Variant)).class(ContainerClass::Sidebar);
```

Todo enum de classe também tem `Custom` e aceita uma closure via `From`, então o builder `.style(…)` do Iced continua funcionando para estilos pontuais.

## Utilitários de contraste

`oito2_iced::theme::contrast` confere seus próprios pares de cores com as fórmulas da WCAG: `ratio(a, b)`, `foreground_for(background)` e `ensure(color, &backgrounds, target)`, com as metas `TEXT_AA` (4,5), `NON_TEXT_AA` (3,0) e `TEXT_AAA` (7,0).

## Densidade

O tema carrega uma densidade (`with_density`); veja [Layout e densidade](./layout-and-density.md).
