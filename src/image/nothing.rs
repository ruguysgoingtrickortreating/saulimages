use crate::prelude::*;

pub fn nothing(input_img: VipsImage) -> Result<VipsImage, Error> {
    dbg!(input_img.get_bands());
    Ok(input_img)
}