# Changelog

sic is a CLI tool to (batch) process images and an [image-rs](https://github.com/image-rs/image) front-end.
It can be used to convert between image formats and manipulate images using image operations. 
It aims to include all primary [image](https://github.com/image-rs/image) functionality, and eventually also
support the most prominent [imageproc](https://github.com/image-rs/imageproc) manipulation routines.

The changelog below lists notable changes for [sic](https://github.com/foresterre/sic). It doesn't list most internal changes.

## [Unreleased]

### Added

- Added horizontal gradient image operation
- Added automatic thresholding using the Otsu method 

## [0.18.0] - 2021-03-09

### Added

- Option to set repeat value for animated GIF encoded images

### Fixed

- Automatic color type adjustment setting was ignored

## [0.17.0] - 2021-02-20

### Added

- Added animated images support: it is now possible to load and save all frames, and apply operations on all frames, of animated images

### Changed

- Animated images no longer load the first frame as static image (by default)
- Frame index selection is now zero-indexed instead of one-indexed

## [0.16.1] - 2021-02-13

### Added

- Added AVIF support to documentation

## [0.16.0] - 2021-02-13

### Added

- Decoding support for AVIF

## [0.15.0] - 2020-12-25

### Added

- Encoding support for TGA
- Encoding support for AVIF

## [0.14.0] - 2020-08-07

### Added

- Image script: add `overlay` operation which can be used to draw one image over another
- `--select-frame` now supports images encoded as APNG
- `--no-skip-unsupported-extensions` CLI flag to enumerate all files when using glob based input; not just files with supported extensions


### Changed

- When using glob paths, `--glob-input` and `--glob-output` should now be used instead of `--input` and `--output` combined with `--mode glob`
- Glob based input now skips unsupported files by default (disable with `--no-skip-unsupported-extensions`)

### Removed

- Removed CLI option `--mode` (use `--glob-input` and `--glob-output` instead)

### Fixed

- Glob input paths starting with "./" or ".\" should now work
- 

## [0.12.0] - 2020-06-01

### Added

- Option to load image script from a file
- Image script: add `draw-text` image operation

### Changed

- Renamed image script operation `fliph` to `flip-horizontal`
- Renamed image script operation `flipv` to `flip-vertical`
- Renamed image script operation `huerotate` to `hue-rotate`
- Renamed image script modifier `preserve_aspect_ratio` to `preserve-aspect-ratio`
- Renamed image script modifier `sampling_filter` to `sampling-filter`
- Changed CLI flag  `--set-preserve-aspect-ratio` to `--preserve-aspect-ratio`
- Changed CLI flag  ` --set-resize-sampling-filter` to `--sampling-filter`


### Fixed

- Folders are now skipped in `glob` mode

## [0.11.0] - 2020-05-06

### Added

- Image script: add `diff` image operation which highlights the differences between images
- Encoding and decoding support for Farbfeld
- Batch process a set of images with glob pattern matching on file inputs (requires option `--mode glob` to be set)
- Better error handling

### Changed

- Set default sampling filter for image resizing to Lanczos3

### Removed

- INPUT_FILE and OUTPUT_FILE positional arguments (use `--input` and `--output` instead!)

## [0.10.1]

### Fixed

- Updated embedded dependency licenses

## [0.10.0] - 2019-09-28 [yanked]

### Added

- CLI interface to use image script operations directly as CLI options and flags
- Option to select specific frames of animated GIFs
- Script to generate shell completions (internal)
- Support reading from stdin and writing to stdout
- Support input and output file paths by setting the `--input` (`-i`) and `--output` (`-o`) directives respectively

### Deprecated

- INPUT_FILE and OUTPUT_FILE positional arguments (to be removed)

### Removed

- Embedded user manual

### Fixed

- The image script blur command took an unsigned integer as argument but it should have been a floating point number

## [0.9.0] - 2019-06-08

### Added

- Image script: add global options table
- Image script: add `set resize keep_aspect_ratio` option
- Image script: add `set resize sampling_filter <value>` option
- Image script: add `crop` image operation
- Add '-x' as shorthand for, and set '-A' to be an alias for '--apply-operations'

### Changed

- ⚠ Require the `;` separator between image operations
- ⚠ Renamed option --force-format to --output-format
- ⚠ Renamed --script to --apply-operations

## [0.8.1] - 2018-12-11

### Changed

- Switched from nightly Rust toolchain to stable (internal)

## [0.8.0] - 2018-11-30 

### Added

- Option to set JPEG quality parameter
- Option to set PNM encoding type
- Option to set PNM subtype
- Option to automatically adjust color type for output format 
- Updated [image](https://github.com/image-rs/image) to v0.20 (internal)

## [0.7.0] - 2018-08-21

### Added

- Image script: add `brighten` image operation
- Image script: add `contrast` image operation
- Image script: add `filter3x3` image operation
- Image script: add `grayscale` image operation
- Image script: add `huerotate` image operation
- Image script: add `invert` image operation
- Image script: add `rotate90` image operation
- Image script: add `rotate180` image operation
- Image script: add `rotate270` image operation
- Image script: add `unsharpen` image operation
- Embedded user manual


## [0.6.0] - 2018-08-10

### Added

- Embedded license in cli

## [0.5.0] - 2018-08-10 [yanked]

### Added

- Image operations which can be applied by providing commands to the `--script` cli option
- Image script: add `blur` image operation
- Image script: add `flip horizontal` image operation
- Image script: add `flip vertical` image operation
- Image script: add `flip resize` image operation

_yanked: `--version` not updated_

## [0.4.0] - 2018-07-03

### Added

- Updated [image](https://github.com/image-rs/image) from v0.17 to v0.19 (internal)

## [0.2.0] - 2018-06-03

### Added

- Determine output format based on file extension

## [0.1.0] - 2017-11-12

### Added

- CLI for converting between image formats, powered by [image](https://github.com/image-rs/image)
- Decoding support for PNG, JPEG, GIF, BMP, ICO, TIFF and WebP (not all formats are completely supported)
- Encoding support for JPEG, PNG, GIF, ICO, PPM

[unreleased]: https://github.com/foresterre/sic/compare/v0.18.0...HEAD
[0.18.0]: https://github.com/foresterre/sic/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/foresterre/sic/compare/v0.16.1...v0.17.0
[0.16.1]: https://github.com/foresterre/sic/compare/v0.16.0...v0.16.1
[0.16.0]: https://github.com/foresterre/sic/compare/v0.15.0...v0.16.0
[0.15.0]: https://github.com/foresterre/sic/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/foresterre/sic/compare/v0.12.0...v0.14.0
[0.12.0]: https://github.com/foresterre/sic/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/foresterre/sic/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/foresterre/sic/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/foresterre/sic/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/foresterre/sic/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/foresterre/sic/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/foresterre/sic/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/foresterre/sic/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/foresterre/sic/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/foresterre/sic/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/foresterre/sic/compare/v0.2.0...v0.4.0
[0.2.0]: https://github.com/foresterre/sic/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/foresterre/sic/releases/tag/v0.1.0
