🌐 [English](../../en/concepts/architecture.md) | **Português** | 🏠 [Índice](../index.md)

---

# Arquitetura

Uma visão geral do oito2_iced: como os dados circulam por ele, onde cada parte fica, quais dependências ele usa e as decisões de design por trás dele. As páginas detalhadas, por subsistema, para quem contribui, estão em [`architecture/`](../architecture/theme-and-tokens.md).

## Fluxo de dados

### Do desktop ao tema

```text
Sistema operacional
  (GSettings, arquivos de configuração e o portal de configurações no Linux,
   Windows Runtime, AppKit)
        │  lido por
        ▼
oito2_settings (crates.io, from oito2_rs)
  read_system_settings() / watch()  ──►  SystemSettings
        │
        ▼
oito2_iced_settings
  read()          primeira leitura, no `new` do app
  subscription()  nova leitura depois de cada mudança
        │  Message::Settings(SystemSettings)
        ▼
Modelo do app  ──►  oito2_iced_settings::theme(&settings, accent, fallback)
        │                      ou Preset::theme(contrast)
        ▼
oito2_iced::Theme  ──►  catálogos de estilo do Iced de cada widget
        │
        └──►  barra de cabeçalho: settings.window_control_style, window_button_layout
```

### De um widget ao app e de volta

```text
view(&model)  ──►  builders dos widgets (valores do modelo)
                       │  entrada do usuário: clique, teclas, arrasto
                       ▼
                  Message (on_press, on_change, Event…)
                       │
update(&mut model, message)
   ├── valores simples: atribuição
   ├── controladores: SelectionChange::apply, Toasts::update, Tour::apply…
   └── tasks: window::perform_latest, pick(), download::save()…
                       │
                       ▼
                  view(&model) de novo
```

Os widgets nunca guardam o valor que mostram: o modelo é a única fonte de verdade, como em todo app Iced no estilo Elm. O que um widget guarda no próprio estado da árvore é transitório (foco, hover, progresso de animação, texto sendo digitado).

## Organização dos crates

| Crate | Depende de | Contém |
|---|---|---|
| `oito2_iced` | `iced`, `material-colors`, `oito2_foundation` | Tema, tokens, fontes, ícones, densidade, foco, overlay, widgets, chrome da janela, animação, coleções, formatos, atalhos, strings, datas |
| `oito2_iced_settings` | `oito2_iced`, `oito2_settings` | `read`, `subscription`, `theme` e auxiliares |
| `oito2_iced_themes` | `oito2_iced`, `catppuccin` | `Preset` e as paletas oficiais |
| `oito2_iced_syntax` | `oito2_iced`, `syntect`, `two-face` | Papéis de token, linguagens, realçador, bloco de código, JetBrains Mono |
| `oito2_iced_extras` | `oito2_iced`, `qrcode`, `image` | QR code, painel de assinatura, recortador de imagens, menu de navegação |
| `oito2_iced_files` | `oito2_iced`, `rfd` | Arquivos e grupos de tipos, seletor de arquivos, área de soltar, envio, download |
| `oito2_iced_test_utils` | `oito2_iced`, `iced_test`, `png` | Auxiliares de teste; não publicado |
| `oito2_iced_showcase` (`apps/showcase`) | todos os crates acima | O app showcase |
| `oito2_iced_desktop_example` (`examples/desktop`) | `oito2_iced` e crates de desktop | O código das receitas de desktop |
| `oito2_iced_parity`, `oito2_iced_icons`, `oito2_iced_gen_docs` (`tools/`) | `serde_json`, `syn` | Fixtures de paridade, o módulo de ícones Lucide e a referência de API do showcase |

Dentro do `oito2_iced`, um módulo por assunto: `theme` (papéis, contraste, um submódulo por catálogo de widget nativo), `tokens`, `font`, `icon`, `density`, `focus`, `overlay`, `widget` (um módulo por família), `window`, `animation`, `collections`, `format`, `shortcuts`, `l10n`, `date` e `foundation` (uma reexportação do `oito2_foundation`).

## Dependências principais

| Dependência | Versão | Por quê |
|---|---|---|
| `iced` | 0.14 | O toolkit. O crate principal habilita só `advanced`, `canvas`, `x11`, `wayland` e `thread-pool`; os apps escolhem os renderizadores |
| `material-colors` | 0.5 | Esquemas de cores tonais a partir de uma semente (HCT), a base de todo papel |
| `oito2_foundation`, `oito2_settings` | 0.1.1 | Regras de plataforma e configurações do desktop compartilhadas por todas as interfaces Rust do OITO2 |
| `catppuccin` | 2.8 | As paletas oficiais do Catppuccin |
| `syntect` + `two-face` | 5.3 / 0.5 | Gramáticas TextMate para realce de sintaxe, sem a biblioteca C Oniguruma |
| `rfd` | 0.18 | Diálogos de arquivo; no Linux pelo XDG desktop portal, em Rust puro |
| `qrcode` | 0.13 | Fluxos de bits de QR code e correção de erros |
| `image` | 0.25 | Decodificação e recorte de imagens |
| `iced_test` | 0.14 | O simulador headless por trás dos testes de comportamento |

## Decisões de design

| Decisão | Justificativa |
|---|---|
| O oito2_ui é a fonte de verdade do design | Um design para três runtimes; as fixtures de paridade pegam divergências na CI. |
| Um `Theme` próprio que implementa todo `Catalog` nativo, com classes como enums mais `Custom` | Widgets nativos e widgets do OITO2 compartilham um único sistema de estilos; o `Custom` mantém uma válvula de escape, e o `From<StyleFn>` mantém o builder `style` do Iced funcionando. |
| Widgets em três níveis: widget nativo estilizado, composição, widget próprio | Um widget nativo é usado quando resolve; widgets próprios só para layout, estado ou overlays que faltam no Iced. |
| Camada de foco própria (`focusable`, `navigation`, grupos de foco itinerante) | O Iced 0.14 só dá foco a campos de texto e não associa o Tab; a camada é um widget, então o simulador headless a exercita. |
| Uma única base de overlay ancorado | Posicionamento, inversão, limitação à janela, seta, fechamento e aninhamento são escritos uma vez e reaproveitados por todo popup. |
| Chrome da janela no lado do cliente com alças de redimensionamento | Uma aparência em todo desktop; o Iced não tem bordas de redimensionamento para janelas sem decorações. |
| Densidade como configuração por thread | O Iced faz o layout dos widgets sem o tema. |
| Instâncias estáticas das fontes (`OITO2 Inter`, `OITO2 JetBrains Mono`) | A pilha de texto casa as faces pelo peso exato, então fontes variáveis caem nas fontes do sistema; o nome de família próprio impede que uma Inter instalada as substitua. |
| Sem árvore de acessibilidade | O Iced 0.14 não tem integração com o AccessKit; declarado, não escondido. |
| Datas com tipos próprios `Date` e `Time` | Sem dependência de calendário; os formatos vêm das fixtures de paridade. |
| Iced fixado na 0.14 no `Cargo.lock` | Versões menores do Iced quebram a API; as migrações acontecem em bloco. |
