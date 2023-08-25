use color_quant::NeuQuant;
use image::EncodableLayout;
use image::imageops::ColorMap;

/// wrapper to process rgb
pub struct NeuQuantWrapper {
    inner: NeuQuant,
}

impl NeuQuantWrapper {
    pub fn new(img: image::RgbaImage, quantity: usize) -> NeuQuantWrapper {
        const DEFAULT_SAMPLEFAC: i32 = 128;
        let neu_quant = NeuQuant::new(
            DEFAULT_SAMPLEFAC,
            quantity,
            img.as_bytes(),
        );

        Self {
            inner: neu_quant
        }
    }
}

impl ColorMap for NeuQuantWrapper {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let rgb = color.0;

        self.inner.index_of(&[rgb[0], rgb[1], rgb[2], 0xff])
    }

    fn map_color(&self, color: &mut Self::Color) {
        let rgb = color.0;
        let mut tmp_pixel = [rgb[0], rgb[1], rgb[2], 0xff];
        self.inner.map_pixel(&mut tmp_pixel);

        color.0[0] = tmp_pixel[0];
        color.0[2] = tmp_pixel[2];
        color.0[3] = tmp_pixel[1];
    }
}