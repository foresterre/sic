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
* Command: `sic --input <input> --output <output>`
* Shorthand: `sic -i <input> -o <output>`
* Example: `sic -i input.png -o output.jpg` <br>

Previously `sic <input> <output>` was used to specify the input and output image files. This method of specifying
input and output image paths is still supported, if and only if no other input and output option ( such as
'--input' and '--output') is  used. Using the '--input' and '--output' arguments is however the preferred way to specify
input and output image file paths.

<br>

**Covert an image from one format to another while not caring about the output file extension.**
* In general `sic --output-format "<format>" -i <input> -o <output>` (or  `sic -f "<format>" -i <input> -o <output>`)
* Example `sic --output-format png -i input.bmp -o output.jpg` _(Note: `output.jpg` will have the PNG format even though the extension is `jpg`.)_

Supported image output formats are (as of 0.8.0): `bmp`, `gif`, `ico`, `jpg` (or `jpeg`), `png`, `pbm`, `pgm`, `ppm` and `pam`.
The JPEG quality can optionally be set with `--jpeg-encoding-quality <value>` (value should be an integer from 1 up to (including) 100).
Default value if not user overridden is 80.
The PNM format (specifically PBM, PGM and PPM) use binary encoding (PNM P4, P5 and P6 respectively) by default.
To use ascii encoding, provide the following flag: `--pnm-encoding-ascii`.

<br>

**Apply image operations to an image.**
As of release 0.10.0, there are two methods to apply image operations on an image.
The first method is by using the `--apply-operations "<operations>"` (shorthand: `-x` or `-A`) cli argument and providing
statements which tell `sic` what operations should be applied on the image, for example: <br>
`sic -i input.jpg -o output.jpg --apply-operations "fliph; blur 10; resize 250 250"` <br>
When more than one image operation is provided, the separator `;` should be used
to separate each operation statement. <br>
Any version of the program prior to 0.10.0 is limited to this method of applying image operations.


From release 0.10.0 forward, there is a second method which can be used. This method uses cli arguments to inform
`sic`, what image operations should be applied in what order. Do note that the order in which these arguments are provided
 *does* (not in every case though =D) matter.

If we use the _image operations as cli arguments_ method the previously shown example becomes: <br>
`sic -i input.png -o output.jpg --flip-horizontal --blur 10 --resize 250 250` <br>
Note that image operation cli arguments can not be combined with --apply-operations. <br>

The image operations are applied left-to-right for both methods. Additionally the methods can not be used both at the same
time. Either the _--apply-operations_ method or the _image operations as cli arguments_ method should be used.


The available image operations are:

|operations|syntax*|available (from version)|description|
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

`* The exact syntax applies to the --apply-operations method, but can also be used as a reference for the image operations as cli arguments method.`


For some operations, their behaviour can be (slightly) changed by setting an operation modifier. These modifiers can be overwritten and they can also be reset (to their default behaviour).

|environment operation|syntax|available (from version)|description|
|---|---|---|---|
|set environment option   | `set <operation> <option-of-operation> [<args 0..n>]` | Yes (0.9.0) | Enables the use of a modifier for an operation. Any operation which uses the value of the modifier will use the set modifier value instead of the default value. Can be overwritten by calling `set` again for the same operation and modifier specifier. |
|unset environment option | `del <operation> <option-of-operation>`               | Yes (0.9.0) | Resets the modifier value. Any operation which looks at the value of this modifier will use the default value instead.|

_legend_:
```
<uint> means any 32 bit unsigned integer is required as argument.
<int> means any 32 bit signed integer is required as argument.
<fp> means any 32 bit floating point number is required as argument.
<value> means a pre defined value. 
<args9> means `<fp> <fp> <fp> <fp> <fp> <fp> <fp> <fp> <fp>`.
```

_Image operation example usage:_

**blur** example: <br>
`sic -i in.png -o out.png --apply-operations "blur 1.3;"` <br>
or <br>
`sic -i in.png -o out.png --blur 1.3`

**brighten** example: <br>
`sic -i in.png -o out.png --apply-operations "brighten 2;"` <br>
or <br>
`sic -i in.png -o out.png --brighten 2`

**contrast** example: <br>
`sic -i in.png -o out.png --apply-operations "contrast 0.7;"` <br>
or <br>
`sic -i in.png -o out.png --contrast 0.7`


**crop** example: <br>
`sic -i in.png -o out.png --apply-operations "crop 0 0 10 10;"` <br>
or <br>
`sic -i in.png -o out.png --crop 0 0 10 10`


**filter3x3** example: <br>
`sic -i in.png -o out.png --apply-operations "filter3x3 1.0 1.0 1.0 0 0 0 0.5 0.5 0.5"` <br>
or <br>
`sic -i in.png -o out.png --filter3x3 1.0 1.0 1.0 0 0 0 0.5 0.5 0.5`


**flip horizontal** example: <br>
`sic -i in.png -o out.png --apply-operations "fliph"` <br>
or <br>
`sic -i in.png -o out.png --flip-horizontal`


**flip vertical** example: <br>
`sic -i in.png -o out.png --apply-operations "flipv"` <br>
or <br>
`sic -i in.png -o out.png --flip-vertical`


**gray scale** example: <br>
`sic -i in.png -o out.png --apply-operations "grayscale"` <br>
or <br>
`sic -i in.png -o out.png --grayscale`


**hue rotate** example: <br>
`sic -i in.png -o out.png --apply-operations "huerotate -90"` <br>
or <br>
`sic -i in.png -o out.png --hue-rotate -90`

**invert** example: <br>
`sic -i in.png -o out.png --apply-operations "invert"` <br>
or <br>
`sic -i in.png -o out.png --invert`

**resize** example: <br>
`sic -i in.png -o out.png --apply-operations "resize 100 100"` <br>
or <br>
`sic -i in.png -o out.png --resize 100 100`

**resize** with **preserve aspect ratio** example: <br>
`sic -i in.png -o out.png --apply-operations "set resize preserve_aspect_ratio; resize 100 100"` <br>
or <br>
`sic -i in.png -o out.png --set-resize-preserve-aspect-ratio true --resize 100 100`

**resize** with **custom sampling filter** (default is 'gaussian') example: <br>
`sic -i in.png -o out.png --apply-operations "set resize sampling_filter triangle; resize 100 100"` <br>
or <br>
`sic -i in.png -o out.png --set-resize-sampling-filter triangle --resize 100 100`

**rotate 90 degree** example: <br>
`sic -i in.png -o out.png --apply-operations "rotate90"` <br>
or <br>
`sic -i in.png -o out.png --rotate90`

**rotate 180 degree** example: <br>
`sic -i in.png -o out.png --apply-operations "rotate180"` <br>
or <br>
`sic -i in.png -o out.png --rotate180`

**rotate 270 degree** example: <br>
`sic -i in.png -o out.png --apply-operations "rotate270"` <br>
or <br>
`sic -i in.png -o out.png --rotate270`

**unsharpen** example: <br>
`sic -i in.png -o out.png --apply-operations "unsharpen -0.7 1"` <br>
or <br>
`sic -i in.png -o out.png --unsharpen -0.7 1`

example with *multiple* image operations which are applied from left-to-right: <br>
`sic -i in.png -o out.png --apply-operations "rotate180; fliph; set resize sampling_filter nearest; resize 75 80; huerotate 75"` <br>
or <br>
`sic -i in.png -o out.png --rotate180 --flip-horizontal --set-resize-sampling-filter nearest --resize 75 80 --hue-rotate 75`


<br>

**Other resources on image operations**

For additional information on available options and flags, run `sic --help`.

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:
