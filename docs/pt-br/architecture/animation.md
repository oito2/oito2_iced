🌐 [English](../../en/architecture/animation.md) | **Português** | 🏠 [Índice](../index.md)

---

# Animação

Como os widgets do OITO2 se animam e como um app anima o próprio estado com o mesmo movimento. Código: `crates/oito2_iced/src/animation.rs`, além de cada widget animado.

## Movimento do Material sobre a `Animation` do Iced

O Iced 0.14 tem `iced::animation::Animation<T>`, construída sobre o crate `lilt`, com uma variante `Easing::Custom(fn(f32) -> f32)`. O `oito2_iced::animation` reexporta `Animation` e `Easing` e acrescenta o movimento do Material do oito2_ui:

| Item | Valor |
|---|---|
| `EMPHASIZED` | A curva enfatizada do Material: dois segmentos cúbicos unidos em (0.166666, 0.4), um início rápido e uma longa desaceleração |
| `EASE_OUT_CUBIC`, `EASE_IN_OUT_CUBIC`, `EASE_IN_OUT` | As outras curvas que os widgets usam; `ease_out` e `ease_out_back` estão disponíveis como funções |
| `duration::SHORT`, `MEDIUM`, `LONG`, `EXTRA_LONG` | 100, 200, 300 e 500 ms |
| `motion_duration(duration, reduce_motion)` | `duration`, ou zero quando se pede movimento reduzido |
| `emphasized_animation(state, duration)` | Uma `Animation` com a curva enfatizada |

As curvas são testadas nas pontas, no ponto médio e quanto à monotonicidade.

## Dentro dos widgets

Os widgets animados guardam o progresso no estado da sua árvore e pedem ao Iced um redesenho só enquanto algo se move; eles leem o tempo dos eventos de redesenho, então um app **não precisa de subscription** para eles. Um widget desenhado pela primeira vez mostra seu estado final, então as páginas não se animam ao carregar.

| Widget | Movimento |
|---|---|
| Checkbox, radio, switch | `TOGGLE_DURATION` a cada mudança |
| Grupo de segmentos, barra de abas | O marcador da seleção desliza (barra de abas: 300 ms) |
| Expansível | A seta gira e a altura do conteúdo entra e sai suavemente em 200 ms |
| Undershoot da área de rolagem | Sombras nas bordas aparecem e somem em 150 ms |
| Toasts, sheets | Deslizam ao entrar; os toasts também deslizam ao sair |
| Avaliação | Preenchimento em 700 ms |
| Carrossel, indicador de página | Deslize da página e troca do ponto |
| Swap | Esmaecer, escalar, girar ou virar entre dois filhos |
| Marquee | Rola a 40 px/s, pausado com o mouse em cima e com o foco |
| Tour | O holofote se move em 250 ms com uma curva ease-out cúbica |
| Ícones animados | Dez ícones desenhados em um canvas, tocados uma vez, repetidos ou parados |
| Indicadores de progresso | Ciclos indeterminados lineares e circulares |

## Movimento reduzido

Não há uma chave global de movimento reduzido: o Iced 0.14 não informa a preferência do sistema. Os widgets com movimento relevante aceitam `.animated(false)` (carrossel, expansível, marquee, indicador de página, progresso, avaliação, área de rolagem, swap, tour, tree view e os ícones animados), que mostra o estado final de imediato. Para as próprias animações, um app passa sua preferência para `motion_duration`.

## Animando o estado do app

Para o estado que pertence ao app, mantenha uma `Animation` no modelo e assine `iced::window::frames()` só enquanto ela roda:

```rust
use iced::time::Instant;
use iced::{Subscription, window};
use oito2_iced::animation::{self, Animation};

struct App {
    open: Animation<bool>,
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Frame(Instant),
}

impl App {
    fn new() -> Self {
        Self {
            open: animation::emphasized_animation(false, animation::duration::MEDIUM),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Toggle => {
                let next = !self.open.value();
                self.open.go_mut(next, Instant::now());
            }
            Message::Frame(_) => {}
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.open.is_animating(Instant::now()) {
            window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        }
    }

    fn height(&self) -> f32 {
        self.open.interpolate(0.0, 120.0, Instant::now())
    }
}
```

## Limites do Iced 0.14

O Iced 0.14 não consegue desenhar uma subárvore com opacidade ou rotação. Onde o oito2_ui esmaece ou gira, o oito2_iced aproxima: os toasts deslizam sem esmaecer, diálogos e sheets somem sem animação de saída, e os efeitos do swap mostram o filho mais visível a cada momento ou o escalam e recortam.
