🌐 [Português](../../pt-br/architecture/animation.md) | **English** | 🏠 [Index](../index.md)

---

# Animation

How OITO2 widgets animate, and how an app animates its own state with the same motion. Code: `crates/oito2_iced/src/animation.rs`, plus each animated widget.

## Material motion on Iced's `Animation`

Iced 0.14 has `iced::animation::Animation<T>`, built on the `lilt` crate, with an `Easing::Custom(fn(f32) -> f32)` variant. `oito2_iced::animation` re-exports `Animation` and `Easing` and adds the Material motion of oito2_ui:

| Item | Value |
|---|---|
| `EMPHASIZED` | The Material emphasized curve: two cubic segments joined at (0.166666, 0.4), a fast start and a long deceleration |
| `EASE_OUT_CUBIC`, `EASE_IN_OUT_CUBIC`, `EASE_IN_OUT` | The other curves the widgets use; `ease_out` and `ease_out_back` are available as functions |
| `duration::SHORT`, `MEDIUM`, `LONG`, `EXTRA_LONG` | 100, 200, 300 and 500 ms |
| `motion_duration(duration, reduce_motion)` | `duration`, or zero when reduced motion is asked for |
| `emphasized_animation(state, duration)` | An `Animation` with the emphasized curve |

The curves are tested at their ends, at the midpoint and for monotonicity.

## Inside the widgets

Animated widgets keep their progress in their tree state and ask Iced for a redraw only while something moves; they read the time from the redraw events, so an app needs **no subscription** for them. A widget drawn for the first time shows its final state, so pages don't animate on load.

| Widget | Motion |
|---|---|
| Checkbox, radio, switch | `TOGGLE_DURATION` on every change |
| Segment group, tab bar | The selection marker slides (tab bar: 300 ms) |
| Expandable | The chevron turns and the content height eases in and out over 200 ms |
| Scroll area undershoot | Edge shadows fade in and out over 150 ms |
| Toasts, sheets | Slide in on entry; toasts also slide out |
| Rating | Fill over 700 ms |
| Carousel, page indicator | Page slide and dot change |
| Swap | Fade, scale, rotate or flip between two children |
| Marquee | Scrolls at 40 px/s, paused on hover and focus |
| Tour | The spotlight moves over 250 ms with an ease-out cubic curve |
| Animated icons | Ten icons drawn on a canvas, played once, repeated or still |
| Progress indicators | Indeterminate linear and circular cycles |

## Reduced motion

There is no global reduced-motion switch: Iced 0.14 doesn't report the system preference. Widgets with motion that matters take `.animated(false)` (carousel, expandable, marquee, page indicator, progress, rating, scroll area, swap, tour, tree view and the animated icons), which shows the final state at once. For its own animations, an app passes its preference to `motion_duration`.

## Animating app state

For state the app owns, keep an `Animation` in the model and subscribe to `iced::window::frames()` only while it runs:

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

## Limits of Iced 0.14

Iced 0.14 can't draw a subtree with opacity or rotation. Where oito2_ui fades or rotates, oito2_iced approximates: toasts slide without a fade, dialogs and sheets disappear without an exit animation, and the swap effects show the more visible child at each moment or scale and clip it.
