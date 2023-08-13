mod constants;
mod image_processing;
mod palette_extraction;
mod utils;

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


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use crate::image_processing::apply_palette_to_image;

    // #[test]
    // fn test_dither_with_neu_quant() {
    //     let pic1 = Path::new("res/pic1.png");
    //     let pic2 = Path::new("res/pic2.jpg");
    //     let pic3 = Path::new("res/pic3.jpg");
    //     let pic4 = Path::new("res/pic4.jpg");
    //
    //     let img_file = File::open(pic3).unwrap();
    //
    //     let processed_image = dither_with_neu_quant(img_file);
    //     let mut save_file = File::create("gen_test_dither").unwrap();
    //     save_file.write_all(&processed_image).unwrap();
    // }

    #[test]
    fn test_apply_palette_to_image() {
        let pic1 = Path::new("res/pic1.png");
        let pic2 = Path::new("res/pic2.jpg");
        let pic3 = Path::new("res/pic3.jpg");
        let pic4 = Path::new("res/pic4.jpg");

        let img_file = File::open(pic3).unwrap();

        let processed_image = apply_palette_to_image(img_file);
        let mut save_file = File::create("gen_test_palette").unwrap();
        save_file.write_all(&processed_image).unwrap();
    }
}