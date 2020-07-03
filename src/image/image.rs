use std::vec::Vec;

use image::{
    jpeg::JPEGEncoder,
    png::{CompressionType, FilterType, PNGEncoder},
    ColorType, GenericImage, GenericImageView, ImageBuffer, Luma, Rgb, Rgba, RgbaImage,
};
use rand::{thread_rng, Rng};

pub type ImageWidth = u32;
pub type ImageHeight = u32;

pub enum ImageType {
    JPG,
    PNG,
    GIF,
}

/*
https://stackoverflow.com/questions/35488820/how-to-create-a-rust-struct-with-an-imageimagebuffer-as-a-member
 */

fn gen(width: ImageWidth, height: ImageHeight) { let mut image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height); }

pub fn gen_verify_image(numbers: &[u8]) -> Vec<u8> {
    let number_len = numbers.len() as u32;
    const WIDTH: u32 = 64u32;
    const HEIGHT: u32 = 64u32;
    let width = number_len * WIDTH;
    // let mut img = ImageBuffer::<Luma<u8>, Vec<u8>>::from_fn(width, height, |x, y| {
    //     if x % 2 == 0 || y % 5 == 0 {
    //         Luma([0u8])
    //     } else {
    //         Luma([255u8])
    //     }
    // });
    let mut img = RgbaImage::new(width, HEIGHT);
    // let raw_data = img.into_raw();
    // let data = raw_data.as_slice();
    // dbg!("{}", data);

    let mut x_offset = 0u32;
    for n in numbers.into_iter() {
        let number = image::load_from_memory_with_format(
            super::asset::rand_group_number_image(*n as usize).data,
            image::ImageFormat::Png,
        )
        .unwrap();
        let mut rng = thread_rng();
        for (x, y, pixel) in number.to_rgba().enumerate_pixels() {
            // pixel.0[3] = 75;
            if x % 10 == 0 || y % 10 == 0 {
                img.put_pixel(
                    x,
                    y,
                    Rgba([rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 100]),
                );
            } else {
                img.put_pixel(x + x_offset, y, *pixel);
            }
        }
        x_offset += WIDTH;
    }

    let mut out = Vec::with_capacity(10240);
    // let mut encoder = JPEGEncoder::new_with_quality(&mut out, 70);
    // let r = encoder.encode_image(&img);
    let encoder = PNGEncoder::new_with_quality(&mut out, CompressionType::Default, FilterType::NoFilter);
    encoder.encode(&img.into_raw(), width, HEIGHT, ColorType::Rgba8);
    dbg!("out.len() = {}", out.len());
    out
}
