use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{get_kmeans, Kmeans, MapColor};
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette::cast::{from_component_slice, into_component_slice};

pub fn load_image_from_unknown_reader(mut reader: impl Read) -> io::Result<DynamicImage> {
    let mut img_buf = Vec::new();
    reader.read_to_end(&mut img_buf)?;

    let img = ImageReader::new(Cursor::new(img_buf))
        .with_guessed_format()?
        .decode().unwrap();

    Ok(img)
}
