use std::cmp::Ordering;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Index;

use image::imageops::ColorMap;
use palette::{FromColor, IntoColor, Lab, LinSrgb, Srgb};
use palette::cast::{ArraysFrom, from_array, from_component_slice};
use palette::color_difference::{EuclideanDistance, HyAb};
use palette::rgb::Rgb;
use rstar::{AABB, Envelope, Point, PointDistance, RTree, RTreeObject};

use crate::image_processing::color_mappers::IndexedColor;

// impl RTreeObject for IndexedColor<LinSrgb> {
//     type Envelope = AABB<[f32; 3]>;
//
//     fn envelope(&self) -> Self::Envelope {
//         AABB::from_point([self.color.red, self.color.green, self.color.blue])
//     }
// }
//
// impl PointDistance for IndexedColor<LinSrgb> {
//     fn distance_2(&self, point: &<Self::Envelope as Envelope>::Point) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
//         (self.color.red - point[0]).powi(2) +
//             (self.color.green - point[1]).powi(2) +
//             (self.color.blue - point[2]).powi(2)
//     }
// }

/// ColorMap that finds most similar color in color_palette_1 and then
/// maps it to corresponding color from color_palette_2 (by index)
// pub struct SwapPaletteMapper<'color_mapper> {
//     color_map_1: &'color_mapper dyn ColorMap<Color=image::Rgb<u8>>,
//     color_map_2: &'color_mapper dyn ColorMap<Color=image::Rgb<u8>>,
// }

pub struct SwapPaletteMapper {
    color_palette_1: RTree<IndexedColor<LinSrgb>>,
    color_palette_2: Vec<LinSrgb>,
}

impl SwapPaletteMapper {
    // TODO return Result (?)
    pub fn new(color_palette_1: Vec<LinSrgb>, color_palette_2: Vec<LinSrgb>) -> Option<Self>
    {
        println!("SwapPaletteMapper with {:?} and {:?} created", color_palette_1.len(), color_palette_2.len());
        if color_palette_1.len() != color_palette_2.len() {
            return None
        }

        let indexed_lab_colors = color_palette_1
            .into_iter().enumerate()
            .map(|(index, color)| {
                IndexedColor { index, color }
            })
            .collect();

        let mut color_set_tree = RTree::bulk_load(indexed_lab_colors);

        Some(Self {
            color_palette_1: color_set_tree,
            color_palette_2,
        })
    }

    fn get_nearest_color(&self, color: LinSrgb) -> &IndexedColor<LinSrgb> {
        let indexed_color = self.color_palette_1.nearest_neighbor(&[
            color.red,
            color.green,
            color.blue,
        ]).unwrap();

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

// pub struct SwapPaletteMapper<CM> {
//     color_map_1: CM,
//     color_map_2: CM,
// }
//
// impl<CM> SwapPaletteMapper<CM>
//     where
//         CM: ColorMap<Color=image::Rgb<u8>>
// {
//     pub fn new(color_map_1: CM, color_map_2: CM) -> Self
//     {
//         println!("SwapPaletteMapper with {:?} created", colors);
//         Self {
//             color_map_1,
//             color_map_2,
//         }
//     }
// }
//
// impl<CM> ColorMap for SwapPaletteMapper<CM>
//     where
//         CM: ColorMap<Color=image::Rgb<u8>>
// {
//     type Color = image::Rgb<u8>;
//
//     fn index_of(&self, color: &Self::Color) -> usize {
//         self.color_map_1.index_of(color)
//     }
//
//     fn map_color(&self, color: &mut Self::Color) {
//
//
//         self.color_map_2.map_color(color);
//
//
//         let srgb: Srgb<u8> = from_array(color.0);
//         let lin_rgb: LinSrgb = srgb.into_linear();
//
//         let mut replacement_color = self.get_nearest_color(lin_rgb).color;
//
//         let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
//         color.0 = [srgb.red, srgb.green, srgb.blue]
//     }
// }
