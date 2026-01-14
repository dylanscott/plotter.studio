use usvg;

use crate::geometry::{BoundingBox, Group};

pub struct DigestResult {
    pub geometry: Group,
    pub bounding_box: BoundingBox,
}

pub struct DigestOptions {
    pub dpi: f32,
    pub curve_tolerance: f64,
}

pub fn digest_svg(text: &str, opt: &DigestOptions) -> Result<DigestResult, usvg::Error> {
    Err(usvg::Error::NotAnUtf8Str)
}
