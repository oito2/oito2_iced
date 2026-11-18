# Animations

oito2_iced components animate their changes, such as a sliding marker, an opening section or a toast, with the Material curves and durations of oito2_ui, and keep every motion short so the interface stays responsive. They ask Iced for a frame only while something moves, so an app needs no subscription for them.

## Reduced motion

Iced 0.14 doesn't report the system preference for reduced motion, so there is no global switch. Components whose motion matters take `.animated(false)`, which shows the final state at once: the carousel, the expandable, the marquee, the page indicator, the progress indicators, the rating, the scroll area, the swap, the tour, the tree view and the animated icons.

```rust
use oito2_iced::widget::expandable::expandable;

let details = expandable("Details", content, self.expanded)
    .on_toggle(Message::Expanded)
    .animated(!self.reduce_motion);
```

> Use the same preference in your own animations: `animation::motion_duration(duration, reduce_motion)` returns zero when reduced motion is on, so the animation jumps to its end.

## Swapping content

`swap` switches between two elements with a fade, scale, rotation or flip, such as a play and a pause icon. It shows `on` while the value is true and `off` otherwise.

```rust
use oito2_iced::icon::{self, lucide};
use oito2_iced::widget::swap::{self, SwapEffect};

let play = swap::swap(self.playing, icon::icon(lucide::PLAY), icon::icon(lucide::PAUSE))
    .effect(SwapEffect::Rotate);
```

## Animated icons

`icon::animated::icon` plays a drawn icon once, in a loop, or holds it still.

```rust
use oito2_iced::icon::animated::{self, Mode};

let heart = animated::icon(animated::HEART).mode(Mode::Repeat);
```

## Your own animations

`oito2_iced::animation` adds the Material curves (`EMPHASIZED`, `EASE_OUT_CUBIC`, `EASE_IN_OUT_CUBIC`, `EASE_IN_OUT`) and durations (`duration::SHORT` to `EXTRA_LONG`) to Iced's `Animation`. Keep the animation in the model and subscribe to `window::frames()` only while it runs:

```rust
use iced::time::Instant;
use iced::{Subscription, window};
use oito2_iced::animation::{self, Animation};

struct Panel {
    open: Animation<bool>,
}

impl Panel {
    fn new() -> Self {
        Self {
            open: animation::emphasized_animation(false, animation::duration::MEDIUM),
        }
    }

    fn toggle(&mut self) {
        let next = !self.open.value();
        self.open.go_mut(next, Instant::now());
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.open.is_animating(Instant::now()) {
            window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        }
    }

    fn height(&self) -> f32 {
        self.open.interpolate(0.0, 240.0, Instant::now())
    }
}
```

## Page transitions

The master-detail page and the navigation page switch their content at once. Iced 0.14 can't draw a subtree with opacity or rotation, so fades are approximated: toasts slide in and out, and dialogs and sheets disappear without an exit animation.
