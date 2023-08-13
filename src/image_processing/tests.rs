use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use image::ImageFormat;

use crate::image_processing::PaletteOperations;
use crate::image_processing::palette_extraction::{get_image_lab_palette, get_image_rgb_palette};
use crate::image_processing::ryemage_palette::PaletteColorMap;
use crate::image_processing::utils::{load_image_from_file, load_image_from_unknown_reader, save_image};

// #[test]
// fn test_dither_with_palette() {
//     let start = SystemTime::now();
//     let pic1 = Path::new("res/pic1.png");
//     let pic2 = Path::new("res/pic2.jpg");
//     let pic3 = Path::new("res/pic3.jpg");
//     let pic4 = Path::new("res/pic4.jpg");
//
//     let donor_image = load_image_from_file(pic3)
//         .expect("Cannot load image");
//     let test_palette = PaletteColorMap::new(get_image_rgb_palette(donor_image, 8));
//
//     let mut img_to_process = load_image_from_file(pic2)
//         .expect("Cannot load image")
//         .to_rgb8();
//
//     img_to_process.dither_with_palette(test_palette);
//     img_to_process.save_with_format(
//         "gen_test_dither",
//         ImageFormat::Png,
//     ).expect("Failed to save file");
//     // save_image("gen_test_dither", processed_image).expect("Failed to save file");
//
//     let end = SystemTime::now();
//     let duration = end.duration_since(start).unwrap();
//     println!("Test took {} seconds", duration.as_secs());
// }

#[test]
fn test_apply_palette_to_image() {
    let start = SystemTime::now();
    let pic1 = Path::new("res/pic1.png");
    let pic2 = Path::new("res/pic2.jpg");
    let pic3 = Path::new("res/pic3.jpg");
    let pic4 = Path::new("res/pic4.jpg");

    let donor_image = load_image_from_file(pic3)
        .expect("Cannot load image");
    let test_palette = PaletteColorMap::new(get_image_rgb_palette(donor_image, 32));

    let mut img_to_process = load_image_from_file(pic2)
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