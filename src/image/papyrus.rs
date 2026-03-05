use crate::prelude::*;

pub(crate) fn papyrus(mut input_img: VipsImage, caption: &str) -> Result<VipsImage, Error> {

    input_img = input_img.colourspace(ops::Interpretation::Srgb)?;
    if !input_img.hasalpha() {
        input_img = input_img.addalpha()?;
    }


    let width = input_img.get_width();
    let size = width / 12;
    let height = input_img.get_page_height();
    let num_pages = input_img.get_n_pages(); // do i need to do the if avif then this = 1 thing that esmbot does?
    let text_width = width - (2 * width / 25);
    let capt = format!("<span foreground=\"white\">{caption}</span>");
    let text = VipsImage::text_with_opts(
        &capt,
        VOption::new()
            .set("rgba", true)
            .set("align", 0 /* <--VIPS_ALIGN_LOW enum */)
            .set("fontfile", "assets/Papyrus.ttf")
            .set("font", &format!("Papyrus {size}"))
            .set("width", text_width),
    )?;

    let v = (0..num_pages).map(|i| {
        input_img
            .crop(0, i * height, width, height).unwrap()
            .composite2_with_opts(
                &text,
                ops::BlendMode::Over,
                VOption::new()
                    .set("x", width / 10)
                    .set("y", height / 6)
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