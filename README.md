# Simple Image Converter (sic)

Converts a single image from one format to another _(plus some other operations)_.

The image conversion is actually done by the awesome [image](https://crates.io/crates/image) crate  :balloon:.
`sic` is a small command line frontend which supports a portion of the conversion operations supported by the __image__ crate.

It was initially created to try out another awesome Rust library:  [clap](https://crates.io/crates/clap) :tada:


# Install

With [cargo](https://crates.io/crates/sic) install: `cargo install --force sic`

Pre build binary: see [releases](https://github.com/foresterre/sic/releases)

From the source of this repo:
- Setup rust and cargo with (for example) [rustup](https://rustup.rs/), a `nightly` version is required.
- Clone this repo: `git clone https://github.com/foresterre/sic.git`
- Switch to this repo: `cd sic`
- Build a release: `cargo build --release`


# Usage

**Convert an image from one format to another, for example from PNG to JPG.**
* In general: `sic <input> <output>`
* Example: `sic input.png output.jpg`

**Covert an image from one format to another while not caring about the output file extension.**
* In general `sic --force-format "<format>" <input> <output>` (or  `sic --force-format "<format>" <input> <output>`)
* Example `sic --force-format png input.bmp output.jpg` _(Note: `output.jpg` will have the PNG format even though the extension is `jpg`.)_

_Note: supported forced formats currently are: bmp, gif, ico, jpg (always 80%), pnm (P6 only). Support for other versions of supported formats is planned._

**Apply image operations to an image.**
* In general: `sic --script "<operations>" <input> <output> `
* Example `sic input.png output.jpg --script "flip_horizontal; blur 10; resize 250 250"`

The separator `;` within the image operation script is optional. It exists to provide clarity.  

_Note: `resize` applies a gaussian sampling filter on resizing. This is currently the only sampling filter available (i.e. not changeable, all resize operations will be done with the gaussian sampling filter)._

Image operations availability:


|operations|syntax|available (from version)|description|
|---|---|---|---|
|blur               | `blur <uint>` [E-BLUR]                | Yes (0.5.0) 	    | Performs a Gaussian blur on the image ([more info](https://docs.rs/image/0.19.0/image/imageops/fn.blur.html)) |
|brighten           | `brighten <int>` [E-BRIGHTEN]         | Yes (0.7.0) 	    | |
|hue rotate         | `huerotate <int>` [E-HUEROTATE]       | Yes (0.7.0) 	    | Rotate's the hue, argument is in degrees. Rotates `<int>%360` degrees. |
|contrast           | `contrast <fp>` [E-CONTRAST]          | Yes (0.7.0) 	    | |
|crop               |                                       | No                | You can use `resize <uint> <uint>`` with values smaller or equal to the current image size for now. |
|filter3x3          | `filter3x3 <args9>` [E-FILTER3X3]     | Yes (0.7.0)       | |
|flip horizontal    | `fliph` [E-FLIPH]                     | Yes (0.5.0) 	    | Flips the image on the horizontal axis |
|flip vertical      | `flipv` [E-FLIPV]                     | Yes (0.5.0) 	    | Flips the image on the horizontal axis |
|gray scale         | `grayscale` [E-GRAYSCALE]             | Yes (0.7.0) 	    | |
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
<args9> means `<fp> <fp> <fp> | <fp> <fp> <fp> | <fp> <fp> <fp>` where the `|` separator is optional. If the separator is used, white space should surround the separator. The separators can only be used like in the example, so one separator after each of the first two triplets.
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

[E-HUEROTATE]. Hue rotate operation example script:
```
huerotate 10;
huerotate -10;
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

# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:
