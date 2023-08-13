use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

use crate::image_processing::{apply_palette_to_image, dither_with_palette};
use crate::image_processing::palette_extraction::get_image_lab_palette;
use crate::image_processing::pallete::PaletteColorMap;
use crate::image_processing::utils::load_image_from_unknown_reader;

#[test]
fn test_dither_with_neu_quant() {
    let start = SystemTime::now();
    let pic1 = Path::new("res/pic1.png");
    let pic2 = Path::new("res/pic2.jpg");
    let pic3 = Path::new("res/pic3.jpg");
    let pic4 = Path::new("res/pic4.jpg");

    let mut donor_img_file = File::open(pic1).unwrap();
    let donor_img_file = load_image_from_unknown_reader(donor_img_file)
        .expect("Cannot load image")
        .to_rgb8();
    let test_palette = PaletteColorMap::new(get_image_lab_palette(donor_img_file, 8));


    let img_file = File::open(pic3).unwrap();
    let processed_image = dither_with_palette(img_file, test_palette);
    let mut save_file = File::create("gen_test_dither").unwrap();
    save_file.write_all(&processed_image).unwrap();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Test took {} seconds", duration.as_secs());
}

#[test]
fn test_apply_palette_to_image() {
    let start = SystemTime::now();
    let pic1 = Path::new("res/pic1.png");
    let pic2 = Path::new("res/pic2.jpg");
    let pic3 = Path::new("res/pic3.jpg");
    let pic4 = Path::new("res/pic4.jpg");

    let mut donor_img_file = File::open(pic1).unwrap();
    let donor_image = load_image_from_unknown_reader(donor_img_file)
        .unwrap()
        .to_rgb8();
    let test_palette = PaletteColorMap::new(get_image_lab_palette(donor_image, 8));

    let img_file = File::open(pic3).unwrap();
    let processed_image = apply_palette_to_image(img_file, test_palette);
    let mut save_file = File::create("gen_test_palette").unwrap();
    save_file.write_all(&processed_image).unwrap();

    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Test took {} seconds", duration.as_secs());
}