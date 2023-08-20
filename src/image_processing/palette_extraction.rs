use std::{cmp, io, result};
use std::fs::File;
use std::io::{BufReader, Cursor, Read};
use std::path::Path;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use image::imageops::{ColorMap, dither, FilterType, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{Calculate, get_kmeans, Kmeans, MapColor, Sort};
use palette::{FromColor, IntoColor, Lab, LinSrgb, Srgb};
use palette::cast::{from_component_slice, into_component_slice};
use palette::luma::channels::La;

use crate::image_processing::color_mappers::{LabPaletteMapper, RgbPaletteMapper, SwapPaletteMapper};
use crate::image_processing::utils::{downscale_to_size, load_image_from_unknown_reader};

const RUN_AMOUNT: u16 = 3;
const MAX_ITER: usize = 5;
const IMAGE_SIZE: (u32, u32) = (256, 256);


fn find_kmeans_clusters<C: Calculate + Clone>(img_buf: &[C], quantity: usize, converge: f32) -> Kmeans<C> {
    let mut result = Kmeans::new();

    let max_iter = match quantity {
        ..=16 => {
            15
        }
        17..=64 => {
            10
        }
        65..=128 => {
            5
        }
        _ => {
            3
        }
    };

    let seed = 351;
    for i in 0..RUN_AMOUNT {
        let run_result = get_kmeans(
            quantity,
            max_iter,
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

/// Extract image palette in Lab values
fn get_image_lab_palette(img: &RgbImage, quantity: usize, need_sort: bool) -> Vec<Lab> {
    let lab: Vec<Lab> = from_component_slice::<Srgb<u8>>(img.as_raw())
        .iter()
        .map(|x| x.into_linear().into_color())
        .collect();
    let result = find_kmeans_clusters(&lab, quantity, 5.0);

    if need_sort {
        Lab::sort_indexed_colors(&result.centroids, &result.indices)
            .iter()
            .map(|cd| cd.centroid)
            .collect()
    } else {
        result.centroids
    }
}

/// Extract image palette in linear RGB values
fn get_image_lin_rgb_palette(img: &RgbImage, quantity: usize, need_sort: bool) -> Vec<LinSrgb> {
    let rgb: Vec<LinSrgb> = from_component_slice::<Srgb<u8>>(img.as_raw())
        .iter()
        .map(|x| x.into_linear())
        .collect();
    let result = find_kmeans_clusters(&rgb, quantity, 0.025);

    if need_sort {
        let srgb: Vec<Srgb> = result.centroids.iter().map(|&cd| Srgb::from_linear(cd)).collect();
        Srgb::sort_indexed_colors(&srgb, &result.indices)
            .iter()
            .map(|cd| cd.centroid.into_linear())
            .collect()
    } else {
        result.centroids
    }
}


pub fn create_lab_palette_mapper(img: DynamicImage, quantity: usize) -> Box<dyn ColorMap<Color=(Rgb<u8>)>> {
    let img = downscale_to_size(&img, IMAGE_SIZE, FilterType::Nearest)
        .unwrap_or(img)
        .to_rgb8();

    let unsorted_palette = get_image_lab_palette(&img, quantity, false);

    Box::new(LabPaletteMapper::new(unsorted_palette))
}

pub fn create_rgb_palette_mapper(img: DynamicImage, quantity: usize) -> Box<dyn ColorMap<Color=(Rgb<u8>)>> {
    let img = downscale_to_size(&img, IMAGE_SIZE, FilterType::Nearest)
        .unwrap_or(img)
        .to_rgb8();

    let unsorted_palette = get_image_lin_rgb_palette(&img, quantity, false);

    Box::new(RgbPaletteMapper::new(unsorted_palette))
}

pub fn create_swap_palette_mapper(img_to_process: &DynamicImage, palette_img: &DynamicImage, quantity: usize) -> Box<dyn ColorMap<Color=(Rgb<u8>)>> {
    let palette_img = match downscale_to_size(palette_img, IMAGE_SIZE, FilterType::Nearest) {
        None => palette_img.to_rgb8(),
        Some(scaled) => scaled.to_rgb8(),
    };

    let img_to_process = match downscale_to_size(img_to_process, IMAGE_SIZE, FilterType::Nearest) {
        None => img_to_process.to_rgb8(),
        Some(scaled) => scaled.to_rgb8(),
    };

    let mut sorted_palette_1 = get_image_lin_rgb_palette(&palette_img, quantity, true);
    let mut sorted_palette_2 = get_image_lin_rgb_palette(&img_to_process, quantity, true);

    // Box::new(SwapPaletteMapper::new(sorted_palette_1, sorted_palette_2).unwrap())
    Box::new(SwapPaletteMapper::new(sorted_palette_2, sorted_palette_1).unwrap())
}


