/// The enumeration of all supported operations.
#[derive(
    Debug, Copy, Clone, Hash, Eq, PartialEq, AsStaticStr, EnumIter, EnumString, EnumVariantNames,
)]
#[strum(serialize_all = "kebab_case")]
pub enum OperationId {
    // image operations
    Blur,
    Brighten,
    Contrast,
    Crop,
    Diff,

    #[cfg(feature = "imageproc-ops")]
    DrawText,

    Filter3x3,
    FlipHorizontal,
    FlipVertical,
    Grayscale,
    HueRotate,
    Invert,
    Overlay,
    Resize,
    Rotate90,
    Rotate180,
    Rotate270,
    Unsharpen,

    // modifiers
    PreserveAspectRatio,
    SamplingFilter,
}