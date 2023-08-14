// use std::cmp::Ordering;
// use std::fmt::Debug;
// use std::marker::PhantomData;
// use std::ops::Index;
//
// use image::imageops::ColorMap;
// use palette::{FromColor, IntoColor, Lab, LinSrgb, Srgb};
// use palette::cast::{ArraysFrom, from_array, from_component_slice};
// use palette::color_difference::{EuclideanDistance, HyAb};
// use palette::rgb::Rgb;
//
// pub struct PaletteColorMap<T> {
//     colors: Vec<T>,
// }
//
// impl<T> PaletteColorMap<T> {
//     pub fn new(colors: Vec<T>) -> Self
//         where T: Debug
//     {
//         println!("PaletteColorMap with {:?} created", colors);
//
//         Self {
//             colors
//         }
//     }
// }
//
// impl ColorMap for PaletteColorMap<Lab> {
//     type Color = image::Rgb<u8>;
//
//     // TODO use KD-Tree instead of linear search
//
//     fn index_of(&self, color: &Self::Color) -> usize {
//         let srgb: Srgb<u8> = from_array(color.0);
//         let lab = Lab::from_color(srgb.into_linear());
//
//         let mut index = 0;
//         let mut similarity: f32 = 1000.0;  // todo fix
//
//         // calculate only chroma difference
//         let calc_distance = |color_1: Lab, color_2: Lab| {
//             f32::sqrt(
//                 (color_1.a - color_2.a).powi(2) +
//                     (color_1.b - color_2.b).powi(2)
//             )
//         };
//
//         for (i, &c) in self.colors.iter().enumerate() {
//             let cur_similarity = calc_distance(c, lab);
//             if cur_similarity < similarity {
//                 similarity = cur_similarity;
//                 index = i;
//             }
//         }
//
//         index
//     }
//
//     fn lookup(&self, index: usize) -> Option<Self::Color> {
//         self.colors.get(index).map(|&color| {
//             let srgb: Srgb<u8> = Srgb::from_format(color.into_color());
//             image::Rgb([srgb.red, srgb.green, srgb.blue])
//         })
//     }
//
//     fn has_lookup(&self) -> bool {
//         true
//     }
//
//     fn map_color(&self, color: &mut Self::Color) {
//         let srgb: Srgb<u8> = from_array(color.0);
//         let lab = Lab::from_color(srgb.into_linear());
//
//         let index = self.index_of(color);
//         let mut replacement_color = self.colors[index];
//         replacement_color.l = lab.l;
//
//         // let srgb = Srgb::from_linear(replacement_color.into_color()); // TODO check why linear here
//         let srgb: Srgb<u8> = Srgb::from_format(replacement_color.into_color());
//         color.0 = [srgb.red, srgb.green, srgb.blue]
//     }
// }
//
// impl ColorMap for PaletteColorMap<LinSrgb> {
//     type Color = image::Rgb<u8>;
//
//     // TODO use KD-Tree instead of linear search
//
//     fn index_of(&self, color: &Self::Color) -> usize {
//         let rgb: Srgb<u8> = from_array(color.0);
//         let rgb: LinSrgb = rgb.into_linear();
//
//         let mut index = 0;
//         let mut similarity: f32 = 1000.0;  // todo fix
//
//         let calc_distance = |color_1: LinSrgb, color_2: LinSrgb| {
//             f32::sqrt(  // TODO replace with squared euclidean ?
//                         (color_1.red - color_2.red).powi(2) +
//                             (color_1.green - color_2.green).powi(2) +
//                             (color_1.blue - color_2.blue).powi(2)
//             )
//         };
//
//         for (i, &c) in self.colors.iter().enumerate() {
//             let cur_similarity = calc_distance(c, rgb);
//             if cur_similarity < similarity {
//                 similarity = cur_similarity;
//                 index = i;
//             }
//         }
//
//         index
//     }
//
//     fn lookup(&self, index: usize) -> Option<Self::Color> {
//         self.colors.get(index).map(|&color| {
//             let srgb: Srgb<u8> = Srgb::from_linear(color);
//             image::Rgb([srgb.red, srgb.green, srgb.blue])
//         })
//     }
//
//     fn has_lookup(&self) -> bool {
//         true
//     }
//
//     fn map_color(&self, color: &mut Self::Color) {
//         let index = self.index_of(color);
//         let lin_rgb = self.colors[index];
//         let srgb: Srgb<u8> = Srgb::from_linear(lin_rgb);
//
//         color.0 = [srgb.red, srgb.green, srgb.blue]
//     }
// }
//
//
// pub trait LimitedColorSet {
//     fn color_set_size(&self) -> usize;
// }
//
// impl<T> LimitedColorSet for PaletteColorMap<T> {
//     fn color_set_size(&self) -> usize {
//         self.colors.len()
//     }
// }