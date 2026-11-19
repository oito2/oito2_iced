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
//! for the theme, preset, contrast, density, language and desktop. The
//! sidebar lists every page of the catalog by group, filtered by a search
//! field; each page documents a component with examples, their source and
//! the API reference.
//!
//! Set `ICED_BACKEND=tiny-skia` to force the software renderer.

mod api;
mod appearance;
mod catalog;
mod doc;
mod pages;

use iced::widget::{Column, button, column, container, pick_list, row, scrollable, text};
use iced::{Element, Length, Subscription, Task, keyboard, window};

use appearance::{Appearance, ContrastChoice, DensityChoice, Desktop, Family, Language, Mode};
use catalog::{Action, Group, PageId, Pages, Shell};
use oito2_iced::shortcuts::{Command, CommandRegistry, KeyChord, OperatingSystem, Shortcut};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{Accent, spacing, typography};
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::{command_palette, dialog, focusable, sheet, toast};
use oito2_iced::window::{Labels, Placement, WindowState};
use oito2_iced::{Theme, focus, font};
use oito2_iced_settings::SystemSettings;

/// The width of the sidebar.
const SIDEBAR_WIDTH: f32 = 220.0;

fn main() -> iced::Result {
    let application = iced::application(Showcase::new, Showcase::update, Showcase::view)
        .title("OITO2 Iced")
        .theme(Showcase::theme)
        .subscription(Showcase::subscription);

    font::INTER_FONTS
        .into_iter()
        .chain(oito2_iced_syntax::font::JETBRAINS_MONO_FONTS)
        .fold(application, |application, bytes| application.font(bytes))
        .default_font(font::INTER)
        .window(window::Settings {
            size: iced::Size::new(1100.0, 760.0),
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
    page: PageId,
    pages: Pages,
    search: String,
    dialog_open: bool,
    sheet: Option<bool>,
    sheet_snap: usize,
    toasts: toast::Toasts<Message>,
    palette_open: bool,
    palette_query: String,
    commands: CommandRegistry<Message>,
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
    Navigate(PageId),
    Search(String),
    Page(catalog::Message),
    Key(keyboard::Event),
    CloseDialog,
    CloseSheet,
    Snap(usize),
    Toast(toast::Event),
    Undo,
    OpenPalette,
    PaletteQuery(String),
    ClosePalette,
}

/// The commands of the showcase: the command palette, the settings and a
/// command per page.
fn commands() -> CommandRegistry<Message> {
    let mut registry = CommandRegistry::new();
    let shortcut = |text: &str| Shortcut::parse(text).ok();
    let mut add = |command: Command<Message>, keys: Option<Shortcut>| {
        let command = match keys {
            Some(keys) => command.with_shortcut(keys),
            None => command,
        };
        registry.register(command);
    };

    add(
        Command::new("palette", "Command palette", Message::OpenPalette)
            .with_group("Window")
            .with_description("Search the commands"),
        shortcut("Mod+K"),
    );
    add(
        Command::new("settings", "Settings", Message::OpenSettings)
            .with_group("Window")
            .with_description("Theme, contrast, density, language and desktop"),
        shortcut("Mod+,"),
    );
    add(
        Command::new("accent", "Next accent", Message::NextAccent).with_group("Theme"),
        None,
    );
    for page in PageId::ALL {
        add(
            Command::new(
                format!("page-{}", page.slug()),
                format!("Go to {page}"),
                Message::Navigate(*page),
            )
            .with_group(page.group().title()),
            None,
        );
    }

    registry
}

impl Showcase {
    fn new() -> Self {
        let settings = oito2_iced_settings::read();
        let mut pages = Pages::default();
        pages::foundation::show(&mut pages.foundation, &settings);

        Self {
            settings,
            appearance: Appearance::default(),
            window: WindowState::default(),
            settings_open: false,
            page: PageId::default(),
            pages,
            search: String::new(),
            dialog_open: false,
            sheet: None,
            sheet_snap: 0,
            toasts: toast::Toasts::new(),
            palette_open: false,
            palette_query: String::new(),
            commands: commands(),
        }
    }

    fn theme(&self) -> Theme {
        self.appearance.theme(&self.settings)
    }

    /// What the pages receive from the application.
    fn context(&self) -> doc::Context {
        doc::Context {
            l10n: self.appearance.language.localizations(),
            direction: ReadingDirection::LeftToRight,
        }
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
            Message::System(settings) => {
                pages::foundation::show(&mut self.pages.foundation, &settings);
                self.settings = settings;
            }
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
            Message::Search(search) => self.search = search,
            Message::Page(message) => {
                let mut shell = Shell::default();
                let task = self.pages.update(message, &mut shell).map(Message::Page);
                for action in shell.into_actions() {
                    self.apply(action);
                }
                return task;
            }
            Message::Key(event) => {
                if let Some(chord) = KeyChord::from_keyboard_event(&event)
                    && let Some(command) = self
                        .commands
                        .find_by_chord(&chord, OperatingSystem::current())
                {
                    let action = command.action.clone();
                    return self.update(action);
                }
            }
            Message::CloseDialog => self.dialog_open = false,
            Message::CloseSheet => self.sheet = None,
            Message::Snap(snap) => self.sheet_snap = snap,
            Message::Toast(event) => {
                if let Some(action) = self.toasts.update(event) {
                    return self.update(action);
                }
            }
            Message::Undo => {
                self.toasts
                    .push(toast::Toast::new("Restored").with_kind(InfoType::Success));
            }
            Message::OpenPalette => {
                self.palette_open = true;
                self.palette_query.clear();
            }
            Message::PaletteQuery(query) => self.palette_query = query,
            Message::ClosePalette => self.palette_open = false,
        }

        Task::none()
    }

    /// Applies an action a page asked for.
    fn apply(&mut self, action: Action) {
        match action {
            Action::Toast {
                title,
                description,
                kind,
                undo,
            } => {
                let mut toast = toast::Toast::new(title).with_kind(kind);
                if let Some(description) = description {
                    toast = toast.with_description(description);
                }
                if undo {
                    toast = toast.with_action("Undo", Message::Undo);
                }
                self.toasts.push(toast);
            }
            Action::OpenDialog => self.dialog_open = true,
            Action::OpenSheet(bottom) => {
                self.sheet = Some(bottom);
                self.sheet_snap = 0;
            }
            Action::OpenPalette => {
                self.palette_open = true;
                self.palette_query.clear();
            }
            Action::OpenSettings => self.settings_open = true,
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            oito2_iced_settings::subscription().map(Message::System),
            window::events().map(|(id, event)| Message::WindowEvent(id, event)),
            keyboard::listen().map(Message::Key),
        ])
    }

    /// The pages whose title contains the search, case-insensitively.
    fn matching_pages(&self) -> impl Iterator<Item = PageId> + '_ {
        let query = self.search.trim().to_lowercase();

        PageId::ALL
            .iter()
            .copied()
            .filter(move |page| query.is_empty() || page.title().to_lowercase().contains(&query))
    }

    /// The sidebar: the search field and the matching pages by group.
    fn sidebar(&self, ctx: &doc::Context) -> Element<'_, Message, Theme> {
        let mut list = Column::new().spacing(spacing::XS).padding(spacing::SM);

        for group in Group::ALL {
            let pages: Vec<PageId> = self
                .matching_pages()
                .filter(|page| page.group() == group)
                .collect();
            if pages.is_empty() {
                continue;
            }

            list = list.push(
                container(
                    oito2_iced::widget::styled_text(group.title(), typography::LABEL_MEDIUM)
                        .class(TextClass::Variant),
                )
                .padding([spacing::SM, spacing::MD]),
            );
            for page in pages {
                list = list.push(
                    oito2_iced::widget::list_tile::list_tile()
                        .title(page.title())
                        .selected(page == self.page)
                        .on_press(Message::Navigate(page)),
                );
            }
        }

        let search = container(
            oito2_iced::widget::search::search_field(&ctx.l10n, &self.search)
                .on_input(Message::Search)
                .on_clear(Message::Search(String::new())),
        )
        .padding(spacing::SM);

        container(column![search, scrollable(list).height(Length::Fill)])
            .class(ContainerClass::Sidebar)
            .width(Length::Fixed(SIDEBAR_WIDTH))
            .height(Length::Fill)
            .into()
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        oito2_iced::density::set(self.appearance.density(&self.settings));

        let ctx = self.context();
        let localizations = ctx.l10n;
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

        let page = self.pages.view(self.page, &ctx).map(Message::Page);

        let body = row![
            self.sidebar(&ctx),
            scrollable(container(page).padding(spacing::PAGE).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::Fill),
        ];

        let window = oito2_iced::widget::tour::host(column![header, body], &self.pages.tour.tour)
            .on_event(|event| {
                Message::Page(catalog::Message::Tour(pages::tour::Message::Event(event)))
            })
            .localizations(&localizations);

        let sheet = self
            .sheet
            .map(|bottom| self.example_sheet(bottom, &localizations));
        let dialog = if self.settings_open {
            Some(self.settings_dialog())
        } else {
            self.dialog_open.then(|| {
                dialog::dialog(
                    text("Discard the changes made to this document?").width(Length::Fixed(320.0)),
                )
                .title("Discard changes?")
                .action(oito2_iced::widget::button::text("Cancel").on_press(Message::CloseDialog))
                .action(
                    oito2_iced::widget::button::danger("Discard").on_press(Message::CloseDialog),
                )
                .on_dismiss(Message::CloseDialog)
            })
        };
        let commands = self.commands.enabled();

        let content = toast::toaster(
            command_palette::command_palette(
                dialog::host(sheet::host(window, sheet), dialog),
                &localizations,
                &commands,
                &self.palette_query,
                Message::PaletteQuery,
                Message::ClosePalette,
            )
            .open(self.palette_open),
            &self.toasts,
            &localizations,
            Message::Toast,
        );

        oito2_iced::window::resize_handles(focus::navigation(content), Message::Window)
    }

    /// The example sheet: at the bottom with two snap points when `bottom`,
    /// at the end otherwise.
    fn example_sheet(
        &self,
        bottom: bool,
        localizations: &oito2_iced::l10n::Localizations,
    ) -> sheet::Sheet<'_, Message> {
        let body = column![
            text("Drag the edge toward the window edge, press Escape or click outside to close.")
                .class(TextClass::Variant),
            oito2_iced::widget::button::filled("Done").on_press(Message::CloseSheet),
        ]
        .spacing(spacing::MD);
        let content = sheet::content(body)
            .title(if bottom { "Bottom sheet" } else { "Details" })
            .on_close(Message::CloseSheet)
            .localizations(localizations);
        let sheet = sheet::sheet(content).on_dismiss(Message::CloseSheet);

        if bottom {
            sheet
                .side(sheet::Side::Bottom)
                .snap_points([0.35, 0.8], self.sheet_snap, Message::Snap)
        } else {
            sheet
        }
    }

    /// The settings dialog.
    fn settings_dialog(&self) -> dialog::Dialog<'_, Message> {
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

        let fields = column![
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
        ]
        .spacing(spacing::MD)
        .width(440);

        dialog::dialog(fields)
            .title("Settings")
            .action(oito2_iced::widget::button::filled("Close").on_press(Message::CloseSettings))
            .on_dismiss(Message::CloseSettings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced_test::simulator::Simulator;
    use oito2_iced::foundation::{DesktopEnvironment, OperatingSystem, WindowControlStyle};
    use oito2_iced_settings::Platform;
    use oito2_iced_themes::Preset;

    /// Simulator settings with the embedded Inter and the JetBrains Mono of
    /// the code blocks, so no text falls back to a system font.
    fn settings() -> iced::Settings {
        let mut settings = oito2_iced_test_utils::settings();
        settings.fonts.extend(
            oito2_iced_syntax::font::JETBRAINS_MONO_FONTS
                .into_iter()
                .map(Into::into),
        );
        settings
    }

    /// The showcase on GNOME with no system preferences.
    fn showcase() -> Showcase {
        let mut app = Showcase::new();
        let _ = app.update(Message::System(SystemSettings::fallback(
            Platform::new(OperatingSystem::Linux, false),
            Some(DesktopEnvironment::Gnome),
        )));
        app
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
            let mut ui = Simulator::with_settings(settings(), app.view());
            ui.click("Settings").expect("settings button");
            ui.into_messages().collect()
        };

        run(&mut app, messages);

        let mut ui = Simulator::with_settings(settings(), app.view());
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

        let mut ui = Simulator::with_settings(settings(), app.view());
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

    /// The value of the row `name` of the settings table of the Foundation
    /// page.
    fn foundation_value(app: &Showcase, name: &str) -> String {
        app.pages
            .foundation
            .rows
            .iter()
            .find(|[row, _]| row == name)
            .map(|[_, value]| value.clone())
            .unwrap_or_default()
    }

    #[test]
    fn the_foundation_page_follows_the_system_settings() {
        let mut app = on_page(PageId::Foundation);
        assert_eq!(foundation_value(&app, "Desktop environment"), "gnome");

        let kde = SystemSettings {
            accent_color: Some("#3A944A".to_owned()),
            high_contrast: true,
            ..SystemSettings::fallback(
                Platform::new(OperatingSystem::Linux, false),
                Some(DesktopEnvironment::Kde),
            )
        };
        run(&mut app, [Message::System(kde)]);

        assert_eq!(foundation_value(&app, "Desktop environment"), "kde");
        assert_eq!(foundation_value(&app, "Window control style"), "breeze");
        assert_eq!(foundation_value(&app, "Accent color"), "#3A944A");
        assert_eq!(foundation_value(&app, "High contrast"), "true");
    }

    /// A press of `key` with the primary modifier of the platform.
    fn primary_press(key: &str) -> keyboard::Event {
        let modifiers = if oito2_iced::shortcuts::uses_command_key(OperatingSystem::current()) {
            keyboard::Modifiers::LOGO
        } else {
            keyboard::Modifiers::CTRL
        };

        keyboard::Event::KeyPressed {
            key: keyboard::Key::Character(key.into()),
            modified_key: keyboard::Key::Character(key.into()),
            physical_key: keyboard::key::Physical::Unidentified(
                keyboard::key::NativeCode::Unidentified,
            ),
            location: keyboard::Location::Standard,
            modifiers,
            text: None,
            repeat: false,
        }
    }

    #[test]
    fn the_primary_k_shortcut_opens_the_palette_and_enter_runs_a_command() {
        let mut app = showcase();
        run(&mut app, [Message::Key(primary_press("k"))]);
        assert!(app.palette_open);

        run(&mut app, [Message::PaletteQuery("qrcode".to_owned())]);
        let messages: Vec<Message> = {
            let mut ui = Simulator::with_settings(settings(), app.view());
            ui.tap_key(keyboard::key::Named::Enter);
            ui.into_messages().collect()
        };
        run(&mut app, messages);

        assert!(!app.palette_open);
        assert_eq!(app.page, PageId::QrCode);
    }

    #[test]
    fn page_actions_open_the_window_surfaces() {
        let mut app = showcase();

        app.apply(Action::OpenDialog);
        app.apply(Action::OpenSheet(false));
        app.apply(Action::Toast {
            title: "Saved".to_owned(),
            description: None,
            kind: InfoType::Success,
            undo: true,
        });

        assert!(app.dialog_open);
        assert_eq!(app.sheet, Some(false));
        assert_eq!(app.toasts.active_count(), 1);

        let mut ui = Simulator::with_size(settings(), TALL, app.view());
        ui.tap_key(keyboard::key::Named::Escape);
        let messages: Vec<Message> = ui.into_messages().collect();
        run(&mut app, messages);

        assert!(!app.dialog_open);
    }

    #[test]
    fn the_search_filters_the_sidebar() {
        let mut app = showcase();
        run(&mut app, [Message::Search("qr".to_owned())]);

        assert_eq!(app.matching_pages().collect::<Vec<_>>(), [PageId::QrCode]);

        let mut ui = Simulator::with_size(settings(), TALL, app.view());
        assert!(ui.find("QrCode").is_ok());
        assert!(ui.find("Changelog").is_err());
    }

    /// A window tall enough to show every card of a page without scrolling.
    const TALL: (f32, f32) = (1280.0, 3200.0);

    /// The showcase on `page`.
    fn on_page(page: PageId) -> Showcase {
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
        for &page in PageId::ALL {
            let mut app = on_page(page);

            for (mode, language, size) in [
                (Mode::Light, Language::English, (1280.0, 800.0)),
                (Mode::Dark, Language::Portuguese, (560.0, 720.0)),
            ] {
                run(&mut app, [Message::Mode(mode), Message::Language(language)]);

                let theme = app.theme();
                let mut ui = Simulator::with_size(settings(), size, app.view());

                assert!(ui.snapshot(&theme).is_ok(), "{page} {mode:?}");
            }
        }
    }

    /// The content of the current page alone, with the focus traversal of
    /// the application, so the audits cover the page and not the sidebar.
    fn content(app: &Showcase) -> Element<'_, Message, Theme> {
        let page = app.pages.view(app.page, &app.context()).map(Message::Page);

        focus::navigation(container(page).padding(spacing::PAGE).width(Length::Fill)).into()
    }

    #[test]
    fn tab_reaches_every_control_of_every_page() {
        for &page in PageId::ALL {
            let app = on_page(page);
            let mut ui = Simulator::with_size(settings(), TALL, content(&app));
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
        for &page in PageId::ALL {
            let app = on_page(page);
            let mut ui = Simulator::with_size(settings(), TALL, content(&app));

            // A carousel takes the focus as a whole for its keys; only its
            // buttons and dots are clickable, and they are checked apart.
            // A floating panel title bar drags the panel and shows the arrow,
            // as a window title bar does.
            let carousel = iced::widget::Id::new(pages::carousel::CAROUSEL);
            let panel_bar = iced::widget::Id::new(pages::floating_panel::PANEL_BAR);
            let mut carousels = Vec::new();
            let _ = ui.find(
                |candidate: iced_test::selector::Candidate<'_>| -> Option<()> {
                    if let iced_test::selector::Candidate::Focusable {
                        id: Some(id),
                        bounds,
                        ..
                    } = candidate
                        && (*id == carousel || *id == panel_bar)
                    {
                        carousels.push(bounds);
                    }
                    None
                },
            );

            for (bounds, text_input, _) in focusables(&mut ui)
                .into_iter()
                .filter(|(bounds, _, visible)| *visible && !carousels.contains(bounds))
            {
                let interaction =
                    oito2_iced_test_utils::mouse_interaction(content(&app), TALL, bounds.center());

                if text_input {
                    assert_eq!(interaction, iced::mouse::Interaction::Text, "{page}");
                } else {
                    assert!(
                        matches!(
                            interaction,
                            // An editable label shows the text cursor, as text
                            // it can be edited in place; a divider handle shows
                            // the resize cursor of its axis; the crop area of an
                            // image cropper shows the move cursor of its drag.
                            iced::mouse::Interaction::Pointer
                                | iced::mouse::Interaction::Grab
                                | iced::mouse::Interaction::Move
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
            let mut ui = Simulator::with_size(settings(), (960.0, 680.0), app.view());
            let snapshot = ui.snapshot(&theme).expect("snapshot");

            oito2_iced_test_utils::golden!(&snapshot, name);
        }
    }
}
