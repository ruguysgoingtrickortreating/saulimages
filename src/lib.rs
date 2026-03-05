mod image;
mod prelude {
    pub use rs_vips::{Vips, VipsImage, error::Error, ops, voption::{VOption, Setter}};
    pub use itertools::Itertools;
}

use prelude::*;

pub enum Operation {
    Caption(String),
    Pugsley,
    RioDeJaneiro,
    Papyrus(String),
}

pub fn init(name: &str) {
    rs_vips::Vips::init(name).unwrap();
}

pub fn chain(buffer: Vec<u8>,operations: Vec<Operation>) -> Result<Vec<u8>, Error> {
    // let mut img = VipsImage::new_from_buffer_with_opts(
    //     buffer.as_slice(),
    //     "",
    //     VOption::new().set("n", -1)
    // )?.colourspace(ops::Interpretation::Srgb)?;
    let mut img = buffer;
    for op in operations {
        // match op {
        //     Operation::Caption(caption) => img = image::caption(img, &caption)?,
        //     Operation::Pugsley => img = image::pugsley(img)?,
        //     Operation::RioDeJaneiro => img = image::rio_de_janeiro(img)?,
        //     Operation::Papyrus(caption) => img = image::papyrus(img, &caption)?,
        // }
        match op {
            Operation::Caption(c) => img = caption(img, &c)?,
            Operation::Pugsley => img = pugsley(img)?,
            Operation::RioDeJaneiro => img = rio_de_janeiro(img)?,
            Operation::Papyrus(c) => img = papyrus(img, &c)?,
        }
    }
    Ok(img)
    // img.webpsave_buffer()
}

pub fn caption(buffer: Vec<u8>, caption: &str) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::caption(input_img, caption)?.webpsave_buffer()
}

pub fn pugsley(buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::pugsley(input_img)?.webpsave_buffer()
}

pub fn nothing(buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::nothing(input_img)?.webpsave_buffer()
}

pub fn rio_de_janeiro(buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::rio_de_janeiro(input_img)?.webpsave_buffer()
}

pub fn papyrus(buffer: Vec<u8>, caption: &str) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::papyrus(input_img, caption)?.webpsave_buffer()
}

pub fn burn(buffer: Vec<u8>) -> Result<Vec<u8>, Error> {
    let input_img = process_buffer_into_image(&buffer)?;
    image::burn(input_img)?.webpsave_buffer()
}



fn process_buffer_into_image(buffer: &Vec<u8>) -> Result<VipsImage, Error> {
    let mut input_img = VipsImage::new_from_buffer_with_opts(
        buffer.as_slice(),
        "",
        VOption::new().set("n", -1)
    )?.colourspace(ops::Interpretation::Srgb)?;
    const MAX_SIZE:i32 = 1080;
    if input_img.get_width() > MAX_SIZE {
        if !input_img.get_page_height() > MAX_SIZE {
            let factor = MAX_SIZE as f64 / input_img.get_width() as f64;
            input_img = input_img.resize(factor)?
        }
    }
    if input_img.get_page_height() > MAX_SIZE {
        let factor = MAX_SIZE as f64 / input_img.get_page_height() as f64;
        input_img = input_img.resize(factor)?
    }

    Ok(input_img)
}