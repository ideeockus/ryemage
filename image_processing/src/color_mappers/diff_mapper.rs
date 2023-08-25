use image::imageops::ColorMap;
use crate::color_mappers::SwapPaletteMapper;
use image;
use log::debug;
use palette::cast::from_array;
use palette::{IntoColor, LinSrgb, Srgb};

pub struct DiffMapper {
    inner: SwapPaletteMapper,
}

impl DiffMapper {
    pub fn new(color_palette_1: Vec<LinSrgb>, color_palette_2: Vec<LinSrgb>) -> Self {
    // pub fn new(inner_swap_mapper: SwapPaletteMapper) -> Self {
        debug!(
            "DiffMapper with {:?} and {:?} creating",
            color_palette_1.len(),
            color_palette_2.len()
        );

        let inner_swap_mapper = SwapPaletteMapper::new(
            color_palette_1,
            color_palette_2
        );

        let mapper = Self {
            inner: inner_swap_mapper
        };

        debug!("DiffMapper created");

        mapper
    }
}

impl ColorMap for DiffMapper {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        self.inner.index_of(color)
    }

    fn map_color(&self, color: &mut Self::Color) {
        // let base_color = self.inner.m
        let srgb: Srgb<u8> = from_array(color.0);
        let lin_rgb: LinSrgb = srgb.into_linear();

        let indexed_color = self.inner.get_nearest_color(lin_rgb);
        let base_color = self.inner.get_color_palette_2()[indexed_color.index];

        let r_diff: f32 = lin_rgb.red - indexed_color.color.red;
        let g_diff: f32 = lin_rgb.green - indexed_color.color.green;
        let b_diff: f32 = lin_rgb.blue - indexed_color.color.blue;

        let replacement_color = LinSrgb::new(
            base_color.red + r_diff,
            base_color.green + g_diff,
            base_color.blue + b_diff,
        );

        let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
        color.0 = [srgb.red, srgb.green, srgb.blue]
    }
}