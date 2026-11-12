// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! F0 prototype of the OITO2 Iced showcase: an undecorated window with the
//! OITO2 title bar and resize handles, focusable buttons (Tab, Shift+Tab,
//! Enter), a popover, an emphasized animation and the Inter weights.
//!
//! Set `ICED_BACKEND=tiny-skia` to force the software renderer.

use std::time::Instant;

use iced::font::Weight;
use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced::window;
use iced::{Element, Length, Subscription, Task};

use oito2_iced::animation::{self, Animation};
use oito2_iced::theme::Brightness;
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::{focusable, popover};
use oito2_iced::{Theme, focus, font};

fn main() -> iced::Result {
    iced::application(Showcase::new, Showcase::update, Showcase::view)
        .title("OITO2 Iced")
        .theme(Showcase::theme)
        .subscription(Showcase::subscription)
        .font(font::INTER_VARIABLE)
        .default_font(font::INTER)
        .window(window::Settings {
            size: iced::Size::new(960.0, 680.0),
            min_size: Some(iced::Size::new(480.0, 360.0)),
            decorations: false,
            platform_specific: window::settings::PlatformSpecific {
                application_id: "io.github.oito2.oito2_iced.showcase".into(),
                ..Default::default()
            },
            ..window::Settings::default()
        })
        .run()
}

struct Showcase {
    brightness: Brightness,
    accent: Accent,
    presses: u32,
    popover_open: bool,
    checked: bool,
    name: String,
    expanded: Animation<bool>,
    now: Instant,
}

#[derive(Debug, Clone)]
enum Message {
    Window(oito2_iced::window::Action),
    Press,
    ToggleBrightness,
    NextAccent,
    TogglePopover,
    DismissPopover,
    Check(bool),
    Name(String),
    Animate,
    Frame(Instant),
}

impl Showcase {
    fn new() -> Self {
        Self {
            brightness: Brightness::Light,
            accent: Accent::Blue,
            presses: 0,
            popover_open: false,
            checked: false,
            name: String::new(),
            expanded: animation::emphasized_animation(false, animation::duration::EXTRA_LONG),
            now: Instant::now(),
        }
    }

    fn theme(&self) -> Theme {
        Theme::new(self.brightness, self.accent)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(action) => return oito2_iced::window::perform_latest(action),
            Message::Press => self.presses += 1,
            Message::ToggleBrightness => {
                self.brightness = match self.brightness {
                    Brightness::Light => Brightness::Dark,
                    Brightness::Dark => Brightness::Light,
                };
            }
            Message::NextAccent => {
                let index = Accent::ALL
                    .iter()
                    .position(|accent| *accent == self.accent)
                    .unwrap_or_default();
                self.accent = Accent::ALL[(index + 1) % Accent::ALL.len()];
            }
            Message::TogglePopover => self.popover_open = !self.popover_open,
            Message::DismissPopover => self.popover_open = false,
            Message::Check(checked) => self.checked = checked,
            Message::Name(name) => self.name = name,
            Message::Animate => {
                self.now = Instant::now();
                let target = !self.expanded.value();
                self.expanded.go_mut(target, self.now);
            }
            Message::Frame(now) => self.now = now,
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.expanded.is_animating(self.now) {
            window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        let header =
            oito2_iced::window::title_bar("OITO2 Iced \u{2014} F0", text(""), Message::Window);

        let action = |label: &'static str, class: ButtonClass<'static>, message: Message| {
            focusable(button(label).class(class).on_press(message.clone())).on_activate(message)
        };

        let buttons = row![
            action("Filled", ButtonClass::Filled, Message::Press),
            action("Tonal", ButtonClass::Tonal, Message::Press),
            action("Outlined", ButtonClass::Outlined, Message::Press),
            action("Text", ButtonClass::Text, Message::Press),
            action("Danger", ButtonClass::Danger, Message::Press),
            focusable(button("Disabled")),
        ]
        .spacing(spacing::SM);

        let theme_controls = row![
            action(
                "Light / dark",
                ButtonClass::Outlined,
                Message::ToggleBrightness
            ),
            action("Next accent", ButtonClass::Outlined, Message::NextAccent),
            popover(
                action("Popover", ButtonClass::Tonal, Message::TogglePopover),
                column![
                    text("Anchored panel").font(font::inter(Weight::Semibold)),
                    text("Click outside or press Escape to close.").class(TextClass::Variant),
                ]
                .spacing(spacing::XS)
                .width(240),
            )
            .open(self.popover_open)
            .on_dismiss(Message::DismissPopover),
        ]
        .spacing(spacing::SM);

        let inputs = row![
            focusable(
                checkbox(self.checked)
                    .label("Checkbox")
                    .on_toggle(Message::Check)
            )
            .on_activate(Message::Check(!self.checked))
            .radius(oito2_iced::tokens::radius::CHECK),
            text_input("Text input", &self.name)
                .on_input(Message::Name)
                .padding([7.0, spacing::MD])
                .width(240),
        ]
        .spacing(spacing::LG)
        .align_y(iced::Alignment::Center);

        let weights = column(
            [
                ("Regular 400", Weight::Normal),
                ("Medium 500", Weight::Medium),
                ("SemiBold 600", Weight::Semibold),
                ("Bold 700", Weight::Bold),
            ]
            .into_iter()
            .map(|(label, weight)| {
                text(format!("Inter Variable \u{2014} {label}"))
                    .font(font::inter(weight))
                    .size(18)
                    .into()
            }),
        )
        .spacing(spacing::XS);

        let progress = self.expanded.interpolate(0.05_f32, 1.0_f32, self.now);

        let animation = column![
            action("Animate (emphasized)", ButtonClass::Tonal, Message::Animate),
            container(
                container(text(""))
                    .class(ContainerClass::Custom(Box::new(|theme: &Theme| {
                        container::Style::default().background(theme.roles().primary)
                    })))
                    .width(Length::FillPortion((progress * 1000.0) as u16))
                    .height(12),
            )
            .width(400),
        ]
        .spacing(spacing::SM);

        let card = |title: &'static str, content: Element<'static, Message, Theme>| {
            container(
                column![text(title).font(font::inter(Weight::Semibold)), content]
                    .spacing(spacing::MD),
            )
            .class(ContainerClass::Card)
            .padding(spacing::LG)
            .width(Length::Fill)
        };

        let page = column![
            text(format!(
                "Presses: {} \u{2014} Tab / Shift+Tab move the focus, Enter or Space activate.",
                self.presses
            ))
            .class(TextClass::Variant),
            card("Buttons", buttons.into()),
            card("Theme and overlay", theme_controls.into()),
            card("Inputs", inputs.into()),
            card("Font weights", weights.into()),
            card("Animation", animation.into()),
        ]
        .spacing(spacing::LG)
        .padding(spacing::PAGE);

        let window = column![header, page];

        oito2_iced::window::resize_handles(focus::navigation(window), Message::Window)
    }
}
