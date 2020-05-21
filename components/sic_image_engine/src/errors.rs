use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicImageEngineError {
    #[error("unable to crop; required top-left anchor < bottom-right anchor; note that (x=0,y=0) is the smallest top-left coordinate; [top-left anchor: (x={0}, y={1}), bottom-right anchor: (x={2}, y={3})]")]
    CropInvalidSelection(u32, u32, u32, u32),

    #[error("unable to crop; anchor coordinates should be within image bounds [image size: (x={0}, y={1}), top-left anchor: (x={2}, y={3}), bottom-right anchor: (x={4}, y={5})]")]
    CropCoordinateOutOfBounds(u32, u32, u32, u32, u32, u32),

    // FIXME 2020-02-08: Should capture the SicIoError like so:
    //      LoadImageFromPath(sic_io::errors::SicIoError),
    //      This however results in tons of "binary operation `!=` cannot be applied to type `sic_image_engine::errors::SicImageEngineError`"
    //      errors, because in the sic_parsing tests we have assert_eq! tests against Result instead
    //      of unwrapped results. Since this would be quite an effort and since these tests have to
    //      be rewritten to make use of parameterized anyways, I left this as a fix me for later.
    //      NOTE: Should also remove the derive(PartialEq, Eq) at that point!
    #[error("unable to load image argument from given path")]
    LoadImageFromPath,

    #[error("filter type '{0}' not found")]
    UnknownFilterType(String),

    #[cfg(feature = "imageproc-ops")]
    #[error("unable to load font: invalid format")]
    FontError,

    #[cfg(feature = "imageproc-ops")]
    #[error("unable to open font file from path: '{0}'")]
    FontFileLoadError(std::io::Error),
}
