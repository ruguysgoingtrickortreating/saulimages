use crate::prelude::*;

pub fn rio_de_janeiro(input_img: VipsImage) -> Result<VipsImage, Error> {
    // input_img.addalpha()?.linear(&[1.0,1.0,1.0,0.45], &[0.0,0.0,0.0,0.0])?.vipssave("assets/janeiro.v")?;
    // todo!();
    // input_img.vipssave("assets/janeiro_text.v")?;
    // todo!();
    let width = input_img.get_width();
    let height = input_img.get_page_height();
    let num_pages = input_img.get_n_pages();

    let janeiro = VipsImage::thumbnail_with_opts(
        "assets/janeiro.v",
        width,
        VOption::new()
            .set("height", height)
            .set("size", 3)
    )?;
    let text = VipsImage::thumbnail(
        "assets/janeiro_text.v",
        width * 3 / 7
    )?;
    let v = (0..num_pages).map(|i| {
        input_img
            .crop(0, i * height, width, height).unwrap()
            .composite2(
                &janeiro,
                ops::BlendMode::Over
            ).unwrap()
            .composite2_with_opts(
                &text,
                ops::BlendMode::Over,
                VOption::new()
                    .set("x", (width - text.get_width()) / 2)
                    .set("y", height * 3 / 7)
            ).unwrap()
    }).collect_vec();
    let output = VipsImage::arrayjoin_with_opts(
        v.as_slice(),
        VOption::new()
            .set("across", 1),
    )?;
    output.set_int("page-height", height)?;
    Ok(output)
}