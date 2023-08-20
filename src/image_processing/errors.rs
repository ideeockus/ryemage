use std::fmt::Debug;
use std::{fmt, io};

use image::ImageError;

pub type ImageProcessingResult = Result<Vec<u8>, ImageProcessingError>;

#[derive(Debug)]
pub enum ImageProcessingError {
    IoError(io::Error),
    ImageError(ImageError),
    UnsupportedMode,
}

impl fmt::Display for ImageProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageProcessingError::IoError(err) => write!(f, "{}", err),
            ImageProcessingError::UnsupportedMode => write!(f, "This mode is unsupported"),
            ImageProcessingError::ImageError(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ImageProcessingError {}

impl From<io::Error> for ImageProcessingError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ImageError> for ImageProcessingError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value)
    }
}
