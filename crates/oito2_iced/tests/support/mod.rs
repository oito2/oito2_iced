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

//! Helpers shared by the simulator tests: settings with the embedded fonts,
//! access to the pixels of a snapshot and a tolerant golden comparison.

#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};
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
        fonts: vec![font::INTER_VARIABLE.into()],
        default_font: font::INTER,
        ..Settings::default()
    }
}

/// An RGBA image.
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    /// File name written by the simulator: `<name>-<renderer>.png`.
    pub file_name: String,
}

/// Reads the pixels of a snapshot. The simulator keeps them private, so the
/// snapshot is written as a PNG into an empty temporary directory (which
/// `matches_image` does when the file is missing) and read back.
pub fn pixels(snapshot: &Snapshot, name: &str) -> Image {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let directory = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "snapshot-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&directory);
    fs::create_dir_all(&directory).expect("create the snapshot directory");

    snapshot
        .matches_image(directory.join(format!("{name}.png")))
        .expect("write the snapshot");

    let entry = fs::read_dir(&directory)
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

fn write_png(path: &Path, image: &Image) {
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

/// Compares a snapshot with `tests/goldens/<file name>`, allowing
/// [`TOLERANCE`] of differing pixels. With `OITO2_UPDATE_GOLDENS=1` the
/// golden is (re)written instead; a missing golden fails the test. On a
/// mismatch the actual image is written next to the golden as
/// `<name>.actual.png`.
pub fn assert_golden(snapshot: &Snapshot, name: &str) {
    let actual = pixels(snapshot, name);
    let golden = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens")
        .join(&actual.file_name);

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
