use crate::prelude::*;

// static FUTURA: &'static [u8] = include_bytes!("../../assets/Futura Condensed Bold.otf");

pub fn caption(mut input_img: VipsImage, caption: &str) -> Result<VipsImage, Error> {
    // let mut input_img = VipsImage::new_from_buffer_with_opts(
    //     buffer.as_slice(),
    //     "",
    //     VOption::new().set("n", -1),
    // )?.colourspace(ops::Interpretation::Srgb)?;
    input_img = input_img.colourspace(ops::Interpretation::Srgb)?;
    if !input_img.hasalpha() {
        input_img = input_img.addalpha()?;
    }


    let width = input_img.get_width();
    let size = width / 10;
    let page_height = input_img.get_page_height();
    let num_pages = input_img.get_n_pages(); // do i need to do the if avif then this = 1 thing that esmbot does?
    let text_width = width - (2 * width / 25);
    let capt = format!("<span background=\"white\">{caption}</span>");

    let txt_img = VipsImage::text_with_opts(
        &capt,
        VOption::new()
            .set("rgba", true)
            .set("align", 1 /* <--VIPS_ALIGN_CENTRE enum */) // lmao vips british ass
            .set("fontfile", "assets/Futura Condensed Bold.otf")
            .set("font", &format!("Futura Condensed {size}"))
            .set("width", text_width),
    )?;

    let mut img_caption = txt_img
        .relational(                        // i am not sure why this code line down to the other comment even exist
            &VipsImage::black(txt_img.get_width(), txt_img.get_height())?,
            ops::OperationRelational::Equal)?
        .bandbool(ops::OperationBoolean::And)?
        .ifthenelse(
            &VipsImage::new_from_image(
                &VipsImage::black(txt_img.get_width(), txt_img.get_height())?,
                &[255.0],
            )?,
            &txt_img)?  // honestly gravity should suffice but esmbot does it so so do i
        .gravity_with_opts(
            ops::CompassDirection::Centre,
            width, txt_img.get_height() + size,
            VOption::new().set("extend", "white"),
        )?;
    
    let output = if num_pages == 1 {
        img_caption.join(&input_img, ops::Direction::Vertical)?
    } else {
        let v = (0..num_pages).into_iter().map(|i| {
            let slice = input_img.crop(0, i * page_height, width, page_height).unwrap();
            img_caption.join(&slice, ops::Direction::Vertical).unwrap()
        }).collect_vec();
        let output = VipsImage::arrayjoin_with_opts(
            v.as_slice(),
            VOption::new()
                .set("across", 1),
        )?;
        output.set_int("page-height", page_height + img_caption.get_height())?;
        output
    };

    Ok(output)
}