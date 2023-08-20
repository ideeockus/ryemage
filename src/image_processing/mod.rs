use std::fs::File;
use std::io;
use std::io::{BufReader, Cursor, Read};
use std::ops::Index;
use std::path::Path;
use std::time::SystemTime;

use color_quant::NeuQuant;
use image::{DynamicImage, ImageBuffer, ImageEncoder, ImageFormat, ImageOutputFormat, Rgb, RgbImage};
use image::codecs::png::FilterType::NoFilter;
use image::imageops::{ColorMap, dither, index_colors};
use image::io::Reader as ImageReader;
use kmeans_colors::{get_kmeans, Kmeans, MapColor};
use log::{debug, info};
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette::cast::{from_component_slice, into_component_slice};

use color_mappers::LabPaletteMapper;
pub use color_mappers::RgbColorMap;
use color_mappers::RgbPaletteMapper;
pub use palette_extraction::create_lab_palette_mapper;
pub use palette_extraction::create_rgb_palette_mapper;
pub use palette_extraction::create_swap_palette_mapper;
pub use utils::load_image_from_file;

use crate::image_processing::errors::{ImageProcessingError, ImageProcessingResult};
use crate::image_processing::palette_operations::PaletteOperations;
use crate::image_processing::utils::load_image_from_unknown_reader;

mod utils;
mod palette_extraction;
mod color_mappers;

#[cfg(test)]
mod tests;
mod errors;
mod palette_operations;

const DEFAULT_QUANTITY: usize = 8;

// TODO add mode params to enum
#[derive(Debug)]
pub enum PaletteMapperMode {
    // replace with color from palette with shortest chromatic distance
    SimpleLab,
    // replace with color from palette with shortest distance
    SimpleRgb,
    // replace with color from palette with shortest distance using dithering
    RgbDither,
    // train neural network and perform palette replacement
    NeuQuant,
    // reduce colors to palette_ and swap to complimentary color from palette_2
    RgbSwap,
    // ???
    PixelDiff,
    //  make a* and b* components of LAB equal
    LabChromaEq,
}

pub fn perform_action_on_files(
    palette_image: &Path,
    img_to_process: &Path,
    mode: PaletteMapperMode,
) -> ImageProcessingResult {
    let start = SystemTime::now();
    debug!("start perform actions on files");

    let palette_image = load_image_from_file(palette_image)?;
    let img_to_process = load_image_from_file(img_to_process)?;

    let processed_image = match mode {
        PaletteMapperMode::SimpleLab => {
            let color_mapper = create_lab_palette_mapper(palette_image, DEFAULT_QUANTITY);
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        PaletteMapperMode::SimpleRgb => {
            let color_mapper = create_rgb_palette_mapper(palette_image, DEFAULT_QUANTITY);
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        PaletteMapperMode::RgbDither => {
            let color_mapper = create_rgb_palette_mapper(palette_image, DEFAULT_QUANTITY);
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.dither_with_palette(color_mapper);
            img_to_process
        }
        // PaletteMapperMode::NeuQuant => {}
        PaletteMapperMode::RgbSwap => {
            let color_mapper = create_swap_palette_mapper(
                &img_to_process,
                &palette_image,
                DEFAULT_QUANTITY,
            );
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        // PaletteMapperMode::PixelDiff => {}
        // PaletteMapperMode::LabChromaEq => {}
        _ => {
            return Err(ImageProcessingError::UnsupportedMode);
        }
    };

    let mut result = Vec::with_capacity(processed_image.len());
    let mut result = Cursor::new(result);
    processed_image.write_to(&mut result, ImageFormat::Png)?;

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    info!("performing action {mode:?} took {} seconds", duration.as_secs());

    Ok(result.into_inner())
}


