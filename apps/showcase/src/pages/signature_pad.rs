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

//! The SignaturePad page: drawing, ink options, outside actions and the
//! SVG export.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::button;
use oito2_iced_extras::signature_pad::{self, Signature, signature_pad};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("signature_pad.rs");

/// Which pad of the page an event is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// The pad of the preview.
    Preview,
    /// The pad with its own ink.
    Ink,
    /// The pad with outside actions.
    Actions,
    /// The pad that exports its strokes.
    Export,
}

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    preview: Signature,
    ink: Signature,
    actions: Signature,
    export: Signature,
    svg: Option<String>,
}

impl State {
    /// The signature of the pad `which`.
    fn signature(&mut self, which: Which) -> &mut Signature {
        match which {
            Which::Preview => &mut self.preview,
            Which::Ink => &mut self.ink,
            Which::Actions => &mut self.actions,
            Which::Export => &mut self.export,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A pad changed its strokes.
    Pad(Which, signature_pad::Event),
    /// Exports the strokes of the export pad as SVG.
    ShowSvg,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pad(which, event) => state.signature(which).apply(event),
        // #region export-update
        Message::ShowSvg => state.svg = Some(state.export.to_svg(None, iced::Color::BLACK)),
        // #endregion
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let width = Length::Fixed(400.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            signature_pad(&state.preview)
                .on_event(|event| Message::Pad(Which::Preview, event))
                .localizations(l10n),
            // #endregion
        )
        .width(width),
    );

    let ink = doc::example(
        ctx,
        SOURCE,
        &["ink"],
        container(
            // #region ink
            signature_pad(&state.ink)
                .height(140.0)
                .color(Accent::Blue.color())
                .min_width(2.0)
                .max_width(6.0)
                .placeholder("Draw here")
                .on_event(|event| Message::Pad(Which::Ink, event))
                .localizations(l10n),
            // #endregion
        )
        .width(width),
    );

    let undo = (!state.actions.is_empty())
        .then_some(Message::Pad(Which::Actions, signature_pad::Event::Undo));
    let clear = (!state.actions.is_empty())
        .then_some(Message::Pad(Which::Actions, signature_pad::Event::Clear));

    let actions = doc::example(
        ctx,
        SOURCE,
        &["actions"],
        // #region actions
        column![
            signature_pad(&state.actions)
                .show_actions(false)
                .on_event(|event| Message::Pad(Which::Actions, event))
                .localizations(l10n),
            row![
                button::outlined("Undo").on_press_maybe(undo),
                button::outlined("Clear").on_press_maybe(clear),
            ]
            .spacing(spacing::SM),
        ]
        .spacing(spacing::SM)
        .align_x(iced::Alignment::End)
        .width(width),
        // #endregion
    );

    let svg = state.svg.as_ref().map(|svg| {
        let start: String = svg.chars().take(160).collect();
        text(format!("{} characters: {start}…", svg.len())).class(TextClass::Variant)
    });

    let export = doc::example(
        ctx,
        SOURCE,
        &["export", "export-update"],
        // #region export
        column![
            signature_pad(&state.export)
                .on_event(|event| Message::Pad(Which::Export, event))
                .localizations(l10n),
            button::filled("Show as SVG").on_press(Message::ShowSvg),
        ]
        .push(svg)
        .spacing(spacing::SM)
        .width(width),
        // #endregion
    );

    doc::page(
        "SignaturePad",
        "An area to sign with a mouse, a finger or a pen, with undo, clear and export to SVG \
         or PNG.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The strokes live in the application, in a `Signature`: `signature_pad` shows \
                 it and publishes `signature_pad::Event`s through `on_event`, which \
                 `Signature::apply` applies; without `on_event` the pad only shows the \
                 strokes. `is_empty` says whether anything was drawn, `undo` removes the last \
                 stroke, `clear` removes them all and `add_stroke` puts back a saved one.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_extras::signature_pad::{self, Signature, signature_pad};",
            ),
            doc::prose(
                "The pad takes the available width and `height` pixels, 180 by default. Only \
                 the first pointer down draws, and a mouse draws with its left button only. \
                 The empty pad shows the localized \"Sign here\".",
            ),
            doc::heading("Examples"),
            doc::subheading("Ink and stroke width"),
            doc::prose(
                "Strokes get thinner as the pointer moves faster, between `min_width` and \
                 `max_width`, like ink. `color` sets the ink, the theme's text color by \
                 default, and `placeholder` replaces the \"Sign here\" text of the empty pad.",
            ),
            ink,
            doc::subheading("Own actions"),
            doc::prose(
                "With `show_actions(false)` the undo and clear buttons in the corner go away, \
                 and the application applies `Event::Undo` and `Event::Clear` itself, enabling \
                 its buttons only when there are strokes.",
            ),
            actions,
            doc::subheading("Export"),
            doc::prose(
                "`to_svg` returns an SVG document of the pad size, or of the size given, one \
                 path per stroke segment so the width can vary, in the opaque RGB value of the \
                 color. `to_image` renders a transparent `image::RgbaImage` at a pixel ratio, \
                 and `to_png` encodes it as PNG, returning `None` when encoding fails.",
            ),
            export,
            doc::note(
                "Drawing needs a pointer, so offer another way to sign, such as typing the \
                 name, where that matters.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_extras::signature_pad::signature_pad",
                    "oito2_iced_extras::signature_pad::SignaturePad",
                    "oito2_iced_extras::signature_pad::Signature",
                    "oito2_iced_extras::signature_pad::Event",
                    "oito2_iced_extras::signature_pad::StrokePoint",
                    "oito2_iced_extras::signature_pad::DEFAULT_HEIGHT",
                ],
            ),
        ],
    )
}
