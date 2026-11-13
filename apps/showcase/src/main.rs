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

//! Showcase of the OITO2 design system for Iced: an undecorated window with
//! the window chrome of the desktop (or of a simulated one), a theme that
//! follows the desktop settings while the app runs, and a settings dialog
//! for the theme, preset, contrast, density, language and desktop.
//!
//! Set `ICED_BACKEND=tiny-skia` to force the software renderer.

mod appearance;
mod pages;

use std::time::Instant;

use iced::font::Weight;
use iced::widget::{button, column, container, opaque, pick_list, row, scrollable, stack, text};
use iced::{Element, Length, Subscription, Task, window};

use appearance::{Appearance, ContrastChoice, DensityChoice, Desktop, Family, Language, Mode};
use oito2_iced::animation::{self, Animation};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::{focusable, popover};
use oito2_iced::window::{Labels, Placement, WindowState};
use oito2_iced::{Theme, focus, font};
use oito2_iced_settings::SystemSettings;
use pages::{Gallery, Page};

fn main() -> iced::Result {
    let application = iced::application(Showcase::new, Showcase::update, Showcase::view)
        .title("OITO2 Iced")
        .theme(Showcase::theme)
        .subscription(Showcase::subscription);

    font::INTER_FONTS
        .into_iter()
        .fold(application, |application, bytes| application.font(bytes))
        .default_font(font::INTER)
        .window(window::Settings {
            size: iced::Size::new(960.0, 680.0),
            min_size: Some(iced::Size::new(480.0, 360.0)),
            decorations: false,
            platform_specific: platform_specific(),
            ..window::Settings::default()
        })
        .run()
}

/// Platform window settings; on Linux they set the application id that
/// Wayland compositors and X11 window managers use to group the window.
#[cfg(target_os = "linux")]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        application_id: "io.github.oito2.oito2_iced.showcase".into(),
        ..Default::default()
    }
}

/// Platform window settings; the defaults on platforms other than Linux.
#[cfg(not(target_os = "linux"))]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
}

struct Showcase {
    settings: SystemSettings,
    appearance: Appearance,
    window: WindowState,
    settings_open: bool,
    page: Page,
    gallery: Gallery,
    popover_open: bool,
    expanded: Animation<bool>,
    now: Instant,
}

#[derive(Debug, Clone)]
enum Message {
    Window(oito2_iced::window::Action),
    WindowEvent(window::Id, window::Event),
    Placement(Placement),
    System(SystemSettings),
    OpenSettings,
    CloseSettings,
    Mode(Mode),
    Family(Family),
    NextAccent,
    Contrast(ContrastChoice),
    Density(DensityChoice),
    Language(Language),
    Desktop(Desktop),
    Navigate(Page),
    Gallery(pages::Message),
    TogglePopover,
    DismissPopover,
    Animate,
    Frame(Instant),
}

impl Showcase {
    fn new() -> Self {
        Self {
            settings: oito2_iced_settings::read(),
            appearance: Appearance::default(),
            window: WindowState::default(),
            settings_open: false,
            page: Page::default(),
            gallery: Gallery::default(),
            popover_open: false,
            expanded: animation::emphasized_animation(false, animation::duration::EXTRA_LONG),
            now: Instant::now(),
        }
    }

    fn theme(&self) -> Theme {
        self.appearance.theme(&self.settings)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Window(action) => return oito2_iced::window::perform_latest(action),
            Message::WindowEvent(id, event) => {
                if self.window.update(&event) {
                    return WindowState::query(id).map(Message::Placement);
                }
            }
            Message::Placement(placement) => self.window.apply(placement),
            Message::System(settings) => self.settings = settings,
            Message::OpenSettings => self.settings_open = true,
            Message::CloseSettings => self.settings_open = false,
            Message::Mode(mode) => self.appearance.mode = mode,
            Message::Family(family) => self.appearance.family = family,
            Message::NextAccent => {
                let next = match self.appearance.accent {
                    None => 0,
                    Some(accent) => {
                        Accent::ALL
                            .iter()
                            .position(|candidate| *candidate == accent)
                            .unwrap_or_default()
                            + 1
                    }
                };
                self.appearance.accent = Accent::ALL.get(next).copied();
            }
            Message::Contrast(contrast) => self.appearance.contrast = contrast,
            Message::Density(density) => self.appearance.density = density,
            Message::Language(language) => self.appearance.language = language,
            Message::Desktop(desktop) => self.appearance.desktop = desktop,
            Message::Navigate(page) => self.page = page,
            Message::Gallery(message) => self.gallery.update(message),
            Message::TogglePopover => self.popover_open = !self.popover_open,
            Message::DismissPopover => self.popover_open = false,
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
        let frames = if self.expanded.is_animating(self.now) {
            window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        };

        Subscription::batch([
            oito2_iced_settings::subscription().map(Message::System),
            window::events().map(|(id, event)| Message::WindowEvent(id, event)),
            frames,
        ])
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        oito2_iced::density::set(self.appearance.density(&self.settings));

        let localizations = self.appearance.language.localizations();
        let (style, layout) = self.appearance.chrome(&self.settings);

        let header = oito2_iced::window::header_bar(Message::Window)
            .title("OITO2 Iced")
            .start(
                focusable(
                    button("Settings")
                        .class(ButtonClass::Chrome)
                        .on_press(Message::OpenSettings),
                )
                .on_activate(Message::OpenSettings),
            )
            .control_style(style)
            .button_layout(layout)
            .state(self.window)
            .labels(Labels::new(&localizations));

        let action = |label: &'static str, class: ButtonClass<'static>, message: Message| {
            focusable(button(label).class(class).on_press(message.clone())).on_activate(message)
        };

        let theme_controls = row![
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

        let weights = column(
            [
                ("Regular 400", Weight::Normal),
                ("Medium 500", Weight::Medium),
                ("SemiBold 600", Weight::Semibold),
                ("Bold 700", Weight::Bold),
            ]
            .into_iter()
            .map(|(label, weight)| {
                text(format!("Inter \u{2014} {label}"))
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

        let overview = column![
            text("Tab / Shift+Tab move the focus, Enter or Space activate.")
                .class(TextClass::Variant),
            card("Theme and overlay", theme_controls.into()),
            card("Font weights", weights.into()),
            card("Animation", animation.into()),
        ]
        .spacing(spacing::LG);

        let page: Element<'_, Message, Theme> = match self.gallery.view(self.page, &localizations) {
            Some(page) => page.map(Message::Gallery),
            None => overview.into(),
        };

        let sidebar = container(
            column(Page::ALL.into_iter().map(|page| {
                let item = oito2_iced::widget::button::text(page.to_string())
                    .width(Length::Fill)
                    .on_press(Message::Navigate(page));

                if page == self.page {
                    item.class(ButtonClass::Tonal).into()
                } else {
                    item.into()
                }
            }))
            .spacing(spacing::XS)
            .padding(spacing::SM),
        )
        .class(ContainerClass::Sidebar)
        .width(Length::Fixed(200.0))
        .height(Length::Fill);

        let body = row![
            sidebar,
            scrollable(container(page).padding(spacing::PAGE).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::Fill),
        ];

        let window = column![header, body];

        let content: Element<'_, Message, Theme> = if self.settings_open {
            stack![window, self.settings_dialog()].into()
        } else {
            window.into()
        };

        oito2_iced::window::resize_handles(focus::navigation(content), Message::Window)
    }

    /// The settings dialog over a scrim.
    fn settings_dialog(&self) -> Element<'_, Message, Theme> {
        let appearance = &self.appearance;

        let field = |label: &'static str, control: Element<'static, Message, Theme>| {
            row![
                text(label)
                    .width(Length::Fixed(140.0))
                    .class(TextClass::Variant),
                control
            ]
            .spacing(spacing::MD)
            .align_y(iced::Alignment::Center)
        };

        let dialog = container(
            column![
                text("Settings")
                    .font(font::inter(Weight::Semibold))
                    .size(oito2_iced::tokens::typography::TITLE_LARGE.size),
                field(
                    "Theme",
                    pick_list(Mode::ALL, Some(appearance.mode), Message::Mode).into()
                ),
                field(
                    "Color theme",
                    pick_list(Family::all(), Some(appearance.family), Message::Family).into()
                ),
                field(
                    "Contrast",
                    pick_list(
                        ContrastChoice::ALL,
                        Some(appearance.contrast),
                        Message::Contrast
                    )
                    .into()
                ),
                field(
                    "Density",
                    pick_list(
                        DensityChoice::ALL,
                        Some(appearance.density),
                        Message::Density
                    )
                    .into()
                ),
                field(
                    "Language",
                    pick_list(Language::ALL, Some(appearance.language), Message::Language).into()
                ),
                field(
                    "Desktop",
                    pick_list(Desktop::all(), Some(appearance.desktop), Message::Desktop).into()
                ),
                row![
                    iced::widget::space::horizontal(),
                    oito2_iced::widget::button::filled("Close").on_press(Message::CloseSettings),
                ],
            ]
            .spacing(spacing::MD)
            .width(440),
        )
        .class(ContainerClass::Dialog)
        .padding(spacing::XL);

        opaque(
            container(dialog)
                .center(Length::Fill)
                .class(ContainerClass::Custom(Box::new(|theme: &Theme| {
                    container::Style::default().background(theme.roles().scrim.scale_alpha(0.32))
                }))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::simulator::Simulator;
    use oito2_iced::foundation::{DesktopEnvironment, OperatingSystem, WindowControlStyle};
    use oito2_iced_settings::Platform;
    use oito2_iced_themes::Preset;

    /// The showcase on GNOME with no system preferences.
    fn showcase() -> Showcase {
        Showcase {
            settings: SystemSettings::fallback(
                Platform::new(OperatingSystem::Linux, false),
                Some(DesktopEnvironment::Gnome),
            ),
            ..Showcase::new()
        }
    }

    /// Runs the messages the simulated interactions published.
    fn run(app: &mut Showcase, messages: impl IntoIterator<Item = Message>) {
        for message in messages {
            let _ = app.update(message);
        }
    }

    #[test]
    fn the_settings_button_opens_the_dialog() {
        let mut app = showcase();
        let messages: Vec<Message> = {
            let mut ui = Simulator::with_settings(oito2_iced_test_utils::settings(), app.view());
            ui.click("Settings").expect("settings button");
            ui.into_messages().collect()
        };

        run(&mut app, messages);

        let mut ui = Simulator::with_settings(oito2_iced_test_utils::settings(), app.view());
        assert!(ui.find("Color theme").is_ok());
        assert!(ui.find("Desktop").is_ok());
    }

    #[test]
    fn the_appearance_drives_the_theme_and_the_chrome() {
        let mut app = showcase();

        assert_eq!(
            app.theme().brightness(),
            oito2_iced::theme::Brightness::Light
        );

        run(
            &mut app,
            [
                Message::Family(Family(Some(Preset::CatppuccinMocha))),
                Message::Contrast(ContrastChoice::High),
                Message::Desktop(Desktop(Some(WindowControlStyle::Macos))),
                Message::Language(Language::Portuguese),
            ],
        );

        let theme = app.theme();
        assert_eq!(
            theme.roles(),
            Preset::CatppuccinMocha
                .theme(oito2_iced::theme::Contrast::High)
                .roles()
        );

        let (style, layout) = app.appearance.chrome(&app.settings);
        assert_eq!(style, WindowControlStyle::Macos);
        assert!(layout.trailing.is_empty());

        let mut ui = Simulator::with_settings(oito2_iced_test_utils::settings(), app.view());
        assert!(ui.find("Fechar").is_ok(), "Portuguese window labels");
    }

    #[test]
    fn system_changes_reach_the_theme() {
        let mut app = showcase();
        let dark = SystemSettings {
            color_scheme: Some(oito2_iced_settings::ColorScheme::Dark),
            accent_color: Some("#3A944A".to_owned()),
            ..app.settings.clone()
        };

        run(&mut app, [Message::System(dark)]);

        assert_eq!(
            app.theme().roles(),
            Theme::base(
                oito2_iced::theme::Brightness::Dark,
                Accent::Green.color(),
                oito2_iced::theme::Contrast::Standard
            )
            .roles()
        );
    }

    /// A window tall enough to show every card of a page without scrolling.
    const TALL: (f32, f32) = (1280.0, 3200.0);

    /// The showcase on `page`.
    fn on_page(page: Page) -> Showcase {
        let mut app = showcase();
        run(&mut app, [Message::Navigate(page)]);
        app
    }

    /// The bounds of every focusable element of the view, whether each one is
    /// a text input, and whether it is fully visible (not clipped by a
    /// scrollable).
    fn focusables(ui: &mut Simulator<'_, Message, Theme>) -> Vec<(iced::Rectangle, bool, bool)> {
        use iced_test::selector::Candidate;

        let mut found = Vec::new();
        let mut text_inputs = Vec::new();
        let _ = ui.find(|candidate: Candidate<'_>| -> Option<()> {
            match candidate {
                Candidate::Focusable {
                    bounds,
                    visible_bounds,
                    ..
                } => found.push((bounds, visible_bounds == Some(bounds))),
                Candidate::TextInput { bounds, .. } => text_inputs.push(bounds),
                _ => {}
            }
            None
        });

        found
            .into_iter()
            .map(|(bounds, visible)| (bounds, text_inputs.contains(&bounds), visible))
            .collect()
    }

    #[test]
    fn every_page_renders_wide_light_and_compact_dark_portuguese() {
        for page in Page::ALL {
            let mut app = on_page(page);

            for (mode, language, size) in [
                (Mode::Light, Language::English, (1280.0, 800.0)),
                (Mode::Dark, Language::Portuguese, (560.0, 720.0)),
            ] {
                run(&mut app, [Message::Mode(mode), Message::Language(language)]);

                let theme = app.theme();
                let mut ui =
                    Simulator::with_size(oito2_iced_test_utils::settings(), size, app.view());

                assert!(ui.snapshot(&theme).is_ok(), "{page} {mode:?}");
            }
        }
    }

    #[test]
    fn tab_reaches_every_control_of_every_page() {
        for page in Page::ALL {
            let app = on_page(page);
            let mut ui = Simulator::with_size(oito2_iced_test_utils::settings(), TALL, app.view());
            let expected = focusables(&mut ui);

            let mut reached = Vec::new();
            for _ in 0..expected.len() {
                ui.tap_key(iced::keyboard::key::Named::Tab);

                if let Ok(target) = ui.find(iced_test::selector::is_focused()) {
                    reached.push(target.bounds());
                }
            }

            // A scrolled chip bar moves its chips, so the stops are counted
            // rather than matched by position.
            let mut distinct = reached.clone();
            distinct.dedup();

            assert!(!expected.is_empty(), "{page}");
            assert_eq!(reached.len(), expected.len(), "{page}: a Tab left no focus");
            assert_eq!(
                distinct.len(),
                expected.len(),
                "{page}: Tab visited {reached:?}, expected {expected:?}"
            );
        }
    }

    #[test]
    fn every_clickable_control_shows_the_hand_cursor() {
        for page in Page::ALL {
            let app = on_page(page);
            let mut ui = Simulator::with_size(oito2_iced_test_utils::settings(), TALL, app.view());

            for (bounds, text_input, _) in focusables(&mut ui)
                .into_iter()
                .filter(|(_, _, visible)| *visible)
            {
                let interaction =
                    oito2_iced_test_utils::mouse_interaction(app.view(), TALL, bounds.center());

                if text_input {
                    assert_eq!(interaction, iced::mouse::Interaction::Text, "{page}");
                } else {
                    assert!(
                        matches!(
                            interaction,
                            // An editable label shows the text cursor, as text
                            // it can be edited in place; a divider handle shows
                            // the resize cursor of its axis.
                            iced::mouse::Interaction::Pointer
                                | iced::mouse::Interaction::Grab
                                | iced::mouse::Interaction::Text
                                | iced::mouse::Interaction::ResizingHorizontally
                                | iced::mouse::Interaction::ResizingVertically
                        ),
                        "{page}: {bounds:?} shows {interaction:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn showcase_golden() {
        let mut app = showcase();
        run(&mut app, [Message::OpenSettings]);

        for (mode, name) in [
            (Mode::Light, "showcase_settings_light"),
            (Mode::Dark, "showcase_settings_dark"),
        ] {
            run(&mut app, [Message::Mode(mode)]);

            let theme = app.theme();
            let mut ui = Simulator::with_size(
                oito2_iced_test_utils::settings(),
                (960.0, 680.0),
                app.view(),
            );
            let snapshot = ui.snapshot(&theme).expect("snapshot");

            oito2_iced_test_utils::golden!(&snapshot, name);
        }
    }
}
