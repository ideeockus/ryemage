use std::io::Cursor;
use std::path::Path;
use std::time::SystemTime;

use image::{DynamicImage, ImageFormat};
use log::{debug, info};

use crate::errors::{ImageProcessingError, ImageProcessingResult};
use crate::palette_extraction::{create_diff_palette_mapper, create_lab_palette_mapper, create_neu_quant_mapper, create_neu_quant_rgb_wrapper_mapper, create_rgb_palette_mapper, create_swap_palette_mapper};
use crate::palette_operations::{apply_palette_to_rgba_image, PaletteOperations};
use crate::utils::mmap_image_from_file;
pub use color_mappers::RgbColorMapper;

mod color_mappers;
mod errors;
mod palette_extraction;
mod palette_operations;
mod utils;

#[cfg(test)]
mod tests;

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

// pub fn apply_palette_to_image();

pub fn perform_action_on_files(
    palette_image: &Path,
    img_to_process: &Path,
    mode: PaletteMapperMode,
    color_quantity: usize,
) -> ImageProcessingResult {
    let start = SystemTime::now();
    debug!("start perform actions on files");

    let palette_image = mmap_image_from_file(palette_image)?;
    // let palette_image = load_image_from_file(palette_image)?;
    debug!("palette image loaded");
    let img_to_process = mmap_image_from_file(img_to_process)?;
    // let img_to_process = load_image_from_file(img_to_process)?;
    debug!("image to process loaded");

    let processed_image = match mode {
        PaletteMapperMode::SimpleLab => {
            let color_mapper = create_lab_palette_mapper(palette_image, color_quantity);
            debug!("palette extracted");
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        PaletteMapperMode::SimpleRgb => {
            let color_mapper = create_rgb_palette_mapper(palette_image, color_quantity);
            debug!("palette extracted");
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        PaletteMapperMode::RgbDither => {
            let color_mapper = create_rgb_palette_mapper(palette_image, color_quantity);
            debug!("palette extracted");
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.dither_with_palette(color_mapper);
            img_to_process
        }
        PaletteMapperMode::NeuQuant => {
            // apply_palette_to_rgba_image


            // let color_mapper = create_neu_quant_rgb_wrapper_mapper(
            //     &img_to_process,
            //     color_quantity,
            // );
            //
            let color_mapper = create_neu_quant_mapper(
                &palette_image,
                color_quantity,
            );
            debug!("palette extracted");
            let img_to_process = img_to_process.to_rgba8();
            // img_to_process.apply_palette_to_image(color_mapper);
            let result_image = apply_palette_to_rgba_image(img_to_process, color_mapper);
            DynamicImage::ImageRgba8(result_image).to_rgb8()
        }
        PaletteMapperMode::RgbSwap => {
            let color_mapper =
                create_swap_palette_mapper(&img_to_process, &palette_image, color_quantity);
            debug!("palette extracted");
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        PaletteMapperMode::PixelDiff => {
            let color_mapper = create_diff_palette_mapper(
                &img_to_process,
                &palette_image,
                color_quantity,
            );
            debug!("palette extracted");
            let mut img_to_process = img_to_process.to_rgb8();
            img_to_process.apply_palette_to_image(color_mapper);
            img_to_process
        }
        // PaletteMapperMode::LabChromaEq => {}
        _ => {
            return Err(ImageProcessingError::UnsupportedMode);
        }
    };
    debug!("image processing finished");

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
