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
use crate::utils::load_image_from_unknown_reader;


pub fn get_image_lab_palette(img: RgbImage) -> Vec<Lab> {
    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(&img)
        .iter()
        .map(|x| x.into_format().into_color())
        .collect();

    let runs = 3;
    let seed = 74391;

    let mut result = Kmeans::new();
    for i in 0..runs {
        let run_result = get_kmeans(
            16,
            20,
            5.0,
            true,
            &lab,
            seed + i * 2 as u64,
        );
        if run_result.score < result.score {
            result = run_result;
        }
    }

    // let rgb = &result.centroids
    //     .iter()
    //     .map(|x| Srgb::from_color(*x).into_format())
    //     .collect::<Vec<Srgb<u8>>>();

    println!("centroids {:?}", result.centroids);
    // result.centroids
    Lab::map_indices_to_centroids(&lab, &result.indices)
    // Srgb::map_indices_to_centroids(&lab, &result.indices)
}

pub fn extract_color_palette() -> Vec<Lab> {
    let pic1 = Path::new("res/pic1.png");
    let mut img_file = File::open(pic1).unwrap();
    let img = load_image_from_unknown_reader(img_file)
        .expect("Cannot load image")
        .to_rgb8();

    let extracted_palette = get_image_lab_palette(img);
    // println!("Extracted palette: {:?}", extracted_palette);

    extracted_palette
}

pub fn train_neu_quant() -> NeuQuant {
    // let mut img = ImageReader::new(Cursor::new(reader))
    //     .with_guessed_format()?
    //     .decode()?
    //     .to_rgba8();
    let pic1 = Path::new("res/pic1.png");
    let mut img_file = File::open(pic1).unwrap();
    let img = load_image_from_unknown_reader(img_file)
        .expect("Cannot load image")
        .to_rgba8();
    println!("Start NeuQuant train");

    NeuQuant::new(1, 16, img.as_raw())
}