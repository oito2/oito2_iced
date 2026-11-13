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

//! Helpers for the tests of the OITO2 Iced crates: simulator settings with
//! the embedded fonts, the pixels of a snapshot, and golden images compared
//! with a tolerance.
//!
//! Run the tests with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1` and look at every changed image before
//! accepting it.

#![deny(missing_docs)]

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use iced::Settings;
use iced_test::simulator::Snapshot;
use oito2_iced::font;

/// Fraction of pixels allowed to differ from a golden image.
pub const TOLERANCE: f64 = 0.005;

/// Largest per-channel difference that still counts as an equal pixel.
pub const CHANNEL_THRESHOLD: u8 = 2;

/// Simulator settings with Inter as the default font.
pub fn settings() -> Settings {
    Settings {
        fonts: font::INTER_FONTS.into_iter().map(Into::into).collect(),
        default_font: font::INTER,
        ..Settings::default()
    }
}

/// An RGBA image.
pub struct Image {
    /// Width, in physical pixels.
    pub width: u32,
    /// Height, in physical pixels.
    pub height: u32,
    /// Pixels, row by row, four bytes per pixel.
    pub rgba: Vec<u8>,
    /// File name written by the simulator: `<name>-<renderer>.png`.
    pub file_name: String,
}

/// Reads the pixels of a snapshot. The simulator keeps them private, so the
/// snapshot is written as a PNG into an empty temporary directory (which
/// `matches_image` does when the file is missing) and read back.
pub fn pixels(snapshot: &Snapshot, name: &str) -> Image {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let directory = std::env::temp_dir()
        .join("oito2_iced_test_utils")
        .join(format!(
            "snapshot-{}-{}",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("create the snapshot directory");

    let image = written(snapshot, &directory, name);
    let _ = fs::remove_dir_all(&directory);

    image
}

/// Writes `snapshot` into the empty `directory` and reads the file back.
fn written(snapshot: &Snapshot, directory: &Path, name: &str) -> Image {
    snapshot
        .matches_image(directory.join(format!("{name}.png")))
        .expect("write the snapshot");

    let entry = fs::read_dir(directory)
        .expect("read the snapshot directory")
        .next()
        .expect("the snapshot was written")
        .expect("read the snapshot entry");

    let file_name = entry.file_name().to_string_lossy().into_owned();
    let image = read_png(&entry.path());

    Image { file_name, ..image }
}

/// Reads an RGBA PNG.
pub fn read_png(path: &Path) -> Image {
    let file = fs::File::open(path).expect("open the PNG");
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().expect("read the PNG header");
    let mut rgba = vec![0; reader.output_buffer_size().expect("PNG fits in memory")];
    let info = reader.next_frame(&mut rgba).expect("decode the PNG");
    rgba.truncate(info.buffer_size());

    Image {
        width: info.width,
        height: info.height,
        rgba,
        file_name: String::new(),
    }
}

/// Writes an RGBA PNG, creating its directory.
pub fn write_png(path: &Path, image: &Image) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create the golden directory");
    }

    let file = fs::File::create(path).expect("create the PNG");
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), image.width, image.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("write the PNG header");
    writer
        .write_image_data(&image.rgba)
        .expect("write the PNG data");
}

/// Fraction of pixels whose channels differ by more than
/// [`CHANNEL_THRESHOLD`]; 1.0 when the sizes differ.
pub fn difference(a: &Image, b: &Image) -> f64 {
    if a.width != b.width || a.height != b.height {
        return 1.0;
    }

    let differing = a
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(b.rgba.as_chunks::<4>().0)
        .filter(|(pa, pb)| {
            pa.iter()
                .zip(pb.iter())
                .any(|(ca, cb)| ca.abs_diff(*cb) > CHANNEL_THRESHOLD)
        })
        .count();

    differing as f64 / (a.width as f64 * a.height as f64)
}

/// Compares a snapshot with `<directory>/<name>-<renderer>.png`, allowing
/// [`TOLERANCE`] of differing pixels. With `OITO2_UPDATE_GOLDENS=1` the
/// golden is (re)written instead; a missing golden fails the test. On a
/// mismatch the actual image is written next to the golden as
/// `<name>-<renderer>.actual.png`.
///
/// The [`golden!`] macro calls it with the `tests/goldens` directory of the
/// calling crate.
pub fn assert_golden_in(directory: &Path, snapshot: &Snapshot, name: &str) {
    let actual = pixels(snapshot, name);
    let golden = directory.join(&actual.file_name);

    if std::env::var("OITO2_UPDATE_GOLDENS").as_deref() == Ok("1") {
        write_png(&golden, &actual);
        return;
    }

    assert!(
        golden.exists(),
        "missing golden {}; run with OITO2_UPDATE_GOLDENS=1 to create it",
        golden.display()
    );

    let expected = read_png(&golden);
    let difference = difference(&actual, &expected);

    let actual_path = golden.with_extension("actual.png");

    if difference > TOLERANCE {
        write_png(&actual_path, &actual);
    }

    assert!(
        difference <= TOLERANCE,
        "{} differs from its golden in {:.3} % of the pixels (tolerance {:.3} %); actual image: {}",
        actual.file_name,
        difference * 100.0,
        TOLERANCE * 100.0,
        actual_path.display()
    );
}

/// Compares a snapshot with its golden image in the `tests/goldens`
/// directory of the calling crate. See [`assert_golden_in`].
#[macro_export]
macro_rules! golden {
    ($snapshot:expr, $name:expr) => {
        $crate::assert_golden_in(
            &::std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens"),
            $snapshot,
            $name,
        )
    };
}

/// The color of the pixel at `point`, given in logical pixels of a window
/// `logical_width` wide; the image may be rendered at a higher scale.
pub fn pixel_at(image: &Image, point: iced::Point, logical_width: f32) -> iced::Color {
    let scale = image.width as f32 / logical_width;
    let (x, y) = ((point.x * scale) as u32, (point.y * scale) as u32);
    let index = ((y * image.width + x) * 4) as usize;
    let [r, g, b, a] = [0, 1, 2, 3].map(|offset| image.rgba[index + offset]);

    iced::Color::from_rgba8(r, g, b, f32::from(a) / 255.0)
}

/// The mouse cursor `element` asks for while the pointer rests at `point`,
/// laid out in a window of `size`, with the renderer the simulator uses.
///
/// The fonts of [`settings`] are loaded by the first simulator of the test
/// process; build one before calling this so text is measured with them.
pub fn mouse_interaction<'a, Message>(
    element: impl Into<iced::Element<'a, Message, oito2_iced::Theme>>,
    size: impl Into<iced::Size>,
    point: iced::Point,
) -> iced::mouse::Interaction {
    use iced::advanced::renderer::Headless;
    use iced_test::runtime::user_interface::{self, State, UserInterface};

    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let mut renderer = iced_test::futures::futures::executor::block_on(
        <iced::Renderer as Headless>::new(font::INTER, iced::Pixels(16.0), backend.as_deref()),
    )
    .expect("headless renderer");

    let mut ui = UserInterface::build(
        element,
        size.into(),
        user_interface::Cache::default(),
        &mut renderer,
    );

    let cursor = iced::mouse::Cursor::Available(point);
    let mut messages = Vec::new();
    let (state, _) = ui.update(
        &[iced::Event::Mouse(iced::mouse::Event::CursorMoved {
            position: point,
        })],
        cursor,
        &mut renderer,
        &mut iced::advanced::clipboard::Null,
        &mut messages,
    );

    match state {
        State::Updated {
            mouse_interaction, ..
        } => mouse_interaction,
        State::Outdated => iced::mouse::Interaction::None,
    }
}
