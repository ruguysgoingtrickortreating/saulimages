use std::f64::consts::PI;
use crate::prelude::*;

pub(crate) fn pugsley(mut input_img: VipsImage) -> Result<VipsImage, Error> {
    // if !input_img.hasalpha() {
    //     input_img = input_img.bandjoin_const(&[255.0])?
    // }
    // const MAX_SIZE:i32 = 1080;
    // if input_img.get_width() > MAX_SIZE {
    //     if !input_img.get_page_height() > MAX_SIZE {
    //         let factor = MAX_SIZE as f64 / input_img.get_width() as f64;
    //         input_img = input_img.resize(factor)?
    //     }
    // }
    // if input_img.get_page_height() > MAX_SIZE {
    //     let factor = MAX_SIZE as f64 / input_img.get_page_height() as f64;
    //     input_img = input_img.resize(factor)?
    // }

    let width = input_img.get_width();
    println!("{width}");
    let height = input_img.get_page_height();
    let num_pages = input_img.get_n_pages();

    let pugsley = VipsImage::new_from_file("assets/pugsley.webp")?
        .resize(0.8 * height as f64 / 1920.0)?;
    let pugsley_width = pugsley.get_width();

    if !input_img.hasalpha() {
        input_img = input_img.addalpha()?;
    }

    const NOTHING_DURATION: i32 = 6;
    let mut frames = (0..NOTHING_DURATION).map(|i|
        input_img.crop(0, (i%num_pages) * height, width, height).unwrap()
    ).collect_vec();

    const SLIDE_DURATION:i32 = 14;
    fn ease_pugsley(i:i32) -> f64 {
        let f = i as f64 / SLIDE_DURATION as f64;
        (1.0 - f).powi(3)
    }

    let shifted = (1..SLIDE_DURATION).map(|i| {
        let pugsley_background = pugsley.embed_with_opts(
            (width as f64 * ease_pugsley(i)) as i32,
            0,
            width,
            height,
            VOption::new().set("background", &[0,0,0,0])
        ).unwrap();
        let back = if num_pages > 1 {
            &input_img.crop(0, ((i + NOTHING_DURATION - 1) % num_pages) * height , width, height).unwrap()
        } else {
            &input_img
        };
        back.composite2_with_opts(
            &pugsley_background, ops::BlendMode::Over,
            VOption::new()
                .set("x", width - pugsley_width)
                .set("y", (height / 40) * 7)
        ).unwrap()
    });
    frames.extend(shifted);

    let vignette = VipsImage::thumbnail_with_opts(
        "assets/vignette_transparent.v",
        width,
        VOption::new()
            .set("height", height)
            .set("size", 3)
    )?;
    let skull = VipsImage::thumbnail(
        "assets/samsung_skull.v",
        (width / 9).clamp(1, i32::MAX)
    )?;
    let skull_x = width / 2 - skull.get_width() / 2;
    let skull_y = height / 2 - skull.get_height() / 2;

    fn ease_screenshake(i:i32) -> f64 {
        let f = i as f64 / 28.0; // ngl should be SHAKE_LENGTH
        (1.0 - f).powi(10)
    }

    const SHAKE_LENGTH:i32 = 28;
    let last = frames.last().unwrap()
        .composite2(
            &vignette,
            ops::BlendMode::Over
        )?;

    // let center_w = width / 2;
    // let center_h = height / 2 ;
    let blur_multiplier = (height + width) as f64 / 1000.0;
    println!("{blur_multiplier}");

    let mut screenshake = (0..SHAKE_LENGTH).map(|i| {
        let e = ease_screenshake(i);
        let offset = -(i as f64 * 5.0 + 2.0).sin() * e * height as f64 / 7.0;
        let offset_rot = (5.0*PI/3.0) - (PI*i as f64/7.0 * 3.0);
        let rot = (i as f64 * 1.3).sin();

        let rotated = last
            .gravity_with_opts(
                ops::CompassDirection::Centre,
                width * 2,
                height * 2,
                VOption::new().set("extend", 2 /*<-- VIPS_EXTEND_REPEAT enum*/)
            ).unwrap()
            .affine_with_opts(
                &[1.0,0.0,0.0,1.0],
                VOption::new()
                    .set("odx", offset * -offset_rot.cos())
                    .set("ody", offset * offset_rot.sin())
            ).unwrap()
            .similarity_with_opts(
                VOption::new()
                    .set("angle", rot / 2.0)
                // .set("idx", width as f64)
                // .set("idy", height as f64)
            ).unwrap();
        // .crop(width / 2, height / 2, width, height).unwrap()
        rotated
            .embed(
                ( width - rotated.get_width()) / 2,
                ( height - rotated.get_height()) / 2,
                width, height
            ).unwrap()
            .gaussblur((e * 5.0 + 0.6) * blur_multiplier).unwrap()
            .composite2_with_opts(
                &skull,
                ops::BlendMode::Over,
                VOption::new()
                    .set("x", skull_x)
                    .set("y", skull_y)
            ).unwrap()
    }).collect_vec();
    frames.append(&mut screenshake);


    let output = VipsImage::arrayjoin_with_opts(
        frames.as_slice(),
        VOption::new()
            .set("across", 1)
    )?;
    output.set_int("page-height", height)?;
    output.set_int("gif-delay", 4)?;

    Ok(output)
}