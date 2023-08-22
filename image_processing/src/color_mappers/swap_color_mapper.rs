use image::imageops::ColorMap;
use log::debug;
use palette::cast::from_array;
use palette::{IntoColor, LinSrgb, Srgb};
use rstar::RTree;

use crate::color_mappers::IndexedColor;

pub struct SwapPaletteMapper {
    color_palette_1: RTree<IndexedColor<LinSrgb>>,
    color_palette_2: Vec<LinSrgb>,
}

impl SwapPaletteMapper {
    pub fn new(color_palette_1: Vec<LinSrgb>, color_palette_2: Vec<LinSrgb>) -> Option<Self> {
        debug!(
            "SwapPaletteMapper with {:?} and {:?} creating",
            color_palette_1.len(),
            color_palette_2.len()
        );
        // if color_palette_1.len() != color_palette_2.len() {
        //     return None
        // }

        // rebuild one palette if sizes not equal
        if color_palette_1.len() > color_palette_2.len() {
            let mul = color_palette_1.len() as f32 / color_palette_2.len() as f32;

            let mut new_palette_1 = Vec::with_capacity(color_palette_2.len());

            let mut i = 0f32;
            while i < color_palette_1.len() as f32 {
                new_palette_1.push(color_palette_1[i.floor() as usize]);
                i += mul;
            }
        }
        if color_palette_2.len() > color_palette_1.len() {
            let mul = color_palette_2.len() as f32 / color_palette_1.len() as f32;

            let mut new_palette_2 = Vec::with_capacity(color_palette_1.len());

            let mut i = 0f32;
            while i < color_palette_2.len() as f32 {
                new_palette_2.push(color_palette_2[i.floor() as usize]);
                i += mul;
            }
        }

        let indexed_lab_colors = color_palette_1
            .into_iter()
            .enumerate()
            .map(|(index, color)| IndexedColor { index, color })
            .collect();

        let color_set_tree = RTree::bulk_load(indexed_lab_colors);
        debug!("SwapPaletteMapper created");

        Some(Self {
            color_palette_1: color_set_tree,
            color_palette_2,
        })
    }

    fn get_nearest_color(&self, color: LinSrgb) -> &IndexedColor<LinSrgb> {
        let indexed_color = self
            .color_palette_1
            .nearest_neighbor(&[color.red, color.green, color.blue])
            .unwrap();

        indexed_color
    }
}

impl ColorMap for SwapPaletteMapper {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let srgb: Srgb<u8> = from_array(color.0);
        let lin_rgb: LinSrgb = srgb.into_linear();

        self.get_nearest_color(lin_rgb).index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let srgb: Srgb<u8> = from_array(color.0);
        let lin_rgb: LinSrgb = srgb.into_linear();

        let color_palette_1_index = self.get_nearest_color(lin_rgb).index;
        let replacement_color = self.color_palette_2[color_palette_1_index];

        let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
        color.0 = [srgb.red, srgb.green, srgb.blue]
    }
}
