# Simple Image Converter (sic)

Converts a single image from one format to another _(plus some other operations)_.

The image conversion is actually done by the awesome [image](https://crates.io/crates/image) crate  :balloon:.
`sic` is a small command line frontend which supports a (growing) portion of the conversion operations supported by the __image__ crate.

It was initially created to try out another awesome Rust library:  [clap](https://crates.io/crates/clap) :tada:

<br>

_TravisCI (branch: master):_

[![Build Status](https://travis-ci.org/foresterre/sic.svg?branch=master)](https://travis-ci.org/foresterre/sic)

# Install

With [cargo](https://crates.io/crates/sic) install: `cargo install --force sic`

Pre build binary: see [releases](https://github.com/foresterre/sic/releases)

From the source of this repo:
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
* In general `sic --force-format "<format>" <input> <output>` (or  `sic -f "<format>" <input> <output>`)
* Example `sic --force-format png input.bmp output.jpg` _(Note: `output.jpg` will have the PNG format even though the extension is `jpg`.)_

Supported image output formats are (as of 0.8.0): `bmp`, `gif`, `ico`, `jpg` (or `jpeg`), `png`, `pbm`, `pgm`, `ppm` and `pam`.
The JPEG quality can optionally be set with `--jpeg-encoding-quality <value>` (value should be an integer from 1 up to (including) 100). Default value if not user overridden is 80.
PNM (PBM, PGM, PPM) by default uses binary encoding (PNM P4, P5 and P6 respectively). To use ascii encoding, provide the following flag:
`--pnm-encoding-ascii`. 

<br>

**Apply image operations to an image.**
* In general: `sic --script "<operations>" <input> <output> `
* Example `sic input.png output.jpg --script "fliph; blur 10; resize 250 250"`

The separator `;` within the image operation script is optional. It exists to provide clarity.  

_Note: `resize` applies a gaussian sampling filter on resizing. This is currently the only sampling filter available.
Additional sampling filters are planned for version 0.9.0_

Image operations availability:


|operations|syntax|available (from version)|description|
|---|---|---|---|
|blur               | `blur <uint>` [E-BLUR]                | Yes (0.5.0) 	    | Performs a Gaussian blur on the image ([more info](https://docs.rs/image/0.19.0/image/imageops/fn.blur.html)) |
|brighten           | `brighten <int>` [E-BRIGHTEN]         | Yes (0.7.0) 	    | |
|contrast           | `contrast <fp>` [E-CONTRAST]          | Yes (0.7.0) 	    | |
|crop               |                                       | Yes               | Will be included in release version 0.9.0 |
|filter3x3          | `filter3x3 <args9>` [E-FILTER3X3]     | Yes (0.7.0)       | |
|flip horizontal    | `fliph` [E-FLIPH]                     | Yes (0.5.0) 	    | Flips the image on the horizontal axis |
|flip vertical      | `flipv` [E-FLIPV]                     | Yes (0.5.0) 	    | Flips the image on the horizontal axis |
|gray scale         | `grayscale` [E-GRAYSCALE]             | Yes (0.7.0) 	    | |
|hue rotate         | `huerotate <int>` [E-HUEROTATE]       | Yes (0.7.0) 	    | Rotate's the hue, argument is in degrees. Rotates `<int>%360` degrees. |
|invert             | `invert` [E-INVERT]                   | Yes (0.7.0) 	    | |
|resize             | `resize <uint> <uint>` [E-RESIZE]     | Yes (0.5.0) 	    | Resize the image using a Gaussian sampling filter ([more info](https://docs.rs/image/0.19.0/image/imageops/fn.resize.html), [filter](https://docs.rs/image/0.19.0/image/enum.FilterType.html#variant.Gaussian)) |
|rotate90           | `rotate90` [E-ROTATE90]               | Yes (0.7.0) 	    | |
|rotate180          | `rotate180` [E-ROTATE180]             | Yes (0.7.0) 	    | |
|rotate270          | `rotate270` [E-ROTATE270]             | Yes (0.7.0) 	    | |
|unsharpen          | `unsharpen <fp> <int>` [E-UNSHARPEN]  | Yes (0.7.0) 	    | |


_legend_:
```
<uint> means any 32 bit unsigned integer is required as parameter input.
<int> means any 32 bit signed integer is required as parameter input.
<fp> means any 32 bit floating point number is required as parameter input.
<args9> means `<fp> <fp> <fp> | <fp> <fp> <fp> | <fp> <fp> <fp>` where the `|` separator is optional. If the separator is used, white space should surround the separator. The separators can only be used like in the example, i.e. `triplet | triplet | triplet`.
```

_Syntax examples:_
For each example: each of the lines are valid syntactically and the full examples are valid syntactically as well.

[E-BLUR], Blur operation example script:
```
blur 10;
```

[E-BRIGHTEN], Brighten operation example script:
```
brighten 10;
brighten -10;
```

[E-CONTRAST], Contrast operation example script:
```
contrast -10;
contrast 10;
contrast 1.35;
```

[E-FILTER3X3] Filter3x3 operation example script:
```
filter3x3 10.0 9.0 8.0 | 7.5 6.5 5.5 | 4 3 2;
filter3x3 10.0 9.0 8.0 7.5 6.5 5.5 4 3 2;
filter3x3 10.0 9.0 8.0 7.5 6.5 5.5 4 3 2
filter3x3 10.0 9.0 8.0 7.5 6.5 5.5 4 3 2 filter3x3 12.0 29.0 28 27.5 26 25.5 14 3 2
```

[E-FLIPH]. Flip horizontal operation example script:
```
fliph;
```

[E-FLIPV]. Flip vertical operation example script:
```
flipv;
```

[E-GRAYSCALE]. Gray scale operation example script:
```
grayscale;
```

[E-HUEROTATE]. Hue rotate operation example script:
```
huerotate 10;
huerotate -10;
```

[E-INVERT]. Invert operation example script:
```
invert;
```

[E-RESIZE]. Resize operation example script:
```
resize 10 10;
resize 1 1;
resize 80 180;
```

[E-ROTATE90]. Rotate 90 degree operation example script:
```
rotate90;
```

[E-ROTATE180]. Rotate 180 degree operation example script:
```
rotate180;
```

[E-ROTATE270]. Rotate 270 degree operation example script:
```
rotate270;
```

[E-UNSHARPEN]. Unsharpen operation example script:
```
unsharpen -12.3 -12;
unsharpen -10.0 12;
unsharpen 12.3 1;
unsharpen 10 1;
```

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
