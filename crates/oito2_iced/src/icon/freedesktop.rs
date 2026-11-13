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

//! Icons named after the freedesktop.org Icon Naming Specification, drawn
//! with Lucide icons.
//!
//! Linux integrations refer to icons by these names, for example in
//! `.desktop` files, notifications and tray items; [`Freedesktop::by_name`]
//! turns such a name into an icon.
//!
//! ```
//! use oito2_iced::icon::Freedesktop;
//!
//! assert_eq!(Freedesktop::DocumentOpen.name(), "document-open");
//! assert_eq!(
//!     Freedesktop::by_name("folder-documents-symbolic"),
//!     Some(Freedesktop::Folder)
//! );
//! ```

use super::{Icon, lucide};

/// An icon of the freedesktop.org Icon Naming Specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Freedesktop {
    // Actions.
    /// `address-book-new`.
    AddressBookNew,
    /// `application-exit`.
    ApplicationExit,
    /// `appointment-new`.
    AppointmentNew,
    /// `call-start`.
    CallStart,
    /// `call-stop`.
    CallStop,
    /// `contact-new`.
    ContactNew,
    /// `document-new`.
    DocumentNew,
    /// `document-open`.
    DocumentOpen,
    /// `document-open-recent`.
    DocumentOpenRecent,
    /// `document-page-setup`.
    DocumentPageSetup,
    /// `document-print`.
    DocumentPrint,
    /// `document-print-preview`.
    DocumentPrintPreview,
    /// `document-properties`.
    DocumentProperties,
    /// `document-revert`.
    DocumentRevert,
    /// `document-save`.
    DocumentSave,
    /// `document-save-as`.
    DocumentSaveAs,
    /// `document-send`.
    DocumentSend,
    /// `edit-clear`.
    EditClear,
    /// `edit-copy`.
    EditCopy,
    /// `edit-cut`.
    EditCut,
    /// `edit-delete`.
    EditDelete,
    /// `edit-find`.
    EditFind,
    /// `edit-find-replace`.
    EditFindReplace,
    /// `edit-paste`.
    EditPaste,
    /// `edit-redo`.
    EditRedo,
    /// `edit-select-all`.
    EditSelectAll,
    /// `edit-undo`.
    EditUndo,
    /// `folder-new`.
    FolderNew,
    /// `format-indent-less`.
    FormatIndentLess,
    /// `format-indent-more`.
    FormatIndentMore,
    /// `format-justify-center`.
    FormatJustifyCenter,
    /// `format-justify-fill`.
    FormatJustifyFill,
    /// `format-justify-left`.
    FormatJustifyLeft,
    /// `format-justify-right`.
    FormatJustifyRight,
    /// `format-text-bold`.
    FormatTextBold,
    /// `format-text-italic`.
    FormatTextItalic,
    /// `format-text-strikethrough`.
    FormatTextStrikethrough,
    /// `format-text-underline`.
    FormatTextUnderline,
    /// `go-bottom`.
    GoBottom,
    /// `go-down`.
    GoDown,
    /// `go-first`.
    GoFirst,
    /// `go-home`.
    GoHome,
    /// `go-jump`.
    GoJump,
    /// `go-last`.
    GoLast,
    /// `go-next`.
    GoNext,
    /// `go-previous`.
    GoPrevious,
    /// `go-top`.
    GoTop,
    /// `go-up`.
    GoUp,
    /// `help-about`.
    HelpAbout,
    /// `help-contents`.
    HelpContents,
    /// `help-faq`.
    HelpFaq,
    /// `insert-image`.
    InsertImage,
    /// `insert-link`.
    InsertLink,
    /// `insert-text`.
    InsertText,
    /// `list-add`.
    ListAdd,
    /// `list-remove`.
    ListRemove,
    /// `mail-forward`.
    MailForward,
    /// `mail-mark-important`.
    MailMarkImportant,
    /// `mail-mark-read`.
    MailMarkRead,
    /// `mail-message-new`.
    MailMessageNew,
    /// `mail-reply-all`.
    MailReplyAll,
    /// `mail-reply-sender`.
    MailReplySender,
    /// `mail-send`.
    MailSend,
    /// `media-eject`.
    MediaEject,
    /// `media-playback-pause`.
    MediaPlaybackPause,
    /// `media-playback-start`.
    MediaPlaybackStart,
    /// `media-playback-stop`.
    MediaPlaybackStop,
    /// `media-record`.
    MediaRecord,
    /// `media-seek-backward`.
    MediaSeekBackward,
    /// `media-seek-forward`.
    MediaSeekForward,
    /// `media-skip-backward`.
    MediaSkipBackward,
    /// `media-skip-forward`.
    MediaSkipForward,
    /// `object-flip-horizontal`.
    ObjectFlipHorizontal,
    /// `object-flip-vertical`.
    ObjectFlipVertical,
    /// `object-rotate-left`.
    ObjectRotateLeft,
    /// `object-rotate-right`.
    ObjectRotateRight,
    /// `process-stop`.
    ProcessStop,
    /// `system-lock-screen`.
    SystemLockScreen,
    /// `system-log-out`.
    SystemLogOut,
    /// `system-reboot`.
    SystemReboot,
    /// `system-run`.
    SystemRun,
    /// `system-search`.
    SystemSearch,
    /// `system-shutdown`.
    SystemShutdown,
    /// `tools-check-spelling`.
    ToolsCheckSpelling,
    /// `view-fullscreen`.
    ViewFullscreen,
    /// `view-refresh`.
    ViewRefresh,
    /// `view-restore`.
    ViewRestore,
    /// `view-sort-ascending`.
    ViewSortAscending,
    /// `view-sort-descending`.
    ViewSortDescending,
    /// `window-close`.
    WindowClose,
    /// `window-new`.
    WindowNew,
    /// `zoom-fit-best`.
    ZoomFitBest,
    /// `zoom-in`.
    ZoomIn,
    /// `zoom-original`.
    ZoomOriginal,
    /// `zoom-out`.
    ZoomOut,
    // Applications.
    /// `accessories-calculator`.
    AccessoriesCalculator,
    /// `accessories-text-editor`.
    AccessoriesTextEditor,
    /// `help-browser`.
    HelpBrowser,
    /// `multimedia-volume-control`.
    MultimediaVolumeControl,
    /// `preferences-desktop-accessibility`.
    PreferencesDesktopAccessibility,
    /// `preferences-desktop-font`.
    PreferencesDesktopFont,
    /// `preferences-desktop-keyboard`.
    PreferencesDesktopKeyboard,
    /// `preferences-desktop-locale`.
    PreferencesDesktopLocale,
    /// `preferences-desktop-theme`.
    PreferencesDesktopTheme,
    /// `preferences-system`.
    PreferencesSystem,
    /// `system-file-manager`.
    SystemFileManager,
    /// `utilities-terminal`.
    UtilitiesTerminal,
    /// `utilities-system-monitor`.
    UtilitiesSystemMonitor,
    // Categories.
    /// `applications-games`.
    ApplicationsGames,
    /// `applications-graphics`.
    ApplicationsGraphics,
    /// `applications-internet`.
    ApplicationsInternet,
    /// `applications-multimedia`.
    ApplicationsMultimedia,
    /// `applications-office`.
    ApplicationsOffice,
    /// `applications-science`.
    ApplicationsScience,
    /// `applications-utilities`.
    ApplicationsUtilities,
    /// `applications-development`.
    ApplicationsDevelopment,
    // Devices.
    /// `audio-card`.
    AudioCard,
    /// `audio-input-microphone`.
    AudioInputMicrophone,
    /// `battery`.
    Battery,
    /// `camera-photo`.
    CameraPhoto,
    /// `camera-video`.
    CameraVideo,
    /// `camera-web`.
    CameraWeb,
    /// `computer`.
    Computer,
    /// `drive-harddisk`.
    DriveHarddisk,
    /// `drive-optical`.
    DriveOptical,
    /// `drive-removable-media`.
    DriveRemovableMedia,
    /// `input-gaming`.
    InputGaming,
    /// `input-keyboard`.
    InputKeyboard,
    /// `input-mouse`.
    InputMouse,
    /// `input-tablet`.
    InputTablet,
    /// `media-flash`.
    MediaFlash,
    /// `network-wired`.
    NetworkWired,
    /// `network-wireless`.
    NetworkWireless,
    /// `phone`.
    Phone,
    /// `printer`.
    Printer,
    /// `scanner`.
    Scanner,
    /// `video-display`.
    VideoDisplay,
    // Emblems.
    /// `emblem-default`.
    EmblemDefault,
    /// `emblem-documents`.
    EmblemDocuments,
    /// `emblem-favorite`.
    EmblemFavorite,
    /// `emblem-important`.
    EmblemImportant,
    /// `emblem-mail`.
    EmblemMail,
    /// `emblem-photos`.
    EmblemPhotos,
    /// `emblem-readonly`.
    EmblemReadonly,
    /// `emblem-shared`.
    EmblemShared,
    /// `emblem-symbolic-link`.
    EmblemSymbolicLink,
    /// `emblem-system`.
    EmblemSystem,
    /// `emblem-unreadable`.
    EmblemUnreadable,
    // Places.
    /// `folder`.
    Folder,
    /// `folder-remote`.
    FolderRemote,
    /// `network-server`.
    NetworkServer,
    /// `network-workgroup`.
    NetworkWorkgroup,
    /// `start-here`.
    StartHere,
    /// `user-bookmarks`.
    UserBookmarks,
    /// `user-desktop`.
    UserDesktop,
    /// `user-home`.
    UserHome,
    /// `user-trash`.
    UserTrash,
    // Status.
    /// `appointment-soon`.
    AppointmentSoon,
    /// `audio-volume-high`.
    AudioVolumeHigh,
    /// `audio-volume-low`.
    AudioVolumeLow,
    /// `audio-volume-medium`.
    AudioVolumeMedium,
    /// `audio-volume-muted`.
    AudioVolumeMuted,
    /// `battery-caution`.
    BatteryCaution,
    /// `battery-low`.
    BatteryLow,
    /// `dialog-error`.
    DialogError,
    /// `dialog-information`.
    DialogInformation,
    /// `dialog-password`.
    DialogPassword,
    /// `dialog-question`.
    DialogQuestion,
    /// `dialog-warning`.
    DialogWarning,
    /// `folder-drag-accept`.
    FolderDragAccept,
    /// `folder-open`.
    FolderOpen,
    /// `image-loading`.
    ImageLoading,
    /// `image-missing`.
    ImageMissing,
    /// `mail-attachment`.
    MailAttachment,
    /// `mail-unread`.
    MailUnread,
    /// `network-error`.
    NetworkError,
    /// `network-idle`.
    NetworkIdle,
    /// `network-offline`.
    NetworkOffline,
    /// `network-receive`.
    NetworkReceive,
    /// `network-transmit`.
    NetworkTransmit,
    /// `printer-error`.
    PrinterError,
    /// `software-update-available`.
    SoftwareUpdateAvailable,
    /// `software-update-urgent`.
    SoftwareUpdateUrgent,
    /// `sync-error`.
    SyncError,
    /// `sync-synchronizing`.
    SyncSynchronizing,
    /// `task-due`.
    TaskDue,
    /// `user-available`.
    UserAvailable,
    /// `user-away`.
    UserAway,
    /// `user-idle`.
    UserIdle,
    /// `user-offline`.
    UserOffline,
    /// `user-trash-full`.
    UserTrashFull,
    /// `weather-clear`.
    WeatherClear,
    /// `weather-few-clouds`.
    WeatherFewClouds,
    /// `weather-fog`.
    WeatherFog,
    /// `weather-overcast`.
    WeatherOvercast,
    /// `weather-severe-alert`.
    WeatherSevereAlert,
    /// `weather-showers`.
    WeatherShowers,
    /// `weather-snow`.
    WeatherSnow,
    /// `weather-storm`.
    WeatherStorm,
}

impl Freedesktop {
    /// Every icon, in declaration order.
    pub const ALL: [Freedesktop; 199] = [
        Freedesktop::AddressBookNew,
        Freedesktop::ApplicationExit,
        Freedesktop::AppointmentNew,
        Freedesktop::CallStart,
        Freedesktop::CallStop,
        Freedesktop::ContactNew,
        Freedesktop::DocumentNew,
        Freedesktop::DocumentOpen,
        Freedesktop::DocumentOpenRecent,
        Freedesktop::DocumentPageSetup,
        Freedesktop::DocumentPrint,
        Freedesktop::DocumentPrintPreview,
        Freedesktop::DocumentProperties,
        Freedesktop::DocumentRevert,
        Freedesktop::DocumentSave,
        Freedesktop::DocumentSaveAs,
        Freedesktop::DocumentSend,
        Freedesktop::EditClear,
        Freedesktop::EditCopy,
        Freedesktop::EditCut,
        Freedesktop::EditDelete,
        Freedesktop::EditFind,
        Freedesktop::EditFindReplace,
        Freedesktop::EditPaste,
        Freedesktop::EditRedo,
        Freedesktop::EditSelectAll,
        Freedesktop::EditUndo,
        Freedesktop::FolderNew,
        Freedesktop::FormatIndentLess,
        Freedesktop::FormatIndentMore,
        Freedesktop::FormatJustifyCenter,
        Freedesktop::FormatJustifyFill,
        Freedesktop::FormatJustifyLeft,
        Freedesktop::FormatJustifyRight,
        Freedesktop::FormatTextBold,
        Freedesktop::FormatTextItalic,
        Freedesktop::FormatTextStrikethrough,
        Freedesktop::FormatTextUnderline,
        Freedesktop::GoBottom,
        Freedesktop::GoDown,
        Freedesktop::GoFirst,
        Freedesktop::GoHome,
        Freedesktop::GoJump,
        Freedesktop::GoLast,
        Freedesktop::GoNext,
        Freedesktop::GoPrevious,
        Freedesktop::GoTop,
        Freedesktop::GoUp,
        Freedesktop::HelpAbout,
        Freedesktop::HelpContents,
        Freedesktop::HelpFaq,
        Freedesktop::InsertImage,
        Freedesktop::InsertLink,
        Freedesktop::InsertText,
        Freedesktop::ListAdd,
        Freedesktop::ListRemove,
        Freedesktop::MailForward,
        Freedesktop::MailMarkImportant,
        Freedesktop::MailMarkRead,
        Freedesktop::MailMessageNew,
        Freedesktop::MailReplyAll,
        Freedesktop::MailReplySender,
        Freedesktop::MailSend,
        Freedesktop::MediaEject,
        Freedesktop::MediaPlaybackPause,
        Freedesktop::MediaPlaybackStart,
        Freedesktop::MediaPlaybackStop,
        Freedesktop::MediaRecord,
        Freedesktop::MediaSeekBackward,
        Freedesktop::MediaSeekForward,
        Freedesktop::MediaSkipBackward,
        Freedesktop::MediaSkipForward,
        Freedesktop::ObjectFlipHorizontal,
        Freedesktop::ObjectFlipVertical,
        Freedesktop::ObjectRotateLeft,
        Freedesktop::ObjectRotateRight,
        Freedesktop::ProcessStop,
        Freedesktop::SystemLockScreen,
        Freedesktop::SystemLogOut,
        Freedesktop::SystemReboot,
        Freedesktop::SystemRun,
        Freedesktop::SystemSearch,
        Freedesktop::SystemShutdown,
        Freedesktop::ToolsCheckSpelling,
        Freedesktop::ViewFullscreen,
        Freedesktop::ViewRefresh,
        Freedesktop::ViewRestore,
        Freedesktop::ViewSortAscending,
        Freedesktop::ViewSortDescending,
        Freedesktop::WindowClose,
        Freedesktop::WindowNew,
        Freedesktop::ZoomFitBest,
        Freedesktop::ZoomIn,
        Freedesktop::ZoomOriginal,
        Freedesktop::ZoomOut,
        Freedesktop::AccessoriesCalculator,
        Freedesktop::AccessoriesTextEditor,
        Freedesktop::HelpBrowser,
        Freedesktop::MultimediaVolumeControl,
        Freedesktop::PreferencesDesktopAccessibility,
        Freedesktop::PreferencesDesktopFont,
        Freedesktop::PreferencesDesktopKeyboard,
        Freedesktop::PreferencesDesktopLocale,
        Freedesktop::PreferencesDesktopTheme,
        Freedesktop::PreferencesSystem,
        Freedesktop::SystemFileManager,
        Freedesktop::UtilitiesTerminal,
        Freedesktop::UtilitiesSystemMonitor,
        Freedesktop::ApplicationsGames,
        Freedesktop::ApplicationsGraphics,
        Freedesktop::ApplicationsInternet,
        Freedesktop::ApplicationsMultimedia,
        Freedesktop::ApplicationsOffice,
        Freedesktop::ApplicationsScience,
        Freedesktop::ApplicationsUtilities,
        Freedesktop::ApplicationsDevelopment,
        Freedesktop::AudioCard,
        Freedesktop::AudioInputMicrophone,
        Freedesktop::Battery,
        Freedesktop::CameraPhoto,
        Freedesktop::CameraVideo,
        Freedesktop::CameraWeb,
        Freedesktop::Computer,
        Freedesktop::DriveHarddisk,
        Freedesktop::DriveOptical,
        Freedesktop::DriveRemovableMedia,
        Freedesktop::InputGaming,
        Freedesktop::InputKeyboard,
        Freedesktop::InputMouse,
        Freedesktop::InputTablet,
        Freedesktop::MediaFlash,
        Freedesktop::NetworkWired,
        Freedesktop::NetworkWireless,
        Freedesktop::Phone,
        Freedesktop::Printer,
        Freedesktop::Scanner,
        Freedesktop::VideoDisplay,
        Freedesktop::EmblemDefault,
        Freedesktop::EmblemDocuments,
        Freedesktop::EmblemFavorite,
        Freedesktop::EmblemImportant,
        Freedesktop::EmblemMail,
        Freedesktop::EmblemPhotos,
        Freedesktop::EmblemReadonly,
        Freedesktop::EmblemShared,
        Freedesktop::EmblemSymbolicLink,
        Freedesktop::EmblemSystem,
        Freedesktop::EmblemUnreadable,
        Freedesktop::Folder,
        Freedesktop::FolderRemote,
        Freedesktop::NetworkServer,
        Freedesktop::NetworkWorkgroup,
        Freedesktop::StartHere,
        Freedesktop::UserBookmarks,
        Freedesktop::UserDesktop,
        Freedesktop::UserHome,
        Freedesktop::UserTrash,
        Freedesktop::AppointmentSoon,
        Freedesktop::AudioVolumeHigh,
        Freedesktop::AudioVolumeLow,
        Freedesktop::AudioVolumeMedium,
        Freedesktop::AudioVolumeMuted,
        Freedesktop::BatteryCaution,
        Freedesktop::BatteryLow,
        Freedesktop::DialogError,
        Freedesktop::DialogInformation,
        Freedesktop::DialogPassword,
        Freedesktop::DialogQuestion,
        Freedesktop::DialogWarning,
        Freedesktop::FolderDragAccept,
        Freedesktop::FolderOpen,
        Freedesktop::ImageLoading,
        Freedesktop::ImageMissing,
        Freedesktop::MailAttachment,
        Freedesktop::MailUnread,
        Freedesktop::NetworkError,
        Freedesktop::NetworkIdle,
        Freedesktop::NetworkOffline,
        Freedesktop::NetworkReceive,
        Freedesktop::NetworkTransmit,
        Freedesktop::PrinterError,
        Freedesktop::SoftwareUpdateAvailable,
        Freedesktop::SoftwareUpdateUrgent,
        Freedesktop::SyncError,
        Freedesktop::SyncSynchronizing,
        Freedesktop::TaskDue,
        Freedesktop::UserAvailable,
        Freedesktop::UserAway,
        Freedesktop::UserIdle,
        Freedesktop::UserOffline,
        Freedesktop::UserTrashFull,
        Freedesktop::WeatherClear,
        Freedesktop::WeatherFewClouds,
        Freedesktop::WeatherFog,
        Freedesktop::WeatherOvercast,
        Freedesktop::WeatherSevereAlert,
        Freedesktop::WeatherShowers,
        Freedesktop::WeatherSnow,
        Freedesktop::WeatherStorm,
    ];

    /// The name in the Icon Naming Specification, such as `edit-copy`.
    pub const fn name(self) -> &'static str {
        match self {
            Freedesktop::AddressBookNew => "address-book-new",
            Freedesktop::ApplicationExit => "application-exit",
            Freedesktop::AppointmentNew => "appointment-new",
            Freedesktop::CallStart => "call-start",
            Freedesktop::CallStop => "call-stop",
            Freedesktop::ContactNew => "contact-new",
            Freedesktop::DocumentNew => "document-new",
            Freedesktop::DocumentOpen => "document-open",
            Freedesktop::DocumentOpenRecent => "document-open-recent",
            Freedesktop::DocumentPageSetup => "document-page-setup",
            Freedesktop::DocumentPrint => "document-print",
            Freedesktop::DocumentPrintPreview => "document-print-preview",
            Freedesktop::DocumentProperties => "document-properties",
            Freedesktop::DocumentRevert => "document-revert",
            Freedesktop::DocumentSave => "document-save",
            Freedesktop::DocumentSaveAs => "document-save-as",
            Freedesktop::DocumentSend => "document-send",
            Freedesktop::EditClear => "edit-clear",
            Freedesktop::EditCopy => "edit-copy",
            Freedesktop::EditCut => "edit-cut",
            Freedesktop::EditDelete => "edit-delete",
            Freedesktop::EditFind => "edit-find",
            Freedesktop::EditFindReplace => "edit-find-replace",
            Freedesktop::EditPaste => "edit-paste",
            Freedesktop::EditRedo => "edit-redo",
            Freedesktop::EditSelectAll => "edit-select-all",
            Freedesktop::EditUndo => "edit-undo",
            Freedesktop::FolderNew => "folder-new",
            Freedesktop::FormatIndentLess => "format-indent-less",
            Freedesktop::FormatIndentMore => "format-indent-more",
            Freedesktop::FormatJustifyCenter => "format-justify-center",
            Freedesktop::FormatJustifyFill => "format-justify-fill",
            Freedesktop::FormatJustifyLeft => "format-justify-left",
            Freedesktop::FormatJustifyRight => "format-justify-right",
            Freedesktop::FormatTextBold => "format-text-bold",
            Freedesktop::FormatTextItalic => "format-text-italic",
            Freedesktop::FormatTextStrikethrough => "format-text-strikethrough",
            Freedesktop::FormatTextUnderline => "format-text-underline",
            Freedesktop::GoBottom => "go-bottom",
            Freedesktop::GoDown => "go-down",
            Freedesktop::GoFirst => "go-first",
            Freedesktop::GoHome => "go-home",
            Freedesktop::GoJump => "go-jump",
            Freedesktop::GoLast => "go-last",
            Freedesktop::GoNext => "go-next",
            Freedesktop::GoPrevious => "go-previous",
            Freedesktop::GoTop => "go-top",
            Freedesktop::GoUp => "go-up",
            Freedesktop::HelpAbout => "help-about",
            Freedesktop::HelpContents => "help-contents",
            Freedesktop::HelpFaq => "help-faq",
            Freedesktop::InsertImage => "insert-image",
            Freedesktop::InsertLink => "insert-link",
            Freedesktop::InsertText => "insert-text",
            Freedesktop::ListAdd => "list-add",
            Freedesktop::ListRemove => "list-remove",
            Freedesktop::MailForward => "mail-forward",
            Freedesktop::MailMarkImportant => "mail-mark-important",
            Freedesktop::MailMarkRead => "mail-mark-read",
            Freedesktop::MailMessageNew => "mail-message-new",
            Freedesktop::MailReplyAll => "mail-reply-all",
            Freedesktop::MailReplySender => "mail-reply-sender",
            Freedesktop::MailSend => "mail-send",
            Freedesktop::MediaEject => "media-eject",
            Freedesktop::MediaPlaybackPause => "media-playback-pause",
            Freedesktop::MediaPlaybackStart => "media-playback-start",
            Freedesktop::MediaPlaybackStop => "media-playback-stop",
            Freedesktop::MediaRecord => "media-record",
            Freedesktop::MediaSeekBackward => "media-seek-backward",
            Freedesktop::MediaSeekForward => "media-seek-forward",
            Freedesktop::MediaSkipBackward => "media-skip-backward",
            Freedesktop::MediaSkipForward => "media-skip-forward",
            Freedesktop::ObjectFlipHorizontal => "object-flip-horizontal",
            Freedesktop::ObjectFlipVertical => "object-flip-vertical",
            Freedesktop::ObjectRotateLeft => "object-rotate-left",
            Freedesktop::ObjectRotateRight => "object-rotate-right",
            Freedesktop::ProcessStop => "process-stop",
            Freedesktop::SystemLockScreen => "system-lock-screen",
            Freedesktop::SystemLogOut => "system-log-out",
            Freedesktop::SystemReboot => "system-reboot",
            Freedesktop::SystemRun => "system-run",
            Freedesktop::SystemSearch => "system-search",
            Freedesktop::SystemShutdown => "system-shutdown",
            Freedesktop::ToolsCheckSpelling => "tools-check-spelling",
            Freedesktop::ViewFullscreen => "view-fullscreen",
            Freedesktop::ViewRefresh => "view-refresh",
            Freedesktop::ViewRestore => "view-restore",
            Freedesktop::ViewSortAscending => "view-sort-ascending",
            Freedesktop::ViewSortDescending => "view-sort-descending",
            Freedesktop::WindowClose => "window-close",
            Freedesktop::WindowNew => "window-new",
            Freedesktop::ZoomFitBest => "zoom-fit-best",
            Freedesktop::ZoomIn => "zoom-in",
            Freedesktop::ZoomOriginal => "zoom-original",
            Freedesktop::ZoomOut => "zoom-out",
            Freedesktop::AccessoriesCalculator => "accessories-calculator",
            Freedesktop::AccessoriesTextEditor => "accessories-text-editor",
            Freedesktop::HelpBrowser => "help-browser",
            Freedesktop::MultimediaVolumeControl => "multimedia-volume-control",
            Freedesktop::PreferencesDesktopAccessibility => "preferences-desktop-accessibility",
            Freedesktop::PreferencesDesktopFont => "preferences-desktop-font",
            Freedesktop::PreferencesDesktopKeyboard => "preferences-desktop-keyboard",
            Freedesktop::PreferencesDesktopLocale => "preferences-desktop-locale",
            Freedesktop::PreferencesDesktopTheme => "preferences-desktop-theme",
            Freedesktop::PreferencesSystem => "preferences-system",
            Freedesktop::SystemFileManager => "system-file-manager",
            Freedesktop::UtilitiesTerminal => "utilities-terminal",
            Freedesktop::UtilitiesSystemMonitor => "utilities-system-monitor",
            Freedesktop::ApplicationsGames => "applications-games",
            Freedesktop::ApplicationsGraphics => "applications-graphics",
            Freedesktop::ApplicationsInternet => "applications-internet",
            Freedesktop::ApplicationsMultimedia => "applications-multimedia",
            Freedesktop::ApplicationsOffice => "applications-office",
            Freedesktop::ApplicationsScience => "applications-science",
            Freedesktop::ApplicationsUtilities => "applications-utilities",
            Freedesktop::ApplicationsDevelopment => "applications-development",
            Freedesktop::AudioCard => "audio-card",
            Freedesktop::AudioInputMicrophone => "audio-input-microphone",
            Freedesktop::Battery => "battery",
            Freedesktop::CameraPhoto => "camera-photo",
            Freedesktop::CameraVideo => "camera-video",
            Freedesktop::CameraWeb => "camera-web",
            Freedesktop::Computer => "computer",
            Freedesktop::DriveHarddisk => "drive-harddisk",
            Freedesktop::DriveOptical => "drive-optical",
            Freedesktop::DriveRemovableMedia => "drive-removable-media",
            Freedesktop::InputGaming => "input-gaming",
            Freedesktop::InputKeyboard => "input-keyboard",
            Freedesktop::InputMouse => "input-mouse",
            Freedesktop::InputTablet => "input-tablet",
            Freedesktop::MediaFlash => "media-flash",
            Freedesktop::NetworkWired => "network-wired",
            Freedesktop::NetworkWireless => "network-wireless",
            Freedesktop::Phone => "phone",
            Freedesktop::Printer => "printer",
            Freedesktop::Scanner => "scanner",
            Freedesktop::VideoDisplay => "video-display",
            Freedesktop::EmblemDefault => "emblem-default",
            Freedesktop::EmblemDocuments => "emblem-documents",
            Freedesktop::EmblemFavorite => "emblem-favorite",
            Freedesktop::EmblemImportant => "emblem-important",
            Freedesktop::EmblemMail => "emblem-mail",
            Freedesktop::EmblemPhotos => "emblem-photos",
            Freedesktop::EmblemReadonly => "emblem-readonly",
            Freedesktop::EmblemShared => "emblem-shared",
            Freedesktop::EmblemSymbolicLink => "emblem-symbolic-link",
            Freedesktop::EmblemSystem => "emblem-system",
            Freedesktop::EmblemUnreadable => "emblem-unreadable",
            Freedesktop::Folder => "folder",
            Freedesktop::FolderRemote => "folder-remote",
            Freedesktop::NetworkServer => "network-server",
            Freedesktop::NetworkWorkgroup => "network-workgroup",
            Freedesktop::StartHere => "start-here",
            Freedesktop::UserBookmarks => "user-bookmarks",
            Freedesktop::UserDesktop => "user-desktop",
            Freedesktop::UserHome => "user-home",
            Freedesktop::UserTrash => "user-trash",
            Freedesktop::AppointmentSoon => "appointment-soon",
            Freedesktop::AudioVolumeHigh => "audio-volume-high",
            Freedesktop::AudioVolumeLow => "audio-volume-low",
            Freedesktop::AudioVolumeMedium => "audio-volume-medium",
            Freedesktop::AudioVolumeMuted => "audio-volume-muted",
            Freedesktop::BatteryCaution => "battery-caution",
            Freedesktop::BatteryLow => "battery-low",
            Freedesktop::DialogError => "dialog-error",
            Freedesktop::DialogInformation => "dialog-information",
            Freedesktop::DialogPassword => "dialog-password",
            Freedesktop::DialogQuestion => "dialog-question",
            Freedesktop::DialogWarning => "dialog-warning",
            Freedesktop::FolderDragAccept => "folder-drag-accept",
            Freedesktop::FolderOpen => "folder-open",
            Freedesktop::ImageLoading => "image-loading",
            Freedesktop::ImageMissing => "image-missing",
            Freedesktop::MailAttachment => "mail-attachment",
            Freedesktop::MailUnread => "mail-unread",
            Freedesktop::NetworkError => "network-error",
            Freedesktop::NetworkIdle => "network-idle",
            Freedesktop::NetworkOffline => "network-offline",
            Freedesktop::NetworkReceive => "network-receive",
            Freedesktop::NetworkTransmit => "network-transmit",
            Freedesktop::PrinterError => "printer-error",
            Freedesktop::SoftwareUpdateAvailable => "software-update-available",
            Freedesktop::SoftwareUpdateUrgent => "software-update-urgent",
            Freedesktop::SyncError => "sync-error",
            Freedesktop::SyncSynchronizing => "sync-synchronizing",
            Freedesktop::TaskDue => "task-due",
            Freedesktop::UserAvailable => "user-available",
            Freedesktop::UserAway => "user-away",
            Freedesktop::UserIdle => "user-idle",
            Freedesktop::UserOffline => "user-offline",
            Freedesktop::UserTrashFull => "user-trash-full",
            Freedesktop::WeatherClear => "weather-clear",
            Freedesktop::WeatherFewClouds => "weather-few-clouds",
            Freedesktop::WeatherFog => "weather-fog",
            Freedesktop::WeatherOvercast => "weather-overcast",
            Freedesktop::WeatherSevereAlert => "weather-severe-alert",
            Freedesktop::WeatherShowers => "weather-showers",
            Freedesktop::WeatherSnow => "weather-snow",
            Freedesktop::WeatherStorm => "weather-storm",
        }
    }

    /// The Lucide icon that draws this icon.
    pub const fn icon(self) -> Icon {
        match self {
            Freedesktop::AddressBookNew => lucide::BOOK_USER,
            Freedesktop::ApplicationExit => lucide::LOG_OUT,
            Freedesktop::AppointmentNew => lucide::CALENDAR_PLUS,
            Freedesktop::CallStart => lucide::PHONE,
            Freedesktop::CallStop => lucide::PHONE_OFF,
            Freedesktop::ContactNew => lucide::USER_PLUS,
            Freedesktop::DocumentNew => lucide::FILE_PLUS,
            Freedesktop::DocumentOpen => lucide::FOLDER_OPEN,
            Freedesktop::DocumentOpenRecent => lucide::FILE_CLOCK,
            Freedesktop::DocumentPageSetup => lucide::FILE_COG,
            Freedesktop::DocumentPrint => lucide::PRINTER,
            Freedesktop::DocumentPrintPreview => lucide::FILE_SEARCH,
            Freedesktop::DocumentProperties => lucide::FILE_COG,
            Freedesktop::DocumentRevert => lucide::UNDO_2,
            Freedesktop::DocumentSave => lucide::SAVE,
            Freedesktop::DocumentSaveAs => lucide::SAVE_ALL,
            Freedesktop::DocumentSend => lucide::SEND,
            Freedesktop::EditClear => lucide::ERASER,
            Freedesktop::EditCopy => lucide::COPY,
            Freedesktop::EditCut => lucide::SCISSORS,
            Freedesktop::EditDelete => lucide::TRASH,
            Freedesktop::EditFind => lucide::SEARCH,
            Freedesktop::EditFindReplace => lucide::REPLACE,
            Freedesktop::EditPaste => lucide::CLIPBOARD_PASTE,
            Freedesktop::EditRedo => lucide::REDO_2,
            Freedesktop::EditSelectAll => lucide::SQUARE_DASHED_MOUSE_POINTER,
            Freedesktop::EditUndo => lucide::UNDO_2,
            Freedesktop::FolderNew => lucide::FOLDER_PLUS,
            Freedesktop::FormatIndentLess => lucide::LIST_INDENT_DECREASE,
            Freedesktop::FormatIndentMore => lucide::LIST_INDENT_INCREASE,
            Freedesktop::FormatJustifyCenter => lucide::TEXT_ALIGN_CENTER,
            Freedesktop::FormatJustifyFill => lucide::TEXT_ALIGN_JUSTIFY,
            Freedesktop::FormatJustifyLeft => lucide::TEXT_ALIGN_START,
            Freedesktop::FormatJustifyRight => lucide::TEXT_ALIGN_END,
            Freedesktop::FormatTextBold => lucide::LETTER_B,
            Freedesktop::FormatTextItalic => lucide::ITALIC,
            Freedesktop::FormatTextStrikethrough => lucide::STRIKETHROUGH,
            Freedesktop::FormatTextUnderline => lucide::UNDERLINE,
            Freedesktop::GoBottom => lucide::ARROW_DOWN_TO_LINE,
            Freedesktop::GoDown => lucide::ARROW_DOWN,
            Freedesktop::GoFirst => lucide::CHEVRONS_LEFT,
            Freedesktop::GoHome => lucide::HOUSE,
            Freedesktop::GoJump => lucide::CORNER_DOWN_RIGHT,
            Freedesktop::GoLast => lucide::CHEVRONS_RIGHT,
            Freedesktop::GoNext => lucide::ARROW_RIGHT,
            Freedesktop::GoPrevious => lucide::ARROW_LEFT,
            Freedesktop::GoTop => lucide::ARROW_UP_TO_LINE,
            Freedesktop::GoUp => lucide::ARROW_UP,
            Freedesktop::HelpAbout => lucide::INFO,
            Freedesktop::HelpContents => lucide::CIRCLE_QUESTION_MARK,
            Freedesktop::HelpFaq => lucide::MESSAGE_CIRCLE_QUESTION_MARK,
            Freedesktop::InsertImage => lucide::IMAGE_PLUS,
            Freedesktop::InsertLink => lucide::LINK,
            Freedesktop::InsertText => lucide::TYPE,
            Freedesktop::ListAdd => lucide::PLUS,
            Freedesktop::ListRemove => lucide::MINUS,
            Freedesktop::MailForward => lucide::FORWARD,
            Freedesktop::MailMarkImportant => lucide::FLAG,
            Freedesktop::MailMarkRead => lucide::MAIL_OPEN,
            Freedesktop::MailMessageNew => lucide::MAIL_PLUS,
            Freedesktop::MailReplyAll => lucide::REPLY_ALL,
            Freedesktop::MailReplySender => lucide::REPLY,
            Freedesktop::MailSend => lucide::SEND,
            Freedesktop::MediaEject => lucide::DISC,
            Freedesktop::MediaPlaybackPause => lucide::PAUSE,
            Freedesktop::MediaPlaybackStart => lucide::PLAY,
            Freedesktop::MediaPlaybackStop => lucide::SQUARE,
            Freedesktop::MediaRecord => lucide::CIRCLE_DOT,
            Freedesktop::MediaSeekBackward => lucide::REWIND,
            Freedesktop::MediaSeekForward => lucide::FAST_FORWARD,
            Freedesktop::MediaSkipBackward => lucide::SKIP_BACK,
            Freedesktop::MediaSkipForward => lucide::SKIP_FORWARD,
            Freedesktop::ObjectFlipHorizontal => lucide::ARROW_LEFT_RIGHT,
            Freedesktop::ObjectFlipVertical => lucide::ARROW_UP_DOWN,
            Freedesktop::ObjectRotateLeft => lucide::ROTATE_CCW,
            Freedesktop::ObjectRotateRight => lucide::ROTATE_CW,
            Freedesktop::ProcessStop => lucide::OCTAGON_X,
            Freedesktop::SystemLockScreen => lucide::LOCK,
            Freedesktop::SystemLogOut => lucide::LOG_OUT,
            Freedesktop::SystemReboot => lucide::ROTATE_CW,
            Freedesktop::SystemRun => lucide::PLAY,
            Freedesktop::SystemSearch => lucide::SEARCH,
            Freedesktop::SystemShutdown => lucide::POWER,
            Freedesktop::ToolsCheckSpelling => lucide::SPELL_CHECK,
            Freedesktop::ViewFullscreen => lucide::MAXIMIZE,
            Freedesktop::ViewRefresh => lucide::REFRESH_CW,
            Freedesktop::ViewRestore => lucide::MINIMIZE,
            Freedesktop::ViewSortAscending => lucide::ARROW_UP_NARROW_WIDE,
            Freedesktop::ViewSortDescending => lucide::ARROW_DOWN_WIDE_NARROW,
            Freedesktop::WindowClose => lucide::X,
            Freedesktop::WindowNew => lucide::APP_WINDOW,
            Freedesktop::ZoomFitBest => lucide::SCAN,
            Freedesktop::ZoomIn => lucide::ZOOM_IN,
            Freedesktop::ZoomOriginal => lucide::SEARCH,
            Freedesktop::ZoomOut => lucide::ZOOM_OUT,
            Freedesktop::AccessoriesCalculator => lucide::CALCULATOR,
            Freedesktop::AccessoriesTextEditor => lucide::FILE_PEN,
            Freedesktop::HelpBrowser => lucide::BOOK_OPEN,
            Freedesktop::MultimediaVolumeControl => lucide::VOLUME_2,
            Freedesktop::PreferencesDesktopAccessibility => lucide::ACCESSIBILITY,
            Freedesktop::PreferencesDesktopFont => lucide::TYPE,
            Freedesktop::PreferencesDesktopKeyboard => lucide::KEYBOARD,
            Freedesktop::PreferencesDesktopLocale => lucide::LANGUAGES,
            Freedesktop::PreferencesDesktopTheme => lucide::PALETTE,
            Freedesktop::PreferencesSystem => lucide::SETTINGS,
            Freedesktop::SystemFileManager => lucide::FOLDER,
            Freedesktop::UtilitiesTerminal => lucide::TERMINAL,
            Freedesktop::UtilitiesSystemMonitor => lucide::ACTIVITY,
            Freedesktop::ApplicationsGames => lucide::GAMEPAD_2,
            Freedesktop::ApplicationsGraphics => lucide::BRUSH,
            Freedesktop::ApplicationsInternet => lucide::GLOBE,
            Freedesktop::ApplicationsMultimedia => lucide::CLAPPERBOARD,
            Freedesktop::ApplicationsOffice => lucide::BRIEFCASE,
            Freedesktop::ApplicationsScience => lucide::FLASK_CONICAL,
            Freedesktop::ApplicationsUtilities => lucide::WRENCH,
            Freedesktop::ApplicationsDevelopment => lucide::CODE,
            Freedesktop::AudioCard => lucide::AUDIO_LINES,
            Freedesktop::AudioInputMicrophone => lucide::MIC,
            Freedesktop::Battery => lucide::BATTERY,
            Freedesktop::CameraPhoto => lucide::CAMERA,
            Freedesktop::CameraVideo => lucide::VIDEO,
            Freedesktop::CameraWeb => lucide::WEBCAM,
            Freedesktop::Computer => lucide::MONITOR,
            Freedesktop::DriveHarddisk => lucide::HARD_DRIVE,
            Freedesktop::DriveOptical => lucide::DISC,
            Freedesktop::DriveRemovableMedia => lucide::USB,
            Freedesktop::InputGaming => lucide::GAMEPAD,
            Freedesktop::InputKeyboard => lucide::KEYBOARD,
            Freedesktop::InputMouse => lucide::MOUSE,
            Freedesktop::InputTablet => lucide::TABLET,
            Freedesktop::MediaFlash => lucide::MEMORY_STICK,
            Freedesktop::NetworkWired => lucide::ETHERNET_PORT,
            Freedesktop::NetworkWireless => lucide::WIFI,
            Freedesktop::Phone => lucide::SMARTPHONE,
            Freedesktop::Printer => lucide::PRINTER,
            Freedesktop::Scanner => lucide::SCAN,
            Freedesktop::VideoDisplay => lucide::MONITOR,
            Freedesktop::EmblemDefault => lucide::CIRCLE_CHECK,
            Freedesktop::EmblemDocuments => lucide::FILE_TEXT,
            Freedesktop::EmblemFavorite => lucide::HEART,
            Freedesktop::EmblemImportant => lucide::CIRCLE_EXCLAMATION_POINT,
            Freedesktop::EmblemMail => lucide::MAIL,
            Freedesktop::EmblemPhotos => lucide::IMAGE,
            Freedesktop::EmblemReadonly => lucide::LOCK,
            Freedesktop::EmblemShared => lucide::SHARE_2,
            Freedesktop::EmblemSymbolicLink => lucide::LINK,
            Freedesktop::EmblemSystem => lucide::SETTINGS,
            Freedesktop::EmblemUnreadable => lucide::FILE_X,
            Freedesktop::Folder => lucide::FOLDER,
            Freedesktop::FolderRemote => lucide::FOLDER_SYNC,
            Freedesktop::NetworkServer => lucide::SERVER,
            Freedesktop::NetworkWorkgroup => lucide::NETWORK,
            Freedesktop::StartHere => lucide::HOUSE,
            Freedesktop::UserBookmarks => lucide::BOOKMARK,
            Freedesktop::UserDesktop => lucide::MONITOR,
            Freedesktop::UserHome => lucide::HOUSE,
            Freedesktop::UserTrash => lucide::TRASH,
            Freedesktop::AppointmentSoon => lucide::CALENDAR_CLOCK,
            Freedesktop::AudioVolumeHigh => lucide::VOLUME_2,
            Freedesktop::AudioVolumeLow => lucide::VOLUME_1,
            Freedesktop::AudioVolumeMedium => lucide::VOLUME_1,
            Freedesktop::AudioVolumeMuted => lucide::VOLUME_X,
            Freedesktop::BatteryCaution => lucide::BATTERY_EXCLAMATION_POINT,
            Freedesktop::BatteryLow => lucide::BATTERY_LOW,
            Freedesktop::DialogError => lucide::CIRCLE_X,
            Freedesktop::DialogInformation => lucide::INFO,
            Freedesktop::DialogPassword => lucide::KEY_ROUND,
            Freedesktop::DialogQuestion => lucide::CIRCLE_QUESTION_MARK,
            Freedesktop::DialogWarning => lucide::TRIANGLE_EXCLAMATION_POINT,
            Freedesktop::FolderDragAccept => lucide::FOLDER_INPUT,
            Freedesktop::FolderOpen => lucide::FOLDER_OPEN,
            Freedesktop::ImageLoading => lucide::LOADER,
            Freedesktop::ImageMissing => lucide::IMAGE_OFF,
            Freedesktop::MailAttachment => lucide::PAPERCLIP,
            Freedesktop::MailUnread => lucide::MAIL,
            Freedesktop::NetworkError => lucide::WIFI_OFF,
            Freedesktop::NetworkIdle => lucide::WIFI,
            Freedesktop::NetworkOffline => lucide::WIFI_OFF,
            Freedesktop::NetworkReceive => lucide::DOWNLOAD,
            Freedesktop::NetworkTransmit => lucide::UPLOAD,
            Freedesktop::PrinterError => lucide::PRINTER,
            Freedesktop::SoftwareUpdateAvailable => lucide::CIRCLE_ARROW_UP,
            Freedesktop::SoftwareUpdateUrgent => lucide::CIRCLE_EXCLAMATION_POINT,
            Freedesktop::SyncError => lucide::REFRESH_CW_OFF,
            Freedesktop::SyncSynchronizing => lucide::REFRESH_CW,
            Freedesktop::TaskDue => lucide::CLOCK_EXCLAMATION_POINT,
            Freedesktop::UserAvailable => lucide::CIRCLE_CHECK,
            Freedesktop::UserAway => lucide::CLOCK,
            Freedesktop::UserIdle => lucide::MOON,
            Freedesktop::UserOffline => lucide::CIRCLE_MINUS,
            Freedesktop::UserTrashFull => lucide::TRASH,
            Freedesktop::WeatherClear => lucide::SUN,
            Freedesktop::WeatherFewClouds => lucide::CLOUD_SUN,
            Freedesktop::WeatherFog => lucide::CLOUD_FOG,
            Freedesktop::WeatherOvercast => lucide::CLOUD,
            Freedesktop::WeatherSevereAlert => lucide::CLOUD_EXCLAMATION_POINT,
            Freedesktop::WeatherShowers => lucide::CLOUD_RAIN,
            Freedesktop::WeatherSnow => lucide::CLOUD_SNOW,
            Freedesktop::WeatherStorm => lucide::CLOUD_LIGHTNING,
        }
    }

    /// The icon for a specification `name`, such as `document-save`, or
    /// `None` when nothing matches.
    ///
    /// Trailing `-symbolic`, `-rtl` and `-ltr` suffixes are ignored, and a
    /// name without a mapping falls back, as the specification does, to the
    /// name without its last dash-separated part: `folder-documents` to
    /// `folder`.
    pub fn by_name(name: &str) -> Option<Self> {
        let mut current = name;

        while let Some(trimmed) = ["-symbolic", "-rtl", "-ltr"]
            .iter()
            .find_map(|suffix| current.strip_suffix(suffix))
        {
            current = trimmed;
        }

        while !current.is_empty() {
            if let Some(icon) = Self::ALL.iter().find(|icon| icon.name() == current) {
                return Some(*icon);
            }

            current = &current[..current.rfind('-')?];
        }

        None
    }
}
