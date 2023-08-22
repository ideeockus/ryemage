// use image_processing::RgbColorMapper;

const DEFAULT_QUANTITY: usize = 64;

#[derive(Clone)]
pub struct UserSettings {
    pub process_file_id: Option<String>, // file_id of picture to process
    pub palette_file_id: Option<String>,
    // pub palette_mapper: Option<Arc<RgbColorMapper>>,
    pub color_amount: usize,
}

impl UserSettings {
    pub fn set_process_file_id(&mut self, file_id: String) {
        self.process_file_id = Some(file_id);
    }

    // pub fn set_color_mapper(&mut self, palette_mapper: RgbColorMapper) {
    //     self.palette_mapper = Some(Arc::new(palette_mapper));
    // }

    pub fn set_color_amount(&mut self, color_amount: usize) {
        self.color_amount = color_amount;
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            process_file_id: None,
            palette_file_id: None,
            color_amount: DEFAULT_QUANTITY,
        }
    }
}
