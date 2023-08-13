use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::ops::Index;
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageEncoder, ImageOutputFormat, Rgb, RgbImage};
use image::codecs::png::FilterType::NoFilter;
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{get_kmeans, Kmeans, MapColor};
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette::cast::{from_component_slice, into_component_slice};
use crate::image_processing::palette_extraction::get_image_rgb_palette;

use crate::image_processing::ryemage_palette::{PaletteColorMap, LimitedColorSet};
use crate::image_processing::utils::load_image_from_unknown_reader;

mod utils;
mod palette_extraction;
mod ryemage_palette;

#[cfg(test)]
mod tests;

trait PaletteOperations {
    fn apply_palette_to_image(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self;
    fn dither_with_palette(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self;
    fn swap_palette<Pal>(&mut self, palette: Pal) -> &mut Self
        where
            Pal: ColorMap<Color=Rgb<u8>> + LimitedColorSet;
    // fn swap_palette_differentiate(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self;
}

impl PaletteOperations for RgbImage {
    fn apply_palette_to_image(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
        println!("loaded image: {:?}", self.dimensions());
        println!("Start image processing");
        for pixel in self.pixels_mut() {
            palette.map_color(pixel)
        }
        println!("source img buf is {:?}", self.len());
        println!("source img dimensions {:?}", self.dimensions());

        self
    }

    fn dither_with_palette(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
        println!("loaded image: {:?}", self.dimensions());
        println!("Start image processing");
        dither(
            self,
            &palette,
        );

        self
    }

    fn swap_palette<Pal>(&mut self, palette: Pal) -> &mut Self
        where
            Pal: ColorMap<Color=Rgb<u8>> + LimitedColorSet
    {
        println!("loaded image: {:?}", self.dimensions());
        println!("Detecting image own palette");
        let self_palette = PaletteColorMap::new(get_image_rgb_palette(
            DynamicImage::ImageRgb8(self.clone()),
            palette.color_set_size(),
        ));

        println!("Start image processing");
        for pixel in self.pixels_mut() {
            let color_index = self_palette.index_of(pixel);
            let complimentary_color = palette.lookup(color_index).unwrap();
            pixel.0 = complimentary_color.0;
        }

        self
    }

    // fn swap_palette_differentiate(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
    //     println!("loaded image: {:?}", self.dimensions());
    //     println!("Detecting image own palette");
    //
    //     println!("Start image processing");
    //     dither(
    //         self,
    //         &palette,
    //     );
    //
    //     self
    // }
}

//
// pub fn apply_palette_to_image(mut img: RgbImage, palette: impl ColorMap<Color=Rgb<u8>>) -> RgbImage {
//     let mut img = load_image_from_unknown_reader(reader)
//         .expect("Cannot load image")
//         .to_rgb8();
//
//     println!("loaded image: {:?}", img.dimensions());
//     println!("Start image processing");
//     for pixel in img.pixels_mut() {
//         palette.map_color(pixel)
//     }
//     println!("source img buf is {:?}", img.len());
//     println!("source img dimensions {:?}", img.dimensions());
//
//     // let mut result_buf: Vec<u8> = Vec::new();
//     // img.write_to(&mut Cursor::new(&mut result_buf), ImageOutputFormat::Png).unwrap();
//     img.to_vec()
// }
//
// pub fn dither_with_palette(mut reader: impl Read, palette: impl ColorMap<Color=Rgb<u8>>) -> Vec<u8> {
//     let mut img = load_image_from_unknown_reader(reader)
//         .expect("Cannot load image")
//         .to_rgb8();
//
//     println!("loaded image: {:?}", img.dimensions());
//     println!("Start image processing");
//     dither(
//         &mut img,
//         &palette,
//     );
//
//     // let mut result_buf: Vec<u8> = Vec::new();
//     // img.write_to(&mut Cursor::new(&mut result_buf), ImageOutputFormat::Png).unwrap();
//     // img.save("empty.jpg")?;
//     // result_buf
//     img.to_vec()
// }