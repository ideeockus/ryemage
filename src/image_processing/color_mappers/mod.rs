pub use lab_color_palette::LabPaletteMapper;
pub use rgb_color_palette::RgbPaletteMapper;

mod lab_color_palette;
mod rgb_color_palette;

/// used to storage color with its index in kdtree
#[derive(Debug)]
struct IndexedColor<T> {
    index: usize,
    color: T,
}