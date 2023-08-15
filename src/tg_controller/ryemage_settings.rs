use std::sync::Arc;
use crate::image_processing::RgbColorMap;

#[derive(Clone, Default)]
pub struct UserSettings {
    pub file_id: Option<String>,  // file_id of picture to process
    // pub palette_mapper: Option<Arc<Box<RgbColorMap>>>,  // extracted palette
    pub palette_mapper: Option<u32>,  // extracted palette
}
