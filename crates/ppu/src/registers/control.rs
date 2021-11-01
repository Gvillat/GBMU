use modular_bitfield::{bitfield, specifiers::B2};

/// LCDC is the main LCD Control register.
/// Its bits toggle what elements are displayed on the screen, and how.

/// Bit	 Name                           Usage notes
/// 7	 LCD and PPU enable             0=Off, 1=On
/// 6	 Window tile map area           0=9800-9BFF, 1=9C00-9FFF
/// 5	 Window enable                  0=Off, 1=On
/// 4	 BG and Window tile data area   0=8800-97FF, 1=8000-8FFF
/// 3	 BG tile map area               0=9800-9BFF, 1=9C00-9FFF
/// 2	 OBJ size                       0=8x8, 1=8x16
/// 1	 OBJ enable                     0=Off, 1=On
/// 0	 BG and Window enable/priority	0=Off, 1=On

#[bitfield]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Control {
    pub(super) priority: bool,
    pub(super) obj_enabled: bool,
    pub(super) obj_size: bool,
    pub(super) bg_area: bool,
    pub(super) tile_data: bool,
    pub(super) window_enabled: bool,
    pub(super) window_area: bool,
    pub(super) enabled: bool,
}
