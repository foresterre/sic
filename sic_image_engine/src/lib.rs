#[macro_use]
extern crate strum_macros;

pub mod engine;
pub mod wrapper;

#[derive(Debug, PartialEq, Clone)]
pub enum ImgOp {
    Blur(f32),
    Brighten(i32),
    Contrast(f32),
    Crop((u32, u32, u32, u32)),
    Filter3x3([f32; 9]),
    FlipHorizontal,
    FlipVertical,
    GrayScale,
    HueRotate(i32),
    Invert,
    Resize((u32, u32)),
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen((f32, i32)),
}
