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
//
// The icon artwork is Lucide 1.70.0: ISC License, Copyright (c) 2026 Lucide
// Icons and Contributors; the icons derived from Feather are also under the
// MIT License, Copyright (c) 2013-present Cole Bemis. The full notices are
// in LICENSE-LUCIDE.

//! Every Lucide 1.70.0 icon, as an [`Icon`] constant named after the icon:
//! `folder-open` is [`FOLDER_OPEN`].
//!
//! This module is generated; regenerate it instead of editing it.

use super::Icon;

/// The Lucide release the icons come from.
pub const VERSION: &str = "1.70.0";

/// Builds the icon `$name` from the elements of its root element.
macro_rules! lucide {
    ($name:literal, $body:literal) => {
        Icon::new(
            $name,
            concat!(
                r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">"#,
                $body,
                "</svg>"
            ),
        )
    };
}

/// `a-arrow-down`.
pub const A_ARROW_DOWN: Icon = lucide!("a-arrow-down", r#"<path d="m14 12 4 4 4-4"/><path d="M18 16V7"/><path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16"/><path d="M3.304 13h6.392"/>"#);

/// `a-arrow-up`.
pub const A_ARROW_UP: Icon = lucide!("a-arrow-up", r#"<path d="m14 11 4-4 4 4"/><path d="M18 16V7"/><path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16"/><path d="M3.304 13h6.392"/>"#);

/// `a-large-small`.
pub const A_LARGE_SMALL: Icon = lucide!("a-large-small", r#"<path d="m15 16 2.536-7.328a1.02 1.02 1 0 1 1.928 0L22 16"/><path d="M15.697 14h5.606"/><path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16"/><path d="M3.304 13h6.392"/>"#);

/// `accessibility`.
pub const ACCESSIBILITY: Icon = lucide!("accessibility", r#"<circle cx="16" cy="4" r="1"/><path d="m18 19 1-7-6 1"/><path d="m5 8 3-3 5.5 3-2.36 3.5"/><path d="M4.24 14.5a5 5 0 0 0 6.88 6"/><path d="M13.76 17.5a5 5 0 0 0-6.88-6"/>"#);

/// `activity`.
pub const ACTIVITY: Icon = lucide!("activity", r#"<path d="M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48 0L9.24 2.18a.25.25 0 0 0-.48 0l-2.35 8.36A2 2 0 0 1 4.49 12H2"/>"#);

/// `ad`.
pub const AD: Icon = lucide!("ad", r#"<path d="M10 13H6"/><path d="M10 15v-4a2 2 0 0 0-4 0v4"/><path d="M14 14.5a.5.5 0 0 0 .5.5h1a2.5 2.5 0 0 0 2.5-2.5v-1A2.5 2.5 0 0 0 15.5 9h-1a.5.5 0 0 0-.5.5z"/><rect x="2" y="5" width="20" height="14" rx="2"/>"#);

/// `air-vent`.
pub const AIR_VENT: Icon = lucide!("air-vent", r#"<path d="M18 17.5a2.5 2.5 0 1 1-4 2.03V12"/><path d="M6 12H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/><path d="M6 8h12"/><path d="M6.6 15.572A2 2 0 1 0 10 17v-5"/>"#);

/// `airplay`.
pub const AIRPLAY: Icon = lucide!("airplay", r#"<path d="M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"/><path d="m12 15 5 6H7Z"/>"#);

/// `alarm-clock`.
pub const ALARM_CLOCK: Icon = lucide!("alarm-clock", r#"<circle cx="12" cy="13" r="8"/><path d="M12 9v4l2 2"/><path d="M5 3 2 6"/><path d="m22 6-3-3"/><path d="M6.38 18.7 4 21"/><path d="M17.64 18.67 20 21"/>"#);

/// `alarm-clock-check`.
pub const ALARM_CLOCK_CHECK: Icon = lucide!("alarm-clock-check", r#"<circle cx="12" cy="13" r="8"/><path d="M5 3 2 6"/><path d="m22 6-3-3"/><path d="M6.38 18.7 4 21"/><path d="M17.64 18.67 20 21"/><path d="m9 13 2 2 4-4"/>"#);

/// `alarm-clock-minus`.
pub const ALARM_CLOCK_MINUS: Icon = lucide!("alarm-clock-minus", r#"<circle cx="12" cy="13" r="8"/><path d="M5 3 2 6"/><path d="m22 6-3-3"/><path d="M6.38 18.7 4 21"/><path d="M17.64 18.67 20 21"/><path d="M9 13h6"/>"#);

/// `alarm-clock-off`.
pub const ALARM_CLOCK_OFF: Icon = lucide!("alarm-clock-off", r#"<path d="M6.87 6.87a8 8 0 1 0 11.26 11.26"/><path d="M19.9 14.25a8 8 0 0 0-9.15-9.15"/><path d="m22 6-3-3"/><path d="M6.26 18.67 4 21"/><path d="m2 2 20 20"/><path d="M4 4 2 6"/>"#);

/// `alarm-clock-plus`.
pub const ALARM_CLOCK_PLUS: Icon = lucide!("alarm-clock-plus", r#"<circle cx="12" cy="13" r="8"/><path d="M5 3 2 6"/><path d="m22 6-3-3"/><path d="M6.38 18.7 4 21"/><path d="M17.64 18.67 20 21"/><path d="M12 10v6"/><path d="M9 13h6"/>"#);

/// `alarm-smoke`.
pub const ALARM_SMOKE: Icon = lucide!("alarm-smoke", r#"<path d="M11 21c0-2.5 2-2.5 2-5"/><path d="M16 21c0-2.5 2-2.5 2-5"/><path d="m19 8-.8 3a1.25 1.25 0 0 1-1.2 1H7a1.25 1.25 0 0 1-1.2-1L5 8"/><path d="M21 3a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a1 1 0 0 1 1-1z"/><path d="M6 21c0-2.5 2-2.5 2-5"/>"#);

/// `align-center-horizontal`.
pub const ALIGN_CENTER_HORIZONTAL: Icon = lucide!("align-center-horizontal", r#"<path d="M2 12h20"/><path d="M10 16v4a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-4"/><path d="M10 8V4a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v4"/><path d="M20 16v1a2 2 0 0 1-2 2h-2a2 2 0 0 1-2-2v-1"/><path d="M14 8V7c0-1.1.9-2 2-2h2a2 2 0 0 1 2 2v1"/>"#);

/// `align-center-vertical`.
pub const ALIGN_CENTER_VERTICAL: Icon = lucide!("align-center-vertical", r#"<path d="M12 2v20"/><path d="M8 10H4a2 2 0 0 1-2-2V6c0-1.1.9-2 2-2h4"/><path d="M16 10h4a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-4"/><path d="M8 20H7a2 2 0 0 1-2-2v-2c0-1.1.9-2 2-2h1"/><path d="M16 14h1a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-1"/>"#);

/// `align-end-horizontal`.
pub const ALIGN_END_HORIZONTAL: Icon = lucide!("align-end-horizontal", r#"<rect width="6" height="16" x="4" y="2" rx="2"/><rect width="6" height="9" x="14" y="9" rx="2"/><path d="M22 22H2"/>"#);

/// `align-end-vertical`.
pub const ALIGN_END_VERTICAL: Icon = lucide!("align-end-vertical", r#"<rect width="16" height="6" x="2" y="4" rx="2"/><rect width="9" height="6" x="9" y="14" rx="2"/><path d="M22 22V2"/>"#);

/// `align-horizontal-distribute-center`.
pub const ALIGN_HORIZONTAL_DISTRIBUTE_CENTER: Icon = lucide!("align-horizontal-distribute-center", r#"<rect width="6" height="14" x="4" y="5" rx="2"/><rect width="6" height="10" x="14" y="7" rx="2"/><path d="M17 22v-5"/><path d="M17 7V2"/><path d="M7 22v-3"/><path d="M7 5V2"/>"#);

/// `align-horizontal-distribute-end`.
pub const ALIGN_HORIZONTAL_DISTRIBUTE_END: Icon = lucide!("align-horizontal-distribute-end", r#"<rect width="6" height="14" x="4" y="5" rx="2"/><rect width="6" height="10" x="14" y="7" rx="2"/><path d="M10 2v20"/><path d="M20 2v20"/>"#);

/// `align-horizontal-distribute-start`.
pub const ALIGN_HORIZONTAL_DISTRIBUTE_START: Icon = lucide!("align-horizontal-distribute-start", r#"<rect width="6" height="14" x="4" y="5" rx="2"/><rect width="6" height="10" x="14" y="7" rx="2"/><path d="M4 2v20"/><path d="M14 2v20"/>"#);

/// `align-horizontal-justify-center`.
pub const ALIGN_HORIZONTAL_JUSTIFY_CENTER: Icon = lucide!("align-horizontal-justify-center", r#"<rect width="6" height="14" x="2" y="5" rx="2"/><rect width="6" height="10" x="16" y="7" rx="2"/><path d="M12 2v20"/>"#);

/// `align-horizontal-justify-end`.
pub const ALIGN_HORIZONTAL_JUSTIFY_END: Icon = lucide!("align-horizontal-justify-end", r#"<rect width="6" height="14" x="2" y="5" rx="2"/><rect width="6" height="10" x="12" y="7" rx="2"/><path d="M22 2v20"/>"#);

/// `align-horizontal-justify-start`.
pub const ALIGN_HORIZONTAL_JUSTIFY_START: Icon = lucide!("align-horizontal-justify-start", r#"<rect width="6" height="14" x="6" y="5" rx="2"/><rect width="6" height="10" x="16" y="7" rx="2"/><path d="M2 2v20"/>"#);

/// `align-horizontal-space-around`.
pub const ALIGN_HORIZONTAL_SPACE_AROUND: Icon = lucide!("align-horizontal-space-around", r#"<rect width="6" height="10" x="9" y="7" rx="2"/><path d="M4 22V2"/><path d="M20 22V2"/>"#);

/// `align-horizontal-space-between`.
pub const ALIGN_HORIZONTAL_SPACE_BETWEEN: Icon = lucide!("align-horizontal-space-between", r#"<rect width="6" height="14" x="3" y="5" rx="2"/><rect width="6" height="10" x="15" y="7" rx="2"/><path d="M3 2v20"/><path d="M21 2v20"/>"#);

/// `align-start-horizontal`.
pub const ALIGN_START_HORIZONTAL: Icon = lucide!("align-start-horizontal", r#"<rect width="6" height="16" x="4" y="6" rx="2"/><rect width="6" height="9" x="14" y="6" rx="2"/><path d="M22 2H2"/>"#);

/// `align-start-vertical`.
pub const ALIGN_START_VERTICAL: Icon = lucide!("align-start-vertical", r#"<rect width="9" height="6" x="6" y="14" rx="2"/><rect width="16" height="6" x="6" y="4" rx="2"/><path d="M2 2v20"/>"#);

/// `align-vertical-distribute-center`.
pub const ALIGN_VERTICAL_DISTRIBUTE_CENTER: Icon = lucide!("align-vertical-distribute-center", r#"<path d="M22 17h-3"/><path d="M22 7h-5"/><path d="M5 17H2"/><path d="M7 7H2"/><rect x="5" y="14" width="14" height="6" rx="2"/><rect x="7" y="4" width="10" height="6" rx="2"/>"#);

/// `align-vertical-distribute-end`.
pub const ALIGN_VERTICAL_DISTRIBUTE_END: Icon = lucide!("align-vertical-distribute-end", r#"<rect width="14" height="6" x="5" y="14" rx="2"/><rect width="10" height="6" x="7" y="4" rx="2"/><path d="M2 20h20"/><path d="M2 10h20"/>"#);

/// `align-vertical-distribute-start`.
pub const ALIGN_VERTICAL_DISTRIBUTE_START: Icon = lucide!("align-vertical-distribute-start", r#"<rect width="14" height="6" x="5" y="14" rx="2"/><rect width="10" height="6" x="7" y="4" rx="2"/><path d="M2 14h20"/><path d="M2 4h20"/>"#);

/// `align-vertical-justify-center`.
pub const ALIGN_VERTICAL_JUSTIFY_CENTER: Icon = lucide!("align-vertical-justify-center", r#"<rect width="14" height="6" x="5" y="16" rx="2"/><rect width="10" height="6" x="7" y="2" rx="2"/><path d="M2 12h20"/>"#);

/// `align-vertical-justify-end`.
pub const ALIGN_VERTICAL_JUSTIFY_END: Icon = lucide!("align-vertical-justify-end", r#"<rect width="14" height="6" x="5" y="12" rx="2"/><rect width="10" height="6" x="7" y="2" rx="2"/><path d="M2 22h20"/>"#);

/// `align-vertical-justify-start`.
pub const ALIGN_VERTICAL_JUSTIFY_START: Icon = lucide!("align-vertical-justify-start", r#"<rect width="14" height="6" x="5" y="16" rx="2"/><rect width="10" height="6" x="7" y="6" rx="2"/><path d="M2 2h20"/>"#);

/// `align-vertical-space-around`.
pub const ALIGN_VERTICAL_SPACE_AROUND: Icon = lucide!("align-vertical-space-around", r#"<rect width="10" height="6" x="7" y="9" rx="2"/><path d="M22 20H2"/><path d="M22 4H2"/>"#);

/// `align-vertical-space-between`.
pub const ALIGN_VERTICAL_SPACE_BETWEEN: Icon = lucide!("align-vertical-space-between", r#"<rect width="14" height="6" x="5" y="15" rx="2"/><rect width="10" height="6" x="7" y="3" rx="2"/><path d="M2 21h20"/><path d="M2 3h20"/>"#);

/// `ambulance`.
pub const AMBULANCE: Icon = lucide!("ambulance", r#"<path d="M10 10H6"/><path d="M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2"/><path d="M19 18h2a1 1 0 0 0 1-1v-3.28a1 1 0 0 0-.684-.948l-1.923-.641a1 1 0 0 1-.578-.502l-1.539-3.076A1 1 0 0 0 16.382 8H14"/><path d="M8 8v4"/><path d="M9 18h6"/><circle cx="17" cy="18" r="2"/><circle cx="7" cy="18" r="2"/>"#);

/// `ampersand`.
pub const AMPERSAND: Icon = lucide!("ampersand", r#"<path d="M16 12h3"/><path d="M17.5 12a8 8 0 0 1-8 8A4.5 4.5 0 0 1 5 15.5c0-6 8-4 8-8.5a3 3 0 1 0-6 0c0 3 2.5 8.5 12 13"/>"#);

/// `ampersands`.
pub const AMPERSANDS: Icon = lucide!("ampersands", r#"<path d="M10 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"/><path d="M22 17c-5-3-7-7-7-9a2 2 0 0 1 4 0c0 2.5-5 2.5-5 6 0 1.7 1.3 3 3 3 2.8 0 5-2.2 5-5"/>"#);

/// `amphora`.
pub const AMPHORA: Icon = lucide!("amphora", r#"<path d="M10 2v5.632c0 .424-.272.795-.653.982A6 6 0 0 0 6 14c.006 4 3 7 5 8"/><path d="M10 5H8a2 2 0 0 0 0 4h.68"/><path d="M14 2v5.632c0 .424.272.795.652.982A6 6 0 0 1 18 14c0 4-3 7-5 8"/><path d="M14 5h2a2 2 0 0 1 0 4h-.68"/><path d="M18 22H6"/><path d="M9 2h6"/>"#);

/// `anchor`.
pub const ANCHOR: Icon = lucide!("anchor", r#"<path d="M12 6v16"/><path d="m19 13 2-1a9 9 0 0 1-18 0l2 1"/><path d="M9 11h6"/><circle cx="12" cy="4" r="2"/>"#);

/// `angle`.
pub const ANGLE: Icon = lucide!("angle", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M3 11a10 10 0 0 1 10 10"/>"#);

/// `antenna`.
pub const ANTENNA: Icon = lucide!("antenna", r#"<path d="M2 12 7 2"/><path d="m7 12 5-10"/><path d="m12 12 5-10"/><path d="m17 12 5-10"/><path d="M4.5 7h15"/><path d="M12 16v6"/>"#);

/// `anvil`.
pub const ANVIL: Icon = lucide!("anvil", r#"<path d="M7 10H6a4 4 0 0 1-4-4 1 1 0 0 1 1-1h4"/><path d="M7 5a1 1 0 0 1 1-1h13a1 1 0 0 1 1 1 7 7 0 0 1-7 7H8a1 1 0 0 1-1-1z"/><path d="M9 12v5"/><path d="M15 12v5"/><path d="M5 20a3 3 0 0 1 3-3h8a3 3 0 0 1 3 3 1 1 0 0 1-1 1H6a1 1 0 0 1-1-1"/>"#);

/// `aperture`.
pub const APERTURE: Icon = lucide!("aperture", r#"<circle cx="12" cy="12" r="10"/><path d="m14.31 8 5.74 9.94"/><path d="M9.69 8h11.48"/><path d="m7.38 12 5.74-9.94"/><path d="M9.69 16 3.95 6.06"/><path d="M14.31 16H2.83"/><path d="m16.62 12-5.74 9.94"/>"#);

/// `app-window`.
pub const APP_WINDOW: Icon = lucide!("app-window", r#"<rect x="2" y="4" width="20" height="16" rx="2"/><path d="M10 4v4"/><path d="M2 8h20"/><path d="M6 4v4"/>"#);

/// `app-window-mac`.
pub const APP_WINDOW_MAC: Icon = lucide!("app-window-mac", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.01"/><path d="M10 8h.01"/><path d="M14 8h.01"/>"#);

/// `apple`.
pub const APPLE: Icon = lucide!("apple", r#"<path d="M12 6.528V3a1 1 0 0 1 1-1h0"/><path d="M18.237 21A15 15 0 0 0 22 11a6 6 0 0 0-10-4.472A6 6 0 0 0 2 11a15.1 15.1 0 0 0 3.763 10 3 3 0 0 0 3.648.648 5.5 5.5 0 0 1 5.178 0A3 3 0 0 0 18.237 21"/>"#);

/// `archive`.
pub const ARCHIVE: Icon = lucide!("archive", r#"<rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"/><path d="M10 12h4"/>"#);

/// `archive-arrow-down`.
pub const ARCHIVE_ARROW_DOWN: Icon = lucide!("archive-arrow-down", r#"<rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h.343"/><path d="M20 8v11a2 2 0 0 1-2 2h-.343"/><path d="m9 18 3 3 3-3"/><path d="M12 12v9"/>"#);

/// `archive-arrow-up`.
pub const ARCHIVE_ARROW_UP: Icon = lucide!("archive-arrow-up", r#"<rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h2"/><path d="M20 8v11a2 2 0 0 1-2 2h-2"/><path d="m9 15 3-3 3 3"/><path d="M12 12v9"/>"#);

/// `archive-x`.
pub const ARCHIVE_X: Icon = lucide!("archive-x", r#"<rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"/><path d="m9.5 17 5-5"/><path d="m9.5 12 5 5"/>"#);

/// `armchair`.
pub const ARMCHAIR: Icon = lucide!("armchair", r#"<path d="M19 9V6a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2v3"/><path d="M3 16a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v1.5a.5.5 0 0 1-.5.5h-9a.5.5 0 0 1-.5-.5V11a2 2 0 0 0-4 0z"/><path d="M5 18v2"/><path d="M19 18v2"/>"#);

/// `armenian-dram`.
pub const ARMENIAN_DRAM: Icon = lucide!("armenian-dram", r#"<path d="M11 10h8"/><path d="M11 14h8"/><path d="M17 20V10a6 6 0 0 0-12 0"/>"#);

/// `arrow-big-down`.
pub const ARROW_BIG_DOWN: Icon = lucide!("arrow-big-down", r#"<path d="M9 5a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v6a1 1 0 0 0 1 1h3.293a.707.707 0 0 1 .5 1.207l-7.086 7.086a1 1 0 0 1-1.414 0l-7.086-7.086a.707.707 0 0 1 .5-1.207H8a1 1 0 0 0 1-1z"/>"#);

/// `arrow-big-down-dash`.
pub const ARROW_BIG_DOWN_DASH: Icon = lucide!("arrow-big-down-dash", r#"<path d="M14 8a1 1 0 0 1 1 1v2a1 1 0 0 0 1 1h3.293a.707.707 0 0 1 .5 1.207l-6.939 6.939a1.207 1.207 0 0 1-1.708 0l-6.94-6.94a.707.707 0 0 1 .5-1.206H8a1 1 0 0 0 1-1V9a1 1 0 0 1 1-1z"/><path d="M9 4h6"/>"#);

/// `arrow-big-left`.
pub const ARROW_BIG_LEFT: Icon = lucide!("arrow-big-left", r#"<path d="M10.793 19.793a.707.707 0 0 0 1.207-.5V16a1 1 0 0 1 1-1h6a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1h-6a1 1 0 0 1-1-1V4.707a.707.707 0 0 0-1.207-.5l-6.94 6.94a1.207 1.207 0 0 0 0 1.707z"/>"#);

/// `arrow-big-left-dash`.
pub const ARROW_BIG_LEFT_DASH: Icon = lucide!("arrow-big-left-dash", r#"<path d="M13 9a1 1 0 0 1-1-1V4.707a.707.707 0 0 0-1.207-.5l-6.94 6.94a1.207 1.207 0 0 0 0 1.707l6.94 6.94a.707.707 0 0 0 1.207-.5V16a1 1 0 0 1 1-1h2a1 1 0 0 0 1-1v-4a1 1 0 0 0-1-1z"/><path d="M20 9v6"/>"#);

/// `arrow-big-right`.
pub const ARROW_BIG_RIGHT: Icon = lucide!("arrow-big-right", r#"<path d="M13.207 19.793a.707.707 0 0 1-1.207-.5V16a1 1 0 0 0-1-1H5a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1h6a1 1 0 0 0 1-1V4.707a.707.707 0 0 1 1.207-.5l6.94 6.94a1.207 1.207 0 0 1 0 1.707z"/>"#);

/// `arrow-big-right-dash`.
pub const ARROW_BIG_RIGHT_DASH: Icon = lucide!("arrow-big-right-dash", r#"<path d="M11 9a1 1 0 0 0 1-1V4.707a.707.707 0 0 1 1.207-.5l6.94 6.94a1.207 1.207 0 0 1 0 1.707l-6.94 6.94a.707.707 0 0 1-1.207-.5V16a1 1 0 0 0-1-1H9a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1z"/><path d="M4 9v6"/>"#);

/// `arrow-big-up`.
pub const ARROW_BIG_UP: Icon = lucide!("arrow-big-up", r#"<path d="M9 19a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1v-6a1 1 0 0 1 1-1h3.293a.707.707 0 0 0 .5-1.207l-7.086-7.086a1 1 0 0 0-1.414 0l-7.086 7.086a.707.707 0 0 0 .5 1.207H8a1 1 0 0 1 1 1z"/>"#);

/// `arrow-big-up-dash`.
pub const ARROW_BIG_UP_DASH: Icon = lucide!("arrow-big-up-dash", r#"<path d="M14 16a1 1 0 0 0 1-1v-2a1 1 0 0 1 1-1h3.293a.707.707 0 0 0 .5-1.207l-6.939-6.939a1.207 1.207 0 0 0-1.708 0l-6.94 6.94a.707.707 0 0 0 .5 1.206H8a1 1 0 0 1 1 1v2a1 1 0 0 0 1 1z"/><path d="M9 20h6"/>"#);

/// `arrow-down`.
pub const ARROW_DOWN: Icon = lucide!("arrow-down", r#"<path d="M12 5v14"/><path d="m19 12-7 7-7-7"/>"#);

/// `arrow-down-0-1`.
pub const ARROW_DOWN_0_1: Icon = lucide!("arrow-down-0-1", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><rect x="15" y="4" width="4" height="6" ry="2"/><path d="M17 20v-6h-2"/><path d="M15 20h4"/>"#);

/// `arrow-down-1-0`.
pub const ARROW_DOWN_1_0: Icon = lucide!("arrow-down-1-0", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><path d="M17 10V4h-2"/><path d="M15 10h4"/><rect x="15" y="14" width="4" height="6" ry="2"/>"#);

/// `arrow-down-a-z`.
pub const ARROW_DOWN_A_Z: Icon = lucide!("arrow-down-a-z", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><path d="M20 8h-5"/><path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"/><path d="M15 14h5l-5 6h5"/>"#);

/// `arrow-down-from-line`.
pub const ARROW_DOWN_FROM_LINE: Icon = lucide!("arrow-down-from-line", r#"<path d="M19 3H5"/><path d="M12 21V7"/><path d="m6 15 6 6 6-6"/>"#);

/// `arrow-down-left`.
pub const ARROW_DOWN_LEFT: Icon = lucide!("arrow-down-left", r#"<path d="M17 7 7 17"/><path d="M17 17H7V7"/>"#);

/// `arrow-down-narrow-wide`.
pub const ARROW_DOWN_NARROW_WIDE: Icon = lucide!("arrow-down-narrow-wide", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><path d="M11 4h4"/><path d="M11 8h7"/><path d="M11 12h10"/>"#);

/// `arrow-down-right`.
pub const ARROW_DOWN_RIGHT: Icon = lucide!("arrow-down-right", r#"<path d="m7 7 10 10"/><path d="M17 7v10H7"/>"#);

/// `arrow-down-to-dot`.
pub const ARROW_DOWN_TO_DOT: Icon = lucide!("arrow-down-to-dot", r#"<path d="M12 2v14"/><path d="m19 9-7 7-7-7"/><circle cx="12" cy="21" r="1"/>"#);

/// `arrow-down-to-line`.
pub const ARROW_DOWN_TO_LINE: Icon = lucide!("arrow-down-to-line", r#"<path d="M12 17V3"/><path d="m6 11 6 6 6-6"/><path d="M19 21H5"/>"#);

/// `arrow-down-up`.
pub const ARROW_DOWN_UP: Icon = lucide!("arrow-down-up", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><path d="m21 8-4-4-4 4"/><path d="M17 4v16"/>"#);

/// `arrow-down-wide-narrow`.
pub const ARROW_DOWN_WIDE_NARROW: Icon = lucide!("arrow-down-wide-narrow", r#"<path d="m3 16 4 4 4-4"/><path d="M7 20V4"/><path d="M11 4h10"/><path d="M11 8h7"/><path d="M11 12h4"/>"#);

/// `arrow-down-z-a`.
pub const ARROW_DOWN_Z_A: Icon = lucide!("arrow-down-z-a", r#"<path d="m3 16 4 4 4-4"/><path d="M7 4v16"/><path d="M15 4h5l-5 6h5"/><path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"/><path d="M20 18h-5"/>"#);

/// `arrow-left`.
pub const ARROW_LEFT: Icon = lucide!("arrow-left", r#"<path d="m12 19-7-7 7-7"/><path d="M19 12H5"/>"#);

/// `arrow-left-from-line`.
pub const ARROW_LEFT_FROM_LINE: Icon = lucide!("arrow-left-from-line", r#"<path d="m9 6-6 6 6 6"/><path d="M3 12h14"/><path d="M21 19V5"/>"#);

/// `arrow-left-right`.
pub const ARROW_LEFT_RIGHT: Icon = lucide!("arrow-left-right", r#"<path d="M8 3 4 7l4 4"/><path d="M4 7h16"/><path d="m16 21 4-4-4-4"/><path d="M20 17H4"/>"#);

/// `arrow-left-to-line`.
pub const ARROW_LEFT_TO_LINE: Icon = lucide!("arrow-left-to-line", r#"<path d="M3 19V5"/><path d="m13 6-6 6 6 6"/><path d="M7 12h14"/>"#);

/// `arrow-right`.
pub const ARROW_RIGHT: Icon = lucide!("arrow-right", r#"<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>"#);

/// `arrow-right-from-line`.
pub const ARROW_RIGHT_FROM_LINE: Icon = lucide!("arrow-right-from-line", r#"<path d="M3 5v14"/><path d="M21 12H7"/><path d="m15 18 6-6-6-6"/>"#);

/// `arrow-right-left`.
pub const ARROW_RIGHT_LEFT: Icon = lucide!("arrow-right-left", r#"<path d="m16 3 4 4-4 4"/><path d="M20 7H4"/><path d="m8 21-4-4 4-4"/><path d="M4 17h16"/>"#);

/// `arrow-right-to-line`.
pub const ARROW_RIGHT_TO_LINE: Icon = lucide!("arrow-right-to-line", r#"<path d="M17 12H3"/><path d="m11 18 6-6-6-6"/><path d="M21 5v14"/>"#);

/// `arrow-up`.
pub const ARROW_UP: Icon = lucide!("arrow-up", r#"<path d="m5 12 7-7 7 7"/><path d="M12 19V5"/>"#);

/// `arrow-up-0-1`.
pub const ARROW_UP_0_1: Icon = lucide!("arrow-up-0-1", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><rect x="15" y="4" width="4" height="6" ry="2"/><path d="M17 20v-6h-2"/><path d="M15 20h4"/>"#);

/// `arrow-up-1-0`.
pub const ARROW_UP_1_0: Icon = lucide!("arrow-up-1-0", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><path d="M17 10V4h-2"/><path d="M15 10h4"/><rect x="15" y="14" width="4" height="6" ry="2"/>"#);

/// `arrow-up-a-z`.
pub const ARROW_UP_A_Z: Icon = lucide!("arrow-up-a-z", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><path d="M20 8h-5"/><path d="M15 10V6.5a2.5 2.5 0 0 1 5 0V10"/><path d="M15 14h5l-5 6h5"/>"#);

/// `arrow-up-down`.
pub const ARROW_UP_DOWN: Icon = lucide!("arrow-up-down", r#"<path d="m21 16-4 4-4-4"/><path d="M17 20V4"/><path d="m3 8 4-4 4 4"/><path d="M7 4v16"/>"#);

/// `arrow-up-from-dot`.
pub const ARROW_UP_FROM_DOT: Icon = lucide!("arrow-up-from-dot", r#"<path d="m5 9 7-7 7 7"/><path d="M12 16V2"/><circle cx="12" cy="21" r="1"/>"#);

/// `arrow-up-from-line`.
pub const ARROW_UP_FROM_LINE: Icon = lucide!("arrow-up-from-line", r#"<path d="m18 9-6-6-6 6"/><path d="M12 3v14"/><path d="M5 21h14"/>"#);

/// `arrow-up-left`.
pub const ARROW_UP_LEFT: Icon = lucide!("arrow-up-left", r#"<path d="M7 17V7h10"/><path d="M17 17 7 7"/>"#);

/// `arrow-up-narrow-wide`.
pub const ARROW_UP_NARROW_WIDE: Icon = lucide!("arrow-up-narrow-wide", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><path d="M11 12h4"/><path d="M11 16h7"/><path d="M11 20h10"/>"#);

/// `arrow-up-right`.
pub const ARROW_UP_RIGHT: Icon = lucide!("arrow-up-right", r#"<path d="M7 7h10v10"/><path d="M7 17 17 7"/>"#);

/// `arrow-up-to-line`.
pub const ARROW_UP_TO_LINE: Icon = lucide!("arrow-up-to-line", r#"<path d="M5 3h14"/><path d="m18 13-6-6-6 6"/><path d="M12 7v14"/>"#);

/// `arrow-up-wide-narrow`.
pub const ARROW_UP_WIDE_NARROW: Icon = lucide!("arrow-up-wide-narrow", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><path d="M11 12h10"/><path d="M11 16h7"/><path d="M11 20h4"/>"#);

/// `arrow-up-z-a`.
pub const ARROW_UP_Z_A: Icon = lucide!("arrow-up-z-a", r#"<path d="m3 8 4-4 4 4"/><path d="M7 4v16"/><path d="M15 4h5l-5 6h5"/><path d="M15 20v-3.5a2.5 2.5 0 0 1 5 0V20"/><path d="M20 18h-5"/>"#);

/// `arrows-up-from-line`.
pub const ARROWS_UP_FROM_LINE: Icon = lucide!("arrows-up-from-line", r#"<path d="m4 6 3-3 3 3"/><path d="M7 17V3"/><path d="m14 6 3-3 3 3"/><path d="M17 17V3"/><path d="M4 21h16"/>"#);

/// `asterisk`.
pub const ASTERISK: Icon = lucide!("asterisk", r#"<path d="M12 5v14"/><path d="m18.065 8.496-12.125 7"/><path d="m5.94 8.504 12.125 7"/>"#);

/// `astroid`.
pub const ASTROID: Icon = lucide!("astroid", r#"<path d="M12.983 21.186a1 1 0 0 1-1.966 0 10 10 0 0 0-8.203-8.203 1 1 0 0 1 0-1.966 10 10 0 0 0 8.203-8.203 1 1 0 0 1 1.966 0 10 10 0 0 0 8.203 8.203 1 1 0 0 1 0 1.966 10 10 0 0 0-8.203 8.203"/>"#);

/// `at-sign`.
pub const AT_SIGN: Icon = lucide!("at-sign", r#"<circle cx="12" cy="12" r="4"/><path d="M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-4 8"/>"#);

/// `atom`.
pub const ATOM: Icon = lucide!("atom", r#"<circle cx="12" cy="12" r="1"/><path d="M20.2 20.2c2.04-2.03.02-7.36-4.5-11.9-4.54-4.52-9.87-6.54-11.9-4.5-2.04 2.03-.02 7.36 4.5 11.9 4.54 4.52 9.87 6.54 11.9 4.5Z"/><path d="M15.7 15.7c4.52-4.54 6.54-9.87 4.5-11.9-2.03-2.04-7.36-.02-11.9 4.5-4.52 4.54-6.54 9.87-4.5 11.9 2.03 2.04 7.36.02 11.9-4.5Z"/>"#);

/// `audio-lines`.
pub const AUDIO_LINES: Icon = lucide!("audio-lines", r#"<path d="M2 10v3"/><path d="M6 6v11"/><path d="M10 3v18"/><path d="M14 8v7"/><path d="M18 5v13"/><path d="M22 10v3"/>"#);

/// `audio-lines-off`.
pub const AUDIO_LINES_OFF: Icon = lucide!("audio-lines-off", r#"<path d="M10 10v11"/><path d="M10 3v1.35"/><path d="M14 14v1"/><path d="M14 8v.35"/><path d="M18 5v7.35"/><path d="M2 10v3"/><path d="m2 2 20 20"/><path d="M22 10v3"/><path d="M6 6v11"/>"#);

/// `audio-lines-x`.
pub const AUDIO_LINES_X: Icon = lucide!("audio-lines-x", r#"<path d="M10 3v18"/><path d="M14 8v6.35"/><path d="m17 17 5 5"/><path d="M18 5v8.1"/><path d="M2 10v3"/><path d="M22 10v3"/><path d="m22 17-5 5"/><path d="M6 6v11"/>"#);

/// `audio-waveform`.
pub const AUDIO_WAVEFORM: Icon = lucide!("audio-waveform", r#"<path d="M2 13a2 2 0 0 0 2-2V7a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0V4a2 2 0 0 1 4 0v13a2 2 0 0 0 4 0v-4a2 2 0 0 1 2-2"/>"#);

/// `avocado`.
pub const AVOCADO: Icon = lucide!("avocado", r#"<path d="M13.465 3.464a5 5 0 017.07 0 5 5 0 010 7.072c-1.204 1.203-2.38 2.89-2.614 4.577A8 8 0 118.886 6.08c1.687-.235 3.375-1.41 4.579-2.615"/><circle cx="10" cy="14" r="3"/>"#);

/// `award`.
pub const AWARD: Icon = lucide!("award", r#"<path d="m15.477 12.89 1.515 8.526a.5.5 0 0 1-.81.47l-3.58-2.687a1 1 0 0 0-1.197 0l-3.586 2.686a.5.5 0 0 1-.81-.469l1.514-8.526"/><circle cx="12" cy="8" r="6"/>"#);

/// `axe`.
pub const AXE: Icon = lucide!("axe", r#"<path d="m14 12-8.381 8.38a1 1 0 0 1-3.001-3L11 9"/><path d="M15 15.5a.5.5 0 0 0 .5.5A6.5 6.5 0 0 0 22 9.5a.5.5 0 0 0-.5-.5h-1.672a2 2 0 0 1-1.414-.586l-5.062-5.062a1.205 1.205 0 0 0-1.704 0L9.352 5.648a1.205 1.205 0 0 0 0 1.704l5.062 5.062A2 2 0 0 1 15 13.828z"/>"#);

/// `axis-3d`.
pub const AXIS_3D: Icon = lucide!("axis-3d", r#"<path d="M13.5 10.5 15 9"/><path d="M4 4v15a1 1 0 0 0 1 1h15"/><path d="M4.293 19.707 6 18"/><path d="m9 15 1.5-1.5"/>"#);

/// `azerbaijani-manat`.
pub const AZERBAIJANI_MANAT: Icon = lucide!("azerbaijani-manat", r#"<path d="M12 4v16"/><path d="M18 20v-8a6 6 0 00-12 0v8"/>"#);

/// `baby`.
pub const BABY: Icon = lucide!("baby", r#"<path d="M10 16c.5.3 1.2.5 2 .5s1.5-.2 2-.5"/><path d="M15 12h.01"/><path d="M19.38 6.813A9 9 0 0 1 20.8 10.2a2 2 0 0 1 0 3.6 9 9 0 0 1-17.6 0 2 2 0 0 1 0-3.6A9 9 0 0 1 12 3c2 0 3.5 1.1 3.5 2.5s-.9 2.5-2 2.5c-.8 0-1.5-.4-1.5-1"/><path d="M9 12h.01"/>"#);

/// `backpack`.
pub const BACKPACK: Icon = lucide!("backpack", r#"<path d="M4 10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2z"/><path d="M8 10h8"/><path d="M8 18h8"/><path d="M8 22v-6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v6"/><path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"/>"#);

/// `bacon`.
pub const BACON: Icon = lucide!("bacon", r#"<path d="M2.795 16.804c-.789-.79-1.09-2.018-.442-2.928 1.933-2.713 4.414-1.05 6.145-5.378 1.868-4.67 4.608-2.366 6.592-6.107.218-.412.78-.53 1.11-.2l5.005 5.005c.789.79 1.09 2.018.442 2.928-1.933 2.713-4.414 1.05-6.145 5.378-1.868 4.67-4.608 2.366-6.591 6.107-.22.412-.78.53-1.11.2z"/><path d="M4.5 18.5c2-5 5-2 7-7s5-2 7-7"/>"#);

/// `badge`.
pub const BADGE: Icon = lucide!("badge", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/>"#);

/// `badge-cent`.
pub const BADGE_CENT: Icon = lucide!("badge-cent", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M12 7v10"/><path d="M15.4 10a4 4 0 1 0 0 4"/>"#);

/// `badge-check`.
pub const BADGE_CHECK: Icon = lucide!("badge-check", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m16 9-5.5 5.5L8 12"/>"#);

/// `badge-dollar-sign`.
pub const BADGE_DOLLAR_SIGN: Icon = lucide!("badge-dollar-sign", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/>"#);

/// `badge-euro`.
pub const BADGE_EURO: Icon = lucide!("badge-euro", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M7 12h5"/><path d="M15 9.4a4 4 0 1 0 0 5.2"/>"#);

/// `badge-exclamation-point`.
pub const BADGE_EXCLAMATION_POINT: Icon = lucide!("badge-exclamation-point", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#);

/// `badge-indian-rupee`.
pub const BADGE_INDIAN_RUPEE: Icon = lucide!("badge-indian-rupee", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M8 8h8"/><path d="M8 12h8"/><path d="m13 17-5-1h1a4 4 0 0 0 0-8"/>"#);

/// `badge-info`.
pub const BADGE_INFO: Icon = lucide!("badge-info", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><line x1="12" x2="12" y1="16" y2="12"/><line x1="12" x2="12.01" y1="8" y2="8"/>"#);

/// `badge-japanese-yen`.
pub const BADGE_JAPANESE_YEN: Icon = lucide!("badge-japanese-yen", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 8 3 3v7"/><path d="m12 11 3-3"/><path d="M9 12h6"/><path d="M9 16h6"/>"#);

/// `badge-minus`.
pub const BADGE_MINUS: Icon = lucide!("badge-minus", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><line x1="8" x2="16" y1="12" y2="12"/>"#);

/// `badge-percent`.
pub const BADGE_PERCENT: Icon = lucide!("badge-percent", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m15 9-6 6"/><path d="M9 9h.01"/><path d="M15 15h.01"/>"#);

/// `badge-plus`.
pub const BADGE_PLUS: Icon = lucide!("badge-plus", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><line x1="12" x2="12" y1="8" y2="16"/><line x1="8" x2="16" y1="12" y2="12"/>"#);

/// `badge-pound-sterling`.
pub const BADGE_POUND_STERLING: Icon = lucide!("badge-pound-sterling", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M8 12h4"/><path d="M10 16V9.5a2.5 2.5 0 0 1 5 0"/><path d="M8 16h7"/>"#);

/// `badge-question-mark`.
pub const BADGE_QUESTION_MARK: Icon = lucide!("badge-question-mark", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" x2="12.01" y1="17" y2="17"/>"#);

/// `badge-russian-ruble`.
pub const BADGE_RUSSIAN_RUBLE: Icon = lucide!("badge-russian-ruble", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M9 16h5"/><path d="M9 12h5a2 2 0 1 0 0-4h-3v9"/>"#);

/// `badge-swiss-franc`.
pub const BADGE_SWISS_FRANC: Icon = lucide!("badge-swiss-franc", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="M11 17V8h4"/><path d="M11 12h3"/><path d="M9 16h4"/>"#);

/// `badge-turkish-lira`.
pub const BADGE_TURKISH_LIRA: Icon = lucide!("badge-turkish-lira", r#"<path d="M11 7v10a5 5 0 0 0 5-5"/><path d="m15 8-6 3"/><path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76"/>"#);

/// `badge-x`.
pub const BADGE_X: Icon = lucide!("badge-x", r#"<path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><line x1="15" x2="9" y1="9" y2="15"/><line x1="9" x2="15" y1="9" y2="15"/>"#);

/// `baggage-claim`.
pub const BAGGAGE_CLAIM: Icon = lucide!("baggage-claim", r#"<path d="M22 18H6a2 2 0 0 1-2-2V7a2 2 0 0 0-2-2"/><path d="M17 14V4a2 2 0 0 0-2-2h-1a2 2 0 0 0-2 2v10"/><rect width="13" height="8" x="8" y="6" rx="1"/><circle cx="18" cy="20" r="2"/><circle cx="9" cy="20" r="2"/>"#);

/// `ball-wave`.
pub const BALL_WAVE: Icon = lucide!("ball-wave", r#"<path d="M2 20c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="M7.757 13.243a2 6 45 1 0 8.486-8.486 2 6 45 1 0-8.486 8.486"/><circle cx="12" cy="9" r="6"/>"#);

/// `balloon`.
pub const BALLOON: Icon = lucide!("balloon", r#"<path d="M12 16v1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v1"/><path d="M12 6a2 2 0 0 1 2 2"/><path d="M18 8c0 4-3.5 8-6 8s-6-4-6-8a6 6 0 0 1 12 0"/>"#);

/// `ban`.
pub const BAN: Icon = lucide!("ban", r#"<circle cx="12" cy="12" r="10"/><path d="M4.929 4.929 19.07 19.071"/>"#);

/// `banana`.
pub const BANANA: Icon = lucide!("banana", r#"<path d="M4 13c3.5-2 8-2 10 2a5.5 5.5 0 0 1 8 5"/><path d="M5.15 17.89c5.52-1.52 8.65-6.89 7-12C11.55 4 11.5 2 13 2c3.22 0 5 5.5 5 8 0 6.5-4.2 12-10.49 12C5.11 22 2 22 2 20c0-1.5 1.14-1.55 3.15-2.11Z"/>"#);

/// `bandage`.
pub const BANDAGE: Icon = lucide!("bandage", r#"<path d="M10 10.01h.01"/><path d="M10 14.01h.01"/><path d="M14 10.01h.01"/><path d="M14 14.01h.01"/><path d="M18 6v12"/><path d="M6 6v12"/><rect x="2" y="6" width="20" height="12" rx="2"/>"#);

/// `bangladeshi-taka`.
pub const BANGLADESHI_TAKA: Icon = lucide!("bangladeshi-taka", r#"<path d="M6 5a2 2 0 0 1 4 0v12a4 4 0 0 0 8 0 2 2 0 0 0-4 0"/><path d="M6 9h12"/>"#);

/// `banknote`.
pub const BANKNOTE: Icon = lucide!("banknote", r#"<rect width="20" height="12" x="2" y="6" rx="2"/><circle cx="12" cy="12" r="2"/><path d="M6 12h.01M18 12h.01"/>"#);

/// `banknote-arrow-down`.
pub const BANKNOTE_ARROW_DOWN: Icon = lucide!("banknote-arrow-down", r#"<path d="M12 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5"/><path d="m16 19 3 3 3-3"/><path d="M18 12h.01"/><path d="M19 16v6"/><path d="M6 12h.01"/><circle cx="12" cy="12" r="2"/>"#);

/// `banknote-arrow-up`.
pub const BANKNOTE_ARROW_UP: Icon = lucide!("banknote-arrow-up", r#"<path d="M12 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5"/><path d="M18 12h.01"/><path d="M19 22v-6"/><path d="m22 19-3-3-3 3"/><path d="M6 12h.01"/><circle cx="12" cy="12" r="2"/>"#);

/// `banknote-check`.
pub const BANKNOTE_CHECK: Icon = lucide!("banknote-check", r#"<path d="M11.748 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4.875"/><path d="m16 19 2 2 4-4"/><path d="M18 12h.01"/><path d="M6 12h.01"/><circle cx="12" cy="12" r="2"/>"#);

/// `banknote-x`.
pub const BANKNOTE_X: Icon = lucide!("banknote-x", r#"<path d="M13 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5"/><path d="m17 17 5 5"/><path d="M18 12h.01"/><path d="m22 17-5 5"/><path d="M6 12h.01"/><circle cx="12" cy="12" r="2"/>"#);

/// `barcode`.
pub const BARCODE: Icon = lucide!("barcode", r#"<path d="M3 5v14"/><path d="M8 5v14"/><path d="M12 5v14"/><path d="M17 5v14"/><path d="M21 5v14"/>"#);

/// `barrel`.
pub const BARREL: Icon = lucide!("barrel", r#"<path d="M10.5 2a34.5 34.5 0 000 20"/><path d="M13.5 2a34.5 34.5 0 010 20"/><path d="M4.67 17h14.67"/><path d="M4.67 7h14.67"/><path d="M6.23 3.06a19 19 0 000 17.88A2 2 0 008 22h8a2 2 0 001.77-1.06 19 19 0 000-17.88A2 2 0 0016 2H8a2 2 0 00-1.77 1.06"/>"#);

/// `baseline`.
pub const BASELINE: Icon = lucide!("baseline", r#"<path d="M4 20h16"/><path d="m6 16 6-12 6 12"/><path d="M8 12h8"/>"#);

/// `bath`.
pub const BATH: Icon = lucide!("bath", r#"<path d="M10 4 8 6"/><path d="M17 19v2"/><path d="M2 12h20"/><path d="M7 19v2"/><path d="M9 5 7.621 3.621A2.121 2.121 0 0 0 4 5v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5"/>"#);

/// `battery`.
pub const BATTERY: Icon = lucide!("battery", r#"<path d="M 22 14 L 22 10"/><rect x="2" y="6" width="16" height="12" rx="2"/>"#);

/// `battery-charging`.
pub const BATTERY_CHARGING: Icon = lucide!("battery-charging", r#"<path d="m11 7-3 5h4l-3 5"/><path d="M14.856 6H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.935"/><path d="M22 14v-4"/><path d="M5.14 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2.936"/>"#);

/// `battery-exclamation-point`.
pub const BATTERY_EXCLAMATION_POINT: Icon = lucide!("battery-exclamation-point", r#"<path d="M10 17h.01"/><path d="M10 7v6"/><path d="M14 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2"/><path d="M22 14v-4"/><path d="M6 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2"/>"#);

/// `battery-full`.
pub const BATTERY_FULL: Icon = lucide!("battery-full", r#"<path d="M10 10v4"/><path d="M14 10v4"/><path d="M22 14v-4"/><path d="M6 10v4"/><rect x="2" y="6" width="16" height="12" rx="2"/>"#);

/// `battery-low`.
pub const BATTERY_LOW: Icon = lucide!("battery-low", r#"<path d="M22 14v-4"/><path d="M6 14v-4"/><rect x="2" y="6" width="16" height="12" rx="2"/>"#);

/// `battery-medium`.
pub const BATTERY_MEDIUM: Icon = lucide!("battery-medium", r#"<path d="M10 14v-4"/><path d="M22 14v-4"/><path d="M6 14v-4"/><rect x="2" y="6" width="16" height="12" rx="2"/>"#);

/// `battery-plus`.
pub const BATTERY_PLUS: Icon = lucide!("battery-plus", r#"<path d="M10 9v6"/><path d="M12.543 6H16a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.605"/><path d="M22 14v-4"/><path d="M7 12h6"/><path d="M7.606 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.606"/>"#);

/// `beaker`.
pub const BEAKER: Icon = lucide!("beaker", r#"<path d="M4.5 3h15"/><path d="M6 3v16a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V3"/><path d="M6 14h12"/>"#);

/// `bean`.
pub const BEAN: Icon = lucide!("bean", r#"<path d="M10.165 6.598C9.954 7.478 9.64 8.36 9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22c7.732 0 14-6.268 14-14a6 6 0 0 0-11.835-1.402Z"/><path d="M5.341 10.62a4 4 0 1 0 5.279-5.28"/>"#);

/// `bean-off`.
pub const BEAN_OFF: Icon = lucide!("bean-off", r#"<path d="M9 9c-.64.64-1.521.954-2.402 1.165A6 6 0 0 0 8 22a13.96 13.96 0 0 0 9.9-4.1"/><path d="M10.75 5.093A6 6 0 0 1 22 8c0 2.411-.61 4.68-1.683 6.66"/><path d="M5.341 10.62a4 4 0 0 0 6.487 1.208M10.62 5.341a4.015 4.015 0 0 1 2.039 2.04"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `bed`.
pub const BED: Icon = lucide!("bed", r#"<path d="M2 4v16"/><path d="M2 8h18a2 2 0 0 1 2 2v10"/><path d="M2 17h20"/><path d="M6 8v9"/>"#);

/// `bed-double`.
pub const BED_DOUBLE: Icon = lucide!("bed-double", r#"<path d="M2 20v-8a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v8"/><path d="M4 10V6a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"/><path d="M12 4v6"/><path d="M2 18h20"/>"#);

/// `bed-single`.
pub const BED_SINGLE: Icon = lucide!("bed-single", r#"<path d="M3 20v-8a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v8"/><path d="M5 10V6a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v4"/><path d="M3 18h18"/>"#);

/// `beef`.
pub const BEEF: Icon = lucide!("beef", r#"<path d="M16.4 13.7A6.5 6.5 0 1 0 6.28 6.6c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c4 0 8.4-1.8 11.4-4.3"/><path d="m18.5 6 1.754 3.5a6.48 6.48 0 0 1-1.854 8.2C15.4 20.2 11 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5"/><circle cx="12.5" cy="8.5" r="2.5"/>"#);

/// `beef-off`.
pub const BEEF_OFF: Icon = lucide!("beef-off", r#"<path d="M11.771 6.109a2.5 2.5 0 0 1 3.12 3.12"/><path d="M17.852 12.185a6.5 6.5 0 0 0-9.035-9.04"/><path d="M18.013 18.013C15.029 20.349 10.831 22 7 22a3 3 0 0 1-2.68-1.66L2.4 16.5"/><path d="m18.5 6 2.19 4.5a6.48 6.48 0 0 1-.139 4.393"/><path d="m2 2 20 20"/><path d="M6.355 6.37a7 7 0 0 0-.075.23c-1.1 3.13-.78 3.9-3.18 6.08A3 3 0 0 0 5 18c3.356 0 6.993-1.267 9.85-3.151"/>"#);

/// `beer`.
pub const BEER: Icon = lucide!("beer", r#"<path d="M17 11h1a3 3 0 0 1 0 6h-1"/><path d="M9 12v6"/><path d="M13 12v6"/><path d="M14 7.5c-1 0-1.44.5-3 .5s-2-.5-3-.5-1.72.5-2.5.5a2.5 2.5 0 0 1 0-5c.78 0 1.57.5 2.5.5S9.44 2 11 2s2 1.5 3 1.5 1.72-.5 2.5-.5a2.5 2.5 0 0 1 0 5c-.78 0-1.5-.5-2.5-.5Z"/><path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8"/>"#);

/// `beer-off`.
pub const BEER_OFF: Icon = lucide!("beer-off", r#"<path d="M13 13v5"/><path d="M17 11.47V8"/><path d="M17 11h1a3 3 0 0 1 2.745 4.211"/><path d="m2 2 20 20"/><path d="M5 8v12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-3"/><path d="M7.536 7.535C6.766 7.649 6.154 8 5.5 8a2.5 2.5 0 0 1-1.768-4.268"/><path d="M8.727 3.204C9.306 2.767 9.885 2 11 2c1.56 0 2 1.5 3 1.5s1.72-.5 2.5-.5a1 1 0 1 1 0 5c-.78 0-1.5-.5-2.5-.5a3.149 3.149 0 0 0-.842.12"/><path d="M9 14.6V18"/>"#);

/// `belarusian-ruble`.
pub const BELARUSIAN_RUBLE: Icon = lucide!("belarusian-ruble", r#"<path d="M3 16 L13 16"/><path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7"/>"#);

/// `bell`.
pub const BELL: Icon = lucide!("bell", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/>"#);

/// `bell-check`.
pub const BELL_CHECK: Icon = lucide!("bell-check", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="m15 8 2 2 4-4"/><path d="M16.8607 4.4824A6 6 0 0 0 6 8C6 12.499 4.589 13.956 3.262 15.326"/><path d="M3.262 15.326A1 1 0 0 0 4 17H20A1 1 0 0 0 20.74 15.327C20.209 14.779 19.665 14.218 19.203 13.454"/>"#);

/// `bell-dot`.
pub const BELL_DOT: Icon = lucide!("bell-dot", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M11.68 2.009A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673c-.824-.85-1.678-1.731-2.21-3.348"/><circle cx="18" cy="5" r="3"/>"#);

/// `bell-electric`.
pub const BELL_ELECTRIC: Icon = lucide!("bell-electric", r#"<path d="M18.518 17.347A7 7 0 0 1 14 19"/><path d="M18.8 4A11 11 0 0 1 20 9"/><path d="M9 9h.01"/><circle cx="20" cy="16" r="2"/><circle cx="9" cy="9" r="7"/><rect x="4" y="16" width="10" height="6" rx="2"/>"#);

/// `bell-minus`.
pub const BELL_MINUS: Icon = lucide!("bell-minus", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M15 8h6"/><path d="M16.243 3.757A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673A9.4 9.4 0 0 1 18.667 12"/>"#);

/// `bell-off`.
pub const BELL_OFF: Icon = lucide!("bell-off", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M17 17H4a1 1 0 0 1-.74-1.673C4.59 13.956 6 12.499 6 8a6 6 0 0 1 .258-1.742"/><path d="m2 2 20 20"/><path d="M8.668 3.01A6 6 0 0 1 18 8c0 2.687.77 4.653 1.707 6.05"/>"#);

/// `bell-plus`.
pub const BELL_PLUS: Icon = lucide!("bell-plus", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M15 8h6"/><path d="M18 5v6"/><path d="M20.002 14.464a9 9 0 0 0 .738.863A1 1 0 0 1 20 17H4a1 1 0 0 1-.74-1.673C4.59 13.956 6 12.499 6 8a6 6 0 0 1 8.75-5.332"/>"#);

/// `bell-ring`.
pub const BELL_RING: Icon = lucide!("bell-ring", r#"<path d="M10.268 21a2 2 0 0 0 3.464 0"/><path d="M22 8c0-2.3-.8-4.3-2-6"/><path d="M3.262 15.326A1 1 0 0 0 4 17h16a1 1 0 0 0 .74-1.673C19.41 13.956 18 12.499 18 8A6 6 0 0 0 6 8c0 4.499-1.411 5.956-2.738 7.326"/><path d="M4 2C2.8 3.7 2 5.7 2 8"/>"#);

/// `between-horizontal-end`.
pub const BETWEEN_HORIZONTAL_END: Icon = lucide!("between-horizontal-end", r#"<rect width="13" height="7" x="3" y="3" rx="1"/><path d="m22 15-3-3 3-3"/><rect width="13" height="7" x="3" y="14" rx="1"/>"#);

/// `between-horizontal-start`.
pub const BETWEEN_HORIZONTAL_START: Icon = lucide!("between-horizontal-start", r#"<rect width="13" height="7" x="8" y="3" rx="1"/><path d="m2 9 3 3-3 3"/><rect width="13" height="7" x="8" y="14" rx="1"/>"#);

/// `between-vertical-end`.
pub const BETWEEN_VERTICAL_END: Icon = lucide!("between-vertical-end", r#"<rect width="7" height="13" x="3" y="3" rx="1"/><path d="m9 22 3-3 3 3"/><rect width="7" height="13" x="14" y="3" rx="1"/>"#);

/// `between-vertical-start`.
pub const BETWEEN_VERTICAL_START: Icon = lucide!("between-vertical-start", r#"<rect width="7" height="13" x="3" y="8" rx="1"/><path d="m15 2-3 3-3-3"/><rect width="7" height="13" x="14" y="8" rx="1"/>"#);

/// `biceps-flexed`.
pub const BICEPS_FLEXED: Icon = lucide!("biceps-flexed", r#"<path d="M12.409 13.017A5 5 0 0 1 22 15c0 3.866-4 7-9 7-4.077 0-8.153-.82-10.371-2.462-.426-.316-.631-.832-.62-1.362C2.118 12.723 2.627 2 10 2a3 3 0 0 1 3 3 2 2 0 0 1-2 2c-1.105 0-1.64-.444-2-1"/><path d="M15 14a5 5 0 0 0-7.584 2"/><path d="M9.964 6.825C8.019 7.977 9.5 13 8 15"/>"#);

/// `bike`.
pub const BIKE: Icon = lucide!("bike", r#"<circle cx="18.5" cy="17.5" r="3.5"/><circle cx="5.5" cy="17.5" r="3.5"/><circle cx="15" cy="5" r="1"/><path d="M12 17.5V14l-3-3 4-3 2 3h2"/>"#);

/// `binary`.
pub const BINARY: Icon = lucide!("binary", r#"<rect x="14" y="14" width="4" height="6" rx="2"/><rect x="6" y="4" width="4" height="6" rx="2"/><path d="M6 20h4"/><path d="M14 10h4"/><path d="M6 14h2v6"/><path d="M14 4h2v6"/>"#);

/// `binoculars`.
pub const BINOCULARS: Icon = lucide!("binoculars", r#"<path d="M10 10h4"/><path d="M19 7V4a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3"/><path d="M20 21a2 2 0 0 0 2-2v-3.851c0-1.39-2-2.962-2-4.829V8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v11a2 2 0 0 0 2 2z"/><path d="M 22 16 L 2 16"/><path d="M4 21a2 2 0 0 1-2-2v-3.851c0-1.39 2-2.962 2-4.829V8a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v11a2 2 0 0 1-2 2z"/><path d="M9 7V4a1 1 0 0 0-1-1H6a1 1 0 0 0-1 1v3"/>"#);

/// `biohazard`.
pub const BIOHAZARD: Icon = lucide!("biohazard", r#"<circle cx="12" cy="11.9" r="2"/><path d="M6.7 3.4c-.9 2.5 0 5.2 2.2 6.7C6.5 9 3.7 9.6 2 11.6"/><path d="m8.9 10.1 1.4.8"/><path d="M17.3 3.4c.9 2.5 0 5.2-2.2 6.7 2.4-1.2 5.2-.6 6.9 1.5"/><path d="m15.1 10.1-1.4.8"/><path d="M16.7 20.8c-2.6-.4-4.6-2.6-4.7-5.3-.2 2.6-2.1 4.8-4.7 5.2"/><path d="M12 13.9v1.6"/><path d="M13.5 5.4c-1-.2-2-.2-3 0"/><path d="M17 16.4c.7-.7 1.2-1.6 1.5-2.5"/><path d="M5.5 13.9c.3.9.8 1.8 1.5 2.5"/>"#);

/// `bird`.
pub const BIRD: Icon = lucide!("bird", r#"<path d="M16 7h.01"/><path d="M3.4 18H12a8 8 0 0 0 8-8V7a4 4 0 0 0-7.28-2.3L2 20"/><path d="m20 7 2 .5-2 .5"/><path d="M10 18v3"/><path d="M14 17.75V21"/><path d="M7 18a6 6 0 0 0 3.84-10.61"/>"#);

/// `birdhouse`.
pub const BIRDHOUSE: Icon = lucide!("birdhouse", r#"<path d="M12 18v4"/><path d="m17 18 1.956-11.468"/><path d="m3 8 7.82-5.615a2 2 0 0 1 2.36 0L21 8"/><path d="M4 18h16"/><path d="M7 18 5.044 6.532"/><circle cx="12" cy="10" r="2"/>"#);

/// `bitcoin`.
pub const BITCOIN: Icon = lucide!("bitcoin", r#"<path d="M11.767 19.089c4.924.868 6.14-6.025 1.216-6.894m-1.216 6.894L5.86 18.047m5.908 1.042-.347 1.97m1.563-8.864c4.924.869 6.14-6.025 1.215-6.893m-1.215 6.893-3.94-.694m5.155-6.2L8.29 4.26m5.908 1.042.348-1.97M7.48 20.364l3.126-17.727"/>"#);

/// `blend`.
pub const BLEND: Icon = lucide!("blend", r#"<circle cx="15" cy="9" r="7"/><circle cx="9" cy="15" r="7"/>"#);

/// `blender`.
pub const BLENDER: Icon = lucide!("blender", r#"<path d="M8 14a2 2 0 0 0-1.963 1.615l-1.018 5.193A1 1 0 0 0 6 22h12a1 1 0 0 0 .981-1.192l-1.018-5.193A2 2 0 0 0 16 14z"/><path d="m17 2-1 12"/><path d="M8.006 14 7 2"/><path d="M7.565 8.787A5 5 0 0 0 12 8a5 5 0 0 1 4.56-.75"/><path d="M19 2H5a2 2 0 0 0-2 2v5a2 2 0 0 0 .688 1.5"/><path d="M12 18h.01"/>"#);

/// `blinds`.
pub const BLINDS: Icon = lucide!("blinds", r#"<path d="M3 3h18"/><path d="M20 7H8"/><path d="M20 11H8"/><path d="M10 19h10"/><path d="M8 15h12"/><path d="M4 3v14"/><circle cx="4" cy="19" r="2"/>"#);

/// `blocks`.
pub const BLOCKS: Icon = lucide!("blocks", r#"<path d="M10 22V7a1 1 0 0 0-1-1H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-5a1 1 0 0 0-1-1H2"/><rect x="14" y="2" width="8" height="8" rx="1"/>"#);

/// `bluetooth`.
pub const BLUETOOTH: Icon = lucide!("bluetooth", r#"<path d="m7 7 10 10-5 5V2l5 5L7 17"/>"#);

/// `bluetooth-connected`.
pub const BLUETOOTH_CONNECTED: Icon = lucide!("bluetooth-connected", r#"<path d="m7 7 10 10-5 5V2l5 5L7 17"/><line x1="18" x2="21" y1="12" y2="12"/><line x1="3" x2="6" y1="12" y2="12"/>"#);

/// `bluetooth-off`.
pub const BLUETOOTH_OFF: Icon = lucide!("bluetooth-off", r#"<path d="m17 17-5 5V12l-5 5"/><path d="m2 2 20 20"/><path d="M14.5 9.5 17 7l-5-5v4.5"/>"#);

/// `bluetooth-searching`.
pub const BLUETOOTH_SEARCHING: Icon = lucide!("bluetooth-searching", r#"<path d="m7 7 10 10-5 5V2l5 5L7 17"/><path d="M20.83 14.83a4 4 0 0 0 0-5.66"/><path d="M18 12h.01"/>"#);

/// `bolt`.
pub const BOLT: Icon = lucide!("bolt", r#"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><circle cx="12" cy="12" r="4"/>"#);

/// `bomb`.
pub const BOMB: Icon = lucide!("bomb", r#"<circle cx="11" cy="13" r="9"/><path d="M14.35 4.65 16.3 2.7a2.41 2.41 0 0 1 3.4 0l1.6 1.6a2.4 2.4 0 0 1 0 3.4l-1.95 1.95"/><path d="m22 2-1.5 1.5"/>"#);

/// `bone`.
pub const BONE: Icon = lucide!("bone", r#"<path d="M17 10c.7-.7 1.69 0 2.5 0a2.5 2.5 0 1 0 0-5 .5.5 0 0 1-.5-.5 2.5 2.5 0 1 0-5 0c0 .81.7 1.8 0 2.5l-7 7c-.7.7-1.69 0-2.5 0a2.5 2.5 0 0 0 0 5c.28 0 .5.22.5.5a2.5 2.5 0 1 0 5 0c0-.81-.7-1.8 0-2.5Z"/>"#);

/// `bone-fracture`.
pub const BONE_FRACTURE: Icon = lucide!("bone-fracture", r#"<path d="M14 4.5a1 1 0 0 1 5 0 .5.5 0 0 0 .5.5 1 1 0 0 1 0 5c-.81 0-1.8-.7-2.5 0l-1.958 1.957a.15.15 0 0 1-.252-.072l-.493-2.07a.15.15 0 0 0-.111-.112l-2.072-.494a.15.15 0 0 1-.072-.252L14 7c.7-.7 0-1.69 0-2.5"/><path d="m16 20-1-2"/><path d="m20 16-2-1"/><path d="m4 8 2 1"/><path d="m8 4 1 2"/><path d="M9.698 14.19a.15.15 0 0 0 .112.112l2.074.489a.15.15 0 0 1 .072.252L10 17c-.7.7 0 1.69 0 2.5a1 1 0 0 1-5 0 .495.495 0 0 0-.5-.5 1 1 0 0 1 0-5c.81 0 1.8.7 2.5 0l1.956-1.957a.15.15 0 0 1 .252.072z"/>"#);

/// `book`.
pub const BOOK: Icon = lucide!("book", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/>"#);

/// `book-a`.
pub const BOOK_A: Icon = lucide!("book-a", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="m8 13 4-7 4 7"/><path d="M9.1 11h5.7"/>"#);

/// `book-audio`.
pub const BOOK_AUDIO: Icon = lucide!("book-audio", r#"<path d="M12 6v7"/><path d="M16 8v3"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M8 8v3"/>"#);

/// `book-bookmark`.
pub const BOOK_BOOKMARK: Icon = lucide!("book-bookmark", r#"<path d="M10 2v7.751a.25.25 0 00.407.195l2.28-1.834a.5.5 0 01.627 0l2.28 1.834A.25.25 0 0016 9.751V2"/><path d="M4 19.5v-15A2.5 2.5 0 016.5 2H19a1 1 0 011 1v18a1 1 0 01-1 1H6.5a1 1 0 010-5H20"/>"#);

/// `book-check`.
pub const BOOK_CHECK: Icon = lucide!("book-check", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="m9 9.5 2 2 4-4"/>"#);

/// `book-copy`.
pub const BOOK_COPY: Icon = lucide!("book-copy", r#"<path d="M5 7a2 2 0 0 0-2 2v11"/><path d="M5.803 18H5a2 2 0 0 0 0 4h9.5a.5.5 0 0 0 .5-.5V21"/><path d="M9 15V4a2 2 0 0 1 2-2h9.5a.5.5 0 0 1 .5.5v14a.5.5 0 0 1-.5.5H11a2 2 0 0 1 0-4h10"/>"#);

/// `book-dashed`.
pub const BOOK_DASHED: Icon = lucide!("book-dashed", r#"<path d="M12 17h1.5"/><path d="M12 22h1.5"/><path d="M12 2h1.5"/><path d="M17.5 22H19a1 1 0 0 0 1-1"/><path d="M17.5 2H19a1 1 0 0 1 1 1v1.5"/><path d="M20 14v3h-2.5"/><path d="M20 8.5V10"/><path d="M4 10V8.5"/><path d="M4 19.5V14"/><path d="M4 4.5A2.5 2.5 0 0 1 6.5 2H8"/><path d="M8 22H6.5a1 1 0 0 1 0-5H8"/>"#);

/// `book-down`.
pub const BOOK_DOWN: Icon = lucide!("book-down", r#"<path d="M12 13V7"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="m9 10 3 3 3-3"/>"#);

/// `book-exclamation-point`.
pub const BOOK_EXCLAMATION_POINT: Icon = lucide!("book-exclamation-point", r#"<path d="M12 13h.01"/><path d="M12 6v3"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/>"#);

/// `book-headphones`.
pub const BOOK_HEADPHONES: Icon = lucide!("book-headphones", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M8 12v-2a4 4 0 0 1 8 0v2"/><circle cx="15" cy="12" r="1"/><circle cx="9" cy="12" r="1"/>"#);

/// `book-heart`.
pub const BOOK_HEART: Icon = lucide!("book-heart", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M8.62 9.8A2.25 2.25 0 1 1 12 6.836a2.25 2.25 0 1 1 3.38 2.966l-2.626 2.856a.998.998 0 0 1-1.507 0z"/>"#);

/// `book-image`.
pub const BOOK_IMAGE: Icon = lucide!("book-image", r#"<path d="m20 13.7-2.1-2.1a2 2 0 0 0-2.8 0L9.7 17"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><circle cx="10" cy="8" r="2"/>"#);

/// `book-key`.
pub const BOOK_KEY: Icon = lucide!("book-key", r#"<path d="M13 2H6.5A2.5 2.5 0 0 0 4 4.5v15"/><path d="M17 2v6"/><path d="M17 4h2"/><path d="M20 15.2V21a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><circle cx="17" cy="10" r="2"/>"#);

/// `book-lock`.
pub const BOOK_LOCK: Icon = lucide!("book-lock", r#"<path d="M18 6V4a2 2 0 1 0-4 0v2"/><path d="M20 15v6a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10"/><rect x="12" y="6" width="8" height="5" rx="1"/>"#);

/// `book-minus`.
pub const BOOK_MINUS: Icon = lucide!("book-minus", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M9 10h6"/>"#);

/// `book-open`.
pub const BOOK_OPEN: Icon = lucide!("book-open", r#"<path d="M12 5v16"/><path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z"/>"#);

/// `book-open-check`.
pub const BOOK_OPEN_CHECK: Icon = lucide!("book-open-check", r#"<path d="M12 5v16"/><path d="m16 12 2 2 4-4"/><path d="M22 6V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2h4.001A2 2 0 0022 17v-1.344"/>"#);

/// `book-open-text`.
pub const BOOK_OPEN_TEXT: Icon = lucide!("book-open-text", r#"<path d="M12 5v16"/><path d="M16 13h2"/><path d="M16 9h2"/><path d="M20.001 19A2 2 0 0022 17V5a2 2 0 00-1.999-2L16 3.002A5 5 0 0012 5a5 5 0 00-4-2H4a2 2 0 00-2 2v12a2 2 0 001.999 2H8a5 5 0 014 2 5 5 0 014-2z"/><path d="M6 13h2"/><path d="M6 9h2"/>"#);

/// `book-plus`.
pub const BOOK_PLUS: Icon = lucide!("book-plus", r#"<path d="M12 7v6"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M9 10h6"/>"#);

/// `book-search`.
pub const BOOK_SEARCH: Icon = lucide!("book-search", r#"<path d="M11 22H5.5a1 1 0 0 1 0-5h4.501"/><path d="m21 22-1.879-1.878"/><path d="M3 19.5v-15A2.5 2.5 0 0 1 5.5 2H18a1 1 0 0 1 1 1v8"/><circle cx="17" cy="18" r="3"/>"#);

/// `book-text`.
pub const BOOK_TEXT: Icon = lucide!("book-text", r#"<path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M8 11h8"/><path d="M8 7h6"/>"#);

/// `book-type`.
pub const BOOK_TYPE: Icon = lucide!("book-type", r#"<path d="M10 13h4"/><path d="M12 6v7"/><path d="M16 8V6H8v2"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/>"#);

/// `book-up`.
pub const BOOK_UP: Icon = lucide!("book-up", r#"<path d="M12 13V7"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="m9 10 3-3 3 3"/>"#);

/// `book-up-2`.
pub const BOOK_UP_2: Icon = lucide!("book-up-2", r#"<path d="M12 13V7"/><path d="M18 2h1a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2"/><path d="m9 10 3-3 3 3"/><path d="m9 5 3-3 3 3"/>"#);

/// `book-user`.
pub const BOOK_USER: Icon = lucide!("book-user", r#"<path d="M15 13a3 3 0 1 0-6 0"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><circle cx="12" cy="8" r="2"/>"#);

/// `book-x`.
pub const BOOK_X: Icon = lucide!("book-x", r#"<path d="m14.5 7.5-5 5"/><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20"/><path d="m9.5 7.5 5 5"/>"#);

/// `bookmark`.
pub const BOOKMARK: Icon = lucide!("bookmark", r#"<path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/>"#);

/// `bookmark-check`.
pub const BOOKMARK_CHECK: Icon = lucide!("bookmark-check", r#"<path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/><path d="m9 10 2 2 4-4"/>"#);

/// `bookmark-minus`.
pub const BOOKMARK_MINUS: Icon = lucide!("bookmark-minus", r#"<path d="M15 10H9"/><path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/>"#);

/// `bookmark-off`.
pub const BOOKMARK_OFF: Icon = lucide!("bookmark-off", r#"<path d="M19 19v1a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5"/><path d="m2 2 20 20"/><path d="M8.656 3H17a2 2 0 0 1 2 2v8.344"/>"#);

/// `bookmark-plus`.
pub const BOOKMARK_PLUS: Icon = lucide!("bookmark-plus", r#"<path d="M12 7v6"/><path d="M15 10H9"/><path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/>"#);

/// `bookmark-x`.
pub const BOOKMARK_X: Icon = lucide!("bookmark-x", r#"<path d="m14.5 7.5-5 5"/><path d="M17 3a2 2 0 0 1 2 2v15a1 1 0 0 1-1.496.868l-4.512-2.578a2 2 0 0 0-1.984 0l-4.512 2.578A1 1 0 0 1 5 20V5a2 2 0 0 1 2-2z"/><path d="m9.5 7.5 5 5"/>"#);

/// `boom-box`.
pub const BOOM_BOX: Icon = lucide!("boom-box", r#"<path d="M4 9V5a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v4"/><path d="M8 8v1"/><path d="M12 8v1"/><path d="M16 8v1"/><rect width="20" height="12" x="2" y="9" rx="2"/><circle cx="8" cy="15" r="2"/><circle cx="16" cy="15" r="2"/>"#);

/// `bot`.
pub const BOT: Icon = lucide!("bot", r#"<path d="M12 8V4H8"/><rect width="16" height="12" x="4" y="8" rx="2"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M15 13v2"/><path d="M9 13v2"/>"#);

/// `bot-message-square`.
pub const BOT_MESSAGE_SQUARE: Icon = lucide!("bot-message-square", r#"<path d="M12 6V2H8"/><path d="M15 11v2"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="M20 16a2 2 0 0 1-2 2H8.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 4 20.286V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2z"/><path d="M9 11v2"/>"#);

/// `bot-off`.
pub const BOT_OFF: Icon = lucide!("bot-off", r#"<path d="M13.67 8H18a2 2 0 0 1 2 2v4.33"/><path d="M2 14h2"/><path d="M20 14h2"/><path d="M22 22 2 2"/><path d="M8 8H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 1.414-.586"/><path d="M9 13v2"/><path d="M9.67 4H12v2.33"/>"#);

/// `bottle-wine`.
pub const BOTTLE_WINE: Icon = lucide!("bottle-wine", r#"<path d="M10 3a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v2a6 6 0 0 0 1.2 3.6l.6.8A6 6 0 0 1 17 13v8a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1v-8a6 6 0 0 1 1.2-3.6l.6-.8A6 6 0 0 0 10 5z"/><path d="M17 13h-4a1 1 0 0 0-1 1v3a1 1 0 0 0 1 1h4"/>"#);

/// `bow-arrow`.
pub const BOW_ARROW: Icon = lucide!("bow-arrow", r#"<path d="M17 3h4v4"/><path d="M18.575 11.082a13 13 0 0 1 1.048 9.027 1.17 1.17 0 0 1-1.914.597L14 17"/><path d="M7 10 3.29 6.29a1.17 1.17 0 0 1 .6-1.91 13 13 0 0 1 9.03 1.05"/><path d="M7 14a1.7 1.7 0 0 0-1.207.5l-2.646 2.646A.5.5 0 0 0 3.5 18H5a1 1 0 0 1 1 1v1.5a.5.5 0 0 0 .854.354L9.5 18.207A1.7 1.7 0 0 0 10 17v-2a1 1 0 0 0-1-1z"/><path d="M9.707 14.293 21 3"/>"#);

/// `box`.
pub const BOX: Icon = lucide!("box", r#"<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/>"#);

/// `boxes`.
pub const BOXES: Icon = lucide!("boxes", r#"<path d="M2.97 12.92A2 2 0 0 0 2 14.63v3.24a2 2 0 0 0 .97 1.71l3 1.8a2 2 0 0 0 2.06 0L12 19v-5.5l-5-3-4.03 2.42Z"/><path d="m7 16.5-4.74-2.85"/><path d="m7 16.5 5-3"/><path d="M7 16.5v5.17"/><path d="M12 13.5V19l3.97 2.38a2 2 0 0 0 2.06 0l3-1.8a2 2 0 0 0 .97-1.71v-3.24a2 2 0 0 0-.97-1.71L17 10.5l-5 3Z"/><path d="m17 16.5-5-3"/><path d="m17 16.5 4.74-2.85"/><path d="M17 16.5v5.17"/><path d="M7.97 4.42A2 2 0 0 0 7 6.13v4.37l5 3 5-3V6.13a2 2 0 0 0-.97-1.71l-3-1.8a2 2 0 0 0-2.06 0l-3 1.8Z"/><path d="M12 8 7.26 5.15"/><path d="m12 8 4.74-2.85"/><path d="M12 13.5V8"/>"#);

/// `boxes-linked`.
pub const BOXES_LINKED: Icon = lucide!("boxes-linked", r#"<path d="M14 6h2a2 2 0 0 1 2 2v5"/><path d="M14.21 15.202 18 17.4V22"/><path d="M14.796 14.611A1.6 1.6 0 0 0 14 15.995v3.013a1.6 1.6 0 0 0 .797 1.384l2.4 1.392a1.6 1.6 0 0 0 1.606 0l2.4-1.392A1.6 1.6 0 0 0 22 19.008v-3.013a1.6 1.6 0 0 0-.796-1.384l-2.4-1.394a1.6 1.6 0 0 0-1.514-.051z"/><path d="m18 17.4 3.79-2.198"/><path d="M2.21 4.202 6 6.4V16a2 2 0 0 0 2 2h2"/><path d="M2.796 3.611A1.6 1.6 0 0 0 2 4.995v3.013a1.6 1.6 0 0 0 .797 1.384l2.4 1.392a1.6 1.6 0 0 0 1.606 0l2.4-1.392A1.6 1.6 0 0 0 10 8.008V4.995a1.6 1.6 0 0 0-.796-1.384l-2.4-1.394a1.6 1.6 0 0 0-1.514-.052z"/><path d="m6 6.4 3.79-2.198"/>"#);

/// `braces`.
pub const BRACES: Icon = lucide!("braces", r#"<path d="M8 3H7a2 2 0 0 0-2 2v5a2 2 0 0 1-2 2 2 2 0 0 1 2 2v5c0 1.1.9 2 2 2h1"/><path d="M16 21h1a2 2 0 0 0 2-2v-5c0-1.1.9-2 2-2a2 2 0 0 1-2-2V5a2 2 0 0 0-2-2h-1"/>"#);

/// `brackets`.
pub const BRACKETS: Icon = lucide!("brackets", r#"<path d="M16 3h3a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-3"/><path d="M8 21H5a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h3"/>"#);

/// `brain`.
pub const BRAIN: Icon = lucide!("brain", r#"<path d="M12 18V5"/><path d="M15 13a4.17 4.17 0 01-3-4 4.17 4.17 0 01-3 4"/><path d="M17.707 6.294A3 3 0 1012 5a3 3 0 10-5.707 1.294"/><path d="M17.996 5.125a4 4 0 012.526 5.77"/><path d="M18 18a4 4 0 002-7.465"/><path d="M19.967 17.482a4 4 0 11-7.967.517 4 4 0 11-7.967-.517"/><path d="M6 18a4 4 0 01-2-7.465"/><path d="M6.004 5.125a4 4 0 00-2.526 5.77"/>"#);

/// `brain-circuit`.
pub const BRAIN_CIRCUIT: Icon = lucide!("brain-circuit", r#"<path d="M12 13h4"/><path d="M12 18h6a2 2 0 012 2v1"/><path d="M12 8h8"/><path d="M12 9a4.17 4.17 0 01-3 4"/><path d="M16 8V5a2 2 0 012-2"/><path d="M6 18a4 4 0 01-2-7.465"/><path d="M6.004 5.125a4 4 0 00-2.526 5.77"/><path d="M6.294 6.294A3 3 0 1112 5v13a4 4 0 11-7.967-.518"/><circle cx="16" cy="13" r="0.5" fill="currentColor"/><circle cx="18" cy="3" r="0.5" fill="currentColor"/><circle cx="20" cy="21" r="0.5" fill="currentColor"/><circle cx="20" cy="8" r="0.5" fill="currentColor"/>"#);

/// `brain-cog`.
pub const BRAIN_COG: Icon = lucide!("brain-cog", r#"<path d="m10.852 14.772-.383.923"/><path d="m10.852 9.228-.383-.923"/><path d="m13.148 14.772.382.924"/><path d="m13.53 8.305-.382.923"/><path d="m14.772 10.852.923-.383"/><path d="m14.772 13.148.923.383"/><path d="M17.997 5.125a4 4 0 012.526 5.77"/><path d="M18 18a4 4 0 002-7.464"/><path d="M4.033 17.483A4 4 0 0011.46 20c.184-.31.896-.31 1.076 0a4 4 0 007.43-2.517"/><path d="M6 18a4 4 0 01-2-7.464"/><path d="M6.002 5.125a4 4 0 00-2.525 5.77"/><path d="M6.293 6.294a3 3 0 01-.287-1.098l-.004-.07a3 3 0 015.968-.543c.05.02.057.01.059 0a3 3 0 015.968.542l-.004.071a3 3 0 01-.287 1.098"/><path d="m9.228 10.852-.923-.383"/><path d="m9.228 13.148-.923.383"/><circle cx="12" cy="12" r="3"/>"#);

/// `brain-off`.
pub const BRAIN_OFF: Icon = lucide!("brain-off", r#"<path d="M12 18v-6"/><path d="M12 5v1.344"/><path d="M17.707 6.293A3 3 0 1012 5a3 3 0 00-4.124-2.781"/><path d="M17.997 5.125a4 4 0 012.526 5.77"/><path d="M19.644 19.644A4 4 0 0112 18a4 4 0 11-7.967-.517"/><path d="m2 2 20 20"/><path d="M20 10.536a4 4 0 011.533 5.34"/><path d="M3.477 10.895a4 4 0 011.876-5.542"/><path d="M6 18a4 4 0 01-2-7.464"/><path d="M9 13a4.17 4.17 0 002.166-1.5 4 4 0 00.138-.196"/>"#);

/// `brick-wall`.
pub const BRICK_WALL: Icon = lucide!("brick-wall", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 9v6"/><path d="M16 15v6"/><path d="M16 3v6"/><path d="M3 15h18"/><path d="M3 9h18"/><path d="M8 15v6"/><path d="M8 3v6"/>"#);

/// `brick-wall-fire`.
pub const BRICK_WALL_FIRE: Icon = lucide!("brick-wall-fire", r#"<path d="M16 3v2.107"/><path d="M17 9c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 22 17a5 5 0 0 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C13 11.5 16 9 17 9"/><path d="M21 8.274V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3.938"/><path d="M3 15h5.253"/><path d="M3 9h8.228"/><path d="M8 15v6"/><path d="M8 3v6"/>"#);

/// `brick-wall-shield`.
pub const BRICK_WALL_SHIELD: Icon = lucide!("brick-wall-shield", r#"<path d="M12 9v1.258"/><path d="M16 3v5.46"/><path d="M21 9.118V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h5.75"/><path d="M22 17.5c0 2.499-1.75 3.749-3.83 4.474a.5.5 0 0 1-.335-.005c-2.085-.72-3.835-1.97-3.835-4.47V14a.5.5 0 0 1 .5-.499c1 0 2.25-.6 3.12-1.36a.6.6 0 0 1 .76-.001c.875.765 2.12 1.36 3.12 1.36a.5.5 0 0 1 .5.5z"/><path d="M3 15h7"/><path d="M3 9h12.142"/><path d="M8 15v6"/><path d="M8 3v6"/>"#);

/// `bridge`.
pub const BRIDGE: Icon = lucide!("bridge", r#"<path d="M10 9.728V16"/><path d="M14 9.728V16"/><path d="M18 20V4"/><path d="m22 11-4-4A7.5 7.5 0 0 1 6 7l-4 4"/><path d="M22 16H2"/><path d="M6 20V4"/>"#);

/// `briefcase`.
pub const BRIEFCASE: Icon = lucide!("briefcase", r#"<path d="M16 20V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/><rect width="20" height="14" x="2" y="6" rx="2"/>"#);

/// `briefcase-business`.
pub const BRIEFCASE_BUSINESS: Icon = lucide!("briefcase-business", r#"<path d="M12 12h.01"/><path d="M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2"/><path d="M22 13a18.15 18.15 0 0 1-20 0"/><rect width="20" height="14" x="2" y="6" rx="2"/>"#);

/// `briefcase-conveyor-belt`.
pub const BRIEFCASE_CONVEYOR_BELT: Icon = lucide!("briefcase-conveyor-belt", r#"<path d="M10 20v2"/><path d="M14 20v2"/><path d="M18 20v2"/><path d="M21 20H3"/><path d="M6 20v2"/><path d="M8 16V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v12"/><rect x="4" y="6" width="16" height="10" rx="2"/>"#);

/// `briefcase-medical`.
pub const BRIEFCASE_MEDICAL: Icon = lucide!("briefcase-medical", r#"<path d="M12 11v4"/><path d="M14 13h-4"/><path d="M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2"/><path d="M18 6v14"/><path d="M6 6v14"/><rect width="20" height="14" x="2" y="6" rx="2"/>"#);

/// `briefcase-plus`.
pub const BRIEFCASE_PLUS: Icon = lucide!("briefcase-plus", r#"<path d="M13.354 20H4a2 2 0 01-2-2V8a2 2 0 012-2h16a2 2 0 012 2v3.354"/><path d="M16 11.354V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v16"/><path d="M16 17h6"/><path d="M19 14v6"/>"#);

/// `bring-to-front`.
pub const BRING_TO_FRONT: Icon = lucide!("bring-to-front", r#"<rect x="8" y="8" width="8" height="8" rx="2"/><path d="M4 10a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2"/><path d="M14 20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2"/>"#);

/// `broccoli`.
pub const BROCCOLI: Icon = lucide!("broccoli", r#"<path d="M10 13a3 3 0 01-2.121-5.121"/><path d="M15.606 14.204c-3.5 1.5-5.899 4.503-8.899 7.503A1 1 0 016 22c-2 0-4-2-4-4a1 1 0 01.293-.707c1.911-1.911 3.823-3.578 5.347-5.441"/><path d="M16.573 14.737A4 4 0 0114 11"/><path d="M7.14 10.907a4 4 0 112.756-7.43A4 4 0 0116.7 4.48a2 2 0 012.82 2.82 4 4 0 011.002 6.805 4 4 0 11-7.51 1.59"/>"#);

/// `broom`.
pub const BROOM: Icon = lucide!("broom", r#"<path d="M13.5 10.5 22 2"/><path d="M14.734 13.841a2 2 0 00-.314-2.42L12.58 9.58a2 2 0 00-2.421-.314l-7.657 4.461A1 1 0 002.3 15.3l6.403 6.403a1 1 0 001.571-.204z"/><path d="m5 18 2-2"/><path d="m7.699 10.7 5.602 5.601"/>"#);

/// `broom-sparkles`.
pub const BROOM_SPARKLES: Icon = lucide!("broom-sparkles", r#"<path d="M11 2v2"/><path d="M12 3h-2"/><path d="M13.5 10.5 22 2"/><path d="M14.734 13.841a2 2 0 00-.314-2.42L12.58 9.58a2 2 0 00-2.421-.314l-7.657 4.461A1 1 0 002.3 15.3l6.403 6.403a1 1 0 001.571-.204z"/><path d="M20 15v4"/><path d="M22 17h-4"/><path d="M4 4v4"/><path d="m5 18 2-2"/><path d="M6 6H2"/><path d="m7.699 10.7 5.602 5.601"/>"#);

/// `brush`.
pub const BRUSH: Icon = lucide!("brush", r#"<path d="m11 10 3 3"/><path d="M6.5 21A3.5 3.5 0 1 0 3 17.5a2.62 2.62 0 0 1-.708 1.792A1 1 0 0 0 3 21z"/><path d="M9.969 17.031 21.378 5.624a1 1 0 0 0-3.002-3.002L6.967 14.031"/>"#);

/// `brush-cleaning`.
pub const BRUSH_CLEANING: Icon = lucide!("brush-cleaning", r#"<path d="m16 22-1-4"/><path d="M19 14a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2h-3a1 1 0 0 1-1-1V4a2 2 0 0 0-4 0v5a1 1 0 0 1-1 1H6a2 2 0 0 0-2 2v1a1 1 0 0 0 1 1"/><path d="M19 14H5l-1.973 6.767A1 1 0 0 0 4 22h16a1 1 0 0 0 .973-1.233z"/><path d="m8 22 1-4"/>"#);

/// `bubbles`.
pub const BUBBLES: Icon = lucide!("bubbles", r#"<path d="M7.001 15.085A1.5 1.5 0 0 1 9 16.5"/><circle cx="18.5" cy="8.5" r="3.5"/><circle cx="7.5" cy="16.5" r="5.5"/><circle cx="7.5" cy="4.5" r="2.5"/>"#);

/// `bug`.
pub const BUG: Icon = lucide!("bug", r#"<path d="M12 20v-9"/><path d="M14 7a4 4 0 0 1 4 4v3a6 6 0 0 1-12 0v-3a4 4 0 0 1 4-4z"/><path d="M14.12 3.88 16 2"/><path d="M21 21a4 4 0 0 0-3.81-4"/><path d="M21 5a4 4 0 0 1-3.55 3.97"/><path d="M22 13h-4"/><path d="M3 21a4 4 0 0 1 3.81-4"/><path d="M3 5a4 4 0 0 0 3.55 3.97"/><path d="M6 13H2"/><path d="m8 2 1.88 1.88"/><path d="M9 7.13V6a3 3 0 1 1 6 0v1.13"/>"#);

/// `bug-off`.
pub const BUG_OFF: Icon = lucide!("bug-off", r#"<path d="M12 20v-8"/><path d="M12.656 7H14a4 4 0 0 1 4 4v1.344"/><path d="M14.12 3.88 16 2"/><path d="M17.123 17.123A6 6 0 0 1 6 14v-3a4 4 0 0 1 1.72-3.287"/><path d="m2 2 20 20"/><path d="M21 5a4 4 0 0 1-3.55 3.97"/><path d="M22 13h-3.344"/><path d="M3 21a4 4 0 0 1 3.81-4"/><path d="M3 5a4 4 0 0 0 3.55 3.97"/><path d="M6 13H2"/><path d="m8 2 1.88 1.88"/><path d="M9.712 4.06A3 3 0 0 1 15 6v1.13"/>"#);

/// `bug-play`.
pub const BUG_PLAY: Icon = lucide!("bug-play", r#"<path d="M10 19.655A6 6 0 0 1 6 14v-3a4 4 0 0 1 4-4h4a4 4 0 0 1 4 3.97"/><path d="M14 15.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997a1 1 0 0 1-1.517-.86z"/><path d="M14.12 3.88 16 2"/><path d="M21 5a4 4 0 0 1-3.55 3.97"/><path d="M3 21a4 4 0 0 1 3.81-4"/><path d="M3 5a4 4 0 0 0 3.55 3.97"/><path d="M6 13H2"/><path d="m8 2 1.88 1.88"/><path d="M9 7.13V6a3 3 0 1 1 6 0v1.13"/>"#);

/// `building`.
pub const BUILDING: Icon = lucide!("building", r#"<path d="M12 10h.01"/><path d="M12 14h.01"/><path d="M12 6h.01"/><path d="M16 10h.01"/><path d="M16 14h.01"/><path d="M16 6h.01"/><path d="M8 10h.01"/><path d="M8 14h.01"/><path d="M8 6h.01"/><path d="M9 22v-3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v3"/><rect x="4" y="2" width="16" height="20" rx="2"/>"#);

/// `building-complex`.
pub const BUILDING_COMPLEX: Icon = lucide!("building-complex", r#"<path d="M10 12h4"/><path d="M10 8h4"/><path d="M14 21v-3a2 2 0 0 0-4 0v3"/><path d="M6 10H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-2"/><path d="M6 21V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v16"/>"#);

/// `building-complex-plus`.
pub const BUILDING_COMPLEX_PLUS: Icon = lucide!("building-complex-plus", r#"<path d="M10 12h4"/><path d="M10 21v-3a2 2 0 013.05-1.702"/><path d="M10 8h4"/><path d="M16 19h6"/><path d="M18 7h2a2 2 0 012 2v4.355"/><path d="M19 16v6"/><path d="M6 10H4a2 2 0 00-2 2v7a2 2 0 002 2h8.535"/><path d="M6 21V5a2 2 0 012-2h8a2 2 0 012 2v7.126"/>"#);

/// `bus`.
pub const BUS: Icon = lucide!("bus", r#"<path d="M8 6v6"/><path d="M15 6v6"/><path d="M2 12h19.6"/><path d="M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3"/><circle cx="7" cy="18" r="2"/><path d="M9 18h5"/><circle cx="16" cy="18" r="2"/>"#);

/// `bus-front`.
pub const BUS_FRONT: Icon = lucide!("bus-front", r#"<path d="M4 6 2 7"/><path d="M10 6h4"/><path d="m22 7-2-1"/><rect width="16" height="16" x="4" y="3" rx="2"/><path d="M4 11h16"/><path d="M8 15h.01"/><path d="M16 15h.01"/><path d="M6 19v2"/><path d="M18 21v-2"/>"#);

/// `cable`.
pub const CABLE: Icon = lucide!("cable", r#"<path d="M17 19a1 1 0 0 1-1-1v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2a1 1 0 0 1-1 1z"/><path d="M17 21v-2"/><path d="M19 14V6.5a1 1 0 0 0-7 0v11a1 1 0 0 1-7 0V10"/><path d="M21 21v-2"/><path d="M3 5V3"/><path d="M4 10a2 2 0 0 1-2-2V6a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2z"/><path d="M7 5V3"/>"#);

/// `cable-car`.
pub const CABLE_CAR: Icon = lucide!("cable-car", r#"<path d="M10 3h.01"/><path d="M14 2h.01"/><path d="m2 9 20-5"/><path d="M12 12V6.5"/><rect width="16" height="10" x="4" y="12" rx="3"/><path d="M9 12v5"/><path d="M15 12v5"/><path d="M4 17h16"/>"#);

/// `cake`.
pub const CAKE: Icon = lucide!("cake", r#"<path d="M20 21v-8a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v8"/><path d="M4 16s.5-1 2-1 2.5 2 4 2 2.5-2 4-2 2.5 2 4 2 2-1 2-1"/><path d="M2 21h20"/><path d="M7 8v3"/><path d="M12 8v3"/><path d="M17 8v3"/><path d="M7 4h.01"/><path d="M12 4h.01"/><path d="M17 4h.01"/>"#);

/// `cake-slice`.
pub const CAKE_SLICE: Icon = lucide!("cake-slice", r#"<path d="M16 13H3"/><path d="M16 17H3"/><path d="m7.2 7.9-3.388 2.5A2 2 0 0 0 3 12.01V20a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-8.654c0-2-2.44-6.026-6.44-8.026a1 1 0 0 0-1.082.057L10.4 5.6"/><circle cx="9" cy="7" r="2"/>"#);

/// `calculator`.
pub const CALCULATOR: Icon = lucide!("calculator", r#"<rect width="16" height="20" x="4" y="2" rx="2"/><line x1="8" x2="16" y1="6" y2="6"/><line x1="16" x2="16" y1="14" y2="18"/><path d="M16 10h.01"/><path d="M12 10h.01"/><path d="M8 10h.01"/><path d="M12 14h.01"/><path d="M8 14h.01"/><path d="M12 18h.01"/><path d="M8 18h.01"/>"#);

/// `calendar`.
pub const CALENDAR: Icon = lucide!("calendar", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/>"#);

/// `calendar-1`.
pub const CALENDAR_1: Icon = lucide!("calendar-1", r#"<path d="M11 13h1v4"/><path d="M16 2v3"/><path d="M3 9h18"/><path d="M8 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `calendar-arrow-down`.
pub const CALENDAR_ARROW_DOWN: Icon = lucide!("calendar-arrow-down", r#"<path d="m14 17 4 4 4-4"/><path d="M16 2v3"/><path d="M18 13v8"/><path d="M21 10.354V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h7.343"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-arrow-left`.
pub const CALENDAR_ARROW_LEFT: Icon = lucide!("calendar-arrow-left", r#"<path d="M16 2v3"/><path d="M21 9H3"/><path d="M3 12.34V5a2 2 0 012-2h14a2 2 0 012 2v14a2 2 0 01-2 2h-7.35"/><path d="M3 18h6"/><path d="m6 21-3-3 3-3"/><path d="M8 2v3"/>"#);

/// `calendar-arrow-right`.
pub const CALENDAR_ARROW_RIGHT: Icon = lucide!("calendar-arrow-right", r#"<path d="M16 2v3"/><path d="m18 21 3-3h-6"/><path d="M21 12.34V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h7.35"/><path d="m21 18-3-3"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-arrow-up`.
pub const CALENDAR_ARROW_UP: Icon = lucide!("calendar-arrow-up", r#"<path d="m14 17 4-4 4 4"/><path d="M16 2v3"/><path d="M18 21v-8"/><path d="M21 10.343V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h9"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-check`.
pub const CALENDAR_CHECK: Icon = lucide!("calendar-check", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="m9 15 2 2 4-4"/>"#);

/// `calendar-check-2`.
pub const CALENDAR_CHECK_2: Icon = lucide!("calendar-check-2", r#"<path d="M 19 3 L 5 3"/><path d="M 21 13 L 21 5"/><path d="M 21 5 A2 2 0 0 0 19 3"/><path d="M 3 19 A2 2 0 0 0 5 21"/><path d="M 3 5 L 3 19"/><path d="M 5 3 A2 2 0 0 0 3 5"/><path d="m16 19 2 2 4-4"/><path d="M16 2v3"/><path d="M3 9h18"/><path d="M5 21 L12.5 21"/><path d="M8 2v3"/>"#);

/// `calendar-chevrons-left`.
pub const CALENDAR_CHEVRONS_LEFT: Icon = lucide!("calendar-chevrons-left", r#"<path d="m11 21-3-3 3-3"/><path d="M16 2v3"/><path d="M21 9H3"/><path d="M3 11.5V5a2 2 0 012-2h14a2 2 0 012 2v14a2 2 0 01-2 2h-4"/><path d="m5 21-3-3 3-3"/><path d="M8 2v3"/>"#);

/// `calendar-chevrons-right`.
pub const CALENDAR_CHEVRONS_RIGHT: Icon = lucide!("calendar-chevrons-right", r#"<path d="m13 21 3-3-3-3"/><path d="M16 2v3"/><path d="m19 21 3-3-3-3"/><path d="M21 11.5V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h4"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-clock`.
pub const CALENDAR_CLOCK: Icon = lucide!("calendar-clock", r#"<path d="M16 14v2.2l1.6 1"/><path d="M16 2v3"/><path d="M21 7.338V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h2.338"/><path d="M3 9h5.859"/><path d="M8 2v3"/><circle cx="16" cy="16" r="6"/>"#);

/// `calendar-cog`.
pub const CALENDAR_COG: Icon = lucide!("calendar-cog", r#"<path d="m15.228 16.852-.923-.383"/><path d="m15.228 19.148-.923.383"/><path d="M16 2v3"/><path d="m16.47 14.305.382.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="m20.773 16.852.924-.383"/><path d="m20.773 19.148.924.383"/><path d="M21 10.5V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h5.5"/><path d="M3 9h18"/><path d="M8 2v3"/><circle cx="18" cy="18" r="3"/>"#);

/// `calendar-days`.
pub const CALENDAR_DAYS: Icon = lucide!("calendar-days", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="M8 13h.01"/><path d="M12 13h.01"/><path d="M16 13h.01"/><path d="M8 17h.01"/><path d="M12 17h.01"/><path d="M16 17h.01"/>"#);

/// `calendar-fold`.
pub const CALENDAR_FOLD: Icon = lucide!("calendar-fold", r#"<path d="M16 2v3"/><path d="M21 15V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h10v-5a1 1 0 011-1za2.4 2.4 0 01-.706 1.706l-3.588 3.588A2.4 2.4 0 0115 21"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-heart`.
pub const CALENDAR_HEART: Icon = lucide!("calendar-heart", r#"<path d="M12.127 21H5a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v5.125"/><path d="M14.62 17.8A2.25 2.25 0 1118 14.836a2.25 2.25 0 113.38 2.966l-2.626 2.856a.998.998 0 01-1.507 0z"/><path d="M16 2v3"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-minus`.
pub const CALENDAR_MINUS: Icon = lucide!("calendar-minus", r#"<path d="M16 18h6"/><path d="M16 2v3"/><path d="M21 14V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h8.3"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-minus-2`.
pub const CALENDAR_MINUS_2: Icon = lucide!("calendar-minus-2", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="M10 15h4"/>"#);

/// `calendar-off`.
pub const CALENDAR_OFF: Icon = lucide!("calendar-off", r#"<path d="M16 2v3"/><path d="m2 2 20 20"/><path d="M21 9h-5.5"/><path d="M3 9h6"/><path d="M3.586 3.586A2 2 0 003 5v14a2 2 0 002 2h14a2 2 0 001.414-.586"/><path d="M8.656 3H19a2 2 0 012 2v10.344"/>"#);

/// `calendar-plus`.
pub const CALENDAR_PLUS: Icon = lucide!("calendar-plus", r#"<path d="M16 18h6"/><path d="M16 2v3"/><path d="M19 15v6"/><path d="M21 11.5V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h8.3"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendar-plus-2`.
pub const CALENDAR_PLUS_2: Icon = lucide!("calendar-plus-2", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="M10 15h4"/><path d="M12 13v4"/>"#);

/// `calendar-range`.
pub const CALENDAR_RANGE: Icon = lucide!("calendar-range", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M16 2v3"/><path d="M3 9h18"/><path d="M8 2v3"/><path d="M17 13h-6"/><path d="M13 17H7"/><path d="M7 13h.01"/><path d="M17 17h.01"/>"#);

/// `calendar-search`.
pub const CALENDAR_SEARCH: Icon = lucide!("calendar-search", r#"<path d="M16 2v3"/><path d="M21 10.69V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h7.25"/><path d="m22 21-1.875-1.875"/><path d="M3 9h18"/><path d="M8 2v3"/><circle cx="18" cy="17" r="3"/>"#);

/// `calendar-sync`.
pub const CALENDAR_SYNC: Icon = lucide!("calendar-sync", r#"<path d="M11 10v4h4"/><path d="m11 14 1.535-1.605a5 5 0 018 1.5"/><path d="M16 2v3"/><path d="m21 18-1.535 1.605a5 5 0 01-8-1.5"/><path d="M21 22v-4h-4"/><path d="M21 8.517V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h3.517"/><path d="M3 9h4"/><path d="M8 2v3"/>"#);

/// `calendar-x`.
pub const CALENDAR_X: Icon = lucide!("calendar-x", r#"<path d="M8 2v3"/><path d="M16 2v3"/><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="m14 13-4 4"/><path d="m10 13 4 4"/>"#);

/// `calendar-x-2`.
pub const CALENDAR_X_2: Icon = lucide!("calendar-x-2", r#"<path d="M16 2v3"/><path d="m17 16 5 5"/><path d="m17 21 5-5"/><path d="M21 12V5a2 2 0 00-2-2H5a2 2 0 00-2 2v14a2 2 0 002 2h8"/><path d="M3 9h18"/><path d="M8 2v3"/>"#);

/// `calendars`.
pub const CALENDARS: Icon = lucide!("calendars", r#"<path d="M12 2v2"/><path d="M15.726 21.01A2 2 0 0 1 14 22H4a2 2 0 0 1-2-2V10a2 2 0 0 1 2-2"/><path d="M18 2v2"/><path d="M2 13h2"/><path d="M8 8h14"/><rect x="8" y="3" width="14" height="14" rx="2"/>"#);

/// `camera`.
pub const CAMERA: Icon = lucide!("camera", r#"<path d="M13.997 4a2 2 0 0 1 1.76 1.05l.486.9A2 2 0 0 0 18.003 7H20a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h1.997a2 2 0 0 0 1.759-1.048l.489-.904A2 2 0 0 1 10.004 4z"/><circle cx="12" cy="13" r="3"/>"#);

/// `camera-off`.
pub const CAMERA_OFF: Icon = lucide!("camera-off", r#"<path d="M14.564 14.558a3 3 0 1 1-4.122-4.121"/><path d="m2 2 20 20"/><path d="M20 20H4a2 2 0 0 1-2-2V9a2 2 0 0 1 2-2h1.997a2 2 0 0 0 .819-.175"/><path d="M9.695 4.024A2 2 0 0 1 10.004 4h3.993a2 2 0 0 1 1.76 1.05l.486.9A2 2 0 0 0 18.003 7H20a2 2 0 0 1 2 2v7.344"/>"#);

/// `can`.
pub const CAN: Icon = lucide!("can", r#"<path d="M21 10.5a9 2.5 0 01-18 0v8a9 2.5 0 0018 0z"/><path d="M21 10.5A9 2.5 25.32 004.59 3.47 9 2.5 25.32 0021 10.5"/><path d="M3 10.5a9 2.5 0 016.527-2.405"/><path d="M9 16.858a31 31 0 006 0"/>"#);

/// `can-soda`.
pub const CAN_SODA: Icon = lucide!("can-soda", r#"<path d="m17 22 1.664-2.496a2 2 0 00.336-1.11V5.606a2 2 0 00-.336-1.11L17 2"/><path d="M18 22H6"/><path d="M18 2H6"/><path d="M5 17h14"/><path d="M5 7h14"/><path d="m7 22-1.664-2.496A2 2 0 015 18.394V5.606a2 2 0 01.336-1.11L7 2"/>"#);

/// `candy`.
pub const CANDY: Icon = lucide!("candy", r#"<path d="M10 7v10.9"/><path d="M14 6.1V17"/><path d="M16 7V3a1 1 0 0 1 1.707-.707 2.5 2.5 0 0 0 2.152.717 1 1 0 0 1 1.131 1.131 2.5 2.5 0 0 0 .717 2.152A1 1 0 0 1 21 8h-4"/><path d="M16.536 7.465a5 5 0 0 0-7.072 0l-2 2a5 5 0 0 0 0 7.07 5 5 0 0 0 7.072 0l2-2a5 5 0 0 0 0-7.07"/><path d="M8 17v4a1 1 0 0 1-1.707.707 2.5 2.5 0 0 0-2.152-.717 1 1 0 0 1-1.131-1.131 2.5 2.5 0 0 0-.717-2.152A1 1 0 0 1 3 16h4"/>"#);

/// `candy-cane`.
pub const CANDY_CANE: Icon = lucide!("candy-cane", r#"<path d="m10.8 5 2.111 4.223"/><path d="M17.75 7 15 2.1"/><path d="m4.874 14.647 2.12 4.24"/><path d="M5.7 21a2 2 0 0 1-3.5-2l8.6-14a6 6 0 0 1 10.4 6 2 2 0 1 1-3.464-2 2 2 0 1 0-3.464-2z"/><path d="m7.906 9.712 2.005 4.411"/>"#);

/// `candy-off`.
pub const CANDY_OFF: Icon = lucide!("candy-off", r#"<path d="M10 10v7.9"/><path d="M11.802 6.145a5 5 0 0 1 6.053 6.053"/><path d="M14 6.1v2.243"/><path d="m15.5 15.571-.964.964a5 5 0 0 1-7.071 0 5 5 0 0 1 0-7.07l.964-.965"/><path d="M16 7V3a1 1 0 0 1 1.707-.707 2.5 2.5 0 0 0 2.152.717 1 1 0 0 1 1.131 1.131 2.5 2.5 0 0 0 .717 2.152A1 1 0 0 1 21 8h-4"/><path d="m2 2 20 20"/><path d="M8 17v4a1 1 0 0 1-1.707.707 2.5 2.5 0 0 0-2.152-.717 1 1 0 0 1-1.131-1.131 2.5 2.5 0 0 0-.717-2.152A1 1 0 0 1 3 16h4"/>"#);

/// `cannabis`.
pub const CANNABIS: Icon = lucide!("cannabis", r#"<path d="M12 22v-4"/><path d="M7 12c-1.5 0-4.5 1.5-5 3 3.5 1.5 6 1 6 1-1.5 1.5-2 3.5-2 5 2.5 0 4.5-1.5 6-3 1.5 1.5 3.5 3 6 3 0-1.5-.5-3.5-2-5 0 0 2.5.5 6-1-.5-1.5-3.5-3-5-3 1.5-1 4-4 4-6-2.5 0-5.5 1.5-7 3 0-2.5-.5-5-2-7-1.5 2-2 4.5-2 7-1.5-1.5-4.5-3-7-3 0 2 2.5 5 4 6"/>"#);

/// `cannabis-off`.
pub const CANNABIS_OFF: Icon = lucide!("cannabis-off", r#"<path d="M12 22v-4c1.5 1.5 3.5 3 6 3 0-1.5-.5-3.5-2-5"/><path d="M13.988 8.327C13.902 6.054 13.365 3.82 12 2a9.3 9.3 0 0 0-1.445 2.9"/><path d="M17.375 11.725C18.882 10.53 21 7.841 21 6c-2.324 0-5.08 1.296-6.662 2.684"/><path d="m2 2 20 20"/><path d="M21.024 15.378A15 15 0 0 0 22 15c-.426-1.279-2.67-2.557-4.25-2.907"/><path d="M6.995 6.992C5.714 6.4 4.29 6 3 6c0 2 2.5 5 4 6-1.5 0-4.5 1.5-5 3 3.5 1.5 6 1 6 1-1.5 1.5-2 3.5-2 5 2.5 0 4.5-1.5 6-3"/>"#);

/// `captions`.
pub const CAPTIONS: Icon = lucide!("captions", r#"<rect width="18" height="14" x="3" y="5" rx="2" ry="2"/><path d="M7 15h4M15 15h2M7 11h2M13 11h4"/>"#);

/// `captions-off`.
pub const CAPTIONS_OFF: Icon = lucide!("captions-off", r#"<path d="M10.5 5H19a2 2 0 0 1 2 2v8.5"/><path d="M17 11h-.5"/><path d="M19 19H5a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2"/><path d="m2 2 20 20"/><path d="M7 11h4"/><path d="M7 15h2.5"/>"#);

/// `car`.
pub const CAR: Icon = lucide!("car", r#"<path d="M19 17h2c.6 0 1-.4 1-1v-3c0-.9-.7-1.7-1.5-1.9C18.7 10.6 16 10 16 10s-1.3-1.4-2.2-2.3c-.5-.4-1.1-.7-1.8-.7H5c-.6 0-1.1.4-1.4.9l-1.4 2.9A3.7 3.7 0 0 0 2 12v4c0 .6.4 1 1 1h2"/><circle cx="7" cy="17" r="2"/><path d="M9 17h6"/><circle cx="17" cy="17" r="2"/>"#);

/// `car-battery`.
pub const CAR_BATTERY: Icon = lucide!("car-battery", r#"<path d="M14 13h4"/><path d="M16 15v-4"/><path d="M18 5v2"/><path d="M6 13h4"/><path d="M6 5v2"/><rect x="2" y="7" width="20" height="12" rx="2"/>"#);

/// `car-front`.
pub const CAR_FRONT: Icon = lucide!("car-front", r#"<path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"/><path d="M7 14h.01"/><path d="M17 14h.01"/><rect width="18" height="8" x="3" y="10" rx="2"/><path d="M5 18v2"/><path d="M19 18v2"/>"#);

/// `car-taxi-front`.
pub const CAR_TAXI_FRONT: Icon = lucide!("car-taxi-front", r#"<path d="M10 2h4"/><path d="m21 8-2 2-1.5-3.7A2 2 0 0 0 15.646 5H8.4a2 2 0 0 0-1.903 1.257L5 10 3 8"/><path d="M7 14h.01"/><path d="M17 14h.01"/><rect width="18" height="8" x="3" y="10" rx="2"/><path d="M5 18v2"/><path d="M19 18v2"/>"#);

/// `caravan`.
pub const CARAVAN: Icon = lucide!("caravan", r#"<path d="M18 19V9a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v8a2 2 0 0 0 2 2h2"/><path d="M2 9h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H2"/><path d="M22 17v1a1 1 0 0 1-1 1H10v-9a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v9"/><circle cx="8" cy="19" r="2"/>"#);

/// `card-sim`.
pub const CARD_SIM: Icon = lucide!("card-sim", r#"<path d="M12 14v4"/><path d="M14.172 2a2 2 0 0 1 1.414.586l3.828 3.828A2 2 0 0 1 20 7.828V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z"/><path d="M8 14h8"/><rect x="8" y="10" width="8" height="8" rx="2"/>"#);

/// `carrot`.
pub const CARROT: Icon = lucide!("carrot", r#"<path d="M15 16a1 1 0 0 0-7-7q-4 4-5.987 12.385a.5.5 0 0 0 .602.602Q11 20 15 16l-3-3"/><path d="M15 9q4 4 7 0-3-4-7 0 4-4 0-7-4 3 0 7"/><path d="m8 15-2.58-2.58"/>"#);

/// `carton`.
pub const CARTON: Icon = lucide!("carton", r#"<path d="M13 22V10a2 2 0 01.539-1.367L16 6H8L5.539 8.633A2 2 0 005 10v10a2 2 0 002 2h10a2 2 0 002-2V10a2 2 0 00-.539-1.367L16 6V3a1 1 0 00-1-1H9a1 1 0 00-1 1v3"/><path d="M5 10h8"/>"#);

/// `carton-off`.
pub const CARTON_OFF: Icon = lucide!("carton-off", r#"<path d="M10 10H5v10a2 2 0 002 2h10a2 2 0 002-2v-1"/><path d="M13 22v-9"/><path d="M13.902 8.245 16 6h-4.343"/><path d="M19 13.343V10a2 2 0 00-.539-1.367L16 6V3a1 1 0 00-1-1H9a1 1 0 00-.857.486"/><path d="m2 2 20 20"/><path d="M7.034 7.034 5.539 8.633A2 2 0 005 10"/>"#);

/// `case-lower`.
pub const CASE_LOWER: Icon = lucide!("case-lower", r#"<path d="M10 9v7"/><path d="M14 6v10"/><circle cx="17.5" cy="12.5" r="3.5"/><circle cx="6.5" cy="12.5" r="3.5"/>"#);

/// `case-sensitive`.
pub const CASE_SENSITIVE: Icon = lucide!("case-sensitive", r#"<path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16"/><path d="M22 9v7"/><path d="M3.304 13h6.392"/><circle cx="18.5" cy="12.5" r="3.5"/>"#);

/// `case-upper`.
pub const CASE_UPPER: Icon = lucide!("case-upper", r#"<path d="M15 11h4.5a1 1 0 0 1 0 5h-4a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h3a1 1 0 0 1 0 5"/><path d="m2 16 4.039-9.69a.5.5 0 0 1 .923 0L11 16"/><path d="M3.304 13h6.392"/>"#);

/// `cassette-tape`.
pub const CASSETTE_TAPE: Icon = lucide!("cassette-tape", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><circle cx="8" cy="10" r="2"/><path d="M8 12h8"/><circle cx="16" cy="10" r="2"/><path d="m6 20 .7-2.9A1.4 1.4 0 0 1 8.1 16h7.8a1.4 1.4 0 0 1 1.4 1l.7 3"/>"#);

/// `cast`.
pub const CAST: Icon = lucide!("cast", r#"<path d="M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6"/><path d="M2 12a9 9 0 0 1 8 8"/><path d="M2 16a5 5 0 0 1 4 4"/><line x1="2" x2="2.01" y1="20" y2="20"/>"#);

/// `castle`.
pub const CASTLE: Icon = lucide!("castle", r#"<path d="M10 5V3"/><path d="M14 5V3"/><path d="M15 21v-3a3 3 0 0 0-6 0v3"/><path d="M18 3v8"/><path d="M18 5H6"/><path d="M22 11H2"/><path d="M22 9v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9"/><path d="M6 3v8"/>"#);

/// `cat`.
pub const CAT: Icon = lucide!("cat", r#"<path d="M12 5c.67 0 1.35.09 2 .26 1.78-2 5.03-2.84 6.42-2.26 1.4.58-.42 7-.42 7 .57 1.07 1 2.24 1 3.44C21 17.9 16.97 21 12 21s-9-3-9-7.56c0-1.25.5-2.4 1-3.44 0 0-1.89-6.42-.5-7 1.39-.58 4.72.23 6.5 2.23A9.04 9.04 0 0 1 12 5Z"/><path d="M8 14v.5"/><path d="M16 14v.5"/><path d="M11.25 16.25h1.5L12 17l-.75-.75Z"/>"#);

/// `cctv`.
pub const CCTV: Icon = lucide!("cctv", r#"<path d="M16.75 12h3.632a1 1 0 0 1 .894 1.447l-2.034 4.069a1 1 0 0 1-1.708.134l-2.124-2.97"/><path d="M17.106 9.053a1 1 0 0 1 .447 1.341l-3.106 6.211a1 1 0 0 1-1.342.447L3.61 12.3a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.92 2.92 0 0 1 3.92-1.3z"/><path d="M2 19h3.76a2 2 0 0 0 1.8-1.1L9 15"/><path d="M2 21v-4"/><path d="M7 9h.01"/>"#);

/// `cctv-off`.
pub const CCTV_OFF: Icon = lucide!("cctv-off", r#"<path d="m12.309 6.652 4.797 2.401a1 1 0 0 1 .447 1.341l-.501 1.001.605.605h2.725a1 1 0 0 1 .894 1.447l-.724 1.448"/><path d="m15.166 15.166-.719 1.439a1 1 0 0 1-1.342.447L3.61 12.3a2.92 2.92 0 0 1-1.3-3.91L3.69 5.6a2.9 2.9 0 0 1 .873-1.037"/><path d="M2 19h3.76a2 2 0 0 0 1.8-1.1l1.441-2.902"/><path d="m2 2 20 20"/><path d="M2 21v-4"/><path d="M7 9h.01"/>"#);

/// `certificate`.
pub const CERTIFICATE: Icon = lucide!("certificate", r#"<path d="M12 10H6"/><path d="m20.69 15.479 1.29 4.88a.5.5 0 01-.698.591l-1.843-.849a1 1 0 00-.879.001l-1.846.85a.5.5 0 01-.692-.593l1.29-4.88"/><path d="M22 6.675V6a2 2 0 00-2-2H4a2 2 0 00-2 2v12a2 2 0 002 2h8"/><path d="M6 14h4"/><circle cx="19" cy="13" r="3"/>"#);

/// `chart-area`.
pub const CHART_AREA: Icon = lucide!("chart-area", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 11.207a.5.5 0 0 1 .146-.353l2-2a.5.5 0 0 1 .708 0l3.292 3.292a.5.5 0 0 0 .708 0l4.292-4.292a.5.5 0 0 1 .854.353V16a1 1 0 0 1-1 1H8a1 1 0 0 1-1-1z"/>"#);

/// `chart-bar`.
pub const CHART_BAR: Icon = lucide!("chart-bar", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 16h8"/><path d="M7 11h12"/><path d="M7 6h3"/>"#);

/// `chart-bar-big`.
pub const CHART_BAR_BIG: Icon = lucide!("chart-bar-big", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><rect x="7" y="13" width="9" height="4" rx="1"/><rect x="7" y="5" width="12" height="4" rx="1"/>"#);

/// `chart-bar-decreasing`.
pub const CHART_BAR_DECREASING: Icon = lucide!("chart-bar-decreasing", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 11h8"/><path d="M7 16h3"/><path d="M7 6h12"/>"#);

/// `chart-bar-increasing`.
pub const CHART_BAR_INCREASING: Icon = lucide!("chart-bar-increasing", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 11h8"/><path d="M7 16h12"/><path d="M7 6h3"/>"#);

/// `chart-bar-stacked`.
pub const CHART_BAR_STACKED: Icon = lucide!("chart-bar-stacked", r#"<path d="M11 13v4"/><path d="M15 5v4"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><rect x="7" y="13" width="9" height="4" rx="1"/><rect x="7" y="5" width="12" height="4" rx="1"/>"#);

/// `chart-bell-curve`.
pub const CHART_BELL_CURVE: Icon = lucide!("chart-bell-curve", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 16c3.5 0 2.5-10 6-10s2.5 10 6 10"/>"#);

/// `chart-candlestick`.
pub const CHART_CANDLESTICK: Icon = lucide!("chart-candlestick", r#"<path d="M9 5v4"/><rect width="4" height="6" x="7" y="9" rx="1"/><path d="M9 15v2"/><path d="M17 3v2"/><rect width="4" height="8" x="15" y="5" rx="1"/><path d="M17 13v3"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/>"#);

/// `chart-column`.
pub const CHART_COLUMN: Icon = lucide!("chart-column", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M18 17V9"/><path d="M13 17V5"/><path d="M8 17v-3"/>"#);

/// `chart-column-big`.
pub const CHART_COLUMN_BIG: Icon = lucide!("chart-column-big", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><rect x="15" y="5" width="4" height="12" rx="1"/><rect x="7" y="8" width="4" height="9" rx="1"/>"#);

/// `chart-column-decreasing`.
pub const CHART_COLUMN_DECREASING: Icon = lucide!("chart-column-decreasing", r#"<path d="M13 17V9"/><path d="M18 17v-3"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M8 17V5"/>"#);

/// `chart-column-increasing`.
pub const CHART_COLUMN_INCREASING: Icon = lucide!("chart-column-increasing", r#"<path d="M13 17V9"/><path d="M18 17V5"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M8 17v-3"/>"#);

/// `chart-column-stacked`.
pub const CHART_COLUMN_STACKED: Icon = lucide!("chart-column-stacked", r#"<path d="M11 13H7"/><path d="M19 9h-4"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><rect x="15" y="5" width="4" height="12" rx="1"/><rect x="7" y="8" width="4" height="9" rx="1"/>"#);

/// `chart-gantt`.
pub const CHART_GANTT: Icon = lucide!("chart-gantt", r#"<path d="M10 6h8"/><path d="M12 16h6"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M8 11h7"/>"#);

/// `chart-line`.
pub const CHART_LINE: Icon = lucide!("chart-line", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="m19 9-5 5-4-4-3 3"/>"#);

/// `chart-network`.
pub const CHART_NETWORK: Icon = lucide!("chart-network", r#"<path d="m13.11 7.664 1.78 2.672"/><path d="m14.162 12.788-3.324 1.424"/><path d="m20 4-6.06 1.515"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/><circle cx="12" cy="6" r="2"/><circle cx="16" cy="12" r="2"/><circle cx="9" cy="15" r="2"/>"#);

/// `chart-no-axes-column`.
pub const CHART_NO_AXES_COLUMN: Icon = lucide!("chart-no-axes-column", r#"<path d="M5 21v-6"/><path d="M12 21V3"/><path d="M19 21V9"/>"#);

/// `chart-no-axes-column-decreasing`.
pub const CHART_NO_AXES_COLUMN_DECREASING: Icon = lucide!("chart-no-axes-column-decreasing", r#"<path d="M5 21V3"/><path d="M12 21V9"/><path d="M19 21v-6"/>"#);

/// `chart-no-axes-column-increasing`.
pub const CHART_NO_AXES_COLUMN_INCREASING: Icon = lucide!("chart-no-axes-column-increasing", r#"<path d="M5 21v-6"/><path d="M12 21V9"/><path d="M19 21V3"/>"#);

/// `chart-no-axes-combined`.
pub const CHART_NO_AXES_COMBINED: Icon = lucide!("chart-no-axes-combined", r#"<path d="M12 16v5"/><path d="M16 14.639V21"/><path d="M20 10.656V21"/><path d="m22 3-8.646 8.646a.5.5 0 0 1-.708 0L9.354 8.354a.5.5 0 0 0-.707 0L2 15"/><path d="M4 18.463V21"/><path d="M8 14.656V21"/>"#);

/// `chart-no-axes-gantt`.
pub const CHART_NO_AXES_GANTT: Icon = lucide!("chart-no-axes-gantt", r#"<path d="M6 5h12"/><path d="M4 12h10"/><path d="M12 19h8"/>"#);

/// `chart-pie`.
pub const CHART_PIE: Icon = lucide!("chart-pie", r#"<path d="M21 12c.552 0 1.005-.449.95-.998a10 10 0 0 0-8.953-8.951c-.55-.055-.998.398-.998.95v8a1 1 0 0 0 1 1z"/><path d="M21.21 15.89A10 10 0 1 1 8 2.83"/>"#);

/// `chart-quadrant`.
pub const CHART_QUADRANT: Icon = lucide!("chart-quadrant", r#"<path d="M13 3v18"/><path d="M3 12h18"/><path d="M3 3v16a2 2 0 002 2h16"/><circle cx="17.5" cy="5.5" r="0.5" fill="currentColor"/><circle cx="19.5" cy="16.5" r="0.5" fill="currentColor"/><circle cx="7.5" cy="16.5" r="0.5" fill="currentColor"/><circle cx="7.5" cy="7.5" r="0.5" fill="currentColor"/>"#);

/// `chart-radar`.
pub const CHART_RADAR: Icon = lucide!("chart-radar", r#"<path d="M12 2v10"/><path d="m16 18 3.085-8.125L12 4.103 3.473 9.442l3.196 10.554z"/><path d="m2 9 10 3 6 9"/><path d="m6 21 6-9 10-3"/>"#);

/// `chart-scatter`.
pub const CHART_SCATTER: Icon = lucide!("chart-scatter", r#"<circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/><circle cx="18.5" cy="5.5" r=".5" fill="currentColor"/><circle cx="11.5" cy="11.5" r=".5" fill="currentColor"/><circle cx="7.5" cy="16.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="14.5" r=".5" fill="currentColor"/><path d="M3 3v16a2 2 0 0 0 2 2h16"/>"#);

/// `chart-spline`.
pub const CHART_SPLINE: Icon = lucide!("chart-spline", r#"<path d="M3 3v16a2 2 0 0 0 2 2h16"/><path d="M7 16c.5-2 1.5-7 4-7 2 0 2 3 4 3 2.5 0 4.5-5 5-7"/>"#);

/// `check`.
pub const CHECK: Icon = lucide!("check", r#"<path d="M20 6 9 17l-5-5"/>"#);

/// `check-check`.
pub const CHECK_CHECK: Icon = lucide!("check-check", r#"<path d="M18 6 7 17l-5-5"/><path d="m22 10-7.5 7.5L13 16"/>"#);

/// `check-line`.
pub const CHECK_LINE: Icon = lucide!("check-line", r#"<path d="M20 4L9 15"/><path d="M21 19L3 19"/><path d="M9 15L4 10"/>"#);

/// `chef-hat`.
pub const CHEF_HAT: Icon = lucide!("chef-hat", r#"<path d="M10 11v5"/><path d="M14 11v5"/><path d="M6 12.65c0-.457-.316-.844-.727-1.041A4 4 0 017.52 4.034a1.53 1.53 0 001.367-.547 4 4 0 016.225 0 1.53 1.53 0 001.367.547 4 4 0 012.248 7.575c-.411.197-.727.584-.727 1.04V19a2 2 0 01-2 2H8a2 2 0 01-2-2z"/><path d="M6 16h12"/>"#);

/// `cherry`.
pub const CHERRY: Icon = lucide!("cherry", r#"<path d="M2 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"/><path d="M12 17a5 5 0 0 0 10 0c0-2.76-2.5-5-5-3-2.5-2-5 .24-5 3Z"/><path d="M7 14c3.22-2.91 4.29-8.75 5-12 1.66 2.38 4.94 9 5 12"/><path d="M22 9c-4.29 0-7.14-2.33-10-7 5.71 0 10 4.67 10 7Z"/>"#);

/// `chess-bishop`.
pub const CHESS_BISHOP: Icon = lucide!("chess-bishop", r#"<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z"/><path d="M15 18c1.5-.615 3-2.461 3-4.923C18 8.769 14.5 4.462 12 2 9.5 4.462 6 8.77 6 13.077 6 15.539 7.5 17.385 9 18"/><path d="m16 7-2.5 2.5"/><path d="M9 2h6"/>"#);

/// `chess-king`.
pub const CHESS_KING: Icon = lucide!("chess-king", r#"<path d="M4 20a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1z"/><path d="m6.7 18-1-1C4.35 15.682 3 14.09 3 12a5 5 0 0 1 4.95-5c1.584 0 2.7.455 4.05 1.818C13.35 7.455 14.466 7 16.05 7A5 5 0 0 1 21 12c0 2.082-1.359 3.673-2.7 5l-1 1"/><path d="M10 4h4"/><path d="M12 2v6.818"/>"#);

/// `chess-knight`.
pub const CHESS_KNIGHT: Icon = lucide!("chess-knight", r#"<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z"/><path d="M16.5 18c1-2 2.5-5 2.5-9a7 7 0 0 0-7-7H6.635a1 1 0 0 0-.768 1.64L7 5l-2.32 5.802a2 2 0 0 0 .95 2.526l2.87 1.456"/><path d="m15 5 1.425-1.425"/><path d="m17 8 1.53-1.53"/><path d="M9.713 12.185 7 18"/>"#);

/// `chess-pawn`.
pub const CHESS_PAWN: Icon = lucide!("chess-pawn", r#"<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z"/><path d="m14.5 10 1.5 8"/><path d="M7 10h10"/><path d="m8 18 1.5-8"/><circle cx="12" cy="6" r="4"/>"#);

/// `chess-queen`.
pub const CHESS_QUEEN: Icon = lucide!("chess-queen", r#"<path d="m10.68 5.499-.996 4.982a.65.65 0 01-1.131.295L5.885 7.663"/><path d="m13.32 5.5.995 4.98a.65.65 0 001.131.296l2.668-3.113"/><path d="M20.428 8.952 17.5 18"/><path d="M4 20a2 2 0 012-2h12a2 2 0 012 2v1a1 1 0 01-1 1H5a1 1 0 01-1-1z"/><path d="M6.5 18 3.571 8.952"/><circle cx="12" cy="4" r="2"/><circle cx="20" cy="7" r="2"/><circle cx="4" cy="7" r="2"/>"#);

/// `chess-rook`.
pub const CHESS_ROOK: Icon = lucide!("chess-rook", r#"<path d="M5 20a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1z"/><path d="M10 2v2"/><path d="M14 2v2"/><path d="m17 18-1-9"/><path d="M6 2v5a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V2"/><path d="M6 4h12"/><path d="m7 18 1-9"/>"#);

/// `chevron-down`.
pub const CHEVRON_DOWN: Icon = lucide!("chevron-down", r#"<path d="m6 9 6 6 6-6"/>"#);

/// `chevron-first`.
pub const CHEVRON_FIRST: Icon = lucide!("chevron-first", r#"<path d="m17 18-6-6 6-6"/><path d="M7 6v12"/>"#);

/// `chevron-last`.
pub const CHEVRON_LAST: Icon = lucide!("chevron-last", r#"<path d="m7 18 6-6-6-6"/><path d="M17 6v12"/>"#);

/// `chevron-left`.
pub const CHEVRON_LEFT: Icon = lucide!("chevron-left", r#"<path d="m15 18-6-6 6-6"/>"#);

/// `chevron-right`.
pub const CHEVRON_RIGHT: Icon = lucide!("chevron-right", r#"<path d="m9 18 6-6-6-6"/>"#);

/// `chevron-up`.
pub const CHEVRON_UP: Icon = lucide!("chevron-up", r#"<path d="m18 15-6-6-6 6"/>"#);

/// `chevrons-down`.
pub const CHEVRONS_DOWN: Icon = lucide!("chevrons-down", r#"<path d="m7 6 5 5 5-5"/><path d="m7 13 5 5 5-5"/>"#);

/// `chevrons-down-up`.
pub const CHEVRONS_DOWN_UP: Icon = lucide!("chevrons-down-up", r#"<path d="m7 20 5-5 5 5"/><path d="m7 4 5 5 5-5"/>"#);

/// `chevrons-left`.
pub const CHEVRONS_LEFT: Icon = lucide!("chevrons-left", r#"<path d="m11 17-5-5 5-5"/><path d="m18 17-5-5 5-5"/>"#);

/// `chevrons-left-right`.
pub const CHEVRONS_LEFT_RIGHT: Icon = lucide!("chevrons-left-right", r#"<path d="m9 7-5 5 5 5"/><path d="m15 7 5 5-5 5"/>"#);

/// `chevrons-left-right-ellipsis`.
pub const CHEVRONS_LEFT_RIGHT_ELLIPSIS: Icon = lucide!("chevrons-left-right-ellipsis", r#"<path d="M12 12h.01"/><path d="M16 12h.01"/><path d="m17 7 5 5-5 5"/><path d="m7 7-5 5 5 5"/><path d="M8 12h.01"/>"#);

/// `chevrons-right`.
pub const CHEVRONS_RIGHT: Icon = lucide!("chevrons-right", r#"<path d="m6 17 5-5-5-5"/><path d="m13 17 5-5-5-5"/>"#);

/// `chevrons-right-left`.
pub const CHEVRONS_RIGHT_LEFT: Icon = lucide!("chevrons-right-left", r#"<path d="m20 17-5-5 5-5"/><path d="m4 17 5-5-5-5"/>"#);

/// `chevrons-up`.
pub const CHEVRONS_UP: Icon = lucide!("chevrons-up", r#"<path d="m17 11-5-5-5 5"/><path d="m17 18-5-5-5 5"/>"#);

/// `chevrons-up-down`.
pub const CHEVRONS_UP_DOWN: Icon = lucide!("chevrons-up-down", r#"<path d="m7 15 5 5 5-5"/><path d="m7 9 5-5 5 5"/>"#);

/// `church`.
pub const CHURCH: Icon = lucide!("church", r#"<path d="M10 9h4"/><path d="M12 7v5"/><path d="M14 21v-3a2 2 0 0 0-4 0v3"/><path d="m18 9 3.52 2.147a1 1 0 0 1 .48.854V19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-6.999a1 1 0 0 1 .48-.854L6 9"/><path d="M6 21V7a1 1 0 0 1 .376-.782l5-3.999a1 1 0 0 1 1.249.001l5 4A1 1 0 0 1 18 7v14"/>"#);

/// `cigarette`.
pub const CIGARETTE: Icon = lucide!("cigarette", r#"<path d="M17 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h14"/><path d="M18 8c0-2.5-2-2.5-2-5"/><path d="M21 16a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"/><path d="M22 8c0-2.5-2-2.5-2-5"/><path d="M7 12v4"/>"#);

/// `cigarette-off`.
pub const CIGARETTE_OFF: Icon = lucide!("cigarette-off", r#"<path d="M12 12H3a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h13"/><path d="M18 8c0-2.5-2-2.5-2-5"/><path d="m2 2 20 20"/><path d="M21 12a1 1 0 0 1 1 1v2a1 1 0 0 1-.5.866"/><path d="M22 8c0-2.5-2-2.5-2-5"/><path d="M7 12v4"/>"#);

/// `circle`.
pub const CIRCLE: Icon = lucide!("circle", r#"<circle cx="12" cy="12" r="10"/>"#);

/// `circle-arrow-down`.
pub const CIRCLE_ARROW_DOWN: Icon = lucide!("circle-arrow-down", r#"<circle cx="12" cy="12" r="10"/><path d="M12 8v8"/><path d="m8 12 4 4 4-4"/>"#);

/// `circle-arrow-left`.
pub const CIRCLE_ARROW_LEFT: Icon = lucide!("circle-arrow-left", r#"<circle cx="12" cy="12" r="10"/><path d="m12 8-4 4 4 4"/><path d="M16 12H8"/>"#);

/// `circle-arrow-out-down-left`.
pub const CIRCLE_ARROW_OUT_DOWN_LEFT: Icon = lucide!("circle-arrow-out-down-left", r#"<path d="M2 12a10 10 0 1 1 10 10"/><path d="m2 22 10-10"/><path d="M8 22H2v-6"/>"#);

/// `circle-arrow-out-down-right`.
pub const CIRCLE_ARROW_OUT_DOWN_RIGHT: Icon = lucide!("circle-arrow-out-down-right", r#"<path d="M12 22a10 10 0 1 1 10-10"/><path d="M22 22 12 12"/><path d="M22 16v6h-6"/>"#);

/// `circle-arrow-out-up-left`.
pub const CIRCLE_ARROW_OUT_UP_LEFT: Icon = lucide!("circle-arrow-out-up-left", r#"<path d="M2 8V2h6"/><path d="m2 2 10 10"/><path d="M12 2A10 10 0 1 1 2 12"/>"#);

/// `circle-arrow-out-up-right`.
pub const CIRCLE_ARROW_OUT_UP_RIGHT: Icon = lucide!("circle-arrow-out-up-right", r#"<path d="M22 12A10 10 0 1 1 12 2"/><path d="M22 2 12 12"/><path d="M16 2h6v6"/>"#);

/// `circle-arrow-right`.
pub const CIRCLE_ARROW_RIGHT: Icon = lucide!("circle-arrow-right", r#"<circle cx="12" cy="12" r="10"/><path d="m12 16 4-4-4-4"/><path d="M8 12h8"/>"#);

/// `circle-arrow-up`.
pub const CIRCLE_ARROW_UP: Icon = lucide!("circle-arrow-up", r#"<circle cx="12" cy="12" r="10"/><path d="m16 12-4-4-4 4"/><path d="M12 16V8"/>"#);

/// `circle-check`.
pub const CIRCLE_CHECK: Icon = lucide!("circle-check", r#"<circle cx="12" cy="12" r="10"/><path d="m16 9-5.5 5.5L8 12"/>"#);

/// `circle-check-big`.
pub const CIRCLE_CHECK_BIG: Icon = lucide!("circle-check-big", r#"<path d="M21.801 10A10 10 0 1 1 17 3.335"/><path d="m9 11 3 3L22 4"/>"#);

/// `circle-chevron-down`.
pub const CIRCLE_CHEVRON_DOWN: Icon = lucide!("circle-chevron-down", r#"<circle cx="12" cy="12" r="10"/><path d="m16 10-4 4-4-4"/>"#);

/// `circle-chevron-left`.
pub const CIRCLE_CHEVRON_LEFT: Icon = lucide!("circle-chevron-left", r#"<circle cx="12" cy="12" r="10"/><path d="m14 16-4-4 4-4"/>"#);

/// `circle-chevron-right`.
pub const CIRCLE_CHEVRON_RIGHT: Icon = lucide!("circle-chevron-right", r#"<circle cx="12" cy="12" r="10"/><path d="m10 8 4 4-4 4"/>"#);

/// `circle-chevron-up`.
pub const CIRCLE_CHEVRON_UP: Icon = lucide!("circle-chevron-up", r#"<circle cx="12" cy="12" r="10"/><path d="m8 14 4-4 4 4"/>"#);

/// `circle-chevrons-down`.
pub const CIRCLE_CHEVRONS_DOWN: Icon = lucide!("circle-chevrons-down", r#"<path d="M9 14L12 17L15 14"/><path d="M9 8L12 11L15 8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-chevrons-left`.
pub const CIRCLE_CHEVRONS_LEFT: Icon = lucide!("circle-chevrons-left", r#"<path d="M10 9L7 12L10 15"/><path d="M16 9L13 12L16 15"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-chevrons-right`.
pub const CIRCLE_CHEVRONS_RIGHT: Icon = lucide!("circle-chevrons-right", r#"<path d="M14 15L17 12L14 9"/><path d="M8 15L11 12L8 9"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-chevrons-up`.
pub const CIRCLE_CHEVRONS_UP: Icon = lucide!("circle-chevrons-up", r#"<path d="M15 10L12 7L9 10"/><path d="M15 16L12 13L9 16"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-dashed`.
pub const CIRCLE_DASHED: Icon = lucide!("circle-dashed", r#"<path d="M10.1 2.182a10 10 0 0 1 3.8 0"/><path d="M13.9 21.818a10 10 0 0 1-3.8 0"/><path d="M17.609 3.721a10 10 0 0 1 2.69 2.7"/><path d="M2.182 13.9a10 10 0 0 1 0-3.8"/><path d="M20.279 17.609a10 10 0 0 1-2.7 2.69"/><path d="M21.818 10.1a10 10 0 0 1 0 3.8"/><path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"/><path d="M6.391 20.279a10 10 0 0 1-2.69-2.7"/>"#);

/// `circle-dashed-check`.
pub const CIRCLE_DASHED_CHECK: Icon = lucide!("circle-dashed-check", r#"<path d="M10.1 2.182a10 10 0 013.8 0"/><path d="M13.9 21.818a10 10 0 01-3.8 0"/><path d="m16 9-5.5 5.5L8 12"/><path d="M17.609 3.721a10 10 0 012.69 2.7"/><path d="M2.182 13.9a10 10 0 010-3.8"/><path d="M20.279 17.609a10 10 0 01-2.7 2.69"/><path d="M21.818 10.1a10 10 0 010 3.8"/><path d="M3.721 6.391a10 10 0 012.7-2.69"/><path d="M6.391 20.279a10 10 0 01-2.69-2.7"/>"#);

/// `circle-divide`.
pub const CIRCLE_DIVIDE: Icon = lucide!("circle-divide", r#"<circle cx="12" cy="12" r="10"/><line x1="8" x2="16" y1="12" y2="12"/><line x1="12" x2="12" y1="16" y2="16"/><line x1="12" x2="12" y1="8" y2="8"/>"#);

/// `circle-dollar-sign`.
pub const CIRCLE_DOLLAR_SIGN: Icon = lucide!("circle-dollar-sign", r#"<circle cx="12" cy="12" r="10"/><path d="M16 8h-6a2 2 0 1 0 0 4h4a2 2 0 1 1 0 4H8"/><path d="M12 18V6"/>"#);

/// `circle-dot`.
pub const CIRCLE_DOT: Icon = lucide!("circle-dot", r#"<circle cx="12" cy="12" r="1"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-dot-dashed`.
pub const CIRCLE_DOT_DASHED: Icon = lucide!("circle-dot-dashed", r#"<path d="M10.1 2.18a9.93 9.93 0 0 1 3.8 0"/><path d="M17.6 3.71a9.95 9.95 0 0 1 2.69 2.7"/><path d="M21.82 10.1a9.93 9.93 0 0 1 0 3.8"/><path d="M20.29 17.6a9.95 9.95 0 0 1-2.7 2.69"/><path d="M13.9 21.82a9.94 9.94 0 0 1-3.8 0"/><path d="M6.4 20.29a9.95 9.95 0 0 1-2.69-2.7"/><path d="M2.18 13.9a9.93 9.93 0 0 1 0-3.8"/><path d="M3.71 6.4a9.95 9.95 0 0 1 2.7-2.69"/><circle cx="12" cy="12" r="1"/>"#);

/// `circle-ellipsis`.
pub const CIRCLE_ELLIPSIS: Icon = lucide!("circle-ellipsis", r#"<circle cx="12" cy="12" r="10"/><path d="M17 12h.01"/><path d="M12 12h.01"/><path d="M7 12h.01"/>"#);

/// `circle-equal`.
pub const CIRCLE_EQUAL: Icon = lucide!("circle-equal", r#"<circle cx="12" cy="12" r="10"/><path d="M7 10h10"/><path d="M7 14h10"/>"#);

/// `circle-euro`.
pub const CIRCLE_EURO: Icon = lucide!("circle-euro", r#"<path d="M15 9.4a4 4 0 1 0 0 5.2"/><path d="M7 12h5"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-exclamation-point`.
pub const CIRCLE_EXCLAMATION_POINT: Icon = lucide!("circle-exclamation-point", r#"<circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#);

/// `circle-fading-arrow-up`.
pub const CIRCLE_FADING_ARROW_UP: Icon = lucide!("circle-fading-arrow-up", r#"<path d="M12 2a10 10 0 0 1 7.38 16.75"/><path d="m16 12-4-4-4 4"/><path d="M12 16V8"/><path d="M2.5 8.875a10 10 0 0 0-.5 3"/><path d="M2.83 16a10 10 0 0 0 2.43 3.4"/><path d="M4.636 5.235a10 10 0 0 1 .891-.857"/><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"/>"#);

/// `circle-fading-check`.
pub const CIRCLE_FADING_CHECK: Icon = lucide!("circle-fading-check", r#"<path d="M12 2a10 10 0 017.38 16.75"/><path d="m16 9-5.5 5.5L8 12"/><path d="M2.5 8.875a10 10 0 00-.5 3"/><path d="M2.83 16a10 10 0 002.43 3.4"/><path d="M4.636 5.235a10 10 0 01.891-.857"/><path d="M8.644 21.42a10 10 0 007.631-.38"/>"#);

/// `circle-fading-play`.
pub const CIRCLE_FADING_PLAY: Icon = lucide!("circle-fading-play", r#"<path d="M12 2a10 10 0 0 1 7.38 16.75"/><path d="M9 9.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997A1 1 0 0 1 9 14.996z"/><path d="M2.5 8.875a10 10 0 0 0-.5 3"/><path d="M2.83 16a10 10 0 0 0 2.43 3.4"/><path d="M4.636 5.235a10 10 0 0 1 .891-.857"/><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"/>"#);

/// `circle-fading-plus`.
pub const CIRCLE_FADING_PLUS: Icon = lucide!("circle-fading-plus", r#"<path d="M12 2a10 10 0 0 1 7.38 16.75"/><path d="M12 8v8"/><path d="M16 12H8"/><path d="M2.5 8.875a10 10 0 0 0-.5 3"/><path d="M2.83 16a10 10 0 0 0 2.43 3.4"/><path d="M4.636 5.235a10 10 0 0 1 .891-.857"/><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"/>"#);

/// `circle-gauge`.
pub const CIRCLE_GAUGE: Icon = lucide!("circle-gauge", r#"<path d="M15.6 2.7a10 10 0 1 0 5.7 5.7"/><circle cx="12" cy="12" r="2"/><path d="M13.4 10.6 19 5"/>"#);

/// `circle-letter-a`.
pub const CIRCLE_LETTER_A: Icon = lucide!("circle-letter-a", r#"<path d="M15 14H9"/><circle cx="12" cy="12" r="10"/><path d="m8 16 3.453-7.648a.6.6 0 0 1 1.094 0L16 16"/>"#);

/// `circle-letter-b`.
pub const CIRCLE_LETTER_B: Icon = lucide!("circle-letter-b", r#"<path d="M9 12h4.846a2.24 2.24 0 011.523.586 2 2 0 01.001 2.828 2.24 2.24 0 01-1.524.586H9.5a.5.5 0 01-.5-.5v-7a.5.5 0 01.5-.5H13a2 2 0 010 4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-c`.
pub const CIRCLE_LETTER_C: Icon = lucide!("circle-letter-c", r#"<path d="M15 9.354a4 4 0 1 0 0 5.292"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-d`.
pub const CIRCLE_LETTER_D: Icon = lucide!("circle-letter-d", r#"<path d="M12 8a4 4 0 010 8H9.412A.41.41 0 019 15.588V8.412A.41.41 0 019.412 8z"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-e`.
pub const CIRCLE_LETTER_E: Icon = lucide!("circle-letter-e", r#"<path d="M15 8H9.5a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5H15"/><path d="M9 12h4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-f`.
pub const CIRCLE_LETTER_F: Icon = lucide!("circle-letter-f", r#"<path d="M9 16V8.5a.5.5 0 0 1 .5-.5H15"/><path d="M9 12h4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-g`.
pub const CIRCLE_LETTER_G: Icon = lucide!("circle-letter-g", r#"<path d="M12.5 12H15v2.268a.88.88 0 01-.263.65A4 4 0 1112 8h2.5"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-h`.
pub const CIRCLE_LETTER_H: Icon = lucide!("circle-letter-h", r#"<path d="M9 16V8"/><path d="M15 16V8"/><path d="M9 12h6"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-i`.
pub const CIRCLE_LETTER_I: Icon = lucide!("circle-letter-i", r#"<path d="M12 16V8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-j`.
pub const CIRCLE_LETTER_J: Icon = lucide!("circle-letter-j", r#"<path d="M14 8v5.5a1 1 0 01-5 0V13"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-k`.
pub const CIRCLE_LETTER_K: Icon = lucide!("circle-letter-k", r#"<path d="M9 16V8"/><path d="m11 12 4 4"/><path d="m15 8-6 6"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-l`.
pub const CIRCLE_LETTER_L: Icon = lucide!("circle-letter-l", r#"<path d="M9 8v7.5a.5.5 0 0 0 .5.5H15"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-m`.
pub const CIRCLE_LETTER_M: Icon = lucide!("circle-letter-m", r#"<path d="M8 16.001v-7.5a.5.5 0 0 1 .9-.3l2.7 3.599a.5.5 0 0 0 .8 0l2.7-3.6a.5.5 0 0 1 .9.3v7.501"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-n`.
pub const CIRCLE_LETTER_N: Icon = lucide!("circle-letter-n", r#"<path d="M9 16V8.576a.496.496 0 01.937-.243l4.126 7.43A.496.496 0 0015 15.52V8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-o`.
pub const CIRCLE_LETTER_O: Icon = lucide!("circle-letter-o", r#"<circle cx="12" cy="12" r="4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-p`.
pub const CIRCLE_LETTER_P: Icon = lucide!("circle-letter-p", r#"<path d="M9 12h4a2 2 0 0 0 0-4H9.5a.5.5 0 0 0-.5.5V16"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-p-off`.
pub const CIRCLE_LETTER_P_OFF: Icon = lucide!("circle-letter-p-off", r#"<path d="M12 12h1a2 2 0 000-4H9.5a.5.5 0 00-.5.5V9"/><path d="M15.652 21.31A10 10 0 012.691 8.348"/><path d="M22 22 2 2"/><path d="M4.929 4.929a10 10 0 0116.729 4.483 10 10 0 01-2.588 9.658"/><path d="M9 14.657V16"/>"#);

/// `circle-letter-q`.
pub const CIRCLE_LETTER_Q: Icon = lucide!("circle-letter-q", r#"<circle cx="12" cy="12" r="4"/><path d="m16 16-2.5-2.5"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-r`.
pub const CIRCLE_LETTER_R: Icon = lucide!("circle-letter-r", r#"<path d="m11 12 4 4"/><path d="M9 12h4a2 2 0 0 0 0-4H9.5a.5.5 0 0 0-.5.5V16"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-s`.
pub const CIRCLE_LETTER_S: Icon = lucide!("circle-letter-s", r#"<path d="M14.5 8H11a2 2 0 100 4h2a2 2 0 110 4H9"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-t`.
pub const CIRCLE_LETTER_T: Icon = lucide!("circle-letter-t", r#"<path d="M12 16V8"/><path d="M9 8h6"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-u`.
pub const CIRCLE_LETTER_U: Icon = lucide!("circle-letter-u", r#"<path d="M15 8v5a3 3 0 1 1-6 0V8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-v`.
pub const CIRCLE_LETTER_V: Icon = lucide!("circle-letter-v", r#"<path d="m9 8 2.525 7.66a.5.5 0 0 0 .95 0L15 8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-w`.
pub const CIRCLE_LETTER_W: Icon = lucide!("circle-letter-w", r#"<path d="m16 8-1.591 7.598a.497.497 0 01-.97.038l-.96-3.276a.5.5 0 00-.959 0l-.96 3.276a.496.496 0 01-.969-.038L8 8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-x`.
pub const CIRCLE_LETTER_X: Icon = lucide!("circle-letter-x", r#"<path d="m9 16 6-8"/><path d="m9 8 6 8"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-y`.
pub const CIRCLE_LETTER_Y: Icon = lucide!("circle-letter-y", r#"<path d="m9 8 3 4 3-4"/><path d="M12 12v4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-letter-z`.
pub const CIRCLE_LETTER_Z: Icon = lucide!("circle-letter-z", r#"<path d="M9 8h5.501a.5.5 0 0 1 .37.836L9.13 15.164A.5.5 0 0 0 9.5 16H15"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-minus`.
pub const CIRCLE_MINUS: Icon = lucide!("circle-minus", r#"<circle cx="12" cy="12" r="10"/><path d="M8 12h8"/>"#);

/// `circle-off`.
pub const CIRCLE_OFF: Icon = lucide!("circle-off", r#"<path d="m2 2 20 20"/><path d="M8.35 2.69A10 10 0 0 1 21.3 15.65"/><path d="M19.08 19.08A10 10 0 1 1 4.92 4.92"/>"#);

/// `circle-pause`.
pub const CIRCLE_PAUSE: Icon = lucide!("circle-pause", r#"<circle cx="12" cy="12" r="10"/><line x1="10" x2="10" y1="15" y2="9"/><line x1="14" x2="14" y1="15" y2="9"/>"#);

/// `circle-percent`.
pub const CIRCLE_PERCENT: Icon = lucide!("circle-percent", r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="M9 9h.01"/><path d="M15 15h.01"/>"#);

/// `circle-pile`.
pub const CIRCLE_PILE: Icon = lucide!("circle-pile", r#"<circle cx="12" cy="19" r="2"/><circle cx="12" cy="5" r="2"/><circle cx="16" cy="12" r="2"/><circle cx="20" cy="19" r="2"/><circle cx="4" cy="19" r="2"/><circle cx="8" cy="12" r="2"/>"#);

/// `circle-play`.
pub const CIRCLE_PLAY: Icon = lucide!("circle-play", r#"<path d="M9 9.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997A1 1 0 0 1 9 14.996z"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-plus`.
pub const CIRCLE_PLUS: Icon = lucide!("circle-plus", r#"<circle cx="12" cy="12" r="10"/><path d="M8 12h8"/><path d="M12 8v8"/>"#);

/// `circle-pound-sterling`.
pub const CIRCLE_POUND_STERLING: Icon = lucide!("circle-pound-sterling", r#"<circle cx="12" cy="12" r="10"/><path d="M10 16V9.5a1 1 0 0 1 5 0"/><path d="M8 12h4"/><path d="M8 16h7"/>"#);

/// `circle-power`.
pub const CIRCLE_POWER: Icon = lucide!("circle-power", r#"<circle cx="12" cy="12" r="10"/><path d="M12 7v4"/><path d="M7.998 9.003a5 5 0 1 0 8-.005"/>"#);

/// `circle-question-mark`.
pub const CIRCLE_QUESTION_MARK: Icon = lucide!("circle-question-mark", r#"<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><path d="M12 17h.01"/>"#);

/// `circle-slash`.
pub const CIRCLE_SLASH: Icon = lucide!("circle-slash", r#"<circle cx="12" cy="12" r="10"/><line x1="9" x2="15" y1="15" y2="9"/>"#);

/// `circle-slash-2`.
pub const CIRCLE_SLASH_2: Icon = lucide!("circle-slash-2", r#"<circle cx="12" cy="12" r="10"/><path d="M22 2 2 22"/>"#);

/// `circle-small`.
pub const CIRCLE_SMALL: Icon = lucide!("circle-small", r#"<circle cx="12" cy="12" r="6"/>"#);

/// `circle-star`.
pub const CIRCLE_STAR: Icon = lucide!("circle-star", r#"<circle cx="12" cy="12" r="10"/><path d="M11.051 7.616a1 1 0 0 1 1.909.024l.737 1.452a1 1 0 0 0 .737.535l1.634.256a1 1 0 0 1 .588 1.806l-1.172 1.168a1 1 0 0 0-.282.866l.259 1.613a1 1 0 0 1-1.541 1.134l-1.465-.75a1 1 0 0 0-.912 0l-1.465.75a1 1 0 0 1-1.539-1.133l.258-1.613a1 1 0 0 0-.282-.867l-1.156-1.152a1 1 0 0 1 .572-1.822l1.633-.256a1 1 0 0 0 .737-.535z"/>"#);

/// `circle-stop`.
pub const CIRCLE_STOP: Icon = lucide!("circle-stop", r#"<circle cx="12" cy="12" r="10"/><rect x="9" y="9" width="6" height="6" rx="1"/>"#);

/// `circle-user`.
pub const CIRCLE_USER: Icon = lucide!("circle-user", r#"<circle cx="12" cy="12" r="10"/><circle cx="12" cy="10" r="3"/><path d="M7 20.662V19a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v1.662"/>"#);

/// `circle-user-round`.
pub const CIRCLE_USER_ROUND: Icon = lucide!("circle-user-round", r#"<path d="M17.925 20.056a6 6 0 0 0-11.851.001"/><circle cx="12" cy="11" r="4"/><circle cx="12" cy="12" r="10"/>"#);

/// `circle-x`.
pub const CIRCLE_X: Icon = lucide!("circle-x", r#"<circle cx="12" cy="12" r="10"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#);

/// `circuit-board`.
pub const CIRCUIT_BOARD: Icon = lucide!("circuit-board", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M11 9h4a2 2 0 0 0 2-2V3"/><circle cx="9" cy="9" r="2"/><path d="M7 21v-4a2 2 0 0 1 2-2h4"/><circle cx="15" cy="15" r="2"/>"#);

/// `citrus`.
pub const CITRUS: Icon = lucide!("citrus", r#"<path d="M21.66 17.67a1.08 1.08 0 0 1-.04 1.6A12 12 0 0 1 4.73 2.38a1.1 1.1 0 0 1 1.61-.04z"/><path d="M19.65 15.66A8 8 0 0 1 8.35 4.34"/><path d="m14 10-5.5 5.5"/><path d="M14 17.85V10H6.15"/>"#);

/// `clapperboard`.
pub const CLAPPERBOARD: Icon = lucide!("clapperboard", r#"<path d="m12.296 3.464 3.02 3.956"/><path d="M20.2 6 3 11l-.9-2.4c-.3-1.1.3-2.2 1.3-2.5l13.5-4c1.1-.3 2.2.3 2.5 1.3z"/><path d="M3 11h18v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="m6.18 5.276 3.1 3.899"/>"#);

/// `clef-alto`.
pub const CLEF_ALTO: Icon = lucide!("clef-alto", r#"<path d="M10 4v16"/><path d="M14 4.764a3 3 0 1 1-.152 4.327A4 4 0 0 1 10 12a4 4 0 0 1 3.848 2.909A3 3 0 1 1 14 19.236"/><path d="M6 4v16"/>"#);

/// `clef-bass`.
pub const CLEF_BASS: Icon = lucide!("clef-bass", r#"<path d="M19 11h.01"/><path d="M19 6h.01"/><path d="M5 8c0-4 4-4 4-4 6 0 6 6 6 6 0 7-10 11-10 11"/><circle cx="7" cy="8" r="2"/>"#);

/// `clef-treble`.
pub const CLEF_TREBLE: Icon = lucide!("clef-treble", r#"<path d="M10.586 21.414a2 2 0 0 0 3.378-1.791L11.036 4.377a2 2 0 1 1 3.378 1.037C12.414 7.414 7 8 7 13a5 5 0 0 0 5 5 5 4 0 0 0 5-4 3 3 0 0 0-3-3 3 2 0 0 0-3 2"/>"#);

/// `clipboard`.
pub const CLIPBOARD: Icon = lucide!("clipboard", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/>"#);

/// `clipboard-check`.
pub const CLIPBOARD_CHECK: Icon = lucide!("clipboard-check", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="m9 14 2 2 4-4"/>"#);

/// `clipboard-clock`.
pub const CLIPBOARD_CLOCK: Icon = lucide!("clipboard-clock", r#"<path d="M16 14v2.2l1.6 1"/><path d="M16 4h2a2 2 0 0 1 2 2v.832"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2"/><circle cx="16" cy="16" r="6"/><rect x="8" y="2" width="8" height="4" rx="1"/>"#);

/// `clipboard-copy`.
pub const CLIPBOARD_COPY: Icon = lucide!("clipboard-copy", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2"/><path d="M16 4h2a2 2 0 0 1 2 2v4"/><path d="M21 14H11"/><path d="m15 10-4 4 4 4"/>"#);

/// `clipboard-list`.
pub const CLIPBOARD_LIST: Icon = lucide!("clipboard-list", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="M12 11h4"/><path d="M12 16h4"/><path d="M8 11h.01"/><path d="M8 16h.01"/>"#);

/// `clipboard-minus`.
pub const CLIPBOARD_MINUS: Icon = lucide!("clipboard-minus", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="M9 14h6"/>"#);

/// `clipboard-paste`.
pub const CLIPBOARD_PASTE: Icon = lucide!("clipboard-paste", r#"<path d="M11 14h10"/><path d="M16 4h2a2 2 0 0 1 2 2v1.344"/><path d="m17 18 4-4-4-4"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 1.793-1.113"/><rect x="8" y="2" width="8" height="4" rx="1"/>"#);

/// `clipboard-pen`.
pub const CLIPBOARD_PEN: Icon = lucide!("clipboard-pen", r#"<path d="M16 4h2a2 2 0 0 1 2 2v2"/><path d="M21.34 15.664a1 1 0 1 0-3.004-3.004l-5.01 5.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/><path d="M8 22H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><rect x="8" y="2" width="8" height="4" rx="1"/>"#);

/// `clipboard-pen-line`.
pub const CLIPBOARD_PEN_LINE: Icon = lucide!("clipboard-pen-line", r#"<rect width="8" height="4" x="8" y="2" rx="1"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-.5"/><path d="M16 4h2a2 2 0 0 1 1.73 1"/><path d="M8 18h1"/><path d="M21.378 12.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/>"#);

/// `clipboard-plus`.
pub const CLIPBOARD_PLUS: Icon = lucide!("clipboard-plus", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="M9 14h6"/><path d="M12 17v-6"/>"#);

/// `clipboard-type`.
pub const CLIPBOARD_TYPE: Icon = lucide!("clipboard-type", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="M9 12v-1h6v1"/><path d="M11 17h2"/><path d="M12 11v6"/>"#);

/// `clipboard-x`.
pub const CLIPBOARD_X: Icon = lucide!("clipboard-x", r#"<rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"/><path d="m14.5 11.5-5 5"/><path d="m9.5 11.5 5 5"/>"#);

/// `clock`.
pub const CLOCK: Icon = lucide!("clock", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>"#);

/// `clock-1`.
pub const CLOCK_1: Icon = lucide!("clock-1", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l2-4"/>"#);

/// `clock-10`.
pub const CLOCK_10: Icon = lucide!("clock-10", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l-4-2"/>"#);

/// `clock-11`.
pub const CLOCK_11: Icon = lucide!("clock-11", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l-2-4"/>"#);

/// `clock-12`.
pub const CLOCK_12: Icon = lucide!("clock-12", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6"/>"#);

/// `clock-2`.
pub const CLOCK_2: Icon = lucide!("clock-2", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l4-2"/>"#);

/// `clock-3`.
pub const CLOCK_3: Icon = lucide!("clock-3", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6h4"/>"#);

/// `clock-4`.
pub const CLOCK_4: Icon = lucide!("clock-4", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l4 2"/>"#);

/// `clock-5`.
pub const CLOCK_5: Icon = lucide!("clock-5", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l2 4"/>"#);

/// `clock-6`.
pub const CLOCK_6: Icon = lucide!("clock-6", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v10"/>"#);

/// `clock-7`.
pub const CLOCK_7: Icon = lucide!("clock-7", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l-2 4"/>"#);

/// `clock-8`.
pub const CLOCK_8: Icon = lucide!("clock-8", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6l-4 2"/>"#);

/// `clock-9`.
pub const CLOCK_9: Icon = lucide!("clock-9", r#"<circle cx="12" cy="12" r="10"/><path d="M12 6v6H8"/>"#);

/// `clock-arrow-down`.
pub const CLOCK_ARROW_DOWN: Icon = lucide!("clock-arrow-down", r#"<path d="M12 6v6l2 1"/><path d="M12.337 21.994a10 10 0 1 1 9.588-8.767"/><path d="m14 18 4 4 4-4"/><path d="M18 14v8"/>"#);

/// `clock-arrow-left`.
pub const CLOCK_ARROW_LEFT: Icon = lucide!("clock-arrow-left", r#"<path d="M12 6v6l1.5.8"/><path d="M12.338 21.994a10 10 0 1 1 9.587-8.767"/><path d="M14 18h8"/><path d="m18 22-4-4 4-4"/>"#);

/// `clock-arrow-right`.
pub const CLOCK_ARROW_RIGHT: Icon = lucide!("clock-arrow-right", r#"<path d="M12 6v6l2 1"/><path d="M13.5 21.885A10 10 0 1 1 22 12"/><path d="M14 18h8"/><path d="m18 22 4-4-4-4"/>"#);

/// `clock-arrow-up`.
pub const CLOCK_ARROW_UP: Icon = lucide!("clock-arrow-up", r#"<path d="M12 6v6l1.56.78"/><path d="M13.227 21.925a10 10 0 1 1 8.767-9.588"/><path d="m14 18 4-4 4 4"/><path d="M18 22v-8"/>"#);

/// `clock-check`.
pub const CLOCK_CHECK: Icon = lucide!("clock-check", r#"<path d="M21.95 13a10 10 0 1 0-8.685 8.92"/><path d="M12 6v6l4 2"/><path d="m16 19 2 2 4-4"/>"#);

/// `clock-exclamation-point`.
pub const CLOCK_EXCLAMATION_POINT: Icon = lucide!("clock-exclamation-point", r#"<path d="M12 6v6l4 2"/><path d="M20 12v5"/><path d="M20 21h.01"/><path d="M21.25 8.2A10 10 0 1 0 16 21.16"/>"#);

/// `clock-fading`.
pub const CLOCK_FADING: Icon = lucide!("clock-fading", r#"<path d="M12 2a10 10 0 0 1 7.38 16.75"/><path d="M12 6v6l4 2"/><path d="M2.5 8.875a10 10 0 0 0-.5 3"/><path d="M2.83 16a10 10 0 0 0 2.43 3.4"/><path d="M4.636 5.235a10 10 0 0 1 .891-.857"/><path d="M8.644 21.42a10 10 0 0 0 7.631-.38"/>"#);

/// `clock-plus`.
pub const CLOCK_PLUS: Icon = lucide!("clock-plus", r#"<path d="M12 6v6l3.644 1.822"/><path d="M16 19h6"/><path d="M19 16v6"/><path d="M21.92 13.267a10 10 0 1 0-8.653 8.653"/>"#);

/// `closed-caption`.
pub const CLOSED_CAPTION: Icon = lucide!("closed-caption", r#"<path d="M10 9.17a3 3 0 1 0 0 5.66"/><path d="M17 9.17a3 3 0 1 0 0 5.66"/><rect x="2" y="5" width="20" height="14" rx="2"/>"#);

/// `cloud`.
pub const CLOUD: Icon = lucide!("cloud", r#"<path d="M17.5 19H9a7 7 0 1 1 6.71-9h1.79a4.5 4.5 0 1 1 0 9Z"/>"#);

/// `cloud-backup`.
pub const CLOUD_BACKUP: Icon = lucide!("cloud-backup", r#"<path d="M21 15.251A4.5 4.5 0 0 0 17.5 8h-1.79A7 7 0 1 0 3 13.607"/><path d="M7 11v4h4"/><path d="M8 19a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5 4.82 4.82 0 0 0-3.41 1.41L7 15"/>"#);

/// `cloud-check`.
pub const CLOUD_CHECK: Icon = lucide!("cloud-check", r#"<path d="m17 15-5.5 5.5L9 18"/><path d="M5.516 16.07A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 3.501 7.327"/>"#);

/// `cloud-cog`.
pub const CLOUD_COG: Icon = lucide!("cloud-cog", r#"<path d="m10.852 19.772-.383.924"/><path d="m13.148 14.228.383-.923"/><path d="M13.148 19.772a3 3 0 1 0-2.296-5.544l-.383-.923"/><path d="m13.53 20.696-.382-.924a3 3 0 1 1-2.296-5.544"/><path d="m14.772 15.852.923-.383"/><path d="m14.772 18.148.923.383"/><path d="M4.2 15.1a7 7 0 1 1 9.93-9.858A7 7 0 0 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.2"/><path d="m9.228 15.852-.923-.383"/><path d="m9.228 18.148-.923.383"/>"#);

/// `cloud-download`.
pub const CLOUD_DOWNLOAD: Icon = lucide!("cloud-download", r#"<path d="M12 13v8l-4-4"/><path d="m12 21 4-4"/><path d="M4.393 15.269A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.436 8.284"/>"#);

/// `cloud-drizzle`.
pub const CLOUD_DRIZZLE: Icon = lucide!("cloud-drizzle", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="M8 19v1"/><path d="M8 14v1"/><path d="M16 19v1"/><path d="M16 14v1"/><path d="M12 21v1"/><path d="M12 16v1"/>"#);

/// `cloud-exclamation-point`.
pub const CLOUD_EXCLAMATION_POINT: Icon = lucide!("cloud-exclamation-point", r#"<path d="M12 12v4"/><path d="M12 20h.01"/><path d="M8.128 16.949A7 7 0 1 1 15.71 8h1.79a1 1 0 0 1 0 9h-1.642"/>"#);

/// `cloud-fog`.
pub const CLOUD_FOG: Icon = lucide!("cloud-fog", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="M16 17H7"/><path d="M17 21H9"/>"#);

/// `cloud-hail`.
pub const CLOUD_HAIL: Icon = lucide!("cloud-hail", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="M16 14v2"/><path d="M8 14v2"/><path d="M16 20h.01"/><path d="M8 20h.01"/><path d="M12 16v2"/><path d="M12 22h.01"/>"#);

/// `cloud-lightning`.
pub const CLOUD_LIGHTNING: Icon = lucide!("cloud-lightning", r#"<path d="M6 16.326A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 .5 8.973"/><path d="m13 12-3 5h4l-3 5"/>"#);

/// `cloud-moon`.
pub const CLOUD_MOON: Icon = lucide!("cloud-moon", r#"<path d="M13 16a3 3 0 0 1 0 6H7a5 5 0 1 1 4.9-6z"/><path d="M18.376 14.512a6 6 0 0 0 3.461-4.127c.148-.625-.659-.97-1.248-.714a4 4 0 0 1-5.259-5.26c.255-.589-.09-1.395-.716-1.248a6 6 0 0 0-4.594 5.36"/>"#);

/// `cloud-moon-rain`.
pub const CLOUD_MOON_RAIN: Icon = lucide!("cloud-moon-rain", r#"<path d="M11 20v2"/><path d="M18.376 14.512a6 6 0 0 0 3.461-4.127c.148-.625-.659-.97-1.248-.714a4 4 0 0 1-5.259-5.26c.255-.589-.09-1.395-.716-1.248a6 6 0 0 0-4.594 5.36"/><path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"/><path d="M7 19v2"/>"#);

/// `cloud-off`.
pub const CLOUD_OFF: Icon = lucide!("cloud-off", r#"<path d="M10.94 5.274A7 7 0 0 1 15.71 10h1.79a4.5 4.5 0 0 1 4.222 6.057"/><path d="M18.796 18.81A4.5 4.5 0 0 1 17.5 19H9A7 7 0 0 1 5.79 5.78"/><path d="m2 2 20 20"/>"#);

/// `cloud-rain`.
pub const CLOUD_RAIN: Icon = lucide!("cloud-rain", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="M16 14v6"/><path d="M8 14v6"/><path d="M12 16v6"/>"#);

/// `cloud-rain-wind`.
pub const CLOUD_RAIN_WIND: Icon = lucide!("cloud-rain-wind", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="m9.2 22 3-7"/><path d="m9 13-3 7"/><path d="m17 13-3 7"/>"#);

/// `cloud-snow`.
pub const CLOUD_SNOW: Icon = lucide!("cloud-snow", r#"<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="M8 15h.01"/><path d="M8 19h.01"/><path d="M12 17h.01"/><path d="M12 21h.01"/><path d="M16 15h.01"/><path d="M16 19h.01"/>"#);

/// `cloud-sun`.
pub const CLOUD_SUN: Icon = lucide!("cloud-sun", r#"<path d="M12 2v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="M20 12h2"/><path d="m19.07 4.93-1.41 1.41"/><path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"/><path d="M13 22H7a5 5 0 1 1 4.9-6H13a3 3 0 0 1 0 6Z"/>"#);

/// `cloud-sun-rain`.
pub const CLOUD_SUN_RAIN: Icon = lucide!("cloud-sun-rain", r#"<path d="M12 2v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="M20 12h2"/><path d="m19.07 4.93-1.41 1.41"/><path d="M15.947 12.65a4 4 0 0 0-5.925-4.128"/><path d="M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24"/><path d="M11 20v2"/><path d="M7 19v2"/>"#);

/// `cloud-sync`.
pub const CLOUD_SYNC: Icon = lucide!("cloud-sync", r#"<path d="m17 18-1.535 1.605a5 5 0 0 1-8-1.5"/><path d="M17 22v-4h-4"/><path d="M20.996 15.251A4.5 4.5 0 0 0 17.495 8h-1.79a7 7 0 1 0-12.709 5.607"/><path d="M7 10v4h4"/><path d="m7 14 1.535-1.605a5 5 0 0 1 8 1.5"/>"#);

/// `cloud-upload`.
pub const CLOUD_UPLOAD: Icon = lucide!("cloud-upload", r#"<path d="M12 13v8"/><path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/><path d="m8 17 4-4 4 4"/>"#);

/// `cloudy`.
pub const CLOUDY: Icon = lucide!("cloudy", r#"<path d="M17.5 12a1 1 0 1 1 0 9H9.006a7 7 0 1 1 6.702-9z"/><path d="M21.832 9A3 3 0 0 0 19 7h-2.207a5.5 5.5 0 0 0-10.72.61"/>"#);

/// `clover`.
pub const CLOVER: Icon = lucide!("clover", r#"<path d="M16.17 7.83 2 22"/><path d="M4.02 12a2.827 2.827 0 1 1 3.81-4.17A2.827 2.827 0 1 1 12 4.02a2.827 2.827 0 1 1 4.17 3.81A2.827 2.827 0 1 1 19.98 12a2.827 2.827 0 1 1-3.81 4.17A2.827 2.827 0 1 1 12 19.98a2.827 2.827 0 1 1-4.17-3.81A1 1 0 1 1 4 12"/><path d="m7.83 7.83 8.34 8.34"/>"#);

/// `club`.
pub const CLUB: Icon = lucide!("club", r#"<path d="M17.28 9.05a5.5 5.5 0 1 0-10.56 0A5.5 5.5 0 1 0 12 17.66a5.5 5.5 0 1 0 5.28-8.6Z"/><path d="M12 17.66L12 22"/>"#);

/// `code`.
pub const CODE: Icon = lucide!("code", r#"<path d="m16 18 6-6-6-6"/><path d="m8 6-6 6 6 6"/>"#);

/// `code-xml`.
pub const CODE_XML: Icon = lucide!("code-xml", r#"<path d="m18 16 4-4-4-4"/><path d="m6 8-4 4 4 4"/><path d="m14.5 4-5 16"/>"#);

/// `coffee`.
pub const COFFEE: Icon = lucide!("coffee", r#"<path d="M10 2v2"/><path d="M14 2v2"/><path d="M16 8a1 1 0 0 1 1 1v8a4 4 0 0 1-4 4H7a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1h14a4 4 0 1 1 0 8h-1"/><path d="M6 2v2"/>"#);

/// `cog`.
pub const COG: Icon = lucide!("cog", r#"<path d="M11 10.27 7 3.34"/><path d="m11 13.73-4 6.93"/><path d="M12 22v-2"/><path d="M12 2v2"/><path d="M14 12h8"/><path d="m17 20.66-1-1.73"/><path d="m17 3.34-1 1.73"/><path d="M2 12h2"/><path d="m20.66 17-1.73-1"/><path d="m20.66 7-1.73 1"/><path d="m3.34 17 1.73-1"/><path d="m3.34 7 1.73 1"/><circle cx="12" cy="12" r="2"/><circle cx="12" cy="12" r="8"/>"#);

/// `coins`.
pub const COINS: Icon = lucide!("coins", r#"<path d="M13.744 17.736a6 6 0 1 1-7.48-7.48"/><path d="M15 6h1v4"/><path d="m6.134 14.768.866-.5 2 3.464"/><circle cx="16" cy="8" r="6"/>"#);

/// `columns-2`.
pub const COLUMNS_2: Icon = lucide!("columns-2", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 3v18"/>"#);

/// `columns-3`.
pub const COLUMNS_3: Icon = lucide!("columns-3", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/><path d="M15 3v18"/>"#);

/// `columns-3-cog`.
pub const COLUMNS_3_COG: Icon = lucide!("columns-3-cog", r#"<path d="M10.6 21H5a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v5.6"/><path d="m14.305 19.53.923-.382"/><path d="M15 3v7.6"/><path d="m15.229 16.852-.924-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="m20.773 16.852.922-.383"/><path d="m20.773 19.148.922.383"/><path d="M9 3v18"/><circle cx="18" cy="18" r="3"/>"#);

/// `columns-4`.
pub const COLUMNS_4: Icon = lucide!("columns-4", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7.5 3v18"/><path d="M12 3v18"/><path d="M16.5 3v18"/>"#);

/// `combine`.
pub const COMBINE: Icon = lucide!("combine", r#"<path d="M14 3a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1"/><path d="M19 3a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1"/><path d="m7 15 3 3"/><path d="m7 21 3-3H5a2 2 0 0 1-2-2v-2"/><rect x="14" y="14" width="7" height="7" rx="1"/><rect x="3" y="3" width="7" height="7" rx="1"/>"#);

/// `command`.
pub const COMMAND: Icon = lucide!("command", r#"<path d="M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3"/>"#);

/// `compass`.
pub const COMPASS: Icon = lucide!("compass", r#"<circle cx="12" cy="12" r="10"/><path d="m16.24 7.76-1.804 5.411a2 2 0 0 1-1.265 1.265L7.76 16.24l1.804-5.411a2 2 0 0 1 1.265-1.265z"/>"#);

/// `component`.
pub const COMPONENT: Icon = lucide!("component", r#"<path d="M15.536 11.293a1 1 0 0 0 0 1.414l2.376 2.377a1 1 0 0 0 1.414 0l2.377-2.377a1 1 0 0 0 0-1.414l-2.377-2.377a1 1 0 0 0-1.414 0z"/><path d="M2.297 11.293a1 1 0 0 0 0 1.414l2.377 2.377a1 1 0 0 0 1.414 0l2.377-2.377a1 1 0 0 0 0-1.414L6.088 8.916a1 1 0 0 0-1.414 0z"/><path d="M8.916 17.912a1 1 0 0 0 0 1.415l2.377 2.376a1 1 0 0 0 1.414 0l2.377-2.376a1 1 0 0 0 0-1.415l-2.377-2.376a1 1 0 0 0-1.414 0z"/><path d="M8.916 4.674a1 1 0 0 0 0 1.414l2.377 2.376a1 1 0 0 0 1.414 0l2.377-2.376a1 1 0 0 0 0-1.414l-2.377-2.377a1 1 0 0 0-1.414 0z"/>"#);

/// `computer`.
pub const COMPUTER: Icon = lucide!("computer", r#"<path d="M12 18h6"/><path d="M6 18h.01"/><path d="M8 6h1"/><rect x="2" y="14" width="20" height="8" rx="2"/><rect x="4" y="2" width="16" height="12" rx="2"/>"#);

/// `concierge-bell`.
pub const CONCIERGE_BELL: Icon = lucide!("concierge-bell", r#"<path d="M3 20a1 1 0 0 1-1-1v-1a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1Z"/><path d="M20 16a8 8 0 1 0-16 0"/><path d="M12 4v4"/><path d="M10 4h4"/>"#);

/// `cone`.
pub const CONE: Icon = lucide!("cone", r#"<path d="m20.9 18.55-8-15.98a1 1 0 0 0-1.8 0l-8 15.98"/><ellipse cx="12" cy="19" rx="9" ry="3"/>"#);

/// `construction`.
pub const CONSTRUCTION: Icon = lucide!("construction", r#"<rect x="2" y="6" width="20" height="8" rx="1"/><path d="M17 14v7"/><path d="M7 14v7"/><path d="M17 3v3"/><path d="M7 3v3"/><path d="M10 14 2.3 6.3"/><path d="m14 6 7.7 7.7"/><path d="m8 6 8 8"/>"#);

/// `contact`.
pub const CONTACT: Icon = lucide!("contact", r#"<path d="M16 2v2"/><path d="M7 21v-2a2 2 0 012-2h6a2 2 0 012 2v2"/><path d="M8 2v2"/><circle cx="12" cy="10" r="3"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `contact-round`.
pub const CONTACT_ROUND: Icon = lucide!("contact-round", r#"<path d="M16 2v2"/><path d="M17.915 21a6 6 0 10-12 0"/><path d="M8 2v2"/><circle cx="12" cy="11" r="4"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `container`.
pub const CONTAINER: Icon = lucide!("container", r#"<path d="M22 7.7c0-.6-.4-1.2-.8-1.5l-6.3-3.9a1.72 1.72 0 0 0-1.7 0l-10.3 6c-.5.2-.9.8-.9 1.4v6.6c0 .5.4 1.2.8 1.5l6.3 3.9a1.72 1.72 0 0 0 1.7 0l10.3-6c.5-.3.9-1 .9-1.5Z"/><path d="M10 21.9V14L2.1 9.1"/><path d="m10 14 11.9-6.9"/><path d="M14 19.8v-8.1"/><path d="M18 17.5V9.4"/>"#);

/// `contrast`.
pub const CONTRAST: Icon = lucide!("contrast", r#"<circle cx="12" cy="12" r="10"/><path d="M12 18a6 6 0 0 0 0-12v12z"/>"#);

/// `cookie`.
pub const COOKIE: Icon = lucide!("cookie", r#"<path d="M11 17h.01"/><path d="M11.496 2c.324-.016.558.292.529.615a4 4 0 004.235 4.368.713.713 0 01.758.757 4 4 0 004.366 4.237c.323-.03.63.204.614.527a10 10 0 01-2.915 6.566A1 1 0 114.93 4.918 10 10 0 0111.496 2"/><path d="M12 12h.01"/><path d="M16 16h.01"/><path d="M16 3h.01"/><path d="M21 4h.01"/><path d="M21 8h.01"/><path d="M7 14h.01"/><path d="M9 8h.01"/>"#);

/// `cooking-pot`.
pub const COOKING_POT: Icon = lucide!("cooking-pot", r#"<path d="M2 12h20"/><path d="M20 12v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8"/><path d="m4 8 16-4"/><path d="m8.86 6.78-.45-1.81a2 2 0 0 1 1.45-2.43l1.94-.48a2 2 0 0 1 2.43 1.46l.45 1.8"/>"#);

/// `copy`.
pub const COPY: Icon = lucide!("copy", r#"<rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#);

/// `copy-check`.
pub const COPY_CHECK: Icon = lucide!("copy-check", r#"<path d="m12 15 2 2 4-4"/><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#);

/// `copy-minus`.
pub const COPY_MINUS: Icon = lucide!("copy-minus", r#"<line x1="12" x2="18" y1="15" y2="15"/><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#);

/// `copy-plus`.
pub const COPY_PLUS: Icon = lucide!("copy-plus", r#"<line x1="15" x2="15" y1="12" y2="18"/><line x1="12" x2="18" y1="15" y2="15"/><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#);

/// `copy-slash`.
pub const COPY_SLASH: Icon = lucide!("copy-slash", r#"<line x1="12" x2="18" y1="18" y2="12"/><rect width="14" height="14" x="8" y="8" rx="2" ry="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>"#);

/// `copy-x`.
pub const COPY_X: Icon = lucide!("copy-x", r#"<path d="M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2"/><rect x="8" y="8" width="14" height="14" rx="2"/><path d="m12.5 12.5 5 5"/><path d="m12.5 17.5 5-5"/>"#);

/// `copyleft`.
pub const COPYLEFT: Icon = lucide!("copyleft", r#"<circle cx="12" cy="12" r="10"/><path d="M9.17 14.83a4 4 0 1 0 0-5.66"/>"#);

/// `corner-down-left`.
pub const CORNER_DOWN_LEFT: Icon = lucide!("corner-down-left", r#"<path d="M20 4v7a4 4 0 0 1-4 4H4"/><path d="m9 10-5 5 5 5"/>"#);

/// `corner-down-right`.
pub const CORNER_DOWN_RIGHT: Icon = lucide!("corner-down-right", r#"<path d="m15 10 5 5-5 5"/><path d="M4 4v7a4 4 0 0 0 4 4h12"/>"#);

/// `corner-left-down`.
pub const CORNER_LEFT_DOWN: Icon = lucide!("corner-left-down", r#"<path d="m14 15-5 5-5-5"/><path d="M20 4h-7a4 4 0 0 0-4 4v12"/>"#);

/// `corner-left-up`.
pub const CORNER_LEFT_UP: Icon = lucide!("corner-left-up", r#"<path d="M14 9 9 4 4 9"/><path d="M20 20h-7a4 4 0 0 1-4-4V4"/>"#);

/// `corner-right-down`.
pub const CORNER_RIGHT_DOWN: Icon = lucide!("corner-right-down", r#"<path d="m10 15 5 5 5-5"/><path d="M4 4h7a4 4 0 0 1 4 4v12"/>"#);

/// `corner-right-up`.
pub const CORNER_RIGHT_UP: Icon = lucide!("corner-right-up", r#"<path d="m10 9 5-5 5 5"/><path d="M4 20h7a4 4 0 0 0 4-4V4"/>"#);

/// `corner-up-left`.
pub const CORNER_UP_LEFT: Icon = lucide!("corner-up-left", r#"<path d="M20 20v-7a4 4 0 0 0-4-4H4"/><path d="M9 14 4 9l5-5"/>"#);

/// `corner-up-right`.
pub const CORNER_UP_RIGHT: Icon = lucide!("corner-up-right", r#"<path d="m15 14 5-5-5-5"/><path d="M4 20v-7a4 4 0 0 1 4-4h12"/>"#);

/// `cpu`.
pub const CPU: Icon = lucide!("cpu", r#"<path d="M12 20v2"/><path d="M12 2v2"/><path d="M17 20v2"/><path d="M17 2v2"/><path d="M2 12h2"/><path d="M2 17h2"/><path d="M2 7h2"/><path d="M20 12h2"/><path d="M20 17h2"/><path d="M20 7h2"/><path d="M7 20v2"/><path d="M7 2v2"/><rect x="4" y="4" width="16" height="16" rx="2"/><rect x="8" y="8" width="8" height="8" rx="1"/>"#);

/// `cpu-ai`.
pub const CPU_AI: Icon = lucide!("cpu-ai", r#"<path d="M12 20v2"/><path d="M12 2v2"/><path d="M16 9v6"/><path d="M17 20v2"/><path d="M17 2v2"/><path d="M2 12h2"/><path d="M2 17h2"/><path d="M2 7h2"/><path d="M20 12h2"/><path d="M20 17h2"/><path d="M20 7h2"/><path d="M7 20v2"/><path d="M7 2v2"/><path d="M8 13h4"/><path d="M8 15v-4a2 2 0 014 0v4"/><rect x="4" y="4" width="16" height="16" rx="2"/>"#);

/// `creative-commons`.
pub const CREATIVE_COMMONS: Icon = lucide!("creative-commons", r#"<circle cx="12" cy="12" r="10"/><path d="M10 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"/><path d="M17 9.3a2.8 2.8 0 0 0-3.5 1 3.1 3.1 0 0 0 0 3.4 2.7 2.7 0 0 0 3.5 1"/>"#);

/// `credit-card`.
pub const CREDIT_CARD: Icon = lucide!("credit-card", r#"<rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/><path d="M6 14h2"/>"#);

/// `credit-card-check`.
pub const CREDIT_CARD_CHECK: Icon = lucide!("credit-card-check", r#"<path d="M12.5 19H4a2 2 0 01-2-2V7a2 2 0 012-2h16a2 2 0 012 2v4"/><path d="M2 10h20"/><path d="M6 14h2"/><path d="m16 17 2 2 4-4"/>"#);

/// `credit-card-minus`.
pub const CREDIT_CARD_MINUS: Icon = lucide!("credit-card-minus", r#"<path d="M22 13V7a2 2 0 00-2-2H4a2 2 0 00-2 2v10a2 2 0 002 2h8.536"/><path d="M22 10H2"/><path d="M6 14h2"/><path d="M16 17h6"/>"#);

/// `credit-card-plus`.
pub const CREDIT_CARD_PLUS: Icon = lucide!("credit-card-plus", r#"<path d="M22 11.354V7a2 2 0 00-2-2H4a2 2 0 00-2 2v10a2 2 0 002 2h8.536"/><path d="M22 10H2"/><path d="M6 14h2"/><path d="M16 17h6"/><path d="M19 14v6"/>"#);

/// `credit-card-reader`.
pub const CREDIT_CARD_READER: Icon = lucide!("credit-card-reader", r#"<path d="M15 16v1"/><path d="M16.963 7.734A1 1 0 0015.999 7H8.003a1 1 0 00-.964.734L4.073 18.467A2 2 0 006 21h12a2 2 0 001.927-2.532z"/><path d="M2.678 8.5A2 2 0 012 7V5a2 2 0 012-2h16a2 2 0 012 2v2a2 2 0 01-.676 1.499"/><path d="m9 21 2-14"/>"#);

/// `credit-card-x`.
pub const CREDIT_CARD_X: Icon = lucide!("credit-card-x", r#"<path d="M12.5 19H4a2 2 0 01-2-2V7a2 2 0 012-2h16a2 2 0 012 2v3.5"/><path d="M2 10h20"/><path d="M6 14h2"/><path d="m16.5 14.5 5 5"/><path d="m21.5 14.5-5 5"/>"#);

/// `croissant`.
pub const CROISSANT: Icon = lucide!("croissant", r#"<path d="M10.2 18H4.774a1.5 1.5 0 0 1-1.352-.97 11 11 0 0 1 .132-6.487"/><path d="M18 10.2V4.774a1.5 1.5 0 0 0-.97-1.352 11 11 0 0 0-6.486.132"/><path d="M18 5a4 3 0 0 1 4 3 2 2 0 0 1-2 2 10 10 0 0 0-5.139 1.42"/><path d="M5 18a3 4 0 0 0 3 4 2 2 0 0 0 2-2 10 10 0 0 1 1.42-5.14"/><path d="M8.709 2.554a10 10 0 0 0-6.155 6.155 1.5 1.5 0 0 0 .676 1.626l9.807 5.42a2 2 0 0 0 2.718-2.718l-5.42-9.807a1.5 1.5 0 0 0-1.626-.676"/>"#);

/// `crop`.
pub const CROP: Icon = lucide!("crop", r#"<path d="M6 2v14a2 2 0 0 0 2 2h14"/><path d="M18 22V8a2 2 0 0 0-2-2H2"/>"#);

/// `cross`.
pub const CROSS: Icon = lucide!("cross", r#"<path d="M4 9a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h4a1 1 0 0 1 1 1v4a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-4a1 1 0 0 1 1-1h4a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2h-4a1 1 0 0 1-1-1V4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4a1 1 0 0 1-1 1z"/>"#);

/// `crosshair`.
pub const CROSSHAIR: Icon = lucide!("crosshair", r#"<circle cx="12" cy="12" r="10"/><line x1="22" x2="18" y1="12" y2="12"/><line x1="6" x2="2" y1="12" y2="12"/><line x1="12" x2="12" y1="6" y2="2"/><line x1="12" x2="12" y1="22" y2="18"/>"#);

/// `crown`.
pub const CROWN: Icon = lucide!("crown", r#"<path d="M11.562 3.266a.5.5 0 0 1 .876 0L15.39 8.87a1 1 0 0 0 1.516.294L21.183 5.5a.5.5 0 0 1 .798.519l-2.834 10.246a1 1 0 0 1-.956.734H5.81a1 1 0 0 1-.957-.734L2.02 6.02a.5.5 0 0 1 .798-.519l4.276 3.664a1 1 0 0 0 1.516-.294z"/><path d="M5 21h14"/>"#);

/// `cuboid`.
pub const CUBOID: Icon = lucide!("cuboid", r#"<path d="M10 22v-8"/><path d="M2.336 8.89 10 14l11.715-7.029"/><path d="M22 14a2 2 0 0 1-.971 1.715l-10 6a2 2 0 0 1-2.138-.05l-6-4A2 2 0 0 1 2 16v-6a2 2 0 0 1 .971-1.715l10-6a2 2 0 0 1 2.138.05l6 4A2 2 0 0 1 22 8z"/>"#);

/// `cup-soda`.
pub const CUP_SODA: Icon = lucide!("cup-soda", r#"<path d="m6 8 1.75 12.28a2 2 0 0 0 2 1.72h4.54a2 2 0 0 0 2-1.72L18 8"/><path d="M5 8h14"/><path d="M7 15a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0"/><path d="m12 8 1-6h2"/>"#);

/// `cupcake`.
pub const CUPCAKE: Icon = lucide!("cupcake", r#"<path d="M12 22v-9"/><path d="M14 4h1a3 3 0 013 3l-.004.125A4 4 0 0121 11v2"/><path d="m15.5 22 1.5-9"/><path d="M21 13a1 1 0 01.919 1.394l-2.74 6.394A2 2 0 0117.34 22H6.659a2 2 0 01-1.838-1.212l-2.74-6.394A1 1 0 013 13z"/><path d="M3 13v-2a4 4 0 013.003-3.875L6 7a3 3 0 013-3h1"/><path d="M8.5 22 7 13"/><circle cx="12" cy="4" r="2"/>"#);

/// `currency`.
pub const CURRENCY: Icon = lucide!("currency", r#"<circle cx="12" cy="12" r="8"/><line x1="3" x2="6" y1="3" y2="6"/><line x1="21" x2="18" y1="3" y2="6"/><line x1="3" x2="6" y1="21" y2="18"/><line x1="21" x2="18" y1="21" y2="18"/>"#);

/// `cylinder`.
pub const CYLINDER: Icon = lucide!("cylinder", r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/>"#);

/// `dam`.
pub const DAM: Icon = lucide!("dam", r#"<path d="M11 11.31c1.17.56 1.54 1.69 3.5 1.69 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="M11.75 18c.35.5 1.45 1 2.75 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><path d="M2 6h4"/><path d="M7 3a1 1 0 0 0-1 1v16a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1L10 4a1 1 0 0 0-1-1z"/>"#);

/// `database`.
pub const DATABASE: Icon = lucide!("database", r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 21 19V5"/><path d="M3 12A9 3 0 0 0 21 12"/>"#);

/// `database-arrow-down`.
pub const DATABASE_ARROW_DOWN: Icon = lucide!("database-arrow-down", r#"<path d="m16 19 3 3 3-3"/><path d="M19 16v6"/><path d="M21 12.536V5"/><path d="M3 12A9 3 0 0 0 15.182 14.806"/><path d="M3 5V19A9 3 0 0 0 13.318 21.968"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-arrow-up`.
pub const DATABASE_ARROW_UP: Icon = lucide!("database-arrow-up", r#"<path d="M19 22v-6"/><path d="M21 12.536V5"/><path d="m22 19-3-3-3 3"/><path d="M3 12A9 3 0 0 0 14.457 14.886"/><path d="M3 5V19A9 3 0 0 0 13.318 21.968"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-backup`.
pub const DATABASE_BACKUP: Icon = lucide!("database-backup", r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 12a9 3 0 0 0 5 2.69"/><path d="M21 9.3V5"/><path d="M3 5v14a9 3 0 0 0 6.47 2.88"/><path d="M12 12v4h4"/><path d="M13 20a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L12 16"/>"#);

/// `database-check`.
pub const DATABASE_CHECK: Icon = lucide!("database-check", r#"<path d="m16 19 2 2 4-4"/><path d="M21 13.127V5"/><path d="M3 12A9 3 0 0 0 21 12"/><path d="M3 5V19A9 3 0 0 0 13.318 21.968"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-cog`.
pub const DATABASE_COG: Icon = lucide!("database-cog", r#"<path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><path d="M21 10.5V5"/><path d="M3 12a9 3 0 0 0 7.599 2.963"/><path d="M3 5v14a9 3 0 0 0 8.148 2.987"/><circle cx="18" cy="18" r="3"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-minus`.
pub const DATABASE_MINUS: Icon = lucide!("database-minus", r#"<path d="M21 15V5"/><path d="M22 19h-6"/><path d="M3 12A9 3 0 0 0 21 12"/><path d="M3 5V19A9 3 0 0 0 13.318 21.968"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-plus`.
pub const DATABASE_PLUS: Icon = lucide!("database-plus", r#"<path d="M19 16v6"/><path d="M21 12.536V5"/><path d="M22 19h-6"/><path d="M3 12A9 3 0 0 0 15.1824 14.8061"/><path d="M3 5V19A9 3 0 0 0 13.318 21.968"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-search`.
pub const DATABASE_SEARCH: Icon = lucide!("database-search", r#"<path d="M21 11.693V5"/><path d="m22 22-1.875-1.875"/><path d="M3 12a9 3 0 0 0 8.697 2.998"/><path d="M3 5v14a9 3 0 0 0 9.28 2.999"/><circle cx="18" cy="18" r="3"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-x`.
pub const DATABASE_X: Icon = lucide!("database-x", r#"<path d="m17 17 5 5"/><path d="M19.323 13.744A9 3 0 0 0 21 12"/><path d="M21 13.127V5"/><path d="m22 17-5 5"/><path d="M3 12A9 3 0 0 0 13.563 14.954"/><path d="M3 5V19A9 3 0 0 0 13 21.981"/><ellipse cx="12" cy="5" rx="9" ry="3"/>"#);

/// `database-zap`.
pub const DATABASE_ZAP: Icon = lucide!("database-zap", r#"<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5V19A9 3 0 0 0 15 21.84"/><path d="M21 5V8"/><path d="M21 12L18 17H22L19 22"/><path d="M3 12A9 3 0 0 0 14.59 14.87"/>"#);

/// `decimals-arrow-left`.
pub const DECIMALS_ARROW_LEFT: Icon = lucide!("decimals-arrow-left", r#"<path d="m13 21-3-3 3-3"/><path d="M20 18H10"/><path d="M3 11h.01"/><rect x="6" y="3" width="5" height="8" rx="2.5"/>"#);

/// `decimals-arrow-right`.
pub const DECIMALS_ARROW_RIGHT: Icon = lucide!("decimals-arrow-right", r#"<path d="M10 18h10"/><path d="m17 21 3-3-3-3"/><path d="M3 11h.01"/><rect x="15" y="3" width="5" height="8" rx="2.5"/><rect x="6" y="3" width="5" height="8" rx="2.5"/>"#);

/// `delete`.
pub const DELETE: Icon = lucide!("delete", r#"<path d="M10 5a2 2 0 0 0-1.344.519l-6.328 5.74a1 1 0 0 0 0 1.481l6.328 5.741A2 2 0 0 0 10 19h10a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2z"/><path d="m12 9 6 6"/><path d="m18 9-6 6"/>"#);

/// `dessert`.
pub const DESSERT: Icon = lucide!("dessert", r#"<path d="M10.162 3.167A10 10 0 0 0 2 13a2 2 0 0 0 4 0v-1a2 2 0 0 1 4 0v4a2 2 0 0 0 4 0v-4a2 2 0 0 1 4 0v1a2 2 0 0 0 4-.006 10 10 0 0 0-8.161-9.826"/><path d="M20.804 14.869a9 9 0 0 1-17.608 0"/><circle cx="12" cy="4" r="2"/>"#);

/// `diagram-flow-horizontal`.
pub const DIAGRAM_FLOW_HORIZONTAL: Icon = lucide!("diagram-flow-horizontal", r#"<path d="M8 12h.5a3.5 3.5 0 013.5 3.5 3.5 3.5 0 003.5 3.5h.5"/><path d="M8.5 12A3.5 3.5 0 0012 8.5 3.5 3.5 0 0115.5 5h.5"/><rect x="16" y="16" width="6" height="6" rx="1"/><rect x="16" y="2" width="6" height="6" rx="1"/><rect x="2" y="9" width="6" height="6" rx="1"/>"#);

/// `diagram-flow-vertical`.
pub const DIAGRAM_FLOW_VERTICAL: Icon = lucide!("diagram-flow-vertical", r#"<path d="M12 8.5a3.5 3.5 0 003.5 3.5 3.5 3.5 0 013.5 3.5v.5"/><path d="M12 8v.5A3.5 3.5 0 018.5 12 3.5 3.5 0 005 15.5v.5"/><rect x="16" y="16" width="6" height="6" rx="1"/><rect x="2" y="16" width="6" height="6" rx="1"/><rect x="9" y="2" width="6" height="6" rx="1"/>"#);

/// `diameter`.
pub const DIAMETER: Icon = lucide!("diameter", r#"<circle cx="19" cy="19" r="2"/><circle cx="5" cy="5" r="2"/><path d="M6.48 3.66a10 10 0 0 1 13.86 13.86"/><path d="m6.41 6.41 11.18 11.18"/><path d="M3.66 6.48a10 10 0 0 0 13.86 13.86"/>"#);

/// `diamond`.
pub const DIAMOND: Icon = lucide!("diamond", r#"<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41l-7.59-7.59a2.41 2.41 0 0 0-3.41 0Z"/>"#);

/// `diamond-minus`.
pub const DIAMOND_MINUS: Icon = lucide!("diamond-minus", r#"<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z"/><path d="M8 12h8"/>"#);

/// `diamond-percent`.
pub const DIAMOND_PERCENT: Icon = lucide!("diamond-percent", r#"<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z"/><path d="M9.2 9.2h.01"/><path d="m14.5 9.5-5 5"/><path d="M14.7 14.8h.01"/>"#);

/// `diamond-plus`.
pub const DIAMOND_PLUS: Icon = lucide!("diamond-plus", r#"<path d="M12 8v8"/><path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0z"/><path d="M8 12h8"/>"#);

/// `dice-1`.
pub const DICE_1: Icon = lucide!("dice-1", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M12 12h.01"/>"#);

/// `dice-2`.
pub const DICE_2: Icon = lucide!("dice-2", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M15 9h.01"/><path d="M9 15h.01"/>"#);

/// `dice-3`.
pub const DICE_3: Icon = lucide!("dice-3", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M16 8h.01"/><path d="M12 12h.01"/><path d="M8 16h.01"/>"#);

/// `dice-4`.
pub const DICE_4: Icon = lucide!("dice-4", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M16 8h.01"/><path d="M8 8h.01"/><path d="M8 16h.01"/><path d="M16 16h.01"/>"#);

/// `dice-5`.
pub const DICE_5: Icon = lucide!("dice-5", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M16 8h.01"/><path d="M8 8h.01"/><path d="M8 16h.01"/><path d="M16 16h.01"/><path d="M12 12h.01"/>"#);

/// `dice-6`.
pub const DICE_6: Icon = lucide!("dice-6", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M16 8h.01"/><path d="M16 12h.01"/><path d="M16 16h.01"/><path d="M8 8h.01"/><path d="M8 12h.01"/><path d="M8 16h.01"/>"#);

/// `dices`.
pub const DICES: Icon = lucide!("dices", r#"<rect width="12" height="12" x="2" y="10" rx="2" ry="2"/><path d="m17.92 14 3.5-3.5a2.24 2.24 0 0 0 0-3l-5-4.92a2.24 2.24 0 0 0-3 0L10 6"/><path d="M6 18h.01"/><path d="M10 14h.01"/><path d="M15 6h.01"/><path d="M18 9h.01"/>"#);

/// `diff`.
pub const DIFF: Icon = lucide!("diff", r#"<path d="M12 3v14"/><path d="M5 10h14"/><path d="M5 21h14"/>"#);

/// `disc`.
pub const DISC: Icon = lucide!("disc", r#"<circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="2"/>"#);

/// `disc-2`.
pub const DISC_2: Icon = lucide!("disc-2", r#"<circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="4"/><path d="M12 12h.01"/>"#);

/// `disc-3`.
pub const DISC_3: Icon = lucide!("disc-3", r#"<circle cx="12" cy="12" r="10"/><path d="M6 12c0-1.7.7-3.2 1.8-4.2"/><circle cx="12" cy="12" r="2"/><path d="M18 12c0 1.7-.7 3.2-1.8 4.2"/>"#);

/// `disc-album`.
pub const DISC_ALBUM: Icon = lucide!("disc-album", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="12" cy="12" r="5"/><path d="M12 12h.01"/>"#);

/// `divide`.
pub const DIVIDE: Icon = lucide!("divide", r#"<circle cx="12" cy="6" r="1"/><line x1="5" x2="19" y1="12" y2="12"/><circle cx="12" cy="18" r="1"/>"#);

/// `dna`.
pub const DNA: Icon = lucide!("dna", r#"<path d="m10 16 1.5 1.5"/><path d="m14 8-1.5-1.5"/><path d="M15 2c-1.798 1.998-2.518 3.995-2.807 5.993"/><path d="m16.5 10.5 1 1"/><path d="m17 6-2.891-2.891"/><path d="M2 15c6.667-6 13.333 0 20-6"/><path d="m20 9 .891.891"/><path d="M3.109 14.109 4 15"/><path d="m6.5 12.5 1 1"/><path d="m7 18 2.891 2.891"/><path d="M9 22c1.798-1.998 2.518-3.995 2.807-5.993"/>"#);

/// `dna-off`.
pub const DNA_OFF: Icon = lucide!("dna-off", r#"<path d="M15 2c-1.35 1.5-2.092 3-2.5 4.5L14 8"/><path d="m17 6-2.891-2.891"/><path d="M2 15c3.333-3 6.667-3 10-3"/><path d="m2 2 20 20"/><path d="m20 9 .891.891"/><path d="M22 9c-1.5 1.35-3 2.092-4.5 2.5l-1-1"/><path d="M3.109 14.109 4 15"/><path d="m6.5 12.5 1 1"/><path d="m7 18 2.891 2.891"/><path d="M9 22c1.35-1.5 2.092-3 2.5-4.5L10 16"/>"#);

/// `dock`.
pub const DOCK: Icon = lucide!("dock", r#"<path d="M2 8h20"/><rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 16h12"/>"#);

/// `dog`.
pub const DOG: Icon = lucide!("dog", r#"<path d="M11.25 16.25h1.5L12 17z"/><path d="M16 14v.5"/><path d="M4.42 11.247A13.152 13.152 0 0 0 4 14.556C4 18.728 7.582 21 12 21s8-2.272 8-6.444a11.702 11.702 0 0 0-.493-3.309"/><path d="M8 14v.5"/><path d="M8.5 8.5c-.384 1.05-1.083 2.028-2.344 2.5-1.931.722-3.576-.297-3.656-1-.113-.994 1.177-6.53 4-7 1.923-.321 3.651.845 3.651 2.235A7.497 7.497 0 0 1 14 5.277c0-1.39 1.844-2.598 3.767-2.277 2.823.47 4.113 6.006 4 7-.08.703-1.725 1.722-3.656 1-1.261-.472-1.855-1.45-2.239-2.5"/>"#);

/// `dollar-sign`.
pub const DOLLAR_SIGN: Icon = lucide!("dollar-sign", r#"<path d="M12 3v18"/><path d="M17 5H9.5a1 1 0 000 7h5a1 1 0 010 7H6"/>"#);

/// `dome`.
pub const DOME: Icon = lucide!("dome", r#"<path d="M10 21v-3a2 2 0 014 0v3"/><path d="M12 2v2"/><path d="M18 12v9"/><path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2v-6a1 1 0 011-1h18a1 1 0 011 1z"/><path d="M4 12a8 8 0 0116 0"/><path d="M6 12v9"/>"#);

/// `donut`.
pub const DONUT: Icon = lucide!("donut", r#"<path d="M20.5 10a2.5 2.5 0 0 1-2.4-3H18a2.95 2.95 0 0 1-2.6-4.4 10 10 0 1 0 6.3 7.1c-.3.2-.8.3-1.2.3"/><circle cx="12" cy="12" r="3"/>"#);

/// `door-closed`.
pub const DOOR_CLOSED: Icon = lucide!("door-closed", r#"<path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16"/><path d="M2 21h20"/><path d="M9 12h.01"/>"#);

/// `door-closed-cog`.
pub const DOOR_CLOSED_COG: Icon = lucide!("door-closed-cog", r#"<path d="m14.305 19.53.923-.382"/><path d="m15.229 16.852-.924-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.773-.383.924"/><path d="M19 10.35V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.697-.382-.924"/><path d="M2 21h8.58"/><path d="m20.773 16.852.922-.383"/><path d="m20.773 19.148.922.383"/><path d="M9 12h.01"/><circle cx="18" cy="18" r="3"/>"#);

/// `door-closed-locked`.
pub const DOOR_CLOSED_LOCKED: Icon = lucide!("door-closed-locked", r#"<path d="M19 8V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16"/><path d="M2 21h8"/><path d="M20 16v-2a2 2 0 00-4 0v2"/><path d="M9 12h.01"/><rect x="14" y="16" width="8" height="5" rx="1"/>"#);

/// `door-closed-package`.
pub const DOOR_CLOSED_PACKAGE: Icon = lucide!("door-closed-package", r#"<path d="M18 13v3"/><path d="M19 9V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16"/><path d="M2 21h8"/><path d="M9 12h.01"/><rect x="14" y="13" width="8" height="8" rx="1"/>"#);

/// `door-open`.
pub const DOOR_OPEN: Icon = lucide!("door-open", r#"<path d="M10 21H2"/><path d="M10 3H7a2 2 0 00-2 2v16"/><path d="M14 12h.01"/><path d="M19 21V5a2 2 0 00-1.675-1.974l-6.163-1.013A1 1 0 0010 3v18a1 1 0 001.124.992z"/><path d="M22 21h-3"/>"#);

/// `door-stairwell`.
pub const DOOR_STAIRWELL: Icon = lucide!("door-stairwell", r#"<path d="M12 17v-3a1 1 0 011-1h6"/><path d="M19 17h-9a1 1 0 00-1 1v3"/><path d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16"/><path d="M19 9h-3a1 1 0 00-1 1v3"/><path d="M22 21H2"/>"#);

/// `dot`.
pub const DOT: Icon = lucide!("dot", r#"<circle cx="12" cy="12" r="1"/>"#);

/// `download`.
pub const DOWNLOAD: Icon = lucide!("download", r#"<path d="M12 15V3"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/>"#);

/// `drafting-compass`.
pub const DRAFTING_COMPASS: Icon = lucide!("drafting-compass", r#"<path d="m12.99 6.74 1.93 3.44"/><path d="M19.136 12a10 10 0 0 1-14.271 0"/><path d="m21 21-2.16-3.84"/><path d="m3 21 8.02-14.26"/><circle cx="12" cy="5" r="2"/>"#);

/// `drama`.
pub const DRAMA: Icon = lucide!("drama", r#"<path d="M10 11h.01"/><path d="M14 6h.01"/><path d="M18 6h.01"/><path d="M6.5 13.1h.01"/><path d="M22 5c0 9-4 12-6 12s-6-3-6-12c0-2 2-3 6-3s6 1 6 3"/><path d="M17.4 9.9c-.8.8-2 .8-2.8 0"/><path d="M10.1 7.1C9 7.2 7.7 7.7 6 8.6c-3.5 2-4.7 3.9-3.7 5.6 4.5 7.8 9.5 8.4 11.2 7.4.9-.5 1.9-2.1 1.9-4.7"/><path d="M9.1 16.5c.3-1.1 1.4-1.7 2.4-1.4"/>"#);

/// `drill`.
pub const DRILL: Icon = lucide!("drill", r#"<path d="M10 18a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1H5a3 3 0 0 1-3-3 1 1 0 0 1 1-1z"/><path d="M13 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a1 1 0 0 1 1 1v6a1 1 0 0 1-1 1l-.81 3.242a1 1 0 0 1-.97.758H8"/><path d="M14 4h3a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-3"/><path d="M18 6h4"/><path d="m5 10-2 8"/><path d="m7 18 2-8"/>"#);

/// `drone`.
pub const DRONE: Icon = lucide!("drone", r#"<path d="M10 10 7 7"/><path d="m10 14-3 3"/><path d="m14 10 3-3"/><path d="m14 14 3 3"/><path d="M14.205 4.139a4 4 0 1 1 5.439 5.863"/><path d="M19.637 14a4 4 0 1 1-5.432 5.868"/><path d="M4.367 10a4 4 0 1 1 5.438-5.862"/><path d="M9.795 19.862a4 4 0 1 1-5.429-5.873"/><rect x="10" y="8" width="4" height="8" rx="1"/>"#);

/// `droplet`.
pub const DROPLET: Icon = lucide!("droplet", r#"<path d="M12 22a7 7 0 0 0 7-7c0-2-1-3.9-3-5.5s-3.5-4-4-6.5c-.5 2.5-2 4.9-4 6.5C6 11.1 5 13 5 15a7 7 0 0 0 7 7z"/>"#);

/// `droplet-off`.
pub const DROPLET_OFF: Icon = lucide!("droplet-off", r#"<path d="M18.715 13.186C18.29 11.858 17.384 10.607 16 9.5c-2-1.6-3.5-4-4-6.5a10.7 10.7 0 0 1-.884 2.586"/><path d="m2 2 20 20"/><path d="M8.795 8.797A11 11 0 0 1 8 9.5C6 11.1 5 13 5 15a7 7 0 0 0 13.222 3.208"/>"#);

/// `droplets`.
pub const DROPLETS: Icon = lucide!("droplets", r#"<path d="M7 16.3c2.2 0 4-1.83 4-4.05 0-1.16-.57-2.26-1.71-3.19S7.29 6.75 7 5.3c-.29 1.45-1.14 2.84-2.29 3.76S3 11.1 3 12.25c0 2.22 1.8 4.05 4 4.05z"/><path d="M12.56 6.6A10.97 10.97 0 0 0 14 3.02c.5 2.5 2 4.9 4 6.5s3 3.5 3 5.5a6.98 6.98 0 0 1-11.91 4.97"/>"#);

/// `drum`.
pub const DRUM: Icon = lucide!("drum", r#"<path d="m2 2 8 8"/><path d="m22 2-8 8"/><ellipse cx="12" cy="9" rx="10" ry="5"/><path d="M7 13.4v7.9"/><path d="M12 14v8"/><path d="M17 13.4v7.9"/><path d="M2 9v8a10 5 0 0 0 20 0V9"/>"#);

/// `drumstick`.
pub const DRUMSTICK: Icon = lucide!("drumstick", r#"<path d="M15.4 15.63a7.875 6 135 1 1 6.23-6.23 4.5 3.43 135 0 0-6.23 6.23"/><path d="m8.29 12.71-2.6 2.6a2.5 2.5 0 1 0-1.65 4.65A2.5 2.5 0 1 0 8.7 18.3l2.59-2.59"/>"#);

/// `dumbbell`.
pub const DUMBBELL: Icon = lucide!("dumbbell", r#"<path d="M17.596 12.768a2 2 0 1 0 2.829-2.829l-1.768-1.767a2 2 0 0 0 2.828-2.829l-2.828-2.828a2 2 0 0 0-2.829 2.828l-1.767-1.768a2 2 0 1 0-2.829 2.829z"/><path d="m2.5 21.5 1.4-1.4"/><path d="m20.1 3.9 1.4-1.4"/><path d="M5.343 21.485a2 2 0 1 0 2.829-2.828l1.767 1.768a2 2 0 1 0 2.829-2.829l-6.364-6.364a2 2 0 1 0-2.829 2.829l1.768 1.767a2 2 0 0 0-2.828 2.829z"/><path d="m9.6 14.4 4.8-4.8"/>"#);

/// `ear`.
pub const EAR: Icon = lucide!("ear", r#"<path d="M6 8.5a6.5 6.5 0 1 1 13 0c0 6-6 6-6 10a3.5 3.5 0 1 1-7 0"/><path d="M15 8.5a2.5 2.5 0 0 0-5 0v1a2 2 0 1 1 0 4"/>"#);

/// `ear-off`.
pub const EAR_OFF: Icon = lucide!("ear-off", r#"<path d="M6 18.5a3.5 3.5 0 1 0 7 0c0-1.57.92-2.52 2.04-3.46"/><path d="M6 8.5c0-.75.13-1.47.36-2.14"/><path d="M8.8 3.15A6.5 6.5 0 0 1 19 8.5c0 1.63-.44 2.81-1.09 3.76"/><path d="M12.5 6A2.5 2.5 0 0 1 15 8.5M10 13a2 2 0 0 0 1.82-1.18"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `earbuds`.
pub const EARBUDS: Icon = lucide!("earbuds", r#"<path d="M18 3a4 4 90 00-4 4v12a2 2 90 004 0v-8a5 5 90 003.75-1.693 1 1 90 00.25-.661V5.354a1 1 90 00-.25-.661A5 5 90 0018 3"/><path d="M6 3a4 4 0 014 4v12a2 2 0 01-4 0v-8a5 5 0 01-3.75-1.693A1 1 0 012 8.646V5.354a1 1 0 01.25-.661A5 5 0 016 3"/>"#);

/// `earbuds-off`.
pub const EARBUDS_OFF: Icon = lucide!("earbuds-off", r#"<path d="M10 10v9a2 2 0 01-4 0v-8a5 5 0 01-3.75-1.693A1 1 0 012 8.646V5.354a1 1 0 01.25-.661 5 5 0 011.359-1.084"/><path d="M14 14v5a2 2 90 004 0v-1"/><path d="M18 12.343V11a5 5 90 003.75-1.693 1 1 90 00.25-.661V5.354a1 1 90 00-.25-.661A5 5 90 0018 3a4 4 90 00-4 4v1.343"/><path d="m2 2 20 20"/>"#);

/// `earth`.
pub const EARTH: Icon = lucide!("earth", r#"<path d="M21.54 15H17a2 2 0 0 0-2 2v4.54"/><path d="M7 3.34V5a3 3 0 0 0 3 3a2 2 0 0 1 2 2c0 1.1.9 2 2 2a2 2 0 0 0 2-2c0-1.1.9-2 2-2h3.17"/><path d="M11 21.95V18a2 2 0 0 0-2-2a2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05"/><circle cx="12" cy="12" r="10"/>"#);

/// `earth-lock`.
pub const EARTH_LOCK: Icon = lucide!("earth-lock", r#"<path d="M7 3.34V5a3 3 0 0 0 3 3"/><path d="M11 21.95V18a2 2 0 0 0-2-2 2 2 0 0 1-2-2v-1a2 2 0 0 0-2-2H2.05"/><path d="M21.54 15H17a2 2 0 0 0-2 2v4.54"/><path d="M12 2a10 10 0 1 0 9.54 13"/><path d="M20 6V4a2 2 0 1 0-4 0v2"/><rect width="8" height="5" x="14" y="6" rx="1"/>"#);

/// `eclipse`.
pub const ECLIPSE: Icon = lucide!("eclipse", r#"<circle cx="12" cy="12" r="10"/><path d="M12 2a7 7 0 1 0 10 10"/>"#);

/// `egg`.
pub const EGG: Icon = lucide!("egg", r#"<path d="M12 2C8 2 4 8 4 14a8 8 0 0 0 16 0c0-6-4-12-8-12"/>"#);

/// `egg-fried`.
pub const EGG_FRIED: Icon = lucide!("egg-fried", r#"<circle cx="11.5" cy="12.5" r="3.5"/><path d="M3 8c0-3.5 2.5-6 6.5-6 5 0 4.83 3 7.5 5s5 2 5 6c0 4.5-2.5 6.5-7 6.5-2.5 0-2.5 2.5-6 2.5s-7-2-7-5.5c0-3 1.5-3 1.5-5C3.5 10 3 9 3 8Z"/>"#);

/// `egg-off`.
pub const EGG_OFF: Icon = lucide!("egg-off", r#"<path d="m2 2 20 20"/><path d="M20 14.347V14c0-6-4-12-8-12-1.078 0-2.157.436-3.157 1.19"/><path d="M6.206 6.21C4.871 8.4 4 11.2 4 14a8 8 0 0 0 14.568 4.568"/>"#);

/// `eighth-note`.
pub const EIGHTH_NOTE: Icon = lucide!("eighth-note", r#"<path d="m19 6-5.2-3.9a.5.5 0 0 0-.8.4V18"/><circle cx="9" cy="18" r="4"/>"#);

/// `eighth-notes`.
pub const EIGHTH_NOTES: Icon = lucide!("eighth-notes", r#"<path d="M21 16V3a1 1 0 0 0-1.28-.96l-10 2.917a1 1 0 0 0-.72.96V19"/><circle cx="18" cy="16" r="3"/><circle cx="6" cy="19" r="3"/>"#);

/// `eject`.
pub const EJECT: Icon = lucide!("eject", r#"<path d="M4 13a1 1 0 0 1-.72-1.695l7.257-7.668a2 2 0 0 1 2.926 0l7.256 7.668A1 1 0 0 1 20 13z"/><rect x="3" y="17" width="18" height="4" rx="1"/>"#);

/// `ellipse`.
pub const ELLIPSE: Icon = lucide!("ellipse", r#"<ellipse cx="12" cy="12" rx="10" ry="6"/>"#);

/// `ellipsis`.
pub const ELLIPSIS: Icon = lucide!("ellipsis", r#"<circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/>"#);

/// `ellipsis-vertical`.
pub const ELLIPSIS_VERTICAL: Icon = lucide!("ellipsis-vertical", r#"<circle cx="12" cy="12" r="1"/><circle cx="12" cy="5" r="1"/><circle cx="12" cy="19" r="1"/>"#);

/// `emirati-dirham`.
pub const EMIRATI_DIRHAM: Icon = lucide!("emirati-dirham", r#"<path d="M11 4a8 8 0 0 1 0 16H6.406A.4.4 0 0 1 6 19.594V4.406A.4.4 0 0 1 6.406 4z"/><path d="M4 10h17"/><path d="M4 14h17"/>"#);

/// `engine`.
pub const ENGINE: Icon = lucide!("engine", r#"<path d="M10 3h6"/><path d="M13 3v4"/><path d="M2 10v6"/><path d="M2 13h4"/><path d="M6 16a2 2 0 002 2h1a2 2 0 011.6.8l.3.4a2 2 0 001.6.8h2.264a2 2 0 001.789-1.106l1.67-3.341a1 1 0 01.895-.553H21a1 1 0 001-1v-4a1 1 0 00-1-1h-3.5a1 1 0 01-.8-.4l-.9-1.2A1 1 0 0015 7h-4a1 1 0 00-.8.4l-.9 1.2a1 1 0 01-.8.4H7a1 1 0 00-1 1z"/>"#);

/// `equal`.
pub const EQUAL: Icon = lucide!("equal", r#"<line x1="5" x2="19" y1="9" y2="9"/><line x1="5" x2="19" y1="15" y2="15"/>"#);

/// `equal-approximately`.
pub const EQUAL_APPROXIMATELY: Icon = lucide!("equal-approximately", r#"<path d="M5 15a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0"/><path d="M5 9a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0"/>"#);

/// `equal-approximately-not`.
pub const EQUAL_APPROXIMATELY_NOT: Icon = lucide!("equal-approximately-not", r#"<path d="M5 15a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0"/><path d="M5 9a6.5 6.5 0 0 1 7 0 6.5 6.5 0 0 0 7 0"/><line x1="19" x2="5" y1="5" y2="19"/>"#);

/// `equal-not`.
pub const EQUAL_NOT: Icon = lucide!("equal-not", r#"<line x1="5" x2="19" y1="9" y2="9"/><line x1="5" x2="19" y1="15" y2="15"/><line x1="19" x2="5" y1="5" y2="19"/>"#);

/// `eraser`.
pub const ERASER: Icon = lucide!("eraser", r#"<path d="M21 21H8a2 2 0 0 1-1.42-.587l-3.994-3.999a2 2 0 0 1 0-2.828l10-10a2 2 0 0 1 2.829 0l5.999 6a2 2 0 0 1 0 2.828L12.834 21"/><path d="m5.082 11.09 8.828 8.828"/>"#);

/// `ethernet-port`.
pub const ETHERNET_PORT: Icon = lucide!("ethernet-port", r#"<path d="M10 8v1"/><path d="M14 8v1"/><path d="M18 8v1"/><path d="M19 17a2 2 0 00-1.765 1.059l-.47.882A2 2 0 0115 20H9a2 2 0 01-1.765-1.059l-.47-.882A2 2 0 005 17H4a2 2 0 01-2-2V6a2 2 0 012-2h16a2 2 0 012 2v9a2 2 0 01-2 2z"/><path d="M6 8v1"/>"#);

/// `euro`.
pub const EURO: Icon = lucide!("euro", r#"<path d="M4 10h12"/><path d="M4 14h9"/><path d="M19 6a7.7 7.7 0 0 0-5.2-2A7.9 7.9 0 0 0 6 12c0 4.4 3.5 8 7.8 8 2 0 3.8-.8 5.2-2"/>"#);

/// `ev-charger`.
pub const EV_CHARGER: Icon = lucide!("ev-charger", r#"<path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 4 0v-6.998a2 2 0 0 0-.59-1.42L18 5"/><path d="M14 21V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16"/><path d="M2 21h13"/><path d="M3 7h11"/><path d="m9 11-2 3h3l-2 3"/>"#);

/// `expand`.
pub const EXPAND: Icon = lucide!("expand", r#"<path d="m15 15 6 6"/><path d="m15 9 6-6"/><path d="M21 16v5h-5"/><path d="M21 8V3h-5"/><path d="M3 16v5h5"/><path d="m3 21 6-6"/><path d="M3 8V3h5"/><path d="M9 9 3 3"/>"#);

/// `external-link`.
pub const EXTERNAL_LINK: Icon = lucide!("external-link", r#"<path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>"#);

/// `eye`.
pub const EYE: Icon = lucide!("eye", r#"<path d="M2.062 12.348a1 1 0 0 1 0-.696 10.75 10.75 0 0 1 19.876 0 1 1 0 0 1 0 .696 10.75 10.75 0 0 1-19.876 0"/><circle cx="12" cy="12" r="3"/>"#);

/// `eye-closed`.
pub const EYE_CLOSED: Icon = lucide!("eye-closed", r#"<path d="m15 18-.722-3.25"/><path d="M2 8a10.645 10.645 0 0 0 20 0"/><path d="m20 15-1.726-2.05"/><path d="m4 15 1.726-2.05"/><path d="m9 18 .722-3.25"/>"#);

/// `eye-dashed`.
pub const EYE_DASHED: Icon = lucide!("eye-dashed", r#"<path d="M13.054 18.946a11 11 0 0 1-2.11 0"/><path d="M13.054 5.054a11 11 0 0 0-2.11-.001"/><path d="M17.072 6.274a11 11 0 0 1 1.753 1.173"/><path d="M18.825 16.552a11 11 0 0 1-1.753 1.174"/><path d="M2.514 13.303a11 11 0 0 1-.452-.954 1 1 0 0 1 0-.697 11 11 0 0 1 .45-.955"/><path d="M21.485 10.697a11 11 0 0 1 .453.955 1 1 0 0 1 0 .697 11 11 0 0 1-.453.954"/><path d="M5.173 7.448a11 11 0 0 1 1.753-1.174"/><path d="M6.926 17.726a11 11 0 0 1-1.753-1.174"/><circle cx="12" cy="12" r="3"/>"#);

/// `eye-off`.
pub const EYE_OFF: Icon = lucide!("eye-off", r#"<path d="M10.733 5.076a10.744 10.744 0 0 1 11.205 6.575 1 1 0 0 1 0 .696 10.747 10.747 0 0 1-1.444 2.49"/><path d="M14.084 14.158a3 3 0 0 1-4.242-4.242"/><path d="M17.479 17.499a10.75 10.75 0 0 1-15.417-5.151 1 1 0 0 1 0-.696 10.75 10.75 0 0 1 4.446-5.143"/><path d="m2 2 20 20"/>"#);

/// `face-angry`.
pub const FACE_ANGRY: Icon = lucide!("face-angry", r#"<path d="M15 12v-1.584"/><path d="M17 10a5 5 0 00-3 1"/><path d="M7 10a5 5 0 013 1"/><path d="M9 12v-1.584"/><path d="M9 17a5 5 0 016.001 0"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-expressionless`.
pub const FACE_EXPRESSIONLESS: Icon = lucide!("face-expressionless", r#"<path d="M14 10h2"/><path d="M8 10h2"/><path d="M8 16h8"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-grinning`.
pub const FACE_GRINNING: Icon = lucide!("face-grinning", r#"<path d="M15 10V9"/><path d="M7.084 14.302a5.12 5.12 0 009.833 0 .24.24 0 00-.235-.302H7.32a.24.24 0 00-.235.302"/><path d="M9 10V9"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-neutral`.
pub const FACE_NEUTRAL: Icon = lucide!("face-neutral", r#"<path d="M15 10V9"/><path d="M8 16h8"/><path d="M9 10V9"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-slightly-frowning`.
pub const FACE_SLIGHTLY_FROWNING: Icon = lucide!("face-slightly-frowning", r#"<path d="M15 10V9"/><path d="M9 10V9"/><path d="M9 16a5 5 0 016 0"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-slightly-smiling`.
pub const FACE_SLIGHTLY_SMILING: Icon = lucide!("face-slightly-smiling", r#"<path d="M15 10V9"/><path d="M16.472 15a6 6 0 01-8.943 0"/><path d="M9 10V9"/><circle cx="12" cy="12" r="10"/>"#);

/// `face-slightly-smiling-plus`.
pub const FACE_SLIGHTLY_SMILING_PLUS: Icon = lucide!("face-slightly-smiling-plus", r#"<path d="M13.267 2.08a10 10 0 108.653 8.653"/><path d="M15 10V9"/><path d="M16 5h6"/><path d="M16.472 15a6 6 0 01-8.943 0"/><path d="M19 2v6"/><path d="M9 10V9"/>"#);

/// `factory`.
pub const FACTORY: Icon = lucide!("factory", r#"<path d="M12 16h.01"/><path d="M16 16h.01"/><path d="M3 19a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.5a.5.5 0 0 0-.769-.422l-4.462 2.844A.5.5 0 0 1 15 10.5v-2a.5.5 0 0 0-.769-.422L9.77 10.922A.5.5 0 0 1 9 10.5V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2z"/><path d="M8 16h.01"/>"#);

/// `fan`.
pub const FAN: Icon = lucide!("fan", r#"<path d="M10.827 16.379a6.082 6.082 0 0 1-8.618-7.002l5.412 1.45a6.082 6.082 0 0 1 7.002-8.618l-1.45 5.412a6.082 6.082 0 0 1 8.618 7.002l-5.412-1.45a6.082 6.082 0 0 1-7.002 8.618l1.45-5.412Z"/><path d="M12 12v.01"/>"#);

/// `fast-forward`.
pub const FAST_FORWARD: Icon = lucide!("fast-forward", r#"<path d="M12 6a2 2 0 0 1 3.414-1.414l6 6a2 2 0 0 1 0 2.828l-6 6A2 2 0 0 1 12 18z"/><path d="M2 6a2 2 0 0 1 3.414-1.414l6 6a2 2 0 0 1 0 2.828l-6 6A2 2 0 0 1 2 18z"/>"#);

/// `faucet`.
pub const FAUCET: Icon = lucide!("faucet", r#"<path d="M10.083 5.428 5.57 4.083a2 2 0 10.001 3.834l4.512-1.345"/><path d="M12 8v3"/><path d="m13.917 5.428 4.511-1.345a2 2 0 110 3.834l-4.51-1.345"/><path d="M18 17v-4.006"/><path d="M22 11v8"/><path d="M22 12h-3a1 1 0 00-1 .994h-2.539a4 4 0 00-6.915-.012L7 13a5 5 0 00-5 5v1a1 1 0 001 1h2a1 1 0 001-1v-1a1 1 0 01.995-1l1.552.018a4 4 0 006.907 0L18 17a1 1 0 001 1h3"/><circle cx="12" cy="6" r="2"/>"#);

/// `feather`.
pub const FEATHER: Icon = lucide!("feather", r#"<path d="M14.086 18.412A2 2 0 0112.67 19H5v-7.672a2 2 0 01.586-1.414L11.75 3.75a6 6 0 118.49 8.49z"/><path d="M16 8 2 22"/><path d="M17.488 15H9"/>"#);

/// `fence`.
pub const FENCE: Icon = lucide!("fence", r#"<path d="M4 3 2 5v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z"/><path d="M6 8h4"/><path d="M6 18h4"/><path d="m12 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z"/><path d="M14 8h4"/><path d="M14 18h4"/><path d="m20 3-2 2v15c0 .6.4 1 1 1h2c.6 0 1-.4 1-1V5Z"/>"#);

/// `ferris-wheel`.
pub const FERRIS_WHEEL: Icon = lucide!("ferris-wheel", r#"<circle cx="12" cy="12" r="2"/><path d="M12 2v4"/><path d="m6.8 15-3.5 2"/><path d="m20.7 7-3.5 2"/><path d="M6.8 9 3.3 7"/><path d="m20.7 17-3.5-2"/><path d="m9 22 3-8 3 8"/><path d="M8 22h8"/><path d="M18 18.7a9 9 0 1 0-12 0"/>"#);

/// `file`.
pub const FILE: Icon = lucide!("file", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/>"#);

/// `file-archive`.
pub const FILE_ARCHIVE: Icon = lucide!("file-archive", r#"<path d="M13.659 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v11.5"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 12v-1"/><path d="M8 18v-2"/><path d="M8 7V6"/><circle cx="8" cy="20" r="2"/>"#);

/// `file-axis-3d`.
pub const FILE_AXIS_3D: Icon = lucide!("file-axis-3d", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m8 18 4-4"/><path d="M8 10v8h8"/>"#);

/// `file-badge`.
pub const FILE_BADGE: Icon = lucide!("file-badge", r#"<path d="M13 22h5a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.3"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m7.69 16.479 1.29 4.88a.5.5 0 0 1-.698.591l-1.843-.849a1 1 0 0 0-.879.001l-1.846.85a.5.5 0 0 1-.692-.593l1.29-4.88"/><circle cx="6" cy="14" r="3"/>"#);

/// `file-box`.
pub const FILE_BOX: Icon = lucide!("file-box", r#"<path d="M14 2v5a1 1 0 001 1h5"/><path d="M14.692 22H18a2 2 0 002-2V8a2.4 2.4 0 00-.706-1.706l-3.588-3.588A2.4 2.4 0 0014 2H6a2 2 0 00-2 2v3.804"/><path d="M2.264 13.752 7 16.5l4.737-2.748"/><path d="M2.995 13.014A2 2 0 002 14.744v3.516a2 2 0 00.996 1.73l3 1.74a2 2 0 002.008 0l3-1.74A2 2 0 0012 18.26v-3.517a2 2 0 00-.995-1.73l-3-1.742a2 2 0 00-1.892-.064z"/><path d="M7 16.5V22"/>"#);

/// `file-braces`.
pub const FILE_BRACES: Icon = lucide!("file-braces", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 12a1 1 0 0 0-1 1v1a1 1 0 0 1-1 1 1 1 0 0 1 1 1v1a1 1 0 0 0 1 1"/><path d="M14 18a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-1a1 1 0 0 0-1-1"/>"#);

/// `file-braces-corner`.
pub const FILE_BRACES_CORNER: Icon = lucide!("file-braces-corner", r#"<path d="M14 22h4a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v6"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M5 14a1 1 0 0 0-1 1v2a1 1 0 0 1-1 1 1 1 0 0 1 1 1v2a1 1 0 0 0 1 1"/><path d="M9 22a1 1 0 0 0 1-1v-2a1 1 0 0 1 1-1 1 1 0 0 1-1-1v-2a1 1 0 0 0-1-1"/>"#);

/// `file-chart-column`.
pub const FILE_CHART_COLUMN: Icon = lucide!("file-chart-column", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 18v-1"/><path d="M12 18v-6"/><path d="M16 18v-3"/>"#);

/// `file-chart-column-increasing`.
pub const FILE_CHART_COLUMN_INCREASING: Icon = lucide!("file-chart-column-increasing", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 18v-2"/><path d="M12 18v-4"/><path d="M16 18v-6"/>"#);

/// `file-chart-line`.
pub const FILE_CHART_LINE: Icon = lucide!("file-chart-line", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m16 13-3.5 3.5-2-2L8 17"/>"#);

/// `file-chart-pie`.
pub const FILE_CHART_PIE: Icon = lucide!("file-chart-pie", r#"<path d="M15.941 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.704l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.512"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M4.017 11.512a6 6 0 1 0 8.466 8.475"/><path d="M9 16a1 1 0 0 1-1-1v-4c0-.552.45-1.008.995-.917a6 6 0 0 1 4.922 4.922c.091.544-.365.995-.917.995z"/>"#);

/// `file-check`.
pub const FILE_CHECK: Icon = lucide!("file-check", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m9 15 2 2 4-4"/>"#);

/// `file-check-corner`.
pub const FILE_CHECK_CORNER: Icon = lucide!("file-check-corner", r#"<path d="M10.5 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v6"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m14 20 2 2 4-4"/>"#);

/// `file-clock`.
pub const FILE_CLOCK: Icon = lucide!("file-clock", r#"<path d="M16 22h2a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v2.85"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 14v2.2l1.6 1"/><circle cx="8" cy="16" r="6"/>"#);

/// `file-code`.
pub const FILE_CODE: Icon = lucide!("file-code", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 12.5 8 15l2 2.5"/><path d="m14 12.5 2 2.5-2 2.5"/>"#);

/// `file-code-corner`.
pub const FILE_CODE_CORNER: Icon = lucide!("file-code-corner", r#"<path d="M4 12.15V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-3.35"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m5 16-3 3 3 3"/><path d="m9 22 3-3-3-3"/>"#);

/// `file-cog`.
pub const FILE_COG: Icon = lucide!("file-cog", r#"<path d="M15 8a1 1 0 0 1-1-1V2a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8z"/><path d="M20 8v12a2 2 0 0 1-2 2h-4.182"/><path d="m3.305 19.53.923-.382"/><path d="M4 10.592V4a2 2 0 0 1 2-2h8"/><path d="m4.228 16.852-.924-.383"/><path d="m5.852 15.228-.383-.923"/><path d="m5.852 20.772-.383.924"/><path d="m8.148 15.228.383-.923"/><path d="m8.53 21.696-.382-.924"/><path d="m9.773 16.852.922-.383"/><path d="m9.773 19.148.922.383"/><circle cx="7" cy="18" r="3"/>"#);

/// `file-database`.
pub const FILE_DATABASE: Icon = lucide!("file-database", r#"<path d="M12 12v8a5 2 0 01-10 0v-8"/><path d="M12 16a5 2 0 01-10 0"/><path d="M14 2v5a1 1 0 001 1h5"/><path d="M4 6.3V4a2 2 0 012-2h8a2.4 2.4 0 011.706.706l3.588 3.588A2.4 2.4 0 0120 8v12a2 2 0 01-2 2h-2.43"/><ellipse cx="7" cy="12" rx="5" ry="2"/>"#);

/// `file-diff`.
pub const FILE_DIFF: Icon = lucide!("file-diff", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M9 10h6"/><path d="M12 13V7"/><path d="M9 17h6"/>"#);

/// `file-digit`.
pub const FILE_DIGIT: Icon = lucide!("file-digit", r#"<path d="M4 12V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 16h2v6"/><path d="M10 22h4"/><rect x="2" y="16" width="4" height="6" rx="2"/>"#);

/// `file-down`.
pub const FILE_DOWN: Icon = lucide!("file-down", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M12 18v-6"/><path d="m9 15 3 3 3-3"/>"#);

/// `file-exclamation-point`.
pub const FILE_EXCLAMATION_POINT: Icon = lucide!("file-exclamation-point", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M12 9v4"/><path d="M12 17h.01"/>"#);

/// `file-headphone`.
pub const FILE_HEADPHONE: Icon = lucide!("file-headphone", r#"<path d="M4 6.835V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-.343"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M2 19a2 2 0 0 1 4 0v1a2 2 0 0 1-4 0v-4a6 6 0 0 1 12 0v4a2 2 0 0 1-4 0v-1a2 2 0 0 1 4 0"/>"#);

/// `file-heart`.
pub const FILE_HEART: Icon = lucide!("file-heart", r#"<path d="M13 22h5a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v7"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M3.62 18.8A2.25 2.25 0 1 1 7 15.836a2.25 2.25 0 1 1 3.38 2.966l-2.626 2.856a1 1 0 0 1-1.507 0z"/>"#);

/// `file-image`.
pub const FILE_IMAGE: Icon = lucide!("file-image", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><circle cx="10" cy="12" r="2"/><path d="m20 17-1.296-1.296a2.41 2.41 0 0 0-3.408 0L9 22"/>"#);

/// `file-input`.
pub const FILE_INPUT: Icon = lucide!("file-input", r#"<path d="M4 11V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-1"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M2 15h10"/><path d="m9 18 3-3-3-3"/>"#);

/// `file-key`.
pub const FILE_KEY: Icon = lucide!("file-key", r#"<path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M4 12v6"/><path d="M4 14h2"/><path d="M9.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v4"/><circle cx="4" cy="20" r="2"/>"#);

/// `file-lock`.
pub const FILE_LOCK: Icon = lucide!("file-lock", r#"<path d="M4 9.8V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-3"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M9 17v-2a2 2 0 0 0-4 0v2"/><rect width="8" height="5" x="3" y="17" rx="1"/>"#);

/// `file-minus`.
pub const FILE_MINUS: Icon = lucide!("file-minus", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M9 15h6"/>"#);

/// `file-minus-corner`.
pub const FILE_MINUS_CORNER: Icon = lucide!("file-minus-corner", r#"<path d="M20 14V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M14 18h6"/>"#);

/// `file-music`.
pub const FILE_MUSIC: Icon = lucide!("file-music", r#"<path d="M11.65 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v10.35"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 20v-7l3 1.474"/><circle cx="6" cy="20" r="2"/>"#);

/// `file-output`.
pub const FILE_OUTPUT: Icon = lucide!("file-output", r#"<path d="M4.226 20.925A2 2 0 0 0 6 22h12a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v3.127"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m5 11-3 3"/><path d="m5 17-3-3h10"/>"#);

/// `file-pen`.
pub const FILE_PEN: Icon = lucide!("file-pen", r#"<path d="M12.659 22H18a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v9.34"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10.378 12.622a1 1 0 0 1 3 3.003L8.36 20.637a2 2 0 0 1-.854.506l-2.867.837a.5.5 0 0 1-.62-.62l.836-2.869a2 2 0 0 1 .506-.853z"/>"#);

/// `file-pen-line`.
pub const FILE_PEN_LINE: Icon = lucide!("file-pen-line", r#"<path d="M14.364 13.634a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506l4.013-4.009a1 1 0 0 0-3.004-3.004z"/><path d="M14.487 7.858A1 1 0 0 1 14 7V2"/><path d="M20 19.645V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l2.516 2.516"/><path d="M8 18h1"/>"#);

/// `file-play`.
pub const FILE_PLAY: Icon = lucide!("file-play", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M15.033 13.44a.647.647 0 0 1 0 1.12l-4.065 2.352a.645.645 0 0 1-.968-.56v-4.704a.645.645 0 0 1 .967-.56z"/>"#);

/// `file-plus`.
pub const FILE_PLUS: Icon = lucide!("file-plus", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M9 15h6"/><path d="M12 18v-6"/>"#);

/// `file-plus-corner`.
pub const FILE_PLUS_CORNER: Icon = lucide!("file-plus-corner", r#"<path d="M11.35 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v5.35"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M14 19h6"/><path d="M17 16v6"/>"#);

/// `file-question-mark`.
pub const FILE_QUESTION_MARK: Icon = lucide!("file-question-mark", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M12 17h.01"/><path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3"/>"#);

/// `file-scan`.
pub const FILE_SCAN: Icon = lucide!("file-scan", r#"<path d="M20 10V8a2.4 2.4 0 0 0-.706-1.704l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h4.35"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M16 14a2 2 0 0 0-2 2"/><path d="M16 22a2 2 0 0 1-2-2"/><path d="M20 14a2 2 0 0 1 2 2"/><path d="M20 22a2 2 0 0 0 2-2"/>"#);

/// `file-search`.
pub const FILE_SEARCH: Icon = lucide!("file-search", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><circle cx="11.5" cy="14.5" r="2.5"/><path d="M13.3 16.3 15 18"/>"#);

/// `file-search-corner`.
pub const FILE_SEARCH_CORNER: Icon = lucide!("file-search-corner", r#"<path d="M11.1 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.589 3.588A2.4 2.4 0 0 1 20 8v3.25"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m21 22-2.88-2.88"/><circle cx="16" cy="17" r="3"/>"#);

/// `file-signal`.
pub const FILE_SIGNAL: Icon = lucide!("file-signal", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 15h.01"/><path d="M11.5 13.5a2.5 2.5 0 0 1 0 3"/><path d="M15 12a5 5 0 0 1 0 6"/>"#);

/// `file-sliders`.
pub const FILE_SLIDERS: Icon = lucide!("file-sliders", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 12h8"/><path d="M10 11v2"/><path d="M8 17h8"/><path d="M14 16v2"/>"#);

/// `file-spreadsheet`.
pub const FILE_SPREADSHEET: Icon = lucide!("file-spreadsheet", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M8 13h2"/><path d="M14 13h2"/><path d="M8 17h2"/><path d="M14 17h2"/>"#);

/// `file-stack`.
pub const FILE_STACK: Icon = lucide!("file-stack", r#"<path d="M11 21a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-8a1 1 0 0 1 1-1"/><path d="M16 16a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1V8a1 1 0 0 1 1-1"/><path d="M21 6a2 2 0 0 0-.586-1.414l-2-2A2 2 0 0 0 17 2h-3a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1z"/>"#);

/// `file-symlink`.
pub const FILE_SYMLINK: Icon = lucide!("file-symlink", r#"<path d="M4 11V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m10 18 3-3-3-3"/>"#);

/// `file-terminal`.
pub const FILE_TERMINAL: Icon = lucide!("file-terminal", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m8 16 2-2-2-2"/><path d="M12 18h4"/>"#);

/// `file-text`.
pub const FILE_TEXT: Icon = lucide!("file-text", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 9H8"/><path d="M16 13H8"/><path d="M16 17H8"/>"#);

/// `file-type`.
pub const FILE_TYPE: Icon = lucide!("file-type", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M11 18h2"/><path d="M12 12v6"/><path d="M9 13v-.5a.5.5 0 0 1 .5-.5h5a.5.5 0 0 1 .5.5v.5"/>"#);

/// `file-type-corner`.
pub const FILE_TYPE_CORNER: Icon = lucide!("file-type-corner", r#"<path d="M12 22h6a2 2 0 0 0 2-2V8a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 14 2H6a2 2 0 0 0-2 2v6"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M3 16v-1.5a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 .5.5V16"/><path d="M6 22h2"/><path d="M7 14v8"/>"#);

/// `file-up`.
pub const FILE_UP: Icon = lucide!("file-up", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M12 12v6"/><path d="m15 15-3-3-3 3"/>"#);

/// `file-user`.
pub const FILE_USER: Icon = lucide!("file-user", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M16 22a4 4 0 0 0-8 0"/><circle cx="12" cy="15" r="3"/>"#);

/// `file-video-camera`.
pub const FILE_VIDEO_CAMERA: Icon = lucide!("file-video-camera", r#"<path d="M4 12V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m10 17.843 3.033-1.755a.64.64 0 0 1 .967.56v4.704a.65.65 0 0 1-.967.56L10 20.157"/><rect width="7" height="6" x="3" y="16" rx="1"/>"#);

/// `file-volume`.
pub const FILE_VOLUME: Icon = lucide!("file-volume", r#"<path d="M4 11.55V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2h-1.95"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M12 15a5 5 0 0 1 0 6"/><path d="M8 14.502a.5.5 0 0 0-.826-.381l-1.893 1.631a1 1 0 0 1-.651.243H3.5a.5.5 0 0 0-.5.501v3.006a.5.5 0 0 0 .5.501h1.129a1 1 0 0 1 .652.243l1.893 1.633a.5.5 0 0 0 .826-.38z"/>"#);

/// `file-x`.
pub const FILE_X: Icon = lucide!("file-x", r#"<path d="M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m14.5 12.5-5 5"/><path d="m9.5 12.5 5 5"/>"#);

/// `file-x-corner`.
pub const FILE_X_CORNER: Icon = lucide!("file-x-corner", r#"<path d="M11 22H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v5"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="m15 17 5 5"/><path d="m20 17-5 5"/>"#);

/// `files`.
pub const FILES: Icon = lucide!("files", r#"<path d="M15 2h-4a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V8"/><path d="M16.706 2.706A2.4 2.4 0 0 0 15 2v5a1 1 0 0 0 1 1h5a2.4 2.4 0 0 0-.706-1.706z"/><path d="M5 7a2 2 0 0 0-2 2v11a2 2 0 0 0 2 2h8a2 2 0 0 0 1.732-1"/>"#);

/// `film`.
pub const FILM: Icon = lucide!("film", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 3v18"/><path d="M3 7.5h4"/><path d="M3 12h18"/><path d="M3 16.5h4"/><path d="M17 3v18"/><path d="M17 7.5h4"/><path d="M17 16.5h4"/>"#);

/// `fingerprint-pattern`.
pub const FINGERPRINT_PATTERN: Icon = lucide!("fingerprint-pattern", r#"<path d="M12 10a2 2 0 0 0-2 2c0 1.02-.1 2.51-.26 4"/><path d="M14 13.12c0 2.38 0 6.38-1 8.88"/><path d="M17.29 21.02c.12-.6.43-2.3.5-3.02"/><path d="M2 12a10 10 0 0 1 18-6"/><path d="M2 16h.01"/><path d="M21.8 16c.2-2 .131-5.354 0-6"/><path d="M5 19.5C5.5 18 6 15 6 12a6 6 0 0 1 .34-2"/><path d="M8.65 22c.21-.66.45-1.32.57-2"/><path d="M9 6.8a6 6 0 0 1 9 5.2v2"/>"#);

/// `fire-extinguisher`.
pub const FIRE_EXTINGUISHER: Icon = lucide!("fire-extinguisher", r#"<path d="M15 6.5V3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3.5"/><path d="M9 18h8"/><path d="M18 3h-3"/><path d="M11 3a6 6 0 0 0-6 6v11"/><path d="M5 13h4"/><path d="M17 10a4 4 0 0 0-8 0v10a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2Z"/>"#);

/// `fish`.
pub const FISH: Icon = lucide!("fish", r#"<path d="M6.5 12c.94-3.46 4.94-6 8.5-6 3.56 0 6.06 2.54 7 6-.94 3.47-3.44 6-7 6s-7.56-2.53-8.5-6Z"/><path d="M18 12v.5"/><path d="M16 17.93a9.77 9.77 0 0 1 0-11.86"/><path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33"/><path d="M10.46 7.26C10.2 5.88 9.17 4.24 8 3h5.8a2 2 0 0 1 1.98 1.67l.23 1.4"/><path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98"/>"#);

/// `fish-off`.
pub const FISH_OFF: Icon = lucide!("fish-off", r#"<path d="M18 12.47v.03m0-.5v.47m-.475 5.056A6.744 6.744 0 0 1 15 18c-3.56 0-7.56-2.53-8.5-6 .348-1.28 1.114-2.433 2.121-3.38m3.444-2.088A8.802 8.802 0 0 1 15 6c3.56 0 6.06 2.54 7 6-.309 1.14-.786 2.177-1.413 3.058"/><path d="M7 10.67C7 8 5.58 5.97 2.73 5.5c-1 1.5-1 5 .23 6.5-1.24 1.5-1.24 5-.23 6.5C5.58 18.03 7 16 7 13.33m7.48-4.372A9.77 9.77 0 0 1 16 6.07m0 11.86a9.77 9.77 0 0 1-1.728-3.618"/><path d="m16.01 17.93-.23 1.4A2 2 0 0 1 13.8 21H9.5a5.96 5.96 0 0 0 1.49-3.98M8.53 3h5.27a2 2 0 0 1 1.98 1.67l.23 1.4M2 2l20 20"/>"#);

/// `fish-symbol`.
pub const FISH_SYMBOL: Icon = lucide!("fish-symbol", r#"<path d="M2 16s9-15 20-4C11 23 2 8 2 8"/>"#);

/// `fishing-hook`.
pub const FISHING_HOOK: Icon = lucide!("fishing-hook", r#"<path d="m17.586 11.414-5.93 5.93a1 1 0 0 1-8-8l3.137-3.137a.707.707 0 0 1 1.207.5V10"/><path d="M20.414 8.586 22 7"/><circle cx="19" cy="10" r="2"/>"#);

/// `fishing-rod`.
pub const FISHING_ROD: Icon = lucide!("fishing-rod", r#"<path d="M4 11h1"/><path d="M8 15a2 2 0 0 1-4 0V3a1 1 0 0 1 1-1h.5C14 2 20 9 20 18v4"/><circle cx="18" cy="18" r="2"/>"#);

/// `flag`.
pub const FLAG: Icon = lucide!("flag", r#"<path d="M4 22V4a1 1 0 0 1 .4-.8A6 6 0 0 1 8 2c3 0 5 2 7.333 2q2 0 3.067-.8A1 1 0 0 1 20 4v10a1 1 0 0 1-.4.8A6 6 0 0 1 16 16c-3 0-5-2-8-2a6 6 0 0 0-4 1.528"/>"#);

/// `flag-off`.
pub const FLAG_OFF: Icon = lucide!("flag-off", r#"<path d="M16 16c-3 0-5-2-8-2a6 6 0 0 0-4 1.528"/><path d="m2 2 20 20"/><path d="M4 22V4"/><path d="M7.656 2H8c3 0 5 2 7.333 2q2 0 3.067-.8A1 1 0 0 1 20 4v10.347"/>"#);

/// `flag-triangle-left`.
pub const FLAG_TRIANGLE_LEFT: Icon = lucide!("flag-triangle-left", r#"<path d="M18 22V2.8a.8.8 0 0 0-1.17-.71L5.45 7.78a.8.8 0 0 0 0 1.44L18 15.5"/>"#);

/// `flag-triangle-right`.
pub const FLAG_TRIANGLE_RIGHT: Icon = lucide!("flag-triangle-right", r#"<path d="M6 22V2.8a.8.8 0 0 1 1.17-.71l11.38 5.69a.8.8 0 0 1 0 1.44L6 15.5"/>"#);

/// `flame`.
pub const FLAME: Icon = lucide!("flame", r#"<path d="M12 3q1 4 4 6.5t3 5.5a1 1 0 0 1-14 0 5 5 0 0 1 1-3 1 1 0 0 0 5 0c0-2-1.5-3-1.5-5q0-2 2.5-4"/>"#);

/// `flame-kindling`.
pub const FLAME_KINDLING: Icon = lucide!("flame-kindling", r#"<path d="M12 2c1 3 2.5 3.5 3.5 4.5A5 5 0 0 1 17 10a5 5 0 1 1-10 0c0-.3 0-.6.1-.9a2 2 0 1 0 3.3-2C8 4.5 11 2 12 2Z"/><path d="m5 22 14-4"/><path d="m5 18 14 4"/>"#);

/// `flashlight`.
pub const FLASHLIGHT: Icon = lucide!("flashlight", r#"<path d="M12 13v1"/><path d="M17 2a1 1 0 0 1 1 1v4a3 3 0 0 1-.6 1.8l-.6.8A4 4 0 0 0 16 12v8a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2v-8a4 4 0 0 0-.8-2.4l-.6-.8A3 3 0 0 1 6 7V3a1 1 0 0 1 1-1z"/><path d="M6 6h12"/>"#);

/// `flashlight-off`.
pub const FLASHLIGHT_OFF: Icon = lucide!("flashlight-off", r#"<path d="M11.652 6H18"/><path d="M12 13v1"/><path d="M16 16v4a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-8a4 4 0 0 0-.8-2.4l-.6-.8A3 3 0 0 1 6 7V6"/><path d="m2 2 20 20"/><path d="M7.649 2H17a1 1 0 0 1 1 1v4a3 3 0 0 1-.6 1.8l-.6.8a4 4 0 0 0-.55 1.007"/>"#);

/// `flask-conical`.
pub const FLASK_CONICAL: Icon = lucide!("flask-conical", r#"<path d="M14 2v6a2 2 0 0 0 .245.96l5.51 10.08A2 2 0 0 1 18 22H6a2 2 0 0 1-1.755-2.96l5.51-10.08A2 2 0 0 0 10 8V2"/><path d="M6.453 15h11.094"/><path d="M8.5 2h7"/>"#);

/// `flask-conical-off`.
pub const FLASK_CONICAL_OFF: Icon = lucide!("flask-conical-off", r#"<path d="M10 2v2.343"/><path d="M14 2v6.343"/><path d="m2 2 20 20"/><path d="M20 20a2 2 0 0 1-2 2H6a2 2 0 0 1-1.755-2.96l5.227-9.563"/><path d="M6.453 15H15"/><path d="M8.5 2h7"/>"#);

/// `flask-round`.
pub const FLASK_ROUND: Icon = lucide!("flask-round", r#"<path d="M10 2v6.292a7 7 0 1 0 4 0V2"/><path d="M5 15h14"/><path d="M8.5 2h7"/>"#);

/// `flower`.
pub const FLOWER: Icon = lucide!("flower", r#"<circle cx="12" cy="12" r="3"/><path d="M12 16.5A4.5 4.5 0 1 1 7.5 12 4.5 4.5 0 1 1 12 7.5a4.5 4.5 0 1 1 4.5 4.5 4.5 4.5 0 1 1-4.5 4.5"/><path d="M12 7.5V9"/><path d="M7.5 12H9"/><path d="M16.5 12H15"/><path d="M12 16.5V15"/><path d="m8 8 1.88 1.88"/><path d="M14.12 9.88 16 8"/><path d="m8 16 1.88-1.88"/><path d="M14.12 14.12 16 16"/>"#);

/// `flower-2`.
pub const FLOWER_2: Icon = lucide!("flower-2", r#"<path d="M12 5a3 3 0 1 1 3 3m-3-3a3 3 0 1 0-3 3m3-3v1M9 8a3 3 0 1 0 3 3M9 8h1m5 0a3 3 0 1 1-3 3m3-3h-1m-2 3v-1"/><circle cx="12" cy="8" r="2"/><path d="M12 10v12"/><path d="M12 22c4.2 0 7-1.667 7-5-4.2 0-7 1.667-7 5Z"/><path d="M12 22c-4.2 0-7-1.667-7-5 4.2 0 7 1.667 7 5Z"/>"#);

/// `focus`.
pub const FOCUS: Icon = lucide!("focus", r#"<circle cx="12" cy="12" r="3"/><path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/>"#);

/// `fold-horizontal`.
pub const FOLD_HORIZONTAL: Icon = lucide!("fold-horizontal", r#"<path d="M2 12h6"/><path d="M22 12h-6"/><path d="M12 2v2"/><path d="M12 8v2"/><path d="M12 14v2"/><path d="M12 20v2"/><path d="m19 9-3 3 3 3"/><path d="m5 15 3-3-3-3"/>"#);

/// `fold-vertical`.
pub const FOLD_VERTICAL: Icon = lucide!("fold-vertical", r#"<path d="M12 22v-6"/><path d="M12 8V2"/><path d="M4 12H2"/><path d="M10 12H8"/><path d="M16 12h-2"/><path d="M22 12h-2"/><path d="m15 19-3-3-3 3"/><path d="m15 5-3 3-3-3"/>"#);

/// `folder`.
pub const FOLDER: Icon = lucide!("folder", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#);

/// `folder-archive`.
pub const FOLDER_ARCHIVE: Icon = lucide!("folder-archive", r#"<circle cx="15" cy="19" r="2"/><path d="M20.9 19.8A2 2 0 0 0 22 18V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2h5.1"/><path d="M15 11v-1"/><path d="M15 17v-2"/>"#);

/// `folder-bookmark`.
pub const FOLDER_BOOKMARK: Icon = lucide!("folder-bookmark", r#"<path d="M12 6v7.751a.25.25 0 00.407.195l2.28-1.834a.5.5 0 01.627 0l2.28 1.834a.25.25 0 00.406-.195V6"/><path d="M20 20a2 2 0 002-2V8a2 2 0 00-2-2h-7.9a2 2 0 01-1.69-.9L9.6 3.9A2 2 0 007.93 3H4a2 2 0 00-2 2v13a2 2 0 002 2z"/>"#);

/// `folder-check`.
pub const FOLDER_CHECK: Icon = lucide!("folder-check", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="m9 13 2 2 4-4"/>"#);

/// `folder-clock`.
pub const FOLDER_CLOCK: Icon = lucide!("folder-clock", r#"<path d="M16 14v2.2l1.6 1"/><path d="M7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2"/><circle cx="16" cy="16" r="6"/>"#);

/// `folder-closed`.
pub const FOLDER_CLOSED: Icon = lucide!("folder-closed", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M2 10h20"/>"#);

/// `folder-code`.
pub const FOLDER_CODE: Icon = lucide!("folder-code", r#"<path d="M10 10.5 8 13l2 2.5"/><path d="m14 10.5 2 2.5-2 2.5"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2z"/>"#);

/// `folder-cog`.
pub const FOLDER_COG: Icon = lucide!("folder-cog", r#"<path d="M10.3 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.98a2 2 0 0 1 1.69.9l.66 1.2A2 2 0 0 0 12 6h8a2 2 0 0 1 2 2v3.3"/><path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><circle cx="18" cy="18" r="3"/>"#);

/// `folder-dot`.
pub const FOLDER_DOT: Icon = lucide!("folder-dot", r#"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><circle cx="12" cy="13" r="1"/>"#);

/// `folder-down`.
pub const FOLDER_DOWN: Icon = lucide!("folder-down", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M12 10v6"/><path d="m15 13-3 3-3-3"/>"#);

/// `folder-git`.
pub const FOLDER_GIT: Icon = lucide!("folder-git", r#"<circle cx="12" cy="13" r="2"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M14 13h3"/><path d="M7 13h3"/>"#);

/// `folder-git-2`.
pub const FOLDER_GIT_2: Icon = lucide!("folder-git-2", r#"<path d="M18 19a5 5 0 0 1-5-5v8"/><path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v5"/><circle cx="13" cy="12" r="2"/><circle cx="20" cy="19" r="2"/>"#);

/// `folder-heart`.
pub const FOLDER_HEART: Icon = lucide!("folder-heart", r#"<path d="M10.638 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v3.417"/><path d="M14.62 18.8A2.25 2.25 0 1 1 18 15.836a2.25 2.25 0 1 1 3.38 2.966l-2.626 2.856a.998.998 0 0 1-1.507 0z"/>"#);

/// `folder-input`.
pub const FOLDER_INPUT: Icon = lucide!("folder-input", r#"<path d="M2 9V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-1"/><path d="M2 13h10"/><path d="m9 16 3-3-3-3"/>"#);

/// `folder-kanban`.
pub const FOLDER_KANBAN: Icon = lucide!("folder-kanban", r#"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><path d="M8 10v4"/><path d="M12 10v2"/><path d="M16 10v6"/>"#);

/// `folder-key`.
pub const FOLDER_KEY: Icon = lucide!("folder-key", r#"<path d="M13 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v1.36"/><path d="M19 12v6"/><path d="M19 14h2"/><circle cx="19" cy="20" r="2"/>"#);

/// `folder-lock`.
pub const FOLDER_LOCK: Icon = lucide!("folder-lock", r#"<rect width="8" height="5" x="14" y="17" rx="1"/><path d="M10 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v2.5"/><path d="M20 17v-2a2 2 0 1 0-4 0v2"/>"#);

/// `folder-minus`.
pub const FOLDER_MINUS: Icon = lucide!("folder-minus", r#"<path d="M9 13h6"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#);

/// `folder-open`.
pub const FOLDER_OPEN: Icon = lucide!("folder-open", r#"<path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/>"#);

/// `folder-open-dot`.
pub const FOLDER_OPEN_DOT: Icon = lucide!("folder-open-dot", r#"<path d="m6 14 1.45-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.55 6a2 2 0 0 1-1.94 1.5H4a2 2 0 0 1-2-2V5c0-1.1.9-2 2-2h3.93a2 2 0 0 1 1.66.9l.82 1.2a2 2 0 0 0 1.66.9H18a2 2 0 0 1 2 2v2"/><circle cx="14" cy="15" r="1"/>"#);

/// `folder-output`.
pub const FOLDER_OUTPUT: Icon = lucide!("folder-output", r#"<path d="M2 7.5V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-1.5"/><path d="M2 13h10"/><path d="m5 10-3 3 3 3"/>"#);

/// `folder-pen`.
pub const FOLDER_PEN: Icon = lucide!("folder-pen", r#"<path d="M2 11.5V5a2 2 0 0 1 2-2h3.9c.7 0 1.3.3 1.7.9l.8 1.2c.4.6 1 .9 1.7.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-9.5"/><path d="M11.378 13.626a1 1 0 1 0-3.004-3.004l-5.01 5.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/>"#);

/// `folder-plus`.
pub const FOLDER_PLUS: Icon = lucide!("folder-plus", r#"<path d="M12 10v6"/><path d="M9 13h6"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>"#);

/// `folder-root`.
pub const FOLDER_ROOT: Icon = lucide!("folder-root", r#"<path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"/><circle cx="12" cy="13" r="2"/><path d="M12 15v5"/>"#);

/// `folder-search`.
pub const FOLDER_SEARCH: Icon = lucide!("folder-search", r#"<path d="M10.7 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v4.1"/><path d="m21 21-1.9-1.9"/><circle cx="17" cy="17" r="3"/>"#);

/// `folder-search-2`.
pub const FOLDER_SEARCH_2: Icon = lucide!("folder-search-2", r#"<circle cx="11.5" cy="12.5" r="2.5"/><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M13.3 14.3 15 16"/>"#);

/// `folder-symlink`.
pub const FOLDER_SYMLINK: Icon = lucide!("folder-symlink", r#"<path d="M2 9.35V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h7"/><path d="m8 16 3-3-3-3"/>"#);

/// `folder-sync`.
pub const FOLDER_SYNC: Icon = lucide!("folder-sync", r#"<path d="M9 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2v.5"/><path d="M12 10v4h4"/><path d="m12 14 1.535-1.605a5 5 0 0 1 8 1.5"/><path d="M22 22v-4h-4"/><path d="m22 18-1.535 1.605a5 5 0 0 1-8-1.5"/>"#);

/// `folder-tree`.
pub const FOLDER_TREE: Icon = lucide!("folder-tree", r#"<path d="M20 10a1 1 0 0 0 1-1V6a1 1 0 0 0-1-1h-2.5a1 1 0 0 1-.8-.4l-.9-1.2A1 1 0 0 0 15 3h-2a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"/><path d="M20 21a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-2.9a1 1 0 0 1-.88-.55l-.42-.85a1 1 0 0 0-.92-.6H13a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1Z"/><path d="M3 5a2 2 0 0 0 2 2h3"/><path d="M3 3v13a2 2 0 0 0 2 2h3"/>"#);

/// `folder-up`.
pub const FOLDER_UP: Icon = lucide!("folder-up", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M12 10v6"/><path d="m9 13 3-3 3 3"/>"#);

/// `folder-user`.
pub const FOLDER_USER: Icon = lucide!("folder-user", r#"<path d="M10.254 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2V9.255"/><path d="M22 22a4 4 0 0 0-8 0"/><circle cx="18" cy="15" r="3"/>"#);

/// `folder-users`.
pub const FOLDER_USERS: Icon = lucide!("folder-users", r#"<path d="M5.254 20H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H20a2 2 0 0 1 2 2V9.255"/><circle cx="13" cy="15" r="3"/><path d="M17 22a4 4 0 0 0-8 0"/><path d="M22 22A4 4 0 0 0 20.24 18.686 1 1 0 0 1 20.1 17.143 3 3 0 0 0 19.5 12.402"/>"#);

/// `folder-x`.
pub const FOLDER_X: Icon = lucide!("folder-x", r#"<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="m9.5 10.5 5 5"/><path d="m14.5 10.5-5 5"/>"#);

/// `folders`.
pub const FOLDERS: Icon = lucide!("folders", r#"<path d="M20 5a2 2 0 0 1 2 2v7a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h2.5a1.5 1.5 0 0 1 1.2.6l.6.8a1.5 1.5 0 0 0 1.2.6z"/><path d="M3 8.268a2 2 0 0 0-1 1.738V19a2 2 0 0 0 2 2h11a2 2 0 0 0 1.732-1"/>"#);

/// `footprints`.
pub const FOOTPRINTS: Icon = lucide!("footprints", r#"<path d="M4 16v-2.38C4 11.5 2.97 10.5 3 8c.03-2.72 1.49-6 4.5-6C9.37 2 10 3.8 10 5.5c0 3.11-2 5.66-2 8.68V16a2 2 0 1 1-4 0Z"/><path d="M20 20v-2.38c0-2.12 1.03-3.12 1-5.62-.03-2.72-1.49-6-4.5-6C14.63 6 14 7.8 14 9.5c0 3.11 2 5.66 2 8.68V20a2 2 0 1 0 4 0Z"/><path d="M16 17h4"/><path d="M4 13h4"/>"#);

/// `forklift`.
pub const FORKLIFT: Icon = lucide!("forklift", r#"<path d="M12 12H5a2 2 0 0 0-2 2v5"/><path d="M15 19h7"/><path d="M16 19V2"/><path d="M6 12V7a2 2 0 0 1 2-2h2.172a2 2 0 0 1 1.414.586l3.828 3.828A2 2 0 0 1 16 10.828"/><path d="M7 19h4"/><circle cx="13" cy="19" r="2"/><circle cx="5" cy="19" r="2"/>"#);

/// `form`.
pub const FORM: Icon = lucide!("form", r#"<path d="M4 14h6"/><path d="M4 2h10"/><rect x="4" y="18" width="16" height="4" rx="1"/><rect x="4" y="6" width="16" height="4" rx="1"/>"#);

/// `forward`.
pub const FORWARD: Icon = lucide!("forward", r#"<path d="m15 17 5-5-5-5"/><path d="M4 18v-2a4 4 0 0 1 4-4h12"/>"#);

/// `frame`.
pub const FRAME: Icon = lucide!("frame", r#"<line x1="22" x2="2" y1="6" y2="6"/><line x1="22" x2="2" y1="18" y2="18"/><line x1="6" x2="6" y1="2" y2="22"/><line x1="18" x2="18" y1="2" y2="22"/>"#);

/// `fuel`.
pub const FUEL: Icon = lucide!("fuel", r#"<path d="M14 13h2a2 2 0 0 1 2 2v2a2 2 0 0 0 4 0v-6.998a2 2 0 0 0-.59-1.42L18 5"/><path d="M14 21V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v16"/><path d="M2 21h13"/><path d="M3 9h11"/>"#);

/// `fullscreen`.
pub const FULLSCREEN: Icon = lucide!("fullscreen", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><rect width="10" height="8" x="7" y="8" rx="1"/>"#);

/// `funnel`.
pub const FUNNEL: Icon = lucide!("funnel", r#"<path d="M10 20a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341L21.74 4.67A1 1 0 0 0 21 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14z"/>"#);

/// `funnel-plus`.
pub const FUNNEL_PLUS: Icon = lucide!("funnel-plus", r#"<path d="M13.354 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14v6a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341l1.218-1.348"/><path d="M16 6h6"/><path d="M19 3v6"/>"#);

/// `funnel-x`.
pub const FUNNEL_X: Icon = lucide!("funnel-x", r#"<path d="M12.531 3H3a1 1 0 0 0-.742 1.67l7.225 7.989A2 2 0 0 1 10 14v6a1 1 0 0 0 .553.895l2 1A1 1 0 0 0 14 21v-7a2 2 0 0 1 .517-1.341l.427-.473"/><path d="m16.5 3.5 5 5"/><path d="m21.5 3.5-5 5"/>"#);

/// `galaxy`.
pub const GALAXY: Icon = lucide!("galaxy", r#"<path d="M16.005 15.108a5.041 6.52 28.25 00-8.008-6.217 5.041 6.52 28.25 008.008 6.217A11.884 7.288-60.76 014.029 7.001"/><path d="M17 21h.01"/><path d="M7 3h.01"/><path d="M7.997 8.891a11.885 7.288-60.756 0111.977 8.107"/><circle cx="12" cy="12" r="1" fill="currentColor"/>"#);

/// `gallery-horizontal`.
pub const GALLERY_HORIZONTAL: Icon = lucide!("gallery-horizontal", r#"<path d="M2 3v18"/><rect width="12" height="18" x="6" y="3" rx="2"/><path d="M22 3v18"/>"#);

/// `gallery-horizontal-end`.
pub const GALLERY_HORIZONTAL_END: Icon = lucide!("gallery-horizontal-end", r#"<path d="M2 7v10"/><path d="M6 5v14"/><rect width="12" height="18" x="10" y="3" rx="2"/>"#);

/// `gallery-thumbnails`.
pub const GALLERY_THUMBNAILS: Icon = lucide!("gallery-thumbnails", r#"<rect width="18" height="14" x="3" y="3" rx="2"/><path d="M4 21h1"/><path d="M9 21h1"/><path d="M14 21h1"/><path d="M19 21h1"/>"#);

/// `gallery-vertical`.
pub const GALLERY_VERTICAL: Icon = lucide!("gallery-vertical", r#"<path d="M3 2h18"/><rect width="18" height="12" x="3" y="6" rx="2"/><path d="M3 22h18"/>"#);

/// `gallery-vertical-end`.
pub const GALLERY_VERTICAL_END: Icon = lucide!("gallery-vertical-end", r#"<path d="M7 2h10"/><path d="M5 6h14"/><rect width="18" height="12" x="3" y="10" rx="2"/>"#);

/// `gamepad`.
pub const GAMEPAD: Icon = lucide!("gamepad", r#"<line x1="6" x2="10" y1="12" y2="12"/><line x1="8" x2="8" y1="10" y2="14"/><line x1="15" x2="15.01" y1="13" y2="13"/><line x1="18" x2="18.01" y1="11" y2="11"/><rect width="20" height="12" x="2" y="6" rx="2"/>"#);

/// `gamepad-2`.
pub const GAMEPAD_2: Icon = lucide!("gamepad-2", r#"<line x1="6" x2="10" y1="11" y2="11"/><line x1="8" x2="8" y1="9" y2="13"/><line x1="15" x2="15.01" y1="12" y2="12"/><line x1="18" x2="18.01" y1="10" y2="10"/><path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"/>"#);

/// `gamepad-directional`.
pub const GAMEPAD_DIRECTIONAL: Icon = lucide!("gamepad-directional", r#"<path d="M11.146 15.854a1.207 1.207 0 0 1 1.708 0l1.56 1.56A2 2 0 0 1 15 18.828V21a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-2.172a2 2 0 0 1 .586-1.414z"/><path d="M18.828 15a2 2 0 0 1-1.414-.586l-1.56-1.56a1.207 1.207 0 0 1 0-1.708l1.56-1.56A2 2 0 0 1 18.828 9H21a1 1 0 0 1 1 1v4a1 1 0 0 1-1 1z"/><path d="M6.586 14.414A2 2 0 0 1 5.172 15H3a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1h2.172a2 2 0 0 1 1.414.586l1.56 1.56a1.207 1.207 0 0 1 0 1.708z"/><path d="M9 3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2.172a2 2 0 0 1-.586 1.414l-1.56 1.56a1.207 1.207 0 0 1-1.708 0l-1.56-1.56A2 2 0 0 1 9 5.172z"/>"#);

/// `gap-horizontal`.
pub const GAP_HORIZONTAL: Icon = lucide!("gap-horizontal", r#"<path d="M12 2v2"/><path d="M12 8v2"/><path d="M12 14v2"/><path d="M12 20v2"/><path d="M21 3h-3a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h3"/><path d="M3 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H3"/>"#);

/// `gap-vertical`.
pub const GAP_VERTICAL: Icon = lucide!("gap-vertical", r#"<path d="M2 12h2"/><path d="M8 12h2"/><path d="M14 12h2"/><path d="M20 12h2"/><path d="M3 21v-3a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3"/><path d="M3 3v3a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V3"/>"#);

/// `gauge`.
pub const GAUGE: Icon = lucide!("gauge", r#"<path d="m12 14 4-4"/><path d="M3.34 19a10 10 0 1 1 17.32 0"/>"#);

/// `gavel`.
pub const GAVEL: Icon = lucide!("gavel", r#"<path d="m14 13-8.381 8.38a1 1 0 0 1-3.001-3l8.384-8.381"/><path d="m16 16 6-6"/><path d="m21.5 10.5-8-8"/><path d="m8 8 6-6"/><path d="m8.5 7.5 8 8"/>"#);

/// `gem`.
pub const GEM: Icon = lucide!("gem", r#"<path d="M10.5 3 8 9l4 13 4-13-2.5-6"/><path d="M17 3a2 2 0 0 1 1.6.8l3 4a2 2 0 0 1 .013 2.382l-7.99 10.986a2 2 0 0 1-3.247 0l-7.99-10.986A2 2 0 0 1 2.4 7.8l2.998-3.997A2 2 0 0 1 7 3z"/><path d="M2 9h20"/>"#);

/// `georgian-lari`.
pub const GEORGIAN_LARI: Icon = lucide!("georgian-lari", r#"<path d="M10 10V3"/><path d="M14 10V3"/><path d="M19 13a7 7 0 10-7 7"/><path d="M5 20h14"/>"#);

/// `germ`.
pub const GERM: Icon = lucide!("germ", r#"<path d="m11 2 .925 1.848"/><path d="M13 15h.01"/><path d="m16 21-1-2.472"/><path d="m19 2-1 1.804"/><path d="m2 19 2.746-1.373"/><path d="m22 16-2.474-2.13"/><path d="m22 5-1.804 1"/><path d="m3 10 2 2"/><path d="M9 16h.01"/><path d="M9 20v2"/><path d="M9.33 7.035c-.51 1.478-1.786 2.93-3.09 3.794A5 5 0 009 20a12.1 12.1 0 0011.902-9.916A6 6 0 009.33 7.035"/><circle cx="15" cy="9" r="2"/>"#);

/// `germ-off`.
pub const GERM_OFF: Icon = lucide!("germ-off", r#"<path d="m11 2 .925 1.848"/><path d="M13 15h.01"/><path d="M13.424 7.768a2 2 0 112.808 2.808"/><path d="m16 21-1-2.472"/><path d="M16.988 16.988A12 12 0 019 20a5 5 0 01-2.759-9.171 8.8 8.8 0 002.307-2.28"/><path d="m19 2-1 1.804"/><path d="m2 19 2.746-1.373"/><path d="m2 2 20 20"/><path d="m22 16-2.474-2.13a12 12 0 001.376-3.786 6 6 0 00-10.313-5.151"/><path d="m22 5-1.804 1"/><path d="m3 10 2 2"/><path d="M9 16h.01"/><path d="M9 20v2"/>"#);

/// `ghost`.
pub const GHOST: Icon = lucide!("ghost", r#"<path d="M15 10v1"/><path d="M7.528 20.472a1.6 1.6 0 012.277 0l1.057 1.056a1.6 1.6 0 002.276 0l1.057-1.056a1.6 1.6 0 012.277 0l1.114 1.114a1.4 1.4 0 002.414-1V10a8 8 0 00-16 0v10.586a1.4 1.4 0 002.414 1z"/><path d="M9 10v1"/>"#);

/// `gift`.
pub const GIFT: Icon = lucide!("gift", r#"<path d="M12 7v14"/><path d="M20 11v8a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2v-8"/><path d="M7.5 7a1 1 0 0 1 0-5A4.8 8 0 0 1 12 7a4.8 8 0 0 1 4.5-5 1 1 0 0 1 0 5"/><rect x="3" y="7" width="18" height="4" rx="1"/>"#);

/// `git-branch`.
pub const GIT_BRANCH: Icon = lucide!("git-branch", r#"<path d="M15 6a9 9 0 0 0-9 9V3"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/>"#);

/// `git-branch-minus`.
pub const GIT_BRANCH_MINUS: Icon = lucide!("git-branch-minus", r#"<path d="M15 6a9 9 0 0 0-9 9V3"/><path d="M21 18h-6"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/>"#);

/// `git-branch-plus`.
pub const GIT_BRANCH_PLUS: Icon = lucide!("git-branch-plus", r#"<path d="M6 3v12"/><path d="M18 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/><path d="M6 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/><path d="M15 6a9 9 0 0 0-9 9"/><path d="M18 15v6"/><path d="M21 18h-6"/>"#);

/// `git-commit-horizontal`.
pub const GIT_COMMIT_HORIZONTAL: Icon = lucide!("git-commit-horizontal", r#"<circle cx="12" cy="12" r="3"/><line x1="3" x2="9" y1="12" y2="12"/><line x1="15" x2="21" y1="12" y2="12"/>"#);

/// `git-commit-vertical`.
pub const GIT_COMMIT_VERTICAL: Icon = lucide!("git-commit-vertical", r#"<path d="M12 3v6"/><circle cx="12" cy="12" r="3"/><path d="M12 15v6"/>"#);

/// `git-compare`.
pub const GIT_COMPARE: Icon = lucide!("git-compare", r#"<circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M13 6h3a2 2 0 0 1 2 2v7"/><path d="M11 18H8a2 2 0 0 1-2-2V9"/>"#);

/// `git-compare-arrows`.
pub const GIT_COMPARE_ARROWS: Icon = lucide!("git-compare-arrows", r#"<circle cx="5" cy="6" r="3"/><path d="M12 6h5a2 2 0 0 1 2 2v7"/><path d="m15 9-3-3 3-3"/><circle cx="19" cy="18" r="3"/><path d="M12 18H7a2 2 0 0 1-2-2V9"/><path d="m9 15 3 3-3 3"/>"#);

/// `git-fork`.
pub const GIT_FORK: Icon = lucide!("git-fork", r#"<circle cx="12" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/><path d="M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9"/><path d="M12 12v3"/>"#);

/// `git-graph`.
pub const GIT_GRAPH: Icon = lucide!("git-graph", r#"<circle cx="5" cy="6" r="3"/><path d="M5 9v6"/><circle cx="5" cy="18" r="3"/><path d="M12 3v18"/><circle cx="19" cy="6" r="3"/><path d="M16 15.7A9 9 0 0 0 19 9"/>"#);

/// `git-merge`.
pub const GIT_MERGE: Icon = lucide!("git-merge", r#"<circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M6 21V9a9 9 0 0 0 9 9"/>"#);

/// `git-merge-conflict`.
pub const GIT_MERGE_CONFLICT: Icon = lucide!("git-merge-conflict", r#"<path d="M12 6h4a2 2 0 0 1 2 2v7"/><path d="M6 12v9"/><path d="m8.5 3.5-5 5"/><path d="m8.5 8.5-5-5"/><circle cx="18" cy="18" r="3"/>"#);

/// `git-pull-request`.
pub const GIT_PULL_REQUEST: Icon = lucide!("git-pull-request", r#"<circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M13 6h3a2 2 0 0 1 2 2v7"/><line x1="6" x2="6" y1="9" y2="21"/>"#);

/// `git-pull-request-arrow`.
pub const GIT_PULL_REQUEST_ARROW: Icon = lucide!("git-pull-request-arrow", r#"<circle cx="5" cy="6" r="3"/><path d="M5 9v12"/><circle cx="19" cy="18" r="3"/><path d="m15 9-3-3 3-3"/><path d="M12 6h5a2 2 0 0 1 2 2v7"/>"#);

/// `git-pull-request-closed`.
pub const GIT_PULL_REQUEST_CLOSED: Icon = lucide!("git-pull-request-closed", r#"<path d="m15.5 3.5 5 5"/><path d="m15.5 8.5 5-5"/><path d="M18 11.62V15"/><path d="M6 9v12"/><circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/>"#);

/// `git-pull-request-create`.
pub const GIT_PULL_REQUEST_CREATE: Icon = lucide!("git-pull-request-create", r#"<circle cx="6" cy="6" r="3"/><path d="M6 9v12"/><path d="M13 6h3a2 2 0 0 1 2 2v3"/><path d="M18 15v6"/><path d="M21 18h-6"/>"#);

/// `git-pull-request-create-arrow`.
pub const GIT_PULL_REQUEST_CREATE_ARROW: Icon = lucide!("git-pull-request-create-arrow", r#"<circle cx="5" cy="6" r="3"/><path d="M5 9v12"/><path d="m15 9-3-3 3-3"/><path d="M12 6h5a2 2 0 0 1 2 2v3"/><path d="M19 15v6"/><path d="M22 18h-6"/>"#);

/// `git-pull-request-draft`.
pub const GIT_PULL_REQUEST_DRAFT: Icon = lucide!("git-pull-request-draft", r#"<circle cx="18" cy="18" r="3"/><circle cx="6" cy="6" r="3"/><path d="M18 6V5"/><path d="M18 11v-1"/><line x1="6" x2="6" y1="9" y2="21"/>"#);

/// `glass-water`.
pub const GLASS_WATER: Icon = lucide!("glass-water", r#"<path d="M5.116 4.104A1 1 0 0 1 6.11 3h11.78a1 1 0 0 1 .994 1.105L17.19 20.21A2 2 0 0 1 15.2 22H8.8a2 2 0 0 1-2-1.79z"/><path d="M6 12a5 5 0 0 1 6 0 5 5 0 0 0 6 0"/>"#);

/// `glasses`.
pub const GLASSES: Icon = lucide!("glasses", r#"<circle cx="6" cy="15" r="4"/><circle cx="18" cy="15" r="4"/><path d="M14 15a2 2 0 0 0-2-2 2 2 0 0 0-2 2"/><path d="M2.5 13 5 7c.7-1.3 1.4-2 3-2"/><path d="M21.5 13 19 7c-.7-1.3-1.5-2-3-2"/>"#);

/// `globe`.
pub const GLOBE: Icon = lucide!("globe", r#"<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>"#);

/// `globe-ban`.
pub const GLOBE_BAN: Icon = lucide!("globe-ban", r#"<path d="M12 22a14.48 14.48 0 01-4.008-10C7.993 9.136 9 5.258 11 2.05A10 10 0 0012 22"/><path d="m15.172 3.172 5.656 5.656"/><path d="M15.86 14a14.5 14.5 0 01-3.859 8 10 10 0 009.949-9"/><path d="M2 12h10.5"/><circle cx="18" cy="6" r="4"/>"#);

/// `globe-check`.
pub const GLOBE_CHECK: Icon = lucide!("globe-check", r#"<path d="m15 6 2 2 4-4"/><path d="M2 12h20A10 10 0 1 1 12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 4-10"/>"#);

/// `globe-code`.
pub const GLOBE_CODE: Icon = lucide!("globe-code", r#"<path d="M15.5 10 13 7.5 15.5 5"/><path d="M15.861 14A14.5 14.5 0 0112 22a14.48 14.48 0 010-20 10 10 0 109.888 11.5"/><path d="M19.5 5 22 7.5 19.5 10"/><path d="M2 12h8.5"/>"#);

/// `globe-lock`.
pub const GLOBE_LOCK: Icon = lucide!("globe-lock", r#"<path d="M15.686 15A14.5 14.5 0 0 1 12 22a14.5 14.5 0 0 1 0-20 10 10 0 1 0 9.542 13"/><path d="M2 12h8.5"/><path d="M20 6V4a2 2 0 1 0-4 0v2"/><rect width="8" height="5" x="14" y="6" rx="1"/>"#);

/// `globe-off`.
pub const GLOBE_OFF: Icon = lucide!("globe-off", r#"<path d="M10.114 4.462A14.5 14.5 0 0 1 12 2a10 10 0 0 1 9.313 13.643"/><path d="M15.557 15.556A14.5 14.5 0 0 1 12 22 10 10 0 0 1 4.929 4.929"/><path d="M15.892 10.234A14.5 14.5 0 0 0 12 2a10 10 0 0 0-3.643.687"/><path d="M17.656 12H22"/><path d="M19.071 19.071A10 10 0 0 1 12 22 14.5 14.5 0 0 1 8.44 8.45"/><path d="M2 12h10"/><path d="m2 2 20 20"/>"#);

/// `globe-plus`.
pub const GLOBE_PLUS: Icon = lucide!("globe-plus", r#"<path d="M15.929 10.563A14.5 14.5 0 0112 22a1 1 0 010-20 15 15 0 01.68.768"/><path d="M19 2v6"/><path d="M21.92 10.734A10 10 0 0112 22a14.5 14.5 0 010-20 10 10 0 011.266.08"/><path d="M22 12H2"/><path d="M22 5h-6"/>"#);

/// `globe-x`.
pub const GLOBE_X: Icon = lucide!("globe-x", r#"<path d="m16 3 5 5"/><path d="M2 12h20A10 10 0 1 1 12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 4-10"/><path d="m21 3-5 5"/>"#);

/// `goal`.
pub const GOAL: Icon = lucide!("goal", r#"<path d="M12 13V2l8 4-8 4"/><path d="M20.561 10.222a9 9 0 1 1-12.55-5.29"/><path d="M8.002 9.997a5 5 0 1 0 8.9 2.02"/>"#);

/// `gpu`.
pub const GPU: Icon = lucide!("gpu", r#"<path d="M2 17h18a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2H2"/><path d="M2 21V3"/><path d="M7 17v3a1 1 0 0 0 1 1h5a1 1 0 0 0 1-1v-3"/><circle cx="16" cy="11" r="2"/><circle cx="8" cy="11" r="2"/>"#);

/// `graduation-cap`.
pub const GRADUATION_CAP: Icon = lucide!("graduation-cap", r#"<path d="M21.42 10.922a1 1 0 0 0-.019-1.838L12.83 5.18a2 2 0 0 0-1.66 0L2.6 9.08a1 1 0 0 0 0 1.832l8.57 3.908a2 2 0 0 0 1.66 0z"/><path d="M22 10v6"/><path d="M6 12.5V16a6 3 0 0 0 12 0v-3.5"/>"#);

/// `grape`.
pub const GRAPE: Icon = lucide!("grape", r#"<path d="M22 5V2l-5.89 5.89"/><circle cx="16.6" cy="15.89" r="3"/><circle cx="8.11" cy="7.4" r="3"/><circle cx="12.35" cy="11.65" r="3"/><circle cx="13.91" cy="5.85" r="3"/><circle cx="18.15" cy="10.09" r="3"/><circle cx="6.56" cy="13.2" r="3"/><circle cx="10.8" cy="17.44" r="3"/><circle cx="5" cy="19" r="3"/>"#);

/// `graph-hub`.
pub const GRAPH_HUB: Icon = lucide!("graph-hub", r#"<circle cx="12" cy="7" r="4"/><circle cx="4" cy="19" r="2"/><circle cx="12" cy="19" r="2"/><circle cx="20" cy="19" r="2"/><path d="M8 7a4 4 0 0 0-4 4v6"/><path d="M12 11v6"/><path d="M16 7a4 4 0 0 1 4 4v6"/>"#);

/// `graph-star`.
pub const GRAPH_STAR: Icon = lucide!("graph-star", r#"<path d="m12.342 9.316 1.764-3.527"/><path d="m12.59 14.545 2.35 3.759"/><path d="m13.982 12.332 4.03.447"/><path d="M5.414 17.586 8.88 14.12"/><path d="M9.025 9.742 5.317 5.505"/><circle cx="11" cy="12" r="3"/><circle cx="15" cy="4" r="2"/><circle cx="16" cy="20" r="2"/><circle cx="20" cy="13" r="2"/><circle cx="4" cy="19" r="2"/><circle cx="4" cy="4" r="2"/>"#);

/// `grid-2x2`.
pub const GRID_2X2: Icon = lucide!("grid-2x2", r#"<path d="M12 3v18"/><path d="M3 12h18"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `grid-2x2-check`.
pub const GRID_2X2_CHECK: Icon = lucide!("grid-2x2-check", r#"<path d="M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3"/><path d="m16 19 2 2 4-4"/>"#);

/// `grid-2x2-plus`.
pub const GRID_2X2_PLUS: Icon = lucide!("grid-2x2-plus", r#"<path d="M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3"/><path d="M16 19h6"/><path d="M19 22v-6"/>"#);

/// `grid-2x2-x`.
pub const GRID_2X2_X: Icon = lucide!("grid-2x2-x", r#"<path d="M12 3v17a1 1 0 0 1-1 1H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v6a1 1 0 0 1-1 1H3"/><path d="m16.5 16.5 5 5"/><path d="m16.5 21.5 5-5"/>"#);

/// `grid-3x2`.
pub const GRID_3X2: Icon = lucide!("grid-3x2", r#"<path d="M15 3v18"/><path d="M3 12h18"/><path d="M9 3v18"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `grid-3x3`.
pub const GRID_3X3: Icon = lucide!("grid-3x3", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/><path d="M9 3v18"/><path d="M15 3v18"/>"#);

/// `grid-3x3-off`.
pub const GRID_3X3_OFF: Icon = lucide!("grid-3x3-off", r#"<path d="M15 9.344V3"/><path d="m2 2 20 20"/><path d="M20.414 20.414A2 2 0 0119 21H5a2 2 0 01-2-2V5a2 2 0 01.586-1.414"/><path d="M20.656 15H21"/><path d="M21 9h-6.344"/><path d="M3 15h12v6"/><path d="M3 9h6v12"/><path d="M8.656 3H19a2 2 0 012 2v10.343"/><path d="M9 3v.344"/>"#);

/// `grip`.
pub const GRIP: Icon = lucide!("grip", r#"<circle cx="12" cy="5" r="1"/><circle cx="19" cy="5" r="1"/><circle cx="5" cy="5" r="1"/><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/><circle cx="12" cy="19" r="1"/><circle cx="19" cy="19" r="1"/><circle cx="5" cy="19" r="1"/>"#);

/// `grip-horizontal`.
pub const GRIP_HORIZONTAL: Icon = lucide!("grip-horizontal", r#"<circle cx="12" cy="9" r="1"/><circle cx="19" cy="9" r="1"/><circle cx="5" cy="9" r="1"/><circle cx="12" cy="15" r="1"/><circle cx="19" cy="15" r="1"/><circle cx="5" cy="15" r="1"/>"#);

/// `grip-vertical`.
pub const GRIP_VERTICAL: Icon = lucide!("grip-vertical", r#"<circle cx="9" cy="12" r="1"/><circle cx="9" cy="5" r="1"/><circle cx="9" cy="19" r="1"/><circle cx="15" cy="12" r="1"/><circle cx="15" cy="5" r="1"/><circle cx="15" cy="19" r="1"/>"#);

/// `groceries`.
pub const GROCERIES: Icon = lucide!("groceries", r#"<path d="M10 15h4"/><path d="M19.424 11.095a2.5 2.5 0 00.471-2.309 2 2 0 00.424-3.286 2 2 0 00-2.143-3.322 2 2 0 00-.676.502 2 2 0 00-3.287.424 2.5 2.5 0 00-3.189 2.06A3 3 0 0012 11l4-4"/><path d="M4 12.006A1 1 0 014.994 11H19a1 1 0 011 1v7a2 2 0 01-2 2H6a2 2 0 01-2-2z"/><path d="M7 11a4 4 0 01-4-4V5a1 1 0 011-1h2a4 4 0 013.584 2.222"/>"#);

/// `group`.
pub const GROUP: Icon = lucide!("group", r#"<path d="M3 7V5c0-1.1.9-2 2-2h2"/><path d="M17 3h2c1.1 0 2 .9 2 2v2"/><path d="M21 17v2c0 1.1-.9 2-2 2h-2"/><path d="M7 21H5c-1.1 0-2-.9-2-2v-2"/><rect width="7" height="5" x="7" y="7" rx="1"/><rect width="7" height="5" x="10" y="12" rx="1"/>"#);

/// `guitar`.
pub const GUITAR: Icon = lucide!("guitar", r#"<path d="m11.9 12.1 4.514-4.514"/><path d="M20.1 2.3a1 1 0 0 0-1.4 0l-1.114 1.114A2 2 0 0 0 17 4.828v1.344a2 2 0 0 1-.586 1.414A2 2 0 0 1 17.828 7h1.344a2 2 0 0 0 1.414-.586L21.7 5.3a1 1 0 0 0 0-1.4z"/><path d="m6 16 2 2"/><path d="M8.23 9.85A3 3 0 0 1 11 8a5 5 0 0 1 5 5 3 3 0 0 1-1.85 2.77l-.92.38A2 2 0 0 0 12 18a4 4 0 0 1-4 4 6 6 0 0 1-6-6 4 4 0 0 1 4-4 2 2 0 0 0 1.85-1.23z"/>"#);

/// `guitar-electric`.
pub const GUITAR_ELECTRIC: Icon = lucide!("guitar-electric", r#"<path d="m12.172 11.828 4.242-4.242"/><path d="M16.414 7.586A2 2 0 0017 6.172V5a2 2 0 011.177-1.823L20.59 2.09a1 1 0 011.32 1.32l-1.088 2.412A2 2 0 0119 7h-1a2 2 0 00-1.586.586"/><path d="m6 16 2 2"/><path d="M8.23 9.85a3 3 0 011.28-1.454 1 1 0 011.496.815 4 4 0 003.783 3.783 1 1 0 01.815 1.496 3 3 0 01-1.454 1.28l-.92.38A2 2 0 0012 18a4 4 0 01-4 4 6 6 0 01-6-6 4 4 0 014-4 2 2 0 001.85-1.23z"/>"#);

/// `half-note`.
pub const HALF_NOTE: Icon = lucide!("half-note", r#"<circle cx="12" cy="18" r="4"/><path d="M16 18V2"/>"#);

/// `ham`.
pub const HAM: Icon = lucide!("ham", r#"<path d="M13.144 21.144A7.274 10.445 45 1 0 2.856 10.856"/><path d="M13.144 21.144A7.274 4.365 45 0 0 2.856 10.856a7.274 4.365 45 0 0 10.288 10.288"/><path d="M16.565 10.435 18.6 8.4a2.501 2.501 0 1 0 1.65-4.65 2.5 2.5 0 1 0-4.66 1.66l-2.024 2.025"/><path d="m8.5 16.5-1-1"/>"#);

/// `hamburger`.
pub const HAMBURGER: Icon = lucide!("hamburger", r#"<path d="M12 16H4a2 2 0 1 1 0-4h16a2 2 0 1 1 0 4h-4.25"/><path d="M5 12a2 2 0 0 1-2-2 9 7 0 0 1 18 0 2 2 0 0 1-2 2"/><path d="M5 16a2 2 0 0 0-2 2 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 2 2 0 0 0-2-2q0 0 0 0"/><path d="m6.67 12 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2"/>"#);

/// `hammer`.
pub const HAMMER: Icon = lucide!("hammer", r#"<path d="m15 12-9.373 9.373a1 1 0 0 1-3.001-3L12 9"/><path d="m18 15 4-4"/><path d="m21.5 11.5-1.914-1.914A2 2 0 0 1 19 8.172v-.344a2 2 0 0 0-.586-1.414l-1.657-1.657A6 6 0 0 0 12.516 3H9l1.243 1.243A6 6 0 0 1 12 8.485V10l2 2h1.172a2 2 0 0 1 1.414.586L18.5 14.5"/>"#);

/// `hand`.
pub const HAND: Icon = lucide!("hand", r#"<path d="M18 11V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2"/><path d="M14 10V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2"/><path d="M10 10.5V6a2 2 0 0 0-2-2a2 2 0 0 0-2 2v8"/><path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"/>"#);

/// `hand-coins`.
pub const HAND_COINS: Icon = lucide!("hand-coins", r#"<path d="M11 15h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 17"/><path d="m7 21 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9"/><path d="m2 16 6 6"/><circle cx="16" cy="9" r="2.9"/><circle cx="6" cy="5" r="3"/>"#);

/// `hand-fist`.
pub const HAND_FIST: Icon = lucide!("hand-fist", r#"<path d="M12.035 17.012a3 3 0 0 0-3-3l-.311-.002a.72.72 0 0 1-.505-1.229l1.195-1.195A2 2 0 0 1 10.828 11H12a2 2 0 0 0 0-4H9.243a3 3 0 0 0-2.122.879l-2.707 2.707A4.83 4.83 0 0 0 3 14a8 8 0 0 0 8 8h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v2a2 2 0 1 0 4 0"/><path d="M13.888 9.662A2 2 0 0 0 17 8V5A2 2 0 1 0 13 5"/><path d="M9 5A2 2 0 1 0 5 5V10"/><path d="M9 7V4A2 2 0 1 1 13 4V7.268"/>"#);

/// `hand-grab`.
pub const HAND_GRAB: Icon = lucide!("hand-grab", r#"<path d="M18 11.5V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1.4"/><path d="M14 10V8a2 2 0 0 0-2-2a2 2 0 0 0-2 2v2"/><path d="M10 9.9V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v5"/><path d="M6 14a2 2 0 0 0-2-2a2 2 0 0 0-2 2"/><path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-4a8 8 0 0 1-8-8 2 2 0 1 1 4 0"/>"#);

/// `hand-heart`.
pub const HAND_HEART: Icon = lucide!("hand-heart", r#"<path d="M11 14h2a2 2 0 0 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 16"/><path d="m14.45 13.39 5.05-4.694C20.196 8 21 6.85 21 5.75a2.75 2.75 0 0 0-4.797-1.837.276.276 0 0 1-.406 0A2.75 2.75 0 0 0 11 5.75c0 1.2.802 2.248 1.5 2.946L16 11.95"/><path d="m2 15 6 6"/><path d="m7 20 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a1 1 0 0 0-2.75-2.91"/>"#);

/// `hand-helping`.
pub const HAND_HELPING: Icon = lucide!("hand-helping", r#"<path d="M11 12h2a2 2 0 1 0 0-4h-3c-.6 0-1.1.2-1.4.6L3 14"/><path d="m7 18 1.6-1.4c.3-.4.8-.6 1.4-.6h4c1.1 0 2.1-.4 2.8-1.2l4.6-4.4a2 2 0 0 0-2.75-2.91l-4.2 3.9"/><path d="m2 13 6 6"/>"#);

/// `hand-metal`.
pub const HAND_METAL: Icon = lucide!("hand-metal", r#"<path d="M18 12.5V10a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1.4"/><path d="M14 11V9a2 2 0 1 0-4 0v2"/><path d="M10 10.5V5a2 2 0 1 0-4 0v9"/><path d="m7 15-1.76-1.76a2 2 0 0 0-2.83 2.82l3.6 3.6C7.5 21.14 9.2 22 12 22h2a8 8 0 0 0 8-8V7a2 2 0 1 0-4 0v5"/>"#);

/// `hand-platter`.
pub const HAND_PLATTER: Icon = lucide!("hand-platter", r#"<path d="M12 3V2"/><path d="m15.4 17.4 3.2-2.8a2 2 0 1 1 2.8 2.9l-3.6 3.3c-.7.8-1.7 1.2-2.8 1.2h-4c-1.1 0-2.1-.4-2.8-1.2l-1.302-1.464A1 1 0 0 0 6.151 19H5"/><path d="M2 14h12a2 2 0 0 1 0 4h-2"/><path d="M4 10h16"/><path d="M5 10a7 7 0 0 1 14 0"/><path d="M5 14v6a1 1 0 0 1-1 1H2"/>"#);

/// `handbag`.
pub const HANDBAG: Icon = lucide!("handbag", r#"<path d="M2.048 18.566A2 2 0 0 0 4 21h16a2 2 0 0 0 1.952-2.434l-2-9A2 2 0 0 0 18 8H6a2 2 0 0 0-1.952 1.566z"/><path d="M8 11V6a4 4 0 0 1 8 0v5"/>"#);

/// `handle-bottom-right`.
pub const HANDLE_BOTTOM_RIGHT: Icon = lucide!("handle-bottom-right", r#"<path d="M21 8 8 21"/><path d="m21 15-6 6"/>"#);

/// `handshake`.
pub const HANDSHAKE: Icon = lucide!("handshake", r#"<path d="m11 17 2 2a1 1 0 1 0 3-3"/><path d="m14 14 2.5 2.5a1 1 0 1 0 3-3l-3.88-3.88a3 3 0 0 0-4.24 0l-.88.88a1 1 0 1 1-3-3l2.81-2.81a5.79 5.79 0 0 1 7.06-.87l.47.28a2 2 0 0 0 1.42.25L21 4"/><path d="m21 3 1 11h-2"/><path d="M3 3 2 14l6.5 6.5a1 1 0 1 0 3-3"/><path d="M3 4h8"/>"#);

/// `hard-drive`.
pub const HARD_DRIVE: Icon = lucide!("hard-drive", r#"<path d="M10 16h.01"/><path d="M2.212 11.577a2 2 0 0 0-.212.896V18a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5.527a2 2 0 0 0-.212-.896L18.55 5.11A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/><path d="M21.946 12.013H2.054"/><path d="M6 16h.01"/>"#);

/// `hard-drive-download`.
pub const HARD_DRIVE_DOWNLOAD: Icon = lucide!("hard-drive-download", r#"<path d="M12 2v8"/><path d="m16 6-4 4-4-4"/><rect width="20" height="8" x="2" y="14" rx="2"/><path d="M6 18h.01"/><path d="M10 18h.01"/>"#);

/// `hard-drive-upload`.
pub const HARD_DRIVE_UPLOAD: Icon = lucide!("hard-drive-upload", r#"<path d="m16 6-4-4-4 4"/><path d="M12 2v8"/><rect width="20" height="8" x="2" y="14" rx="2"/><path d="M6 18h.01"/><path d="M10 18h.01"/>"#);

/// `hard-hat`.
pub const HARD_HAT: Icon = lucide!("hard-hat", r#"<path d="M10 10V5a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1v5"/><path d="M14 6a6 6 0 0 1 6 6v3"/><path d="M4 15v-3a6 6 0 0 1 6-6"/><rect x="2" y="15" width="20" height="4" rx="1"/>"#);

/// `hash`.
pub const HASH: Icon = lucide!("hash", r#"<line x1="4" x2="20" y1="9" y2="9"/><line x1="4" x2="20" y1="15" y2="15"/><line x1="10" x2="8" y1="3" y2="21"/><line x1="16" x2="14" y1="3" y2="21"/>"#);

/// `hat-glasses`.
pub const HAT_GLASSES: Icon = lucide!("hat-glasses", r#"<path d="M14 18a2 2 0 0 0-4 0"/><path d="m19 11-2.11-6.657a2 2 0 0 0-2.752-1.148l-1.276.61A2 2 0 0 1 12 4H8.5a2 2 0 0 0-1.925 1.456L5 11"/><path d="M2 11h20"/><circle cx="17" cy="18" r="3"/><circle cx="7" cy="18" r="3"/>"#);

/// `haze`.
pub const HAZE: Icon = lucide!("haze", r#"<path d="m5.2 6.2 1.4 1.4"/><path d="M2 13h2"/><path d="M20 13h2"/><path d="m17.4 7.6 1.4-1.4"/><path d="M22 17H2"/><path d="M22 21H2"/><path d="M16 13a4 4 0 0 0-8 0"/><path d="M12 5V2.5"/>"#);

/// `hd`.
pub const HD: Icon = lucide!("hd", r#"<path d="M10 12H6"/><path d="M10 15V9"/><path d="M14 14.5a.5.5 0 0 0 .5.5h1a2.5 2.5 0 0 0 2.5-2.5v-1A2.5 2.5 0 0 0 15.5 9h-1a.5.5 0 0 0-.5.5z"/><path d="M6 15V9"/><rect x="2" y="5" width="20" height="14" rx="2"/>"#);

/// `hdmi-port`.
pub const HDMI_PORT: Icon = lucide!("hdmi-port", r#"<path d="M22 9a1 1 0 00-1-1H3a1 1 0 00-1 1v4a1 1 0 001 1h.5a2 2 0 011.6.8l.3.4A2 2 0 007 16h10a2 2 0 001.6-.8l.3-.4a2 2 0 011.6-.8h.5a1 1 0 001-1z"/><path d="M8 12h8"/>"#);

/// `heading-1`.
pub const HEADING_1: Icon = lucide!("heading-1", r#"<path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><path d="m17 12 3-2v8"/>"#);

/// `heading-2`.
pub const HEADING_2: Icon = lucide!("heading-2", r#"<path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><path d="M21 18h-4c0-4 4-3 4-6 0-1.5-2-2.5-4-1"/>"#);

/// `heading-3`.
pub const HEADING_3: Icon = lucide!("heading-3", r#"<path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><path d="M17.5 10.5c1.7-1 3.5 0 3.5 1.5a2 2 0 0 1-2 2"/><path d="M17 17.5c2 1.5 4 .3 4-1.5a2 2 0 0 0-2-2"/>"#);

/// `heading-4`.
pub const HEADING_4: Icon = lucide!("heading-4", r#"<path d="M12 18V6"/><path d="M17 10v3a1 1 0 0 0 1 1h3"/><path d="M21 10v8"/><path d="M4 12h8"/><path d="M4 18V6"/>"#);

/// `heading-5`.
pub const HEADING_5: Icon = lucide!("heading-5", r#"<path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><path d="M17 13v-3h4"/><path d="M17 17.7c.4.2.8.3 1.3.3 1.5 0 2.7-1.1 2.7-2.5S19.8 13 18.3 13H17"/>"#);

/// `heading-6`.
pub const HEADING_6: Icon = lucide!("heading-6", r#"<path d="M4 12h8"/><path d="M4 18V6"/><path d="M12 18V6"/><circle cx="19" cy="16" r="2"/><path d="M20 10c-2 2-3 3.5-3 6"/>"#);

/// `headphone-off`.
pub const HEADPHONE_OFF: Icon = lucide!("headphone-off", r#"<path d="M21 14h-1.343"/><path d="M9.128 3.47A9 9 0 0 1 21 12v3.343"/><path d="m2 2 20 20"/><path d="M20.414 20.414A2 2 0 0 1 19 21h-1a2 2 0 0 1-2-2v-3"/><path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 2.636-6.364"/>"#);

/// `headphones`.
pub const HEADPHONES: Icon = lucide!("headphones", r#"<path d="M3 14h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-7a9 9 0 0 1 18 0v7a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3"/>"#);

/// `headset`.
pub const HEADSET: Icon = lucide!("headset", r#"<path d="M3 11h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5Zm0 0a9 9 0 1 1 18 0m0 0v5a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3Z"/><path d="M21 16v2a4 4 0 0 1-4 4h-5"/>"#);

/// `heart`.
pub const HEART: Icon = lucide!("heart", r#"<path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/>"#);

/// `heart-crack`.
pub const HEART_CRACK: Icon = lucide!("heart-crack", r#"<path d="M12.409 5.824c-.702.792-1.15 1.496-1.415 2.166l2.153 2.156a.5.5 0 0 1 0 .707l-2.293 2.293a.5.5 0 0 0 0 .707L12 15"/><path d="M13.508 20.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.591-3.677.6.6 0 0 0 .818.001A5.5 5.5 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5z"/>"#);

/// `heart-handshake`.
pub const HEART_HANDSHAKE: Icon = lucide!("heart-handshake", r#"<path d="M19.414 14.414C21 12.828 22 11.5 22 9.5a5.5 5.5 0 0 0-9.591-3.676.6.6 0 0 1-.818.001A5.5 5.5 0 0 0 2 9.5c0 2.3 1.5 4 3 5.5l5.535 5.362a2 2 0 0 0 2.879.052 2.12 2.12 0 0 0-.004-3 2.124 2.124 0 1 0 3-3 2.124 2.124 0 0 0 3.004 0 2 2 0 0 0 0-2.828l-1.881-1.882a2.41 2.41 0 0 0-3.409 0l-1.71 1.71a2 2 0 0 1-2.828 0 2 2 0 0 1 0-2.828l2.823-2.762"/>"#);

/// `heart-minus`.
pub const HEART_MINUS: Icon = lucide!("heart-minus", r#"<path d="m14.876 18.99-1.368 1.323a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5a5.2 5.2 0 0 1-.244 1.572"/><path d="M15 15h6"/>"#);

/// `heart-off`.
pub const HEART_OFF: Icon = lucide!("heart-off", r#"<path d="M10.5 4.893a5.5 5.5 0 0 1 1.091.931.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 1.872-1.002 3.356-2.187 4.655"/><path d="m16.967 16.967-3.459 3.346a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 2.747-4.761"/><path d="m2 2 20 20"/>"#);

/// `heart-plus`.
pub const HEART_PLUS: Icon = lucide!("heart-plus", r#"<path d="m14.479 19.374-.971.939a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5a5.2 5.2 0 0 1-.219 1.49"/><path d="M15 15h6"/><path d="M18 12v6"/>"#);

/// `heart-pulse`.
pub const HEART_PULSE: Icon = lucide!("heart-pulse", r#"<path d="M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5"/><path d="M3.22 13H9.5l.5-1 2 4.5 2-7 1.5 3.5h5.27"/>"#);

/// `heart-x`.
pub const HEART_X: Icon = lucide!("heart-x", r#"<path d="m15.5 12.5 5 5"/><path d="m20.5 12.5-5 5"/><path d="M21.955 8.774a5.5 5.5 0 0 0-9.546-2.95.6.6 0 0 1-.818 0A5.5 5.5 0 0 0 2 9.5c0 2.3 1.5 4 3 5.5l5.508 5.332a2 2 0 0 0 2.57.352"/>"#);

/// `heater`.
pub const HEATER: Icon = lucide!("heater", r#"<path d="M11 8c2-3-2-3 0-6"/><path d="M15.5 8c2-3-2-3 0-6"/><path d="M6 10h.01"/><path d="M6 14h.01"/><path d="M10 16v-4"/><path d="M14 16v-4"/><path d="M18 16v-4"/><path d="M20 6a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3"/><path d="M5 20v2"/><path d="M19 20v2"/>"#);

/// `helicopter`.
pub const HELICOPTER: Icon = lucide!("helicopter", r#"<path d="M11 17v4"/><path d="M14 3v8a2 2 0 0 0 2 2h5.865"/><path d="M17 17v4"/><path d="M18 17a4 4 0 0 0 4-4 8 6 0 0 0-8-6 6 5 0 0 0-6 5v3a2 2 0 0 0 2 2z"/><path d="M2 10v5"/><path d="M6 3h16"/><path d="M7 21h14"/><path d="M8 13H2"/>"#);

/// `hexagon`.
pub const HEXAGON: Icon = lucide!("hexagon", r#"<path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>"#);

/// `highlighter`.
pub const HIGHLIGHTER: Icon = lucide!("highlighter", r#"<path d="m9 11-6 6v3h9l3-3"/><path d="m22 12-4.6 4.6a2 2 0 0 1-2.8 0l-5.2-5.2a2 2 0 0 1 0-2.8L14 4"/>"#);

/// `hiking-stick`.
pub const HIKING_STICK: Icon = lucide!("hiking-stick", r#"<path d="M13.5 10.5 2 22"/><path d="M16.352 11.648a1.205 1.205 0 01-1.704 0l-2.296-2.296a1.205 1.205 0 010-1.704l.72-.72a2 2 0 011.022-.546l.599-.12a2 2 0 001.569-1.57l.12-.598a2 2 0 01.546-1.022l.72-.72a1.205 1.205 0 011.704 0l2.296 2.296a1.205 1.205 0 010 1.704l-6.02 6.02a1 1 0 103 3l.201-.201A7.4 7.4 0 0021 9.93V7"/><path d="m6 21-3-3"/>"#);

/// `hockey-stick`.
pub const HOCKEY_STICK: Icon = lucide!("hockey-stick", r#"<path d="M11 4v4c0 1.1-2.025 2-4.5 2S2 9.1 2 8V4"/><path d="M18.21 3.106a2 2 0 013.579 1.789l-6.895 13.789A6 6 0 019.529 22H4.5a1 1 0 010-5h5.528a2 2 0 001.788-1.105z"/><path d="m18.59 11.295-3.578-1.789"/><path d="m20.79 6.895-3.578-1.789"/><path d="M7 17v5"/><ellipse cx="6.5" cy="4" rx="4.5" ry="2"/>"#);

/// `hop`.
pub const HOP: Icon = lucide!("hop", r#"<path d="M10.82 16.12c1.69.6 3.91.79 5.18.85.55.03 1-.42.97-.97-.06-1.27-.26-3.5-.85-5.18"/><path d="M11.5 6.5c1.64 0 5-.38 6.71-1.07.52-.2.55-.82.12-1.17A10 10 0 0 0 4.26 18.33c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.88.88 0 0 0 .73-.74c.3-2.14-.15-3.5-.61-4.88"/><path d="M15.62 16.95c.2.85.62 2.76.5 4.28a.77.77 0 0 1-.9.7 16.64 16.64 0 0 1-4.08-1.36"/><path d="M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .96-.96 17.68 17.68 0 0 0-.9-4.87"/><path d="M16.94 15.62c.86.2 2.77.62 4.29.5a.77.77 0 0 0 .7-.9 16.64 16.64 0 0 0-1.36-4.08"/><path d="M17.99 5.52a20.82 20.82 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-2.33.2-5.3-.32-8.27-1.57"/><path d="M4.93 4.93 3 3a.7.7 0 0 1 0-1"/><path d="M9.58 12.18c1.24 2.98 1.77 5.95 1.57 8.28a.8.8 0 0 1-1.13.68 20.82 20.82 0 0 1-4.5-3.15"/>"#);

/// `hop-off`.
pub const HOP_OFF: Icon = lucide!("hop-off", r#"<path d="M10.82 16.12c1.69.6 3.91.79 5.18.85.28.01.53-.09.7-.27"/><path d="M11.14 20.57c.52.24 2.44 1.12 4.08 1.37.46.06.86-.25.9-.71.12-1.52-.3-3.43-.5-4.28"/><path d="M16.13 21.05c1.65.63 3.68.84 4.87.91a.9.9 0 0 0 .7-.26"/><path d="M17.99 5.52a20.83 20.83 0 0 1 3.15 4.5.8.8 0 0 1-.68 1.13c-1.17.1-2.5.02-3.9-.25"/><path d="M20.57 11.14c.24.52 1.12 2.44 1.37 4.08.04.3-.08.59-.31.75"/><path d="M4.93 4.93a10 10 0 0 0-.67 13.4c.35.43.96.4 1.17-.12.69-1.71 1.07-5.07 1.07-6.71 1.34.45 3.1.9 4.88.62a.85.85 0 0 0 .48-.24"/><path d="M5.52 17.99c1.05.95 2.91 2.42 4.5 3.15a.8.8 0 0 0 1.13-.68c.2-2.34-.33-5.3-1.57-8.28"/><path d="M8.35 2.68a10 10 0 0 1 9.98 1.58c.43.35.4.96-.12 1.17-1.5.6-4.3.98-6.07 1.05"/><path d="m2 2 20 20"/>"#);

/// `hospital`.
pub const HOSPITAL: Icon = lucide!("hospital", r#"<path d="M12 7v4"/><path d="M14 21v-3a2 2 0 0 0-4 0v3"/><path d="M14 9h-4"/><path d="M18 11h2a2 2 0 0 1 2 2v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-9a2 2 0 0 1 2-2h2"/><path d="M18 21V5a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16"/>"#);

/// `hot-air-balloon`.
pub const HOT_AIR_BALLOON: Icon = lucide!("hot-air-balloon", r#"<path d="M11.5 18C10 15 9 12 9 9a3 7 0 0 1 6 0c0 3-1 6-2.5 9"/><path d="M15 18c3-3 5-5 5-9A8 7 0 0 0 4 9c0 4 2 6 5 9"/><path d="M15 18H9v3a1 1 0 0 0 1 1h4a1 1 0 0 0 1-1z"/>"#);

/// `hotel`.
pub const HOTEL: Icon = lucide!("hotel", r#"<path d="M10 22v-6.57"/><path d="M12 11h.01"/><path d="M12 7h.01"/><path d="M14 15.43V22"/><path d="M15 16a5 5 0 0 0-6 0"/><path d="M16 11h.01"/><path d="M16 7h.01"/><path d="M8 11h.01"/><path d="M8 7h.01"/><rect x="4" y="2" width="16" height="20" rx="2"/>"#);

/// `hourglass`.
pub const HOURGLASS: Icon = lucide!("hourglass", r#"<path d="M5 22h14"/><path d="M5 2h14"/><path d="M17 22v-4.172a2 2 0 0 0-.586-1.414L12 12l-4.414 4.414A2 2 0 0 0 7 17.828V22"/><path d="M7 2v4.172a2 2 0 0 0 .586 1.414L12 12l4.414-4.414A2 2 0 0 0 17 6.172V2"/>"#);

/// `hourglass-cog`.
pub const HOURGLASS_COG: Icon = lucide!("hourglass-cog", r#"<path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="M17 2v4.172a2 2 0 0 1-.586 1.414l-8.828 8.828A2 2 0 0 0 7 17.828V22"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><path d="M5 22h6.159"/><path d="M5 2h14"/><path d="M7 2v4.172a2 2 0 0 0 .586 1.414l5.188 5.188"/><circle cx="18" cy="18" r="3"/>"#);

/// `house`.
pub const HOUSE: Icon = lucide!("house", r#"<path d="M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>"#);

/// `house-cog`.
pub const HOUSE_COG: Icon = lucide!("house-cog", r#"<path d="M10.584 21H5a2 2 0 01-2-2v-9a2 2 0 01.709-1.527l7-6a2 2 0 012.582 0l7 6A2 2 0 0121 10.001v.583"/><path d="M14 12H10a1 1 0 00-1 1v8"/><path d="m14.305 19.53.923-.382"/><path d="m15.229 16.852-.924-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.773-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.697-.382-.924"/><path d="m20.773 16.852.922-.383"/><path d="m20.773 19.148.922.383"/><circle cx="18" cy="18" r="3"/>"#);

/// `house-heart`.
pub const HOUSE_HEART: Icon = lucide!("house-heart", r#"<path d="M8.62 13.8A2.25 2.25 0 1 1 12 10.836a2.25 2.25 0 1 1 3.38 2.966l-2.626 2.856a.998.998 0 0 1-1.507 0z"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>"#);

/// `house-plug`.
pub const HOUSE_PLUG: Icon = lucide!("house-plug", r#"<path d="M10 12V8.964"/><path d="M14 12V8.964"/><path d="M15 12a1 1 0 0 1 1 1v2a2 2 0 0 1-2 2h-4a2 2 0 0 1-2-2v-2a1 1 0 0 1 1-1z"/><path d="M8.5 21H5a2 2 0 0 1-2-2v-9a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2h-5a2 2 0 0 1-2-2v-2"/>"#);

/// `house-plus`.
pub const HOUSE_PLUS: Icon = lucide!("house-plus", r#"<path d="M12.35 21H5a2 2 0 0 1-2-2v-9a2 2 0 0 1 .71-1.53l7-6a2 2 0 0 1 2.58 0l7 6A2 2 0 0 1 21 10v2.35"/><path d="M14.8 12.4A1 1 0 0 0 14 12h-4a1 1 0 0 0-1 1v8"/><path d="M15 18h6"/><path d="M18 15v6"/>"#);

/// `house-wifi`.
pub const HOUSE_WIFI: Icon = lucide!("house-wifi", r#"<path d="M9.5 13.866a4 4 0 0 1 5 .01"/><path d="M12 17h.01"/><path d="M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M7 10.754a8 8 0 0 1 10 0"/>"#);

/// `houses`.
pub const HOUSES: Icon = lucide!("houses", r#"<path d="m12.681 4.24.834-.715a1.45 1.45 0 011.88 0l5.09 4.364A1.45 1.45 0 0121 9v6.546a1.45 1.45 0 01-1 1.381"/><path d="M15.485 11.889A1.45 1.45 0 0116 13v6.546A1.454 1.454 0 0114.546 21H4.364a1.454 1.454 0 01-1.454-1.454V13a1.45 1.45 0 01.515-1.111l5.09-4.364a1.45 1.45 0 011.88 0z"/><path d="M7.41 20.546v-4a1 1 0 011-1h2a1 1 0 011 1v4"/>"#);

/// `ice-cream-bowl`.
pub const ICE_CREAM_BOWL: Icon = lucide!("ice-cream-bowl", r#"<path d="M12 17c5 0 8-2.69 8-6H4c0 3.31 3 6 8 6m-4 4h8m-4-3v3M5.14 11a3.5 3.5 0 1 1 6.71 0"/><path d="M12.14 11a3.5 3.5 0 1 1 6.71 0"/><path d="M15.5 6.5a3.5 3.5 0 1 0-7 0"/>"#);

/// `ice-cream-cone`.
pub const ICE_CREAM_CONE: Icon = lucide!("ice-cream-cone", r#"<path d="m7 11 4.08 10.35a1 1 0 0 0 1.84 0L17 11"/><path d="M17 7A5 5 0 0 0 7 7"/><path d="M17 7a2 2 0 0 1 0 4H7a2 2 0 0 1 0-4"/>"#);

/// `id-card`.
pub const ID_CARD: Icon = lucide!("id-card", r#"<path d="M13 19a4 4 0 00-8 0"/><path d="M16 10h2"/><path d="M16 14h2"/><circle cx="9" cy="12" r="3"/><rect x="2" y="5" width="20" height="14" rx="2"/>"#);

/// `id-card-lanyard`.
pub const ID_CARD_LANYARD: Icon = lucide!("id-card-lanyard", r#"<path d="M13.5 8h-3"/><path d="m15 2-1 2h3a2 2 0 012 2v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6a2 2 0 012-2h3"/><path d="M16 22a4 4 0 00-8 0"/><path d="m9 2 3 6"/><circle cx="12" cy="15" r="3"/>"#);

/// `image`.
pub const IMAGE: Icon = lucide!("image", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/>"#);

/// `image-down`.
pub const IMAGE_DOWN: Icon = lucide!("image-down", r#"<path d="M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.814.014L6 21"/><path d="m14 19 3 3v-5.5"/><path d="m17 22 3-3"/><circle cx="9" cy="9" r="2"/>"#);

/// `image-minus`.
pub const IMAGE_MINUS: Icon = lucide!("image-minus", r#"<path d="M21 9v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"/><line x1="16" x2="22" y1="5" y2="5"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/>"#);

/// `image-off`.
pub const IMAGE_OFF: Icon = lucide!("image-off", r#"<line x1="2" x2="22" y1="2" y2="22"/><path d="M10.41 10.41a2 2 0 1 1-2.83-2.83"/><line x1="13.5" x2="6" y1="13.5" y2="21"/><line x1="18" x2="21" y1="12" y2="15"/><path d="M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59"/><path d="M21 15V5a2 2 0 0 0-2-2H9"/>"#);

/// `image-play`.
pub const IMAGE_PLAY: Icon = lucide!("image-play", r#"<path d="M15 15.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997a1 1 0 0 1-1.517-.86z"/><path d="M21 12.17V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"/><path d="m6 21 5-5"/><circle cx="9" cy="9" r="2"/>"#);

/// `image-plus`.
pub const IMAGE_PLUS: Icon = lucide!("image-plus", r#"<path d="M16 5h6"/><path d="M19 2v6"/><path d="M21 11.5V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7.5"/><path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21"/><circle cx="9" cy="9" r="2"/>"#);

/// `image-up`.
pub const IMAGE_UP: Icon = lucide!("image-up", r#"<path d="M10.3 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10l-3.1-3.1a2 2 0 0 0-2.814.014L6 21"/><path d="m14 19.5 3-3 3 3"/><path d="M17 22v-5.5"/><circle cx="9" cy="9" r="2"/>"#);

/// `image-upscale`.
pub const IMAGE_UPSCALE: Icon = lucide!("image-upscale", r#"<path d="M16 3h5v5"/><path d="M17 21h2a2 2 0 0 0 2-2"/><path d="M21 12v3"/><path d="m21 3-5 5"/><path d="M3 7V5a2 2 0 0 1 2-2"/><path d="m5 21 4.144-4.144a1.21 1.21 0 0 1 1.712 0L13 19"/><path d="M9 3h3"/><rect x="3" y="11" width="10" height="10" rx="1"/>"#);

/// `images`.
pub const IMAGES: Icon = lucide!("images", r#"<path d="m22 11-1.296-1.296a2.4 2.4 0 0 0-3.408 0L11 16"/><path d="M4 8a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2"/><circle cx="13" cy="7" r="1" fill="currentColor"/><rect x="8" y="2" width="14" height="14" rx="2"/>"#);

/// `import`.
pub const IMPORT: Icon = lucide!("import", r#"<path d="M12 3v12"/><path d="m8 11 4 4 4-4"/><path d="M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4"/>"#);

/// `inbox`.
pub const INBOX: Icon = lucide!("inbox", r#"<polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/>"#);

/// `indian-rupee`.
pub const INDIAN_RUPEE: Icon = lucide!("indian-rupee", r#"<path d="m14 20-8-8h4c5.333 0 5.333-8 0-8"/><path d="M6 4h12"/><path d="M6 8h12"/>"#);

/// `infinity`.
pub const INFINITY: Icon = lucide!("infinity", r#"<path d="M6 16c5 0 7-8 12-8a4 4 0 0 1 0 8c-5 0-7-8-12-8a4 4 0 1 0 0 8"/>"#);

/// `info`.
pub const INFO: Icon = lucide!("info", r#"<circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/>"#);

/// `inspection-panel`.
pub const INSPECTION_PANEL: Icon = lucide!("inspection-panel", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 7h.01"/><path d="M17 7h.01"/><path d="M7 17h.01"/><path d="M17 17h.01"/>"#);

/// `italic`.
pub const ITALIC: Icon = lucide!("italic", r#"<line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/>"#);

/// `iteration-ccw`.
pub const ITERATION_CCW: Icon = lucide!("iteration-ccw", r#"<path d="m16 14 4 4-4 4"/><path d="M20 10a8 8 0 1 0-8 8h8"/>"#);

/// `iteration-cw`.
pub const ITERATION_CW: Icon = lucide!("iteration-cw", r#"<path d="M4 10a8 8 0 1 1 8 8H4"/><path d="m8 22-4-4 4-4"/>"#);

/// `iv-bag`.
pub const IV_BAG: Icon = lucide!("iv-bag", r#"<path d="M12 18v2a2 2 0 002 2h6"/><path d="M6 11c.72.5 1.44 1 3 1 3 0 3-2 6-2 1.56 0 2.28.5 3 1"/><path d="M9.293 3c.453 0 .887-.18 1.207-.5s.754-.5 1.207-.5h.586c.453 0 .887.18 1.207.5s.754.5 1.207.5H16a2 2 0 012 2v11a2 2 0 01-2 2H8a2 2 0 01-2-2V5a2 2 0 012-2z"/>"#);

/// `japanese-yen`.
pub const JAPANESE_YEN: Icon = lucide!("japanese-yen", r#"<path d="M12 12v8"/><path d="m6 4 6 8 6-8"/><path d="M8 13h8"/><path d="M8 17h8"/>"#);

/// `joystick`.
pub const JOYSTICK: Icon = lucide!("joystick", r#"<path d="M21 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-2Z"/><path d="M6 15v-2"/><path d="M12 15V9"/><circle cx="12" cy="6" r="3"/>"#);

/// `kanban`.
pub const KANBAN: Icon = lucide!("kanban", r#"<path d="M5 3v14"/><path d="M12 3v8"/><path d="M19 3v18"/>"#);

/// `kayak`.
pub const KAYAK: Icon = lucide!("kayak", r#"<path d="M18 17a1 1 0 0 0-1 1v1a2 2 0 1 0 2-2z"/><path d="M20.97 3.61a.45.45 0 0 0-.58-.58C10.2 6.6 6.6 10.2 3.03 20.39a.45.45 0 0 0 .58.58C13.8 17.4 17.4 13.8 20.97 3.61"/><path d="m6.707 6.707 10.586 10.586"/><path d="M7 5a2 2 0 1 0-2 2h1a1 1 0 0 0 1-1z"/>"#);

/// `kazakh-tenge`.
pub const KAZAKH_TENGE: Icon = lucide!("kazakh-tenge", r#"<path d="M12 8v12"/><path d="M6 4h12"/><path d="M6 8h12"/>"#);

/// `key`.
pub const KEY: Icon = lucide!("key", r#"<path d="m2 21 9.6-9.6"/><path d="m7.5 15.5 2.3 2.3a1 1 0 0 1 0 1.4l-2.1 2.1a1 1 0 0 1-1.4 0L4 19"/><circle cx="15.5" cy="7.5" r="5.5"/>"#);

/// `key-round`.
pub const KEY_ROUND: Icon = lucide!("key-round", r#"<path d="M2.586 17.414A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814a6.5 6.5 0 1 0-4-4z"/><circle cx="16.5" cy="7.5" r=".5" fill="currentColor"/>"#);

/// `key-square`.
pub const KEY_SQUARE: Icon = lucide!("key-square", r#"<path d="M12.4 2.7a2.5 2.5 0 0 1 3.4 0l5.5 5.5a2.5 2.5 0 0 1 0 3.4l-3.7 3.7a2.5 2.5 0 0 1-3.4 0L8.7 9.8a2.5 2.5 0 0 1 0-3.4z"/><path d="m14 7 3 3"/><path d="m9.4 10.6-6.814 6.814A2 2 0 0 0 2 18.828V21a1 1 0 0 0 1 1h3a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h1a1 1 0 0 0 1-1v-1a1 1 0 0 1 1-1h.172a2 2 0 0 0 1.414-.586l.814-.814"/>"#);

/// `keyboard`.
pub const KEYBOARD: Icon = lucide!("keyboard", r#"<path d="M10 8h.01"/><path d="M12 12h.01"/><path d="M14 8h.01"/><path d="M16 12h.01"/><path d="M18 8h.01"/><path d="M6 8h.01"/><path d="M7 16h10"/><path d="M8 12h.01"/><rect width="20" height="16" x="2" y="4" rx="2"/>"#);

/// `keyboard-music`.
pub const KEYBOARD_MUSIC: Icon = lucide!("keyboard-music", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h4"/><path d="M14 8h.01"/><path d="M18 8h.01"/><path d="M2 12h20"/><path d="M6 12v4"/><path d="M10 12v4"/><path d="M14 12v4"/><path d="M18 12v4"/>"#);

/// `keyboard-off`.
pub const KEYBOARD_OFF: Icon = lucide!("keyboard-off", r#"<path d="M 20 4 A2 2 0 0 1 22 6"/><path d="M 22 6 L 22 16.41"/><path d="M 7 16 L 16 16"/><path d="M 9.69 4 L 20 4"/><path d="M14 8h.01"/><path d="M18 8h.01"/><path d="m2 2 20 20"/><path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2"/><path d="M6 8h.01"/><path d="M8 12h.01"/>"#);

/// `lambda`.
pub const LAMBDA: Icon = lucide!("lambda", r#"<path d="M11.38 10 5 20"/><path d="M19 18a2 2 0 01-2 2c-4.87-.003-5.052-16-10-16a2 2 0 00-2 2"/>"#);

/// `lamp`.
pub const LAMP: Icon = lucide!("lamp", r#"<path d="M12 12v6"/><path d="M4.077 10.615A1 1 0 0 0 5 12h14a1 1 0 0 0 .923-1.385l-3.077-7.384A2 2 0 0 0 15 2H9a2 2 0 0 0-1.846 1.23Z"/><path d="M8 20a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H9a1 1 0 0 1-1-1z"/>"#);

/// `lamp-ceiling`.
pub const LAMP_CEILING: Icon = lucide!("lamp-ceiling", r#"<path d="M12 2v5"/><path d="M14.829 15.998a3 3 0 1 1-5.658 0"/><path d="M20.92 14.606A1 1 0 0 1 20 16H4a1 1 0 0 1-.92-1.394l3-7A1 1 0 0 1 7 7h10a1 1 0 0 1 .92.606z"/>"#);

/// `lamp-desk`.
pub const LAMP_DESK: Icon = lucide!("lamp-desk", r#"<path d="M10.293 2.293a1 1 0 0 1 1.414 0l2.5 2.5 5.994 1.227a1 1 0 0 1 .506 1.687l-7 7a1 1 0 0 1-1.687-.506l-1.227-5.994-2.5-2.5a1 1 0 0 1 0-1.414z"/><path d="m14.207 4.793-3.414 3.414"/><path d="M3 20a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1z"/><path d="m9.086 6.5-4.793 4.793a1 1 0 0 0-.18 1.17L7 18"/>"#);

/// `lamp-floor`.
pub const LAMP_FLOOR: Icon = lucide!("lamp-floor", r#"<path d="M12 10v12"/><path d="M17.929 7.629A1 1 0 0 1 17 9H7a1 1 0 0 1-.928-1.371l2-5A1 1 0 0 1 9 2h6a1 1 0 0 1 .928.629z"/><path d="M9 22h6"/>"#);

/// `lamp-wall-down`.
pub const LAMP_WALL_DOWN: Icon = lucide!("lamp-wall-down", r#"<path d="M19.929 18.629A1 1 0 0 1 19 20H9a1 1 0 0 1-.928-1.371l2-5A1 1 0 0 1 11 13h6a1 1 0 0 1 .928.629z"/><path d="M6 3a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1z"/><path d="M8 6h4a2 2 0 0 1 2 2v5"/>"#);

/// `lamp-wall-up`.
pub const LAMP_WALL_UP: Icon = lucide!("lamp-wall-up", r#"<path d="M19.929 9.629A1 1 0 0 1 19 11H9a1 1 0 0 1-.928-1.371l2-5A1 1 0 0 1 11 4h6a1 1 0 0 1 .928.629z"/><path d="M6 15a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H5a1 1 0 0 1-1-1v-4a1 1 0 0 1 1-1z"/><path d="M8 18h4a2 2 0 0 0 2-2v-5"/>"#);

/// `land-plot`.
pub const LAND_PLOT: Icon = lucide!("land-plot", r#"<path d="m12 8 6-3-6-3v10"/><path d="m8 11.99-5.5 3.14a1 1 0 0 0 0 1.74l8.5 4.86a2 2 0 0 0 2 0l8.5-4.86a1 1 0 0 0 0-1.74L16 12"/><path d="m6.49 12.85 11.02 6.3"/><path d="M17.51 12.85 6.5 19.15"/>"#);

/// `landmark`.
pub const LANDMARK: Icon = lucide!("landmark", r#"<path d="M10 18v-7"/><path d="M11.119 2.205a2 2 0 0 1 1.762 0l7.84 3.846A.5.5 0 0 1 20.5 7h-17a.5.5 0 0 1-.22-.949z"/><path d="M14 18v-7"/><path d="M18 18v-7"/><path d="M3 22h18"/><path d="M6 18v-7"/>"#);

/// `languages`.
pub const LANGUAGES: Icon = lucide!("languages", r#"<path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/>"#);

/// `laptop`.
pub const LAPTOP: Icon = lucide!("laptop", r#"<path d="M18 5a2 2 0 0 1 2 2v8.526a2 2 0 0 0 .212.897l1.068 2.127a1 1 0 0 1-.9 1.45H3.62a1 1 0 0 1-.9-1.45l1.068-2.127A2 2 0 0 0 4 15.526V7a2 2 0 0 1 2-2z"/><path d="M20.054 15.987H3.946"/>"#);

/// `laptop-minimal`.
pub const LAPTOP_MINIMAL: Icon = lucide!("laptop-minimal", r#"<rect width="18" height="12" x="3" y="4" rx="2" ry="2"/><line x1="2" x2="22" y1="20" y2="20"/>"#);

/// `laptop-minimal-check`.
pub const LAPTOP_MINIMAL_CHECK: Icon = lucide!("laptop-minimal-check", r#"<path d="M2 20h20"/><path d="m9 10 2 2 4-4"/><rect x="3" y="4" width="18" height="12" rx="2"/>"#);

/// `laser`.
pub const LASER: Icon = lucide!("laser", r#"<path d="m13 8 .95-.95"/><path d="M13.95 16.95 13 16"/><path d="M22 12h-9"/><path d="M4 12H2"/><path d="M4.05 16.95 5 16"/><path d="m5 8-.95-.95"/><path d="M9 12h.01"/><path d="M9 19v-2"/><path d="M9 7V5"/>"#);

/// `lasso`.
pub const LASSO: Icon = lucide!("lasso", r#"<path d="M3.704 14.467a10 8 0 1 1 3.115 2.375"/><path d="M7 22a5 5 0 0 1-2-3.994"/><circle cx="5" cy="16" r="2"/>"#);

/// `lasso-select`.
pub const LASSO_SELECT: Icon = lucide!("lasso-select", r#"<path d="M7 22a5 5 0 0 1-2-4"/><path d="M7 16.93c.96.43 1.96.74 2.99.91"/><path d="M3.34 14A6.8 6.8 0 0 1 2 10c0-4.42 4.48-8 10-8s10 3.58 10 8a7.19 7.19 0 0 1-.33 2"/><path d="M5 18a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/><path d="M14.33 22h-.09a.35.35 0 0 1-.24-.32v-10a.34.34 0 0 1 .33-.34c.08 0 .15.03.21.08l7.34 6a.33.33 0 0 1-.21.59h-4.49l-2.57 3.85a.35.35 0 0 1-.28.14z"/>"#);

/// `layer-arrow-down`.
pub const LAYER_ARROW_DOWN: Icon = lucide!("layer-arrow-down", r#"<path d="M12 10v10"/><path d="M22 10a1 1 0 01-.59.92l-5.077 2.308"/><path d="M22.017 10.005a1 1 0 00-.597-.916l-8.59-3.91a2 2 0 00-1.66.001L2.6 9.08a1 1 0 00-.02 1.831l5.093 2.316"/><path d="m9 17 3 3 3-3"/>"#);

/// `layer-arrow-up`.
pub const LAYER_ARROW_UP: Icon = lucide!("layer-arrow-up", r#"<path d="M12 14V4"/><path d="M7.674 10.774 2.58 13.09a1 1 0 000 1.822l8.6 3.91a2 2 0 001.65 0l8.58-3.9a1 1 0 00.59-.92 1 1 0 00-.59-.922l-5.078-2.308"/><path d="m9 7 3-3 3 3"/>"#);

/// `layers`.
pub const LAYERS: Icon = lucide!("layers", r#"<path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83z"/><path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 12"/><path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l8.58-3.9A1 1 0 0 0 22 17"/>"#);

/// `layers-2`.
pub const LAYERS_2: Icon = lucide!("layers-2", r#"<path d="M13 13.74a2 2 0 0 1-2 0L2.5 8.87a1 1 0 0 1 0-1.74L11 2.26a2 2 0 0 1 2 0l8.5 4.87a1 1 0 0 1 0 1.74z"/><path d="m20 14.285 1.5.845a1 1 0 0 1 0 1.74L13 21.74a2 2 0 0 1-2 0l-8.5-4.87a1 1 0 0 1 0-1.74l1.5-.845"/>"#);

/// `layers-arrow-down`.
pub const LAYERS_ARROW_DOWN: Icon = lucide!("layers-arrow-down", r#"<path d="M12 7v15"/><path d="M2 12a1 1 0 00.58.91l5.093 2.316"/><path d="M22 12a1 1 0 01-.59.92l-5.077 2.308"/><path d="M8 10.37 2.6 7.91a1 1 0 010-1.831l8.57-3.9a2 2 0 011.66.001l8.59 3.91a1 1 0 010 1.831l-5.392 2.45"/><path d="m9 19 3 3 3-3"/>"#);

/// `layers-arrow-up`.
pub const LAYERS_ARROW_UP: Icon = lucide!("layers-arrow-up", r#"<path d="M12 12V2"/><path d="M2 17.002a1 1 0 00.58.91l8.6 3.91a2 2 0 001.65 0l8.58-3.9a1 1 0 00.59-.92"/><path d="M7.674 8.774 2.58 11.09a1 1 0 000 1.822l8.6 3.91a2 2 0 001.65 0l8.58-3.9a1 1 0 00.59-.92 1 1 0 00-.59-.922l-5.078-2.308"/><path d="m9 5 3-3 3 3"/>"#);

/// `layers-minus`.
pub const LAYERS_MINUS: Icon = lucide!("layers-minus", r#"<path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 .83.18 2 2 0 0 0 .83-.18l8.58-3.9a1 1 0 0 0 0-1.832z"/><path d="M16 17h6"/><path d="M2.003 11.995a1 1 0 0 0 .597.915l8.58 3.91a2 2 0 0 0 .83.18"/><path d="M2.003 16.995a1 1 0 0 0 .597.915l8.58 3.91a2 2 0 0 0 .83.18 2 2 0 0 0 .83-.18l2.11-.96"/><path d="M22.018 12.004a1 1 0 0 1-.598.916l-.177.08"/>"#);

/// `layers-plus`.
pub const LAYERS_PLUS: Icon = lucide!("layers-plus", r#"<path d="M12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 .83.18 2 2 0 0 0 .83-.18l8.58-3.9a1 1 0 0 0 0-1.831z"/><path d="M16 17h6"/><path d="M19 14v6"/><path d="M2 12a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 .825.178"/><path d="M2 17a1 1 0 0 0 .58.91l8.6 3.91a2 2 0 0 0 1.65 0l2.116-.962"/>"#);

/// `layout-arrow-down`.
pub const LAYOUT_ARROW_DOWN: Icon = lucide!("layout-arrow-down", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/><path d="M18 3v18"/><path d="m21 18-3 3-3-3"/>"#);

/// `layout-arrow-right`.
pub const LAYOUT_ARROW_RIGHT: Icon = lucide!("layout-arrow-right", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><path d="M3 18h18"/><path d="m18 21 3-3-3-3"/>"#);

/// `layout-dashboard`.
pub const LAYOUT_DASHBOARD: Icon = lucide!("layout-dashboard", r#"<rect width="7" height="9" x="3" y="3" rx="1"/><rect width="7" height="5" x="14" y="3" rx="1"/><rect width="7" height="9" x="14" y="12" rx="1"/><rect width="7" height="5" x="3" y="16" rx="1"/>"#);

/// `layout-freeform`.
pub const LAYOUT_FREEFORM: Icon = lucide!("layout-freeform", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="4" rx="1"/><rect width="7" height="7" x="4" y="14" rx="1"/>"#);

/// `layout-grid`.
pub const LAYOUT_GRID: Icon = lucide!("layout-grid", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>"#);

/// `layout-grid-circles`.
pub const LAYOUT_GRID_CIRCLES: Icon = lucide!("layout-grid-circles", r#"<circle cx="17.5" cy="17.5" r="3.5"/><circle cx="17.5" cy="6.5" r="3.5"/><circle cx="6.5" cy="17.5" r="3.5"/><circle cx="6.5" cy="6.5" r="3.5"/>"#);

/// `layout-list`.
pub const LAYOUT_LIST: Icon = lucide!("layout-list", r#"<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/><path d="M14 4h7"/><path d="M14 9h7"/><path d="M14 15h7"/><path d="M14 20h7"/>"#);

/// `layout-panel-left`.
pub const LAYOUT_PANEL_LEFT: Icon = lucide!("layout-panel-left", r#"<rect width="7" height="18" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/>"#);

/// `layout-panel-top`.
pub const LAYOUT_PANEL_TOP: Icon = lucide!("layout-panel-top", r#"<rect width="18" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/>"#);

/// `layout-template`.
pub const LAYOUT_TEMPLATE: Icon = lucide!("layout-template", r#"<rect width="18" height="7" x="3" y="3" rx="1"/><rect width="9" height="7" x="3" y="14" rx="1"/><rect width="5" height="7" x="16" y="14" rx="1"/>"#);

/// `leaf`.
pub const LEAF: Icon = lucide!("leaf", r#"<path d="M11 20a10 10 0 0010-10 25.9 25.9 0 00-1.04-7.281 1 1 0 00-1.755-.325C15.833 5.5 13 5.5 9.8 6.1A7 7 0 0011 20"/><path d="M2 21a5 5 0 012.911-4.544C7.613 15.212 8.351 15.24 11 13"/>"#);

/// `leafy-green`.
pub const LEAFY_GREEN: Icon = lucide!("leafy-green", r#"<path d="M2 22c1.25-.987 2.27-1.975 3.9-2.2a5.56 5.56 0 0 1 3.8 1.5 4 4 0 0 0 6.187-2.353 3.5 3.5 0 0 0 3.69-5.116A3.5 3.5 0 0 0 20.95 8 3.5 3.5 0 1 0 16 3.05a3.5 3.5 0 0 0-5.831 1.373 3.5 3.5 0 0 0-5.116 3.69 4 4 0 0 0-2.348 6.155C3.499 15.42 4.409 16.712 4.2 18.1 3.926 19.743 3.014 20.732 2 22"/><path d="M2 22 17 7"/>"#);

/// `lectern`.
pub const LECTERN: Icon = lucide!("lectern", r#"<path d="M15 13h4a2 2 0 001.901-1.38l1.057-4.333A1 1 0 0021 6H3a1 1 0 00-.958 1.287L3.1 11.621A2 2 0 005.001 13h4"/><path d="M15 22V11a1 1 0 00-1-1h-4a1 1 0 00-1 1v11"/><path d="M18 22H6"/><path d="M18 6V3a1 1 0 00-1-1h-3"/>"#);

/// `lens`.
pub const LENS: Icon = lucide!("lens", r#"<path d="M15.725 7.296h.01"/><path d="M6 12a6 6 0 016-6"/><circle cx="12" cy="12" r="10"/>"#);

/// `lens-concave`.
pub const LENS_CONCAVE: Icon = lucide!("lens-concave", r#"<path d="M7 2a1 1 0 0 0-.8 1.6 14 14 0 0 1 0 16.8A1 1 0 0 0 7 22h10a1 1 0 0 0 .8-1.6 14 14 0 0 1 0-16.8A1 1 0 0 0 17 2z"/>"#);

/// `lens-convex`.
pub const LENS_CONVEX: Icon = lucide!("lens-convex", r#"<path d="M13.433 2a1 1 0 0 1 .824.448 18 18 0 0 1 0 19.104 1 1 0 0 1-.824.448h-2.866a1 1 0 0 1-.824-.448 18 18 0 0 1 0-19.104A1 1 0 0 1 10.567 2z"/>"#);

/// `letter-a`.
pub const LETTER_A: Icon = lucide!("letter-a", r#"<path d="M17.117 14H6.883"/><path d="m4 20 7.55-15.712a.5.5 0 01.9 0L20 20"/>"#);

/// `letter-b`.
pub const LETTER_B: Icon = lucide!("letter-b", r#"<path d="M6 12h9a4 4 0 010 8H7a1 1 0 01-1-1V5a1 1 0 011-1h7a4 4 0 010 8"/>"#);

/// `letter-c`.
pub const LETTER_C: Icon = lucide!("letter-c", r#"<path d="M19 6.708a8 8 0 100 10.584"/>"#);

/// `letter-d`.
pub const LETTER_D: Icon = lucide!("letter-d", r#"<path d="M11 4a8 8 0 010 16H6.406A.406.406 0 016 19.594V4.406A.406.406 0 016.406 4z"/>"#);

/// `letter-e`.
pub const LETTER_E: Icon = lucide!("letter-e", r#"<path d="M18 4H7a1 1 0 00-1 1v14a1 1 0 001 1h11"/><path d="M6 12h8"/>"#);

/// `letter-f`.
pub const LETTER_F: Icon = lucide!("letter-f", r#"<path d="M6 12h8"/><path d="M6 20V5a1 1 0 011-1h11"/>"#);

/// `letter-g`.
pub const LETTER_G: Icon = lucide!("letter-g", r#"<path d="M19 6.708a8 8 0 10-.526 11.126c.355-.333.526-.811.526-1.299V13a1 1 0 00-1-1h-4"/>"#);

/// `letter-h`.
pub const LETTER_H: Icon = lucide!("letter-h", r#"<path d="M18 20V4"/><path d="M6 12H18"/><path d="M6 20V4"/>"#);

/// `letter-i`.
pub const LETTER_I: Icon = lucide!("letter-i", r#"<path d="M12 20V4"/>"#);

/// `letter-j`.
pub const LETTER_J: Icon = lucide!("letter-j", r#"<path d="M17 4v11a5 5 0 01-10 0"/>"#);

/// `letter-k`.
pub const LETTER_K: Icon = lucide!("letter-k", r#"<path d="M17 4 6 15"/><path d="M6 20V4"/><path d="m9 12 8 8"/>"#);

/// `letter-l`.
pub const LETTER_L: Icon = lucide!("letter-l", r#"<path d="M6 4v15a1 1 0 001 1h11"/>"#);

/// `letter-m`.
pub const LETTER_M: Icon = lucide!("letter-m", r#"<path d="M4 20V4.253a.25.25 0 01.454-.144l6.727 9.467a1 1 0 001.63 0l6.735-9.468a.25.25 0 01.454.145V20"/>"#);

/// `letter-n`.
pub const LETTER_N: Icon = lucide!("letter-n", r#"<path d="M6 20V4.25a.25.25 0 01.455-.143l11.09 15.786a.25.25 0 00.455-.144V4"/>"#);

/// `letter-o`.
pub const LETTER_O: Icon = lucide!("letter-o", r#"<circle cx="12" cy="12" r="8"/>"#);

/// `letter-p`.
pub const LETTER_P: Icon = lucide!("letter-p", r#"<path d="M6 12h8a4 4 0 000-8H7a1 1 0 00-1 1v15"/>"#);

/// `letter-q`.
pub const LETTER_Q: Icon = lucide!("letter-q", r#"<circle cx="12" cy="12" r="8"/><path d="M20 20L15 15"/>"#);

/// `letter-r`.
pub const LETTER_R: Icon = lucide!("letter-r", r#"<path d="m10 12 8 8"/><path d="M6 12h8a4 4 0 000-8H7a1 1 0 00-1 1v15"/>"#);

/// `letter-s`.
pub const LETTER_S: Icon = lucide!("letter-s", r#"<path d="M17 4h-7a4 4 0 000 8h4a4 4 0 010 8H6"/>"#);

/// `letter-spacing`.
pub const LETTER_SPACING: Icon = lucide!("letter-spacing", r#"<path d="M15 14H9"/><path d="M21 3v18"/><path d="M3 3v18"/><path d="m8 16 3.453-7.648a.6.6 0 011.094.001L16 16"/>"#);

/// `letter-t`.
pub const LETTER_T: Icon = lucide!("letter-t", r#"<path d="M12 20V4"/><path d="M5 4h14"/>"#);

/// `letter-u`.
pub const LETTER_U: Icon = lucide!("letter-u", r#"<path d="M6 4v10a6 6 0 0012 0V4"/>"#);

/// `letter-v`.
pub const LETTER_V: Icon = lucide!("letter-v", r#"<path d="m6 4 5.765 15.835a.25.25 0 00.47 0L18 4"/>"#);

/// `letter-w`.
pub const LETTER_W: Icon = lucide!("letter-w", r#"<path d="m21 4-4.794 15.821a.25.25 0 01-.471.021l-3.503-8.685a.25.25 0 00-.464 0l-3.503 8.685a.25.25 0 01-.47-.021L3 4"/>"#);

/// `letter-x`.
pub const LETTER_X: Icon = lucide!("letter-x", r#"<path d="M18 4 6 20"/><path d="m6 4 12 16"/>"#);

/// `letter-y`.
pub const LETTER_Y: Icon = lucide!("letter-y", r#"<path d="M12 12v8"/><path d="m6 4 6 8 6-8"/>"#);

/// `letter-z`.
pub const LETTER_Z: Icon = lucide!("letter-z", r#"<path d="M6 4h11.5a.5.5 0 01.387.817L6.114 19.183a.5.5 0 00.387.817H18"/>"#);

/// `letters`.
pub const LETTERS: Icon = lucide!("letters", r#"<path d="M15 8H9"/><path d="M21 15.354a4 4 0 100 5.292"/><path d="M3 18h4a2 2 0 010 4H3.5a.5.5 0 01-.5-.5v-7a.5.5 0 01.5-.5H6a2 2 0 010 4"/><path d="m8 10 3.453-7.648a.6.6 0 011.094 0L16 10"/>"#);

/// `library`.
pub const LIBRARY: Icon = lucide!("library", r#"<path d="m16 6 4 14"/><path d="M12 6v14"/><path d="M8 8v12"/><path d="M4 4v16"/>"#);

/// `library-big`.
pub const LIBRARY_BIG: Icon = lucide!("library-big", r#"<rect width="8" height="18" x="3" y="3" rx="1"/><path d="M7 3v18"/><path d="M20.4 18.9c.2.5-.1 1.1-.6 1.3l-1.9.7c-.5.2-1.1-.1-1.3-.6L11.1 5.1c-.2-.5.1-1.1.6-1.3l1.9-.7c.5-.2 1.1.1 1.3.6Z"/>"#);

/// `life-buoy`.
pub const LIFE_BUOY: Icon = lucide!("life-buoy", r#"<circle cx="12" cy="12" r="10"/><path d="m4.93 4.93 4.24 4.24"/><path d="m14.83 9.17 4.24-4.24"/><path d="m14.83 14.83 4.24 4.24"/><path d="m9.17 14.83-4.24 4.24"/><circle cx="12" cy="12" r="4"/>"#);

/// `ligature`.
pub const LIGATURE: Icon = lucide!("ligature", r#"<path d="M14 12h2v8"/><path d="M14 20h4"/><path d="M6 12h4"/><path d="M6 20h4"/><path d="M8 20V8a4 4 0 0 1 7.464-2"/>"#);

/// `lightbulb`.
pub const LIGHTBULB: Icon = lucide!("lightbulb", r#"<path d="M15 14c.2-1 .7-1.7 1.5-2.5 1-.9 1.5-2.2 1.5-3.5A6 6 0 0 0 6 8c0 1 .2 2.2 1.5 3.5.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/>"#);

/// `lightbulb-off`.
pub const LIGHTBULB_OFF: Icon = lucide!("lightbulb-off", r#"<path d="M16.8 11.2c.8-.9 1.2-2 1.2-3.2a6 6 0 0 0-9.3-5"/><path d="m2 2 20 20"/><path d="M6.3 6.3a4.67 4.67 0 0 0 1.2 5.2c.7.7 1.3 1.5 1.5 2.5"/><path d="M9 18h6"/><path d="M10 22h4"/>"#);

/// `lighthouse`.
pub const LIGHTHOUSE: Icon = lucide!("lighthouse", r#"<path d="M12 3V2"/><path d="M16.066 16.865 7 22l2-11V6a3 3 0 016 0v5l2 11"/><path d="m19.792 4.5.866-.5"/><path d="m19.797 13.5.866.5"/><path d="M21 9h1"/><path d="M3 9H2"/><path d="m4.203 13.5-.866.5"/><path d="M4.208 4.5 3.342 4"/><path d="M5.5 22h13"/><path d="m7.932 16.875 7.377-4.178"/><path d="M8 11h8"/><path d="M8 7h8"/>"#);

/// `line-dot-bottom-vertical`.
pub const LINE_DOT_BOTTOM_VERTICAL: Icon = lucide!("line-dot-bottom-vertical", r#"<path d="M12 3v12"/><circle cx="12" cy="18" r="3"/>"#);

/// `line-dot-left-horizontal`.
pub const LINE_DOT_LEFT_HORIZONTAL: Icon = lucide!("line-dot-left-horizontal", r#"<path d="M9 12h12"/><circle cx="6" cy="12" r="3"/>"#);

/// `line-dot-right-horizontal`.
pub const LINE_DOT_RIGHT_HORIZONTAL: Icon = lucide!("line-dot-right-horizontal", r#"<path d="M 3 12 L 15 12"/><circle cx="18" cy="12" r="3"/>"#);

/// `line-dot-top-vertical`.
pub const LINE_DOT_TOP_VERTICAL: Icon = lucide!("line-dot-top-vertical", r#"<path d="M12 9v12"/><circle cx="12" cy="6" r="3"/>"#);

/// `line-height`.
pub const LINE_HEIGHT: Icon = lucide!("line-height", r#"<path d="M15 14H9"/><path d="M21 21H3"/><path d="M21 3H3"/><path d="m8 16 3.453-7.648a.6.6 0 011.094.001L16 16"/>"#);

/// `line-squiggle`.
pub const LINE_SQUIGGLE: Icon = lucide!("line-squiggle", r#"<path d="M7 3.5c5-2 7 2.5 3 4C1.5 10 2 15 5 16c5 2 9-10 14-7s.5 13.5-4 12c-5-2.5.5-11 6-2"/>"#);

/// `line-style`.
pub const LINE_STYLE: Icon = lucide!("line-style", r#"<path d="M11 5h2"/><path d="M15 12h6"/><path d="M19 5h2"/><path d="M3 12h6"/><path d="M3 19h18"/><path d="M3 5h2"/>"#);

/// `link`.
pub const LINK: Icon = lucide!("link", r#"<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>"#);

/// `link-2`.
pub const LINK_2: Icon = lucide!("link-2", r#"<path d="M9 17H7A5 5 0 0 1 7 7h2"/><path d="M15 7h2a5 5 0 1 1 0 10h-2"/><line x1="8" x2="16" y1="12" y2="12"/>"#);

/// `link-2-off`.
pub const LINK_2_OFF: Icon = lucide!("link-2-off", r#"<path d="M9 17H7A5 5 0 0 1 7 7"/><path d="M15 7h2a5 5 0 0 1 4 8"/><line x1="8" x2="12" y1="12" y2="12"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `list`.
pub const LIST: Icon = lucide!("list", r#"<path d="M3 5h.01"/><path d="M3 12h.01"/><path d="M3 19h.01"/><path d="M8 5h13"/><path d="M8 12h13"/><path d="M8 19h13"/>"#);

/// `list-check`.
pub const LIST_CHECK: Icon = lucide!("list-check", r#"<path d="M16 5H3"/><path d="M16 12H3"/><path d="M11 19H3"/><path d="m15 18 2 2 4-4"/>"#);

/// `list-checks`.
pub const LIST_CHECKS: Icon = lucide!("list-checks", r#"<path d="M13 5h8"/><path d="M13 12h8"/><path d="M13 19h8"/><path d="m3 17 2 2 4-4"/><path d="m3 7 2 2 4-4"/>"#);

/// `list-chevrons-down-up`.
pub const LIST_CHEVRONS_DOWN_UP: Icon = lucide!("list-chevrons-down-up", r#"<path d="M3 5h8"/><path d="M3 12h8"/><path d="M3 19h8"/><path d="m15 5 3 3 3-3"/><path d="m15 19 3-3 3 3"/>"#);

/// `list-chevrons-up-down`.
pub const LIST_CHEVRONS_UP_DOWN: Icon = lucide!("list-chevrons-up-down", r#"<path d="M3 5h8"/><path d="M3 12h8"/><path d="M3 19h8"/><path d="m15 8 3-3 3 3"/><path d="m15 16 3 3 3-3"/>"#);

/// `list-clock`.
pub const LIST_CLOCK: Icon = lucide!("list-clock", r#"<path d="M16 13v2.2l1.6 1"/><path d="M3 12h3.458"/><path d="M3 19h3.832"/><path d="M3 5h18"/><circle cx="16" cy="15" r="6"/>"#);

/// `list-collapse`.
pub const LIST_COLLAPSE: Icon = lucide!("list-collapse", r#"<path d="M10 5h11"/><path d="M10 12h11"/><path d="M10 19h11"/><path d="m3 10 3-3-3-3"/><path d="m3 20 3-3-3-3"/>"#);

/// `list-end`.
pub const LIST_END: Icon = lucide!("list-end", r#"<path d="M16 5H3"/><path d="M16 12H3"/><path d="M9 19H3"/><path d="m16 16-3 3 3 3"/><path d="M21 5v12a2 2 0 0 1-2 2h-6"/>"#);

/// `list-filter`.
pub const LIST_FILTER: Icon = lucide!("list-filter", r#"<path d="M2 5h20"/><path d="M6 12h12"/><path d="M9 19h6"/>"#);

/// `list-filter-plus`.
pub const LIST_FILTER_PLUS: Icon = lucide!("list-filter-plus", r#"<path d="M12 5H2"/><path d="M6 12h12"/><path d="M9 19h6"/><path d="M16 5h6"/><path d="M19 8V2"/>"#);

/// `list-indent-decrease`.
pub const LIST_INDENT_DECREASE: Icon = lucide!("list-indent-decrease", r#"<path d="M21 5H11"/><path d="M21 12H11"/><path d="M21 19H11"/><path d="m7 8-4 4 4 4"/>"#);

/// `list-indent-increase`.
pub const LIST_INDENT_INCREASE: Icon = lucide!("list-indent-increase", r#"<path d="M21 5H11"/><path d="M21 12H11"/><path d="M21 19H11"/><path d="m3 8 4 4-4 4"/>"#);

/// `list-minus`.
pub const LIST_MINUS: Icon = lucide!("list-minus", r#"<path d="M16 5H3"/><path d="M11 12H3"/><path d="M16 19H3"/><path d="M21 12h-6"/>"#);

/// `list-music`.
pub const LIST_MUSIC: Icon = lucide!("list-music", r#"<path d="M16 5H3"/><path d="M11 12H3"/><path d="M11 19H3"/><path d="M21 16V5"/><circle cx="18" cy="16" r="3"/>"#);

/// `list-ordered`.
pub const LIST_ORDERED: Icon = lucide!("list-ordered", r#"<path d="M11 5h10"/><path d="M11 12h10"/><path d="M11 19h10"/><path d="M4 4h1v5"/><path d="M4 9h2"/><path d="M6.5 20H3.4c0-1 2.6-1.925 2.6-3.5a1.5 1.5 0 0 0-2.6-1.02"/>"#);

/// `list-plus`.
pub const LIST_PLUS: Icon = lucide!("list-plus", r#"<path d="M16 5H3"/><path d="M11 12H3"/><path d="M16 19H3"/><path d="M18 9v6"/><path d="M21 12h-6"/>"#);

/// `list-restart`.
pub const LIST_RESTART: Icon = lucide!("list-restart", r#"<path d="M21 5H3"/><path d="M7 12H3"/><path d="M7 19H3"/><path d="M12 18a5 5 0 0 0 9-3 4.5 4.5 0 0 0-4.5-4.5c-1.33 0-2.54.54-3.41 1.41L11 14"/><path d="M11 10v4h4"/>"#);

/// `list-sort-ascending`.
pub const LIST_SORT_ASCENDING: Icon = lucide!("list-sort-ascending", r#"<path d="M3 19h18"/><path d="M15 12H3"/><path d="M9 5H3"/>"#);

/// `list-sort-descending`.
pub const LIST_SORT_DESCENDING: Icon = lucide!("list-sort-descending", r#"<path d="M15 12H3"/><path d="M3 5h18"/><path d="M9 19H3"/>"#);

/// `list-start`.
pub const LIST_START: Icon = lucide!("list-start", r#"<path d="M3 5h6"/><path d="M3 12h13"/><path d="M3 19h13"/><path d="m16 8-3-3 3-3"/><path d="M21 19V7a2 2 0 0 0-2-2h-6"/>"#);

/// `list-todo`.
pub const LIST_TODO: Icon = lucide!("list-todo", r#"<path d="M13 5h8"/><path d="M13 12h8"/><path d="M13 19h8"/><path d="m3 17 2 2 4-4"/><rect x="3" y="4" width="6" height="6" rx="1"/>"#);

/// `list-tree`.
pub const LIST_TREE: Icon = lucide!("list-tree", r#"<path d="M8 5h13"/><path d="M13 12h8"/><path d="M13 19h8"/><path d="M3 10a2 2 0 0 0 2 2h3"/><path d="M3 5v12a2 2 0 0 0 2 2h3"/>"#);

/// `list-video`.
pub const LIST_VIDEO: Icon = lucide!("list-video", r#"<path d="M21 5H3"/><path d="M10 12H3"/><path d="M10 19H3"/><path d="M15 12.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997a1 1 0 0 1-1.517-.86z"/>"#);

/// `list-x`.
pub const LIST_X: Icon = lucide!("list-x", r#"<path d="M16 5H3"/><path d="M11 12H3"/><path d="M16 19H3"/><path d="m15.5 9.5 5 5"/><path d="m20.5 9.5-5 5"/>"#);

/// `loader`.
pub const LOADER: Icon = lucide!("loader", r#"<path d="M12 2v4"/><path d="m16.2 7.8 2.9-2.9"/><path d="M18 12h4"/><path d="m16.2 16.2 2.9 2.9"/><path d="M12 18v4"/><path d="m4.9 19.1 2.9-2.9"/><path d="M2 12h4"/><path d="m4.9 4.9 2.9 2.9"/>"#);

/// `loader-circle`.
pub const LOADER_CIRCLE: Icon = lucide!("loader-circle", r#"<path d="M21 12a9 9 0 1 1-6.219-8.56"/>"#);

/// `loader-pinwheel`.
pub const LOADER_PINWHEEL: Icon = lucide!("loader-pinwheel", r#"<path d="M22 12a1 1 0 0 1-10 0 1 1 0 0 0-10 0"/><path d="M7 20.7a1 1 0 1 1 5-8.7 1 1 0 1 0 5-8.6"/><path d="M7 3.3a1 1 0 1 1 5 8.6 1 1 0 1 0 5 8.6"/><circle cx="12" cy="12" r="10"/>"#);

/// `locate`.
pub const LOCATE: Icon = lucide!("locate", r#"<line x1="2" x2="5" y1="12" y2="12"/><line x1="19" x2="22" y1="12" y2="12"/><line x1="12" x2="12" y1="2" y2="5"/><line x1="12" x2="12" y1="19" y2="22"/><circle cx="12" cy="12" r="7"/>"#);

/// `locate-fixed`.
pub const LOCATE_FIXED: Icon = lucide!("locate-fixed", r#"<line x1="2" x2="5" y1="12" y2="12"/><line x1="19" x2="22" y1="12" y2="12"/><line x1="12" x2="12" y1="2" y2="5"/><line x1="12" x2="12" y1="19" y2="22"/><circle cx="12" cy="12" r="7"/><circle cx="12" cy="12" r="3"/>"#);

/// `locate-off`.
pub const LOCATE_OFF: Icon = lucide!("locate-off", r#"<path d="M12 19v3"/><path d="M12 2v3"/><path d="M18.89 13.24a7 7 0 0 0-8.13-8.13"/><path d="M19 12h3"/><path d="M2 12h3"/><path d="m2 2 20 20"/><path d="M7.05 7.05a7 7 0 0 0 9.9 9.9"/>"#);

/// `lock`.
pub const LOCK: Icon = lucide!("lock", r#"<rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>"#);

/// `lock-keyhole`.
pub const LOCK_KEYHOLE: Icon = lucide!("lock-keyhole", r#"<circle cx="12" cy="16" r="1"/><rect x="3" y="10" width="18" height="12" rx="2"/><path d="M7 10V7a5 5 0 0 1 10 0v3"/>"#);

/// `lock-keyhole-open`.
pub const LOCK_KEYHOLE_OPEN: Icon = lucide!("lock-keyhole-open", r#"<circle cx="12" cy="16" r="1"/><rect width="18" height="12" x="3" y="10" rx="2"/><path d="M7 10V7a5 5 0 0 1 9.33-2.5"/>"#);

/// `lock-open`.
pub const LOCK_OPEN: Icon = lucide!("lock-open", r#"<rect width="18" height="11" x="3" y="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 9.9-1"/>"#);

/// `log-in`.
pub const LOG_IN: Icon = lucide!("log-in", r#"<path d="m10 17 5-5-5-5"/><path d="M15 12H3"/><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>"#);

/// `log-out`.
pub const LOG_OUT: Icon = lucide!("log-out", r#"<path d="m16 17 5-5-5-5"/><path d="M21 12H9"/><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>"#);

/// `logs`.
pub const LOGS: Icon = lucide!("logs", r#"<path d="M3 5h1"/><path d="M3 12h1"/><path d="M3 19h1"/><path d="M8 5h1"/><path d="M8 12h1"/><path d="M8 19h1"/><path d="M13 5h8"/><path d="M13 12h8"/><path d="M13 19h8"/>"#);

/// `lollipop`.
pub const LOLLIPOP: Icon = lucide!("lollipop", r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/><path d="M11 11a2 2 0 0 0 4 0 4 4 0 0 0-8 0 6 6 0 0 0 12 0"/>"#);

/// `luggage`.
pub const LUGGAGE: Icon = lucide!("luggage", r#"<path d="M6 20a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2"/><path d="M8 18V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v14"/><path d="M10 20h4"/><circle cx="16" cy="20" r="2"/><circle cx="8" cy="20" r="2"/>"#);

/// `magnet`.
pub const MAGNET: Icon = lucide!("magnet", r#"<path d="m12 15 4 4"/><path d="M2.352 10.648a1.205 1.205 0 0 0 0 1.704l2.296 2.296a1.205 1.205 0 0 0 1.704 0l6.029-6.029a1 1 0 1 1 3 3l-6.029 6.029a1.205 1.205 0 0 0 0 1.704l2.296 2.296a1.205 1.205 0 0 0 1.704 0l6.365-6.367A1 1 0 0 0 8.716 4.282z"/><path d="m5 8 4 4"/>"#);

/// `mail`.
pub const MAIL: Icon = lucide!("mail", r#"<path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7"/><rect x="2" y="4" width="20" height="16" rx="2"/>"#);

/// `mail-badge`.
pub const MAIL_BADGE: Icon = lucide!("mail-badge", r#"<path d="M22 7.7V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8.25"/><path d="M12 12.996a1.94 1.94 0 0 1-1.03-.296L2 7"/><path d="m20.69 16.479 1.29 4.88a.5.5 0 0 1-.698.591l-1.843-.849a1 1 0 0 0-.879.001l-1.846.85a.5.5 0 0 1-.692-.593l1.29-4.88"/><circle cx="19" cy="14" r="3"/>"#);

/// `mail-check`.
pub const MAIL_CHECK: Icon = lucide!("mail-check", r#"<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="m16 19 2 2 4-4"/>"#);

/// `mail-clock`.
pub const MAIL_CLOCK: Icon = lucide!("mail-clock", r#"<path d="M16 14v2.2l1.6 1"/><path d="m22 7-.759.484"/><path d="M6.835 20H4a2 2 0 01-2-2V6a2 2 0 012-2h16a2 2 0 012 2v2"/><path d="M7.605 10.567 2 7"/><circle cx="16" cy="16" r="6"/>"#);

/// `mail-dot`.
pub const MAIL_DOT: Icon = lucide!("mail-dot", r#"<path d="M12.674 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4.675"/><path d="m2 7 9 5.701a2 2 0 0 0 2 0l1.084-.686"/><circle cx="19" cy="7" r="3"/>"#);

/// `mail-exclamation-point`.
pub const MAIL_EXCLAMATION_POINT: Icon = lucide!("mail-exclamation-point", r#"<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="M20 14v4"/><path d="M20 22v.01"/>"#);

/// `mail-minus`.
pub const MAIL_MINUS: Icon = lucide!("mail-minus", r#"<path d="M22 15V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="M16 19h6"/>"#);

/// `mail-open`.
pub const MAIL_OPEN: Icon = lucide!("mail-open", r#"<path d="M21.2 8.4c.5.38.8.97.8 1.6v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 .8-1.6l8-6a2 2 0 0 1 2.4 0l8 6Z"/><path d="m22 10-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 10"/>"#);

/// `mail-pen`.
pub const MAIL_PEN: Icon = lucide!("mail-pen", r#"<path d="M15.363 17.634a2 2 0 00-.506.854l-.837 2.87a.5.5 0 00.62.62l2.87-.837a2 2 0 00.854-.506l3.013-3.009a1 1 0 10-3.004-3.004z"/><path d="M22 10.38V6a2 2 0 00-2-2H4a2 2 0 00-2 2v12a2 2 0 002 2h6.25"/><path d="m22 7-8.991 5.727a2 2 0 01-2.009 0L2 7"/>"#);

/// `mail-plus`.
pub const MAIL_PLUS: Icon = lucide!("mail-plus", r#"<path d="M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="M19 16v6"/><path d="M16 19h6"/>"#);

/// `mail-question-mark`.
pub const MAIL_QUESTION_MARK: Icon = lucide!("mail-question-mark", r#"<path d="M22 10.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h12.5"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="M18 15.28c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"/><path d="M20 22v.01"/>"#);

/// `mail-search`.
pub const MAIL_SEARCH: Icon = lucide!("mail-search", r#"<path d="M22 12.5V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h7.5"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/><path d="M18 21a3 3 0 1 0 0-6 3 3 0 0 0 0 6Z"/><circle cx="18" cy="18" r="3"/><path d="m22 22-1.5-1.5"/>"#);

/// `mail-x`.
pub const MAIL_X: Icon = lucide!("mail-x", r#"<path d="M22 12.532V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h8.792"/><path d="m22 7-8.991 5.727a2 2 0 0 1-2.009 0L2 7"/><path d="m16.5 16.5 5 5"/><path d="m21.5 16.5-5 5"/>"#);

/// `mailbox`.
pub const MAILBOX: Icon = lucide!("mailbox", r#"<path d="M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z"/><polyline points="15,9 18,9 18,11"/><path d="M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2"/><line x1="6" x2="7" y1="10" y2="10"/>"#);

/// `mails`.
pub const MAILS: Icon = lucide!("mails", r#"<path d="M17 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 1-1.732"/><path d="m22 5.5-6.419 4.179a2 2 0 0 1-2.162 0L7 5.5"/><rect x="7" y="3" width="15" height="12" rx="2"/>"#);

/// `mango`.
pub const MANGO: Icon = lucide!("mango", r#"<path d="m17 6 3-3"/><path d="M7 22a4 4 0 01-2.62-7.023A2 2 0 005 13.383V10a8 8 0 1116 0A12 12 0 019 22z"/>"#);

/// `map`.
pub const MAP: Icon = lucide!("map", r#"<path d="M14.106 5.553a2 2 0 0 0 1.788 0l3.659-1.83A1 1 0 0 1 21 4.619v12.764a1 1 0 0 1-.553.894l-4.553 2.277a2 2 0 0 1-1.788 0l-4.212-2.106a2 2 0 0 0-1.788 0l-3.659 1.83A1 1 0 0 1 3 19.381V6.618a1 1 0 0 1 .553-.894l4.553-2.277a2 2 0 0 1 1.788 0z"/><path d="M15 5.764v15"/><path d="M9 3.236v15"/>"#);

/// `map-minus`.
pub const MAP_MINUS: Icon = lucide!("map-minus", r#"<path d="m11 19-1.106-.552a2 2 0 0 0-1.788 0l-3.659 1.83A1 1 0 0 1 3 19.381V6.618a1 1 0 0 1 .553-.894l4.553-2.277a2 2 0 0 1 1.788 0l4.212 2.106a2 2 0 0 0 1.788 0l3.659-1.83A1 1 0 0 1 21 4.619V14"/><path d="M15 5.764V14"/><path d="M21 18h-6"/><path d="M9 3.236v15"/>"#);

/// `map-pin`.
pub const MAP_PIN: Icon = lucide!("map-pin", r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><circle cx="12" cy="10" r="3"/>"#);

/// `map-pin-check`.
pub const MAP_PIN_CHECK: Icon = lucide!("map-pin-check", r#"<path d="M19.43 12.935c.357-.967.57-1.955.57-2.935a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 32.197 32.197 0 0 0 .813-.728"/><circle cx="12" cy="10" r="3"/><path d="m16 18 2 2 4-4"/>"#);

/// `map-pin-check-inside`.
pub const MAP_PIN_CHECK_INSIDE: Icon = lucide!("map-pin-check-inside", r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><path d="m9 10 2 2 4-4"/>"#);

/// `map-pin-house`.
pub const MAP_PIN_HOUSE: Icon = lucide!("map-pin-house", r#"<path d="M15 22a1 1 0 0 1-1-1v-4a1 1 0 0 1 .445-.832l3-2a1 1 0 0 1 1.11 0l3 2A1 1 0 0 1 22 17v4a1 1 0 0 1-1 1z"/><path d="M18 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 .601.2"/><path d="M18 22v-3"/><circle cx="10" cy="10" r="3"/>"#);

/// `map-pin-minus`.
pub const MAP_PIN_MINUS: Icon = lucide!("map-pin-minus", r#"<path d="M18.977 14C19.6 12.701 20 11.343 20 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 32 32 0 0 0 .824-.738"/><circle cx="12" cy="10" r="3"/><path d="M16 18h6"/>"#);

/// `map-pin-minus-inside`.
pub const MAP_PIN_MINUS_INSIDE: Icon = lucide!("map-pin-minus-inside", r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><path d="M9 10h6"/>"#);

/// `map-pin-off`.
pub const MAP_PIN_OFF: Icon = lucide!("map-pin-off", r#"<path d="M12.75 7.09a3 3 0 0 1 2.16 2.16"/><path d="M17.072 17.072c-1.634 2.17-3.527 3.912-4.471 4.727a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 1.432-4.568"/><path d="m2 2 20 20"/><path d="M8.475 2.818A8 8 0 0 1 20 10c0 1.183-.31 2.377-.81 3.533"/><path d="M9.13 9.13a3 3 0 0 0 3.74 3.74"/>"#);

/// `map-pin-pen`.
pub const MAP_PIN_PEN: Icon = lucide!("map-pin-pen", r#"<path d="M17.97 9.304A8 8 0 0 0 2 10c0 4.69 4.887 9.562 7.022 11.468"/><path d="M21.378 16.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/><circle cx="10" cy="10" r="3"/>"#);

/// `map-pin-plus`.
pub const MAP_PIN_PLUS: Icon = lucide!("map-pin-plus", r#"<path d="M19.914 11.105A7.298 7.298 0 0 0 20 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 32 32 0 0 0 .824-.738"/><circle cx="12" cy="10" r="3"/><path d="M16 18h6"/><path d="M19 15v6"/>"#);

/// `map-pin-plus-inside`.
pub const MAP_PIN_PLUS_INSIDE: Icon = lucide!("map-pin-plus-inside", r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><path d="M12 7v6"/><path d="M9 10h6"/>"#);

/// `map-pin-search`.
pub const MAP_PIN_SEARCH: Icon = lucide!("map-pin-search", r#"<path d="M 12.248 21.969 a 1 1 0 0 1 -0.849 -0.17 C 9.539 20.193 4 14.993 4 10 a 8 8 0 0 1 16 0 C 20 10.42 19.961 10.841 19.888 11.262"/><path d="m22 22-1.88-1.88"/><circle cx="12" cy="10" r="3"/><circle cx="18" cy="18" r="3"/>"#);

/// `map-pin-x`.
pub const MAP_PIN_X: Icon = lucide!("map-pin-x", r#"<path d="M19.752 11.901A7.78 7.78 0 0 0 20 10a8 8 0 0 0-16 0c0 4.993 5.539 10.193 7.399 11.799a1 1 0 0 0 1.202 0 19 19 0 0 0 .09-.077"/><circle cx="12" cy="10" r="3"/><path d="m21.5 15.5-5 5"/><path d="m21.5 20.5-5-5"/>"#);

/// `map-pin-x-inside`.
pub const MAP_PIN_X_INSIDE: Icon = lucide!("map-pin-x-inside", r#"<path d="M20 10c0 4.993-5.539 10.193-7.399 11.799a1 1 0 0 1-1.202 0C9.539 20.193 4 14.993 4 10a8 8 0 0 1 16 0"/><path d="m14.5 7.5-5 5"/><path d="m9.5 7.5 5 5"/>"#);

/// `map-pinned`.
pub const MAP_PINNED: Icon = lucide!("map-pinned", r#"<path d="M18 8c0 3.613-3.869 7.429-5.393 8.795a1 1 0 01-1.214 0C9.87 15.429 6 11.613 6 8a6 6 0 0112 0"/><path d="M4.474 15h-.197a1 1 0 00-.969.753l-1.097 4.35a1.5 1.5 0 001.444 1.898L20.344 22a1.5 1.5 0 001.446-1.897l-1.098-4.35a1 1 0 00-.969-.753h-.197"/><circle cx="12" cy="8" r="2"/>"#);

/// `map-plus`.
pub const MAP_PLUS: Icon = lucide!("map-plus", r#"<path d="m11 19-1.106-.552a2 2 0 0 0-1.788 0l-3.659 1.83A1 1 0 0 1 3 19.381V6.618a1 1 0 0 1 .553-.894l4.553-2.277a2 2 0 0 1 1.788 0l4.212 2.106a2 2 0 0 0 1.788 0l3.659-1.83A1 1 0 0 1 21 4.619V12"/><path d="M15 5.764V12"/><path d="M18 15v6"/><path d="M21 18h-6"/><path d="M9 3.236v15"/>"#);

/// `margin`.
pub const MARGIN: Icon = lucide!("margin", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M21 6v12"/><path d="M3 6v12"/><path d="M6 21h12"/><path d="M6 3h12"/>"#);

/// `margin-bottom`.
pub const MARGIN_BOTTOM: Icon = lucide!("margin-bottom", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M3 21h18"/>"#);

/// `margin-horizontal`.
pub const MARGIN_HORIZONTAL: Icon = lucide!("margin-horizontal", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M21 3v18"/><path d="M3 3v18"/>"#);

/// `margin-left`.
pub const MARGIN_LEFT: Icon = lucide!("margin-left", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M3 3v18"/>"#);

/// `margin-right`.
pub const MARGIN_RIGHT: Icon = lucide!("margin-right", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M21 3v18"/>"#);

/// `margin-top`.
pub const MARGIN_TOP: Icon = lucide!("margin-top", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M3 3h18"/>"#);

/// `margin-vertical`.
pub const MARGIN_VERTICAL: Icon = lucide!("margin-vertical", r#"<rect x="9" y="9" width="6" height="6" rx="1"/><path d="M3 21h18"/><path d="M3 3h18"/>"#);

/// `mars`.
pub const MARS: Icon = lucide!("mars", r#"<path d="M16 3h5v5"/><path d="m21 3-6.75 6.75"/><circle cx="10" cy="14" r="6"/>"#);

/// `mars-stroke`.
pub const MARS_STROKE: Icon = lucide!("mars-stroke", r#"<path d="m14 6 4 4"/><path d="M17 3h4v4"/><path d="m21 3-7.75 7.75"/><circle cx="9" cy="15" r="6"/>"#);

/// `martini`.
pub const MARTINI: Icon = lucide!("martini", r#"<path d="M12 12 4.207 4.207A.707.707 0 0 1 4.707 3h14.586a.707.707 0 0 1 .5 1.207z"/><path d="M12 12v10"/><path d="M7 22h10"/>"#);

/// `maximize`.
pub const MAXIMIZE: Icon = lucide!("maximize", r#"<path d="M8 3H5a2 2 0 0 0-2 2v3"/><path d="M21 8V5a2 2 0 0 0-2-2h-3"/><path d="M3 16v3a2 2 0 0 0 2 2h3"/><path d="M16 21h3a2 2 0 0 0 2-2v-3"/>"#);

/// `maximize-2`.
pub const MAXIMIZE_2: Icon = lucide!("maximize-2", r#"<path d="M15 3h6v6"/><path d="m21 3-7 7"/><path d="m3 21 7-7"/><path d="M9 21H3v-6"/>"#);

/// `medal`.
pub const MEDAL: Icon = lucide!("medal", r#"<path d="M7.21 15 2.66 7.14a2 2 0 0 1 .13-2.2L4.4 2.8A2 2 0 0 1 6 2h12a2 2 0 0 1 1.6.8l1.6 2.14a2 2 0 0 1 .14 2.2L16.79 15"/><path d="M11 12 5.12 2.2"/><path d="m13 12 5.88-9.8"/><path d="M8 7h8"/><circle cx="12" cy="17" r="5"/><path d="M12 18v-2h-.5"/>"#);

/// `megaphone`.
pub const MEGAPHONE: Icon = lucide!("megaphone", r#"<path d="M11 6a13 13 0 0 0 8.4-2.8A1 1 0 0 1 21 4v12a1 1 0 0 1-1.6.8A13 13 0 0 0 11 14H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z"/><path d="M6 14a12 12 0 0 0 2.4 7.2 2 2 0 0 0 3.2-2.4A8 8 0 0 1 10 14"/><path d="M8 6v8"/>"#);

/// `megaphone-off`.
pub const MEGAPHONE_OFF: Icon = lucide!("megaphone-off", r#"<path d="M11.636 6A13 13 0 0 0 19.4 3.2 1 1 0 0 1 21 4v11.344"/><path d="M14.378 14.357A13 13 0 0 0 11 14H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h1"/><path d="m2 2 20 20"/><path d="M6 14a12 12 0 0 0 2.4 7.2 2 2 0 0 0 3.2-2.4A8 8 0 0 1 10 14"/><path d="M8 8v6"/>"#);

/// `memory-stick`.
pub const MEMORY_STICK: Icon = lucide!("memory-stick", r#"<path d="M12 12v-2"/><path d="M12 18v-2"/><path d="M16 12v-2"/><path d="M16 18v-2"/><path d="M2 11h1.5"/><path d="M20 18v-2"/><path d="M20.5 11H22"/><path d="M4 18v-2"/><path d="M8 12v-2"/><path d="M8 18v-2"/><rect x="2" y="6" width="20" height="10" rx="2"/>"#);

/// `menu`.
pub const MENU: Icon = lucide!("menu", r#"<path d="M4 5h16"/><path d="M4 12h16"/><path d="M4 19h16"/>"#);

/// `merge`.
pub const MERGE: Icon = lucide!("merge", r#"<path d="m8 6 4-4 4 4"/><path d="M12 2v10.3a4 4 0 0 1-1.172 2.872L4 22"/><path d="m20 22-5-5"/>"#);

/// `message-circle`.
pub const MESSAGE_CIRCLE: Icon = lucide!("message-circle", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/>"#);

/// `message-circle-check`.
pub const MESSAGE_CIRCLE_CHECK: Icon = lucide!("message-circle-check", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="m16 9-5.5 5.5L8 12"/>"#);

/// `message-circle-code`.
pub const MESSAGE_CIRCLE_CODE: Icon = lucide!("message-circle-code", r#"<path d="m10 9-3 3 3 3"/><path d="m14 15 3-3-3-3"/><path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/>"#);

/// `message-circle-dashed`.
pub const MESSAGE_CIRCLE_DASHED: Icon = lucide!("message-circle-dashed", r#"<path d="M10.1 2.182a10 10 0 0 1 3.8 0"/><path d="M13.9 21.818a10 10 0 0 1-3.8 0"/><path d="M17.609 3.72a10 10 0 0 1 2.69 2.7"/><path d="M2.182 13.9a10 10 0 0 1 0-3.8"/><path d="M20.28 17.61a10 10 0 0 1-2.7 2.69"/><path d="M21.818 10.1a10 10 0 0 1 0 3.8"/><path d="M3.721 6.391a10 10 0 0 1 2.7-2.69"/><path d="m6.163 21.117-2.906.85a1 1 0 0 1-1.236-1.169l.965-2.98"/>"#);

/// `message-circle-dashed-check`.
pub const MESSAGE_CIRCLE_DASHED_CHECK: Icon = lucide!("message-circle-dashed-check", r#"<path d="M10.1 2.182a10 10 0 013.8 0"/><path d="M13.9 21.818a10 10 0 01-3.8 0"/><path d="M17.609 3.72a10 10 0 012.69 2.7"/><path d="M2.182 13.9a10 10 0 010-3.8"/><path d="M20.28 17.61a10 10 0 01-2.7 2.69"/><path d="M21.818 10.1a10 10 0 010 3.8"/><path d="M3.721 6.391a10 10 0 012.7-2.69"/><path d="m6.163 21.117-2.906.85a1 1 0 01-1.236-1.169l.965-2.98"/><path d="m16 9-5.5 5.5L8 12"/>"#);

/// `message-circle-exclamation-point`.
pub const MESSAGE_CIRCLE_EXCLAMATION_POINT: Icon = lucide!("message-circle-exclamation-point", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="M12 8v4"/><path d="M12 16h.01"/>"#);

/// `message-circle-heart`.
pub const MESSAGE_CIRCLE_HEART: Icon = lucide!("message-circle-heart", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="M7.828 13.07A3 3 0 0 1 12 8.764a3 3 0 0 1 5.004 2.224 3 3 0 0 1-.832 2.083l-3.447 3.62a1 1 0 0 1-1.45-.001z"/>"#);

/// `message-circle-more`.
pub const MESSAGE_CIRCLE_MORE: Icon = lucide!("message-circle-more", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="M8 12h.01"/><path d="M12 12h.01"/><path d="M16 12h.01"/>"#);

/// `message-circle-off`.
pub const MESSAGE_CIRCLE_OFF: Icon = lucide!("message-circle-off", r#"<path d="m2 2 20 20"/><path d="M4.93 4.929a10 10 0 0 0-1.938 11.412 2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 0 0 11.302-1.989"/><path d="M8.35 2.69A10 10 0 0 1 21.3 15.65"/>"#);

/// `message-circle-plus`.
pub const MESSAGE_CIRCLE_PLUS: Icon = lucide!("message-circle-plus", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="M8 12h8"/><path d="M12 8v8"/>"#);

/// `message-circle-question-mark`.
pub const MESSAGE_CIRCLE_QUESTION_MARK: Icon = lucide!("message-circle-question-mark", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><path d="M12 17h.01"/>"#);

/// `message-circle-reply`.
pub const MESSAGE_CIRCLE_REPLY: Icon = lucide!("message-circle-reply", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="m10 15-3-3 3-3"/><path d="M7 12h8a2 2 0 0 1 2 2v1"/>"#);

/// `message-circle-text`.
pub const MESSAGE_CIRCLE_TEXT: Icon = lucide!("message-circle-text", r#"<path d="M3.086 17.509a2 2 0 00-.094-1.167 10 10 0 114.777 4.719 2 2 0 00-1.099-.092l-3.413.998a1 1 0 01-1.236-1.168z"/><path d="M8 12h9"/><path d="M8 16h5"/><path d="M8 8h7"/>"#);

/// `message-circle-x`.
pub const MESSAGE_CIRCLE_X: Icon = lucide!("message-circle-x", r#"<path d="M2.992 16.342a2 2 0 0 1 .094 1.167l-1.065 3.29a1 1 0 0 0 1.236 1.168l3.413-.998a2 2 0 0 1 1.099.092 10 10 0 1 0-4.777-4.719"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#);

/// `message-square`.
pub const MESSAGE_SQUARE: Icon = lucide!("message-square", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/>"#);

/// `message-square-check`.
pub const MESSAGE_SQUARE_CHECK: Icon = lucide!("message-square-check", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.7.7 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="m9 11 2 2 4-4"/>"#);

/// `message-square-code`.
pub const MESSAGE_SQUARE_CODE: Icon = lucide!("message-square-code", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="m10 8-3 3 3 3"/><path d="m14 14 3-3-3-3"/>"#);

/// `message-square-dashed`.
pub const MESSAGE_SQUARE_DASHED: Icon = lucide!("message-square-dashed", r#"<path d="M14 3h2"/><path d="M16 19h-2"/><path d="M2 12v-2"/><path d="M2 16v5.286a.71.71 0 0 0 1.212.502l1.149-1.149"/><path d="M20 19a2 2 0 0 0 2-2v-1"/><path d="M22 10v2"/><path d="M22 6V5a2 2 0 0 0-2-2"/><path d="M4 3a2 2 0 0 0-2 2v1"/><path d="M8 19h2"/><path d="M8 3h2"/>"#);

/// `message-square-diff`.
pub const MESSAGE_SQUARE_DIFF: Icon = lucide!("message-square-diff", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M10 15h4"/><path d="M10 9h4"/><path d="M12 7v4"/>"#);

/// `message-square-dot`.
pub const MESSAGE_SQUARE_DOT: Icon = lucide!("message-square-dot", r#"<path d="M12.7 3H4a2 2 0 0 0-2 2v16.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H20a2 2 0 0 0 2-2v-4.7"/><circle cx="19" cy="6" r="3"/>"#);

/// `message-square-exclamation-point`.
pub const MESSAGE_SQUARE_EXCLAMATION_POINT: Icon = lucide!("message-square-exclamation-point", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M12 15h.01"/><path d="M12 7v4"/>"#);

/// `message-square-heart`.
pub const MESSAGE_SQUARE_HEART: Icon = lucide!("message-square-heart", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M7.5 9.5c0 .687.265 1.383.697 1.844l3.009 3.264a1.14 1.14 0 0 0 .407.314 1 1 0 0 0 .783-.004 1.14 1.14 0 0 0 .398-.31l3.008-3.264A2.77 2.77 0 0 0 16.5 9.5 2.5 2.5 0 0 0 12 8a2.5 2.5 0 0 0-4.5 1.5"/>"#);

/// `message-square-lock`.
pub const MESSAGE_SQUARE_LOCK: Icon = lucide!("message-square-lock", r#"<path d="M22 8.5V5a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v16.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H10"/><path d="M20 15v-2a2 2 0 0 0-4 0v2"/><rect x="14" y="15" width="8" height="5" rx="1"/>"#);

/// `message-square-more`.
pub const MESSAGE_SQUARE_MORE: Icon = lucide!("message-square-more", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M12 11h.01"/><path d="M16 11h.01"/><path d="M8 11h.01"/>"#);

/// `message-square-off`.
pub const MESSAGE_SQUARE_OFF: Icon = lucide!("message-square-off", r#"<path d="M19 19H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.7.7 0 0 1 2 21.286V5a2 2 0 0 1 1.184-1.826"/><path d="m2 2 20 20"/><path d="M8.656 3H20a2 2 0 0 1 2 2v11.344"/>"#);

/// `message-square-plus`.
pub const MESSAGE_SQUARE_PLUS: Icon = lucide!("message-square-plus", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M12 8v6"/><path d="M9 11h6"/>"#);

/// `message-square-quote`.
pub const MESSAGE_SQUARE_QUOTE: Icon = lucide!("message-square-quote", r#"<path d="M14 14a2 2 0 0 0 2-2V8h-2"/><path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M8 14a2 2 0 0 0 2-2V8H8"/>"#);

/// `message-square-reply`.
pub const MESSAGE_SQUARE_REPLY: Icon = lucide!("message-square-reply", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="m10 8-3 3 3 3"/><path d="M17 14v-1a2 2 0 0 0-2-2H7"/>"#);

/// `message-square-share`.
pub const MESSAGE_SQUARE_SHARE: Icon = lucide!("message-square-share", r#"<path d="M12 3H4a2 2 0 0 0-2 2v16.286a.71.71 0 0 0 1.212.502l2.202-2.202A2 2 0 0 1 6.828 19H20a2 2 0 0 0 2-2v-4"/><path d="M16 3h6v6"/><path d="m16 9 6-6"/>"#);

/// `message-square-text`.
pub const MESSAGE_SQUARE_TEXT: Icon = lucide!("message-square-text", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="M7 11h10"/><path d="M7 15h6"/><path d="M7 7h8"/>"#);

/// `message-square-x`.
pub const MESSAGE_SQUARE_X: Icon = lucide!("message-square-x", r#"<path d="M22 17a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 21.286V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2z"/><path d="m14.5 8.5-5 5"/><path d="m9.5 8.5 5 5"/>"#);

/// `messages-circle`.
pub const MESSAGES_CIRCLE: Icon = lucide!("messages-circle", r#"<path d="M19.95 10.05a7 7 0 011.412 7.872 1 1 0 00-.058.787l.675 2.089a1 1 0 01-1.236 1.168l-2.155-.631a1 1 0 00-.745.06 7 7 0 01-7.793-1.445"/><path d="M2.696 12.708a1 1 0 00-.058-.785 7 7 0 113.518 3.473 1 1 0 00-.744-.061l-2.155.63a1 1 0 01-1.236-1.167z"/>"#);

/// `messages-square`.
pub const MESSAGES_SQUARE: Icon = lucide!("messages-square", r#"<path d="M16 10a2 2 0 0 1-2 2H6.828a2 2 0 0 0-1.414.586l-2.202 2.202A.71.71 0 0 1 2 14.286V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z"/><path d="M20 9a2 2 0 0 1 2 2v10.286a.71.71 0 0 1-1.212.502l-2.202-2.202A2 2 0 0 0 17.172 19H10a2 2 0 0 1-2-2v-1"/>"#);

/// `metronome`.
pub const METRONOME: Icon = lucide!("metronome", r#"<path d="M12 11.4V9.1"/><path d="m12 17 6.59-6.59"/><path d="m15.05 5.7-.218-.691a3 3 0 0 0-5.663 0L4.418 19.695A1 1 0 0 0 5.37 21h13.253a1 1 0 0 0 .951-1.31L18.45 16.2"/><circle cx="20" cy="9" r="2"/>"#);

/// `mic`.
pub const MIC: Icon = lucide!("mic", r#"<path d="M12 19v3"/><path d="M19 10v2a7 7 0 0 1-14 0v-2"/><rect x="9" y="2" width="6" height="13" rx="3"/>"#);

/// `mic-audio-lines`.
pub const MIC_AUDIO_LINES: Icon = lucide!("mic-audio-lines", r#"<path d="M10 3v2.341"/><path d="M12 17v4"/><path d="M14 5v.341"/><path d="M18 5v13"/><path d="M2 10v3"/><path d="M22 10v3"/><path d="M6 6v11"/><path d="M9 21h6"/><rect width="4" height="8" x="10" y="9" rx="2"/>"#);

/// `mic-off`.
pub const MIC_OFF: Icon = lucide!("mic-off", r#"<path d="M12 19v3"/><path d="M15 9.34V5a3 3 0 0 0-5.68-1.33"/><path d="M16.95 16.95A7 7 0 0 1 5 12v-2"/><path d="M18.89 13.23A7 7 0 0 0 19 12v-2"/><path d="m2 2 20 20"/><path d="M9 9v3a3 3 0 0 0 5.12 2.12"/>"#);

/// `mic-signal`.
pub const MIC_SIGNAL: Icon = lucide!("mic-signal", r#"<path d="M12 17v4"/><path d="M18 11a6 6 0 00-3-5.197"/><path d="M2 11a10 10 0 015-8.662"/><path d="M22 11a10 10 0 00-5-8.662"/><path d="M6 11a6 6 0 013-5.197"/><path d="M9 21h6"/><rect x="10" y="9" width="4" height="8" rx="2"/>"#);

/// `mic-vocal`.
pub const MIC_VOCAL: Icon = lucide!("mic-vocal", r#"<path d="m11 7.601-5.994 8.19a1 1 0 0 0 .1 1.298l.817.818a1 1 0 0 0 1.314.087L15.09 12"/><path d="M16.5 21.174C15.5 20.5 14.372 20 13 20c-2.058 0-3.928 2.356-6 2-2.072-.356-2.775-3.369-1.5-4.5"/><circle cx="16" cy="7" r="5"/>"#);

/// `microchip`.
pub const MICROCHIP: Icon = lucide!("microchip", r#"<path d="M10 12h4"/><path d="M10 17h4"/><path d="M10 7h4"/><path d="M18 12h2"/><path d="M18 18h2"/><path d="M18 6h2"/><path d="M4 12h2"/><path d="M4 18h2"/><path d="M4 6h2"/><rect x="6" y="2" width="12" height="20" rx="2"/>"#);

/// `microscope`.
pub const MICROSCOPE: Icon = lucide!("microscope", r#"<path d="M6 18h8"/><path d="M3 22h18"/><path d="M14 22a7 7 0 1 0 0-14h-1"/><path d="M9 14h2"/><path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z"/><path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"/>"#);

/// `microwave`.
pub const MICROWAVE: Icon = lucide!("microwave", r#"<rect width="20" height="15" x="2" y="4" rx="2"/><rect width="8" height="7" x="6" y="8" rx="1"/><path d="M18 8v7"/><path d="M6 19v2"/><path d="M18 19v2"/>"#);

/// `midi-port`.
pub const MIDI_PORT: Icon = lucide!("midi-port", r#"<path d="M12 18h.01"/><path d="M15 2.458V5a1 1 0 01-1 1h-4a1 1 0 01-1-1V2.458"/><path d="M16 16h.01"/><path d="M18 12h.01"/><path d="M6 12h.01"/><path d="M8 16h.01"/><circle cx="12" cy="12" r="10"/>"#);

/// `milestone`.
pub const MILESTONE: Icon = lucide!("milestone", r#"<path d="M12 13v8"/><path d="M12 3v3"/><path d="M18.172 6a2 2 0 0 1 1.414.586l2.06 2.06a1.207 1.207 0 0 1 0 1.708l-2.06 2.06a2 2 0 0 1-1.414.586H4a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1z"/>"#);

/// `milk`.
pub const MILK: Icon = lucide!("milk", r#"<path d="M8 2h8"/><path d="M9 2v2.789a4 4 0 0 1-.672 2.219l-.656.984A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-9.789a4 4 0 0 0-.672-2.219l-.656-.984A4 4 0 0 1 15 4.788V2"/><path d="M7 15a6.472 6.472 0 0 1 5 0 6.47 6.47 0 0 0 5 0"/>"#);

/// `milk-off`.
pub const MILK_OFF: Icon = lucide!("milk-off", r#"<path d="M8 2h8"/><path d="M9 2v1.343M15 2v2.789a4 4 0 0 0 .672 2.219l.656.984a4 4 0 0 1 .672 2.22v1.131M7.8 7.8l-.128.192A4 4 0 0 0 7 10.212V20a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2v-3"/><path d="M7 15a6.47 6.47 0 0 1 5 0 6.472 6.472 0 0 0 3.435.435"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `minimize`.
pub const MINIMIZE: Icon = lucide!("minimize", r#"<path d="M8 3v3a2 2 0 0 1-2 2H3"/><path d="M21 8h-3a2 2 0 0 1-2-2V3"/><path d="M3 16h3a2 2 0 0 1 2 2v3"/><path d="M16 21v-3a2 2 0 0 1 2-2h3"/>"#);

/// `minimize-2`.
pub const MINIMIZE_2: Icon = lucide!("minimize-2", r#"<path d="m14 10 7-7"/><path d="M20 10h-6V4"/><path d="m3 21 7-7"/><path d="M4 14h6v6"/>"#);

/// `minus`.
pub const MINUS: Icon = lucide!("minus", r#"<path d="M5 12h14"/>"#);

/// `mirror-rectangular`.
pub const MIRROR_RECTANGULAR: Icon = lucide!("mirror-rectangular", r#"<path d="M11 6 8 9"/><path d="m16 7-8 8"/><rect x="4" y="2" width="16" height="20" rx="2"/>"#);

/// `mirror-round`.
pub const MIRROR_ROUND: Icon = lucide!("mirror-round", r#"<path d="M10 6.6 8.6 8"/><path d="M12 18v4"/><path d="M15 7.5 9.5 13"/><path d="M7 22h10"/><circle cx="12" cy="10" r="8"/>"#);

/// `monitor`.
pub const MONITOR: Icon = lucide!("monitor", r#"<rect width="20" height="14" x="2" y="3" rx="2"/><line x1="8" x2="16" y1="21" y2="21"/><line x1="12" x2="12" y1="17" y2="21"/>"#);

/// `monitor-activity`.
pub const MONITOR_ACTIVITY: Icon = lucide!("monitor-activity", r#"<path d="M12 17v4"/><path d="M18 10h-2.315a1 1 0 00-.932.64l-.787 2.034a.5.5 0 01-.932 0l-2.068-5.348a.5.5 0 00-.932 0l-.787 2.035a1 1 0 01-.932.639H6"/><path d="M8 21h8"/><rect x="2" y="3" width="20" height="14" rx="2"/>"#);

/// `monitor-check`.
pub const MONITOR_CHECK: Icon = lucide!("monitor-check", r#"<path d="m9 10 2 2 4-4"/><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M12 17v4"/><path d="M8 21h8"/>"#);

/// `monitor-cloud`.
pub const MONITOR_CLOUD: Icon = lucide!("monitor-cloud", r#"<path d="M11 13a3 3 0 1 1 2.83-4H14a2 2 0 0 1 0 4z"/><path d="M12 17v4"/><path d="M8 21h8"/><rect x="2" y="3" width="20" height="14" rx="2"/>"#);

/// `monitor-cog`.
pub const MONITOR_COG: Icon = lucide!("monitor-cog", r#"<path d="M12 17v4"/><path d="m14.305 7.53.923-.382"/><path d="m15.228 4.852-.923-.383"/><path d="m16.852 3.228-.383-.924"/><path d="m16.852 8.772-.383.923"/><path d="m19.148 3.228.383-.924"/><path d="m19.53 9.696-.382-.924"/><path d="m20.772 4.852.924-.383"/><path d="m20.772 7.148.924.383"/><path d="M22 13v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h7"/><path d="M8 21h8"/><circle cx="18" cy="6" r="3"/>"#);

/// `monitor-dot`.
pub const MONITOR_DOT: Icon = lucide!("monitor-dot", r#"<path d="M12 17v4"/><path d="M22 12.307V15a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h8.693"/><path d="M8 21h8"/><circle cx="19" cy="6" r="3"/>"#);

/// `monitor-down`.
pub const MONITOR_DOWN: Icon = lucide!("monitor-down", r#"<path d="M12 13V7"/><path d="m15 10-3 3-3-3"/><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M12 17v4"/><path d="M8 21h8"/>"#);

/// `monitor-off`.
pub const MONITOR_OFF: Icon = lucide!("monitor-off", r#"<path d="M12 17v4"/><path d="M17 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 1.184-1.826"/><path d="m2 2 20 20"/><path d="M8 21h8"/><path d="M8.656 3H20a2 2 0 0 1 2 2v10a2 2 0 0 1-.293 1.042"/>"#);

/// `monitor-pause`.
pub const MONITOR_PAUSE: Icon = lucide!("monitor-pause", r#"<path d="M10 13V7"/><path d="M14 13V7"/><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M12 17v4"/><path d="M8 21h8"/>"#);

/// `monitor-pc`.
pub const MONITOR_PC: Icon = lucide!("monitor-pc", r#"<path d="M10 15H4a2 2 0 01-2-2V7a2 2 0 012-2h6"/><path d="M10 19H5"/><path d="M14 11h8"/><path d="M14 7h8"/><path d="M18 17h.01"/><path d="M9 19v-4"/><rect x="14" y="3" width="8" height="18" rx="1"/>"#);

/// `monitor-play`.
pub const MONITOR_PLAY: Icon = lucide!("monitor-play", r#"<path d="M15.033 9.44a.647.647 0 0 1 0 1.12l-4.065 2.352a.645.645 0 0 1-.968-.56V7.648a.645.645 0 0 1 .967-.56z"/><path d="M12 17v4"/><path d="M8 21h8"/><rect x="2" y="3" width="20" height="14" rx="2"/>"#);

/// `monitor-smartphone`.
pub const MONITOR_SMARTPHONE: Icon = lucide!("monitor-smartphone", r#"<path d="M18 8V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h8"/><path d="M10 19v-3.96 3.15"/><path d="M7 19h5"/><rect width="6" height="10" x="16" y="12" rx="2"/>"#);

/// `monitor-speaker`.
pub const MONITOR_SPEAKER: Icon = lucide!("monitor-speaker", r#"<path d="M5.5 20H8"/><path d="M17 9h.01"/><rect width="10" height="16" x="12" y="4" rx="2"/><path d="M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4"/><circle cx="17" cy="15" r="1"/>"#);

/// `monitor-stop`.
pub const MONITOR_STOP: Icon = lucide!("monitor-stop", r#"<path d="M12 17v4"/><path d="M8 21h8"/><rect x="2" y="3" width="20" height="14" rx="2"/><rect x="9" y="7" width="6" height="6" rx="1"/>"#);

/// `monitor-up`.
pub const MONITOR_UP: Icon = lucide!("monitor-up", r#"<path d="m9 10 3-3 3 3"/><path d="M12 13V7"/><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M12 17v4"/><path d="M8 21h8"/>"#);

/// `monitor-x`.
pub const MONITOR_X: Icon = lucide!("monitor-x", r#"<path d="m14.5 12.5-5-5"/><path d="m9.5 12.5 5-5"/><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M12 17v4"/><path d="M8 21h8"/>"#);

/// `moon`.
pub const MOON: Icon = lucide!("moon", r#"<path d="M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"/>"#);

/// `moon-star`.
pub const MOON_STAR: Icon = lucide!("moon-star", r#"<path d="M18 5h4"/><path d="M20 3v4"/><path d="M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401"/>"#);

/// `mop`.
pub const MOP: Icon = lucide!("mop", r#"<path d="M10 22c2.761 0 5-1.79 5-4-4.42 0-4.08-5-8.5-5a1 1 0 100 9za3 3 0 01-3-3"/><path d="M12.5 11.5 22 2"/><path d="m6.98 13.02 2.665-2.664a1.21 1.21 0 011.71 0l2.29 2.288a1.21 1.21 0 010 1.712l-2.088 2.087"/>"#);

/// `mop-sparkles`.
pub const MOP_SPARKLES: Icon = lucide!("mop-sparkles", r#"<path d="M10 22a3 3 0 01-3-3"/><path d="M10 22c2.761 0 5-1.79 5-4-4.42 0-4.08-5-8.5-5a4.501 4.501 0 000 9z"/><path d="M10 3H8"/><path d="M12.5 11.5 22 2"/><path d="M20 13v4"/><path d="M22 15h-4"/><path d="M4 5v4"/><path d="M6 7H2"/><path d="m6.98 13.02 2.665-2.664a1.21 1.21 0 011.71 0l2.29 2.288a1.21 1.21 0 010 1.712l-2.088 2.087"/><path d="M9 2v2"/>"#);

/// `moped`.
pub const MOPED: Icon = lucide!("moped", r#"<path d="m18.16 14.12-2.927-8.447A1 1 0 0 0 14.288 5H13"/><path d="M19 10a7 7 0 0 0-6.928 6"/><path d="M2 10h5.5a3 3 0 0 1 2.747 1.795l1.669 3.803A1 1 0 0 1 11.01 17H8"/><circle cx="19" cy="17" r="3"/><circle cx="5" cy="17" r="3"/>"#);

/// `mosque`.
pub const MOSQUE: Icon = lucide!("mosque", r#"<path d="M12.268 2a2 2 0 003.465 2"/><path d="M14 5 L14 8"/><path d="M16 22v-3a2 2 0 00-4 0v3"/><path d="M21 13c-.662-1.497-1.666-2.753-2.9-3.63C16.825 8.47 15.422 8 14 8s-2.826.47-4.1 1.37C8.668 10.248 7.663 11.504 7 13z"/><path d="M3 9h4"/><path d="M7 22V6a5 5 0 00-2-4 5 5 0 00-2 4v14a2 2 0 002 2h14a2 2 0 002-2v-7"/>"#);

/// `motorbike`.
pub const MOTORBIKE: Icon = lucide!("motorbike", r#"<path d="m18 14-1-3"/><path d="m3 9 6 2a2 2 0 0 1 2-2h2a2 2 0 0 1 1.99 1.81"/><path d="M8 17h3a1 1 0 0 0 1-1 6 6 0 0 1 6-6 1 1 0 0 0 1-1v-.75A5 5 0 0 0 17 5"/><circle cx="19" cy="17" r="3"/><circle cx="5" cy="17" r="3"/>"#);

/// `mountain`.
pub const MOUNTAIN: Icon = lucide!("mountain", r#"<path d="m8 3 4 8 5-5 5 15H2L8 3z"/>"#);

/// `mountain-snow`.
pub const MOUNTAIN_SNOW: Icon = lucide!("mountain-snow", r#"<path d="m8 3 4 8 5-5 5 15H2L8 3z"/><path d="M4.14 15.08c2.62-1.57 5.24-1.43 7.86.42 2.74 1.94 5.49 2 8.23.19"/>"#);

/// `mouse`.
pub const MOUSE: Icon = lucide!("mouse", r#"<rect x="5" y="2" width="14" height="20" rx="7"/><path d="M12 6v4"/>"#);

/// `mouse-left`.
pub const MOUSE_LEFT: Icon = lucide!("mouse-left", r#"<path d="M12 7.318V10"/><path d="M5 10v5a7 7 0 0 0 14 0V9c0-3.527-2.608-6.515-6-7"/><circle cx="7" cy="4" r="2"/>"#);

/// `mouse-off`.
pub const MOUSE_OFF: Icon = lucide!("mouse-off", r#"<path d="M12 6v.343"/><path d="M18.218 18.218A7 7 0 0 1 5 15V9a7 7 0 0 1 .782-3.218"/><path d="M19 13.343V9A7 7 0 0 0 8.56 2.902"/><path d="M22 22 2 2"/>"#);

/// `mouse-pointer`.
pub const MOUSE_POINTER: Icon = lucide!("mouse-pointer", r#"<path d="M12.586 12.586 19 19"/><path d="M3.688 3.037a.497.497 0 0 0-.651.651l6.5 15.999a.501.501 0 0 0 .947-.062l1.569-6.083a2 2 0 0 1 1.448-1.479l6.124-1.579a.5.5 0 0 0 .063-.947z"/>"#);

/// `mouse-pointer-2`.
pub const MOUSE_POINTER_2: Icon = lucide!("mouse-pointer-2", r#"<path d="M4.037 4.688a.495.495 0 0 1 .651-.651l16 6.5a.5.5 0 0 1-.063.947l-6.124 1.58a2 2 0 0 0-1.438 1.435l-1.579 6.126a.5.5 0 0 1-.947.063z"/>"#);

/// `mouse-pointer-2-off`.
pub const MOUSE_POINTER_2_OFF: Icon = lucide!("mouse-pointer-2-off", r#"<path d="m15.55 8.45 5.138 2.087a.5.5 0 0 1-.063.947l-6.124 1.58a2 2 0 0 0-1.438 1.435l-1.579 6.126a.5.5 0 0 1-.947.063L8.45 15.551"/><path d="M22 2 2 22"/><path d="m6.816 11.528-2.779-6.84a.495.495 0 0 1 .651-.651l6.84 2.779"/>"#);

/// `mouse-pointer-ban`.
pub const MOUSE_POINTER_BAN: Icon = lucide!("mouse-pointer-ban", r#"<path d="M2.034 2.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.944L8.204 7.545a1 1 0 0 0-.66.66l-1.066 3.443a.5.5 0 0 1-.944.033z"/><circle cx="16" cy="16" r="6"/><path d="m11.8 11.8 8.4 8.4"/>"#);

/// `mouse-pointer-click`.
pub const MOUSE_POINTER_CLICK: Icon = lucide!("mouse-pointer-click", r#"<path d="M14 4.1 12 6"/><path d="m5.1 8-2.9-.8"/><path d="m6 12-1.9 2"/><path d="M7.2 2.2 8 5.1"/><path d="M9.037 9.69a.498.498 0 0 1 .653-.653l11 4.5a.5.5 0 0 1-.074.949l-4.349 1.041a1 1 0 0 0-.74.739l-1.04 4.35a.5.5 0 0 1-.95.074z"/>"#);

/// `mouse-right`.
pub const MOUSE_RIGHT: Icon = lucide!("mouse-right", r#"<path d="M12 7.318V10"/><path d="M19 10v5a7 7 0 0 1-14 0V9c0-3.527 2.608-6.515 6-7"/><circle cx="17" cy="4" r="2"/>"#);

/// `mouth`.
pub const MOUTH: Icon = lucide!("mouth", r#"<path d="M2 12a50.5 50.5 0 0020 0 1 1 0 00-1-1"/><path d="M2.457 11.159a1 1 0 00-.307 1.369 11.59 11.59 0 0019.7 0 1 1 0 00-.308-1.368c-2.426-1.568-3.65-2.284-5.479-3.644a2.6 2.6 0 00-3.373.208 1 1 0 01-1.38 0 2.62 2.62 0 00-3.373-.208c-1.83 1.36-3.053 2.076-5.48 3.643"/>"#);

/// `mouth-off`.
pub const MOUTH_OFF: Icon = lucide!("mouth-off", r#"<path d="M13.074 7.417a2.6 2.6 0 012.989.099c1.829 1.36 3.053 2.076 5.479 3.644a1 1 0 01.308 1.368 11.6 11.6 0 01-1.617 2.05"/><path d="M2 12a50.5 50.5 0 0010.99.99"/><path d="m2 2 20 20"/><path d="M21 11a1 1 0 011 1 51 51 0 01-3.734.61"/><path d="M7.695 7.695c-1.7 1.247-2.92 1.967-5.238 3.464a1 1 0 00-.307 1.369 11.6 11.6 0 0014.766 4.388"/>"#);

/// `move`.
pub const MOVE: Icon = lucide!("move", r#"<path d="M12 2v20"/><path d="m15 19-3 3-3-3"/><path d="m19 9 3 3-3 3"/><path d="M2 12h20"/><path d="m5 9-3 3 3 3"/><path d="m9 5 3-3 3 3"/>"#);

/// `move-3d`.
pub const MOVE_3D: Icon = lucide!("move-3d", r#"<path d="M5 3v16h16"/><path d="m5 19 6-6"/><path d="m2 6 3-3 3 3"/><path d="m18 16 3 3-3 3"/>"#);

/// `move-diagonal`.
pub const MOVE_DIAGONAL: Icon = lucide!("move-diagonal", r#"<path d="M11 19H5v-6"/><path d="M13 5h6v6"/><path d="M19 5 5 19"/>"#);

/// `move-diagonal-2`.
pub const MOVE_DIAGONAL_2: Icon = lucide!("move-diagonal-2", r#"<path d="M19 13v6h-6"/><path d="M5 11V5h6"/><path d="m5 5 14 14"/>"#);

/// `move-down`.
pub const MOVE_DOWN: Icon = lucide!("move-down", r#"<path d="M8 18L12 22L16 18"/><path d="M12 2V22"/>"#);

/// `move-down-left`.
pub const MOVE_DOWN_LEFT: Icon = lucide!("move-down-left", r#"<path d="M11 19H5V13"/><path d="M19 5L5 19"/>"#);

/// `move-down-right`.
pub const MOVE_DOWN_RIGHT: Icon = lucide!("move-down-right", r#"<path d="M19 13V19H13"/><path d="M5 5L19 19"/>"#);

/// `move-horizontal`.
pub const MOVE_HORIZONTAL: Icon = lucide!("move-horizontal", r#"<path d="m18 8 4 4-4 4"/><path d="M2 12h20"/><path d="m6 8-4 4 4 4"/>"#);

/// `move-left`.
pub const MOVE_LEFT: Icon = lucide!("move-left", r#"<path d="M6 8L2 12L6 16"/><path d="M2 12H22"/>"#);

/// `move-right`.
pub const MOVE_RIGHT: Icon = lucide!("move-right", r#"<path d="M18 8L22 12L18 16"/><path d="M2 12H22"/>"#);

/// `move-up`.
pub const MOVE_UP: Icon = lucide!("move-up", r#"<path d="M8 6L12 2L16 6"/><path d="M12 2V22"/>"#);

/// `move-up-left`.
pub const MOVE_UP_LEFT: Icon = lucide!("move-up-left", r#"<path d="M5 11V5H11"/><path d="M5 5L19 19"/>"#);

/// `move-up-right`.
pub const MOVE_UP_RIGHT: Icon = lucide!("move-up-right", r#"<path d="M13 5H19V11"/><path d="M19 5L5 19"/>"#);

/// `move-vertical`.
pub const MOVE_VERTICAL: Icon = lucide!("move-vertical", r#"<path d="M12 2v20"/><path d="m8 18 4 4 4-4"/><path d="m8 6 4-4 4 4"/>"#);

/// `navigation`.
pub const NAVIGATION: Icon = lucide!("navigation", r#"<polygon points="3 11 22 2 13 21 11 13 3 11"/>"#);

/// `navigation-2`.
pub const NAVIGATION_2: Icon = lucide!("navigation-2", r#"<polygon points="12 2 19 21 12 17 5 21 12 2"/>"#);

/// `navigation-2-off`.
pub const NAVIGATION_2_OFF: Icon = lucide!("navigation-2-off", r#"<path d="M9.31 9.31 5 21l7-4 7 4-1.17-3.17"/><path d="M14.53 8.88 12 2l-1.17 3.17"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `navigation-off`.
pub const NAVIGATION_OFF: Icon = lucide!("navigation-off", r#"<path d="M8.43 8.43 3 11l8 2 2 8 2.57-5.43"/><path d="M17.39 11.73 22 2l-9.73 4.61"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `nepali-rupee`.
pub const NEPALI_RUPEE: Icon = lucide!("nepali-rupee", r#"<path d="m14 20-8-8h4c5.333 0 5.333-8 0-8"/><path d="M14 8c4.078 0 5.038 4.678 2.88 6.88"/><path d="M6 4h12"/>"#);

/// `network`.
pub const NETWORK: Icon = lucide!("network", r#"<rect x="16" y="16" width="6" height="6" rx="1"/><rect x="2" y="16" width="6" height="6" rx="1"/><rect x="9" y="2" width="6" height="6" rx="1"/><path d="M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"/><path d="M12 12V8"/>"#);

/// `newspaper`.
pub const NEWSPAPER: Icon = lucide!("newspaper", r#"<path d="M15 18h-5"/><path d="M18 14h-8"/><path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-4 0v-9a2 2 0 0 1 2-2h2"/><rect width="8" height="4" x="10" y="6" rx="1"/>"#);

/// `nfc`.
pub const NFC: Icon = lucide!("nfc", r#"<path d="M6 8.32a7.43 7.43 0 0 1 0 7.36"/><path d="M9.46 6.21a11.76 11.76 0 0 1 0 11.58"/><path d="M12.91 4.1a15.91 15.91 0 0 1 .01 15.8"/><path d="M16.37 2a20.16 20.16 0 0 1 0 20"/>"#);

/// `non-binary`.
pub const NON_BINARY: Icon = lucide!("non-binary", r#"<path d="M12 2v10"/><path d="m8.5 4 7 4"/><path d="m8.5 8 7-4"/><circle cx="12" cy="17" r="5"/>"#);

/// `notebook`.
pub const NOTEBOOK: Icon = lucide!("notebook", r#"<path d="M2 6h4"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><rect width="16" height="20" x="4" y="2" rx="2"/><path d="M16 2v20"/>"#);

/// `notebook-dot`.
pub const NOTEBOOK_DOT: Icon = lucide!("notebook-dot", r#"<path d="M16 11.75V22"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><path d="M2 6h4"/><path d="M20 11.75V20a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h5.65"/><circle cx="18" cy="5" r="3"/>"#);

/// `notebook-pen`.
pub const NOTEBOOK_PEN: Icon = lucide!("notebook-pen", r#"<path d="M13.4 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-7.4"/><path d="M2 6h4"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><path d="M21.378 5.626a1 1 0 1 0-3.004-3.004l-5.01 5.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/>"#);

/// `notebook-tabs`.
pub const NOTEBOOK_TABS: Icon = lucide!("notebook-tabs", r#"<path d="M2 6h4"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><rect width="16" height="20" x="4" y="2" rx="2"/><path d="M15 2v20"/><path d="M15 7h5"/><path d="M15 12h5"/><path d="M15 17h5"/>"#);

/// `notebook-text`.
pub const NOTEBOOK_TEXT: Icon = lucide!("notebook-text", r#"<path d="M2 6h4"/><path d="M2 10h4"/><path d="M2 14h4"/><path d="M2 18h4"/><rect width="16" height="20" x="4" y="2" rx="2"/><path d="M9.5 8h5"/><path d="M9.5 12H16"/><path d="M9.5 16H14"/>"#);

/// `notepad-text`.
pub const NOTEPAD_TEXT: Icon = lucide!("notepad-text", r#"<path d="M8 2v4"/><path d="M12 2v4"/><path d="M16 2v4"/><rect width="16" height="18" x="4" y="4" rx="2"/><path d="M8 10h6"/><path d="M8 14h8"/><path d="M8 18h5"/>"#);

/// `notepad-text-dashed`.
pub const NOTEPAD_TEXT_DASHED: Icon = lucide!("notepad-text-dashed", r#"<path d="M8 2v4"/><path d="M12 2v4"/><path d="M16 2v4"/><path d="M16 4h2a2 2 0 0 1 2 2v2"/><path d="M20 12v2"/><path d="M20 18v2a2 2 0 0 1-2 2h-1"/><path d="M13 22h-2"/><path d="M7 22H6a2 2 0 0 1-2-2v-2"/><path d="M4 14v-2"/><path d="M4 8V6a2 2 0 0 1 2-2h2"/><path d="M8 10h6"/><path d="M8 14h8"/><path d="M8 18h5"/>"#);

/// `numbers`.
pub const NUMBERS: Icon = lucide!("numbers", r#"<path d="m10 4 2.6-1.95a.25.25 0 01.4.2V10"/><path d="M16 14.535A4 4 0 0118 14a2 2 0 010 4 2 2 0 010 4 4 4 0 01-2-.535"/><path d="M8 22H4.5a.5.5 0 01-.5-.5c0-2.68 4-2.932 4-5.5a2 2 0 00-2-2 2.5 2.5 0 00-2 1"/>"#);

/// `nut`.
pub const NUT: Icon = lucide!("nut", r#"<path d="M16.847 5.847 20 9a7.23 7.23 0 011.551 7.516C21.241 17.352 21 17.932 21 19v1a1 1 0 01-1 1h-1c-1.069 0-1.648.242-2.485.552A7.2 7.2 0 019.002 20l-3.155-3.153"/><path d="M18.21 5.43c-1.71.69-5.07 1.07-6.71 1.07.46 1.38.91 2.74.61 4.88a.88.88 0 01-.73.74c-1.78.28-3.54-.17-4.88-.62 0 1.64-.38 5-1.07 6.71-.21.52-.82.55-1.17.12A10 10 0 0118.33 4.26c.43.35.4.97-.12 1.17"/><path d="M4.93 4.93 3 3a.7.7 0 010-1"/>"#);

/// `nut-off`.
pub const NUT_OFF: Icon = lucide!("nut-off", r#"<path d="M11.868 11.868a.88.88 0 01-.488.252c-1.78.28-3.54-.17-4.88-.62 0 1.272-.229 3.578-.653 5.347a10 10 0 01-.417 1.363c-.21.52-.82.55-1.17.12a10 10 0 01.677-13.393"/><path d="M12.14 6.485a27.4 27.4 0 004.707-.638L20 9a7.23 7.23 0 011.706 7.05"/><path d="m2 2 20 20"/><path d="M20.707 20.707A1 1 0 0120 21h-1c-1.069 0-1.648.242-2.485.552A7.2 7.2 0 019.002 20l-3.155-3.153"/><path d="M8.356 2.7a10 10 0 019.974 1.56c.43.35.4.97-.12 1.17a10 10 0 01-1.363.417"/>"#);

/// `octagon`.
pub const OCTAGON: Icon = lucide!("octagon", r#"<path d="M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z"/>"#);

/// `octagon-exclamation-point`.
pub const OCTAGON_EXCLAMATION_POINT: Icon = lucide!("octagon-exclamation-point", r#"<path d="M12 16h.01"/><path d="M12 8v4"/><path d="M15.312 2a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586l-4.688-4.688A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2z"/>"#);

/// `octagon-minus`.
pub const OCTAGON_MINUS: Icon = lucide!("octagon-minus", r#"<path d="M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z"/><path d="M8 12h8"/>"#);

/// `octagon-pause`.
pub const OCTAGON_PAUSE: Icon = lucide!("octagon-pause", r#"<path d="M10 15V9"/><path d="M14 15V9"/><path d="M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z"/>"#);

/// `octagon-x`.
pub const OCTAGON_X: Icon = lucide!("octagon-x", r#"<path d="m15 9-6 6"/><path d="M2.586 16.726A2 2 0 0 1 2 15.312V8.688a2 2 0 0 1 .586-1.414l4.688-4.688A2 2 0 0 1 8.688 2h6.624a2 2 0 0 1 1.414.586l4.688 4.688A2 2 0 0 1 22 8.688v6.624a2 2 0 0 1-.586 1.414l-4.688 4.688a2 2 0 0 1-1.414.586H8.688a2 2 0 0 1-1.414-.586z"/><path d="m9 9 6 6"/>"#);

/// `omega`.
pub const OMEGA: Icon = lucide!("omega", r#"<path d="M3 20h4.5a.5.5 0 0 0 .5-.5v-.282a.52.52 0 0 0-.247-.437 8 8 0 1 1 8.494-.001.52.52 0 0 0-.247.438v.282a.5.5 0 0 0 .5.5H21"/>"#);

/// `opacity`.
pub const OPACITY: Icon = lucide!("opacity", r#"<path d="M12 3h.01"/><path d="M12 9h.01"/><path d="M15 6h.01"/><path d="M18 3h.01"/><path d="M21 6v13a2 2 0 0 1-2 2H6"/><path d="M3 12h.01"/><path d="M3 18h.01"/><path d="M3 6h.01"/><path d="M6 15h.01"/><path d="M6 3h.01"/><path d="M6 9h.01"/><path d="M9 12h.01"/><path d="M9 6h.01"/>"#);

/// `option`.
pub const OPTION: Icon = lucide!("option", r#"<path d="M14 3h7"/><path d="M3 3h5.28a1 1 0 0 1 .948.684l5.544 16.632a1 1 0 0 0 .949.684H21"/>"#);

/// `orbit`.
pub const ORBIT: Icon = lucide!("orbit", r#"<path d="M20.341 6.484A10 10 0 0 1 10.266 21.85"/><path d="M3.659 17.516A10 10 0 0 1 13.74 2.152"/><circle cx="12" cy="12" r="3"/><circle cx="19" cy="5" r="2"/><circle cx="5" cy="19" r="2"/>"#);

/// `origami`.
pub const ORIGAMI: Icon = lucide!("origami", r#"<path d="M12 12V4a1 1 0 0 1 1-1h6.297a1 1 0 0 1 .651 1.759l-4.696 4.025"/><path d="m12 21-7.414-7.414A2 2 0 0 1 4 12.172V6.415a1.002 1.002 0 0 1 1.707-.707L20 20.009"/><path d="m12.214 3.381 8.414 14.966a1 1 0 0 1-.167 1.199l-1.168 1.163a1 1 0 0 1-.706.291H6.351a1 1 0 0 1-.625-.219L3.25 18.8a1 1 0 0 1 .631-1.781l4.165.027"/>"#);

/// `package`.
pub const PACKAGE: Icon = lucide!("package", r#"<path d="M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z"/><path d="M12 22V12"/><polyline points="3.29 7 12 12 20.71 7"/><path d="m7.5 4.27 9 5.15"/>"#);

/// `package-2`.
pub const PACKAGE_2: Icon = lucide!("package-2", r#"<path d="M12 3v6"/><path d="M16.76 3a2 2 0 0 1 1.8 1.1l2.23 4.479a2 2 0 0 1 .21.891V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9.472a2 2 0 0 1 .211-.894L5.45 4.1A2 2 0 0 1 7.24 3z"/><path d="M3.054 9.013h17.893"/>"#);

/// `package-check`.
pub const PACKAGE_CHECK: Icon = lucide!("package-check", r#"<path d="M12 22V12"/><path d="m16 17 2 2 4-4"/><path d="M21 11.127V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.729l7 4a2 2 0 0 0 2 .001l1.32-.753"/><path d="M3.29 7 12 12l8.71-5"/><path d="m7.5 4.27 8.997 5.148"/>"#);

/// `package-minus`.
pub const PACKAGE_MINUS: Icon = lucide!("package-minus", r#"<path d="M12 22V12"/><path d="M16 17h6"/><path d="M21 13V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.729l7 4a2 2 0 0 0 2 .001l1.675-.955"/><path d="M3.29 7 12 12l8.71-5"/><path d="m7.5 4.27 8.997 5.148"/>"#);

/// `package-open`.
pub const PACKAGE_OPEN: Icon = lucide!("package-open", r#"<path d="M12 22v-9"/><path d="M15.17 2.21a1.67 1.67 0 0 1 1.63 0L21 4.57a1.93 1.93 0 0 1 0 3.36L8.82 14.79a1.655 1.655 0 0 1-1.64 0L3 12.43a1.93 1.93 0 0 1 0-3.36z"/><path d="M20 13v3.87a2.06 2.06 0 0 1-1.11 1.83l-6 3.08a1.93 1.93 0 0 1-1.78 0l-6-3.08A2.06 2.06 0 0 1 4 16.87V13"/><path d="M21 12.43a1.93 1.93 0 0 0 0-3.36L8.83 2.2a1.64 1.64 0 0 0-1.63 0L3 4.57a1.93 1.93 0 0 0 0 3.36l12.18 6.86a1.636 1.636 0 0 0 1.63 0z"/>"#);

/// `package-pen`.
pub const PACKAGE_PEN: Icon = lucide!("package-pen", r#"<path d="M12 14.34V12l8.71-5"/><path d="M16.511 21.141a2 2 0 0 0 .854-.506l4.013-4.009a1 1 0 1 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .599.626z"/><path d="M21 9.114V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l5.087 2.907"/><path d="M3.29 7 12 12"/><path d="m7.5 4.27 9 5.15"/>"#);

/// `package-plus`.
pub const PACKAGE_PLUS: Icon = lucide!("package-plus", r#"<path d="M12 22V12"/><path d="M16 17h6"/><path d="M19 14v6"/><path d="M21 10.535V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.729l7 4a2 2 0 0 0 2 .001l1.675-.955"/><path d="M3.29 7 12 12l8.71-5"/><path d="m7.5 4.27 8.997 5.148"/>"#);

/// `package-question-mark`.
pub const PACKAGE_QUESTION_MARK: Icon = lucide!("package-question-mark", r#"<path d="M12 22V12l8.71-5"/><path d="M17 15.27c.2-.4.5-.8.9-1a2.1 2.1 0 0 1 2.6.4c.3.4.5.8.5 1.3 0 1.3-2 2-2 2"/><path d="M19 21.99h.01"/><path d="M21 10.342V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2.33-1.332"/><path d="M3.29 7 12 12"/><path d="m7.5 4.27 9 5.15"/>"#);

/// `package-search`.
pub const PACKAGE_SEARCH: Icon = lucide!("package-search", r#"<path d="M12 22V12"/><path d="M20.27 18.27 22 20"/><path d="M21 10.498V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.729l7 4a2 2 0 0 0 2 .001l.98-.559"/><path d="M3.29 7 12 12l8.71-5"/><path d="m7.5 4.27 8.997 5.148"/><circle cx="18.5" cy="16.5" r="2.5"/>"#);

/// `package-x`.
pub const PACKAGE_X: Icon = lucide!("package-x", r#"<path d="M12 22V12"/><path d="m16.5 14.5 5 5"/><path d="m16.5 19.5 5-5"/><path d="M21 10.5V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.729l7 4a2 2 0 0 0 2 .001l.13-.074"/><path d="M3.29 7 12 12l8.71-5"/><path d="m7.5 4.27 8.997 5.148"/>"#);

/// `paint-bucket`.
pub const PAINT_BUCKET: Icon = lucide!("paint-bucket", r#"<path d="M11 7 6 2"/><path d="M18.992 12H2.041"/><path d="M21.145 18.38A3.34 3.34 0 0 1 20 16.5a3.3 3.3 0 0 1-1.145 1.88c-.575.46-.855 1.02-.855 1.595A2 2 0 0 0 20 22a2 2 0 0 0 2-2.025c0-.58-.285-1.13-.855-1.595"/><path d="m8.5 4.5 2.148-2.148a1.205 1.205 0 0 1 1.704 0l7.296 7.296a1.205 1.205 0 0 1 0 1.704l-7.592 7.592a3.615 3.615 0 0 1-5.112 0l-3.888-3.888a3.615 3.615 0 0 1 0-5.112L5.67 7.33"/>"#);

/// `paint-roller`.
pub const PAINT_ROLLER: Icon = lucide!("paint-roller", r#"<rect width="16" height="6" x="2" y="2" rx="2"/><path d="M10 16v-2a2 2 0 0 1 2-2h8a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2"/><rect width="4" height="6" x="8" y="16" rx="1"/>"#);

/// `paintbrush`.
pub const PAINTBRUSH: Icon = lucide!("paintbrush", r#"<path d="m14.622 17.897-10.68-2.913"/><path d="M18.376 2.622a1 1 0 1 1 3.002 3.002L17.36 9.643a.5.5 0 0 0 0 .707l.944.944a2.41 2.41 0 0 1 0 3.408l-.944.944a.5.5 0 0 1-.707 0L8.354 7.348a.5.5 0 0 1 0-.707l.944-.944a2.41 2.41 0 0 1 3.408 0l.944.944a.5.5 0 0 0 .707 0z"/><path d="M9 8c-1.804 2.71-3.97 3.46-6.583 3.948a.507.507 0 0 0-.302.819l7.32 8.883a1 1 0 0 0 1.185.204C12.735 20.405 16 16.792 16 15"/>"#);

/// `paintbrush-vertical`.
pub const PAINTBRUSH_VERTICAL: Icon = lucide!("paintbrush-vertical", r#"<path d="M10 2v2"/><path d="M14 2v4"/><path d="M17 2a1 1 0 0 1 1 1v9H6V3a1 1 0 0 1 1-1z"/><path d="M6 12a1 1 0 0 0-1 1v1a2 2 0 0 0 2 2h2a1 1 0 0 1 1 1v2.9a2 2 0 1 0 4 0V17a1 1 0 0 1 1-1h2a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1"/>"#);

/// `palette`.
pub const PALETTE: Icon = lucide!("palette", r#"<path d="M12 22a1 1 0 0 1 0-20 10 9 0 0 1 10 9 5 5 0 0 1-5 5h-2.25a1.75 1.75 0 0 0-1.4 2.8l.3.4a1.75 1.75 0 0 1-1.4 2.8z"/><circle cx="13.5" cy="6.5" r=".5" fill="currentColor"/><circle cx="17.5" cy="10.5" r=".5" fill="currentColor"/><circle cx="6.5" cy="12.5" r=".5" fill="currentColor"/><circle cx="8.5" cy="7.5" r=".5" fill="currentColor"/>"#);

/// `panda`.
pub const PANDA: Icon = lucide!("panda", r#"<path d="M11.25 17.25h1.5L12 18z"/><path d="m15 12 2 2"/><path d="M17.902 6.599a8 8 0 0 0-.5-.5"/><path d="M2 14.5C2 19.47 6.48 22 12 22s10-2.53 10-7.5a10 10 0 0 0-1.3-4.83 4.5 4.5 0 1 0-7.05-5.5 8 8 0 0 0-3.3 0 4.5 4.5 0 1 0-7.04 5.5A10 10 0 0 0 2 14.5"/><path d="M6.099 6.599a8 8 0 0 1 .5-.5"/><path d="m9 12-2 2"/>"#);

/// `panel-bottom`.
pub const PANEL_BOTTOM: Icon = lucide!("panel-bottom", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 15h18"/>"#);

/// `panel-bottom-close`.
pub const PANEL_BOTTOM_CLOSE: Icon = lucide!("panel-bottom-close", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 15h18"/><path d="m15 8-3 3-3-3"/>"#);

/// `panel-bottom-dashed`.
pub const PANEL_BOTTOM_DASHED: Icon = lucide!("panel-bottom-dashed", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M14 15h1"/><path d="M19 15h2"/><path d="M3 15h2"/><path d="M9 15h1"/>"#);

/// `panel-bottom-open`.
pub const PANEL_BOTTOM_OPEN: Icon = lucide!("panel-bottom-open", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 15h18"/><path d="m9 10 3-3 3 3"/>"#);

/// `panel-left`.
pub const PANEL_LEFT: Icon = lucide!("panel-left", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/>"#);

/// `panel-left-close`.
pub const PANEL_LEFT_CLOSE: Icon = lucide!("panel-left-close", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/><path d="m16 15-3-3 3-3"/>"#);

/// `panel-left-dashed`.
pub const PANEL_LEFT_DASHED: Icon = lucide!("panel-left-dashed", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 14v1"/><path d="M9 19v2"/><path d="M9 3v2"/><path d="M9 9v1"/>"#);

/// `panel-left-open`.
pub const PANEL_LEFT_OPEN: Icon = lucide!("panel-left-open", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/><path d="m14 9 3 3-3 3"/>"#);

/// `panel-left-right-dashed`.
pub const PANEL_LEFT_RIGHT_DASHED: Icon = lucide!("panel-left-right-dashed", r#"<path d="M15 10V9"/><path d="M15 15v-1"/><path d="M15 21v-2"/><path d="M15 5V3"/><path d="M9 10V9"/><path d="M9 15v-1"/><path d="M9 21v-2"/><path d="M9 5V3"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `panel-right`.
pub const PANEL_RIGHT: Icon = lucide!("panel-right", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/>"#);

/// `panel-right-close`.
pub const PANEL_RIGHT_CLOSE: Icon = lucide!("panel-right-close", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/><path d="m8 9 3 3-3 3"/>"#);

/// `panel-right-dashed`.
pub const PANEL_RIGHT_DASHED: Icon = lucide!("panel-right-dashed", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 14v1"/><path d="M15 19v2"/><path d="M15 3v2"/><path d="M15 9v1"/>"#);

/// `panel-right-open`.
pub const PANEL_RIGHT_OPEN: Icon = lucide!("panel-right-open", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/><path d="m10 15-3-3 3-3"/>"#);

/// `panel-top`.
pub const PANEL_TOP: Icon = lucide!("panel-top", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/>"#);

/// `panel-top-bottom-dashed`.
pub const PANEL_TOP_BOTTOM_DASHED: Icon = lucide!("panel-top-bottom-dashed", r#"<path d="M14 15h1"/><path d="M14 9h1"/><path d="M19 15h2"/><path d="M19 9h2"/><path d="M3 15h2"/><path d="M3 9h2"/><path d="M9 15h1"/><path d="M9 9h1"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `panel-top-close`.
pub const PANEL_TOP_CLOSE: Icon = lucide!("panel-top-close", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="m9 16 3-3 3 3"/>"#);

/// `panel-top-dashed`.
pub const PANEL_TOP_DASHED: Icon = lucide!("panel-top-dashed", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M14 9h1"/><path d="M19 9h2"/><path d="M3 9h2"/><path d="M9 9h1"/>"#);

/// `panel-top-open`.
pub const PANEL_TOP_OPEN: Icon = lucide!("panel-top-open", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="m15 14-3 3-3-3"/>"#);

/// `panels-left-bottom`.
pub const PANELS_LEFT_BOTTOM: Icon = lucide!("panels-left-bottom", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 3v18"/><path d="M9 15h12"/>"#);

/// `panels-right-bottom`.
pub const PANELS_RIGHT_BOTTOM: Icon = lucide!("panels-right-bottom", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 15h12"/><path d="M15 3v18"/>"#);

/// `panels-top-left`.
pub const PANELS_TOP_LEFT: Icon = lucide!("panels-top-left", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/>"#);

/// `paper-bag`.
pub const PAPER_BAG: Icon = lucide!("paper-bag", r#"<path d="M5.364 3.848C4 6 3 9.652 3 12.652V19a2 2 0 002 2h14a2 2 0 002-2v-5c0-2.334-1.816-4.668-2.622-7.002"/><path d="M7 3h11.379a2 2 0 011.789 1.106l.723 1.447A1 1 0 0119.997 7h-8.525a2 2 0 01-1.789-1.106L8.79 4.105a2 2 0 10-3.579 1.789l2.261 4.522A5 5 0 018 12.652V21"/>"#);

/// `paperclip`.
pub const PAPERCLIP: Icon = lucide!("paperclip", r#"<path d="m16 6-8.414 8.586a2 2 0 0 0 2.829 2.829l8.414-8.586a4 4 0 1 0-5.657-5.657l-8.379 8.551a6 6 0 1 0 8.485 8.485l8.379-8.551"/>"#);

/// `parasol`.
pub const PARASOL: Icon = lucide!("parasol", r#"<path d="M12.5 11.134 18.196 21"/><path d="M20.425 5.299a10 10 0 0 0-16.941 9.78c.183.563.843.774 1.355.478L20.16 6.711c.512-.296.66-.973.264-1.413"/><path d="M21 21H3"/>"#);

/// `parentheses`.
pub const PARENTHESES: Icon = lucide!("parentheses", r#"<path d="M8 21s-4-3-4-9 4-9 4-9"/><path d="M16 3s4 3 4 9-4 9-4 9"/>"#);

/// `park`.
pub const PARK: Icon = lucide!("park", r#"<path d="M12 18h10"/><path d="M13.248 9.998A4.5 4.5 0 0 0 11.75 8.6V8a1 1 0 0 0-7.5 0 4.9 4.9 0 0 0 2.25 9H8"/><path d="m15 14-2 6"/><path d="m19 14 2 6"/><path d="M21 14h-8"/><path d="M8 20v-5.922a2 2 0 0 0-.586-1.414L6.5 11.75"/><path d="M9.205 12.795 8 14"/><circle cx="19" cy="6" r="2"/>"#);

/// `parking-meter`.
pub const PARKING_METER: Icon = lucide!("parking-meter", r#"<path d="M11 15h2"/><path d="M12 12v3"/><path d="M12 19v3"/><path d="M15.282 19a1 1 0 0 0 .948-.68l2.37-6.988a7 7 0 1 0-13.2 0l2.37 6.988a1 1 0 0 0 .948.68z"/><path d="M9 9a3 3 0 1 1 6 0"/>"#);

/// `party-popper`.
pub const PARTY_POPPER: Icon = lucide!("party-popper", r#"<path d="M5.8 11.3 2 22l10.7-3.79"/><path d="M4 3h.01"/><path d="M22 8h.01"/><path d="M15 2h.01"/><path d="M22 20h.01"/><path d="m22 2-2.24.75a2.9 2.9 0 0 0-1.96 3.12c.1.86-.57 1.63-1.45 1.63h-.38c-.86 0-1.6.6-1.76 1.44L14 10"/><path d="m22 13-.82-.33c-.86-.34-1.82.2-1.98 1.11c-.11.7-.72 1.22-1.43 1.22H17"/><path d="m11 2 .33.82c.34.86-.2 1.82-1.11 1.98C9.52 4.9 9 5.52 9 6.23V7"/><path d="M11 13c1.93 1.93 2.83 4.17 2 5-.83.83-3.07-.07-5-2-1.93-1.93-2.83-4.17-2-5 .83-.83 3.07.07 5 2Z"/>"#);

/// `pause`.
pub const PAUSE: Icon = lucide!("pause", r#"<rect x="14" y="3" width="5" height="18" rx="1"/><rect x="5" y="3" width="5" height="18" rx="1"/>"#);

/// `paw-print`.
pub const PAW_PRINT: Icon = lucide!("paw-print", r#"<circle cx="11" cy="4" r="2"/><circle cx="18" cy="8" r="2"/><circle cx="20" cy="16" r="2"/><path d="M9 10a5 5 0 0 1 5 5v3.5a3.5 3.5 0 0 1-6.84 1.045Q6.52 17.48 4.46 16.84A3.5 3.5 0 0 1 5.5 10Z"/>"#);

/// `pc-case`.
pub const PC_CASE: Icon = lucide!("pc-case", r#"<rect width="14" height="20" x="5" y="2" rx="2"/><path d="M15 14h.01"/><path d="M9 6h6"/><path d="M9 10h6"/>"#);

/// `pen`.
pub const PEN: Icon = lucide!("pen", r#"<path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/>"#);

/// `pen-line`.
pub const PEN_LINE: Icon = lucide!("pen-line", r#"<path d="M13 21h8"/><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/>"#);

/// `pen-off`.
pub const PEN_OFF: Icon = lucide!("pen-off", r#"<path d="m10 10-6.157 6.162a2 2 0 0 0-.5.833l-1.322 4.36a.5.5 0 0 0 .622.624l4.358-1.323a2 2 0 0 0 .83-.5L14 13.982"/><path d="m12.829 7.172 4.359-4.346a1 1 0 1 1 3.986 3.986l-4.353 4.353"/><path d="m2 2 20 20"/>"#);

/// `pen-tool`.
pub const PEN_TOOL: Icon = lucide!("pen-tool", r#"<path d="m18 13-.654-4.736a4 4 0 00-2.756-3.266l-9.163-2.9a2 2 0 00-2.017.493l-.82.819a2 2 0 00-.492 2.017l2.9 9.163a4 4 0 003.266 2.756L13 18"/><path d="M18.648 12.352a1.205 1.205 0 011.704 0l1.296 1.296a1.205 1.205 0 010 1.704l-6.296 6.296a1.205 1.205 0 01-1.704 0l-1.296-1.296a1.205 1.205 0 010-1.704z"/><path d="m3 3 6.586 6.586"/><circle cx="11" cy="11" r="2"/>"#);

/// `pencil`.
pub const PENCIL: Icon = lucide!("pencil", r#"<path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/>"#);

/// `pencil-line`.
pub const PENCIL_LINE: Icon = lucide!("pencil-line", r#"<path d="M13 21h8"/><path d="m15 5 4 4"/><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/>"#);

/// `pencil-off`.
pub const PENCIL_OFF: Icon = lucide!("pencil-off", r#"<path d="m10 10-6.157 6.162a2 2 0 0 0-.5.833l-1.322 4.36a.5.5 0 0 0 .622.624l4.358-1.323a2 2 0 0 0 .83-.5L14 13.982"/><path d="m12.829 7.172 4.359-4.346a1 1 0 1 1 3.986 3.986l-4.353 4.353"/><path d="m15 5 4 4"/><path d="m2 2 20 20"/>"#);

/// `pencil-ruler`.
pub const PENCIL_RULER: Icon = lucide!("pencil-ruler", r#"<path d="M13 7 8.7 2.7a2.41 2.41 0 0 0-3.4 0L2.7 5.3a2.41 2.41 0 0 0 0 3.4L7 13"/><path d="m8 6 2-2"/><path d="m18 16 2-2"/><path d="m17 11 4.3 4.3c.94.94.94 2.46 0 3.4l-2.6 2.6c-.94.94-2.46.94-3.4 0L11 17"/><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/>"#);

/// `pencil-sparkles`.
pub const PENCIL_SPARKLES: Icon = lucide!("pencil-sparkles", r#"<path d="M10 3H8"/><path d="m15.007 5.008 3.987 3.986"/><path d="M20 15v4"/><path d="M21.174 6.813a2.82 2.82 0 0 0-3.986-3.987L3.842 16.175a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="M22 17h-4"/><path d="M4 5v4"/><path d="M6 7H2"/><path d="M9 2v2"/>"#);

/// `pentagon`.
pub const PENTAGON: Icon = lucide!("pentagon", r#"<path d="M10.83 2.38a2 2 0 0 1 2.34 0l8 5.74a2 2 0 0 1 .73 2.25l-3.04 9.26a2 2 0 0 1-1.9 1.37H7.04a2 2 0 0 1-1.9-1.37L2.1 10.37a2 2 0 0 1 .73-2.25z"/>"#);

/// `percent`.
pub const PERCENT: Icon = lucide!("percent", r#"<line x1="19" x2="5" y1="5" y2="19"/><circle cx="6.5" cy="6.5" r="2.5"/><circle cx="17.5" cy="17.5" r="2.5"/>"#);

/// `perfume-bottle`.
pub const PERFUME_BOTTLE: Icon = lucide!("perfume-bottle", r#"<path d="M 5 22 H 11.072"/><path d="M15 5 C 23 4 23.328 12.676 20 14.536"/><path d="M15.802 10.308 C 14.776 9.465 14 8.496 14 7"/><path d="M2 2h.01"/><path d="M2 7h.01"/><path d="M5 4.5h.01"/><path d="M8.127 22 A 8 8 0 0 1 4 15 3 3 0 0 1 4.879 12.879C 6.879 10.879 10 10 10 7"/><circle cx="18" cy="18" r="4"/><rect x="9" y="2" width="6" height="5" rx="1"/>"#);

/// `person-standing`.
pub const PERSON_STANDING: Icon = lucide!("person-standing", r#"<circle cx="12" cy="5" r="1"/><path d="m9 20 3-6 3 6"/><path d="m6 8 6 2 6-2"/><path d="M12 10v4"/>"#);

/// `phi`.
pub const PHI: Icon = lucide!("phi", r#"<path d="M12 2v20"/><circle cx="12" cy="12" r="7"/>"#);

/// `philippine-peso`.
pub const PHILIPPINE_PESO: Icon = lucide!("philippine-peso", r#"<path d="M4 12h16"/><path d="M4 8h16"/><path d="M6 20V5a1 1 0 011-1h5a6 6 0 010 12H6"/>"#);

/// `phone`.
pub const PHONE: Icon = lucide!("phone", r#"<path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `phone-call`.
pub const PHONE_CALL: Icon = lucide!("phone-call", r#"<path d="M13 2a9 9 0 0 1 9 9"/><path d="M13 6a5 5 0 0 1 5 5"/><path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `phone-forwarded`.
pub const PHONE_FORWARDED: Icon = lucide!("phone-forwarded", r#"<path d="M14 6h8"/><path d="m18 2 4 4-4 4"/><path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `phone-incoming`.
pub const PHONE_INCOMING: Icon = lucide!("phone-incoming", r#"<path d="M16 2v6h6"/><path d="m22 2-6 6"/><path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `phone-log`.
pub const PHONE_LOG: Icon = lucide!("phone-log", r#"<path d="M13.832 16.568a1 1 0 001.213-.303l.355-.465A2 2 0 0117 15h3a2 2 0 012 2v3a2 2 0 01-2 2A18 18 0 012 4a2 2 0 012-2h3a2 2 0 012 2v3a2 2 0 01-.8 1.6l-.468.351a1 1 0 00-.292 1.233 14 14 0 006.392 6.384"/><path d="M14.396 4.171h7.171"/><path d="M17.343 8.17h4.224"/>"#);

/// `phone-missed`.
pub const PHONE_MISSED: Icon = lucide!("phone-missed", r#"<path d="m16 2 6 6"/><path d="m22 2-6 6"/><path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `phone-off`.
pub const PHONE_OFF: Icon = lucide!("phone-off", r#"<path d="M10.1 13.9a14 14 0 0 0 3.732 2.668 1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2 18 18 0 0 1-12.728-5.272"/><path d="M22 2 2 22"/><path d="M4.76 13.582A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 .244.473"/>"#);

/// `phone-outgoing`.
pub const PHONE_OUTGOING: Icon = lucide!("phone-outgoing", r#"<path d="m16 8 6-6"/><path d="M22 8V2h-6"/><path d="M13.832 16.568a1 1 0 0 0 1.213-.303l.355-.465A2 2 0 0 1 17 15h3a2 2 0 0 1 2 2v3a2 2 0 0 1-2 2A18 18 0 0 1 2 4a2 2 0 0 1 2-2h3a2 2 0 0 1 2 2v3a2 2 0 0 1-.8 1.6l-.468.351a1 1 0 0 0-.292 1.233 14 14 0 0 0 6.392 6.384"/>"#);

/// `pi`.
pub const PI: Icon = lucide!("pi", r#"<line x1="9" x2="9" y1="4" y2="20"/><path d="M4 7c0-1.7 1.3-3 3-3h13"/><path d="M18 20c-1.7 0-3-1.3-3-3V4"/>"#);

/// `piano`.
pub const PIANO: Icon = lucide!("piano", r#"<path d="M10 13v4"/><path d="M14 13v4"/><path d="M18 13v4"/><path d="M2 13h20"/><path d="M22 11.5A3.5 3.5 0 0018.5 8a3.52 3.52 0 01-3.173-2A7 7 0 002 9v10a2 2 0 002 2h16a2 2 0 002-2z"/><path d="M6 13v4"/>"#);

/// `pickaxe`.
pub const PICKAXE: Icon = lucide!("pickaxe", r#"<path d="m14 13-8.381 8.38a1 1 0 0 1-3.001-3L11 9.999"/><path d="M15.973 4.027A13 13 0 0 0 5.902 2.373c-1.398.342-1.092 2.158.277 2.601a19.9 19.9 0 0 1 5.822 3.024"/><path d="M16.001 11.999a19.9 19.9 0 0 1 3.024 5.824c.444 1.369 2.26 1.676 2.603.278A13 13 0 0 0 20 8.069"/><path d="M18.352 3.352a1.205 1.205 0 0 0-1.704 0l-5.296 5.296a1.205 1.205 0 0 0 0 1.704l2.296 2.296a1.205 1.205 0 0 0 1.704 0l5.296-5.296a1.205 1.205 0 0 0 0-1.704z"/>"#);

/// `picture-in-picture`.
pub const PICTURE_IN_PICTURE: Icon = lucide!("picture-in-picture", r#"<path d="M2 10h6V4"/><path d="m2 4 6 6"/><path d="M21 10V7a2 2 0 0 0-2-2h-7"/><path d="M3 14v2a2 2 0 0 0 2 2h3"/><rect x="12" y="14" width="10" height="7" rx="1"/>"#);

/// `picture-in-picture-2`.
pub const PICTURE_IN_PICTURE_2: Icon = lucide!("picture-in-picture-2", r#"<path d="M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4"/><rect width="10" height="7" x="12" y="13" rx="2"/>"#);

/// `piggy-bank`.
pub const PIGGY_BANK: Icon = lucide!("piggy-bank", r#"<path d="M11 17h3v2a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a3.16 3.16 0 0 0 2-2h1a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1h-1a5 5 0 0 0-2-4V3a4 4 0 0 0-3.2 1.6l-.3.4H11a6 6 0 0 0-6 6v1a5 5 0 0 0 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1z"/><path d="M16 10h.01"/><path d="M2 8v1a2 2 0 0 0 2 2h1"/>"#);

/// `pilcrow`.
pub const PILCROW: Icon = lucide!("pilcrow", r#"<path d="M13 4v16"/><path d="M17 4v16"/><path d="M19 4H9.5a4.5 4.5 0 0 0 0 9H13"/>"#);

/// `pilcrow-left`.
pub const PILCROW_LEFT: Icon = lucide!("pilcrow-left", r#"<path d="M14 3v11"/><path d="M14 9h-3a3 3 0 0 1 0-6h9"/><path d="M18 3v11"/><path d="M22 18H2l4-4"/><path d="m6 22-4-4"/>"#);

/// `pilcrow-right`.
pub const PILCROW_RIGHT: Icon = lucide!("pilcrow-right", r#"<path d="M10 3v11"/><path d="M10 9H7a1 1 0 0 1 0-6h8"/><path d="M14 3v11"/><path d="m18 14 4 4H2"/><path d="m22 18-4 4"/>"#);

/// `pill`.
pub const PILL: Icon = lucide!("pill", r#"<path d="m10.5 20.5 10-10a4.95 4.95 0 1 0-7-7l-10 10a4.95 4.95 0 1 0 7 7Z"/><path d="m8.5 8.5 7 7"/>"#);

/// `pill-bottle`.
pub const PILL_BOTTLE: Icon = lucide!("pill-bottle", r#"<path d="M18 11h-4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h4"/><path d="M6 7v13a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2V7"/><rect width="16" height="5" x="4" y="2" rx="1"/>"#);

/// `pin`.
pub const PIN: Icon = lucide!("pin", r#"<path d="M12 17v5"/><path d="M9 10.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-.76a2 2 0 0 0-1.11-1.79l-1.78-.9A2 2 0 0 1 15 10.76V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H8a2 2 0 0 0 0 4 1 1 0 0 1 1 1z"/>"#);

/// `pin-off`.
pub const PIN_OFF: Icon = lucide!("pin-off", r#"<path d="M12 17v5"/><path d="M15 9.34V7a1 1 0 0 1 1-1 2 2 0 0 0 0-4H7.89"/><path d="m2 2 20 20"/><path d="M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V16a1 1 0 0 0 1 1h11"/>"#);

/// `pipette`.
pub const PIPETTE: Icon = lucide!("pipette", r#"<path d="m12 9-8.414 8.414A2 2 0 0 0 3 18.828v1.344a2 2 0 0 1-.586 1.414A2 2 0 0 1 3.828 21h1.344a2 2 0 0 0 1.414-.586L15 12"/><path d="m18 9 .4.4a1 1 0 1 1-3 3l-3.8-3.8a1 1 0 1 1 3-3l.4.4 3.4-3.4a1 1 0 1 1 3 3z"/><path d="m2 22 .414-.414"/>"#);

/// `pizza`.
pub const PIZZA: Icon = lucide!("pizza", r#"<path d="m12 14-1 1"/><path d="m13.75 18.25-1.25 1.42"/><path d="M17.775 5.654a15.68 15.68 0 0 0-12.121 12.12"/><path d="M18.8 9.3a1 1 0 0 0 2.1 7.7"/><path d="M21.964 20.732a1 1 0 0 1-1.232 1.232l-18-5a1 1 0 0 1-.695-1.232A19.68 19.68 0 0 1 15.732 2.037a1 1 0 0 1 1.232.695z"/>"#);

/// `plane`.
pub const PLANE: Icon = lucide!("plane", r#"<path d="M17.8 19.2 16 11l3.5-3.5C21 6 21.5 4 21 3c-1-.5-3 0-4.5 1.5L13 8 4.8 6.2c-.5-.1-.9.1-1.1.5l-.3.5c-.2.5-.1 1 .3 1.3L9 12l-2 3H4l-1 1 3 2 2 3 1-1v-3l3-2 3.5 5.3c.3.4.8.5 1.3.3l.5-.2c.4-.3.6-.7.5-1.2z"/>"#);

/// `plane-landing`.
pub const PLANE_LANDING: Icon = lucide!("plane-landing", r#"<path d="M2 22h20"/><path d="M3.77 10.77 2 9l2-4.5 1.1.55c.55.28.9.84.9 1.45s.35 1.17.9 1.45L8 8.5l3-6 1.05.53a2 2 0 0 1 1.09 1.52l.72 5.4a2 2 0 0 0 1.09 1.52l4.4 2.2c.42.22.78.55 1.01.96l.6 1.03c.49.88-.06 1.98-1.06 2.1l-1.18.15c-.47.06-.95-.02-1.37-.24L4.29 11.15a2 2 0 0 1-.52-.38Z"/>"#);

/// `plane-takeoff`.
pub const PLANE_TAKEOFF: Icon = lucide!("plane-takeoff", r#"<path d="M2 22h20"/><path d="M6.36 17.4 4 17l-2-4 1.1-.55a2 2 0 0 1 1.8 0l.17.1a2 2 0 0 0 1.8 0L8 12 5 6l.9-.45a2 2 0 0 1 2.09.2l4.02 3a2 2 0 0 0 2.1.2l4.19-2.06a2.41 2.41 0 0 1 1.73-.17L21 7a1.4 1.4 0 0 1 .87 1.99l-.38.76c-.23.46-.6.84-1.07 1.08L7.58 17.2a2 2 0 0 1-1.22.18Z"/>"#);

/// `plant-pot`.
pub const PLANT_POT: Icon = lucide!("plant-pot", r#"<path d="M14 8.536V6a4 4 0 014-4h1.5a.5.5 0 01.5.5V4a4 4 0 01-4 4 4 4 0 00-4 4 5 5 0 01-8-4 5 5 0 018 4c0 2 1 3 1 5"/><path d="m18 17-1.085 3.58A2 2 0 0115 22H9.002a2 2 0 01-1.913-1.418L6 17"/><path d="M5 17h14"/>"#);

/// `play`.
pub const PLAY: Icon = lucide!("play", r#"<path d="M5 5a2 2 0 0 1 3.008-1.728l11.997 6.998a2 2 0 0 1 .003 3.458l-12 7A2 2 0 0 1 5 19z"/>"#);

/// `play-off`.
pub const PLAY_OFF: Icon = lucide!("play-off", r#"<path d="m10.215 4.56 9.79 5.71a2 2 0 0 1 .003 3.458l-.393.23"/><path d="m16.042 16.042-8.034 4.686A2 2 0 0 1 5 19V5"/><path d="m2 2 20 20"/>"#);

/// `playing-card`.
pub const PLAYING_CARD: Icon = lucide!("playing-card", r#"<path d="M12.832 8.445a1 1 0 00-1.589-.098l-2.075 3.098a1 1 0 000 1.11l2 3a1 1 0 001.664 0l2-3a1 1 0 000-1.11z"/><rect x="5" y="2" width="14" height="20" rx="2"/>"#);

/// `playing-cards`.
pub const PLAYING_CARDS: Icon = lucide!("playing-cards", r#"<path d="M14.832 8.445a1 1 0 00-1.589-.098l-2.075 3.098a1 1 0 000 1.11l2 3a1 1 0 001.664 0l2-3a1 1 0 000-1.11z"/><path d="m7.18 20.827-5-11a2 2 0 01.993-2.647L7 5.44"/><rect x="7" y="2" width="14" height="20" rx="2"/>"#);

/// `playing-cards-fan`.
pub const PLAYING_CARDS_FAN: Icon = lucide!("playing-cards-fan", r#"<path d="M12.65 7.65a2 2 0 012.629-1.046l5.51 2.374a2 2 0 011.046 2.628l-3.957 9.184a2 2 0 01-2.628 1.046l-5.51-2.374a2 2 0 01-1.046-2.628z"/><path d="M18 7.777V4a2 2 0 00-2-2h-6a2 2 0 00-2 2v10a2 2 0 001.137 1.805"/><path d="m8 4.389-4.364.809a2 2 0 00-1.602 2.33l1.822 9.833a2 2 0 002.331 1.602l2.542-.47"/>"#);

/// `plug`.
pub const PLUG: Icon = lucide!("plug", r#"<path d="M12 22v-5"/><path d="M15 8V2"/><path d="M17 8a1 1 0 0 1 1 1v4a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4V9a1 1 0 0 1 1-1z"/><path d="M9 8V2"/>"#);

/// `plug-2`.
pub const PLUG_2: Icon = lucide!("plug-2", r#"<path d="M9 2v6"/><path d="M15 2v6"/><path d="M12 17v5"/><path d="M5 8h14"/><path d="M6 11V8h12v3a6 6 0 1 1-12 0Z"/>"#);

/// `plug-zap`.
pub const PLUG_ZAP: Icon = lucide!("plug-zap", r#"<path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"/><path d="m2 22 3-3"/><path d="M7.5 13.5 10 11"/><path d="M10.5 16.5 13 14"/><path d="m18 3-4 4h6l-4 4"/>"#);

/// `plus`.
pub const PLUS: Icon = lucide!("plus", r#"<path d="M5 12h14"/><path d="M12 5v14"/>"#);

/// `pocket-knife`.
pub const POCKET_KNIFE: Icon = lucide!("pocket-knife", r#"<path d="M3 2v1c0 1 2 1 2 2S3 6 3 7s2 1 2 2-2 1-2 2 2 1 2 2"/><path d="M18 6h.01"/><path d="M6 18h.01"/><path d="M20.83 8.83a4 4 0 0 0-5.66-5.66l-12 12a4 4 0 1 0 5.66 5.66Z"/><path d="M18 11.66V22a4 4 0 0 0 4-4V6"/>"#);

/// `podium`.
pub const PODIUM: Icon = lucide!("podium", r#"<path d="M12 6V2h-1"/><path d="M9 15a1 1 0 0 0-1-1H4a1 1 0 0 0-1 1v5a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1v-3a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1"/><path d="M9 21V11a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v10"/>"#);

/// `pointer`.
pub const POINTER: Icon = lucide!("pointer", r#"<path d="M22 14a8 8 0 0 1-8 8"/><path d="M18 11v-1a2 2 0 0 0-2-2a2 2 0 0 0-2 2"/><path d="M14 10V9a2 2 0 0 0-2-2a2 2 0 0 0-2 2v1"/><path d="M10 9.5V4a2 2 0 0 0-2-2a2 2 0 0 0-2 2v10"/><path d="M18 11a2 2 0 1 1 4 0v3a8 8 0 0 1-8 8h-2c-2.8 0-4.5-.86-5.99-2.34l-3.6-3.6a2 2 0 0 1 2.83-2.82L7 15"/>"#);

/// `pointer-off`.
pub const POINTER_OFF: Icon = lucide!("pointer-off", r#"<path d="M10 4.5V4a2 2 0 0 0-2.41-1.957"/><path d="M13.9 8.4a2 2 0 0 0-1.26-1.295"/><path d="M21.7 16.2A8 8 0 0 0 22 14v-3a2 2 0 1 0-4 0v-1a2 2 0 0 0-3.63-1.158"/><path d="m7 15-1.8-1.8a2 2 0 0 0-2.79 2.86L6 19.7a7.74 7.74 0 0 0 6 2.3h2a8 8 0 0 0 5.657-2.343"/><path d="M6 6v8"/><path d="m2 2 20 20"/>"#);

/// `popcorn`.
pub const POPCORN: Icon = lucide!("popcorn", r#"<path d="M18 8a2 2 0 0 0 0-4 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0-4 0 2 2 0 0 0 0 4"/><path d="M10 22 9 8"/><path d="m14 22 1-14"/><path d="M20 8c.5 0 .9.4.8 1l-2.6 12c-.1.5-.7 1-1.2 1H7c-.6 0-1.1-.4-1.2-1L3.2 9c-.1-.6.3-1 .8-1Z"/>"#);

/// `popsicle`.
pub const POPSICLE: Icon = lucide!("popsicle", r#"<path d="M18.6 14.4c.8-.8.8-2 0-2.8l-8.1-8.1a4.95 4.95 0 1 0-7.1 7.1l8.1 8.1c.9.7 2.1.7 2.9-.1Z"/><path d="m22 22-5.5-5.5"/>"#);

/// `pound-sterling`.
pub const POUND_STERLING: Icon = lucide!("pound-sterling", r#"<path d="M17 18a2 2 0 01-2 2H5a3 3 0 003-3V8a4 4 0 018 0"/><path d="M6 12h7"/>"#);

/// `power`.
pub const POWER: Icon = lucide!("power", r#"<path d="M12 2v10"/><path d="M18.4 6.6a9 9 0 1 1-12.77.04"/>"#);

/// `power-off`.
pub const POWER_OFF: Icon = lucide!("power-off", r#"<path d="M18.36 6.64A9 9 0 0 1 20.77 15"/><path d="M6.16 6.16a9 9 0 1 0 12.68 12.68"/><path d="M12 2v4"/><path d="m2 2 20 20"/>"#);

/// `presentation`.
pub const PRESENTATION: Icon = lucide!("presentation", r#"<path d="M2 3h20"/><path d="M21 3v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V3"/><path d="m7 21 5-5 5 5"/>"#);

/// `printer`.
pub const PRINTER: Icon = lucide!("printer", r#"<path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"/><path d="M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6"/><rect x="6" y="14" width="12" height="8" rx="1"/>"#);

/// `printer-3d`.
pub const PRINTER_3D: Icon = lucide!("printer-3d", r#"<path d="M10 11v1"/><path d="M12 8h8"/><path d="M15 20v-3a1 1 0 00-1-1H9a1 1 0 00-1 1v3"/><path d="M4 20h16"/><path d="M4 22V4a2 2 0 012-2h12a2 2 0 012 2v18"/><path d="M4 8h4"/><path d="M8.635 10.093A2 2 0 018 8.631V7a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 01-.293.707l-1 1a1 1 0 01-1.414 0z"/>"#);

/// `printer-check`.
pub const PRINTER_CHECK: Icon = lucide!("printer-check", r#"<path d="M13.5 22H7a1 1 0 0 1-1-1v-6a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v.5"/><path d="m16 19 2 2 4-4"/><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2"/><path d="M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6"/>"#);

/// `printer-x`.
pub const PRINTER_X: Icon = lucide!("printer-x", r#"<path d="M12.531 22H7a1 1 0 0 1-1-1v-6a1 1 0 0 1 1-1h6.377"/><path d="m16.5 16.5 5 5"/><path d="m16.5 21.5 5-5"/><path d="M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v1.5"/><path d="M6 9V3a1 1 0 0 1 1-1h10a1 1 0 0 1 1 1v6"/>"#);

/// `projector`.
pub const PROJECTOR: Icon = lucide!("projector", r#"<path d="M5 7 3 5"/><path d="M9 6V3"/><path d="m13 7 2-2"/><circle cx="9" cy="13" r="3"/><path d="M11.83 12H20a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h2.17"/><path d="M16 16h2"/>"#);

/// `proportions`.
pub const PROPORTIONS: Icon = lucide!("proportions", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M12 9v11"/><path d="M2 9h13a2 2 0 0 1 2 2v9"/>"#);

/// `puzzle`.
pub const PUZZLE: Icon = lucide!("puzzle", r#"<path d="M15.39 4.39a1 1 0 0 0 1.68-.474 2.5 2.5 0 1 1 3.014 3.015 1 1 0 0 0-.474 1.68l1.683 1.682a2.414 2.414 0 0 1 0 3.414L19.61 15.39a1 1 0 0 1-1.68-.474 2.5 2.5 0 1 0-3.014 3.015 1 1 0 0 1 .474 1.68l-1.683 1.682a2.414 2.414 0 0 1-3.414 0L8.61 19.61a1 1 0 0 0-1.68.474 2.5 2.5 0 1 1-3.014-3.015 1 1 0 0 0 .474-1.68l-1.683-1.682a2.414 2.414 0 0 1 0-3.414L4.39 8.61a1 1 0 0 1 1.68.474 2.5 2.5 0 1 0 3.014-3.015 1 1 0 0 1-.474-1.68l1.683-1.682a2.414 2.414 0 0 1 3.414 0z"/>"#);

/// `pyramid`.
pub const PYRAMID: Icon = lucide!("pyramid", r#"<path d="M2.5 16.88a1 1 0 0 1-.32-1.43l9-13.02a1 1 0 0 1 1.64 0l9 13.01a1 1 0 0 1-.32 1.44l-8.51 4.86a2 2 0 0 1-1.98 0Z"/><path d="M12 2v20"/>"#);

/// `qr-code`.
pub const QR_CODE: Icon = lucide!("qr-code", r#"<rect width="5" height="5" x="3" y="3" rx="1"/><rect width="5" height="5" x="16" y="3" rx="1"/><rect width="5" height="5" x="3" y="16" rx="1"/><path d="M21 16h-3a2 2 0 0 0-2 2v3"/><path d="M21 21v.01"/><path d="M12 7v3a2 2 0 0 1-2 2H7"/><path d="M3 12h.01"/><path d="M12 3h.01"/><path d="M12 16v.01"/><path d="M16 12h1"/><path d="M21 12v.01"/><path d="M12 21v-1"/>"#);

/// `quote`.
pub const QUOTE: Icon = lucide!("quote", r#"<path d="M16 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/><path d="M5 3a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2 1 1 0 0 1 1 1v1a2 2 0 0 1-2 2 1 1 0 0 0-1 1v2a1 1 0 0 0 1 1 6 6 0 0 0 6-6V5a2 2 0 0 0-2-2z"/>"#);

/// `rabbit`.
pub const RABBIT: Icon = lucide!("rabbit", r#"<path d="M13 16a3 3 0 0 1 2.24 5"/><path d="M18 12h.01"/><path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1 1 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1a3 3 0 0 0-3 3"/><path d="M20 8.54V4a2 2 0 1 0-4 0v3"/><path d="M7.612 12.524a3 3 0 1 0-1.6 4.3"/>"#);

/// `radar`.
pub const RADAR: Icon = lucide!("radar", r#"<path d="M19.07 4.93A10 10 0 0 0 6.99 3.34"/><path d="M4 6h.01"/><path d="M2.29 9.62A10 10 0 1 0 21.31 8.35"/><path d="M16.24 7.76A6 6 0 1 0 8.23 16.67"/><path d="M12 18h.01"/><path d="M17.99 11.66A6 6 0 0 1 15.77 16.67"/><circle cx="12" cy="12" r="2"/><path d="m13.41 10.59 5.66-5.66"/>"#);

/// `radiation`.
pub const RADIATION: Icon = lucide!("radiation", r#"<path d="M12 12h.01"/><path d="M14 15.4641a4 4 0 0 1-4 0L7.52786 19.74597 A 1 1 0 0 0 7.99303 21.16211 10 10 0 0 0 16.00697 21.16211 1 1 0 0 0 16.47214 19.74597z"/><path d="M16 12a4 4 0 0 0-2-3.464l2.472-4.282a1 1 0 0 1 1.46-.305 10 10 0 0 1 4.006 6.94A1 1 0 0 1 21 12z"/><path d="M8 12a4 4 0 0 1 2-3.464L7.528 4.254a1 1 0 0 0-1.46-.305 10 10 0 0 0-4.006 6.94A1 1 0 0 0 3 12z"/>"#);

/// `radical`.
pub const RADICAL: Icon = lucide!("radical", r#"<path d="M3 12h3.28a1 1 0 0 1 .948.684l2.298 7.934a.5.5 0 0 0 .96-.044L13.82 4.771A1 1 0 0 1 14.792 4H21"/>"#);

/// `radio`.
pub const RADIO: Icon = lucide!("radio", r#"<path d="M16.247 7.761a6 6 0 0 1 0 8.478"/><path d="M19.075 4.933a10 10 0 0 1 0 14.134"/><path d="M4.925 19.067a10 10 0 0 1 0-14.134"/><path d="M7.753 16.239a6 6 0 0 1 0-8.478"/><circle cx="12" cy="12" r="2"/>"#);

/// `radio-off`.
pub const RADIO_OFF: Icon = lucide!("radio-off", r#"<path d="M13.414 13.414a2 2 0 1 1-2.828-2.828"/><path d="M16.247 7.761a6 6 0 0 1 1.744 4.572"/><path d="M19.075 4.933a10 10 0 0 1 2.234 10.72"/><path d="m2 2 20 20"/><path d="M4.925 19.067a10 10 0 0 1 0-14.134"/><path d="M7.753 16.239a6 6 0 0 1 0-8.478"/>"#);

/// `radio-receiver`.
pub const RADIO_RECEIVER: Icon = lucide!("radio-receiver", r#"<path d="M5 16v2"/><path d="M19 16v2"/><rect width="20" height="8" x="2" y="8" rx="2"/><path d="M18 12h.01"/>"#);

/// `radio-tower`.
pub const RADIO_TOWER: Icon = lucide!("radio-tower", r#"<path d="M4.9 16.1C1 12.2 1 5.8 4.9 1.9"/><path d="M7.8 4.7a6.14 6.14 0 0 0-.8 7.5"/><circle cx="12" cy="9" r="2"/><path d="M16.2 4.8c2 2 2.26 5.11.8 7.47"/><path d="M19.1 1.9a9.96 9.96 0 0 1 0 14.1"/><path d="M9.5 18h5"/><path d="m8 22 4-11 4 11"/>"#);

/// `radius`.
pub const RADIUS: Icon = lucide!("radius", r#"<path d="M20.34 17.52a10 10 0 1 0-2.82 2.82"/><circle cx="19" cy="19" r="2"/><path d="m13.41 13.41 4.18 4.18"/><circle cx="12" cy="12" r="2"/>"#);

/// `rainbow`.
pub const RAINBOW: Icon = lucide!("rainbow", r#"<path d="M22 17a10 10 0 0 0-20 0"/><path d="M6 17a6 6 0 0 1 12 0"/><path d="M10 17a2 2 0 0 1 4 0"/>"#);

/// `rat`.
pub const RAT: Icon = lucide!("rat", r#"<path d="M13 22H4a2 2 0 0 1 0-4h12"/><path d="M13.236 18a3 3 0 0 0-2.2-5"/><path d="M16 9h.01"/><path d="M16.82 3.94a3 3 0 1 1 3.237 4.868l1.815 2.587a1.5 1.5 0 0 1-1.5 2.1l-2.872-.453a3 3 0 0 0-3.5 3"/><path d="M17 4.988a3 3 0 1 0-5.2 2.052A7 7 0 0 0 4 14.015 4 4 0 0 0 8 18"/>"#);

/// `ratio`.
pub const RATIO: Icon = lucide!("ratio", r#"<rect width="12" height="20" x="6" y="2" rx="2"/><rect width="20" height="12" x="2" y="6" rx="2"/>"#);

/// `receipt`.
pub const RECEIPT: Icon = lucide!("receipt", r#"<path d="M12 17V7"/><path d="M16 8h-6a2 2 0 0 0 0 4h4a2 2 0 0 1 0 4H8"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/>"#);

/// `receipt-cent`.
pub const RECEIPT_CENT: Icon = lucide!("receipt-cent", r#"<path d="M12 7v10"/><path d="M14.828 14.829a4 4 0 0 1-5.656 0 4 4 0 0 1 0-5.657 4 4 0 0 1 5.656 0"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/>"#);

/// `receipt-euro`.
pub const RECEIPT_EURO: Icon = lucide!("receipt-euro", r#"<path d="M15.828 14.829a4 4 0 0 1-5.656 0 4 4 0 0 1 0-5.657 4 4 0 0 1 5.656 0"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M8 12h5"/>"#);

/// `receipt-indian-rupee`.
pub const RECEIPT_INDIAN_RUPEE: Icon = lucide!("receipt-indian-rupee", r#"<path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M8 11h8"/><path d="M8 7h8"/><path d="M9 7a4 4 0 0 1 0 8H8l3 2"/>"#);

/// `receipt-japanese-yen`.
pub const RECEIPT_JAPANESE_YEN: Icon = lucide!("receipt-japanese-yen", r#"<path d="m12 10 3-3"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M9 11h6"/><path d="M9 15h6"/><path d="m9 7 3 3v7"/>"#);

/// `receipt-pound-sterling`.
pub const RECEIPT_POUND_STERLING: Icon = lucide!("receipt-pound-sterling", r#"<path d="M10 17V9.5a1 1 0 0 1 5 0"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M8 13h5"/><path d="M8 17h7"/>"#);

/// `receipt-russian-ruble`.
pub const RECEIPT_RUSSIAN_RUBLE: Icon = lucide!("receipt-russian-ruble", r#"<path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M8 11h5a2 2 0 0 0 0-4h-3v10"/><path d="M8 15h5"/>"#);

/// `receipt-swiss-franc`.
pub const RECEIPT_SWISS_FRANC: Icon = lucide!("receipt-swiss-franc", r#"<path d="M10 11h4"/><path d="M10 17V7h5"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/><path d="M8 15h5"/>"#);

/// `receipt-text`.
pub const RECEIPT_TEXT: Icon = lucide!("receipt-text", r#"<path d="M13 16H8"/><path d="M14 8H8"/><path d="M16 12H8"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/>"#);

/// `receipt-turkish-lira`.
pub const RECEIPT_TURKISH_LIRA: Icon = lucide!("receipt-turkish-lira", r#"<path d="M10 7v10a5 5 0 0 0 5-5"/><path d="m14 8-6 3"/><path d="M4 3a1 1 0 0 1 1-1 1.3 1.3 0 0 1 .7.2l.933.6a1.3 1.3 0 0 0 1.4 0l.934-.6a1.3 1.3 0 0 1 1.4 0l.933.6a1.3 1.3 0 0 0 1.4 0l.933-.6a1.3 1.3 0 0 1 1.4 0l.934.6a1.3 1.3 0 0 0 1.4 0l.933-.6A1.3 1.3 0 0 1 19 2a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1 1.3 1.3 0 0 1-.7-.2l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.934.6a1.3 1.3 0 0 1-1.4 0l-.933-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-1.4 0l-.934-.6a1.3 1.3 0 0 0-1.4 0l-.933.6a1.3 1.3 0 0 1-.7.2 1 1 0 0 1-1-1z"/>"#);

/// `rectangle-circle`.
pub const RECTANGLE_CIRCLE: Icon = lucide!("rectangle-circle", r#"<path d="M14 4v16H3a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1z"/><circle cx="14" cy="12" r="8"/>"#);

/// `rectangle-ellipsis`.
pub const RECTANGLE_ELLIPSIS: Icon = lucide!("rectangle-ellipsis", r#"<rect width="20" height="12" x="2" y="6" rx="2"/><path d="M12 12h.01"/><path d="M17 12h.01"/><path d="M7 12h.01"/>"#);

/// `rectangle-goggles`.
pub const RECTANGLE_GOGGLES: Icon = lucide!("rectangle-goggles", r#"<path d="M20 6a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-4a2 2 0 0 1-1.6-.8l-1.6-2.13a1 1 0 0 0-1.6 0L9.6 17.2A2 2 0 0 1 8 18H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z"/>"#);

/// `rectangle-horizontal`.
pub const RECTANGLE_HORIZONTAL: Icon = lucide!("rectangle-horizontal", r#"<rect width="20" height="12" x="2" y="6" rx="2"/>"#);

/// `rectangle-vertical`.
pub const RECTANGLE_VERTICAL: Icon = lucide!("rectangle-vertical", r#"<rect width="12" height="20" x="6" y="2" rx="2"/>"#);

/// `recycle`.
pub const RECYCLE: Icon = lucide!("recycle", r#"<path d="M7 19H4.815a1.83 1.83 0 0 1-1.57-.881 1.785 1.785 0 0 1-.004-1.784L7.196 9.5"/><path d="M11 19h8.203a1.83 1.83 0 0 0 1.556-.89 1.784 1.784 0 0 0 0-1.775l-1.226-2.12"/><path d="m14 16-3 3 3 3"/><path d="M8.293 13.596 7.196 9.5 3.1 10.598"/><path d="m9.344 5.811 1.093-1.892A1.83 1.83 0 0 1 11.985 3a1.784 1.784 0 0 1 1.546.888l3.943 6.843"/><path d="m13.378 9.633 4.096 1.098 1.097-4.096"/>"#);

/// `redo`.
pub const REDO: Icon = lucide!("redo", r#"<path d="M21 7v6h-6"/><path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"/>"#);

/// `redo-2`.
pub const REDO_2: Icon = lucide!("redo-2", r#"<path d="m15 14 5-5-5-5"/><path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5A5.5 5.5 0 0 0 9.5 20H13"/>"#);

/// `redo-dot`.
pub const REDO_DOT: Icon = lucide!("redo-dot", r#"<circle cx="12" cy="17" r="1"/><path d="M21 7v6h-6"/><path d="M3 17a9 9 0 0 1 9-9 9 9 0 0 1 6 2.3l3 2.7"/>"#);

/// `refresh-ccw`.
pub const REFRESH_CCW: Icon = lucide!("refresh-ccw", r#"<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 16h5v5"/>"#);

/// `refresh-ccw-dot`.
pub const REFRESH_CCW_DOT: Icon = lucide!("refresh-ccw-dot", r#"<path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16"/><path d="M16 16h5v5"/><circle cx="12" cy="12" r="1"/>"#);

/// `refresh-cw`.
pub const REFRESH_CW: Icon = lucide!("refresh-cw", r#"<path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M8 16H3v5"/>"#);

/// `refresh-cw-off`.
pub const REFRESH_CW_OFF: Icon = lucide!("refresh-cw-off", r#"<path d="M21 8L18.74 5.74A9.75 9.75 0 0 0 12 3C11 3 10.03 3.16 9.13 3.47"/><path d="M8 16H3v5"/><path d="M3 12C3 9.51 4 7.26 5.64 5.64"/><path d="m3 16 2.26 2.26A9.75 9.75 0 0 0 12 21c2.49 0 4.74-1 6.36-2.64"/><path d="M21 12c0 1-.16 1.97-.47 2.87"/><path d="M21 3v5h-5"/><path d="M22 22 2 2"/>"#);

/// `refrigerator`.
pub const REFRIGERATOR: Icon = lucide!("refrigerator", r#"<path d="M5 6a4 4 0 0 1 4-4h6a4 4 0 0 1 4 4v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6Z"/><path d="M5 10h14"/><path d="M15 7v6"/>"#);

/// `regex`.
pub const REGEX: Icon = lucide!("regex", r#"<path d="M17 3v10"/><path d="m12.67 5.5 8.66 5"/><path d="m12.67 10.5 8.66-5"/><path d="M9 17a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2a2 2 0 0 0 2 2h2a2 2 0 0 0 2-2v-2z"/>"#);

/// `remove-formatting`.
pub const REMOVE_FORMATTING: Icon = lucide!("remove-formatting", r#"<path d="M4 7V4h16v3"/><path d="M5 20h6"/><path d="M13 4 8 20"/><path d="m15 15 5 5"/><path d="m20 15-5 5"/>"#);

/// `repeat`.
pub const REPEAT: Icon = lucide!("repeat", r#"<path d="m17 2 4 4-4 4"/><path d="M3 11v-1a4 4 0 0 1 4-4h14"/><path d="m7 22-4-4 4-4"/><path d="M21 13v1a4 4 0 0 1-4 4H3"/>"#);

/// `repeat-1`.
pub const REPEAT_1: Icon = lucide!("repeat-1", r#"<path d="m17 2 4 4-4 4"/><path d="M3 11v-1a4 4 0 0 1 4-4h14"/><path d="m7 22-4-4 4-4"/><path d="M21 13v1a4 4 0 0 1-4 4H3"/><path d="M11 10h1v4"/>"#);

/// `repeat-2`.
pub const REPEAT_2: Icon = lucide!("repeat-2", r#"<path d="m2 9 3-3 3 3"/><path d="M13 18H7a2 2 0 0 1-2-2V6"/><path d="m22 15-3 3-3-3"/><path d="M11 6h6a2 2 0 0 1 2 2v10"/>"#);

/// `repeat-off`.
pub const REPEAT_OFF: Icon = lucide!("repeat-off", r#"<path d="M11.656 6H21l-4-4"/><path d="M17.898 17.898A4 4 0 0 1 17 18H3l4-4"/><path d="m2 2 20 20"/><path d="M21 13v1a4 4 0 0 1-.171 1.159"/><path d="m21 6-4 4"/><path d="M3 11v-1a4 4 0 0 1 3.102-3.898"/><path d="m7 22-4-4"/>"#);

/// `replace`.
pub const REPLACE: Icon = lucide!("replace", r#"<path d="M14 4a1 1 0 0 1 1-1"/><path d="M15 10a1 1 0 0 1-1-1"/><path d="M21 4a1 1 0 0 0-1-1"/><path d="M21 9a1 1 0 0 1-1 1"/><path d="m3 7 3 3 3-3"/><path d="M6 10V5a2 2 0 0 1 2-2h2"/><rect x="3" y="14" width="7" height="7" rx="1"/>"#);

/// `replace-all`.
pub const REPLACE_ALL: Icon = lucide!("replace-all", r#"<path d="M14 14a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1"/><path d="M14 4a1 1 0 0 1 1-1"/><path d="M15 10a1 1 0 0 1-1-1"/><path d="M19 14a1 1 0 0 1 1 1v5a1 1 0 0 1-1 1"/><path d="M21 4a1 1 0 0 0-1-1"/><path d="M21 9a1 1 0 0 1-1 1"/><path d="m3 7 3 3 3-3"/><path d="M6 10V5a2 2 0 0 1 2-2h2"/><rect x="3" y="14" width="7" height="7" rx="1"/>"#);

/// `reply`.
pub const REPLY: Icon = lucide!("reply", r#"<path d="M20 18v-2a4 4 0 0 0-4-4H4"/><path d="m9 17-5-5 5-5"/>"#);

/// `reply-all`.
pub const REPLY_ALL: Icon = lucide!("reply-all", r#"<path d="m12 17-5-5 5-5"/><path d="M22 18v-2a4 4 0 0 0-4-4H7"/><path d="m7 17-5-5 5-5"/>"#);

/// `rewind`.
pub const REWIND: Icon = lucide!("rewind", r#"<path d="M12 6a2 2 0 0 0-3.414-1.414l-6 6a2 2 0 0 0 0 2.828l6 6A2 2 0 0 0 12 18z"/><path d="M22 6a2 2 0 0 0-3.414-1.414l-6 6a2 2 0 0 0 0 2.828l6 6A2 2 0 0 0 22 18z"/>"#);

/// `ribbon`.
pub const RIBBON: Icon = lucide!("ribbon", r#"<path d="M12 11.22C11 9.997 10 9 10 8a2 2 0 0 1 4 0c0 1-.998 2.002-2.01 3.22"/><path d="m12 18 2.57-3.5"/><path d="M6.243 9.016a7 7 0 0 1 11.507-.009"/><path d="M9.35 14.53 12 11.22"/><path d="M9.35 14.53C7.728 12.246 6 10.221 6 7a6 5 0 0 1 12 0c-.005 3.22-1.778 5.235-3.43 7.5l3.557 4.527a1 1 0 0 1-.203 1.43l-1.894 1.36a1 1 0 0 1-1.384-.215L12 18l-2.679 3.593a1 1 0 0 1-1.39.213l-1.865-1.353a1 1 0 0 1-.203-1.422z"/>"#);

/// `road`.
pub const ROAD: Icon = lucide!("road", r#"<path d="M12 17v4"/><path d="M12 5V3"/><path d="M12 9v3"/><path d="M2.077 18.449A2 2 0 0 0 4 21h16a2 2 0 0 0 1.924-2.55l-4-14A2 2 0 0 0 16 3H8a2 2 0 0 0-1.924 1.45z"/>"#);

/// `robot-arm`.
pub const ROBOT_ARM: Icon = lucide!("robot-arm", r#"<path d="M12 21 7.5 8.322"/><path d="m14 7 1.75-3.767a.5.5 0 0 1 .662-.172L20 5.005"/><path d="m20 8.998-3.588 1.944a.5.5 0 0 1-.662-.172L14 7H8"/><path d="M3.486 21h10"/><path d="M5 21V8.732"/><circle cx="6" cy="7" r="2"/>"#);

/// `robot-vacuum`.
pub const ROBOT_VACUUM: Icon = lucide!("robot-vacuum", r#"<path d="M11 17h2"/><path d="M12 12h.01"/><path d="M17 12a5 5 0 00-10 0"/><path d="M19 2v2.8"/><path d="M2 5h2.8"/><path d="M22 5h-2.8"/><path d="M5 2v2.8"/><circle cx="12" cy="12" r="10"/>"#);

/// `rocket`.
pub const ROCKET: Icon = lucide!("rocket", r#"<path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09"/><path d="M9 12a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.4 22.4 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 .05 5 .05"/>"#);

/// `rocking-chair`.
pub const ROCKING_CHAIR: Icon = lucide!("rocking-chair", r#"<path d="m15 13 3.708 7.416"/><path d="M3 19a15 15 0 0 0 18 0"/><path d="m3 2 3.21 9.633A2 2 0 0 0 8.109 13H18"/><path d="m9 13-3.708 7.416"/>"#);

/// `roller-coaster`.
pub const ROLLER_COASTER: Icon = lucide!("roller-coaster", r#"<path d="M6 19V5"/><path d="M10 19V6.8"/><path d="M14 19v-7.8"/><path d="M18 5v4"/><path d="M18 19v-6"/><path d="M22 19V9"/><path d="M2 19V9a4 4 0 0 1 4-4c2 0 4 1.33 6 4s4 4 6 4a4 4 0 1 0-3-6.65"/>"#);

/// `rose`.
pub const ROSE: Icon = lucide!("rose", r#"<path d="M17 10h-1a4 4 0 1 1 4-4v.534"/><path d="M17 6h1a4 4 0 0 1 1.42 7.74l-2.29.87a6 6 0 0 1-5.339-10.68l2.069-1.31"/><path d="M4.5 17c2.8-.5 4.4 0 5.5.8s1.8 2.2 2.3 3.7c-2 .4-3.5.4-4.8-.3-1.2-.6-2.3-1.9-3-4.2"/><path d="M9.77 12C4 15 2 22 2 22"/><circle cx="17" cy="8" r="2"/>"#);

/// `rotate-3d`.
pub const ROTATE_3D: Icon = lucide!("rotate-3d", r#"<path d="m15.194 13.707 3.814 1.86-1.86 3.814"/><path d="M16.47214 7.52786 A 5 10 0 1 0 13 21.79796"/><path d="M21.79796 11 A 10 5 0 1 0 19 15.57071"/>"#);

/// `rotate-ccw`.
pub const ROTATE_CCW: Icon = lucide!("rotate-ccw", r#"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/>"#);

/// `rotate-ccw-clock`.
pub const ROTATE_CCW_CLOCK: Icon = lucide!("rotate-ccw-clock", r#"<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><path d="M12 7v5l4 2"/>"#);

/// `rotate-ccw-key`.
pub const ROTATE_CCW_KEY: Icon = lucide!("rotate-ccw-key", r#"<path d="M12 7v6"/><path d="M12 9h2"/><path d="M3 12a9 9 0 1 0 9-9 9.74 9.74 0 0 0-6.74 2.74L3 8"/><path d="M3 3v5h5"/><circle cx="12" cy="15" r="2"/>"#);

/// `rotate-ccw-square`.
pub const ROTATE_CCW_SQUARE: Icon = lucide!("rotate-ccw-square", r#"<path d="M20 9V7a2 2 0 0 0-2-2h-6"/><path d="m15 2-3 3 3 3"/><path d="M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2"/>"#);

/// `rotate-cw`.
pub const ROTATE_CW: Icon = lucide!("rotate-cw", r#"<path d="M21 12a9 9 0 1 1-9-9c2.52 0 4.93 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/>"#);

/// `rotate-cw-clock`.
pub const ROTATE_CW_CLOCK: Icon = lucide!("rotate-cw-clock", r#"<path d="M12 7v5l4 2"/><path d="M16 8h5V3"/><path d="m21 8-2.3-2.3A9.7 9.7 0 0012 3a9 9 0 109 9"/>"#);

/// `rotate-cw-fading-clock`.
pub const ROTATE_CW_FADING_CLOCK: Icon = lucide!("rotate-cw-fading-clock", r#"<path d="M12 3a9.75 9.75 0 0 1 6.74 2.74"/><path d="M18.74 5.74 21 8"/><path d="M21 8V3"/><path d="M7.5 19.794c-6-3.464-6-12.124 0-15.588"/><path d="M7.5 4.206A9 9 0 0 1 12 3"/><path d="M12 7v5l4 2"/><path d="M14 20.775A9 9 0 0 1 12 21"/><path d="M19 17.656a9 9 0 0 1-1.5 1.456"/><path d="M21 12a9 9 0 0 1-.228 2"/><path d="M21 8h-5"/>"#);

/// `rotate-cw-square`.
pub const ROTATE_CW_SQUARE: Icon = lucide!("rotate-cw-square", r#"<path d="M12 5H6a2 2 0 0 0-2 2v3"/><path d="m9 8 3-3-3-3"/><path d="M4 14v4a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-2"/>"#);

/// `route`.
pub const ROUTE: Icon = lucide!("route", r#"<circle cx="6" cy="19" r="3"/><path d="M9 19h8.5a3.5 3.5 0 0 0 0-7h-11a3.5 3.5 0 0 1 0-7H15"/><circle cx="18" cy="5" r="3"/>"#);

/// `route-off`.
pub const ROUTE_OFF: Icon = lucide!("route-off", r#"<circle cx="6" cy="19" r="3"/><path d="M9 19h8.5c.4 0 .9-.1 1.3-.2"/><path d="M5.2 5.2A3.5 3.53 0 0 0 6.5 12H12"/><path d="m2 2 20 20"/><path d="M21 15.3a3.5 3.5 0 0 0-3.3-3.3"/><path d="M15 5h-4.3"/><circle cx="18" cy="5" r="3"/>"#);

/// `router`.
pub const ROUTER: Icon = lucide!("router", r#"<rect width="20" height="8" x="2" y="14" rx="2"/><path d="M6.01 18H6"/><path d="M10.01 18H10"/><path d="M15 10v4"/><path d="M17.84 7.17a4 4 0 0 0-5.66 0"/><path d="M20.66 4.34a8 8 0 0 0-11.31 0"/>"#);

/// `rows-2`.
pub const ROWS_2: Icon = lucide!("rows-2", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 12h18"/>"#);

/// `rows-3`.
pub const ROWS_3: Icon = lucide!("rows-3", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M21 9H3"/><path d="M21 15H3"/>"#);

/// `rows-4`.
pub const ROWS_4: Icon = lucide!("rows-4", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M21 7.5H3"/><path d="M21 12H3"/><path d="M21 16.5H3"/>"#);

/// `rss`.
pub const RSS: Icon = lucide!("rss", r#"<path d="M4 11a9 9 0 0 1 9 9"/><path d="M4 4a16 16 0 0 1 16 16"/><circle cx="5" cy="19" r="1"/>"#);

/// `rugby-ball`.
pub const RUGBY_BALL: Icon = lucide!("rugby-ball", r#"<path d="m10 10 4 4"/><path d="m13 7 4 4"/><path d="M15.34 2.138A15 15 0 002.138 15.34c-.357 2.94.004 4.919.805 5.717.798.8 2.778 1.162 5.718.805A15 15 0 0021.862 8.661c.357-2.94-.004-4.92-.805-5.718-.798-.8-2.778-1.162-5.717-.805"/><path d="M17 7 7 17"/><path d="m7 13 4 4"/>"#);

/// `ruler`.
pub const RULER: Icon = lucide!("ruler", r#"<path d="M21.3 15.3a2.4 2.4 0 0 1 0 3.4l-2.6 2.6a2.4 2.4 0 0 1-3.4 0L2.7 8.7a2.41 2.41 0 0 1 0-3.4l2.6-2.6a2.41 2.41 0 0 1 3.4 0Z"/><path d="m14.5 12.5 2-2"/><path d="m11.5 9.5 2-2"/><path d="m8.5 6.5 2-2"/><path d="m17.5 15.5 2-2"/>"#);

/// `ruler-dimension-line`.
pub const RULER_DIMENSION_LINE: Icon = lucide!("ruler-dimension-line", r#"<path d="M10 15v-3"/><path d="M14 15v-3"/><path d="M18 15v-3"/><path d="M2 8V4"/><path d="M22 6H2"/><path d="M22 8V4"/><path d="M6 15v-3"/><rect x="2" y="12" width="20" height="8" rx="2"/>"#);

/// `russian-ruble`.
pub const RUSSIAN_RUBLE: Icon = lucide!("russian-ruble", r#"<path d="M4 16h6"/><path d="M7 20V5a1 1 0 011-1h6a4 4 0 010 8H4"/>"#);

/// `sailboat`.
pub const SAILBOAT: Icon = lucide!("sailboat", r#"<path d="M10 2v15"/><path d="M7 22a4 4 0 0 1-4-4 1 1 0 0 1 1-1h16a1 1 0 0 1 1 1 4 4 0 0 1-4 4z"/><path d="M9.159 2.46a1 1 0 0 1 1.521-.193l9.977 8.98A1 1 0 0 1 20 13H4a1 1 0 0 1-.824-1.567z"/>"#);

/// `salad`.
pub const SALAD: Icon = lucide!("salad", r#"<path d="M19.496 12a2.5 2.5 0 00.399-2.214A2 2 0 0020.32 6.5 2 2 0 0019 3a2 2 0 00-1.5.68 2 2 0 00-3.287.424 2.5 2.5 0 00-3.189 2.06A3 3 0 0012 12l4-4"/><path d="M4 12a1 1 0 00-.99 1.133A9 9 0 0012 21a9 9 0 008.99-7.867A1 1 0 0020 12z"/><path d="M7 21h10"/><path d="M9.85 6.907A3.5 3.5 0 005.05 12"/>"#);

/// `sandwich`.
pub const SANDWICH: Icon = lucide!("sandwich", r#"<path d="m2.37 11.223 8.372-6.777a2 2 0 0 1 2.516 0l8.371 6.777"/><path d="M21 15a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-5.25"/><path d="M3 15a1 1 0 0 0-1 1v2a1 1 0 0 0 1 1h9"/><path d="m6.67 15 6.13 4.6a2 2 0 0 0 2.8-.4l3.15-4.2"/><rect width="20" height="4" x="2" y="11" rx="1"/>"#);

/// `satellite`.
pub const SATELLITE: Icon = lucide!("satellite", r#"<path d="m13.5 6.5-3.148-3.148a1.205 1.205 0 0 0-1.704 0L6.352 5.648a1.205 1.205 0 0 0 0 1.704L9.5 10.5"/><path d="M16.5 7.5 19 5"/><path d="m17.5 10.5 3.148 3.148a1.205 1.205 0 0 1 0 1.704l-2.296 2.296a1.205 1.205 0 0 1-1.704 0L13.5 14.5"/><path d="M9 21a6 6 0 0 0-6-6"/><path d="M9.352 10.648a1.205 1.205 0 0 0 0 1.704l2.296 2.296a1.205 1.205 0 0 0 1.704 0l4.296-4.296a1.205 1.205 0 0 0 0-1.704l-2.296-2.296a1.205 1.205 0 0 0-1.704 0z"/>"#);

/// `satellite-dish`.
pub const SATELLITE_DISH: Icon = lucide!("satellite-dish", r#"<path d="M18 12a6 6 0 00-6-6"/><path d="M2.824 10.459a8 8 0 0010.717 10.717c.558-.276.623-1.012.183-1.452l-9.448-9.448c-.44-.44-1.176-.375-1.452.183"/><path d="M22 12A10 10 0 0012 2"/><path d="m9 15 4-4"/>"#);

/// `saudi-riyal`.
pub const SAUDI_RIYAL: Icon = lucide!("saudi-riyal", r#"<path d="M10 2.977v13.19a2 2 0 01-1.23 1.846L4 19.999"/><path d="M14 4v9.523a1 1 0 001.371.928L19 13"/><path d="m19 18-5 2"/><path d="M19 8 4 14"/>"#);

/// `save`.
pub const SAVE: Icon = lucide!("save", r#"<path d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"/><path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"/><path d="M7 3v4a1 1 0 0 0 1 1h7"/>"#);

/// `save-all`.
pub const SAVE_ALL: Icon = lucide!("save-all", r#"<path d="M10 2v3a1 1 0 0 0 1 1h5"/><path d="M18 18v-6a1 1 0 0 0-1-1h-6a1 1 0 0 0-1 1v6"/><path d="M18 22H4a2 2 0 0 1-2-2V6"/><path d="M8 18a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9.172a2 2 0 0 1 1.414.586l2.828 2.828A2 2 0 0 1 22 6.828V16a2 2 0 0 1-2.01 2z"/>"#);

/// `save-check`.
pub const SAVE_CHECK: Icon = lucide!("save-check", r#"<path d="M12.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h10.2a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4v4.35"/><path d="m16 19 2 2 4-4"/><path d="M17 15.13V14a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"/><path d="M7 3v4a1 1 0 0 0 1 1h7"/>"#);

/// `save-off`.
pub const SAVE_OFF: Icon = lucide!("save-off", r#"<path d="M13 13H8a1 1 0 0 0-1 1v7"/><path d="M14 8h1"/><path d="M17 21v-4"/><path d="m2 2 20 20"/><path d="M20.41 20.41A2 2 0 0 1 19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 .59-1.41"/><path d="M9 3h6.2a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V15"/>"#);

/// `save-pen`.
pub const SAVE_PEN: Icon = lucide!("save-pen", r#"<path d="M13.33 13H8a1 1 0 00-1 1v7"/><path d="M14.363 17.634a2 2 0 00-.506.854l-.837 2.87a.5.5 0 00.62.62l2.87-.837a2 2 0 00.854-.506l4.013-4.009a1 1 0 10-3.004-3.004z"/><path d="M7 3v4a1 1 0 001 1h7"/><path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h10.2a2 2 0 011.4.6l3.8 3.8a2 2 0 01.6 1.4v.3"/>"#);

/// `save-plus`.
pub const SAVE_PLUS: Icon = lucide!("save-plus", r#"<path d="M12.5 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h10.2a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V12"/><path d="M16 13H8a1 1 0 0 0-1 1v7"/><path d="M19 22v-6"/><path d="M22 19h-6"/><path d="M7 3v4a1 1 0 0 0 1 1h7"/>"#);

/// `scale`.
pub const SCALE: Icon = lucide!("scale", r#"<path d="M12 3v18"/><path d="m19 8 3 8a5 5 0 0 1-6 0zV7"/><path d="M3 7h1a17 17 0 0 0 8-2 17 17 0 0 0 8 2h1"/><path d="m5 8 3 8a5 5 0 0 1-6 0zV7"/><path d="M7 21h10"/>"#);

/// `scale-3d`.
pub const SCALE_3D: Icon = lucide!("scale-3d", r#"<path d="M5 7v11a1 1 0 0 0 1 1h11"/><path d="M5.293 18.707 11 13"/><circle cx="19" cy="19" r="2"/><circle cx="5" cy="5" r="2"/>"#);

/// `scaling`.
pub const SCALING: Icon = lucide!("scaling", r#"<path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M14 15H9v-5"/><path d="M16 3h5v5"/><path d="M21 3 9 15"/>"#);

/// `scan`.
pub const SCAN: Icon = lucide!("scan", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/>"#);

/// `scan-barcode`.
pub const SCAN_BARCODE: Icon = lucide!("scan-barcode", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M8 7v10"/><path d="M12 7v10"/><path d="M17 7v10"/>"#);

/// `scan-box`.
pub const SCAN_BOX: Icon = lucide!("scan-box", r#"<path d="M12 12v5.5"/><path d="M17 3h2a2 2 0 012 2v2"/><path d="M21 17v2a2 2 0 01-2 2h-2"/><path d="M3 7V5a2 2 0 012-2h2"/><path d="M7 21H5a2 2 0 01-2-2v-2"/><path d="M7.264 9.252 12 12l4.737-2.748"/><path d="M7.995 8.514A2 2 0 007 10.244v3.516a2 2 0 00.996 1.73l3 1.74a2 2 0 002.008 0l3-1.74A2 2 0 0017 13.76v-3.517a2 2 0 00-.995-1.73l-3-1.742a2 2 0 00-1.892-.064z"/>"#);

/// `scan-eye`.
pub const SCAN_EYE: Icon = lucide!("scan-eye", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><circle cx="12" cy="12" r="1"/><path d="M18.944 12.33a1 1 0 0 0 0-.66 7.5 7.5 0 0 0-13.888 0 1 1 0 0 0 0 .66 7.5 7.5 0 0 0 13.888 0"/>"#);

/// `scan-face`.
pub const SCAN_FACE: Icon = lucide!("scan-face", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><path d="M9 9h.01"/><path d="M15 9h.01"/>"#);

/// `scan-heart`.
pub const SCAN_HEART: Icon = lucide!("scan-heart", r#"<path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M7.828 13.07A3 3 0 0 1 12 8.764a3 3 0 0 1 4.172 4.306l-3.447 3.62a1 1 0 0 1-1.449 0z"/>"#);

/// `scan-line`.
pub const SCAN_LINE: Icon = lucide!("scan-line", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M7 12h10"/>"#);

/// `scan-qr-code`.
pub const SCAN_QR_CODE: Icon = lucide!("scan-qr-code", r#"<path d="M17 12v4a1 1 0 0 1-1 1h-4"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M17 8V7"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M7 17h.01"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><rect x="7" y="7" width="5" height="5" rx="1"/>"#);

/// `scan-search`.
pub const SCAN_SEARCH: Icon = lucide!("scan-search", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><circle cx="12" cy="12" r="3"/><path d="m16 16-1.9-1.9"/>"#);

/// `scan-square`.
pub const SCAN_SQUARE: Icon = lucide!("scan-square", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><rect width="8" height="8" x="8" y="8" rx="1"/>"#);

/// `scan-text`.
pub const SCAN_TEXT: Icon = lucide!("scan-text", r#"<path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><path d="M7 8h8"/><path d="M7 12h10"/><path d="M7 16h6"/>"#);

/// `school`.
pub const SCHOOL: Icon = lucide!("school", r#"<path d="M14 21v-3a2 2 0 0 0-4 0v3"/><path d="M18 4.933V21"/><path d="m4 6 7.106-3.79a2 2 0 0 1 1.788 0L20 6"/><path d="m6 11-3.52 2.147a1 1 0 0 0-.48.854V19a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a1 1 0 0 0-.48-.853L18 11"/><path d="M6 4.933V21"/><circle cx="12" cy="9" r="2"/>"#);

/// `scissors`.
pub const SCISSORS: Icon = lucide!("scissors", r#"<circle cx="6" cy="6" r="3"/><path d="M8.12 8.12 12 12"/><path d="M20 4 8.12 15.88"/><circle cx="6" cy="18" r="3"/><path d="M14.8 14.8 20 20"/>"#);

/// `scissors-line-dashed`.
pub const SCISSORS_LINE_DASHED: Icon = lucide!("scissors-line-dashed", r#"<path d="M5.42 9.42 8 12"/><circle cx="4" cy="8" r="2"/><path d="m14 6-8.58 8.58"/><circle cx="4" cy="16" r="2"/><path d="M10.8 14.8 14 18"/><path d="M16 12h-2"/><path d="M22 12h-2"/>"#);

/// `scooter`.
pub const SCOOTER: Icon = lucide!("scooter", r#"<path d="M21 4h-3.5l2 11.05"/><path d="M6.95 17h5.142c.523 0 .95-.406 1.063-.916a6.5 6.5 0 0 1 5.345-5.009"/><circle cx="19.5" cy="17.5" r="2.5"/><circle cx="4.5" cy="17.5" r="2.5"/>"#);

/// `scratch-blocks`.
pub const SCRATCH_BLOCKS: Icon = lucide!("scratch-blocks", r#"<path d="M19 5a2 2 0 012 2v11a2 2 0 01-2 2h-5.586a1 1 0 00-.707.293l-1.414 1.414a1 1 0 01-.707.293H8.414a1 1 0 01-.707-.293l-1.414-1.414A1 1 0 005.586 20H5a2 2 0 01-2-2V5.286c0-.394.11-.785.36-1.09a6 6 0 019.168-.133C12.996 4.6 13.637 5 14.35 5z"/><path d="M21 12h-7.586a1 1 0 00-.707.293l-1.414 1.414a1 1 0 01-.707.293H8.414a1 1 0 01-.707-.293l-1.414-1.414A1 1 0 005.586 12H3"/>"#);

/// `screen-share`.
pub const SCREEN_SHARE: Icon = lucide!("screen-share", r#"<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"/><path d="M8 21h8"/><path d="M12 17v4"/><path d="m17 8 5-5"/><path d="M17 3h5v5"/>"#);

/// `screen-share-off`.
pub const SCREEN_SHARE_OFF: Icon = lucide!("screen-share-off", r#"<path d="M13 3H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-3"/><path d="M8 21h8"/><path d="M12 17v4"/><path d="m22 3-5 5"/><path d="m17 3 5 5"/>"#);

/// `screw`.
pub const SCREW: Icon = lucide!("screw", r#"<path d="M10 9v10"/><path d="M14 13v2"/><path d="M16.5 10.5 5.706 21.294A2.4 2.4 0 014.002 22H2.5a.5.5 0 01-.5-.5v-1.502a2.4 2.4 0 01.706-1.704L13.5 7.5"/><path d="M20.819 11.575a6 6 0 00.969-5.162.6.6 0 00-.981-.219l-.455.454a1.2 1.2 0 01-1.704 0l-1.296-1.296a1.2 1.2 0 010-1.704l.454-.453a.595.595 0 00-.219-.98 6 6 0 00-5.162.967 2 2 0 00-.222 3.019l5.596 5.597a2 2 0 003.02-.222"/><path d="M6 13v8"/>"#);

/// `scroll`.
pub const SCROLL: Icon = lucide!("scroll", r#"<path d="M19 17V5a2 2 0 0 0-2-2H4"/><path d="M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3"/>"#);

/// `scroll-text`.
pub const SCROLL_TEXT: Icon = lucide!("scroll-text", r#"<path d="M15 12h-5"/><path d="M15 8h-5"/><path d="M19 17V5a2 2 0 0 0-2-2H4"/><path d="M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3"/>"#);

/// `search`.
pub const SEARCH: Icon = lucide!("search", r#"<path d="m21 21-4.34-4.34"/><circle cx="11" cy="11" r="8"/>"#);

/// `search-check`.
pub const SEARCH_CHECK: Icon = lucide!("search-check", r#"<path d="m8 11 2 2 4-4"/><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#);

/// `search-code`.
pub const SEARCH_CODE: Icon = lucide!("search-code", r#"<path d="m13 13.5 2-2.5-2-2.5"/><path d="m21 21-4.3-4.3"/><path d="M9 8.5 7 11l2 2.5"/><circle cx="11" cy="11" r="8"/>"#);

/// `search-exclamation-point`.
pub const SEARCH_EXCLAMATION_POINT: Icon = lucide!("search-exclamation-point", r#"<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/><path d="M11 7v4"/><path d="M11 15h.01"/>"#);

/// `search-slash`.
pub const SEARCH_SLASH: Icon = lucide!("search-slash", r#"<path d="m13.5 8.5-5 5"/><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#);

/// `search-x`.
pub const SEARCH_X: Icon = lucide!("search-x", r#"<path d="m13.5 8.5-5 5"/><path d="m8.5 8.5 5 5"/><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>"#);

/// `section`.
pub const SECTION: Icon = lucide!("section", r#"<path d="M16 5a4 3 0 0 0-8 0c0 4 8 3 8 7a4 3 0 0 1-8 0"/><path d="M8 19a4 3 0 0 0 8 0c0-4-8-3-8-7a4 3 0 0 1 8 0"/>"#);

/// `send`.
pub const SEND: Icon = lucide!("send", r#"<path d="M14.536 21.686a.5.5 0 0 0 .937-.024l6.5-19a.496.496 0 0 0-.635-.635l-19 6.5a.5.5 0 0 0-.024.937l7.93 3.18a2 2 0 0 1 1.112 1.11z"/><path d="m21.854 2.147-10.94 10.939"/>"#);

/// `send-horizontal`.
pub const SEND_HORIZONTAL: Icon = lucide!("send-horizontal", r#"<path d="M3.714 3.048a.498.498 0 0 0-.683.627l2.843 7.627a2 2 0 0 1 0 1.396l-2.842 7.627a.498.498 0 0 0 .682.627l18-8.5a.5.5 0 0 0 0-.904z"/><path d="M6 12h16"/>"#);

/// `send-to-back`.
pub const SEND_TO_BACK: Icon = lucide!("send-to-back", r#"<rect x="14" y="14" width="8" height="8" rx="2"/><rect x="2" y="2" width="8" height="8" rx="2"/><path d="M7 14v1a2 2 0 0 0 2 2h1"/><path d="M14 7h1a2 2 0 0 1 2 2v1"/>"#);

/// `separator-horizontal`.
pub const SEPARATOR_HORIZONTAL: Icon = lucide!("separator-horizontal", r#"<path d="m16 16-4 4-4-4"/><path d="M3 12h18"/><path d="m8 8 4-4 4 4"/>"#);

/// `separator-vertical`.
pub const SEPARATOR_VERTICAL: Icon = lucide!("separator-vertical", r#"<path d="M12 3v18"/><path d="m16 16 4-4-4-4"/><path d="m8 8-4 4 4 4"/>"#);

/// `server`.
pub const SERVER: Icon = lucide!("server", r#"<rect width="20" height="8" x="2" y="2" rx="2" ry="2"/><rect width="20" height="8" x="2" y="14" rx="2" ry="2"/><line x1="6" x2="6.01" y1="6" y2="6"/><line x1="6" x2="6.01" y1="18" y2="18"/>"#);

/// `server-cog`.
pub const SERVER_COG: Icon = lucide!("server-cog", r#"<path d="m10.852 14.772-.383.923"/><path d="M13.148 14.772a3 3 0 1 0-2.296-5.544l-.383-.923"/><path d="m13.148 9.228.383-.923"/><path d="m13.53 15.696-.382-.924a3 3 0 1 1-2.296-5.544"/><path d="m14.772 10.852.923-.383"/><path d="m14.772 13.148.923.383"/><path d="M4.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-.5"/><path d="M4.5 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-.5"/><path d="M6 18h.01"/><path d="M6 6h.01"/><path d="m9.228 10.852-.923-.383"/><path d="m9.228 13.148-.923.383"/>"#);

/// `server-crash`.
pub const SERVER_CRASH: Icon = lucide!("server-crash", r#"<path d="M6 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-2"/><path d="M6 14H4a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-4a2 2 0 0 0-2-2h-2"/><path d="M6 6h.01"/><path d="M6 18h.01"/><path d="m13 6-4 6h6l-4 6"/>"#);

/// `server-nas`.
pub const SERVER_NAS: Icon = lucide!("server-nas", r#"<path d="M10 16V8"/><path d="M14 16V8"/><path d="M18 16V8"/><path d="M6 12h.01"/><path d="M6 16h.01"/><rect x="2" y="4" width="20" height="16" rx="2"/>"#);

/// `server-off`.
pub const SERVER_OFF: Icon = lucide!("server-off", r#"<path d="M7 2h13a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-5"/><path d="M10 10 2.5 2.5C2 2 2 2.5 2 5v3a2 2 0 0 0 2 2h6z"/><path d="M22 17v-1a2 2 0 0 0-2-2h-1"/><path d="M4 14a2 2 0 0 0-2 2v4a2 2 0 0 0 2 2h16.5l1-.5.5.5-8-8H4z"/><path d="M6 18h.01"/><path d="m2 2 20 20"/>"#);

/// `server-plus`.
pub const SERVER_PLUS: Icon = lucide!("server-plus", r#"<path d="M12.5 10H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v2"/><path d="M16 12h6"/><path d="M19 9v6"/><path d="M22 18v2a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h8.5"/><path d="M6 18h.01"/><path d="M6 6h.01"/>"#);

/// `settings`.
pub const SETTINGS: Icon = lucide!("settings", r#"<path d="M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"/><circle cx="12" cy="12" r="3"/>"#);

/// `settings-2`.
pub const SETTINGS_2: Icon = lucide!("settings-2", r#"<path d="M14 17H5"/><path d="M19 7h-9"/><circle cx="17" cy="17" r="3"/><circle cx="7" cy="7" r="3"/>"#);

/// `shapes`.
pub const SHAPES: Icon = lucide!("shapes", r#"<path d="M8.3 10a.7.7 0 0 1-.626-1.079L11.4 3a.7.7 0 0 1 1.198-.043L16.3 8.9a.7.7 0 0 1-.572 1.1Z"/><rect x="3" y="14" width="7" height="7" rx="1"/><circle cx="17.5" cy="17.5" r="3.5"/>"#);

/// `share`.
pub const SHARE: Icon = lucide!("share", r#"<path d="M12 2v13"/><path d="m16 6-4-4-4 4"/><path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"/>"#);

/// `share-2`.
pub const SHARE_2: Icon = lucide!("share-2", r#"<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" x2="15.42" y1="13.51" y2="17.49"/><line x1="15.41" x2="8.59" y1="6.51" y2="10.49"/>"#);

/// `sheet`.
pub const SHEET: Icon = lucide!("sheet", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><line x1="3" x2="21" y1="9" y2="9"/><line x1="3" x2="21" y1="15" y2="15"/><line x1="9" x2="9" y1="9" y2="21"/><line x1="15" x2="15" y1="9" y2="21"/>"#);

/// `shell`.
pub const SHELL: Icon = lucide!("shell", r#"<path d="M14 11a2 2 0 1 1-4 0 4 4 0 0 1 8 0 6 6 0 0 1-12 0 8 8 0 0 1 16 0 10 10 0 1 1-20 0 11.93 11.93 0 0 1 2.42-7.22 2 2 0 1 1 3.16 2.44"/>"#);

/// `shelving-unit`.
pub const SHELVING_UNIT: Icon = lucide!("shelving-unit", r#"<path d="M12 12V9a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"/><path d="M16 20v-3a1 1 0 0 0-1-1h-2a1 1 0 0 0-1 1v3"/><path d="M20 22V2"/><path d="M4 12h16"/><path d="M4 20h16"/><path d="M4 2v20"/><path d="M4 4h16"/>"#);

/// `shield`.
pub const SHIELD: Icon = lucide!("shield", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>"#);

/// `shield-ban`.
pub const SHIELD_BAN: Icon = lucide!("shield-ban", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="m4.243 5.21 14.39 12.472"/>"#);

/// `shield-check`.
pub const SHIELD_CHECK: Icon = lucide!("shield-check", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="m9 12 2 2 4-4"/>"#);

/// `shield-cog`.
pub const SHIELD_COG: Icon = lucide!("shield-cog", r#"<path d="m10.929 14.467-.383.924"/><path d="M10.929 8.923 10.546 8"/><path d="M13.225 8.923 13.608 8"/><path d="m13.607 15.391-.382-.924"/><path d="m14.849 10.547.923-.383"/><path d="m14.849 12.843.923.383"/><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="m9.305 10.547-.923-.383"/><path d="m9.305 12.843-.923.383"/><circle cx="12.077" cy="11.695" r="3"/>"#);

/// `shield-cog-corner`.
pub const SHIELD_COG_CORNER: Icon = lucide!("shield-cog-corner", r#"<path d="M11 22c-3.806-1.45-7-3.966-7-9V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v4"/><path d="M14.923 16.547 14 16.164"/><path d="m14.923 18.843-.923.383"/><path d="M16.547 14.923 16.164 14"/><path d="m16.547 20.467-.383.924"/><path d="m18.843 14.923.383-.923"/><path d="m19.225 21.391-.382-.924"/><path d="m20.467 16.547.923-.383"/><path d="m20.467 18.843.923.383"/><circle cx="17.695" cy="17.695" r="3"/>"#);

/// `shield-ellipsis`.
pub const SHIELD_ELLIPSIS: Icon = lucide!("shield-ellipsis", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M8 12h.01"/><path d="M12 12h.01"/><path d="M16 12h.01"/>"#);

/// `shield-exclamation-point`.
pub const SHIELD_EXCLAMATION_POINT: Icon = lucide!("shield-exclamation-point", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M12 8v4"/><path d="M12 16h.01"/>"#);

/// `shield-half`.
pub const SHIELD_HALF: Icon = lucide!("shield-half", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M12 22V2"/>"#);

/// `shield-house`.
pub const SHIELD_HOUSE: Icon = lucide!("shield-house", r#"<path d="M15 22a1 1 0 01-1-1v-4a1 1 0 01.445-.832l3-2a1 1 0 011.11 0l3 2A1 1 0 0122 17v4a1 1 0 01-1 1z"/><path d="M18 22v-3"/><path d="M20 10.418V6a1 1 0 00-1-1c-2 0-4.49-1.19-6.24-2.72a1.17 1.17 0 00-1.52 0C9.5 3.8 7 5 5 5a1 1 0 00-1 1v7c0 4.602 2.67 7.1 6.037 8.603"/>"#);

/// `shield-keyhole`.
pub const SHIELD_KEYHOLE: Icon = lucide!("shield-keyhole", r#"<path d="M12 13v3"/><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 01-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 011-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 011.52 0C14.51 3.81 17 5 19 5a1 1 0 011 1z"/><circle cx="12" cy="11" r="2"/>"#);

/// `shield-lock`.
pub const SHIELD_LOCK: Icon = lucide!("shield-lock", r#"<path d="M20 9.807V6a1 1 0 00-1-1c-2 0-4.49-1.19-6.24-2.72a1.17 1.17 0 00-1.52 0C9.5 3.8 7 5 5 5a1 1 0 00-1 1v7c0 3.88 2.107 6.254 5 7.796"/><path d="M19 17v-2a2 2 0 00-4 0v2"/><rect x="13" y="17" width="8" height="5" rx="1"/>"#);

/// `shield-minus`.
pub const SHIELD_MINUS: Icon = lucide!("shield-minus", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M9 12h6"/>"#);

/// `shield-off`.
pub const SHIELD_OFF: Icon = lucide!("shield-off", r#"<path d="m2 2 20 20"/><path d="M5 5a1 1 0 0 0-1 1v7c0 5 3.5 7.5 7.67 8.94a1 1 0 0 0 .67.01c2.35-.82 4.48-1.97 5.9-3.71"/><path d="M9.309 3.652A12.252 12.252 0 0 0 11.24 2.28a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1v7a9.784 9.784 0 0 1-.08 1.264"/>"#);

/// `shield-plus`.
pub const SHIELD_PLUS: Icon = lucide!("shield-plus", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M9 12h6"/><path d="M12 9v6"/>"#);

/// `shield-question-mark`.
pub const SHIELD_QUESTION_MARK: Icon = lucide!("shield-question-mark", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M9.1 9a3 3 0 0 1 5.82 1c0 2-3 3-3 3"/><path d="M12 17h.01"/>"#);

/// `shield-user`.
pub const SHIELD_USER: Icon = lucide!("shield-user", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M6.376 18.91a6 6 0 0 1 11.249.003"/><circle cx="12" cy="11" r="4"/>"#);

/// `shield-x`.
pub const SHIELD_X: Icon = lucide!("shield-x", r#"<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="m14.5 9.5-5 5"/><path d="m9.5 9.5 5 5"/>"#);

/// `ship`.
pub const SHIP: Icon = lucide!("ship", r#"<path d="M12 2v2"/><path d="M12 9.189V13"/><path d="M19 12V6a2 2 0 00-2-2H7a2 2 0 00-2 2v6"/><path d="M19.38 19A11.6 11.6 0 0021 13l-8.188-3.639a2 2 0 00-1.624 0L3 13.001a11.6 11.6 0 002.81 7.76"/><path d="M2 20c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1s1.2 1 2.5 1c2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/>"#);

/// `ship-cargo`.
pub const SHIP_CARGO: Icon = lucide!("ship-cargo", r#"<path d="M12 15v-3"/><path d="M12 2v2"/><path d="M16.5 12V9a1 1 0 011-1h1a1 1 0 001-1V5a1 1 0 00-1-1h-13a1 1 0 00-1 1v2a1 1 0 001 1h1a1 1 0 011 1v3"/><path d="M19.38 19c1.076-1.815 1.636-4.89 1.628-6.008a1 1 0 00-1-.992H3.984a1 1 0 00-1 .984c-.03 1.86.97 5.621 2.826 7.776"/><path d="M2 20c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1s1.2 1 2.5 1c2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/>"#);

/// `ship-wheel`.
pub const SHIP_WHEEL: Icon = lucide!("ship-wheel", r#"<circle cx="12" cy="12" r="8"/><path d="M12 2v7.5"/><path d="m19 5-5.23 5.23"/><path d="M22 12h-7.5"/><path d="m19 19-5.23-5.23"/><path d="M12 14.5V22"/><path d="M10.23 13.77 5 19"/><path d="M9.5 12H2"/><path d="M10.23 10.23 5 5"/><circle cx="12" cy="12" r="2.5"/>"#);

/// `shirt`.
pub const SHIRT: Icon = lucide!("shirt", r#"<path d="M20.38 3.46 16 2a4 4 0 0 1-8 0L3.62 3.46a2 2 0 0 0-1.34 2.23l.58 3.47a1 1 0 0 0 .99.84H6v10c0 1.1.9 2 2 2h8a2 2 0 0 0 2-2V10h2.15a1 1 0 0 0 .99-.84l.58-3.47a2 2 0 0 0-1.34-2.23z"/>"#);

/// `shopping-bag`.
pub const SHOPPING_BAG: Icon = lucide!("shopping-bag", r#"<path d="M16 10a4 4 0 0 1-8 0"/><path d="M3.103 6.034h17.794"/><path d="M3.4 5.467a2 2 0 0 0-.4 1.2V20a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6.667a2 2 0 0 0-.4-1.2l-2-2.667A2 2 0 0 0 17 2H7a2 2 0 0 0-1.6.8z"/>"#);

/// `shopping-basket`.
pub const SHOPPING_BASKET: Icon = lucide!("shopping-basket", r#"<path d="m15 11-1 9"/><path d="m19 11-4-7"/><path d="M2 11h20"/><path d="m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8a2 2 0 0 0 2-1.6l1.7-7.4"/><path d="M4.5 15.5h15"/><path d="m5 11 4-7"/><path d="m9 11 1 9"/>"#);

/// `shopping-cart`.
pub const SHOPPING_CART: Icon = lucide!("shopping-cart", r#"<path d="m2.05 2.05 1.099-.028a1 1 0 0 1 1.008.815l2.69 14.347A1 1 0 0 0 7.83 18H18"/><path d="M4.563 5h16.435a1 1 0 0 1 .981 1.204l-1.026 6.226A2 2 0 0 1 18.962 14H6.25"/><circle cx="18" cy="20" r="2"/><circle cx="8" cy="20" r="2"/>"#);

/// `shopping-cart-minus`.
pub const SHOPPING_CART_MINUS: Icon = lucide!("shopping-cart-minus", r#"<path d="M16 5h6"/><path d="m2.05 2.05 1.099-.028a1 1 0 011.008.815l2.69 14.347A1 1 0 007.83 18H18"/><path d="M4.564 5H12"/><path d="M6.25 14h12.712a2 2 0 001.991-1.57l.514-3.113"/><circle cx="18" cy="20" r="2"/><circle cx="8" cy="20" r="2"/>"#);

/// `shopping-cart-plus`.
pub const SHOPPING_CART_PLUS: Icon = lucide!("shopping-cart-plus", r#"<path d="M16 5h6"/><path d="M19 2v6"/><path d="m2.05 2.05 1.099-.028a1 1 0 011.008.815l2.69 14.347A1 1 0 007.83 18H18"/><path d="M4.564 5H12"/><path d="M6.25 14h12.712a2 2 0 001.991-1.57l.172-1.041"/><circle cx="18" cy="20" r="2"/><circle cx="8" cy="20" r="2"/>"#);

/// `shovel`.
pub const SHOVEL: Icon = lucide!("shovel", r#"<path d="M21.56 4.56a1.5 1.5 0 0 1 0 2.122l-.47.47a3 3 0 0 1-4.212-.03 3 3 0 0 1 0-4.243l.44-.44a1.5 1.5 0 0 1 2.121 0z"/><path d="M3 22a1 1 0 0 1-1-1v-3.586a1 1 0 0 1 .293-.707l3.355-3.355a1.205 1.205 0 0 1 1.704 0l3.296 3.296a1.205 1.205 0 0 1 0 1.704l-3.355 3.355a1 1 0 0 1-.707.293z"/><path d="m9 15 7.879-7.878"/>"#);

/// `shower-head`.
pub const SHOWER_HEAD: Icon = lucide!("shower-head", r#"<path d="m4 4 2.5 2.5"/><path d="M13.5 6.5a4.95 4.95 0 0 0-7 7"/><path d="M15 5 5 15"/><path d="M14 17v.01"/><path d="M10 16v.01"/><path d="M13 13v.01"/><path d="M16 10v.01"/><path d="M11 20v.01"/><path d="M17 14v.01"/><path d="M20 11v.01"/>"#);

/// `shredder`.
pub const SHREDDER: Icon = lucide!("shredder", r#"<path d="M4 13V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 20 8v5"/><path d="M14 2v5a1 1 0 0 0 1 1h5"/><path d="M10 22v-5"/><path d="M14 19v-2"/><path d="M18 20v-3"/><path d="M2 13h20"/><path d="M6 20v-3"/>"#);

/// `shrimp`.
pub const SHRIMP: Icon = lucide!("shrimp", r#"<path d="M10 2a3.28 3.28 0 003.227 1.798l6.17-.561A1 1 0 1119.614 8H8.5a6.44 6.44 0 00-5.63 9.75A6.5 6.5 0 008.5 21c1.38 0 2-.5 2.5-1"/><path d="M10 8a8.5 8.5 0 000 8"/><path d="M11 22c-.5-.5-1.12-1-2.5-1a1 1 0 010-5H12a7 7 0 007-7V8"/><path d="M13 12h.01"/><path d="M8 16c-2 0-4.5-4-4-6"/>"#);

/// `shrimp-off`.
pub const SHRIMP_OFF: Icon = lucide!("shrimp-off", r#"<path d="M10 2a3.28 3.28 0 003.227 1.798l6.17-.561A1 1 0 1119.614 8H13.5"/><path d="M11 20c-.5.5-1.12 1-2.5 1a1 1 0 010-5H12a7 7 0 003.283-.817"/><path d="M11 22c-.5-.5-1.12-1-2.5-1a6.5 6.5 0 01-5.63-3.25 6.44 6.44 0 015.236-9.744"/><path d="M18.04 12.54A7 7 0 0019 9V8"/><path d="m2 2 20 20"/><path d="M8 16c-2 0-4.5-4-4-6"/><path d="M9.43 9.33A8.5 8.5 0 0010 16"/>"#);

/// `shrink`.
pub const SHRINK: Icon = lucide!("shrink", r#"<path d="m15 15 6 6m-6-6v4.8m0-4.8h4.8"/><path d="M9 19.8V15m0 0H4.2M9 15l-6 6"/><path d="M15 4.2V9m0 0h4.8M15 9l6-6"/><path d="M9 4.2V9m0 0H4.2M9 9 3 3"/>"#);

/// `shrub`.
pub const SHRUB: Icon = lucide!("shrub", r#"<path d="M12 22v-5.172a2 2 0 0 0-.586-1.414L9.5 13.5"/><path d="M14.5 14.5 12 17"/><path d="M17 8.8A6 6 0 0 1 13.8 20H10A6.5 6.5 0 0 1 7 8a5 5 0 0 1 10 0z"/>"#);

/// `shuffle`.
pub const SHUFFLE: Icon = lucide!("shuffle", r#"<path d="m18 14 4 4-4 4"/><path d="m18 2 4 4-4 4"/><path d="M2 18h1.973a4 4 0 0 0 3.3-1.7l5.454-8.6a4 4 0 0 1 3.3-1.7H22"/><path d="M2 6h1.972a4 4 0 0 1 3.6 2.2"/><path d="M22 18h-6.041a4 4 0 0 1-3.3-1.8l-.359-.45"/>"#);

/// `sigma`.
pub const SIGMA: Icon = lucide!("sigma", r#"<path d="M18 7V5a1 1 0 0 0-1-1H6.5a.5.5 0 0 0-.4.8l4.5 6a2 2 0 0 1 0 2.4l-4.5 6a.5.5 0 0 0 .4.8H17a1 1 0 0 0 1-1v-2"/>"#);

/// `signal`.
pub const SIGNAL: Icon = lucide!("signal", r#"<path d="M2 20h.01"/><path d="M7 20v-4"/><path d="M12 20v-8"/><path d="M17 20V8"/><path d="M22 4v16"/>"#);

/// `signal-high`.
pub const SIGNAL_HIGH: Icon = lucide!("signal-high", r#"<path d="M2 20h.01"/><path d="M7 20v-4"/><path d="M12 20v-8"/><path d="M17 20V8"/>"#);

/// `signal-low`.
pub const SIGNAL_LOW: Icon = lucide!("signal-low", r#"<path d="M2 20h.01"/><path d="M7 20v-4"/>"#);

/// `signal-medium`.
pub const SIGNAL_MEDIUM: Icon = lucide!("signal-medium", r#"<path d="M2 20h.01"/><path d="M7 20v-4"/><path d="M12 20v-8"/>"#);

/// `signal-zero`.
pub const SIGNAL_ZERO: Icon = lucide!("signal-zero", r#"<path d="M2 20h.01"/>"#);

/// `signature`.
pub const SIGNATURE: Icon = lucide!("signature", r#"<path d="m21 17-2.156-1.868A.5.5 0 0 0 18 15.5v.5a1 1 0 0 1-1 1h-2a1 1 0 0 1-1-1c0-2.545-3.991-3.97-8.5-4a1 1 0 0 0 0 5c4.153 0 4.745-11.295 5.708-13.5a2.5 2.5 0 1 1 3.31 3.284"/><path d="M3 21h18"/>"#);

/// `signpost`.
pub const SIGNPOST: Icon = lucide!("signpost", r#"<path d="M12 13v8"/><path d="M12 3v3"/><path d="M2.354 10.354a1.207 1.207 0 0 1 0-1.708l2.06-2.06A2 2 0 0 1 5.828 6h12.344a2 2 0 0 1 1.414.586l2.06 2.06a1.207 1.207 0 0 1 0 1.708l-2.06 2.06a2 2 0 0 1-1.414.586H5.828a2 2 0 0 1-1.414-.586z"/>"#);

/// `signpost-big`.
pub const SIGNPOST_BIG: Icon = lucide!("signpost-big", r#"<path d="M10 9H4L2 7l2-2h6"/><path d="M14 5h6l2 2-2 2h-6"/><path d="M10 22V4a2 2 0 1 1 4 0v18"/><path d="M8 22h8"/>"#);

/// `siren`.
pub const SIREN: Icon = lucide!("siren", r#"<path d="M7 18v-6a5 5 0 1 1 10 0v6"/><path d="M5 21a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2z"/><path d="M21 12h1"/><path d="M18.5 4.5 18 5"/><path d="M2 12h1"/><path d="M12 2v1"/><path d="m4.929 4.929.707.707"/><path d="M12 12v6"/>"#);

/// `siren-off`.
pub const SIREN_OFF: Icon = lucide!("siren-off", r#"<path d="M12 12v6"/><path d="M12 2v1"/><path d="M12.707 7.05a5 5 0 0 1 4.243 4.243"/><path d="M17 17v1H7a2 2 0 0 0-2 2v1a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2"/><path d="M18.5 4.5 18 5"/><path d="M2 12h1"/><path d="m2 2 20 20"/><path d="M21 12h1"/><path d="M7 18v-6a5 5 0 0 1 1.464-3.536"/>"#);

/// `sixteenth-notes`.
pub const SIXTEENTH_NOTES: Icon = lucide!("sixteenth-notes", r#"<path d="M21 16V3a1 1 0 0 0-1.28-.96l-10 2.917a1 1 0 0 0-.72.96V19"/><path d="M21 6 9 9.5"/><circle cx="18" cy="16" r="3"/><circle cx="6" cy="19" r="3"/>"#);

/// `skip-back`.
pub const SKIP_BACK: Icon = lucide!("skip-back", r#"<path d="M17.971 4.285A2 2 0 0 1 21 6v12a2 2 0 0 1-3.029 1.715l-9.997-5.998a2 2 0 0 1-.003-3.432z"/><path d="M3 20V4"/>"#);

/// `skip-forward`.
pub const SKIP_FORWARD: Icon = lucide!("skip-forward", r#"<path d="M21 4v16"/><path d="M6.029 4.285A2 2 0 0 0 3 6v12a2 2 0 0 0 3.029 1.715l9.997-5.998a2 2 0 0 0 .003-3.432z"/>"#);

/// `skull`.
pub const SKULL: Icon = lucide!("skull", r#"<path d="m12.5 17-.5-1-.5 1h1z"/><path d="M15 22a1 1 0 0 0 1-1v-1a2 2 0 0 0 1.56-3.25 8 8 0 1 0-11.12 0A2 2 0 0 0 8 20v1a1 1 0 0 0 1 1z"/><circle cx="15" cy="12" r="1"/><circle cx="9" cy="12" r="1"/>"#);

/// `slash`.
pub const SLASH: Icon = lucide!("slash", r#"<path d="M22 2 2 22"/>"#);

/// `slice`.
pub const SLICE: Icon = lucide!("slice", r#"<path d="M11 16.586V19a1 1 0 0 1-1 1H2L18.37 3.63a1 1 0 1 1 3 3l-9.663 9.663a1 1 0 0 1-1.414 0L8 14"/>"#);

/// `sliders-horizontal`.
pub const SLIDERS_HORIZONTAL: Icon = lucide!("sliders-horizontal", r#"<path d="M10 5H3"/><path d="M12 19H3"/><path d="M14 3v4"/><path d="M16 17v4"/><path d="M21 12h-9"/><path d="M21 19h-5"/><path d="M21 5h-7"/><path d="M8 10v4"/><path d="M8 12H3"/>"#);

/// `sliders-vertical`.
pub const SLIDERS_VERTICAL: Icon = lucide!("sliders-vertical", r#"<path d="M10 8h4"/><path d="M12 21v-9"/><path d="M12 8V3"/><path d="M17 16h4"/><path d="M19 12V3"/><path d="M19 21v-5"/><path d="M3 14h4"/><path d="M5 10V3"/><path d="M5 21v-7"/>"#);

/// `smartphone`.
pub const SMARTPHONE: Icon = lucide!("smartphone", r#"<rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/>"#);

/// `smartphone-charging`.
pub const SMARTPHONE_CHARGING: Icon = lucide!("smartphone-charging", r#"<rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12.667 8 10 12h4l-2.667 4"/>"#);

/// `smartphone-nfc`.
pub const SMARTPHONE_NFC: Icon = lucide!("smartphone-nfc", r#"<rect width="7" height="12" x="2" y="6" rx="1"/><path d="M13 8.32a7.43 7.43 0 0 1 0 7.36"/><path d="M16.46 6.21a11.76 11.76 0 0 1 0 11.58"/><path d="M19.91 4.1a15.91 15.91 0 0 1 .01 15.8"/>"#);

/// `smartphone-user-round`.
pub const SMARTPHONE_USER_ROUND: Icon = lucide!("smartphone-user-round", r#"<path d="M10 22H6a2 2 0 01-2-2V4a2 2 0 012-2h10a2 2 0 012 2v4"/><path d="M11 18h.01"/><path d="M22 22a4 4 0 00-8 0"/><circle cx="18" cy="15" r="3"/>"#);

/// `snail`.
pub const SNAIL: Icon = lucide!("snail", r#"<path d="M2 13a6 6 0 1 0 12 0 4 4 0 1 0-8 0 2 2 0 0 0 4 0"/><circle cx="10" cy="13" r="8"/><path d="M2 21h12c4.4 0 8-3.6 8-8V7a2 2 0 1 0-4 0v6"/><path d="M18 3 19.1 5.2"/><path d="M22 3 20.9 5.2"/>"#);

/// `snowflake`.
pub const SNOWFLAKE: Icon = lucide!("snowflake", r#"<path d="m10 20-1.25-2.5L6 18"/><path d="M10 4 8.75 6.5 6 6"/><path d="m14 20 1.25-2.5L18 18"/><path d="m14 4 1.25 2.5L18 6"/><path d="m17 21-3-6h-4"/><path d="m17 3-3 6 1.5 3"/><path d="M2 12h6.5L10 9"/><path d="m20 10-1.5 2 1.5 2"/><path d="M22 12h-6.5L14 15"/><path d="m4 10 1.5 2L4 14"/><path d="m7 21 3-6-1.5-3"/><path d="m7 3 3 6h4"/>"#);

/// `soap-dispenser-droplet`.
pub const SOAP_DISPENSER_DROPLET: Icon = lucide!("soap-dispenser-droplet", r#"<path d="M10.5 2v4"/><path d="M14 2H7a2 2 0 0 0-2 2"/><path d="M19.29 14.76A6.67 6.67 0 0 1 17 11a6.6 6.6 0 0 1-2.29 3.76c-1.15.92-1.71 2.04-1.71 3.19 0 2.22 1.8 4.05 4 4.05s4-1.83 4-4.05c0-1.16-.57-2.26-1.71-3.19"/><path d="M9.607 21H6a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h7V7a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"/>"#);

/// `sofa`.
pub const SOFA: Icon = lucide!("sofa", r#"<path d="M20 9V6a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v3"/><path d="M2 16a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-5a2 2 0 0 0-4 0v1.5a.5.5 0 0 1-.5.5h-11a.5.5 0 0 1-.5-.5V11a2 2 0 0 0-4 0z"/><path d="M4 18v2"/><path d="M20 18v2"/><path d="M12 4v9"/>"#);

/// `solar-panel`.
pub const SOLAR_PANEL: Icon = lucide!("solar-panel", r#"<path d="M11 2h2"/><path d="m14.28 14-4.56 8"/><path d="m21 22-1.558-4H4.558"/><path d="M3 10v2"/><path d="M6.245 15.04A2 2 0 0 1 8 14h12a1 1 0 0 1 .864 1.505l-3.11 5.457A2 2 0 0 1 16 22H4a1 1 0 0 1-.863-1.506z"/><path d="M7 2a4 4 0 0 1-4 4"/><path d="m8.66 7.66 1.41 1.41"/>"#);

/// `soup`.
pub const SOUP: Icon = lucide!("soup", r#"<path d="M11.248 3c.273.1.808.53.747 1.36-.05.83-.938 1.2-.989 2.02-.06.78.333 1.24.727 1.62"/><path d="M16.252 3c.268.1.794.53.745 1.36-.06.83-.923 1.2-.993 2.02-.05.78.338 1.24.725 1.62"/><path d="M19.5 12 22 6"/><path d="M4 12a1 1 0 00-.99 1.133A9 9 0 0012 21a9 9 0 008.99-7.867A1 1 0 0020 12z"/><path d="M6.252 3c.268.1.794.53.745 1.36-.06.83-.923 1.2-.993 2.02-.05.78.338 1.24.735 1.62"/><path d="M7 21h10"/>"#);

/// `space`.
pub const SPACE: Icon = lucide!("space", r#"<path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"/>"#);

/// `spade`.
pub const SPADE: Icon = lucide!("spade", r#"<path d="M12 18v4"/><path d="M2 14.499a5.5 5.5 0 0 0 9.591 3.675.6.6 0 0 1 .818.001A5.5 5.5 0 0 0 22 14.5c0-2.29-1.5-4-3-5.5l-5.492-5.312a2 2 0 0 0-3-.02L5 8.999c-1.5 1.5-3 3.2-3 5.5"/>"#);

/// `sparkle`.
pub const SPARKLE: Icon = lucide!("sparkle", r#"<path d="M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z"/>"#);

/// `sparkles`.
pub const SPARKLES: Icon = lucide!("sparkles", r#"<path d="M11.017 2.814a1 1 0 0 1 1.966 0l1.051 5.558a2 2 0 0 0 1.594 1.594l5.558 1.051a1 1 0 0 1 0 1.966l-5.558 1.051a2 2 0 0 0-1.594 1.594l-1.051 5.558a1 1 0 0 1-1.966 0l-1.051-5.558a2 2 0 0 0-1.594-1.594l-5.558-1.051a1 1 0 0 1 0-1.966l5.558-1.051a2 2 0 0 0 1.594-1.594z"/><path d="M20 2v4"/><path d="M22 4h-4"/><circle cx="4" cy="20" r="2"/>"#);

/// `speaker`.
pub const SPEAKER: Icon = lucide!("speaker", r#"<rect width="16" height="20" x="4" y="2" rx="2"/><path d="M12 6h.01"/><circle cx="12" cy="14" r="4"/><path d="M12 14h.01"/>"#);

/// `speech`.
pub const SPEECH: Icon = lucide!("speech", r#"<path d="M8.8 20v-4.1l1.9.2a2.3 2.3 0 0 0 2.164-2.1V8.3A5.37 5.37 0 0 0 2 8.25c0 2.8.656 3.054 1 4.55a5.77 5.77 0 0 1 .029 2.758L2 20"/><path d="M19.8 17.8a7.5 7.5 0 0 0 .003-10.603"/><path d="M17 15a3.5 3.5 0 0 0-.025-4.975"/>"#);

/// `spell-check`.
pub const SPELL_CHECK: Icon = lucide!("spell-check", r#"<path d="m20 15-5.5 5.5L12 18"/><path d="m4 16 6-12 5.115 10.23"/><path d="M6 12h8"/>"#);

/// `spell-check-2`.
pub const SPELL_CHECK_2: Icon = lucide!("spell-check-2", r#"<path d="m6 16 6-12 6 12"/><path d="M8 12h8"/><path d="M4 21c1.1 0 1.1-1 2.3-1s1.1 1 2.3 1c1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1 1.1 0 1.1 1 2.3 1 1.1 0 1.1-1 2.3-1"/>"#);

/// `spline`.
pub const SPLINE: Icon = lucide!("spline", r#"<circle cx="19" cy="5" r="2"/><circle cx="5" cy="19" r="2"/><path d="M5 17A12 12 0 0 1 17 5"/>"#);

/// `spline-pointer`.
pub const SPLINE_POINTER: Icon = lucide!("spline-pointer", r#"<path d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z"/><path d="M5 17A12 12 0 0 1 17 5"/><circle cx="19" cy="5" r="2"/><circle cx="5" cy="19" r="2"/>"#);

/// `split`.
pub const SPLIT: Icon = lucide!("split", r#"<path d="M16 3h5v5"/><path d="M8 3H3v5"/><path d="M12 22v-8.3a4 4 0 0 0-1.172-2.872L3 3"/><path d="m15 9 6-6"/>"#);

/// `spool`.
pub const SPOOL: Icon = lucide!("spool", r#"<path d="M17 13.44 4.442 17.082A2 2 0 0 0 4.982 21H19a2 2 0 0 0 .558-3.921l-1.115-.32A2 2 0 0 1 17 14.837V7.66"/><path d="m7 10.56 12.558-3.642A2 2 0 0 0 19.018 3H5a2 2 0 0 0-.558 3.921l1.115.32A2 2 0 0 1 7 9.163v7.178"/>"#);

/// `sport-shoe`.
pub const SPORT_SHOE: Icon = lucide!("sport-shoe", r#"<path d="m15 10.42 4.8-5.07"/><path d="M19 18h3"/><path d="M9.5 22 21.414 9.415A2 2 0 0 0 21.2 6.4l-5.61-4.208A1 1 0 0 0 14 3v2a2 2 0 0 1-1.394 1.906L8.677 8.053A1 1 0 0 0 8 9c-.155 6.393-2.082 9-4 9a2 2 0 0 0 0 4h14"/>"#);

/// `spotlight`.
pub const SPOTLIGHT: Icon = lucide!("spotlight", r#"<path d="M15.295 19.562 16 22"/><path d="m17 16 3.758 2.098"/><path d="m19 12.5 3.026-.598"/><path d="M7.61 6.3a3 3 0 0 0-3.92 1.3l-1.38 2.79a3 3 0 0 0 1.3 3.91l6.89 3.597a1 1 0 0 0 1.342-.447l3.106-6.211a1 1 0 0 0-.447-1.341z"/><path d="M8 9V2"/>"#);

/// `spray-can`.
pub const SPRAY_CAN: Icon = lucide!("spray-can", r#"<path d="M3 3h.01"/><path d="M7 5h.01"/><path d="M11 7h.01"/><path d="M3 7h.01"/><path d="M7 9h.01"/><path d="M3 11h.01"/><rect width="4" height="4" x="15" y="5"/><path d="m19 9 2 2v10c0 .6-.4 1-1 1h-6c-.6 0-1-.4-1-1V11l2-2"/><path d="m13 14 8-2"/><path d="m13 19 8-2"/>"#);

/// `sprout`.
pub const SPROUT: Icon = lucide!("sprout", r#"<path d="M14 9.536V7a4 4 0 0 1 4-4h1.5a.5.5 0 0 1 .5.5V5a4 4 0 0 1-4 4 4 4 0 0 0-4 4c0 2 1 3 1 5a5 5 0 0 1-1 3"/><path d="M4 9a5 5 0 0 1 8 4 5 5 0 0 1-8-4"/><path d="M5 21h14"/>"#);

/// `square`.
pub const SQUARE: Icon = lucide!("square", r#"<rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-activity`.
pub const SQUARE_ACTIVITY: Icon = lucide!("square-activity", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M17 12h-2l-2 5-2-10-2 5H7"/>"#);

/// `square-arrow-down`.
pub const SQUARE_ARROW_DOWN: Icon = lucide!("square-arrow-down", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 8v8"/><path d="m8 12 4 4 4-4"/>"#);

/// `square-arrow-down-left`.
pub const SQUARE_ARROW_DOWN_LEFT: Icon = lucide!("square-arrow-down-left", r#"<path d="M15 15H9l6-6"/><path d="M9 15V9"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-arrow-down-right`.
pub const SQUARE_ARROW_DOWN_RIGHT: Icon = lucide!("square-arrow-down-right", r#"<path d="M15 15 9 9"/><path d="M9 15h6V9"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-arrow-left`.
pub const SQUARE_ARROW_LEFT: Icon = lucide!("square-arrow-left", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m12 8-4 4 4 4"/><path d="M16 12H8"/>"#);

/// `square-arrow-out-down-left`.
pub const SQUARE_ARROW_OUT_DOWN_LEFT: Icon = lucide!("square-arrow-out-down-left", r#"<path d="M13 21h6a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v6"/><path d="m3 21 9-9"/><path d="M9 21H3v-6"/>"#);

/// `square-arrow-out-down-right`.
pub const SQUARE_ARROW_OUT_DOWN_RIGHT: Icon = lucide!("square-arrow-out-down-right", r#"<path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"/><path d="m21 21-9-9"/><path d="M21 15v6h-6"/>"#);

/// `square-arrow-out-up-left`.
pub const SQUARE_ARROW_OUT_UP_LEFT: Icon = lucide!("square-arrow-out-up-left", r#"<path d="M13 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-6"/><path d="m3 3 9 9"/><path d="M3 9V3h6"/>"#);

/// `square-arrow-out-up-right`.
pub const SQUARE_ARROW_OUT_UP_RIGHT: Icon = lucide!("square-arrow-out-up-right", r#"<path d="M21 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6"/><path d="m21 3-9 9"/><path d="M15 3h6v6"/>"#);

/// `square-arrow-right`.
pub const SQUARE_ARROW_RIGHT: Icon = lucide!("square-arrow-right", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M8 12h8"/><path d="m12 16 4-4-4-4"/>"#);

/// `square-arrow-right-enter`.
pub const SQUARE_ARROW_RIGHT_ENTER: Icon = lucide!("square-arrow-right-enter", r#"<path d="m10 16 4-4-4-4"/><path d="M3 12h11"/><path d="M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"/>"#);

/// `square-arrow-right-exit`.
pub const SQUARE_ARROW_RIGHT_EXIT: Icon = lucide!("square-arrow-right-exit", r#"<path d="M10 12h11"/><path d="m17 16 4-4-4-4"/><path d="M21 6.344V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-1.344"/>"#);

/// `square-arrow-up`.
pub const SQUARE_ARROW_UP: Icon = lucide!("square-arrow-up", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m16 12-4-4-4 4"/><path d="M12 16V8"/>"#);

/// `square-arrow-up-left`.
pub const SQUARE_ARROW_UP_LEFT: Icon = lucide!("square-arrow-up-left", r#"<path d="M15 15 9 9"/><path d="M9 15V9h6"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-arrow-up-right`.
pub const SQUARE_ARROW_UP_RIGHT: Icon = lucide!("square-arrow-up-right", r#"<path d="M15 15V9H9"/><path d="m9 15 6-6"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-asterisk`.
pub const SQUARE_ASTERISK: Icon = lucide!("square-asterisk", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 8v8"/><path d="m8.5 14 7-4"/><path d="m8.5 10 7 4"/>"#);

/// `square-bookmark`.
pub const SQUARE_BOOKMARK: Icon = lucide!("square-bookmark", r#"<path d="M11 3v7.751a.25.25 0 00.407.195l2.28-1.834a.5.5 0 01.627 0l2.28 1.834a.25.25 0 00.406-.195V3"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-bottom-dashed-scissors`.
pub const SQUARE_BOTTOM_DASHED_SCISSORS: Icon = lucide!("square-bottom-dashed-scissors", r#"<path d="M14 21h1"/><path d="m17 17-2.18-2.18"/><path d="M5 21a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v14a2 2 0 01-2 2"/><path d="M9 21h1"/><path d="M9.56 14.44 17 7"/><path d="M9.56 9.56 12 12"/><circle cx="8.5" cy="15.5" r="1.5"/><circle cx="8.5" cy="8.5" r="1.5"/>"#);

/// `square-centerline-dashed-horizontal`.
pub const SQUARE_CENTERLINE_DASHED_HORIZONTAL: Icon = lucide!("square-centerline-dashed-horizontal", r#"<path d="M8 3H5a2 2 0 0 0-2 2v14c0 1.1.9 2 2 2h3"/><path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3"/><path d="M12 20v2"/><path d="M12 14v2"/><path d="M12 8v2"/><path d="M12 2v2"/>"#);

/// `square-centerline-dashed-vertical`.
pub const SQUARE_CENTERLINE_DASHED_VERTICAL: Icon = lucide!("square-centerline-dashed-vertical", r#"<path d="M21 8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v3"/><path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"/><path d="M4 12H2"/><path d="M10 12H8"/><path d="M16 12h-2"/><path d="M22 12h-2"/>"#);

/// `square-chart-gantt`.
pub const SQUARE_CHART_GANTT: Icon = lucide!("square-chart-gantt", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M9 8h7"/><path d="M8 12h6"/><path d="M11 16h5"/>"#);

/// `square-check`.
pub const SQUARE_CHECK: Icon = lucide!("square-check", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m16 9-5.5 5.5L8 12"/>"#);

/// `square-check-big`.
pub const SQUARE_CHECK_BIG: Icon = lucide!("square-check-big", r#"<path d="M21 10.656V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h12.344"/><path d="m9 11 3 3L22 4"/>"#);

/// `square-chevron-down`.
pub const SQUARE_CHEVRON_DOWN: Icon = lucide!("square-chevron-down", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m16 10-4 4-4-4"/>"#);

/// `square-chevron-left`.
pub const SQUARE_CHEVRON_LEFT: Icon = lucide!("square-chevron-left", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m14 16-4-4 4-4"/>"#);

/// `square-chevron-right`.
pub const SQUARE_CHEVRON_RIGHT: Icon = lucide!("square-chevron-right", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m10 8 4 4-4 4"/>"#);

/// `square-chevron-up`.
pub const SQUARE_CHEVRON_UP: Icon = lucide!("square-chevron-up", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m8 14 4-4 4 4"/>"#);

/// `square-chevrons-down`.
pub const SQUARE_CHEVRONS_DOWN: Icon = lucide!("square-chevrons-down", r#"<path d="M9 14L12 17L15 14"/><path d="M9 8L12 11L15 8"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-chevrons-left`.
pub const SQUARE_CHEVRONS_LEFT: Icon = lucide!("square-chevrons-left", r#"<path d="M10 9L7 12L10 15"/><path d="M16 9L13 12L16 15"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-chevrons-right`.
pub const SQUARE_CHEVRONS_RIGHT: Icon = lucide!("square-chevrons-right", r#"<path d="M14 15L17 12L14 9"/><path d="M8 15L11 12L8 9"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-chevrons-up`.
pub const SQUARE_CHEVRONS_UP: Icon = lucide!("square-chevrons-up", r#"<path d="M15 10L12 7L9 10"/><path d="M15 16L12 13L9 16"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-code`.
pub const SQUARE_CODE: Icon = lucide!("square-code", r#"<path d="m10 9-3 3 3 3"/><path d="m14 15 3-3-3-3"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-dashed`.
pub const SQUARE_DASHED: Icon = lucide!("square-dashed", r#"<path d="M5 3a2 2 0 0 0-2 2"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M9 3h1"/><path d="M9 21h1"/><path d="M14 3h1"/><path d="M14 21h1"/><path d="M3 9v1"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M21 14v1"/>"#);

/// `square-dashed-bottom`.
pub const SQUARE_DASHED_BOTTOM: Icon = lucide!("square-dashed-bottom", r#"<path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"/><path d="M9 21h1"/><path d="M14 21h1"/>"#);

/// `square-dashed-bottom-code`.
pub const SQUARE_DASHED_BOTTOM_CODE: Icon = lucide!("square-dashed-bottom-code", r#"<path d="M10 9.5 8 12l2 2.5"/><path d="M14 21h1"/><path d="m14 9.5 2 2.5-2 2.5"/><path d="M5 21a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2"/><path d="M9 21h1"/>"#);

/// `square-dashed-kanban`.
pub const SQUARE_DASHED_KANBAN: Icon = lucide!("square-dashed-kanban", r#"<path d="M8 7v7"/><path d="M12 7v4"/><path d="M16 7v9"/><path d="M5 3a2 2 0 0 0-2 2"/><path d="M9 3h1"/><path d="M14 3h1"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M21 9v1"/><path d="M21 14v1"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M14 21h1"/><path d="M9 21h1"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M3 14v1"/><path d="M3 9v1"/>"#);

/// `square-dashed-mouse-pointer`.
pub const SQUARE_DASHED_MOUSE_POINTER: Icon = lucide!("square-dashed-mouse-pointer", r#"<path d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z"/><path d="M5 3a2 2 0 0 0-2 2"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M9 3h1"/><path d="M9 21h2"/><path d="M14 3h1"/><path d="M3 9v1"/><path d="M21 9v2"/><path d="M3 14v1"/>"#);

/// `square-dashed-plus`.
pub const SQUARE_DASHED_PLUS: Icon = lucide!("square-dashed-plus", r#"<path d="M5 3a2 2 0 0 0-2 2"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M9 3h1"/><path d="M9 21h1"/><path d="M14 3h1"/><path d="M14 21h1"/><path d="M3 9v1"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M21 14v1"/><path d="M8 12h8"/><path d="M12 8v8"/>"#);

/// `square-dashed-search`.
pub const SQUARE_DASHED_SEARCH: Icon = lucide!("square-dashed-search", r#"<path d="M14 3h1"/><path d="M19 3a2 2 0 012 2"/><path d="m21 21-1.875-1.875"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M3 9v1"/><path d="M5 21a2 2 0 01-2-2"/><path d="M5 3a2 2 0 00-2 2"/><path d="M9 21h1"/><path d="M9 3h1"/><circle cx="17" cy="17" r="3"/>"#);

/// `square-dashed-text`.
pub const SQUARE_DASHED_TEXT: Icon = lucide!("square-dashed-text", r#"<path d="M14 21h1"/><path d="M14 3h1"/><path d="M19 3a2 2 0 0 1 2 2"/><path d="M21 14v1"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M3 9v1"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M5 3a2 2 0 0 0-2 2"/><path d="M7 12h10"/><path d="M7 16h6"/><path d="M7 8h8"/><path d="M9 21h1"/><path d="M9 3h1"/>"#);

/// `square-dashed-top-solid`.
pub const SQUARE_DASHED_TOP_SOLID: Icon = lucide!("square-dashed-top-solid", r#"<path d="M14 21h1"/><path d="M21 14v1"/><path d="M21 19a2 2 0 0 1-2 2"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M3 5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2"/><path d="M3 9v1"/><path d="M5 21a2 2 0 0 1-2-2"/><path d="M9 21h1"/>"#);

/// `square-dashed-x`.
pub const SQUARE_DASHED_X: Icon = lucide!("square-dashed-x", r#"<path d="M14 21h1"/><path d="M14 3h1"/><path d="M19 3a2 2 0 012 2"/><path d="M21 14v1"/><path d="M21 19a2 2 0 01-2 2"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M3 9v1"/><path d="M5 21a2 2 0 01-2-2"/><path d="M5 3a2 2 0 00-2 2"/><path d="m9 15 6-6"/><path d="M9 21h1"/><path d="M9 3h1"/><path d="m9 9 6 6"/>"#);

/// `square-dashed-x-corner`.
pub const SQUARE_DASHED_X_CORNER: Icon = lucide!("square-dashed-x-corner", r#"<path d="M14 3h1"/><path d="m16 16 5 5"/><path d="M19 3a2 2 0 012 2"/><path d="m21 16-5 5"/><path d="M21 9v1"/><path d="M3 14v1"/><path d="M3 9v1"/><path d="M5 21a2 2 0 01-2-2"/><path d="M5 3a2 2 0 00-2 2"/><path d="M9 21h1"/><path d="M9 3h1"/>"#);

/// `square-dimensions`.
pub const SQUARE_DIMENSIONS: Icon = lucide!("square-dimensions", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 7H7v5"/><path d="M12 17h5v-5"/>"#);

/// `square-divide`.
pub const SQUARE_DIVIDE: Icon = lucide!("square-divide", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><line x1="8" x2="16" y1="12" y2="12"/><line x1="12" x2="12" y1="16" y2="16"/><line x1="12" x2="12" y1="8" y2="8"/>"#);

/// `square-dot`.
pub const SQUARE_DOT: Icon = lucide!("square-dot", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="12" cy="12" r="1"/>"#);

/// `square-equal`.
pub const SQUARE_EQUAL: Icon = lucide!("square-equal", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 10h10"/><path d="M7 14h10"/>"#);

/// `square-exclamation-point`.
pub const SQUARE_EXCLAMATION_POINT: Icon = lucide!("square-exclamation-point", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/>"#);

/// `square-function`.
pub const SQUARE_FUNCTION: Icon = lucide!("square-function", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="M9 17c2 0 2.8-1 2.8-2.8V10c0-2 1-3.3 3.2-3"/><path d="M9 11.2h5.7"/>"#);

/// `square-kanban`.
pub const SQUARE_KANBAN: Icon = lucide!("square-kanban", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M8 7v7"/><path d="M12 7v4"/><path d="M16 7v9"/>"#);

/// `square-kanban-plus`.
pub const SQUARE_KANBAN_PLUS: Icon = lucide!("square-kanban-plus", r#"<path d="M12 7v4"/><path d="M12.354 21H5a2 2 0 01-2-2V5a2 2 0 012-2h14a2 2 0 012 2v7.354"/><path d="M15 18h6"/><path d="M16 7v4.536"/><path d="M18 21v-6"/><path d="M8 7v7"/>"#);

/// `square-letter-a`.
pub const SQUARE_LETTER_A: Icon = lucide!("square-letter-a", r#"<path d="m8 16 3.453-7.648a.6.6 0 0 1 1.094 0L16 16"/><path d="M15 14H9"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-b`.
pub const SQUARE_LETTER_B: Icon = lucide!("square-letter-b", r#"<path d="M9 12h4.846a2.24 2.24 0 011.523.586 2 2 0 01.001 2.828 2.24 2.24 0 01-1.524.586H9.5a.5.5 0 01-.5-.5v-7a.5.5 0 01.5-.5H13a2 2 0 010 4"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-c`.
pub const SQUARE_LETTER_C: Icon = lucide!("square-letter-c", r#"<path d="M15 9.354a4 4 0 1 0 0 5.292"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-d`.
pub const SQUARE_LETTER_D: Icon = lucide!("square-letter-d", r#"<path d="M12 8a4 4 0 010 8H9.412A.41.41 0 019 15.588V8.412A.41.41 0 019.412 8z"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-e`.
pub const SQUARE_LETTER_E: Icon = lucide!("square-letter-e", r#"<path d="M15 8H9.5a.5.5 0 0 0-.5.5v7a.5.5 0 0 0 .5.5H15"/><path d="M9 12h4"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-f`.
pub const SQUARE_LETTER_F: Icon = lucide!("square-letter-f", r#"<path d="M9 16V8.5a.5.5 0 0 1 .5-.5H15"/><path d="M9 12h4"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-g`.
pub const SQUARE_LETTER_G: Icon = lucide!("square-letter-g", r#"<path d="M12.5 12H15v2.268a.88.88 0 01-.263.65A4 4 0 1112 8h2.5"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-h`.
pub const SQUARE_LETTER_H: Icon = lucide!("square-letter-h", r#"<path d="M9 16V8"/><path d="M15 16V8"/><path d="M9 12h6"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-i`.
pub const SQUARE_LETTER_I: Icon = lucide!("square-letter-i", r#"<path d="M12 16V8"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-j`.
pub const SQUARE_LETTER_J: Icon = lucide!("square-letter-j", r#"<path d="M14 8v5.5a1 1 0 01-5 0V13"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-k`.
pub const SQUARE_LETTER_K: Icon = lucide!("square-letter-k", r#"<path d="M9 16V8"/><path d="m11 12 4 4"/><path d="m15 8-6 6"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-l`.
pub const SQUARE_LETTER_L: Icon = lucide!("square-letter-l", r#"<path d="M9 8v7.5a.5.5 0 0 0 .5.5H15"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-m`.
pub const SQUARE_LETTER_M: Icon = lucide!("square-letter-m", r#"<path d="M8 16.001v-7.5a.5.5 0 0 1 .9-.3l2.7 3.599a.5.5 0 0 0 .8 0l2.7-3.6a.5.5 0 0 1 .9.3v7.501"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-n`.
pub const SQUARE_LETTER_N: Icon = lucide!("square-letter-n", r#"<path d="M9 16V8.576a.496.496 0 01.937-.243l4.126 7.43A.496.496 0 0015 15.52V8"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-o`.
pub const SQUARE_LETTER_O: Icon = lucide!("square-letter-o", r#"<circle cx="12" cy="12" r="4"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-p`.
pub const SQUARE_LETTER_P: Icon = lucide!("square-letter-p", r#"<path d="M9 12h4a2 2 0 0 0 0-4H9.5a.5.5 0 0 0-.5.5V16"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-p-off`.
pub const SQUARE_LETTER_P_OFF: Icon = lucide!("square-letter-p-off", r#"<path d="M12 12h1a2 2 0 000-4H9.5a.5.5 0 00-.5.5V9"/><path d="M15.343 21H5a2 2 0 01-2-2V8.657"/><path d="M22 22 2 2"/><path d="M3.586 3.586A2 2 0 015 3h14a2 2 0 012 2v14a2 2 0 01-.586 1.414"/><path d="M9 14.657V16"/>"#);

/// `square-letter-q`.
pub const SQUARE_LETTER_Q: Icon = lucide!("square-letter-q", r#"<circle cx="12" cy="12" r="4"/><path d="m16 16-2.5-2.5"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-r`.
pub const SQUARE_LETTER_R: Icon = lucide!("square-letter-r", r#"<path d="m11 12 4 4"/><path d="M9 12h4a2 2 0 0 0 0-4H9.5a.5.5 0 0 0-.5.5V16"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-s`.
pub const SQUARE_LETTER_S: Icon = lucide!("square-letter-s", r#"<path d="M14.5 8H11a2 2 0 100 4h2a2 2 0 110 4H9"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-t`.
pub const SQUARE_LETTER_T: Icon = lucide!("square-letter-t", r#"<path d="M12 16V8"/><path d="M9 8h6"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-u`.
pub const SQUARE_LETTER_U: Icon = lucide!("square-letter-u", r#"<path d="M15 8v5a3 3 0 1 1-6 0V8"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-v`.
pub const SQUARE_LETTER_V: Icon = lucide!("square-letter-v", r#"<path d="m9 8 2.525 7.66a.5.5 0 0 0 .95 0L15 8"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-w`.
pub const SQUARE_LETTER_W: Icon = lucide!("square-letter-w", r#"<path d="m16 8-1.591 7.598a.497.497 0 01-.97.038l-.96-3.276a.5.5 0 00-.959 0l-.96 3.276a.496.496 0 01-.969-.038L8 8"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-x`.
pub const SQUARE_LETTER_X: Icon = lucide!("square-letter-x", r#"<path d="m9 16 6-8"/><path d="m9 8 6 8"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-letter-y`.
pub const SQUARE_LETTER_Y: Icon = lucide!("square-letter-y", r#"<path d="m9 8 3 4 3-4"/><path d="M12 12v4"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-letter-z`.
pub const SQUARE_LETTER_Z: Icon = lucide!("square-letter-z", r#"<path d="M9 8h5.501a.5.5 0 0 1 .37.836L9.13 15.164A.5.5 0 0 0 9.5 16H15"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-library`.
pub const SQUARE_LIBRARY: Icon = lucide!("square-library", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 7v10"/><path d="M11 7v10"/><path d="m15 7 2 10"/>"#);

/// `square-menu`.
pub const SQUARE_MENU: Icon = lucide!("square-menu", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 8h10"/><path d="M7 12h10"/><path d="M7 16h10"/>"#);

/// `square-minus`.
pub const SQUARE_MINUS: Icon = lucide!("square-minus", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M8 12h8"/>"#);

/// `square-mouse-pointer`.
pub const SQUARE_MOUSE_POINTER: Icon = lucide!("square-mouse-pointer", r#"<path d="M12.034 12.681a.498.498 0 0 1 .647-.647l9 3.5a.5.5 0 0 1-.033.943l-3.444 1.068a1 1 0 0 0-.66.66l-1.067 3.443a.5.5 0 0 1-.943.033z"/><path d="M21 11V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"/>"#);

/// `square-off`.
pub const SQUARE_OFF: Icon = lucide!("square-off", r#"<path d="M20.4 20.4a2 2 0 01-1.4.6H5a2 2 0 01-2-2V5a2 2 0 01.59-1.41"/><path d="M21 15.3V5a2 2 0 00-2-2H8.7"/><path d="M22 22 2 2"/>"#);

/// `square-padding`.
pub const SQUARE_PADDING: Icon = lucide!("square-padding", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M14 17h-4"/><path d="M14 7h-4"/><path d="M17 14v-4"/><path d="M7 14v-4"/>"#);

/// `square-padding-bottom`.
pub const SQUARE_PADDING_BOTTOM: Icon = lucide!("square-padding-bottom", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M8 17h8"/>"#);

/// `square-padding-horizontal`.
pub const SQUARE_PADDING_HORIZONTAL: Icon = lucide!("square-padding-horizontal", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M17 16V8"/><path d="M7 16V8"/>"#);

/// `square-padding-left`.
pub const SQUARE_PADDING_LEFT: Icon = lucide!("square-padding-left", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M7 16V8"/>"#);

/// `square-padding-right`.
pub const SQUARE_PADDING_RIGHT: Icon = lucide!("square-padding-right", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M17 16V8"/>"#);

/// `square-padding-top`.
pub const SQUARE_PADDING_TOP: Icon = lucide!("square-padding-top", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M8 7h8"/>"#);

/// `square-padding-vertical`.
pub const SQUARE_PADDING_VERTICAL: Icon = lucide!("square-padding-vertical", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M8 17h8"/><path d="M8 7h8"/>"#);

/// `square-pause`.
pub const SQUARE_PAUSE: Icon = lucide!("square-pause", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><line x1="10" x2="10" y1="15" y2="9"/><line x1="14" x2="14" y1="15" y2="9"/>"#);

/// `square-pen`.
pub const SQUARE_PEN: Icon = lucide!("square-pen", r#"<path d="M12 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/><path d="M18.375 2.625a1 1 0 0 1 3 3l-9.013 9.014a2 2 0 0 1-.853.505l-2.873.84a.5.5 0 0 1-.62-.62l.84-2.873a2 2 0 0 1 .506-.852z"/>"#);

/// `square-percent`.
pub const SQUARE_PERCENT: Icon = lucide!("square-percent", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="m15 9-6 6"/><path d="M9 9h.01"/><path d="M15 15h.01"/>"#);

/// `square-pi`.
pub const SQUARE_PI: Icon = lucide!("square-pi", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 7h10"/><path d="M10 7v10"/><path d="M16 17a2 2 0 0 1-2-2V7"/>"#);

/// `square-pilcrow`.
pub const SQUARE_PILCROW: Icon = lucide!("square-pilcrow", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M12 12H9.5a2.5 2.5 0 0 1 0-5H17"/><path d="M12 7v10"/><path d="M16 7v10"/>"#);

/// `square-play`.
pub const SQUARE_PLAY: Icon = lucide!("square-play", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 9.003a1 1 0 0 1 1.517-.859l4.997 2.997a1 1 0 0 1 0 1.718l-4.997 2.997A1 1 0 0 1 9 14.996z"/>"#);

/// `square-plus`.
pub const SQUARE_PLUS: Icon = lucide!("square-plus", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M8 12h8"/><path d="M12 8v8"/>"#);

/// `square-power`.
pub const SQUARE_POWER: Icon = lucide!("square-power", r#"<path d="M12 7v4"/><path d="M7.998 9.003a5 5 0 1 0 8-.005"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-radical`.
pub const SQUARE_RADICAL: Icon = lucide!("square-radical", r#"<path d="M7 12h2l2 5 2-10h4"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-round-corner`.
pub const SQUARE_ROUND_CORNER: Icon = lucide!("square-round-corner", r#"<path d="M21 11a8 8 0 0 0-8-8"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>"#);

/// `square-scissors`.
pub const SQUARE_SCISSORS: Icon = lucide!("square-scissors", r#"<path d="m17 17-2.18-2.18"/><path d="M9.56 14.44 17 7"/><path d="M9.56 9.56 12 12"/><circle cx="8.5" cy="15.5" r="1.5"/><circle cx="8.5" cy="8.5" r="1.5"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-sigma`.
pub const SQUARE_SIGMA: Icon = lucide!("square-sigma", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M16 8.9V7H8l4 5-4 5h8v-1.9"/>"#);

/// `square-slash`.
pub const SQUARE_SLASH: Icon = lucide!("square-slash", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><line x1="9" x2="15" y1="15" y2="9"/>"#);

/// `square-sparkles`.
pub const SQUARE_SPARKLES: Icon = lucide!("square-sparkles", r#"<path d="M11 15H7"/><path d="M15.41 2.49a.6.6 0 011.18 0l.63 3.334a1.2 1.2 0 00.956.955l3.334.631a.6.6 0 010 1.18l-3.334.63a1.2 1.2 0 00-.955.956l-.631 3.334a.6.6 0 01-1.18 0l-.63-3.334a1.2 1.2 0 00-.956-.955L10.49 8.59a.6.6 0 010-1.18l3.334-.63a1.2 1.2 0 00.955-.956z"/><path d="M21 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V5a2 2 0 012-2h6"/><path d="M9 13v4"/>"#);

/// `square-split-horizontal`.
pub const SQUARE_SPLIT_HORIZONTAL: Icon = lucide!("square-split-horizontal", r#"<path d="M12 2v20"/><path d="M16 3h3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-3"/><path d="M8 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3"/>"#);

/// `square-split-vertical`.
pub const SQUARE_SPLIT_VERTICAL: Icon = lucide!("square-split-vertical", r#"<path d="M2 12h20"/><path d="M21 16v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-3"/><path d="M3 8V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v3"/>"#);

/// `square-square`.
pub const SQUARE_SQUARE: Icon = lucide!("square-square", r#"<rect x="3" y="3" width="18" height="18" rx="2"/><rect x="8" y="8" width="8" height="8" rx="1"/>"#);

/// `square-stack`.
pub const SQUARE_STACK: Icon = lucide!("square-stack", r#"<path d="M4 10c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"/><path d="M10 16c-1.1 0-2-.9-2-2v-4c0-1.1.9-2 2-2h4c1.1 0 2 .9 2 2"/><rect width="8" height="8" x="14" y="14" rx="2"/>"#);

/// `square-star`.
pub const SQUARE_STAR: Icon = lucide!("square-star", r#"<path d="M11.035 7.69a1 1 0 0 1 1.909.024l.737 1.452a1 1 0 0 0 .737.535l1.634.256a1 1 0 0 1 .588 1.806l-1.172 1.168a1 1 0 0 0-.282.866l.259 1.613a1 1 0 0 1-1.541 1.134l-1.465-.75a1 1 0 0 0-.912 0l-1.465.75a1 1 0 0 1-1.539-1.133l.258-1.613a1 1 0 0 0-.282-.866l-1.156-1.153a1 1 0 0 1 .572-1.822l1.633-.256a1 1 0 0 0 .737-.535z"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `square-stop`.
pub const SQUARE_STOP: Icon = lucide!("square-stop", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><rect x="9" y="9" width="6" height="6" rx="1"/>"#);

/// `square-terminal`.
pub const SQUARE_TERMINAL: Icon = lucide!("square-terminal", r#"<path d="m7 11 2-2-2-2"/><path d="M11 13h4"/><rect width="18" height="18" x="3" y="3" rx="2" ry="2"/>"#);

/// `square-text`.
pub const SQUARE_TEXT: Icon = lucide!("square-text", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M7 8h8"/><path d="M7 12h10"/><path d="M7 16h6"/>"#);

/// `square-user`.
pub const SQUARE_USER: Icon = lucide!("square-user", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="12" cy="10" r="3"/><path d="M7 21v-2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v2"/>"#);

/// `square-user-round`.
pub const SQUARE_USER_ROUND: Icon = lucide!("square-user-round", r#"<path d="M18 21a6 6 0 0 0-12 0"/><circle cx="12" cy="11" r="4"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `square-x`.
pub const SQUARE_X: Icon = lucide!("square-x", r#"<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/><path d="m15 9-6 6"/><path d="m9 9 6 6"/>"#);

/// `squares-exclude`.
pub const SQUARES_EXCLUDE: Icon = lucide!("squares-exclude", r#"<path d="M16 12v2a2 2 0 0 1-2 2H9a1 1 0 0 0-1 1v3a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V10a2 2 0 0 0-2-2h0"/><path d="M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v3a1 1 0 0 1-1 1h-5a2 2 0 0 0-2 2v2"/>"#);

/// `squares-intersect`.
pub const SQUARES_INTERSECT: Icon = lucide!("squares-intersect", r#"<path d="M10 22a2 2 0 0 1-2-2"/><path d="M14 2a2 2 0 0 1 2 2"/><path d="M16 22h-2"/><path d="M2 10V8"/><path d="M2 4a2 2 0 0 1 2-2"/><path d="M20 8a2 2 0 0 1 2 2"/><path d="M22 14v2"/><path d="M22 20a2 2 0 0 1-2 2"/><path d="M4 16a2 2 0 0 1-2-2"/><path d="M8 10a2 2 0 0 1 2-2h5a1 1 0 0 1 1 1v5a2 2 0 0 1-2 2H9a1 1 0 0 1-1-1z"/><path d="M8 2h2"/>"#);

/// `squares-subtract`.
pub const SQUARES_SUBTRACT: Icon = lucide!("squares-subtract", r#"<path d="M10 22a2 2 0 0 1-2-2"/><path d="M16 22h-2"/><path d="M16 4a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h3a1 1 0 0 0 1-1v-5a2 2 0 0 1 2-2h5a1 1 0 0 0 1-1z"/><path d="M20 8a2 2 0 0 1 2 2"/><path d="M22 14v2"/><path d="M22 20a2 2 0 0 1-2 2"/>"#);

/// `squares-unite`.
pub const SQUARES_UNITE: Icon = lucide!("squares-unite", r#"<path d="M4 16a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v3a1 1 0 0 0 1 1h3a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H10a2 2 0 0 1-2-2v-3a1 1 0 0 0-1-1z"/>"#);

/// `squircle`.
pub const SQUIRCLE: Icon = lucide!("squircle", r#"<path d="M12 3c7.2 0 9 1.8 9 9s-1.8 9-9 9-9-1.8-9-9 1.8-9 9-9"/>"#);

/// `squircle-dashed`.
pub const SQUIRCLE_DASHED: Icon = lucide!("squircle-dashed", r#"<path d="M13.77 3.043a34 34 0 0 0-3.54 0"/><path d="M13.771 20.956a33 33 0 0 1-3.541.001"/><path d="M20.18 17.74c-.51 1.15-1.29 1.93-2.439 2.44"/><path d="M20.18 6.259c-.51-1.148-1.291-1.929-2.44-2.438"/><path d="M20.957 10.23a33 33 0 0 1 0 3.54"/><path d="M3.043 10.23a34 34 0 0 0 .001 3.541"/><path d="M6.26 20.179c-1.15-.508-1.93-1.29-2.44-2.438"/><path d="M6.26 3.82c-1.149.51-1.93 1.291-2.44 2.44"/>"#);

/// `squirrel`.
pub const SQUIRREL: Icon = lucide!("squirrel", r#"<path d="M15.236 22a3 3 0 0 0-2.2-5"/><path d="M16 20a3 3 0 0 1 3-3h1a2 2 0 0 0 2-2v-2a4 4 0 0 0-4-4V4"/><path d="M18 13h.01"/><path d="M18 6a4 4 0 0 0-4 4 7 7 0 0 0-7 7c0-5 4-5 4-10.5a4.5 4.5 0 1 0-9 0 2.5 2.5 0 0 0 5 0C7 10 3 11 3 17c0 2.8 2.2 5 5 5h10"/>"#);

/// `stamp`.
pub const STAMP: Icon = lucide!("stamp", r#"<path d="M14 13V8.5C14 7 15 7 15 5a3 3 0 0 0-6 0c0 2 1 2 1 3.5V13"/><path d="M20 15.5a2.5 2.5 0 0 0-2.5-2.5h-11A2.5 2.5 0 0 0 4 15.5V17a1 1 0 0 0 1 1h14a1 1 0 0 0 1-1z"/><path d="M5 22h14"/>"#);

/// `star`.
pub const STAR: Icon = lucide!("star", r#"<path d="M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z"/>"#);

/// `star-check`.
pub const STAR_CHECK: Icon = lucide!("star-check", r#"<path d="m19.06 12.501 2.78-2.707a.53.53 0 0 0-.294-.905l-5.166-.755a2.1 2.1 0 0 1-1.595-1.16l-2.31-4.68a.53.53 0 0 0-.95.001L9.216 6.974a2.1 2.1 0 0 1-1.597 1.16l-5.165.755a.53.53 0 0 0-.294.906l3.736 3.637a2.1 2.1 0 0 1 .611 1.879l-.88 5.139a.53.53 0 0 0 .769.56l4.617-2.428.027-.014"/><path d="m15 18 2 2 4-4"/>"#);

/// `star-half`.
pub const STAR_HALF: Icon = lucide!("star-half", r#"<path d="M12 18.338a2.1 2.1 0 0 0-.987.244L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.12 2.12 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.12 2.12 0 0 0 1.597-1.16l2.309-4.679A.53.53 0 0 1 12 2"/>"#);

/// `star-minus`.
pub const STAR_MINUS: Icon = lucide!("star-minus", r#"<path d="M15 18h6"/><path d="M17.688 14a2.1 2.1 0 0 1 .416-.568l3.736-3.638a.53.53 0 0 0-.294-.905l-5.166-.755a2.1 2.1 0 0 1-1.595-1.16l-2.31-4.68a.53.53 0 0 0-.95.001L9.216 6.974a2.1 2.1 0 0 1-1.597 1.16l-5.165.755a.53.53 0 0 0-.294.906l3.736 3.637a2.1 2.1 0 0 1 .611 1.879l-.88 5.139a.53.53 0 0 0 .769.56l4.617-2.428.027-.014"/>"#);

/// `star-off`.
pub const STAR_OFF: Icon = lucide!("star-off", r#"<path d="m10.344 4.688 1.181-2.393a.53.53 0 0 1 .95 0l2.31 4.679a2.12 2.12 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.237 3.152"/><path d="m17.945 17.945.43 2.505a.53.53 0 0 1-.771.56l-4.618-2.428a2.12 2.12 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.12 2.12 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a8 8 0 0 0 .4-.099"/><path d="m2 2 20 20"/>"#);

/// `star-plus`.
pub const STAR_PLUS: Icon = lucide!("star-plus", r#"<path d="M11.013 18.582 6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.12 2.12 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.12 2.12 0 0 0 1.597-1.16l2.309-4.679a.53.53 0 0 1 .95 0l2.31 4.679a2.12 2.12 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904L20 11.5"/><path d="M15 18h6"/><path d="M18 15v6"/>"#);

/// `star-x`.
pub const STAR_X: Icon = lucide!("star-x", r#"<path d="m15.5 15.5 5 5"/><path d="m20.063 11.525 1.777-1.731a.53.53 0 0 0-.294-.905l-5.166-.755a2.1 2.1 0 0 1-1.595-1.16l-2.31-4.68a.53.53 0 0 0-.95.001L9.216 6.974a2.1 2.1 0 0 1-1.597 1.16l-5.165.755a.53.53 0 0 0-.294.906l3.736 3.637a2.1 2.1 0 0 1 .611 1.879l-.88 5.139a.53.53 0 0 0 .769.56l4.617-2.428a2.1 2.1 0 0 1 .987-.243 2 2 0 0 1 .132.004"/><path d="m20.5 15.5-5 5"/>"#);

/// `step-back`.
pub const STEP_BACK: Icon = lucide!("step-back", r#"<path d="M13.971 4.285A2 2 0 0 1 17 6v12a2 2 0 0 1-3.029 1.715l-9.997-5.998a2 2 0 0 1-.003-3.432z"/><path d="M21 20V4"/>"#);

/// `step-forward`.
pub const STEP_FORWARD: Icon = lucide!("step-forward", r#"<path d="M10.029 4.285A2 2 0 0 0 7 6v12a2 2 0 0 0 3.029 1.715l9.997-5.998a2 2 0 0 0 .003-3.432z"/><path d="M3 4v16"/>"#);

/// `stethoscope`.
pub const STETHOSCOPE: Icon = lucide!("stethoscope", r#"<path d="M11 2v2"/><path d="M5 2v2"/><path d="M5 3H4a2 2 0 0 0-2 2v4a6 6 0 0 0 12 0V5a2 2 0 0 0-2-2h-1"/><path d="M8 15a6 6 0 0 0 12 0v-3"/><circle cx="20" cy="10" r="2"/>"#);

/// `sticker`.
pub const STICKER: Icon = lucide!("sticker", r#"<path d="M21 9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z"/><path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="M8 13h.01"/><path d="M16 13h.01"/><path d="M10 16s.8 1 2 1c1.3 0 2-1 2-1"/>"#);

/// `sticky-note`.
pub const STICKY_NOTE: Icon = lucide!("sticky-note", r#"<path d="M21 9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2z"/><path d="M15 3v5a1 1 0 0 0 1 1h5"/>"#);

/// `sticky-note-check`.
pub const STICKY_NOTE_CHECK: Icon = lucide!("sticky-note-check", r#"<path d="m15 19 2 2 4-4"/><path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="M21 13V9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6.5"/>"#);

/// `sticky-note-minus`.
pub const STICKY_NOTE_MINUS: Icon = lucide!("sticky-note-minus", r#"<path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="M21 14V9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.35"/><path d="M21 18h-6"/>"#);

/// `sticky-note-off`.
pub const STICKY_NOTE_OFF: Icon = lucide!("sticky-note-off", r#"<path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="m2 2 20 20"/><path d="M3.586 3.586A2 2 0 0 0 3 5v14a2 2 0 0 0 2 2h14a2 2 0 0 0 1.414-.586"/><path d="M8.656 3H15a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 21 9v6.344"/>"#);

/// `sticky-note-plus`.
pub const STICKY_NOTE_PLUS: Icon = lucide!("sticky-note-plus", r#"<path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="M18 15v6"/><path d="M21 12.356V9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7.355"/><path d="M21 18h-6"/>"#);

/// `sticky-note-x`.
pub const STICKY_NOTE_X: Icon = lucide!("sticky-note-x", r#"<path d="M15 3v5a1 1 0 0 0 1 1h5"/><path d="m16 16 5 5"/><path d="M21 12V9a2.4 2.4 0 0 0-.706-1.706l-3.588-3.588A2.4 2.4 0 0 0 15 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7"/><path d="m21 16-5 5"/>"#);

/// `sticky-notes`.
pub const STICKY_NOTES: Icon = lucide!("sticky-notes", r#"<path d="M10 8a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 16 14v6a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V10a2 2 0 0 1 2-2z"/><path d="M10 8v5a1 1 0 0 0 1 1h5"/><path d="M8 4a2 2 0 0 1 2-2h6a2.4 2.4 0 0 1 1.706.706l3.588 3.588A2.4 2.4 0 0 1 22 8v6a2 2 0 0 1-2 2"/><path d="M16 2v5a1 1 0 0 0 1 1h5"/>"#);

/// `stone`.
pub const STONE: Icon = lucide!("stone", r#"<path d="M11.264 2.205A4 4 0 0 0 6.42 4.211l-4 8a4 4 0 0 0 1.359 5.117l6 4a4 4 0 0 0 4.438 0l6-4a4 4 0 0 0 1.576-4.592l-2-6a4 4 0 0 0-2.53-2.53z"/><path d="M11.99 22 14 12l7.822 3.184"/><path d="M14 12 8.47 2.302"/>"#);

/// `store`.
pub const STORE: Icon = lucide!("store", r#"<path d="M15 21v-5a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v5"/><path d="M17.774 10.31a1.12 1.12 0 0 0-1.549 0 2.5 2.5 0 0 1-3.451 0 1.12 1.12 0 0 0-1.548 0 2.5 2.5 0 0 1-3.452 0 1.12 1.12 0 0 0-1.549 0 2.5 2.5 0 0 1-3.77-3.248l2.889-4.184A2 2 0 0 1 7 2h10a2 2 0 0 1 1.653.873l2.895 4.192a2.5 2.5 0 0 1-3.774 3.244"/><path d="M4 10.95V19a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8.05"/>"#);

/// `stretch-horizontal`.
pub const STRETCH_HORIZONTAL: Icon = lucide!("stretch-horizontal", r#"<rect width="20" height="6" x="2" y="4" rx="2"/><rect width="20" height="6" x="2" y="14" rx="2"/>"#);

/// `stretch-vertical`.
pub const STRETCH_VERTICAL: Icon = lucide!("stretch-vertical", r#"<rect width="6" height="20" x="4" y="2" rx="2"/><rect width="6" height="20" x="14" y="2" rx="2"/>"#);

/// `strikethrough`.
pub const STRIKETHROUGH: Icon = lucide!("strikethrough", r#"<path d="M16 4H9a3 3 0 0 0-2.83 4"/><path d="M14 12a4 4 0 0 1 0 8H6"/><line x1="4" x2="20" y1="12" y2="12"/>"#);

/// `subscript`.
pub const SUBSCRIPT: Icon = lucide!("subscript", r#"<path d="m4 5 8 8"/><path d="m12 5-8 8"/><path d="M20 19h-4c0-1.5.44-2 1.5-2.5S20 15.33 20 14c0-.47-.17-.93-.48-1.29a2.11 2.11 0 0 0-2.62-.44c-.42.24-.74.62-.9 1.07"/>"#);

/// `summary`.
pub const SUMMARY: Icon = lucide!("summary", r#"<path d="M15 4H7"/><path d="m18 16 3 3-3 3"/><path d="M3 4v13a2 2 0 0 0 2 2h16"/><path d="M7 14h7"/><path d="M7 9h12"/>"#);

/// `sun`.
pub const SUN: Icon = lucide!("sun", r#"<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>"#);

/// `sun-dim`.
pub const SUN_DIM: Icon = lucide!("sun-dim", r#"<circle cx="12" cy="12" r="4"/><path d="M12 4h.01"/><path d="M20 12h.01"/><path d="M12 20h.01"/><path d="M4 12h.01"/><path d="M17.657 6.343h.01"/><path d="M17.657 17.657h.01"/><path d="M6.343 17.657h.01"/><path d="M6.343 6.343h.01"/>"#);

/// `sun-medium`.
pub const SUN_MEDIUM: Icon = lucide!("sun-medium", r#"<circle cx="12" cy="12" r="4"/><path d="M12 3v1"/><path d="M12 20v1"/><path d="M3 12h1"/><path d="M20 12h1"/><path d="m18.364 5.636-.707.707"/><path d="m6.343 17.657-.707.707"/><path d="m5.636 5.636.707.707"/><path d="m17.657 17.657.707.707"/>"#);

/// `sun-moon`.
pub const SUN_MOON: Icon = lucide!("sun-moon", r#"<path d="M12 2v2"/><path d="M14.837 16.385a6 6 0 1 1-7.223-7.222c.624-.147.97.66.715 1.248a4 4 0 0 0 5.26 5.259c.589-.255 1.396.09 1.248.715"/><path d="M16 12a4 4 0 0 0-4-4"/><path d="m19 5-1.256 1.256"/><path d="M20 12h2"/>"#);

/// `sun-snow`.
pub const SUN_SNOW: Icon = lucide!("sun-snow", r#"<path d="M10 21v-1"/><path d="M10 4V3"/><path d="M10 9a3 3 0 0 0 0 6"/><path d="m14 20 1.25-2.5L18 18"/><path d="m14 4 1.25 2.5L18 6"/><path d="m17 21-3-6 1.5-3H22"/><path d="m17 3-3 6 1.5 3"/><path d="M2 12h1"/><path d="m20 10-1.5 2 1.5 2"/><path d="m3.64 18.36.7-.7"/><path d="m4.34 6.34-.7-.7"/>"#);

/// `sunrise`.
pub const SUNRISE: Icon = lucide!("sunrise", r#"<path d="M12 2v8"/><path d="m4.93 10.93 1.41 1.41"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="m19.07 10.93-1.41 1.41"/><path d="M22 22H2"/><path d="m8 6 4-4 4 4"/><path d="M16 18a4 4 0 0 0-8 0"/>"#);

/// `sunset`.
pub const SUNSET: Icon = lucide!("sunset", r#"<path d="M12 10V2"/><path d="m4.93 10.93 1.41 1.41"/><path d="M2 18h2"/><path d="M20 18h2"/><path d="m19.07 10.93-1.41 1.41"/><path d="M22 22H2"/><path d="m16 6-4 4-4-4"/><path d="M16 18a4 4 0 0 0-8 0"/>"#);

/// `superscript`.
pub const SUPERSCRIPT: Icon = lucide!("superscript", r#"<path d="m4 19 8-8"/><path d="m12 19-8-8"/><path d="M20 12h-4c0-1.5.442-2 1.5-2.5S20 8.334 20 7.002c0-.472-.17-.93-.484-1.29a2.105 2.105 0 0 0-2.617-.436c-.42.239-.738.614-.899 1.06"/>"#);

/// `swatch-book`.
pub const SWATCH_BOOK: Icon = lucide!("swatch-book", r#"<path d="M11 17a4 4 0 0 1-8 0V5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2Z"/><path d="M16.7 13H19a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2H7"/><path d="M 7 17h.01"/><path d="m11 8 2.3-2.3a2.4 2.4 0 0 1 3.404.004L18.6 7.6a2.4 2.4 0 0 1 .026 3.434L9.9 19.8"/>"#);

/// `swiss-franc`.
pub const SWISS_FRANC: Icon = lucide!("swiss-franc", r#"<path d="M10 10h7"/><path d="M10 20V5a1 1 0 011-1h7"/><path d="M6 16h9"/>"#);

/// `switch-camera`.
pub const SWITCH_CAMERA: Icon = lucide!("switch-camera", r#"<path d="M11 19H4a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h5"/><path d="M13 5h7a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-5"/><circle cx="12" cy="12" r="3"/><path d="m18 22-3-3 3-3"/><path d="m6 2 3 3-3 3"/>"#);

/// `sword`.
pub const SWORD: Icon = lucide!("sword", r#"<path d="m11 19-6-6"/><path d="m5 21-2-2"/><path d="m8 16-4 4"/><path d="M9.5 17.5 20.414 6.586A2 2 0 0021 5.172V3h-2.172a2 2 0 00-1.414.586L6.5 14.5"/>"#);

/// `swords`.
pub const SWORDS: Icon = lucide!("swords", r#"<path d="m13 19 6-6"/><path d="M14.5 17.5 3.586 6.586A2 2 0 013 5.172V3h2.172a2 2 0 011.414.586L17.5 14.5"/><path d="m14.828 6.172 2.586-2.586A2 2 0 0118.828 3H21v2.172a2 2 0 01-.586 1.414l-2.586 2.586"/><path d="m16 16 4 4"/><path d="m19 21 2-2"/><path d="m5 14 4 4"/><path d="m5 21-2-2"/><path d="M7.5 16.5 4 20"/>"#);

/// `syringe`.
pub const SYRINGE: Icon = lucide!("syringe", r#"<path d="m18 2 4 4"/><path d="m17 7 3-3"/><path d="M19 9 8.7 19.3c-1 1-2.5 1-3.4 0l-.6-.6c-1-1-1-2.5 0-3.4L15 5"/><path d="m9 11 4 4"/><path d="m5 19-3 3"/><path d="m14 4 6 6"/>"#);

/// `table`.
pub const TABLE: Icon = lucide!("table", r#"<path d="M12 3v18"/><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/>"#);

/// `table-2`.
pub const TABLE_2: Icon = lucide!("table-2", r#"<path d="M3 9h18"/><path d="M9 3v18"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `table-cells-merge`.
pub const TABLE_CELLS_MERGE: Icon = lucide!("table-cells-merge", r#"<path d="M12 21v-6"/><path d="M12 9V3"/><path d="M3 15h18"/><path d="M3 9h18"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `table-cells-split`.
pub const TABLE_CELLS_SPLIT: Icon = lucide!("table-cells-split", r#"<path d="M12 15V9"/><path d="M3 15h18"/><path d="M3 9h18"/><rect width="18" height="18" x="3" y="3" rx="2"/>"#);

/// `table-columns-split`.
pub const TABLE_COLUMNS_SPLIT: Icon = lucide!("table-columns-split", r#"<path d="M14 14v2"/><path d="M14 20v2"/><path d="M14 2v2"/><path d="M14 8v2"/><path d="M2 15h8"/><path d="M2 3h6a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H2"/><path d="M2 9h8"/><path d="M22 15h-4"/><path d="M22 3h-2a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h2"/><path d="M22 9h-4"/><path d="M5 3v18"/>"#);

/// `table-of-contents`.
pub const TABLE_OF_CONTENTS: Icon = lucide!("table-of-contents", r#"<path d="M16 5H3"/><path d="M16 12H3"/><path d="M16 19H3"/><path d="M21 5h.01"/><path d="M21 12h.01"/><path d="M21 19h.01"/>"#);

/// `table-properties`.
pub const TABLE_PROPERTIES: Icon = lucide!("table-properties", r#"<path d="M15 3v18"/><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M21 9H3"/><path d="M21 15H3"/>"#);

/// `table-rows-split`.
pub const TABLE_ROWS_SPLIT: Icon = lucide!("table-rows-split", r#"<path d="M14 10h2"/><path d="M15 22v-8"/><path d="M15 2v4"/><path d="M2 10h2"/><path d="M20 10h2"/><path d="M3 19h18"/><path d="M3 22v-6a2 2 135 0 1 2-2h14a2 2 45 0 1 2 2v6"/><path d="M3 2v2a2 2 45 0 0 2 2h14a2 2 135 0 0 2-2V2"/><path d="M8 10h2"/><path d="M9 22v-8"/><path d="M9 2v4"/>"#);

/// `tablet`.
pub const TABLET: Icon = lucide!("tablet", r#"<rect width="16" height="20" x="4" y="2" rx="2" ry="2"/><line x1="12" x2="12.01" y1="18" y2="18"/>"#);

/// `tablet-smartphone`.
pub const TABLET_SMARTPHONE: Icon = lucide!("tablet-smartphone", r#"<rect width="10" height="14" x="3" y="8" rx="2"/><path d="M5 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v16a2 2 0 0 1-2 2h-2.4"/><path d="M8 18h.01"/>"#);

/// `tablets`.
pub const TABLETS: Icon = lucide!("tablets", r#"<circle cx="7" cy="7" r="5"/><circle cx="17" cy="17" r="5"/><path d="M12 17h10"/><path d="m3.46 10.54 7.08-7.08"/>"#);

/// `tag`.
pub const TAG: Icon = lucide!("tag", r#"<path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l6.58-6.58a2.426 2.426 0 0 0 0-3.42z"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/>"#);

/// `tag-plus`.
pub const TAG_PLUS: Icon = lucide!("tag-plus", r#"<path d="M16 13h6"/><path d="m16.5 6.5-3.914-3.914A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.426 2.426 0 0 0 3.42 0l1.79-1.79"/><path d="M19 10v6"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/>"#);

/// `tag-x`.
pub const TAG_X: Icon = lucide!("tag-x", r#"<path d="m16.5 6.5-3.914-3.914A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8.704 8.704a2.43 2.43 0 0 0 3.42 0l1.79-1.79"/><path d="m16.5 10.5 5 5"/><path d="m21.5 10.5-5 5"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/>"#);

/// `tags`.
pub const TAGS: Icon = lucide!("tags", r#"<path d="M13.172 2a2 2 0 0 1 1.414.586l6.71 6.71a2.4 2.4 0 0 1 0 3.408l-4.592 4.592a2.4 2.4 0 0 1-3.408 0l-6.71-6.71A2 2 0 0 1 6 9.172V3a1 1 0 0 1 1-1z"/><path d="M2 7v6.172a2 2 0 0 0 .586 1.414l6.71 6.71a2.4 2.4 0 0 0 3.191.193"/><circle cx="10.5" cy="6.5" r=".5" fill="currentColor"/>"#);

/// `tally-1`.
pub const TALLY_1: Icon = lucide!("tally-1", r#"<path d="M4 4v16"/>"#);

/// `tally-2`.
pub const TALLY_2: Icon = lucide!("tally-2", r#"<path d="M4 4v16"/><path d="M9 4v16"/>"#);

/// `tally-3`.
pub const TALLY_3: Icon = lucide!("tally-3", r#"<path d="M4 4v16"/><path d="M9 4v16"/><path d="M14 4v16"/>"#);

/// `tally-4`.
pub const TALLY_4: Icon = lucide!("tally-4", r#"<path d="M4 4v16"/><path d="M9 4v16"/><path d="M14 4v16"/><path d="M19 4v16"/>"#);

/// `tally-5`.
pub const TALLY_5: Icon = lucide!("tally-5", r#"<path d="M4 4v16"/><path d="M9 4v16"/><path d="M14 4v16"/><path d="M19 4v16"/><path d="M22 6 2 18"/>"#);

/// `tangent`.
pub const TANGENT: Icon = lucide!("tangent", r#"<circle cx="17" cy="4" r="2"/><path d="M15.59 5.41 5.41 15.59"/><circle cx="4" cy="17" r="2"/><path d="M12 22s-4-9-1.5-11.5S22 12 22 12"/>"#);

/// `target`.
pub const TARGET: Icon = lucide!("target", r#"<circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/>"#);

/// `telescope`.
pub const TELESCOPE: Icon = lucide!("telescope", r#"<path d="m10.065 12.493-6.18 1.318a.934.934 0 0 1-1.108-.702l-.537-2.15a1.07 1.07 0 0 1 .691-1.265l13.504-4.44"/><path d="m13.56 11.747 4.332-.924"/><path d="m16 21-3.105-6.21"/><path d="M16.485 5.94a2 2 0 0 1 1.455-2.425l1.09-.272a1 1 0 0 1 1.212.727l1.515 6.06a1 1 0 0 1-.727 1.213l-1.09.272a2 2 0 0 1-2.425-1.455z"/><path d="m6.158 8.633 1.114 4.456"/><path d="m8 21 3.105-6.21"/><circle cx="12" cy="13" r="2"/>"#);

/// `tent`.
pub const TENT: Icon = lucide!("tent", r#"<path d="M3.5 21 14 3"/><path d="M20.5 21 10 3"/><path d="M15.5 21 12 15l-3.5 6"/><path d="M2 21h20"/>"#);

/// `tent-tree`.
pub const TENT_TREE: Icon = lucide!("tent-tree", r#"<circle cx="4" cy="4" r="2"/><path d="m14 5 3-3 3 3"/><path d="m14 10 3-3 3 3"/><path d="M17 14V2"/><path d="M17 14H7l-5 8h20Z"/><path d="M8 14v8"/><path d="m9 14 5 8"/>"#);

/// `terminal`.
pub const TERMINAL: Icon = lucide!("terminal", r#"<path d="M12 19h8"/><path d="m4 17 6-6-6-6"/>"#);

/// `test-tube`.
pub const TEST_TUBE: Icon = lucide!("test-tube", r#"<path d="M14.5 2v17.5c0 1.4-1.1 2.5-2.5 2.5c-1.4 0-2.5-1.1-2.5-2.5V2"/><path d="M8.5 2h7"/><path d="M14.5 16h-5"/>"#);

/// `test-tube-diagonal`.
pub const TEST_TUBE_DIAGONAL: Icon = lucide!("test-tube-diagonal", r#"<path d="M21 7 6.82 21.18a2.83 2.83 0 0 1-3.99-.01a2.83 2.83 0 0 1 0-4L17 3"/><path d="m16 2 6 6"/><path d="M12 16H4"/>"#);

/// `test-tubes`.
pub const TEST_TUBES: Icon = lucide!("test-tubes", r#"<path d="M9 2v17.5A2.5 2.5 0 0 1 6.5 22A2.5 2.5 0 0 1 4 19.5V2"/><path d="M20 2v17.5a2.5 2.5 0 0 1-2.5 2.5a2.5 2.5 0 0 1-2.5-2.5V2"/><path d="M3 2h7"/><path d="M14 2h7"/><path d="M9 16H4"/><path d="M20 16h-5"/>"#);

/// `text-align-center`.
pub const TEXT_ALIGN_CENTER: Icon = lucide!("text-align-center", r#"<path d="M21 5H3"/><path d="M17 12H7"/><path d="M19 19H5"/>"#);

/// `text-align-end`.
pub const TEXT_ALIGN_END: Icon = lucide!("text-align-end", r#"<path d="M21 5H3"/><path d="M21 12H9"/><path d="M21 19H7"/>"#);

/// `text-align-justify`.
pub const TEXT_ALIGN_JUSTIFY: Icon = lucide!("text-align-justify", r#"<path d="M3 5h18"/><path d="M3 12h18"/><path d="M3 19h18"/>"#);

/// `text-align-justify-center`.
pub const TEXT_ALIGN_JUSTIFY_CENTER: Icon = lucide!("text-align-justify-center", r#"<path d="M3 5h18"/><path d="M3 12h18"/><path d="M7 19h10"/>"#);

/// `text-align-justify-end`.
pub const TEXT_ALIGN_JUSTIFY_END: Icon = lucide!("text-align-justify-end", r#"<path d="M3 5h18"/><path d="M3 12h18"/><path d="M11 19h10"/>"#);

/// `text-align-justify-start`.
pub const TEXT_ALIGN_JUSTIFY_START: Icon = lucide!("text-align-justify-start", r#"<path d="M3 5h18"/><path d="M3 12h18"/><path d="M3 19h10"/>"#);

/// `text-align-start`.
pub const TEXT_ALIGN_START: Icon = lucide!("text-align-start", r#"<path d="M21 5H3"/><path d="M15 12H3"/><path d="M17 19H3"/>"#);

/// `text-cursor`.
pub const TEXT_CURSOR: Icon = lucide!("text-cursor", r#"<path d="M17 22h-1a4 4 0 0 1-4-4V6a4 4 0 0 1 4-4h1"/><path d="M7 22h1a4 4 0 0 0 4-4"/><path d="M7 2h1a4 4 0 0 1 4 4"/>"#);

/// `text-cursor-input`.
pub const TEXT_CURSOR_INPUT: Icon = lucide!("text-cursor-input", r#"<path d="M12 20h-1a2 2 0 0 1-2-2 2 2 0 0 1-2 2H6"/><path d="M13 8h7a2 2 0 0 1 2 2v4a2 2 0 0 1-2 2h-7"/><path d="M5 16H4a2 2 0 0 1-2-2v-4a2 2 0 0 1 2-2h1"/><path d="M6 4h1a2 2 0 0 1 2 2 2 2 0 0 1 2-2h1"/><path d="M9 6v12"/>"#);

/// `text-initial`.
pub const TEXT_INITIAL: Icon = lucide!("text-initial", r#"<path d="M15 5h6"/><path d="M15 12h6"/><path d="M3 19h18"/><path d="m3 12 3.553-7.724a.5.5 0 0 1 .894 0L11 12"/><path d="M3.92 10h6.16"/>"#);

/// `text-quote`.
pub const TEXT_QUOTE: Icon = lucide!("text-quote", r#"<path d="M17 5H3"/><path d="M21 12H8"/><path d="M21 19H8"/><path d="M3 12v7"/>"#);

/// `text-search`.
pub const TEXT_SEARCH: Icon = lucide!("text-search", r#"<path d="M21 5H3"/><path d="M10 12H3"/><path d="M10 19H3"/><circle cx="17" cy="15" r="3"/><path d="m21 19-1.9-1.9"/>"#);

/// `text-wrap`.
pub const TEXT_WRAP: Icon = lucide!("text-wrap", r#"<path d="m16 16-3 3 3 3"/><path d="M3 12h14.5a1 1 0 0 1 0 7H13"/><path d="M3 19h6"/><path d="M3 5h18"/>"#);

/// `theater`.
pub const THEATER: Icon = lucide!("theater", r#"<path d="M2 10s3-3 3-8"/><path d="M22 10s-3-3-3-8"/><path d="M10 2c0 4.4-3.6 8-8 8"/><path d="M14 2c0 4.4 3.6 8 8 8"/><path d="M2 10s2 2 2 5"/><path d="M22 10s-2 2-2 5"/><path d="M8 15h8"/><path d="M2 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1"/><path d="M14 22v-1a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v1"/>"#);

/// `thermometer`.
pub const THERMOMETER: Icon = lucide!("thermometer", r#"<path d="M14 4v10.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0Z"/>"#);

/// `thermometer-snowflake`.
pub const THERMOMETER_SNOWFLAKE: Icon = lucide!("thermometer-snowflake", r#"<path d="m10 20-1.25-2.5L6 18"/><path d="M10 4 8.75 6.5 6 6"/><path d="M10.585 15H10"/><path d="M2 12h6.5L10 9"/><path d="M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z"/><path d="m4 10 1.5 2L4 14"/><path d="m7 21 3-6-1.5-3"/><path d="m7 3 3 6h2"/>"#);

/// `thermometer-sun`.
pub const THERMOMETER_SUN: Icon = lucide!("thermometer-sun", r#"<path d="M12 2v2"/><path d="M12 8a4 4 0 0 0-1.645 7.647"/><path d="M2 12h2"/><path d="M20 14.54a4 4 0 1 1-4 0V4a2 2 0 0 1 4 0z"/><path d="m4.93 4.93 1.41 1.41"/><path d="m6.34 17.66-1.41 1.41"/>"#);

/// `thumbs-down`.
pub const THUMBS_DOWN: Icon = lucide!("thumbs-down", r#"<path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z"/><path d="M17 14V2"/>"#);

/// `thumbs-up`.
pub const THUMBS_UP: Icon = lucide!("thumbs-up", r#"<path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z"/><path d="M7 10v12"/>"#);

/// `tic-tac-toe`.
pub const TIC_TAC_TOE: Icon = lucide!("tic-tac-toe", r#"<path d="M12 2v20"/><path d="m21 16-5 5"/><path d="m21 21-5-5"/><path d="M22 12H2"/><path d="M8 3 3 8"/><path d="M8 8 3 3"/><circle cx="18.5" cy="5.5" r="2.5"/><circle cx="5.5" cy="18.5" r="2.5"/>"#);

/// `ticket`.
pub const TICKET: Icon = lucide!("ticket", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="M13 5v2"/><path d="M13 17v2"/><path d="M13 11v2"/>"#);

/// `ticket-check`.
pub const TICKET_CHECK: Icon = lucide!("ticket-check", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="m9 12 2 2 4-4"/>"#);

/// `ticket-minus`.
pub const TICKET_MINUS: Icon = lucide!("ticket-minus", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="M9 12h6"/>"#);

/// `ticket-percent`.
pub const TICKET_PERCENT: Icon = lucide!("ticket-percent", r#"<path d="M2 9a3 3 0 1 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 1 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="M9 9h.01"/><path d="m15 9-6 6"/><path d="M15 15h.01"/>"#);

/// `ticket-plus`.
pub const TICKET_PLUS: Icon = lucide!("ticket-plus", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="M9 12h6"/><path d="M12 9v6"/>"#);

/// `ticket-slash`.
pub const TICKET_SLASH: Icon = lucide!("ticket-slash", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="m9.5 14.5 5-5"/>"#);

/// `ticket-x`.
pub const TICKET_X: Icon = lucide!("ticket-x", r#"<path d="M2 9a3 3 0 0 1 0 6v2a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-2a3 3 0 0 1 0-6V7a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2Z"/><path d="m9.5 14.5 5-5"/><path d="m9.5 9.5 5 5"/>"#);

/// `tickets`.
pub const TICKETS: Icon = lucide!("tickets", r#"<path d="m3.173 8.18 11-5a2 2 0 0 1 2.647.993L18.56 8"/><path d="M6 10V8"/><path d="M6 14v1"/><path d="M6 19v2"/><rect x="2" y="8" width="20" height="13" rx="2"/>"#);

/// `tickets-plane`.
pub const TICKETS_PLANE: Icon = lucide!("tickets-plane", r#"<path d="M10.5 17h1.227a2 2 0 0 0 1.345-.52L18 12"/><path d="m12 13.5 3.794.506"/><path d="m3.173 8.18 11-5a2 2 0 0 1 2.647.993L18.56 8"/><path d="M6 10V8"/><path d="M6 14v1"/><path d="M6 19v2"/><rect x="2" y="8" width="20" height="13" rx="2"/>"#);

/// `timeline`.
pub const TIMELINE: Icon = lucide!("timeline", r#"<path d="M4 12h.01"/><path d="M4 16h.01"/><path d="M4 20h.01"/><path d="M4 4h.01"/><path d="M4 8h.01"/><path d="M9.414 13.414a2 2 0 0 0 1.414.586H19a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1h-8.172a2 2 0 0 0-1.414.586L8 12z"/><path d="M9.414 21.414a2 2 0 0 0 1.414.586H19a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1h-8.172a2 2 0 0 0-1.414.586L8 20z"/><path d="M9.414 5.414A2 2 0 0 0 10.828 6H19a1 1 0 0 0 1-1V3a1 1 0 0 0-1-1h-8.172a2 2 0 0 0-1.414.586L8 4z"/>"#);

/// `timer`.
pub const TIMER: Icon = lucide!("timer", r#"<line x1="10" x2="14" y1="2" y2="2"/><line x1="12" x2="15" y1="14" y2="11"/><circle cx="12" cy="14" r="8"/>"#);

/// `timer-off`.
pub const TIMER_OFF: Icon = lucide!("timer-off", r#"<path d="M10 2h4"/><path d="M4.6 11a8 8 0 0 0 1.7 8.7 8 8 0 0 0 8.7 1.7"/><path d="M7.4 7.4a8 8 0 0 1 10.3 1 8 8 0 0 1 .9 10.2"/><path d="m2 2 20 20"/><path d="M12 12v-2"/>"#);

/// `timer-reset`.
pub const TIMER_RESET: Icon = lucide!("timer-reset", r#"<path d="M10 2h4"/><path d="M12 14v-4"/><path d="M4 13a8 8 0 0 1 8-7 8 8 0 1 1-5.3 14L4 17.6"/><path d="M9 17H4v5"/>"#);

/// `toggle-left`.
pub const TOGGLE_LEFT: Icon = lucide!("toggle-left", r#"<circle cx="9" cy="12" r="3"/><rect width="20" height="14" x="2" y="5" rx="7"/>"#);

/// `toggle-right`.
pub const TOGGLE_RIGHT: Icon = lucide!("toggle-right", r#"<circle cx="15" cy="12" r="3"/><rect width="20" height="14" x="2" y="5" rx="7"/>"#);

/// `toilet`.
pub const TOILET: Icon = lucide!("toilet", r#"<path d="M7 12h13a1 1 0 0 1 1 1 5 5 0 0 1-5 5h-.598a.5.5 0 0 0-.424.765l1.544 2.47a.5.5 0 0 1-.424.765H5.402a.5.5 0 0 1-.424-.765L7 18"/><path d="M8 18a5 5 0 0 1-5-5V4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8"/>"#);

/// `tombstone`.
pub const TOMBSTONE: Icon = lucide!("tombstone", r#"<path d="M11 10h2"/><path d="M19 21V10a7 7 0 00-14 0v11"/><path d="M2 21h20"/><path d="M9 14h6"/>"#);

/// `tool-case`.
pub const TOOL_CASE: Icon = lucide!("tool-case", r#"<path d="M10 15h4"/><path d="m14.5 4.058.571-1.429a1 1 0 011.3-.558l5 2a1 1 0 01.549.534.88.88 0 01.01.765l-2.332 5.83"/><path d="M15.634 11a2 2 0 00-.186-.34l-1.17-1.757 1.31-1.656a2 2 0 00-2.109-3.165l-2.032.57-1.17-1.758a2 2 0 00-3.664 1.028l-.086 2.11-2.032.568a2 2 0 00-.155 3.8l1.617.6"/><path d="M4 12.006A1 1 0 014.994 11H19a1 1 0 011 1v7a2 2 0 01-2 2H6a2 2 0 01-2-2z"/>"#);

/// `toolbox`.
pub const TOOLBOX: Icon = lucide!("toolbox", r#"<path d="M16 12v4"/><path d="M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2"/><path d="M17 6a2 2 0 011.414.586l3 3A2 2 0 0122 11v8a2 2 0 01-2 2H4a2 2 0 01-2-2v-8a2 2 0 01.586-1.414l3-3A2 2 0 017 6z"/><path d="M2 14h20"/><path d="M8 12v4"/>"#);

/// `toothbrush`.
pub const TOOTHBRUSH: Icon = lucide!("toothbrush", r#"<path d="M15 11c-2 2-4 2-6 4l-7 7"/><path d="m22 4-7.414 7.414-2-2A2 2 0 0114 6c0-.512.196-1.024.586-1.414A2 2 0 0116 4a2 2 0 013.262-1.552l2.152 2.138"/>"#);

/// `toothbrush-sparkles`.
pub const TOOTHBRUSH_SPARKLES: Icon = lucide!("toothbrush-sparkles", r#"<path d="M10 3H8"/><path d="M14.586 11.414 22 4"/><path d="M15 11a8 8 0 01-.429.4"/><path d="m2 22 7-7c1.857-1.857 3.714-1.99 5.571-3.6l-1.985-1.986A2 2 0 0114 6a2 2 0 012-2 2 2 0 013.262-1.552l2.152 2.138"/><path d="M20 15v4"/><path d="M22 17h-4"/><path d="M4 5v4"/><path d="M6 7H2"/><path d="M9 2v2"/>"#);

/// `tornado`.
pub const TORNADO: Icon = lucide!("tornado", r#"<path d="M21 4H3"/><path d="M18 8H6"/><path d="M19 12H9"/><path d="M16 16h-6"/><path d="M11 20H9"/>"#);

/// `torus`.
pub const TORUS: Icon = lucide!("torus", r#"<ellipse cx="12" cy="11" rx="3" ry="2"/><ellipse cx="12" cy="12.5" rx="10" ry="8.5"/>"#);

/// `touchpad`.
pub const TOUCHPAD: Icon = lucide!("touchpad", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M2 14h20"/><path d="M12 20v-6"/>"#);

/// `touchpad-off`.
pub const TOUCHPAD_OFF: Icon = lucide!("touchpad-off", r#"<path d="M12 20v-6"/><path d="M19.656 14H22"/><path d="M2 14h12"/><path d="m2 2 20 20"/><path d="M20 20H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2"/><path d="M9.656 4H20a2 2 0 0 1 2 2v10.344"/>"#);

/// `towel-rack`.
pub const TOWEL_RACK: Icon = lucide!("towel-rack", r#"<path d="M22 7h-2"/><path d="M6.5 3h11A2.5 2.5 0 0 1 20 5.5V20a1 1 0 0 1-1 1h-9a1 1 0 0 1-1-1V5.5a1 1 0 0 0-5 0V17a1 1 0 0 0 1 1h4"/><path d="M9 7H2"/>"#);

/// `tower-control`.
pub const TOWER_CONTROL: Icon = lucide!("tower-control", r#"<path d="M18.2 12.27 20 6H4l1.8 6.27a1 1 0 0 0 .95.73h10.5a1 1 0 0 0 .96-.73Z"/><path d="M8 13v9"/><path d="M16 22v-9"/><path d="m9 6 1 7"/><path d="m15 6-1 7"/><path d="M12 6V2"/><path d="M13 2h-2"/>"#);

/// `toy-brick`.
pub const TOY_BRICK: Icon = lucide!("toy-brick", r#"<rect width="18" height="12" x="3" y="8" rx="1"/><path d="M10 8V5c0-.6-.4-1-1-1H6a1 1 0 0 0-1 1v3"/><path d="M19 8V5c0-.6-.4-1-1-1h-3a1 1 0 0 0-1 1v3"/>"#);

/// `tractor`.
pub const TRACTOR: Icon = lucide!("tractor", r#"<path d="m10 11 11 .9a1 1 0 0 1 .8 1.1l-.665 4.158a1 1 0 0 1-.988.842H20"/><path d="M16 18h-5"/><path d="M18 5a1 1 0 0 0-1 1v5.573"/><path d="M3 4h8.129a1 1 0 0 1 .99.863L13 11.246"/><path d="M4 11V4"/><path d="M7 15h.01"/><path d="M8 10.1V4"/><circle cx="18" cy="18" r="2"/><circle cx="7" cy="15" r="5"/>"#);

/// `traffic-cone`.
pub const TRAFFIC_CONE: Icon = lucide!("traffic-cone", r#"<path d="M16.05 10.966a5 2.5 0 0 1-8.1 0"/><path d="m16.923 14.049 4.48 2.04a1 1 0 0 1 .001 1.831l-8.574 3.9a2 2 0 0 1-1.66 0l-8.574-3.91a1 1 0 0 1 0-1.83l4.484-2.04"/><path d="M16.949 14.14a5 2.5 0 1 1-9.9 0L10.063 3.5a2 2 0 0 1 3.874 0z"/><path d="M9.194 6.57a5 2.5 0 0 0 5.61 0"/>"#);

/// `trailer`.
pub const TRAILER: Icon = lucide!("trailer", r#"<path d="M10 11.341V10"/><path d="M14 13v-3"/><path d="M18 17V8a2 2 0 00-2-2H4a2 2 0 00-2 2v7a2 2 0 002 2h2"/><path d="M22 15v1a1 1 0 01-1 1H10"/><path d="M6 11.341V10"/><circle cx="8" cy="17" r="2"/>"#);

/// `train-front`.
pub const TRAIN_FRONT: Icon = lucide!("train-front", r#"<path d="M8 3.1V7a4 4 0 0 0 8 0V3.1"/><path d="m9 15-1-1"/><path d="m15 15 1-1"/><path d="M9 19c-2.8 0-5-2.2-5-5v-4a8 8 0 0 1 16 0v4c0 2.8-2.2 5-5 5Z"/><path d="m8 19-2 3"/><path d="m16 19 2 3"/>"#);

/// `train-front-tunnel`.
pub const TRAIN_FRONT_TUNNEL: Icon = lucide!("train-front-tunnel", r#"<path d="M2 22V12a10 10 0 1 1 20 0v10"/><path d="M15 6.8v1.4a3 2.8 0 1 1-6 0V6.8"/><path d="M10 15h.01"/><path d="M14 15h.01"/><path d="M10 19a4 4 0 0 1-4-4v-3a6 6 0 1 1 12 0v3a4 4 0 0 1-4 4Z"/><path d="m9 19-2 3"/><path d="m15 19 2 3"/>"#);

/// `train-track`.
pub const TRAIN_TRACK: Icon = lucide!("train-track", r#"<path d="M2 17 17 2"/><path d="m2 14 8 8"/><path d="m5 11 8 8"/><path d="m8 8 8 8"/><path d="m11 5 8 8"/><path d="m14 2 8 8"/><path d="M7 22 22 7"/>"#);

/// `tram-front`.
pub const TRAM_FRONT: Icon = lucide!("tram-front", r#"<rect width="16" height="16" x="4" y="3" rx="2"/><path d="M4 11h16"/><path d="M12 3v8"/><path d="m8 19-2 3"/><path d="m18 22-2-3"/><path d="M8 15h.01"/><path d="M16 15h.01"/>"#);

/// `transgender`.
pub const TRANSGENDER: Icon = lucide!("transgender", r#"<path d="M12 16v6"/><path d="M14 20h-4"/><path d="M18 2h4v4"/><path d="m2 2 7.17 7.17"/><path d="M2 5.355V2h3.357"/><path d="m22 2-7.17 7.17"/><path d="M8 5 5 8"/><circle cx="12" cy="12" r="4"/>"#);

/// `trash`.
pub const TRASH: Icon = lucide!("trash", r#"<path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>"#);

/// `trash-off`.
pub const TRASH_OFF: Icon = lucide!("trash-off", r#"<path d="M10 11v6"/><path d="M14 17v-3"/><path d="M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-1.576.768"/><path d="M19 6v7.344"/><path d="m2 2 20 20"/><path d="M21 6h-9.344"/><path d="M3 6h3"/><path d="M5 6v14a2 2 0 002 2h10a2 2 0 002-2v-1"/>"#);

/// `tree-deciduous`.
pub const TREE_DECIDUOUS: Icon = lucide!("tree-deciduous", r#"<path d="M8 19a4 4 0 0 1-2.24-7.32A3.5 3.5 0 0 1 9 6.03V6a3 3 0 1 1 6 0v.04a3.5 3.5 0 0 1 3.24 5.65A4 4 0 0 1 16 19Z"/><path d="M12 19v3"/>"#);

/// `tree-palm`.
pub const TREE_PALM: Icon = lucide!("tree-palm", r#"<path d="M13 8c0-2.76-2.46-5-5.5-5S2 5.24 2 8h2l1-1 1 1h4"/><path d="M13 7.14A5.82 5.82 0 0 1 16.5 6c3.04 0 5.5 2.24 5.5 5h-3l-1-1-1 1h-3"/><path d="M5.89 9.71c-2.15 2.15-2.3 5.47-.35 7.43l4.24-4.25.7-.7.71-.71 2.12-2.12c-1.95-1.96-5.27-1.8-7.42.35"/><path d="M11 15.5c.5 2.5-.17 4.5-1 6.5h4c2-5.5-.5-12-1-14"/>"#);

/// `tree-pine`.
pub const TREE_PINE: Icon = lucide!("tree-pine", r#"<path d="m17 14 3 3.3a1 1 0 0 1-.7 1.7H4.7a1 1 0 0 1-.7-1.7L7 14h-.3a1 1 0 0 1-.7-1.7L9 9h-.2A1 1 0 0 1 8 7.3L12 3l4 4.3a1 1 0 0 1-.8 1.7H15l3 3.3a1 1 0 0 1-.7 1.7H17Z"/><path d="M12 22v-3"/>"#);

/// `trees`.
pub const TREES: Icon = lucide!("trees", r#"<path d="M10 10v.2A3 3 0 0 1 8.9 16H5a3 3 0 0 1-1-5.8V10a3 3 0 0 1 6 0Z"/><path d="M7 16v6"/><path d="M13 19v3"/><path d="M12 19h8.3a1 1 0 0 0 .7-1.7L18 14h.3a1 1 0 0 0 .7-1.7L16 9h.2a1 1 0 0 0 .8-1.7L13 3l-1.4 1.5"/>"#);

/// `trending-down`.
pub const TRENDING_DOWN: Icon = lucide!("trending-down", r#"<path d="M16 17h6v-6"/><path d="m22 17-8.5-8.5-5 5L2 7"/>"#);

/// `trending-up`.
pub const TRENDING_UP: Icon = lucide!("trending-up", r#"<path d="M16 7h6v6"/><path d="m22 7-8.5 8.5-5-5L2 17"/>"#);

/// `trending-up-down`.
pub const TRENDING_UP_DOWN: Icon = lucide!("trending-up-down", r#"<path d="M14.828 14.828 21 21"/><path d="M21 16v5h-5"/><path d="m21 3-9 9-4-4-6 6"/><path d="M21 8V3h-5"/>"#);

/// `triangle`.
pub const TRIANGLE: Icon = lucide!("triangle", r#"<path d="M13.73 4a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/>"#);

/// `triangle-dashed`.
pub const TRIANGLE_DASHED: Icon = lucide!("triangle-dashed", r#"<path d="M10.17 4.193a2 2 0 0 1 3.666.013"/><path d="M14 21h2"/><path d="m15.874 7.743 1 1.732"/><path d="m18.849 12.952 1 1.732"/><path d="M21.824 18.18a2 2 0 0 1-1.835 2.824"/><path d="M4.024 21a2 2 0 0 1-1.839-2.839"/><path d="m5.136 12.952-1 1.732"/><path d="M8 21h2"/><path d="m8.102 7.743-1 1.732"/>"#);

/// `triangle-exclamation-point`.
pub const TRIANGLE_EXCLAMATION_POINT: Icon = lucide!("triangle-exclamation-point", r#"<path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/>"#);

/// `triangle-right`.
pub const TRIANGLE_RIGHT: Icon = lucide!("triangle-right", r#"<path d="M22 18a2 2 0 0 1-2 2H3c-1.1 0-1.3-.6-.4-1.3L20.4 4.3c.9-.7 1.6-.4 1.6.7Z"/>"#);

/// `triangles-centerline-dashed-horizontal`.
pub const TRIANGLES_CENTERLINE_DASHED_HORIZONTAL: Icon = lucide!("triangles-centerline-dashed-horizontal", r#"<path d="M10 12H8"/><path d="M16 12h-2"/><path d="M22 12h-2"/><path d="M4 12H2"/><path d="M7.298 20.288A1 1 0 008 22h8a1 1 0 00.703-1.712l-3.991-3.99a1 1 0 00-1.424-.001z"/><path d="M7.298 3.712A1 1 0 018 2h8a1 1 0 01.703 1.712l-3.991 3.99a1 1 0 01-1.424.001z"/>"#);

/// `triangles-centerline-dashed-vertical`.
pub const TRIANGLES_CENTERLINE_DASHED_VERTICAL: Icon = lucide!("triangles-centerline-dashed-vertical", r#"<path d="M12 14v2"/><path d="M12 20v2"/><path d="M12 2v2"/><path d="M12 8v2"/><path d="M20.288 16.703A1 1 0 0022 16V8a1 1 0 00-1.712-.703l-3.99 3.991a1 1 0 00-.001 1.424z"/><path d="M3.712 16.703A1 1 0 012 16V8a1 1 0 011.712-.703l3.99 3.991a1 1 0 01.001 1.424z"/>"#);

/// `trophy`.
pub const TROPHY: Icon = lucide!("trophy", r#"<path d="M10 14.66V17a1 1 0 0 1-1 1 2 2 0 0 0-2 2v2"/><path d="M14 14.66V17a1 1 0 0 0 1 1 2 2 0 0 1 2 2v2"/><path d="M17.916 10H19.5A2.5 2.5 0 0 0 22 7.5V5a1 1 0 0 0-1-1h-3"/><path d="M4 22h16"/><path d="M6 9a6 6 0 0 0 12 0V3a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1z"/><path d="M6.084 10H4.5A2.5 2.5 0 0 1 2 7.5V5a1 1 0 0 1 1-1h3"/>"#);

/// `truck`.
pub const TRUCK: Icon = lucide!("truck", r#"<path d="M14 18V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v11a1 1 0 0 0 1 1h2"/><path d="M15 18H9"/><path d="M19 18h2a1 1 0 0 0 1-1v-3.65a1 1 0 0 0-.22-.624l-3.48-4.35A1 1 0 0 0 17.52 8H14"/><circle cx="17" cy="18" r="2"/><circle cx="7" cy="18" r="2"/>"#);

/// `truck-electric`.
pub const TRUCK_ELECTRIC: Icon = lucide!("truck-electric", r#"<path d="M14 19V7a2 2 0 0 0-2-2H9"/><path d="M15 19H9"/><path d="M19 19h2a1 1 0 0 0 1-1v-3.65a1 1 0 0 0-.22-.62L18.3 9.38a1 1 0 0 0-.78-.38H14"/><path d="M2 13v5a1 1 0 0 0 1 1h2"/><path d="M4 3 2.15 5.15a.495.495 0 0 0 .35.86h2.15a.47.47 0 0 1 .35.86L3 9.02"/><circle cx="17" cy="19" r="2"/><circle cx="7" cy="19" r="2"/>"#);

/// `tube-lotion`.
pub const TUBE_LOTION: Icon = lucide!("tube-lotion", r#"<path d="M15 18v3a1 1 0 0 1-1 1h-4a1 1 0 0 1-1-1v-3"/><path d="M17 2a2 2 0 0 1 1.6 3.2A8 8 0 0 0 17 10v6a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2v-6a8 8 0 0 0-1.6-4.8A2 2 0 0 1 7 2z"/><path d="M7 10a6.47 6.47 0 0 1 5 0 6.47 6.47 0 0 0 5 0"/>"#);

/// `turkish-lira`.
pub const TURKISH_LIRA: Icon = lucide!("turkish-lira", r#"<path d="m14 10-8 4"/><path d="M14 5 6 9"/><path d="M18 12a8 8 0 01-7.003 7.937c-.547.07-.997-.385-.997-.937V4"/>"#);

/// `turntable`.
pub const TURNTABLE: Icon = lucide!("turntable", r#"<path d="M10 12.01h.01"/><path d="M18 8v4a8 8 0 0 1-1.07 4"/><circle cx="10" cy="12" r="4"/><rect x="2" y="4" width="20" height="16" rx="2"/>"#);

/// `turtle`.
pub const TURTLE: Icon = lucide!("turtle", r#"<path d="m12 10 2 4v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3a8 8 0 1 0-16 0v3a1 1 0 0 0 1 1h2a1 1 0 0 0 1-1v-3l2-4h4Z"/><path d="M4.82 7.9 8 10"/><path d="M15.18 7.9 12 10"/><path d="M16.93 10H20a2 2 0 0 1 0 4H2"/>"#);

/// `tv`.
pub const TV: Icon = lucide!("tv", r#"<path d="m17 2-5 5-5-5"/><rect width="20" height="15" x="2" y="7" rx="2"/>"#);

/// `tv-minimal`.
pub const TV_MINIMAL: Icon = lucide!("tv-minimal", r#"<path d="M7 21h10"/><rect width="20" height="14" x="2" y="3" rx="2"/>"#);

/// `tv-minimal-play`.
pub const TV_MINIMAL_PLAY: Icon = lucide!("tv-minimal-play", r#"<path d="M15.033 9.44a.647.647 0 0 1 0 1.12l-4.065 2.352a.645.645 0 0 1-.968-.56V7.648a.645.645 0 0 1 .967-.56z"/><path d="M7 21h10"/><rect width="20" height="14" x="2" y="3" rx="2"/>"#);

/// `type`.
pub const TYPE: Icon = lucide!("type", r#"<path d="M12 4v16"/><path d="M4 7V5a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2"/><path d="M9 20h6"/>"#);

/// `type-outline`.
pub const TYPE_OUTLINE: Icon = lucide!("type-outline", r#"<path d="M14 16.5a.5.5 0 0 0 .5.5h.5a2 2 0 0 1 0 4H9a2 2 0 0 1 0-4h.5a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5V8a2 2 0 0 1-4 0V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v3a2 2 0 0 1-4 0v-.5a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5Z"/>"#);

/// `umbrella`.
pub const UMBRELLA: Icon = lucide!("umbrella", r#"<path d="M12 13v7a2 2 0 0 0 4 0"/><path d="M12 2v2"/><path d="M20.992 13a1 1 0 0 0 .97-1.274 10.284 10.284 0 0 0-19.923 0A1 1 0 0 0 3 13z"/>"#);

/// `umbrella-off`.
pub const UMBRELLA_OFF: Icon = lucide!("umbrella-off", r#"<path d="M12 13v7a2 2 0 0 0 4 0"/><path d="M12 2v2"/><path d="M18.656 13h2.336a1 1 0 0 0 .97-1.274 10.284 10.284 0 0 0-12.07-7.51"/><path d="m2 2 20 20"/><path d="M5.961 5.957a10.28 10.28 0 0 0-3.922 5.769A1 1 0 0 0 3 13h10"/>"#);

/// `underline`.
pub const UNDERLINE: Icon = lucide!("underline", r#"<path d="M6 4v6a6 6 0 0 0 12 0V4"/><line x1="4" x2="20" y1="20" y2="20"/>"#);

/// `undo`.
pub const UNDO: Icon = lucide!("undo", r#"<path d="M3 7v6h6"/><path d="M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13"/>"#);

/// `undo-2`.
pub const UNDO_2: Icon = lucide!("undo-2", r#"<path d="M9 14 4 9l5-5"/><path d="M4 9h10.5a5.5 5.5 0 0 1 5.5 5.5a5.5 5.5 0 0 1-5.5 5.5H11"/>"#);

/// `undo-dot`.
pub const UNDO_DOT: Icon = lucide!("undo-dot", r#"<path d="M21 17a9 9 0 0 0-15-6.7L3 13"/><path d="M3 7v6h6"/><circle cx="12" cy="17" r="1"/>"#);

/// `unfold-horizontal`.
pub const UNFOLD_HORIZONTAL: Icon = lucide!("unfold-horizontal", r#"<path d="M16 12h6"/><path d="M8 12H2"/><path d="M12 2v2"/><path d="M12 8v2"/><path d="M12 14v2"/><path d="M12 20v2"/><path d="m19 15 3-3-3-3"/><path d="m5 9-3 3 3 3"/>"#);

/// `unfold-vertical`.
pub const UNFOLD_VERTICAL: Icon = lucide!("unfold-vertical", r#"<path d="M12 22v-6"/><path d="M12 8V2"/><path d="M4 12H2"/><path d="M10 12H8"/><path d="M16 12h-2"/><path d="M22 12h-2"/><path d="m15 19-3 3-3-3"/><path d="m15 5-3-3-3 3"/>"#);

/// `ungroup`.
pub const UNGROUP: Icon = lucide!("ungroup", r#"<rect x="11" y="14" width="10" height="7" rx="2"/><rect x="3" y="3" width="10" height="7" rx="2"/>"#);

/// `university`.
pub const UNIVERSITY: Icon = lucide!("university", r#"<path d="M14 21v-3a2 2 0 0 0-4 0v3"/><path d="M18 12h.01"/><path d="M18 16h.01"/><path d="M22 7a1 1 0 0 0-1-1h-2a2 2 0 0 1-1.143-.359L13.143 2.36a2 2 0 0 0-2.286-.001L6.143 5.64A2 2 0 0 1 5 6H3a1 1 0 0 0-1 1v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2z"/><path d="M6 12h.01"/><path d="M6 16h.01"/><circle cx="12" cy="10" r="2"/>"#);

/// `unlink`.
pub const UNLINK: Icon = lucide!("unlink", r#"<path d="m18.84 12.25 1.72-1.71h-.02a5.004 5.004 0 0 0-.12-7.07 5.006 5.006 0 0 0-6.95 0l-1.72 1.71"/><path d="m5.17 11.75-1.71 1.71a5.004 5.004 0 0 0 .12 7.07 5.006 5.006 0 0 0 6.95 0l1.71-1.71"/><line x1="8" x2="8" y1="2" y2="5"/><line x1="2" x2="5" y1="8" y2="8"/><line x1="16" x2="16" y1="19" y2="22"/><line x1="19" x2="22" y1="16" y2="16"/>"#);

/// `unlink-2`.
pub const UNLINK_2: Icon = lucide!("unlink-2", r#"<path d="M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2"/>"#);

/// `unplug`.
pub const UNPLUG: Icon = lucide!("unplug", r#"<path d="m19 5 3-3"/><path d="m2 22 3-3"/><path d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"/><path d="M7.5 13.5 10 11"/><path d="M10.5 16.5 13 14"/><path d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z"/>"#);

/// `upload`.
pub const UPLOAD: Icon = lucide!("upload", r#"<path d="M12 3v12"/><path d="m17 8-5-5-5 5"/><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>"#);

/// `usb`.
pub const USB: Icon = lucide!("usb", r#"<circle cx="10" cy="7" r="1"/><circle cx="4" cy="20" r="1"/><path d="M4.7 19.3 19 5"/><path d="m21 3-3 1 2 2Z"/><path d="M9.26 7.68 5 12l2 5"/><path d="m10 14 5 2 3.5-3.5"/><path d="m18 12 1-1 1 1-1 1Z"/>"#);

/// `usb-c-port`.
pub const USB_C_PORT: Icon = lucide!("usb-c-port", r#"<path d="M6 12h12"/><rect x="2" y="8" width="20" height="8" rx="4"/>"#);

/// `user`.
pub const USER: Icon = lucide!("user", r#"<path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>"#);

/// `user-check`.
pub const USER_CHECK: Icon = lucide!("user-check", r#"<path d="m16 11 2 2 4-4"/><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/>"#);

/// `user-cog`.
pub const USER_COG: Icon = lucide!("user-cog", r#"<path d="M10 15H6a4 4 0 0 0-4 4v2"/><path d="m14.305 16.53.923-.382"/><path d="m15.228 13.852-.923-.383"/><path d="m16.852 12.228-.383-.923"/><path d="m16.852 17.772-.383.924"/><path d="m19.148 12.228.383-.923"/><path d="m19.53 18.696-.382-.924"/><path d="m20.772 13.852.924-.383"/><path d="m20.772 16.148.924.383"/><circle cx="18" cy="15" r="3"/><circle cx="9" cy="7" r="4"/>"#);

/// `user-exclamation-point`.
pub const USER_EXCLAMATION_POINT: Icon = lucide!("user-exclamation-point", r#"<path d="M15 15H9a4 4 0 00-4 4v2"/><path d="M18.988 13v4"/><path d="M18.988 21h.01"/><circle cx="12" cy="7" r="4"/>"#);

/// `user-group`.
pub const USER_GROUP: Icon = lucide!("user-group", r#"<path d="M17 21v-1a2 2 0 00-2-2H9a2 2 0 00-2 2v1"/><path d="M19 10h1a2 2 0 012 2v1"/><path d="M5 10H4a2 2 0 00-2 2v1"/><circle cx="12" cy="11" r="3"/><circle cx="18" cy="4" r="2"/><circle cx="6" cy="4" r="2"/>"#);

/// `user-key`.
pub const USER_KEY: Icon = lucide!("user-key", r#"<path d="M20 11v6"/><path d="M20 13h2"/><path d="M3 21v-2a4 4 0 0 1 4-4h6a4 4 0 0 1 2.072.578"/><circle cx="10" cy="7" r="4"/><circle cx="20" cy="19" r="2"/>"#);

/// `user-lock`.
pub const USER_LOCK: Icon = lucide!("user-lock", r#"<path d="M19 16v-2a2 2 0 0 0-4 0v2"/><path d="M9.5 15H7a4 4 0 0 0-4 4v2"/><circle cx="10" cy="7" r="4"/><rect x="13" y="16" width="8" height="5" rx=".899"/>"#);

/// `user-minus`.
pub const USER_MINUS: Icon = lucide!("user-minus", r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="22" x2="16" y1="11" y2="11"/>"#);

/// `user-pen`.
pub const USER_PEN: Icon = lucide!("user-pen", r#"<path d="M11.5 15H7a4 4 0 0 0-4 4v2"/><path d="M21.378 16.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/><circle cx="10" cy="7" r="4"/>"#);

/// `user-plus`.
pub const USER_PLUS: Icon = lucide!("user-plus", r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="19" x2="19" y1="8" y2="14"/><line x1="22" x2="16" y1="11" y2="11"/>"#);

/// `user-round`.
pub const USER_ROUND: Icon = lucide!("user-round", r#"<circle cx="12" cy="8" r="5"/><path d="M20 21a8 8 0 0 0-16 0"/>"#);

/// `user-round-arrow-left`.
pub const USER_ROUND_ARROW_LEFT: Icon = lucide!("user-round-arrow-left", r#"<path d="m19 16-3 3"/><path d="M2 21a8 8 0 0 1 12.664-6.5"/><path d="M22 19h-6l3 3"/><circle cx="10" cy="8" r="5"/>"#);

/// `user-round-check`.
pub const USER_ROUND_CHECK: Icon = lucide!("user-round-check", r#"<path d="M2 21a8 8 0 0 1 13.292-6"/><circle cx="10" cy="8" r="5"/><path d="m16 19 2 2 4-4"/>"#);

/// `user-round-cog`.
pub const USER_ROUND_COG: Icon = lucide!("user-round-cog", r#"<path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="M2 21a8 8 0 0 1 10.434-7.62"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><circle cx="10" cy="8" r="5"/><circle cx="18" cy="18" r="3"/>"#);

/// `user-round-exclamation-point`.
pub const USER_ROUND_EXCLAMATION_POINT: Icon = lucide!("user-round-exclamation-point", r#"<path d="M19.99 13v4"/><path d="M19.99 21H20"/><path d="M4 21a8 8 0 0111.99-6.934"/><circle cx="12" cy="8" r="5"/>"#);

/// `user-round-group`.
pub const USER_ROUND_GROUP: Icon = lucide!("user-round-group", r#"<path d="M17 21a5 5 0 00-10 0"/><path d="M22 10.5a3.5 3.5 0 00-5.507-2.868"/><path d="M7.507 7.632A3.5 3.5 0 002 10.5"/><circle cx="12" cy="13" r="3"/><circle cx="18.5" cy="4.5" r="2.5"/><circle cx="5.5" cy="4.5" r="2.5"/>"#);

/// `user-round-key`.
pub const USER_ROUND_KEY: Icon = lucide!("user-round-key", r#"<path d="M19 11v6"/><path d="M19 13h2"/><path d="M2 21a8 8 0 0 1 12.868-6.349"/><circle cx="10" cy="8" r="5"/><circle cx="19" cy="19" r="2"/>"#);

/// `user-round-minus`.
pub const USER_ROUND_MINUS: Icon = lucide!("user-round-minus", r#"<path d="M2 21a8 8 0 0 1 13.292-6"/><circle cx="10" cy="8" r="5"/><path d="M22 19h-6"/>"#);

/// `user-round-pen`.
pub const USER_ROUND_PEN: Icon = lucide!("user-round-pen", r#"<path d="M2 21a8 8 0 0 1 10.821-7.487"/><path d="M21.378 16.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/><circle cx="10" cy="8" r="5"/>"#);

/// `user-round-plus`.
pub const USER_ROUND_PLUS: Icon = lucide!("user-round-plus", r#"<path d="M2 21a8 8 0 0 1 13.292-6"/><circle cx="10" cy="8" r="5"/><path d="M19 16v6"/><path d="M22 19h-6"/>"#);

/// `user-round-search`.
pub const USER_ROUND_SEARCH: Icon = lucide!("user-round-search", r#"<circle cx="10" cy="8" r="5"/><path d="M2 21a8 8 0 0 1 10.434-7.62"/><circle cx="18" cy="18" r="3"/><path d="m22 22-1.9-1.9"/>"#);

/// `user-round-shield`.
pub const USER_ROUND_SHIELD: Icon = lucide!("user-round-shield", r#"<path d="M2 21a8 8 0 0 1 8-8 5 5 0 1 1 4.845-3.759"/><path d="M22 17.5c0 2.5-1.75 3.75-3.83 4.474a.5.5 0 0 1-.335-.005C15.75 21.25 14 20 14 17.5V14a.5.5 0 0 1 .5-.499c1 0 2.25-.6 3.12-1.36a.6.6 0 0 1 .76-.001c.875.765 2.12 1.36 3.12 1.36a.5.5 0 0 1 .5.5z"/>"#);

/// `user-round-star`.
pub const USER_ROUND_STAR: Icon = lucide!("user-round-star", r#"<path d="M16.051 12.616a1 1 0 0 1 1.909.024l.737 1.452a1 1 0 0 0 .737.535l1.634.256a1 1 0 0 1 .588 1.806l-1.172 1.168a1 1 0 0 0-.282.866l.259 1.613a1 1 0 0 1-1.541 1.134l-1.465-.75a1 1 0 0 0-.912 0l-1.465.75a1 1 0 0 1-1.539-1.133l.258-1.613a1 1 0 0 0-.282-.867l-1.156-1.152a1 1 0 0 1 .572-1.822l1.633-.256a1 1 0 0 0 .737-.535z"/><path d="M2 21a8 8 0 0 1 6.778-7.905"/><path d="M8.927 12.885a5 5 0 1 1 6.061-4.535"/>"#);

/// `user-round-x`.
pub const USER_ROUND_X: Icon = lucide!("user-round-x", r#"<path d="m16.5 16.5 5 5"/><path d="M2 21a8 8 0 0 1 11.531-7.18"/><path d="m21.5 16.5-5 5"/><circle cx="10" cy="8" r="5"/>"#);

/// `user-search`.
pub const USER_SEARCH: Icon = lucide!("user-search", r#"<circle cx="10" cy="7" r="4"/><path d="M10.3 15H7a4 4 0 0 0-4 4v2"/><circle cx="17" cy="17" r="3"/><path d="m21 21-1.9-1.9"/>"#);

/// `user-shield`.
pub const USER_SHIELD: Icon = lucide!("user-shield", r#"<path d="M10 15H6a4 4 0 0 0-4 4v2"/><path d="M22 17.5c0 2.499-1.75 3.749-3.83 4.474a.5.5 0 0 1-.335-.005c-2.085-.72-3.835-1.97-3.835-4.47V14a.5.5 0 0 1 .5-.499c1 0 2.25-.6 3.12-1.36a.6.6 0 0 1 .76-.001c.875.765 2.12 1.36 3.12 1.36a.5.5 0 0 1 .5.5z"/><circle cx="9" cy="7" r="4"/>"#);

/// `user-star`.
pub const USER_STAR: Icon = lucide!("user-star", r#"<path d="M16.051 12.616a1 1 0 0 1 1.909.024l.737 1.452a1 1 0 0 0 .737.535l1.634.256a1 1 0 0 1 .588 1.806l-1.172 1.168a1 1 0 0 0-.282.866l.259 1.613a1 1 0 0 1-1.541 1.134l-1.465-.75a1 1 0 0 0-.912 0l-1.465.75a1 1 0 0 1-1.539-1.133l.258-1.613a1 1 0 0 0-.282-.866l-1.156-1.153a1 1 0 0 1 .572-1.822l1.633-.256a1 1 0 0 0 .737-.535z"/><path d="M8 15H7a4 4 0 0 0-4 4v2"/><circle cx="10" cy="7" r="4"/>"#);

/// `user-x`.
pub const USER_X: Icon = lucide!("user-x", r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><line x1="17" x2="22" y1="8" y2="13"/><line x1="22" x2="17" y1="8" y2="13"/>"#);

/// `users`.
pub const USERS: Icon = lucide!("users", r#"<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><path d="M16 3.128a4 4 0 0 1 0 7.744"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><circle cx="9" cy="7" r="4"/>"#);

/// `users-collaboration`.
pub const USERS_COLLABORATION: Icon = lucide!("users-collaboration", r#"<path d="M12.925 3.047a9 9 0 017.616 6.11"/><path d="M13 22a2 2 0 012-2h4a2 2 0 012 2"/><path d="M3 12a2 2 0 012-2h4a2 2 0 012 2"/><path d="M3.889 15.9a9 9 0 005.247 4.632"/><circle cx="17" cy="14" r="2"/><circle cx="7" cy="4" r="2"/>"#);

/// `users-round`.
pub const USERS_ROUND: Icon = lucide!("users-round", r#"<path d="M18 21a8 8 0 0 0-16 0"/><circle cx="10" cy="8" r="5"/><path d="M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3"/>"#);

/// `users-round-collaboration`.
pub const USERS_ROUND_COLLABORATION: Icon = lucide!("users-round-collaboration", r#"<path d="M20.453 8.911a9 9 0 00-6.696-5.738"/><path d="M21 22a4 4 0 00-8 0"/><path d="M3 12a4 4 0 017.963-.543"/><path d="M3.889 15.9a9 9 0 005.247 4.632"/><circle cx="17" cy="15" r="3"/><circle cx="7" cy="5" r="3"/>"#);

/// `utensils`.
pub const UTENSILS: Icon = lucide!("utensils", r#"<path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 0 0 2-2V2"/><path d="M7 2v20"/><path d="M21 15V2a5 5 0 0 0-5 5v6c0 1.1.9 2 2 2h3Zm0 0v7"/>"#);

/// `utensils-crossed`.
pub const UTENSILS_CROSSED: Icon = lucide!("utensils-crossed", r#"<path d="m16 2-2.3 2.3a3 3 0 0 0 0 4.2l1.8 1.8a3 3 0 0 0 4.2 0L22 8"/><path d="M15 15 3.3 3.3a4.2 4.2 0 0 0 0 6l7.3 7.3c.7.7 2 .7 2.8 0L15 15Zm0 0 7 7"/><path d="m2.1 21.8 6.4-6.3"/><path d="m19 5-7 7"/>"#);

/// `utility-pole`.
pub const UTILITY_POLE: Icon = lucide!("utility-pole", r#"<path d="M12 2v20"/><path d="M2 5h20"/><path d="M3 3v2"/><path d="M7 3v2"/><path d="M17 3v2"/><path d="M21 3v2"/><path d="m19 5-7 7-7-7"/>"#);

/// `van`.
pub const VAN: Icon = lucide!("van", r#"<path d="M13 6v5a1 1 0 0 0 1 1h6.102a1 1 0 0 1 .712.298l.898.91a1 1 0 0 1 .288.702V17a1 1 0 0 1-1 1h-3"/><path d="M5 18H3a1 1 0 0 1-1-1V8a2 2 0 0 1 2-2h12c1.1 0 2.1.8 2.4 1.8l1.176 4.2"/><path d="M9 18h5"/><circle cx="16" cy="18" r="2"/><circle cx="7" cy="18" r="2"/>"#);

/// `variable`.
pub const VARIABLE: Icon = lucide!("variable", r#"<path d="M8 21s-4-3-4-9 4-9 4-9"/><path d="M16 3s4 3 4 9-4 9-4 9"/><line x1="15" x2="9" y1="9" y2="15"/><line x1="9" x2="15" y1="9" y2="15"/>"#);

/// `vault`.
pub const VAULT: Icon = lucide!("vault", r#"<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="7.5" cy="7.5" r=".5" fill="currentColor"/><path d="m7.9 7.9 2.7 2.7"/><circle cx="16.5" cy="7.5" r=".5" fill="currentColor"/><path d="m13.4 10.6 2.7-2.7"/><circle cx="7.5" cy="16.5" r=".5" fill="currentColor"/><path d="m7.9 16.1 2.7-2.7"/><circle cx="16.5" cy="16.5" r=".5" fill="currentColor"/><path d="m13.4 13.4 2.7 2.7"/><circle cx="12" cy="12" r="2"/>"#);

/// `vector-polygon`.
pub const VECTOR_POLYGON: Icon = lucide!("vector-polygon", r#"<path d="m12.828 4.813 5.344 2.375"/><path d="m15.769 18.153 3.461-8.306"/><path d="m5.687 14.074 7.625 4.852"/><path d="M9.772 5.579 5.228 11.42"/><circle cx="11" cy="4" r="2"/><circle cx="15" cy="20" r="2"/><circle cx="20" cy="8" r="2"/><circle cx="4" cy="13" r="2"/>"#);

/// `vector-square`.
pub const VECTOR_SQUARE: Icon = lucide!("vector-square", r#"<path d="M17.055 4.533a24 24 0 00-10.11 0"/><path d="M19.467 17.055a24 24 0 000-10.11"/><path d="M4.533 6.945a24 24 0 000 10.11"/><path d="M6.945 19.467a24 24 0 0010.11 0"/><circle cx="19" cy="19" r="2"/><circle cx="19" cy="5" r="2"/><circle cx="5" cy="19" r="2"/><circle cx="5" cy="5" r="2"/>"#);

/// `vegan`.
pub const VEGAN: Icon = lucide!("vegan", r#"<path d="M16 8q6 0 6-6-6 0-6 6"/><path d="M17.41 3.59a10 10 0 1 0 3 3"/><path d="M2 2a26.6 26.6 0 0 1 10 20c.9-6.82 1.5-9.5 4-14"/>"#);

/// `venetian-mask`.
pub const VENETIAN_MASK: Icon = lucide!("venetian-mask", r#"<path d="M18 11c-1.5 0-2.5.5-3 2"/><path d="M4 6a2 2 0 0 0-2 2v4a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V8a2 2 0 0 0-2-2h-3a8 8 0 0 0-5 2 8 8 0 0 0-5-2z"/><path d="M6 11c1.5 0 2.5.5 3 2"/>"#);

/// `venus`.
pub const VENUS: Icon = lucide!("venus", r#"<path d="M12 15v7"/><path d="M9 19h6"/><circle cx="12" cy="9" r="6"/>"#);

/// `venus-and-mars`.
pub const VENUS_AND_MARS: Icon = lucide!("venus-and-mars", r#"<path d="M10 20h4"/><path d="M12 16v6"/><path d="M17 2h4v4"/><path d="m21 2-5.46 5.46"/><circle cx="12" cy="11" r="5"/>"#);

/// `vibrate`.
pub const VIBRATE: Icon = lucide!("vibrate", r#"<path d="m2 8 2 2-2 2 2 2-2 2"/><path d="m22 8-2 2 2 2-2 2 2 2"/><rect width="8" height="14" x="8" y="5" rx="1"/>"#);

/// `vibrate-off`.
pub const VIBRATE_OFF: Icon = lucide!("vibrate-off", r#"<path d="m2 8 2 2-2 2 2 2-2 2"/><path d="m22 8-2 2 2 2-2 2 2 2"/><path d="M8 8v10c0 .55.45 1 1 1h6c.55 0 1-.45 1-1v-2"/><path d="M16 10.34V6c0-.55-.45-1-1-1h-4.34"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `video`.
pub const VIDEO: Icon = lucide!("video", r#"<path d="m16 13 5.223 3.482a.5.5 0 0 0 .777-.416V7.87a.5.5 0 0 0-.752-.432L16 10.5"/><rect x="2" y="6" width="14" height="12" rx="2"/>"#);

/// `video-off`.
pub const VIDEO_OFF: Icon = lucide!("video-off", r#"<path d="M10.66 6H14a2 2 0 0 1 2 2v2.5l5.248-3.062A.5.5 0 0 1 22 7.87v8.196"/><path d="M16 16a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h2"/><path d="m2 2 20 20"/>"#);

/// `videotape`.
pub const VIDEOTAPE: Icon = lucide!("videotape", r#"<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M2 8h20"/><circle cx="8" cy="14" r="2"/><path d="M8 12h8"/><circle cx="16" cy="14" r="2"/>"#);

/// `view`.
pub const VIEW: Icon = lucide!("view", r#"<path d="M21 17v2a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-2"/><path d="M21 7V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v2"/><circle cx="12" cy="12" r="1"/><path d="M18.944 12.33a1 1 0 0 0 0-.66 7.5 7.5 0 0 0-13.888 0 1 1 0 0 0 0 .66 7.5 7.5 0 0 0 13.888 0"/>"#);

/// `virus`.
pub const VIRUS: Icon = lucide!("virus", r#"<path d="M12 14.991h.01"/><path d="M12 22v-3"/><path d="M12 2v3"/><path d="M13 22h-2"/><path d="M13 2h-2"/><path d="M13.99 10H14"/><path d="m16.5 19.794-1-1.733"/><path d="m16.5 4.205-1 1.732"/><path d="m19.794 16.5-1.732-1"/><path d="m19.794 7.5-1.732 1"/><path d="M2 12h3"/><path d="M2 13v-2"/><path d="M22 12h-3"/><path d="M22 13v-2"/><path d="m4.206 16.5 1.732-1"/><path d="m4.206 7.5 1.732 1"/><path d="m7.5 19.794 1-1.733"/><path d="m7.5 4.205 1 1.732"/><path d="M9 12h.01"/><circle cx="12" cy="12" r="7"/>"#);

/// `virus-off`.
pub const VIRUS_OFF: Icon = lucide!("virus-off", r#"<path d="M10.01 10h.01"/><path d="M12 14.991h.01"/><path d="M12 22v-3"/><path d="M12 2v3"/><path d="M13 22h-2"/><path d="M13 2h-2"/><path d="m16.5 19.794-1-1.733"/><path d="m16.5 4.205-1 1.732"/><path d="M18.891 13.235a7 7 0 00-8.126-8.126"/><path d="m19.794 7.5-1.732 1"/><path d="M2 12h3"/><path d="M2 13v-2"/><path d="m2 2 20 20"/><path d="M22 12h-3"/><path d="M22 13v-2"/><path d="m4.206 16.5 1.732-1"/><path d="m4.206 7.5 1.732 1"/><path d="M7.05 7.05a7 7 0 009.9 9.9"/><path d="m7.5 19.794 1-1.733"/><path d="M9 12h.01"/>"#);

/// `voicemail`.
pub const VOICEMAIL: Icon = lucide!("voicemail", r#"<circle cx="6" cy="12" r="4"/><circle cx="18" cy="12" r="4"/><line x1="6" x2="18" y1="16" y2="16"/>"#);

/// `volleyball`.
pub const VOLLEYBALL: Icon = lucide!("volleyball", r#"<path d="M11 7a16 16 20 0 1 10.98 4.362"/><path d="M12 12a13 13 0 0 1-8.66 5"/><path d="M16.83 13.634a16 16 0 0 1-9.267 7.328"/><path d="M20.66 17A13 13 0 0 0 12 12a13 13 0 0 1 0-10"/><path d="M8.17 15.366a16 16 0 0 1-1.713-11.69"/><circle cx="12" cy="12" r="10"/>"#);

/// `volume`.
pub const VOLUME: Icon = lucide!("volume", r#"<path d="M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"/>"#);

/// `volume-1`.
pub const VOLUME_1: Icon = lucide!("volume-1", r#"<path d="M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"/><path d="M16 9a5 5 0 0 1 0 6"/>"#);

/// `volume-2`.
pub const VOLUME_2: Icon = lucide!("volume-2", r#"<path d="M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z"/><path d="M16 9a5 5 0 0 1 0 6"/><path d="M19.364 18.364a9 9 0 0 0 0-12.728"/>"#);

/// `volume-off`.
pub const VOLUME_OFF: Icon = lucide!("volume-off", r#"<path d="M16 9a5 5 0 0 1 .95 2.293"/><path d="M19.364 5.636a9 9 0 0 1 1.889 9.96"/><path d="m2 2 20 20"/><path d="m7 7-.587.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298V11"/><path d="M9.828 4.172A.686.686 0 0 1 11 4.657v.686"/>"#);

/// `volume-x`.
pub const VOLUME_X: Icon = lucide!("volume-x", r#"<path d="M11 4.702a.7.7 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.7.7 0 0 0 11 19.298z"/><path d="m16.5 14.5 5-5"/><path d="m16.5 9.5 5 5"/>"#);

/// `vote`.
pub const VOTE: Icon = lucide!("vote", r#"<path d="m9 12 2 2 4-4"/><path d="M5 7c0-1.1.9-2 2-2h10a2 2 0 0 1 2 2v12H5V7Z"/><path d="M22 19H2"/>"#);

/// `wallet`.
pub const WALLET: Icon = lucide!("wallet", r#"<path d="M19 7V4a1 1 0 0 0-1-1H5a2 2 0 0 0 0 4h15a1 1 0 0 1 1 1v4h-3a2 2 0 0 0 0 4h3a1 1 0 0 0 1-1v-2a1 1 0 0 0-1-1"/><path d="M3 5v14a2 2 0 0 0 2 2h15a1 1 0 0 0 1-1v-4"/>"#);

/// `wallet-cards`.
pub const WALLET_CARDS: Icon = lucide!("wallet-cards", r#"<path d="M3 11h3.75a2 2 0 0 1 1.6.8l.45.6a4 4 0 0 0 6.4 0l.45-.6a2 2 0 0 1 1.6-.8H21"/><path d="M3 7h18"/><rect x="3" y="3" width="18" height="18" rx="2"/>"#);

/// `wallet-minimal`.
pub const WALLET_MINIMAL: Icon = lucide!("wallet-minimal", r#"<path d="M17 14h.01"/><path d="M7 7h12a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h14"/>"#);

/// `wallpaper`.
pub const WALLPAPER: Icon = lucide!("wallpaper", r#"<path d="M12 17v4"/><path d="M8 21h8"/><path d="m9 17 6.1-6.1a2 2 0 0 1 2.81.01L22 15"/><circle cx="8" cy="9" r="2"/><rect x="2" y="3" width="20" height="14" rx="2"/>"#);

/// `wand`.
pub const WAND: Icon = lucide!("wand", r#"<path d="M15 4V2"/><path d="M15 16v-2"/><path d="M8 9h2"/><path d="M20 9h2"/><path d="M17.8 11.8 19 13"/><path d="M15 9h.01"/><path d="M17.8 6.2 19 5"/><path d="m3 21 9-9"/><path d="M12.2 6.2 11 5"/>"#);

/// `wand-sparkles`.
pub const WAND_SPARKLES: Icon = lucide!("wand-sparkles", r#"<path d="m21.64 3.64-1.28-1.28a1.21 1.21 0 0 0-1.72 0L2.36 18.64a1.21 1.21 0 0 0 0 1.72l1.28 1.28a1.2 1.2 0 0 0 1.72 0L21.64 5.36a1.2 1.2 0 0 0 0-1.72"/><path d="m14 7 3 3"/><path d="M5 6v4"/><path d="M19 14v4"/><path d="M10 2v2"/><path d="M7 8H3"/><path d="M21 16h-4"/><path d="M11 3H9"/>"#);

/// `warehouse`.
pub const WAREHOUSE: Icon = lucide!("warehouse", r#"<path d="M18 21V10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1v11"/><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V8a2 2 0 0 1 1.132-1.803l7.95-3.974a2 2 0 0 1 1.837 0l7.948 3.974A2 2 0 0 1 22 8z"/><path d="M6 13h12"/><path d="M6 17h12"/>"#);

/// `washing-machine`.
pub const WASHING_MACHINE: Icon = lucide!("washing-machine", r#"<path d="M3 6h3"/><path d="M17 6h.01"/><rect width="18" height="20" x="3" y="2" rx="2"/><circle cx="12" cy="13" r="5"/><path d="M12 18a2.5 2.5 0 0 0 0-5 2.5 2.5 0 0 1 0-5"/>"#);

/// `watch`.
pub const WATCH: Icon = lucide!("watch", r#"<path d="M12 10v2.2l1.6 1"/><path d="m16.13 7.66-.81-4.05a2 2 0 0 0-2-1.61h-2.68a2 2 0 0 0-2 1.61l-.78 4.05"/><path d="m7.88 16.36.8 4a2 2 0 0 0 2 1.61h2.72a2 2 0 0 0 2-1.61l.81-4.05"/><circle cx="12" cy="12" r="6"/>"#);

/// `waves-arrow-down`.
pub const WAVES_ARROW_DOWN: Icon = lucide!("waves-arrow-down", r#"<path d="M12 10L12 2"/><path d="M16 6L12 10L8 6"/><path d="M2 15C2.6 15.5 3.2 16 4.5 16C7 16 7 14 9.5 14C12.1 14 11.9 16 14.5 16C17 16 17 14 19.5 14C20.8 14 21.4 14.5 22 15"/><path d="M2 21C2.6 21.5 3.2 22 4.5 22C7 22 7 20 9.5 20C12.1 20 11.9 22 14.5 22C17 22 17 20 19.5 20C20.8 20 21.4 20.5 22 21"/>"#);

/// `waves-arrow-up`.
pub const WAVES_ARROW_UP: Icon = lucide!("waves-arrow-up", r#"<path d="M12 2v8"/><path d="M2 15c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="M2 21c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="m8 6 4-4 4 4"/>"#);

/// `waves-horizontal`.
pub const WAVES_HORIZONTAL: Icon = lucide!("waves-horizontal", r#"<path d="M2 12q2.5 2 5 0t5 0 5 0 5 0"/><path d="M2 19q2.5 2 5 0t5 0 5 0 5 0"/><path d="M2 5q2.5 2 5 0t5 0 5 0 5 0"/>"#);

/// `waves-ladder`.
pub const WAVES_LADDER: Icon = lucide!("waves-ladder", r#"<path d="M19 5a2 2 0 0 0-2 2v11"/><path d="M2 18c.6.5 1.2 1 2.5 1 2.5 0 2.5-2 5-2 2.6 0 2.4 2 5 2 2.5 0 2.5-2 5-2 1.3 0 1.9.5 2.5 1"/><path d="M7 13h10"/><path d="M7 9h10"/><path d="M9 5a2 2 0 0 0-2 2v11"/>"#);

/// `waves-vertical`.
pub const WAVES_VERTICAL: Icon = lucide!("waves-vertical", r#"<path d="M12 2q2 2.5 0 5t0 5 0 5 0 5"/><path d="M19 2q2 2.5 0 5t0 5 0 5 0 5"/><path d="M5 2q2 2.5 0 5t0 5 0 5 0 5"/>"#);

/// `waypoints`.
pub const WAYPOINTS: Icon = lucide!("waypoints", r#"<path d="m10.586 5.414-5.172 5.172"/><path d="m18.586 13.414-5.172 5.172"/><path d="M6 12h12"/><circle cx="12" cy="20" r="2"/><circle cx="12" cy="4" r="2"/><circle cx="20" cy="12" r="2"/><circle cx="4" cy="12" r="2"/>"#);

/// `webcam`.
pub const WEBCAM: Icon = lucide!("webcam", r#"<circle cx="12" cy="10" r="8"/><circle cx="12" cy="10" r="3"/><path d="M7 22h10"/><path d="M12 22v-4"/>"#);

/// `webcam-off`.
pub const WEBCAM_OFF: Icon = lucide!("webcam-off", r#"<path d="M12 22v-4"/><path d="M12.754 7.096a3 3 0 0 1 2.15 2.15"/><path d="M12.863 12.873a3 3 0 0 1-3.736-3.735"/><path d="M16.566 16.57A8 8 0 0 1 5.43 5.433"/><path d="m2 2 20 20"/><path d="M7 22h10"/><path d="M8.478 2.817a8 8 0 0 1 10.705 10.705"/>"#);

/// `webhook`.
pub const WEBHOOK: Icon = lucide!("webhook", r#"<path d="M18 16.98h-5.99c-1.1 0-1.95.94-2.48 1.9A4 4 0 0 1 2 17c.01-.7.2-1.4.57-2"/><path d="m6 17 3.13-5.78c.53-.97.1-2.18-.5-3.1a4 4 0 1 1 6.89-4.06"/><path d="m12 6 3.13 5.73C15.66 12.7 16.9 13 18 13a4 4 0 0 1 0 8"/>"#);

/// `webhook-off`.
pub const WEBHOOK_OFF: Icon = lucide!("webhook-off", r#"<path d="M17 17h-5c-1.09-.02-1.94.92-2.5 1.9A3 3 0 1 1 2.57 15"/><path d="M9 3.4a4 4 0 0 1 6.52.66"/><path d="m6 17 3.1-5.8a2.5 2.5 0 0 0 .057-2.05"/><path d="M20.3 20.3a4 4 0 0 1-2.3.7"/><path d="M18.6 13a4 4 0 0 1 3.357 3.414"/><path d="m12 6 .6 1"/><path d="m2 2 20 20"/>"#);

/// `weight`.
pub const WEIGHT: Icon = lucide!("weight", r#"<circle cx="12" cy="5" r="3"/><path d="M6.5 8a2 2 0 0 0-1.905 1.46L2.1 18.5A2 2 0 0 0 4 21h16a2 2 0 0 0 1.925-2.54L19.4 9.5A2 2 0 0 0 17.48 8Z"/>"#);

/// `weight-tilde`.
pub const WEIGHT_TILDE: Icon = lucide!("weight-tilde", r#"<path d="M6.5 8a2 2 0 0 0-1.906 1.46L2.1 18.5A2 2 0 0 0 4 21h16a2 2 0 0 0 1.925-2.54L19.4 9.5A2 2 0 0 0 17.48 8z"/><path d="M7.999 15a2.5 2.5 0 0 1 4 0 2.5 2.5 0 0 0 4 0"/><circle cx="12" cy="5" r="3"/>"#);

/// `wheat`.
pub const WHEAT: Icon = lucide!("wheat", r#"<path d="M2 22 16 8"/><path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"/><path d="M7.47 8.53 9 7l1.53 1.53a3.5 3.5 0 0 1 0 4.94L9 15l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"/><path d="M11.47 4.53 13 3l1.53 1.53a3.5 3.5 0 0 1 0 4.94L13 11l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"/><path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"/><path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"/><path d="M15.47 13.47 17 15l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"/><path d="M19.47 9.47 21 11l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L13 11l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"/>"#);

/// `wheat-off`.
pub const WHEAT_OFF: Icon = lucide!("wheat-off", r#"<path d="m2 22 10-10"/><path d="m16 8-1.17 1.17"/><path d="M3.47 12.53 5 11l1.53 1.53a3.5 3.5 0 0 1 0 4.94L5 19l-1.53-1.53a3.5 3.5 0 0 1 0-4.94Z"/><path d="m8 8-.53.53a3.5 3.5 0 0 0 0 4.94L9 15l1.53-1.53c.55-.55.88-1.25.98-1.97"/><path d="M10.91 5.26c.15-.26.34-.51.56-.73L13 3l1.53 1.53a3.5 3.5 0 0 1 .28 4.62"/><path d="M20 2h2v2a4 4 0 0 1-4 4h-2V6a4 4 0 0 1 4-4Z"/><path d="M11.47 17.47 13 19l-1.53 1.53a3.5 3.5 0 0 1-4.94 0L5 19l1.53-1.53a3.5 3.5 0 0 1 4.94 0Z"/><path d="m16 16-.53.53a3.5 3.5 0 0 1-4.94 0L9 15l1.53-1.53a3.49 3.49 0 0 1 1.97-.98"/><path d="M18.74 13.09c.26-.15.51-.34.73-.56L21 11l-1.53-1.53a3.5 3.5 0 0 0-4.62-.28"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `whistle`.
pub const WHISTLE: Icon = lucide!("whistle", r#"<path d="M10 6v4"/><path d="M21 6a1 1 0 0 1 1 1v2a1 1 0 0 1-1 1h-5.675A7 7 0 1 1 9 6z"/>"#);

/// `whole-word`.
pub const WHOLE_WORD: Icon = lucide!("whole-word", r#"<circle cx="7" cy="12" r="3"/><path d="M10 9v6"/><circle cx="17" cy="12" r="3"/><path d="M14 7v8"/><path d="M22 17v1c0 .5-.5 1-1 1H3c-.5 0-1-.5-1-1v-1"/>"#);

/// `wifi`.
pub const WIFI: Icon = lucide!("wifi", r#"<path d="M12 20h.01"/><path d="M2 8.82a15 15 0 0 1 20 0"/><path d="M5 12.859a10 10 0 0 1 14 0"/><path d="M8.5 16.429a5 5 0 0 1 7 0"/>"#);

/// `wifi-cog`.
pub const WIFI_COG: Icon = lucide!("wifi-cog", r#"<path d="m14.305 19.53.923-.382"/><path d="m15.228 16.852-.923-.383"/><path d="m16.852 15.228-.383-.923"/><path d="m16.852 20.772-.383.924"/><path d="m19.148 15.228.383-.923"/><path d="m19.53 21.696-.382-.924"/><path d="M2 8.82a15 15 0 0120 0"/><path d="m20.772 16.852.924-.383"/><path d="m20.772 19.148.924.383"/><path d="M5 12.859a10 10 0 0110.18-2.342"/><path d="M8.5 16.429a5 5 0 011.998-1.2"/><circle cx="18" cy="18" r="3"/>"#);

/// `wifi-high`.
pub const WIFI_HIGH: Icon = lucide!("wifi-high", r#"<path d="M12 20h.01"/><path d="M5 12.859a10 10 0 0 1 14 0"/><path d="M8.5 16.429a5 5 0 0 1 7 0"/>"#);

/// `wifi-lock`.
pub const WIFI_LOCK: Icon = lucide!("wifi-lock", r#"<path d="M2 8.82a15 15 0 0120 0"/><path d="M20 16v-2a2 2 0 00-4 0v2"/><path d="M5 12.859a10 10 0 018.436-2.756"/><path d="M8.5 16.429a5 5 0 011.794-1.13"/><rect x="14" y="16" width="8" height="5" rx="1"/>"#);

/// `wifi-low`.
pub const WIFI_LOW: Icon = lucide!("wifi-low", r#"<path d="M12 20h.01"/><path d="M8.5 16.429a5 5 0 0 1 7 0"/>"#);

/// `wifi-off`.
pub const WIFI_OFF: Icon = lucide!("wifi-off", r#"<path d="M12 20h.01"/><path d="M8.5 16.429a5 5 0 0 1 7 0"/><path d="M5 12.859a10 10 0 0 1 5.17-2.69"/><path d="M19 12.859a10 10 0 0 0-2.007-1.523"/><path d="M2 8.82a15 15 0 0 1 4.177-2.643"/><path d="M22 8.82a15 15 0 0 0-11.288-3.764"/><path d="m2 2 20 20"/>"#);

/// `wifi-pen`.
pub const WIFI_PEN: Icon = lucide!("wifi-pen", r#"<path d="M2 8.82a15 15 0 0 1 20 0"/><path d="M21.378 16.626a1 1 0 0 0-3.004-3.004l-4.01 4.012a2 2 0 0 0-.506.854l-.837 2.87a.5.5 0 0 0 .62.62l2.87-.837a2 2 0 0 0 .854-.506z"/><path d="M5 12.859a10 10 0 0 1 10.5-2.222"/><path d="M8.5 16.429a5 5 0 0 1 3-1.406"/>"#);

/// `wifi-sync`.
pub const WIFI_SYNC: Icon = lucide!("wifi-sync", r#"<path d="M11.965 10.105v4L13.5 12.5a5 5 0 0 1 8 1.5"/><path d="M11.965 14.105h4"/><path d="M17.965 18.105h4L20.43 19.71a5 5 0 0 1-8-1.5"/><path d="M2 8.82a15 15 0 0 1 20 0"/><path d="M21.965 22.105v-4"/><path d="M5 12.86a10 10 0 0 1 3-2.032"/><path d="M8.5 16.429h.01"/>"#);

/// `wifi-zero`.
pub const WIFI_ZERO: Icon = lucide!("wifi-zero", r#"<path d="M12 20h.01"/>"#);

/// `wind`.
pub const WIND: Icon = lucide!("wind", r#"<path d="M12.8 19.6A2 2 0 1 0 14 16H2"/><path d="M17.5 8a2.5 2.5 0 1 1 2 4H2"/><path d="M9.8 4.4A2 2 0 1 1 11 8H2"/>"#);

/// `wind-arrow-down`.
pub const WIND_ARROW_DOWN: Icon = lucide!("wind-arrow-down", r#"<path d="M10 2v8"/><path d="M12.8 21.6A2 2 0 1 0 14 18H2"/><path d="M17.5 10a2.5 2.5 0 1 1 2 4H2"/><path d="m6 6 4 4 4-4"/>"#);

/// `wind-arrow-up`.
pub const WIND_ARROW_UP: Icon = lucide!("wind-arrow-up", r#"<path d="M10 2v8"/><path d="M12.8 21.6A2 2 0 1 0 14 18H2"/><path d="M17.5 10a2.5 2.5 0 1 1 2 4H2"/><path d="m6 6 4 -4 4 4"/>"#);

/// `wine`.
pub const WINE: Icon = lucide!("wine", r#"<path d="M8 22h8"/><path d="M7 10h10"/><path d="M12 15v7"/><path d="M12 15a5 5 0 0 0 5-5c0-2-.5-4-2-8H9c-1.5 4-2 6-2 8a5 5 0 0 0 5 5Z"/>"#);

/// `wine-off`.
pub const WINE_OFF: Icon = lucide!("wine-off", r#"<path d="M8 22h8"/><path d="M7 10h3m7 0h-1.343"/><path d="M12 15v7"/><path d="M7.307 7.307A12.33 12.33 0 0 0 7 10a5 5 0 0 0 7.391 4.391M8.638 2.981C8.75 2.668 8.872 2.34 9 2h6c1.5 4 2 6 2 8 0 .407-.05.809-.145 1.198"/><line x1="2" x2="22" y1="2" y2="22"/>"#);

/// `workflow`.
pub const WORKFLOW: Icon = lucide!("workflow", r#"<rect width="8" height="8" x="3" y="3" rx="2"/><path d="M7 11v4a2 2 0 0 0 2 2h4"/><rect width="8" height="8" x="13" y="13" rx="2"/>"#);

/// `worm`.
pub const WORM: Icon = lucide!("worm", r#"<path d="m19 12-1.5 3"/><path d="M19.63 18.81 22 20"/><path d="M6.47 8.23a1.68 1.68 0 0 1 2.44 1.93l-.64 2.08a6.76 6.76 0 0 0 10.16 7.67l.42-.27a1 1 0 1 0-2.73-4.21l-.42.27a1.76 1.76 0 0 1-2.63-1.99l.64-2.08A6.66 6.66 0 0 0 3.94 3.9l-.7.4a1 1 0 1 0 2.55 4.34z"/>"#);

/// `wrench`.
pub const WRENCH: Icon = lucide!("wrench", r#"<path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.106-3.105c.32-.322.863-.22.983.218a6 6 0 0 1-8.259 7.057l-7.91 7.91a1 1 0 0 1-2.999-3l7.91-7.91a6 6 0 0 1 7.057-8.259c.438.12.54.662.219.984z"/>"#);

/// `wrench-off`.
pub const WRENCH_OFF: Icon = lucide!("wrench-off", r#"<path d="M10.747 5.093a6 6 0 0 1 6.841-2.882c.438.12.54.662.219.984L14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.106-3.105c.32-.322.863-.22.983.218a6 6 0 0 1-2.882 6.842"/><path d="m13.5 13.5-7.88 7.88a1 1 0 0 1-2.999-3l7.88-7.88"/><path d="m2 2 20 20"/>"#);

/// `x`.
pub const X: Icon = lucide!("x", r#"<path d="M18 6 6 18"/><path d="m6 6 12 12"/>"#);

/// `x-line-top`.
pub const X_LINE_TOP: Icon = lucide!("x-line-top", r#"<path d="M18 4H6"/><path d="M18 8 6 20"/><path d="m6 8 12 12"/>"#);

/// `zap`.
pub const ZAP: Icon = lucide!("zap", r#"<path d="M15.914 4a1.5 1.5 0 00-2.474-1.561l-9 9A1.5 1.5 0 005.5 14h4.002a.5.5 0 01.471.666L8.086 20a1.5 1.5 0 002.475 1.56l9-9A1.5 1.5 0 0018.5 10h-3.997a.5.5 0 01-.472-.667z"/>"#);

/// `zap-off`.
pub const ZAP_OFF: Icon = lucide!("zap-off", r#"<path d="M10.768 5.111 13.44 2.44a1.5 1.5 0 012.474 1.561l-1.633 4.625"/><path d="m18.889 13.232.672-.672A1.5 1.5 0 0018.5 10h-2.844"/><path d="m2 2 20 20"/><path d="m7.94 7.94-3.5 3.499A1.5 1.5 0 005.5 14h4.002a.5.5 0 01.471.666L8.086 20a1.5 1.5 0 002.475 1.56l5.5-5.5"/>"#);

/// `zodiac-aquarius`.
pub const ZODIAC_AQUARIUS: Icon = lucide!("zodiac-aquarius", r#"<path d="m2 10 2.456-3.684a.7.7 0 0 1 1.106-.013l2.39 3.413a.7.7 0 0 0 1.096-.001l2.402-3.432a.7.7 0 0 1 1.098 0l2.402 3.432a.7.7 0 0 0 1.098 0l2.389-3.413a.7.7 0 0 1 1.106.013L22 10"/><path d="m2 18.002 2.456-3.684a.7.7 0 0 1 1.106-.013l2.39 3.413a.7.7 0 0 0 1.097 0l2.402-3.432a.7.7 0 0 1 1.098 0l2.402 3.432a.7.7 0 0 0 1.098 0l2.389-3.413a.7.7 0 0 1 1.106.013L22 18.002"/>"#);

/// `zodiac-aries`.
pub const ZODIAC_ARIES: Icon = lucide!("zodiac-aries", r#"<path d="M12 7.5a4.5 4.5 0 1 1 5 4.5"/><path d="M7 12a4.5 4.5 0 1 1 5-4.5V21"/>"#);

/// `zodiac-cancer`.
pub const ZODIAC_CANCER: Icon = lucide!("zodiac-cancer", r#"<path d="M21 14.5A9 6.5 0 0 1 5.5 19"/><path d="M3 9.5A9 6.5 0 0 1 18.5 5"/><circle cx="17.5" cy="14.5" r="3.5"/><circle cx="6.5" cy="9.5" r="3.5"/>"#);

/// `zodiac-capricorn`.
pub const ZODIAC_CAPRICORN: Icon = lucide!("zodiac-capricorn", r#"<path d="M11 21a3 3 0 0 0 3-3V6.5a1 1 0 0 0-7 0"/><path d="M7 19V6a3 3 0 0 0-3-3h0"/><circle cx="17" cy="17" r="3"/>"#);

/// `zodiac-gemini`.
pub const ZODIAC_GEMINI: Icon = lucide!("zodiac-gemini", r#"<path d="M16 4.525v14.948"/><path d="M20 3A17 17 0 0 1 4 3"/><path d="M4 21a17 17 0 0 1 16 0"/><path d="M8 4.525v14.948"/>"#);

/// `zodiac-leo`.
pub const ZODIAC_LEO: Icon = lucide!("zodiac-leo", r#"<path d="M10 16c0-4-3-4.5-3-8a5 5 0 0 1 10 0c0 3.466-3 6.196-3 10a3 3 0 0 0 6 0"/><circle cx="7" cy="16" r="3"/>"#);

/// `zodiac-libra`.
pub const ZODIAC_LIBRA: Icon = lucide!("zodiac-libra", r#"<path d="M3 16h6.857c.162-.012.19-.323.038-.38a6 6 0 1 1 4.212 0c-.153.057-.125.368.038.38H21"/><path d="M3 20h18"/>"#);

/// `zodiac-ophiuchus`.
pub const ZODIAC_OPHIUCHUS: Icon = lucide!("zodiac-ophiuchus", r#"<path d="M3 10A6.06 6.06 0 0 1 12 10 A6.06 6.06 0 0 0 21 10"/><path d="M6 3v12a6 6 0 0 0 12 0V3"/>"#);

/// `zodiac-pisces`.
pub const ZODIAC_PISCES: Icon = lucide!("zodiac-pisces", r#"<path d="M19 21a15 15 0 0 1 0-18"/><path d="M20 12H4"/><path d="M5 3a15 15 0 0 1 0 18"/>"#);

/// `zodiac-sagittarius`.
pub const ZODIAC_SAGITTARIUS: Icon = lucide!("zodiac-sagittarius", r#"<path d="M15 3h6v6"/><path d="M21 3 3 21"/><path d="m9 9 6 6"/>"#);

/// `zodiac-scorpio`.
pub const ZODIAC_SCORPIO: Icon = lucide!("zodiac-scorpio", r#"<path d="M10 19V5.5a1 1 0 0 1 5 0V17a2 2 0 0 0 2 2h5l-3-3"/><path d="m22 19-3 3"/><path d="M5 19V5.5a1 1 0 0 1 5 0"/><path d="M5 5.5A2.5 2.5 0 0 0 2.5 3"/>"#);

/// `zodiac-taurus`.
pub const ZODIAC_TAURUS: Icon = lucide!("zodiac-taurus", r#"<circle cx="12" cy="15" r="6"/><path d="M18 3A6 6 0 0 1 6 3"/>"#);

/// `zodiac-virgo`.
pub const ZODIAC_VIRGO: Icon = lucide!("zodiac-virgo", r#"<path d="M11 5.5a1 1 0 0 1 5 0V16a5 5 0 0 0 5 5"/><path d="M16 11.5a1 1 0 0 1 5 0V16a5 5 0 0 1-5 5"/><path d="M6 19V6a3 3 0 0 0-3-3h0"/><path d="M6 5.5a1 1 0 0 1 5 0V19"/>"#);

/// `zoom-in`.
pub const ZOOM_IN: Icon = lucide!("zoom-in", r#"<circle cx="11" cy="11" r="8"/><line x1="21" x2="16.65" y1="21" y2="16.65"/><line x1="11" x2="11" y1="8" y2="14"/><line x1="8" x2="14" y1="11" y2="11"/>"#);

/// `zoom-out`.
pub const ZOOM_OUT: Icon = lucide!("zoom-out", r#"<circle cx="11" cy="11" r="8"/><line x1="21" x2="16.65" y1="21" y2="16.65"/><line x1="8" x2="14" y1="11" y2="11"/>"#);
