# Features & Implementation

[ ] Accept different versions of supported formats

For example, PNM supports PBM,PGM,PPM, standard PAM decoding modes.
Idea's for implementing:
- If just specifying a extension, use a default format (as decided by `image::save()`).
- If `-f` option is used use as `-f [format] [mode :optional]`
    - `format`: ...
    - `mode`:
        - `PNM` => (example) `PPM`
            -  usage example `sic input.png output -f PNM PPM`
            - match on `-f <arg>`, `<arg>.asList` `List(format: &str, " ", mode: &str)
        - `JPEG` ?
            - `baseline` xor `progressive` OR `number` xor ``
            - usage example `sic input.png output -f JPEG progressive 90`
[ ] Improve error handling by defining an Error type. Possibly use the 'failure' crate.

[ ] Add 'macro language' for image processing functions support

- Command line syntax? ideas:
    - `sic in out.ext --script "invert; rotate180; flip_vertical"`
    - `sic in out.ext --script-from-file example.sic`
    - `sic in out -f PNG --map "invert; rotate180"`

# Tests:

[ ] Add tests with images
[ ] Test properly

# Known bugs

[ ] if an image can't be encoded, at this moment, an empty file will still be created

