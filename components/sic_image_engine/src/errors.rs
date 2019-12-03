use thiserror::Error;

#[derive(Debug, Error)]
pub enum SicImageEngineError {
    #[error("unable to crop; required top-left anchor < bottom-right anchor; note that (x=0,y=0) is the smallest top-left coordinate; [top-left anchor: (x={0}, y={1}), bottom-right anchor: (x={2}, y={3})]")]
    CropInvalidSelection(u32, u32, u32, u32),

    #[error("unable to crop; anchor coordinates should be within image bounds [image size: (x={0}, y={1}), top-left anchor: (x={2}, y={3}), bottom-right anchor: (x={4}, y={5})]")]
    CropCoordinateOutOfBounds(u32, u32, u32, u32, u32, u32),
}
