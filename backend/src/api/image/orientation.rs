use image::DynamicImage;

#[derive(Debug)]
pub enum ExifOrientation {
    Normal = 1,
    FlippedNormal,
    UpsideDown,
    FlippedUpsideDown,
    FlippedRotatedLeft,
    RotatedLeft,
    FlippedRotatedRight,
    RotatedRight,
}

impl TryFrom<u16> for ExifOrientation {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        use ExifOrientation::*;
        match v {
            x if x == Normal as u16 => Ok(Normal),
            x if x == FlippedNormal as u16 => Ok(FlippedNormal),
            x if x == UpsideDown as u16 => Ok(UpsideDown),
            x if x == FlippedUpsideDown as u16 => Ok(FlippedUpsideDown),
            x if x == FlippedRotatedLeft as u16 => Ok(FlippedRotatedLeft),
            x if x == RotatedLeft as u16 => Ok(RotatedLeft),
            x if x == FlippedRotatedRight as u16 => Ok(FlippedRotatedRight),
            x if x == RotatedRight as u16 => Ok(RotatedRight),
            _ => Err(()),
        }
    }
}

impl ExifOrientation {
    pub fn apply_to_image(&self, image: DynamicImage) -> DynamicImage {
        use ExifOrientation::*;
        match self {
            Normal => image,
            FlippedNormal => image.fliph(),
            UpsideDown => image.rotate180(),
            FlippedUpsideDown => image.rotate180().fliph(),
            FlippedRotatedLeft => image.rotate90().fliph(),
            RotatedLeft => image.rotate90(),
            FlippedRotatedRight => image.rotate270().fliph(),
            RotatedRight => image.rotate270(),
        }
    }
}
