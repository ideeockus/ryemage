use image::{Rgb, Rgba, RgbaImage, RgbImage};

use image::imageops::{dither, ColorMap};

use log::{debug, info};

pub trait PaletteOperations {
    fn apply_palette_to_image(&mut self, palette: Box<dyn ColorMap<Color=Rgb<u8>>>) -> &mut Self;
    fn dither_with_palette(&mut self, palette: Box<dyn ColorMap<Color=Rgb<u8>>>) -> &mut Self;
}

impl PaletteOperations for RgbImage {
    // fn apply_palette_to_image(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
    fn apply_palette_to_image(&mut self, palette: Box<dyn ColorMap<Color=Rgb<u8>>>) -> &mut Self {
        info!("loaded image: {:?}", self.dimensions());
        debug!("Start image processing");
        for pixel in self.pixels_mut() {
            palette.map_color(pixel)
        }
        debug!("source img buf is {:?}", self.len());
        debug!("source img dimensions {:?}", self.dimensions());

        self
    }

    // fn dither_with_palette(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
    fn dither_with_palette(&mut self, palette: Box<dyn ColorMap<Color=Rgb<u8>>>) -> &mut Self {
        info!("loaded image: {:?}", self.dimensions());
        debug!("Start image processing");

        dither(self, palette.as_ref());

        self
    }
}

// same as fn from PaletteOperations, but for RGBA (not RGB)
pub fn apply_palette_to_rgba_image(mut img: RgbaImage, palette: Box<dyn ColorMap<Color=Rgba<u8>>>) -> RgbaImage {
    info!("loaded image: {:?}", img.dimensions());
    debug!("Start image processing");
    for pixel in img.pixels_mut() {
        palette.map_color(pixel)
    }
    debug!("source img buf is {:?}", img.len());
    debug!("source img dimensions {:?}", img.dimensions());

    img
}