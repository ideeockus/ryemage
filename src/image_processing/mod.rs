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
use log::info;
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
mod ryemage_palette;
mod color_mappers;

#[cfg(test)]
mod tests;
mod errors;
mod palette_operations;

const DEFAULT_QUANTITY: usize = 32;

// TODO add mode params to enum
#[derive(Debug)]
pub enum PaletteMapperMode {
    SimpleLab,  // replace with color from palette with shortest chromatic distance
    SimpleRgb,  // replace with color from palette with shortest distance
    RgbDither,  // replace with color from palette with shortest distance using dithering
    NeuQuant,  // train neural network and perform palette replacement
    RgbSwap,  // reduce colors to palette_ and swap to complimentary color from palette_2
    PixelDiff,  // ???
    LabChromaEq, //  make a* and b* components of LAB equal
}

pub fn perform_action_on_files(
    palette_image: &Path,
    img_to_process: &Path,
    mode: PaletteMapperMode,
) -> ImageProcessingResult {
    let start = SystemTime::now();

    let palette_image = load_image_from_file(palette_image)?;
    let img_to_process = load_image_from_file(img_to_process)?;

    // let color_mapper = create_swap_palette_mapper(
    //     &donor_image,
    //     &img_to_process,
    //     PALETTE_QUANTITY,
    // );

    let color_mapper = match mode {
        PaletteMapperMode::SimpleLab => {
            create_lab_palette_mapper(palette_image, DEFAULT_QUANTITY)
        }
        PaletteMapperMode::SimpleRgb => {
            create_rgb_palette_mapper(palette_image, DEFAULT_QUANTITY)
        }
        // PaletteMapperMode::RgbDither => {}
        // PaletteMapperMode::NeuQuant => {}
        PaletteMapperMode::RgbSwap => {
            create_swap_palette_mapper(
                &img_to_process,
                &palette_image,
                DEFAULT_QUANTITY
            )
        }
        // PaletteMapperMode::PixelDiff => {}
        // PaletteMapperMode::LabChromaEq => {}
        _ => {
            return Err(ImageProcessingError::UnsupportedMode)
        }
    };

    let mut img_to_process = img_to_process.to_rgb8();
    img_to_process.apply_palette_to_image(color_mapper);
    // img_to_process.save_with_format(
    //     "gen_test_swap",
    //     ImageFormat::Png,
    // ).expect("Failed to save file");
    let mut result = Vec::with_capacity(img_to_process.len());
    let mut result = Cursor::new(result);
    img_to_process.write_to(&mut result, ImageFormat::Png)?;

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    info!("performing action {mode:?} took {} seconds", duration.as_secs());

    Ok(result.into_inner())
}


