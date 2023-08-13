use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use image::{ImageFormat, open};

use crate::image_processing::PaletteOperations;
use crate::image_processing::palette_extraction::{get_image_lab_palette, get_image_rgb_palette};
use crate::image_processing::ryemage_palette::PaletteColorMap;
use crate::image_processing::utils::{load_image_from_file, load_image_from_unknown_reader, save_image};


const PIC_1: &str = "res/pic1.png";
const PIC_2: &str = "res/pic2.jpg";
const PIC_3: &str = "res/pic3.jpg";
const PIC_4: &str = "res/pic4.jpg";
const PIC_5: &str = "res/pic5.jpg";
const PIC_6: &str = "res/pic6.jpg";
const PIC_7: &str = "res/pic7.jpg";
const PIC_8: &str = "res/pic8.png";
const PIC_9: &str = "res/pic9.jpg";
const PIC_10: &str = "res/pic10.jpg";

const DONOR_PIC: &str = PIC_2;
const RECEPTOR_PIC: &str = PIC_4;


#[test]
fn test_dither_with_palette() {
    let start = SystemTime::now();
    let donor_image = load_image_from_file(DONOR_PIC)
        .expect("Cannot load image");
    let test_palette = PaletteColorMap::new(get_image_rgb_palette(donor_image, 32));

    let mut img_to_process = load_image_from_file(RECEPTOR_PIC)
        .expect("Cannot load image")
        .to_rgb8();

    img_to_process.dither_with_palette(test_palette);
    img_to_process.save_with_format(
        "gen_test_dither",
        ImageFormat::Png,
    ).expect("Failed to save file");
    // save_image("gen_test_dither", processed_image).expect("Failed to save file");

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Test took {} seconds", duration.as_secs());
}

#[test]
fn test_apply_palette_to_image() {
    let start = SystemTime::now();

    let donor_image = load_image_from_file(DONOR_PIC)
        .expect("Cannot load image");
    let test_palette = PaletteColorMap::new(get_image_rgb_palette(donor_image, 32));

    let mut img_to_process = load_image_from_file(RECEPTOR_PIC)
        .expect("Cannot load image")
        .to_rgb8();

    img_to_process.apply_palette_to_image(test_palette);
    img_to_process.save_with_format(
        "gen_test_palette",
        ImageFormat::Png,
    ).expect("Failed to save file");
    // save_image("gen_test_palette", processed_image).expect("Failed to save file");

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Test took {} seconds", duration.as_secs());
}

#[test]
fn test_swap_palette() {
    let start = SystemTime::now();

    let donor_image = load_image_from_file(DONOR_PIC)
        .expect("Cannot load image");
    let test_palette = PaletteColorMap::new(get_image_rgb_palette(donor_image, 32));

    let mut img_to_process = load_image_from_file(RECEPTOR_PIC)
        .expect("Cannot load image")
        .to_rgb8();

    img_to_process.swap_palette(test_palette);
    img_to_process.save_with_format(
        "gen_test_swap",
        ImageFormat::Png,
    ).expect("Failed to save file");
    // save_image("gen_test_palette", processed_image).expect("Failed to save file");

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Test took {} seconds", duration.as_secs());
}