use image::imageops::ColorMap;
use image::Rgb;
use palette::{Lab, LinSrgb};
use rstar::{Envelope, Point, PointDistance, RTreeObject, AABB};

pub use lab_color_palette::LabPaletteMapper;
pub use rgb_color_palette::RgbPaletteMapper;
pub use swap_color_mapper::SwapPaletteMapper;
pub use diff_mapper::DiffMapper;
pub use neu_quant_wrapper::NeuQuantWrapper;

mod lab_color_palette;
mod rgb_color_palette;
mod swap_color_mapper;
mod diff_mapper;
mod neu_quant_wrapper;

#[allow(unused)]
pub type RgbColorMapper = Box<dyn ColorMap<Color = Rgb<u8>>>;

/// used to storage color with its index
#[derive(Debug)]
struct IndexedColor<T> {
    index: usize,
    color: T,
}

impl RTreeObject for IndexedColor<Lab> {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.color.a, self.color.b])
    }
}

impl PointDistance for IndexedColor<Lab> {
    fn distance_2(
        &self,
        point: &<Self::Envelope as Envelope>::Point,
    ) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
        (self.color.a - point[0]).powi(2) + (self.color.b - point[1]).powi(2)
    }
}

impl RTreeObject for IndexedColor<LinSrgb> {
    type Envelope = AABB<[f32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.color.red, self.color.green, self.color.blue])
    }
}

impl PointDistance for IndexedColor<LinSrgb> {
    fn distance_2(
        &self,
        point: &<Self::Envelope as Envelope>::Point,
    ) -> <<Self::Envelope as Envelope>::Point as Point>::Scalar {
        (self.color.red - point[0]).powi(2)
            + (self.color.green - point[1]).powi(2)
            + (self.color.blue - point[2]).powi(2)
    }
}

// TODO: add ColorMapper with rgb square euclidean distance + palette bias
// TODO: add ColorMapper based on NeuQuant
