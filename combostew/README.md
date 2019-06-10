# combostew

Combostew is an intermediate layer between the 'image' crate used as back-end by image tool cli front-ends
[sic](https://github.com/foresterre/sic) and [stew](https://github.com/foresterre/stew).

<br>

_crates.io: [Combostew](https://crates.io/crates/combostew)_

# Components

_Some components could be split to separate crates, perhaps in a workspace, later on._

* Import / export image
* Image operations engine
    * Operations supported:
        * [x] `blur` `[u32]`
        * [x] `brighten` `[i32]`
        * [x] `contrast` `[f32]`
        * [x] `convert`
        * [x] `crop` `[u32] [u32] [u32] [u32]`
        * [x] `filter3x3` `[f32] [f32] [f32] [f32] [f32] [f32] [f32] [f32] [f32]`
        * [x] `fliph`
        * [x] `flipv`
        * [x] `grayscale`
        * [x] `huerotate` `[i32]`
        * [x] `invert`
        * [x] `resize` `[u32] [u32]`
        * [x] `rotate90`
        * [x] `rotate180`
        * [x] `rotate270`
        * [x] `unsharpen` `[f32] [i32]`
    * Ability to set options or flags
* Display of licenses of (third party) components used _(will be moved)_


# Suggestions, Questions, Bugs

Feel free to open an issue :mailbox_with_mail: if you have a suggestion, a question or found a bug =).

:guitar: :trumpet: :violin: :saxophone:
