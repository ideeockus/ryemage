use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{Calculate, get_kmeans, Kmeans, MapColor};
use palette::{FromColor, IntoColor, Lab, LinSrgb, Srgb};
use palette::cast::{from_component_slice, into_component_slice};
use palette::luma::channels::La;

use crate::image_processing::utils::load_image_from_unknown_reader;

const RUN_AMOUNT: u16 = 5;
const MAX_ITER: usize = 20;
// const CONVERGE: f32 = 5.0;


fn find_kmeans_clusters<C: Calculate + Clone>(img_buf: &[C], quantity: usize, converge: f32) -> Kmeans<C> {
    let mut result = Kmeans::new();

    let seed = 351;
    for i in 0..RUN_AMOUNT {
        let run_result = get_kmeans(
            quantity,
            MAX_ITER,
            converge,
            true,
            img_buf,
            seed + i as u64,
        );
        if run_result.score < result.score {
            result = run_result;
        }
    }

    result
}

pub fn get_image_lab_palette(img: RgbImage, quantity: usize) -> Vec<Lab> {
    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(&img)
        .iter()
        .map(|x| x.into_linear().into_color())
        .collect();


    // let mut result = Kmeans::new();
    // for i in 0..RUN_AMOUNT {
    //     let run_result = get_kmeans(
    //         quantity,
    //         MAX_ITER,
    //         CONVERGE,
    //         true,
    //         &lab,
    //         seed + i as u64,
    //     );
    //     if run_result.score < result.score {
    //         result = run_result;
    //     }
    // }
    let result = find_kmeans_clusters(&lab, quantity, 5.0);

    result.centroids
    // result.centroids.iter().map(
    //     |&x| Srgb::from_linear(x.into_color())
    // ).collect()
    // Srgb::map_indices_to_centroids(&srgb_centroids, &result.indices)
}

pub fn get_image_rgb_palette(img: RgbImage, quantity: usize) -> Vec<Srgb> {
    let rgb: Vec<LinSrgb> = from_component_slice::<Srgb<u8>>(&img)
        .iter()
        .map(|x| x.into_linear())
        .collect();

    let result = find_kmeans_clusters(&rgb, quantity, 0.025);

    // let mut result = Kmeans::new();
    // for i in 0..RUN_AMOUNT {
    //     let run_result = get_kmeans(
    //         quantity,
    //         MAX_ITER,
    //         CONVERGE,
    //         true,
    //         &rgb,
    //         seed + i as u64,
    //     );
    //     if run_result.score < result.score {
    //         result = run_result;
    //     }
    // }

    result.centroids.iter().map(
        |&x| Srgb::from_linear(x.into_color())
    ).collect()

    // Srgb::map_indices_to_centroids(srgb_centroids.as_slice(), &result.indices)
}

// pub fn test_color_palette() -> Vec<Srgb> {
//     let pic1 = Path::new("res/pic1.png");
//     let mut img_file = File::open(pic1).unwrap();
//     let img = load_image_from_unknown_reader(img_file)
//         .expect("Cannot load image")
//         .to_rgb8();
//
//     // let extracted_palette = get_image_lab_palette(img, 16);
//     let extracted_palette = get_image_rgb_palette(img, 16);
//
//     extracted_palette
// }
//
// pub fn test_neu_quant() -> NeuQuant {
//     let pic1 = Path::new("res/pic1.png");
//     let mut img_file = File::open(pic1).unwrap();
//     let img = load_image_from_unknown_reader(img_file)
//         .expect("Cannot load image")
//         .to_rgba8();
//     println!("Start NeuQuant train");
//
//     NeuQuant::new(1, 16, img.as_raw())
// }