use image::imageops::ColorMap;
use palette::cast::from_array;
use palette::{IntoColor, LinSrgb, Srgb};
use rstar::RTree;

use crate::image_processing::color_mappers::IndexedColor;

pub struct RgbPaletteMapper {
    colors_tree: RTree<IndexedColor<LinSrgb>>,
}

impl RgbPaletteMapper {
    pub fn new(colors: Vec<LinSrgb>) -> Self {
        println!("RgbPaletteMapper with {:?} created", colors);

        let indexed_lab_colors = colors
            .into_iter()
            .enumerate()
            .map(|(index, color)| IndexedColor { index, color })
            .collect();

        let color_set_tree = RTree::bulk_load(indexed_lab_colors);

        Self {
            colors_tree: color_set_tree,
        }
    }

    fn get_nearest_color(&self, color: LinSrgb) -> &IndexedColor<LinSrgb> {
        let indexed_color = self
            .colors_tree
            .nearest_neighbor(&[color.red, color.green, color.blue])
            .unwrap();

        indexed_color
    }
}

impl ColorMap for RgbPaletteMapper {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let srgb: Srgb<u8> = from_array(color.0);
        let lin_rgb: LinSrgb = srgb.into_linear();

        self.get_nearest_color(lin_rgb).index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let srgb: Srgb<u8> = from_array(color.0);
        let lin_rgb: LinSrgb = srgb.into_linear();

        let replacement_color = self.get_nearest_color(lin_rgb).color;

        let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
        color.0 = [srgb.red, srgb.green, srgb.blue]
    }
}
