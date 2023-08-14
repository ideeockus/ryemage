pub use lab_color_palette::LabPaletteMapper;
pub use rgb_color_palette::RgbPaletteMapper;
pub use swap_color_mapper::SwapPaletteMapper;

mod lab_color_palette;
mod rgb_color_palette;
mod swap_color_mapper;

/// used to storage color with its index in kdtree
#[derive(Debug)]
struct IndexedColor<T> {
    index: usize,
    color: T,
}