# Documentação — oito2_iced

🌐 [English](../en/index.md) | **Português** | 🏠 [Voltar ao README](leiame.md)

---

O oito2_iced é um design system para apps desktop nativos em Rust feitos com o Iced 0.14, com o Linux em primeiro lugar e o Windows e o macOS em regime de melhor esforço. Ele dá aos apps sem webview a mesma identidade visual do design system Flutter oito2_ui e do design system Tauri oito2_uikit — neutros no estilo GTK, densidade desktop de 34 px, papéis de cor do Material 3, 13 presets e contraste verificado pela WCAG — com 84 componentes principais que recebem foco de teclado, chrome de janela no lado do cliente em seis estilos e crates satélites para configurações do sistema, presets, realce de sintaxe, extras e arquivos.

---

## 🚀 Primeiros passos

- [Pré-requisitos](./getting-started/prerequisites.md) — a toolchain do Rust e os pacotes de sistema para cada sistema operacional.
- [Instalação](./getting-started/installation.md) — como adicionar os crates a um app e compilar o showcase a partir do código-fonte, por sistema operacional.
- [Início rápido](./getting-started/quickstart.md) — o caminho mais curto até um app com o tema OITO2, as fontes, o foco de teclado e o chrome da janela.

---

## 🧠 Conceitos

- [Por que o oito2_iced](./concepts/why-oito2_iced.md) — o problema que ele resolve, por que Iced, e quando escolher o oito2_uikit no lugar dele.
- [Como o oito2_iced funciona](./concepts/how-oito2_iced-works.md) — um passeio do tema aos widgets, foco, overlays, chrome da janela e configurações do sistema.
- [Visão geral da arquitetura](./concepts/architecture.md) — fluxo de dados, organização dos crates, dependências principais e as principais decisões de design.
- [Glossário](./concepts/glossary.md) — os termos do projeto, de papéis de cor a goldens.

---

## 🏗️ Arquitetura (para quem contribui)

- [Tema e tokens](./architecture/theme-and-tokens.md) — tokens, papéis de cor, contraste, os catálogos de estilo, presets e fontes.
- [Foco e teclado](./architecture/focus-and-keyboard.md) — o wrapper focável, a navegação com Tab, o foco itinerante, os anéis de foco e os atalhos.
- [Overlay](./architecture/overlay.md) — o painel ancorado por trás de popovers, menus e tooltips, e os hosts na raiz para diálogos, painéis, toasts e tours.
- [Chrome da janela](./architecture/window-chrome.md) — barra de título, barra de cabeçalho, controles da janela, alças de redimensionamento e o estado da janela.
- [Animação](./architecture/animation.md) — curvas e durações do Material, e como os widgets animam sem subscriptions.
- [Configurações do sistema](./architecture/system-settings.md) — dos armazenamentos do desktop ao tema, passando pelo `oito2_settings` e pela subscription.
- [Paridade e geradores](./architecture/parity-and-generators.md) — as fixtures de paridade, os geradores de ícones e de fontes, e os workflows de CI.
- [Testes e goldens](./architecture/testing-and-goldens.md) — o simulador, as imagens golden, as auditorias e a organização dos testes.

---

## 🧭 Guias

- [Temas](./guides/theming.md) — cores de destaque, claro e escuro, alto contraste, presets, papéis personalizados e classes de estilo.
- [Layout e densidade](./guides/layout-and-density.md) — tamanhos para ponteiro e toque, tokens de espaçamento e layouts responsivos.
- [Localização](./guides/localization.md) — strings em inglês e em português, formatos, datas e direção de leitura.
- [Arquivos](./guides/files.md) — seletor de arquivos, área de soltar, validação de envios e downloads.
- [Integração com o desktop](./guides/desktop-integration.md) — janelas sem decoração, chrome da janela, configurações do sistema e ids de aplicativo.
- [Receitas](./guides/recipes/index.md) — ícones na bandeja com `iced::daemon`, notificações, início automático e empacotamento.

---

## 📚 Referência

- [Crates e features](./reference/crates-and-features.md) — todos os crates, seus módulos, features do Cargo e dependências.
- [Componentes](./reference/components.md) — todas as páginas de componentes do showcase e o módulo Rust por trás de cada uma.
- [Páginas do showcase](./reference/showcase-pages.md) — todas as páginas do showcase, por grupo.
- [Ferramentas do repositório](./reference/repository-tools.md) — comandos do Cargo, ferramentas, variáveis de ambiente e workflows de CI.

---

## 🛠️ Solução de problemas

- [Problemas comuns](./troubleshooting/common-issues.md) — problemas de compilação, renderização, fontes, arquivos e testes, indexados por sintoma.

---

## 🤝 Como contribuir

- [Guia de contribuição](contribuindo.md) — ambiente de desenvolvimento, verificações, testes e goldens, e o processo de lançamento.
- [Código de Conduta](codigo-de-conduta.md) e [Política de Segurança](seguranca.md).
