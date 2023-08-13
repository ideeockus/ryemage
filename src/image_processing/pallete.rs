use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::Index;
use image::imageops::ColorMap;
use image::Rgb;
use palette::cast::{ArraysFrom, from_array, from_component_slice};
use palette::{IntoColor, Srgb, Lab, FromColor};
use palette::color_difference::HyAb;

pub struct PaletteColorMap<T> {
    colors: Vec<T>,
}

impl PaletteColorMap<Lab> {
    pub fn new(mut colors: Vec<Lab>) -> Self {
        colors.sort_by(|left, right| left.l.partial_cmp(&right.l).unwrap_or(Ordering::Equal));

        Self {
            colors
        }
    }
}

impl ColorMap for PaletteColorMap<Lab> {
    type Color = Rgb<u8>;

    // TODO use KD-Tree instead of linear search

    fn index_of(&self, color: &Self::Color) -> usize {
        let srgb: Srgb<u8> = from_array(color.0);
        let lab= Lab::from_color(srgb.into_linear());
        // lab.hybrid_distance()

        let mut index = 0;
        let mut similarity: f32 = 1000.0;  // todo fix

        let calc_distance = |color_1: Lab, color_2: Lab| {
            f32::sqrt(
                (color_1.a - color_2.a).powi(2) +
                    (color_1.b - color_2.b).powi(2)
            )
        };

        for (i, &c) in self.colors.iter().enumerate() {
            let cur_similarity = calc_distance(c, lab);
            if cur_similarity < similarity {
                similarity = cur_similarity;
                index = i;
            }
        }

        index

        // match self.colors.binary_search(lab) {
        //     Ok(pos) => pos,
        //     Err(potential_pos) => potential_pos,
        // }
    }

    fn map_color(&self, color: &mut Self::Color) {
        let srgb: Srgb<u8> = from_array(color.0);
        let lab = Lab::from_color(srgb.into_linear());

        let index = self.index_of(color);
        let mut replacement_color = self.colors[index];
        replacement_color.l = lab.l;

        let srgb = Srgb::from_linear(replacement_color.into_color());
        color.0 = [srgb.red, srgb.green, srgb.blue]
    }
}