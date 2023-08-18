use std::sync::{Arc, Mutex};
use crate::image_processing::RgbColorMap;

#[derive(Clone, Default)]
pub struct UserSettings {
    pub process_file_id: Option<String>,  // file_id of picture to process
    // pub palette_mapper: Option<Arc<Mutex<RgbColorMap>>>,  // extracted palette
    // pub palette_mapper: Option<u32>,  // extracted palette
    pub palette_file_id: Option<String>,
}
