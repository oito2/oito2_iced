🌐 [English](../../en/concepts/glossary.md) | **Português** | 🏠 [Índice](../index.md)

---

# Glossário

| Termo | Significado |
|---|---|
| **Accent (cor de destaque)** | A cor semente dos papéis primários de um tema: um dos nove valores de `tokens::Accent` (azul, verde-azulado, verde, amarelo, laranja, vermelho, rosa, roxo, ardósia) ou qualquer cor, como a cor de destaque do desktop. |
| **Anchored panel (painel ancorado)** | O overlay de `overlay::anchored`: um painel posicionado junto a um elemento âncora, invertido e limitado para ficar dentro da janela, fechado com Escape ou com um clique fora. |
| **Catalog (catálogo)** | O trait do Iced que um tema implementa para estilizar um widget (`button::Catalog`, `container::Catalog`…). O `oito2_iced::Theme` implementa o catálogo de todo widget nativo do Iced. |
| **Chrome colors (cores do chrome)** | As superfícies da janela de um tema: fundo, barra lateral, área principal, barra de título e diálogo (`ColorRoles::chrome`). |
| **Class (classe)** | A variante de estilo que um widget pede ao seu catálogo, como `button::Class::Tonal`. As classes do OITO2 são enums com uma variante `Custom` para estilos avulsos. |
| **Color roles (papéis de cor)** | Todas as cores que um tema resolve (`ColorRoles`): os papéis do Material 3 (primary, surface containers, outline…) mais as cores de chrome, semânticas e de sintaxe. |
| **Contrast (contraste)** | `Contrast::Standard` ou `Contrast::High`; o alto contraste reforça bordas e textos em relação às suas superfícies. |
| **Controller (controlador)** | Uma struct simples com o estado de um widget mais rico, guardada no modelo do app: `SelectionController`, `TreeState`, `Tour`, `Toasts`, `upload::Controller`, `Crop`, `Signature`… |
| **CSD** | Decorações no lado do cliente (client-side decorations): o app desenha a própria barra de título e os controles da janela em uma janela sem decorações. |
| **Density (densidade)** | Tamanhos de ponteiro (controles de 34 pixels) ou de toque (controles de 40 pixels), definidos por thread com `density::set`. |
| **Desktop environment (ambiente desktop)** | O desktop Linux em que o app roda (GNOME, KDE, Xfce, COSMIC, Cinnamon, MATE, Budgie, LXQt, Pantheon, Unity), detectado pelo `oito2_foundation`. |
| **Focus ring (anel de foco)** | O contorno desenhado em volta do elemento que tem o foco de teclado, mostrado enquanto o teclado está em uso. |
| **Focusable** | O wrapper `focus::focusable`, que dá foco de teclado, anel de foco e ativação por Enter/Espaço a qualquer elemento. |
| **Golden** | Uma imagem de referência (`tests/goldens/<name>-tiny-skia.png`) com a qual um teste compara um snapshot renderizado, dentro de uma tolerância de 0,5 %. |
| **Header bar (barra de cabeçalho)** | Uma barra de título com áreas de início e de fim e um campo de busca opcional, montada com `window::header_bar`. |
| **Host** | Um widget que envolve a raiz da view para desenhar uma superfície sobre a janela inteira: `toast::toaster`, `dialog::host`, `sheet::host`, `tour::host`. |
| **oito2_ui** | O design system em Flutter, fonte de verdade de tokens, cores, strings e comportamento. |
| **oito2_uikit** | O design system em Svelte 5 + Tauri v2 com a mesma identidade. |
| **oito2_foundation, oito2_settings** | Os crates do repositório `oito2_rs` com as regras de plataforma e os leitores de configurações do desktop compartilhados pelas interfaces Rust do OITO2. |
| **Parity fixtures (fixtures de paridade)** | Arquivos JSON exportados pelo oito2_ui (tokens, esquemas, contraste, strings, formatos, cores de sintaxe, matrizes de QR) que os testes comparam com a implementação em Rust. |
| **Preset** | Um tema de cores nomeado do `oito2_iced_themes`: Everforest (claro, escuro), Rosé Pine (principal, Moon, Dawn), Nord (escuro, claro), Dracula, Alucard, Catppuccin (Latte, Frappé, Macchiato, Mocha). |
| **Roving focus (foco itinerante)** | Um grupo que é uma única parada de Tab, cujos itens movem o foco com as setas (barras de abas, grupos de segmentos, grupos de alternância, trilhos, caixas de lista). |
| **Semantic colors (cores semânticas)** | Tons de status e de destaque seguros para texto: texto primário, texto de erro, sucesso, aviso, informação e link (`ColorRoles::semantic`). |
| **Simulator (simulador)** | O runtime headless do Iced, do `iced_test`, que os testes de comportamento conduzem com cliques e teclas. |
| **tiny-skia** | O renderizador por software do Iced, usado pelos testes e disponível para os apps com `ICED_BACKEND=tiny-skia`. |
| **Tokens** | Os números compartilhados do design: espaçamento, raios, tamanhos e a escala tipográfica (`oito2_iced::tokens`). |
| **Window control style (estilo dos controles da janela)** | A família visual dos botões da janela: GNOME, Breeze, Xfwm, COSMIC, Windows, macOS ou nenhum. |
| **wgpu** | O renderizador por GPU do Iced (Vulkan, Metal, DirectX 12, OpenGL), o padrão quando disponível. |
