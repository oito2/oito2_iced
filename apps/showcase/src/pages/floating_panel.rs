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

//! The FloatingPanel page: panels that move, resize, minimize and
//! maximize inside an area of the application.

use iced::widget::{column, container, row, stack, text};
use iced::{Element, Length, Point, Rectangle, Size, Task};

use oito2_iced::Theme;
use oito2_iced::foundation::WindowControlStyle;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::floating_panel::{PanelState, floating_panel};
use oito2_iced::window::Labels;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The id of the title bar of the floating panel.
pub const PANEL_BAR: &str = "showcase-floating-panel-bar";

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("floating_panel.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    inspector: Option<PanelState>,
    layers: Option<PanelState>,
    palette: PanelState,
    position: PanelState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            inspector: Some(restored(30.0, 30.0, 320.0, 220.0)),
            layers: Some(restored(400.0, 120.0, 240.0, 240.0)),
            palette: restored(24.0, 24.0, 260.0, 150.0),
            position: restored(24.0, 24.0, 240.0, 150.0),
        }
    }
}

/// A restored panel at `x`, `y` of `width` by `height`.
fn restored(x: f32, y: f32, width: f32, height: f32) -> PanelState {
    PanelState::new(Rectangle::new(Point::new(x, y), Size::new(width, height)))
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The inspector moved, resized, minimized or maximized.
    Inspector(PanelState),
    /// The inspector opened or closed.
    InspectorOpen(bool),
    /// The layers panel moved, resized, minimized or maximized.
    Layers(PanelState),
    /// The layers panel opened or closed.
    LayersOpen(bool),
    /// The fixed palette moved, minimized or maximized.
    Palette(PanelState),
    /// The tracked panel moved, resized, minimized or maximized.
    Position(PanelState),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Inspector(panel) => state.inspector = Some(panel),
        Message::InspectorOpen(open) => {
            state.inspector = open.then(|| restored(30.0, 30.0, 320.0, 220.0));
        }
        // #endregion
        Message::Layers(panel) => state.layers = Some(panel),
        Message::LayersOpen(open) => {
            state.layers = open.then(|| restored(400.0, 120.0, 240.0, 240.0));
        }
        Message::Palette(panel) => state.palette = panel,
        Message::Position(panel) => state.position = panel,
    }

    Task::none()
}

/// The bordered area of `height` the panels of an example float over.
fn area<'a>(
    height: f32,
    layers: impl IntoIterator<Item = Element<'a, Message, Theme>>,
) -> Element<'a, Message, Theme> {
    border_container(
        container(stack(layers).width(Length::Fill).height(Length::Fill))
            .class(ContainerClass::Background)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fixed(height))
    .clip(true)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let reopen = row![]
        .push(
            state
                .inspector
                .is_none()
                .then(|| button::outlined("Open Inspector").on_press(Message::InspectorOpen(true))),
        )
        .push(
            state
                .layers
                .is_none()
                .then(|| button::outlined("Open Layers").on_press(Message::LayersOpen(true))),
        )
        .spacing(spacing::SM);

    let mut layers: Vec<Element<'a, Message, Theme>> =
        vec![container(reopen).center(Length::Fill).into()];
    if let Some(panel) = state.layers {
        layers.push(
            floating_panel(
                panel,
                "Layers",
                column(
                    ["Text", "Shape", "Background"]
                        .into_iter()
                        .map(|layer| text(layer).into()),
                )
                .spacing(spacing::SM)
                .padding(spacing::MD),
            )
            .on_change(Message::Layers)
            .on_close(Message::LayersOpen(false))
            .labels(Labels::new(l10n))
            .id(PANEL_BAR)
            .into(),
        );
    }
    if let Some(panel) = state.inspector {
        layers.push(
            // #region preview
            floating_panel(
                panel,
                "Inspector",
                container(text("Width: 320\nHeight: 240\nOpacity: 100%")).padding(spacing::MD),
            )
            .on_change(Message::Inspector)
            .on_close(Message::InspectorOpen(false))
            .labels(Labels::new(l10n))
            // #endregion
            .id(PANEL_BAR)
            .into(),
        );
    }
    let preview = doc::example(ctx, SOURCE, &["preview", "update"], area(420.0, layers));

    let fixed = doc::example(
        ctx,
        SOURCE,
        &["fixed"],
        area(
            240.0,
            [
                // #region fixed
                floating_panel(
                    state.palette,
                    "Palette",
                    container("Fixed size").center(Length::Fill),
                )
                .resizable(false)
                .control_style(WindowControlStyle::Breeze)
                .on_change(Message::Palette)
                .labels(Labels::new(l10n))
                // #endregion
                .id(PANEL_BAR)
                .into(),
            ],
        ),
    );

    let rect = state.position.rect;
    let tracked = doc::example(
        ctx,
        SOURCE,
        &["rect"],
        area(
            240.0,
            [
                // #region rect
                floating_panel(
                    state.position,
                    "Position",
                    container(
                        text(format!(
                            "{}, {}\n{} × {}",
                            rect.x.round(),
                            rect.y.round(),
                            rect.width.round(),
                            rect.height.round()
                        ))
                        .center(),
                    )
                    .center(Length::Fill),
                )
                .on_change(Message::Position)
                .labels(Labels::new(l10n))
                // #endregion
                .id(PANEL_BAR)
                .into(),
            ],
        ),
    );

    doc::page(
        "FloatingPanel",
        "A non-modal window inside the app, such as an inspector or a tool palette, that the \
         user moves, resizes, minimizes and maximizes.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Place `floating_panel` in a layer of a `stack` over the content: the panel \
                 fills its layer to know its bounds, but takes pointer input only on itself. \
                 Give it a `PanelState`, the rectangle in the coordinates of the area and the \
                 minimized and maximized flags, a title and its content.",
            ),
            doc::prose(
                "The state lives in the application: `on_change` publishes the next \
                 `PanelState` on every move, resize, minimize or maximize, including each \
                 step of a drag, and the application stores it. Without `on_change` the \
                 panel stays still. Dragging the title bar moves the panel, which keeps its \
                 title bar inside the area, and a double click maximizes or restores it. Its \
                 right and bottom edges and its corner resize it, down to `min_size` (220 by \
                 140 by default). Minimizing leaves only the title bar, and the content keeps \
                 its state. The close button shows only with `on_close`.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::floating_panel::{PanelState, floating_panel};",
            ),
            doc::note(
                "Panels keep the order of the layers of their stack: a click on a panel does \
                 not bring it to the front, so reorder the layers when you want that.",
            ),
            doc::prose(
                "`labels` gives the title bar buttons the strings of the current language; \
                 they are in English by default.",
            ),
            doc::heading("Examples"),
            doc::subheading("Fixed size and control style"),
            doc::prose(
                "With `resizable(false)` the edges no longer resize the panel. \
                 `control_style` draws the title bar buttons in the look of a desktop, GNOME \
                 by default.",
            ),
            fixed,
            doc::subheading("Tracking the rectangle"),
            doc::prose(
                "Since the application keeps the `PanelState`, it always knows the rectangle, \
                 for example to save the layout.",
            ),
            tracked,
            doc::heading("Keyboard"),
            doc::prose(
                "The title bar takes the focus, and from there the keyboard moves and resizes \
                 the panel by `keyboard_step` pixels, 16 by default.",
            ),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "Arrow keys",
                        "Move the panel, while the title bar has focus.",
                    ],
                    vec![
                        "`Shift` + arrow keys",
                        "Resize the panel, while the title bar has focus.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::floating_panel::floating_panel",
                    "oito2_iced::widget::floating_panel::FloatingPanel",
                    "oito2_iced::widget::floating_panel::PanelState",
                    "oito2_iced::widget::floating_panel::MIN_SIZE",
                    "oito2_iced::widget::floating_panel::KEYBOARD_STEP",
                ],
            ),
        ],
    )
}
