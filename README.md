[![Build Status](https://travis-ci.org/foresterre/sic.svg?branch=master)](https://travis-ci.org/foresterre/sic)

# sic image cli

Enables you to perform operations on images and convert images to a variety of image formats.
'sic' or 'sic image cli' is a (partial) image crate cli front-end.

The image conversion and operations are performed by the awesome [image](https://crates.io/crates/image) crate  :balloon:.
It was initially created to try out another awesome Rust library:  [clap](https://crates.io/crates/clap) :tada:.


# Install

Install with [cargo](https://crates.io/crates/sic): `cargo install sic`<br>
Update with [cargo](https://crates.io/crates/sic): `cargo install --force sic`

Pre build binary: see [releases](https://github.com/foresterre/sic/releases).

From the source:
- Setup rust and cargo with (for example) [rustup](https://rustup.rs/) <br> 
  _Rust version >= 1.31 with 'Rust edition 2018' is required._
- Clone this repo: `git clone https://github.com/foresterre/sic.git`
- Switch to this repo: `cd sic`
- Build a release: `cargo build --release`


# Usage

**Convert an image from one format to another, for example from PNG to JPG.**
* In general: `sic <input> <output>`
* Example: `sic input.png output.jpg`

<br>

**Covert an image from one format to another while not caring about the output file extension.**
* In general `sic --output-format "<format>" <input> <output>` (or  `sic -f "<format>" <input> <output>`)
* Example `sic --output-format png input.bmp output.jpg` _(Note: `output.jpg` will have the PNG format even though the extension is `jpg`.)_

Supported image output formats are (as of 0.8.0): `bmp`, `gif`, `ico`, `jpg` (or `jpeg`), `png`, `pbm`, `pgm`, `ppm` and `pam`.
The JPEG quality can optionally be set with `--jpeg-encoding-quality <value>` (value should be an integer from 1 up to (including) 100). Default value if not user overridden is 80.
PNM (PBM, PGM, PPM) by default uses binary encoding (PNM P4, P5 and P6 respectively). To use ascii encoding, provide the following flag:
`--pnm-encoding-ascii`.

<br>

**Apply image operations to an image.**
* In general: `sic --apply-operations "<operations>" <input> <output> ` (shorthand: `-x` or `-A`) 
* Example `sic -i input.png -o output.jpg --apply-operations "fliph; blur 10; resize 250 250"`
Alternatively from release 0.10.0, you can also use image operations as cli arguments. The example above then becomes:
* `sic -i input.png -o output.jpg --flip-horizontal --blur 10 --resize 250 250`
* Note that image operation cli arguments can not be combined with --apply-operations.

When more than one image operation is provided, the separator `;` should be used 
between each operation.

The available image operations are:

|operations|syntax|available (from version)|description|
|---|---|---|---|
|blur               | `blur <fp>`                           | Yes (0.5.0) 	    | Performs a Gaussian blur on the image ([more info](https://docs.rs/image/0.19.0/image/imageops/fn.blur.html)). An argument below `0.0`, will use `1.0` instead. |
|brighten           | `brighten <int>`                      | Yes (0.7.0) 	    | |
|contrast           | `contrast <fp>`                       | Yes (0.7.0) 	    | |
|crop               | `crop <int> <int> <int> <int>`        | Yes (0.9.0)       | Syntax: `crop <lx> <ly> <rx> <ry>`, where `lx` is top left corner x pixel coordinate starting at 0, `ly` is the top left corner y pixel coordinate starting at 0, `rx` is the  bottom right corner x pixel coordinate and `ry` is the bottom right corner y pixel coordinate. `rx` and `ry` should be larger than `lx` and `ly` respectively. |
|filter3x3          | `filter3x3 <args9>`                   | Yes (0.7.0)       | |
|flip horizontal    | `fliph`                               | Yes (0.5.0) 	    | Flips the image on the horizontal axis. |
|flip vertical      | `flipv`                               | Yes (0.5.0) 	    | Flips the image on the vertical axis. |
|gray scale         | `grayscale`                           | Yes (0.7.0) 	    | |
|hue rotate         | `huerotate <int>`                     | Yes (0.7.0) 	    | Rotate's the hue, argument is in degrees. Rotates `<int>%360` degrees. |
|invert             | `invert`                              | Yes (0.7.0) 	    | |
|resize             | `resize <uint> <uint>`                | Yes (0.5.0) 	    | Resize the image to x by y pixels. Can both up- and downscale. Uses a gaussian sampling filter if no override value is set. |
| >                 | `set resize preserve_aspect_ratio`    | Yes (0.9.0)       | Enables preservation of the aspect ratio when resizing. |
| >                 | `set resize sampling_filter <value>`  | Yes (0.9.0)       | When resizing use the `<value>` sampling filter. Choices are `catmullrom`, `gaussian`,`lanczos3`,`nearest`,`triangle`. |
|rotate90           | `rotate90`                            | Yes (0.7.0) 	    | |
|rotate180          | `rotate180`                           | Yes (0.7.0) 	    | |
|rotate270          | `rotate270`                           | Yes (0.7.0) 	    | |
|unsharpen          | `unsharpen <fp> <int>`                | Yes (0.7.0) 	    | |

For some operations, their behaviour can be (slightly) changed by choosing and enabling an option. These options can be overwritten and they can also be disabled.

|environment operation|syntax|available (from version)|description|
|---|---|---|---|
|set environment option   | `set <operation> <option-of-operation> [<args 0..n>]` | Yes (0.9.0) | Enables the use of an option by an operation. Any operation which looks at this option will use the selected option value instead of the default value. Can be overwritten by calling `set` again for the same operation and option. |
|unset environment option | `del <operation> <option-of-operation>`               | Yes (0.9.0) | Disables the use of an option by an operation. Any operation which looks at this option will use the default value instead. |

_legend_:
```
<uint> means any 32 bit unsigned integer is required as argument.
<int> means any 32 bit signed integer is required as argument.
<fp> means any 32 bit floating point number is required as argument.
<value> means a pre defined value. 
<args9> means `<fp> <fp> <fp> <fp> <fp> <fp> <fp> <fp> <fp>`.
```

_Image operation example usage:_

**blur** example:

`sic -i in.png -o out.png --apply-operations "blur 1.3;"`
or
`sic -i in.png -o out.png --blur 1.3`

**brighten** example:
`sic -i in.png -o out.png --apply-operations "brighten 2;"`
or
`sic -i in.png -o out.png --brighten 2`

**contrast** example:
`sic -i in.png -o out.png --apply-operations "contrast 0.7;"`
or
`sic -i in.png -o out.png --contrast 0.7`


**crop** example:
`sic -i in.png -o out.png --apply-operations "crop 0 0 10 10;"`
or
`sic -i in.png -o out.png --crop 0 0 10 10`


**filter3x3** example:
`sic -i in.png -o out.png --apply-operations "filter3x3 1.0 1.0 1.0 0 0 0 0.5 0.5 0.5"`
or
`sic -i in.png -o out.png --filter3x3 1.0 1.0 1.0 0 0 0 0.5 0.5 0.5`


**flip horizontal** example:
`sic -i in.png -o out.png --apply-operations "fliph"`
or
`sic -i in.png -o out.png --flip-horizontal`


**flip vertical** example:
`sic -i in.png -o out.png --apply-operations "flipv"`
or
`sic -i in.png -o out.png --flip-vertical`


**gray scale** example:
`sic -i in.png -o out.png --apply-operations "grayscale"`
or
`sic -i in.png -o out.png --grayscale`


**hue rotate** example:
`sic -i in.png -o out.png --apply-operations "huerotate -90"`
or
`sic -i in.png -o out.png --hue-rotate -90`

**invert** example:
`sic -i in.png -o out.png --apply-operations "invert"`
or
`sic -i in.png -o out.png --invert`

**resize** example:
`sic -i in.png -o out.png --apply-operations "resize 100 100"`
or
`sic -i in.png -o out.png --resize 100 100`

**resize** with **preserve aspect ratio** example:
`sic -i in.png -o out.png --apply-operations "set resize preserve_aspect_ratio; resize 100 100"`
or
`sic -i in.png -o out.png --set-resize-preserve-aspect-ratio --resize 100 100`

**resize** with **custom sampling filter** (default is 'gaussian') example:
`sic -i in.png -o out.png --apply-operations "set resize sampling_filter triangle; resize 100 100"`
or
`sic -i in.png -o out.png --set-resize-sampling-filter triangle --resize 100 100`

**rotate 90 degree** example:
`sic -i in.png -o out.png --apply-operations "rotate90"`
or
`sic -i in.png -o out.png --rotate90`

**rotate 180 degree** example:
`sic -i in.png -o out.png --apply-operations "rotate180"`
or
`sic -i in.png -o out.png --rotate180`

**rotate 270 degree** example:
`sic -i in.png -o out.png --apply-operations "rotate270"`
or
`sic -i in.png -o out.png --rotate270`

**unsharpen** example:
`sic -i in.png -o out.png --apply-operations "unsharpen -0.7 1"`
or
`sic -i in.png -o out.png --unsharpen -0.7 1`

example with multiple image operations combined:
`sic -i in.png -o out.png --apply-operations "rotate180; fliph; set resize sampling_filter nearest; resize 75 80 huerotate 75"`
or
`sic -i in.png -o out.png --rotate180 --flip-horizontal --set-resize-sampling-filter nearest --resize 75 80 --hue-rotate 75`


<br>

**User manual**

For additional information on available options and flags, run `sic --help`.
Additional information on the available image operations can be found by running `sic --user-manual <topic>` (or `sic -H <topic>`).
Available topics can be listed by running `sic --user-manual index`.

The provided help pages in this command line accessible user manual are still a bit minimal. Additionally only the image operations
are available and the layout is sub optimal for a command line. This is definitely something which is planned to be addressed in an upcoming release.   


# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:
