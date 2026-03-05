use crate::prelude::*;

pub(crate) fn burn(input_img: VipsImage) -> Result<VipsImage,Error> {
    // VipsImage::new_from_file("inputs/burn.webp")?.vipssave("assets/burn.v")?; panic!();
    let overlay = VipsImage::thumbnail_with_opts(
        "assets/janeiro.v",
        input_img.get_width(),
        VOption::new()
            .set("height", input_img.get_height())
            .set("size", 3)
    )?;

    let img = input_img.composite2(
        &overlay,
        ops::BlendMode::Over
    )?;

    Ok(img)
}