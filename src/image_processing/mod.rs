use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageEncoder, ImageOutputFormat, Rgb, RgbImage};
use image::codecs::png::FilterType::NoFilter;
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{get_kmeans, Kmeans, MapColor};
use palette::{FromColor, IntoColor, Lab, Srgb, Srgba};
use palette::cast::{from_component_slice, into_component_slice};

use crate::image_processing::pallete::PaletteColorMap;
use crate::image_processing::utils::load_image_from_unknown_reader;

mod utils;
mod palette_extraction;
mod constants;
#[cfg(test)]
mod tests;
mod pallete;


pub fn apply_palette_to_image(mut reader: impl Read, palette: impl ColorMap<Color=Rgb<u8>>) -> Vec<u8> {
    let mut img = load_image_from_unknown_reader(reader)
        .expect("Cannot load image")
        .to_rgb8();

    println!("loaded image: {:?}", img.dimensions());

    // let color_map = get_color_map();
    // let color_map = test_color_palette();
    // todo fix
    // let a = img.iter().map(|x| {
    //     let a: Srgb<u8> = Srgb::from_color(x).into_format();
    //     [a.red, a.green, a.blue]
    // }).flatten().collect::<Vec<u8>>();
    println!("Start image processing");
    for pixel in img.pixels_mut() {
        palette.map_color(pixel)
    }
    // let rgb: Vec<Srgb<u8>> = color_map
    //     .iter()
    //     .map(
    //         // |&x| Srgb::from_linear(x.into_color())
    //         |&x| Srgb::from_color(x).into_format()
    //     )
    //     .collect();
    // let result_buf: Vec<u8> = into_component_slice(&rgb).to_vec();
    // let img: RgbImage = ImageBuffer::from_vec(
    //     img.width(),
    //     img.height(),
    //     result_buf,
    // ).expect("Oops error when creating file");
    // let processed: &[u8] = into_component_slice(&rgb);
    // println!("processed buf is {:?}", processed.len());
    println!("source img buf is {:?}", img.len());
    println!("source img dimensions {:?}", img.dimensions());


    // let img: RgbImage = ImageReader::new(Cursor::new(processed))
    //     .decode().expect("Cannot decode :(")
    //     .to_rgb8();
    // let img: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(
    //     img.width(),
    //     img.height(),
    //     processed,
    // ).expect("Cannot create imgae bufer");

    let mut result_buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut result_buf), ImageOutputFormat::Png).unwrap();
    // img.save("empty.jpg")?;
    result_buf

}

pub fn dither_with_palette(mut reader: impl Read, palette: impl ColorMap<Color=Rgb<u8>>) -> Vec<u8> {
    // let mut buf = Vec::new();
    // reader.read_to_end(&mut buf)?;

    // let mut img = ImageReader::new(Cursor::new(img_buf))
    //     .with_guessed_format()?
    //     .decode()?
    //     .to_rgb8();
    let mut img = load_image_from_unknown_reader(reader)
        .expect("Cannot load image")
        .to_rgb8();

    println!("loaded image: {:?}", img.dimensions());

    // let color_map = get_color_map();

    println!("Start image processing");
    dither(
        &mut img,
        &palette,
    );

    let mut result_buf: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut result_buf), ImageOutputFormat::Png).unwrap();
    // img.save("empty.jpg")?;
    result_buf
}