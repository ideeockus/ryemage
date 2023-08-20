use std::io::Cursor;
use std::path::Path;
use std::time::SystemTime;

use image::ImageFormat;
use log::{debug, info};

use crate::errors::{ImageProcessingError, ImageProcessingResult};
use crate::palette_extraction::{create_lab_palette_mapper, create_rgb_palette_mapper, create_swap_palette_mapper};
use crate::palette_operations::PaletteOperations;
use crate::utils::load_image_from_file;

mod palette_extraction;
mod errors;
mod palette_operations;
mod utils;
mod color_mappers;

#[cfg(test)]
mod tests;

const DEFAULT_QUANTITY: usize = 32;

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
            let color_mapper =
                create_swap_palette_mapper(&img_to_process, &palette_image, DEFAULT_QUANTITY);
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

    let result = Vec::with_capacity(processed_image.len());
    let mut result = Cursor::new(result);
    processed_image.write_to(&mut result, ImageFormat::Png)?;

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    info!(
        "performing action {mode:?} took {} seconds",
        duration.as_secs()
    );

    Ok(result.into_inner())
}
