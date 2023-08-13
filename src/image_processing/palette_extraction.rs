use std::fs::File;
use std::{io, result};
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{Calculate, get_kmeans, Kmeans, MapColor, Sort};
use palette::{FromColor, IntoColor, Lab, LinSrgb, Srgb};
use palette::cast::{from_component_slice, into_component_slice};
use palette::luma::channels::La;

use crate::image_processing::utils::{downscale_to_size, load_image_from_unknown_reader};

const RUN_AMOUNT: u16 = 3;
const MAX_ITER: usize = 10;
const IMAGE_SIZE: (u32, u32) = (512, 512);


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

pub fn get_image_lab_palette(img: DynamicImage, quantity: usize) -> Vec<Lab> {
    let img = downscale_to_size(img, IMAGE_SIZE).to_rgb8();

    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(&img)
        .iter()
        .map(|x| x.into_linear().into_color())
        .collect();
    let result = find_kmeans_clusters(&lab, quantity, 5.0);

    Lab::sort_indexed_colors(&result.centroids, &result.indices)
        .iter()
        .map(|cd| cd.centroid)
        .collect()
    // result.centroids
}

pub fn get_image_rgb_palette(img: DynamicImage, quantity: usize) -> Vec<LinSrgb> {
    let img = downscale_to_size(img, IMAGE_SIZE).to_rgb8();

    let rgb: Vec<LinSrgb> = from_component_slice::<Srgb<u8>>(&img)
        .iter()
        .map(|x| x.into_linear())
        .collect();
    let result = find_kmeans_clusters(&rgb, quantity, 0.025);

    // result.centroids.iter().map(
    //     |&x| Srgb::from_linear(x.into_color())
    // ).collect()

    let srgb: Vec<Srgb> = result.centroids.iter().map(|&cd| Srgb::from_linear(cd)).collect();
    Srgb::sort_indexed_colors(&srgb, &result.indices)
        .iter()
        .map(|cd| cd.centroid.into_linear())
        .collect()

    // LinSrgb::sort_indexed_colors(&result.centroids, &result.indices)
    //     .iter()
    //     .map(|cd| cd.centroid)
    //     .collect()
    // result.centroids
}
