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
use log::{debug, info};
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette::cast::{from_component_slice, into_component_slice};


pub trait PaletteOperations {
    fn apply_palette_to_image(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self;
    fn dither_with_palette(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self;
}

impl PaletteOperations for RgbImage {
    fn apply_palette_to_image(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
        info!("loaded image: {:?}", self.dimensions());
        debug!("Start image processing");
        for pixel in self.pixels_mut() {
            palette.map_color(pixel)
        }
        debug!("source img buf is {:?}", self.len());
        debug!("source img dimensions {:?}", self.dimensions());

        self
    }

    fn dither_with_palette(&mut self, palette: impl ColorMap<Color=Rgb<u8>>) -> &mut Self {
        info!("loaded image: {:?}", self.dimensions());
        debug!("Start image processing");
        dither(
            self,
            &palette,
        );

        self
    }
}