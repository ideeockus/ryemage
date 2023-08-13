mod image_processing;

use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;
use color_quant::NeuQuant;
use image::imageops::{ColorMap, dither, index_colors};
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use image::io::Reader as ImageReader;
use kmeans_colors::{get_kmeans, Kmeans, MapColor};
use palette::cast::{from_component_slice, into_component_slice};
use palette::{FromColor, IntoColor, Lab, Srgb};

fn main() {
    println!("Hello, world!");
}


mod tests {

}