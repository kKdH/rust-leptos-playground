mod pie;
mod bar;

pub use bar::{BarChart,  Dataset, Record};
pub use pie::{PieChart, PieChartData, PieChartItem};

const DEFAULT_COLOR_PALLET: [u32; 16] = [0xf94144, 0xf3722c, 0xf8961e, 0xf9844a, 0xf9c74f, 0x90be6d, 0x43aa8b, 0x4d908e, 0x577590, 0x277da1, 0x2d3057, 0x2b2d42, 0x333a56, 0x4e6e58, 0xbcbd8b, 0xaa5042];
const MATERIAL_COLOR_PALLET: [u32; 16] = [0xf44336, 0xe81e63, 0x9c27b0, 0x673ab7, 0x3f51b5, 0x2196f3, 0x03a9f4, 0x00bcd4, 0x009688, 0x4caf50, 0x8bc34a, 0xcddc39, 0xffeb3b, 0xffc107, 0xff9800, 0xff5722];
const DARK_BLUE_COLOR_PALLET: [u32; 16] = [0x001233, 0x001845, 0x012a4a, 0x013a63, 0x01497c, 0x014f86, 0x2a6f97, 0x2c7da0, 0x468faf, 0x61a5c2, 0x89c2d9, 0xa9d6e5, 0x90e0ef, 0xade8f4, 0xcaf0f8, 0x0466c8];
const ORANGE_FIRE_COLOR_PALLET: [u32; 16] = [0xfc2f00, 0xff4800, 0xff5400, 0xff6000, 0xff6d00, 0xff7900, 0xff7b00, 0xff8800, 0xff9500, 0xffa200, 0xffaa00, 0xffb700, 0xffc300, 0xffd000, 0xffdd00, 0xffea00];

#[derive(Clone, Copy)]
pub enum ColorPallet {
    Default,
    OrangeFire,
    DarkBlue,
    Material,
}

impl ColorPallet {

    pub fn colors(&self) -> &'static [u32; 16] {
        match self {
            ColorPallet::Default => &DEFAULT_COLOR_PALLET,
            ColorPallet::OrangeFire => &ORANGE_FIRE_COLOR_PALLET,
            ColorPallet::DarkBlue => &DARK_BLUE_COLOR_PALLET,
            ColorPallet::Material => &MATERIAL_COLOR_PALLET,
        }
    }
}

pub trait IntoColorCode {
    fn into_color_code(self) -> String;
}

impl IntoColorCode for u32 {
    fn into_color_code(self) -> String {
        format!("#{:06x}", self)
    }
}
