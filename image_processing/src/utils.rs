use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Write};
use std::path::Path;
use std::{cmp, io};

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;

const IMAGE_SIZE_LIMIT: (u32, u32) = (1280, 1280);
// const IMAGE_SIZE_LIMIT: (u32, u32) = (512, 512);

pub fn downscale_to_size(
    img: &DynamicImage,
    target_size: (u32, u32),
    filter: FilterType,
) -> Option<DynamicImage> {
    let width = img.width() as f32;
    let height = img.height() as f32;

    let resize_factor: f32 = cmp::max_by(
        width / target_size.0 as f32,
        height / target_size.1 as f32,
        |a, b| a.partial_cmp(b).unwrap(),
    );

    if resize_factor > 1.0 {
        return Some(img.resize(
            (width / resize_factor) as u32,
            (height / resize_factor) as u32,
            filter,
        ));
    }

    None
}

pub fn load_image_from_file<P>(path: P) -> io::Result<DynamicImage>
where
    P: AsRef<Path>,
{
    let fd = File::open(path)?;
    let file_size = fd.metadata().map(|m| m.len()).ok();
    println!("file size is {:?}", file_size);

    load_image_from_unknown_reader(BufReader::new(fd), file_size)
}

pub fn load_image_from_unknown_reader(
    mut reader: impl BufRead,
    size: Option<u64>,
) -> io::Result<DynamicImage> {
    let mut img_buf = match size {
        Some(size) if size > usize::MAX as u64 => {
            return Err(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "Image size is too much",
            ));
        }
        Some(size) => Vec::with_capacity(size as usize),
        None => Vec::new(),
    };

    reader.read_to_end(&mut img_buf)?;

    let img = ImageReader::new(Cursor::new(img_buf))
        .with_guessed_format()?
        .decode()
        .unwrap();

    let img = downscale_to_size(&img, IMAGE_SIZE_LIMIT, FilterType::Lanczos3).unwrap_or(img);

    Ok(img)
}

#[allow(unused)]
pub fn save_image<P, B>(path: P, processed_image: B) -> io::Result<()>
where
    P: AsRef<Path>,
    B: AsRef<[u8]>,
{
    let mut save_file = File::create(path)?;
    save_file.write_all(processed_image.as_ref())
}
