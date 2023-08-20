use image::imageops::ColorMap;
use palette::{FromColor, IntoColor, Lab, Srgb};
use palette::cast::from_array;
use rstar::RTree;

use crate::color_mappers::IndexedColor;

pub struct LabPaletteMapper {
    colors_tree: RTree<IndexedColor<Lab>>,
}

impl LabPaletteMapper {
    pub fn new(colors: Vec<Lab>) -> Self {
        println!("PaletteColorMap with {:?} created", colors);

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

    fn get_nearest_color(&self, color_lab: Lab) -> &IndexedColor<Lab> {
        let indexed_color = self
            .colors_tree
            .nearest_neighbor(&[color_lab.a, color_lab.b])
            .unwrap();

        indexed_color
    }
}

impl ColorMap for LabPaletteMapper {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let srgb: Srgb<u8> = from_array(color.0);
        let color_lab = Lab::from_color(srgb.into_linear());

        self.get_nearest_color(color_lab).index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let srgb: Srgb<u8> = from_array(color.0);
        let color_lab = Lab::from_color(srgb.into_linear());

        let mut replacement_color = self.get_nearest_color(color_lab).color;
        replacement_color.l = color_lab.l;

        let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
        color.0 = [srgb.red, srgb.green, srgb.blue]
    }
}
