# Changelog

All notable changes to `oito2_iced_files` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `pick` opens the file chooser of the platform (the XDG desktop portal on Linux) as an Iced task.
- `drop_zone`: a dashed zone that opens the chooser on a click, Enter or Space and receives the files dropped on the window (not under Wayland, where the windowing library reports no drops).
- `upload`: a controller that checks files against accepted types, a size limit and a largest number of files, and `file_upload`, which shows the zone, the refusals and the accepted files with image previews.
- `download`: `save` writes bytes through the "Save as" dialog, and `download_trigger` is the button that starts it.
- `File` and `TypeGroup` describe the files and the accepted types.
