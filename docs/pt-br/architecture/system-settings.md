🌐 [English](../../en/architecture/system-settings.md) | **Português** | 🏠 [Índice](../index.md)

---

# Configurações do sistema

Como um app descobre a cor de destaque do desktop, a preferência por claro ou escuro, o alto contraste e os botões da janela, e continua acompanhando essas configurações. Código: `crates/oito2_iced_settings/src/lib.rs`, sobre os crates `oito2_settings` e `oito2_foundation` (versão 0.1.1, do repositório `oito2_rs`, no crates.io).

## Camadas

```text
oito2_settings (leitores por plataforma)
  Linux:   GSettings, arquivos de configuração (KDE, Xfce, COSMIC…) e o
           portal de configurações, via GIO
  Windows: Windows Runtime
  macOS:   AppKit e os user defaults
        │  RawSettings
        ▼
oito2_foundation (regras, sem E/S)
  ambiente de desktop, estilo dos controles da janela, layout dos botões,
  cor de destaque, esquema de cores, alto contraste  ──►  SystemSettings
        │
        ▼
oito2_iced_settings (integração com o Iced)
  read(), subscription(), theme(), brightness(), contrast(), accent()
```

Todo leitor pode ser chamado com segurança de qualquer thread e nunca falha: um armazenamento que não pode ser lido deixa seus valores vazios, e a composição cai para os padrões do desktop. Os mesmos dois crates atendem aos plugins Tauri do oito2_uikit, então todo app Rust do OITO2 lê o sistema do mesmo jeito.

## `SystemSettings`

| Campo | Significado |
|---|---|
| `platform` | Sistema operacional e entrada por toque; `platform.density()` dá a densidade de ponteiro ou de toque |
| `desktop_environment` | GNOME, KDE, Xfce, Budgie, COSMIC, Cinnamon, MATE, LXQt, Pantheon, Unity ou Unknown; `None` fora do Linux nativo |
| `window_control_style` | A família visual dos botões da janela |
| `window_button_layout` | Quais botões aparecem e em que lado |
| `accent_color` | A cor de destaque do sistema como `#RRGGBB`, ou `None` |
| `color_scheme` | `Some(Light)`, `Some(Dark)` ou `None` quando o sistema não declara preferência |
| `high_contrast` | Se o sistema pede alto contraste |

## O lado do Iced

- `read()` devolve as configurações atuais; chame-o quando o app inicia.
- `subscription()` é um `Subscription::run` sobre um stream que entrega as configurações atuais de imediato e, depois, as configurações após cada mudança informada pelo `oito2_settings::watch`, um observador em segundo plano que vive enquanto a subscription existir. Quando a plataforma não pode ser observada, só a primeira leitura chega.
- `theme(&settings, accent, fallback)` monta o tema base: a `accent` explícita, senão a cor de destaque do sistema, senão a cor de destaque padrão (azul); a preferência do sistema por claro ou escuro, senão `fallback`; alto contraste quando o sistema pede; e a densidade da plataforma.
- `brightness`, `contrast` e `accent` expõem cada regra separadamente, para apps que montam o tema de outro jeito (por exemplo a partir de um preset: `Preset::theme(contrast(&settings))`).

## O que o app faz com elas

O app guarda `SystemSettings` no seu modelo e reconstrói o que depende delas em `theme` e `view`: o tema, a densidade passada para `density::set`, e o `control_style` e o `button_layout` da header bar. O showcase também deixa o usuário sobrepor cada escolha no seu diálogo de configurações (modo, preset, cor de destaque, contraste, densidade, desktop simulado).

## Testes

As regras ficam no `oito2_foundation` e são testadas lá. O `oito2_iced_settings` tem testes unitários para as regras do tema (valores padrão, preferências do sistema, cor de destaque explícita) e um doc test da subscription.
