use crate::arm9_bindings::{_ext_palette, _palette};

/// background graphics memory pointer
pub const BG_GFX: *mut u16 = 0x6000000 as *mut u16;

/// background graphics memory (sub engine)
pub const BG_GFX_SUB: *mut u16 = 0x6200000 as *mut u16;

/// background palette memory
pub const BG_PALETTE: *mut u16 = 0x05000000 as *mut u16;

/// background palette memory (sub engine)
pub const BG_PALETTE_SUB: *mut u16 = 0x05000400 as *mut u16;

/// pointer to Object Attribute Memory
pub const OAM: *mut u16 = 0x07000000 as *mut u16;

/// pointer to Object Attribute Memory (Sub engine)
pub const OAM_SUB: *mut u16 = 0x07000400 as *mut u16;

/// sprite graphics memory
pub const SPRITE_GFX: *mut u16 = 0x6400000 as *mut u16;

/// sprite graphics memory (sub engine)
pub const SPRITE_GFX_SUB: *mut u16 = 0x6600000 as *mut u16;

/// sprite palette memory
pub const SPRITE_PALETTE: *mut u16 = 0x05000200 as *mut u16;

/// sprite palette memory (sub engine)
pub const SPRITE_PALETTE_SUB: *mut u16 = 0x05000600 as *mut u16;

/// pointer to vram bank A mapped as LCD
pub const VRAM_A: *mut u16 = 0x6800000 as *mut u16;

/// pointer to vram bank B mapped as LCD
pub const VRAM_B: *mut u16 = 0x6820000 as *mut u16;

/// pointer to vram bank C mapped as LCD
pub const VRAM_C: *mut u16 = 0x6840000 as *mut u16;

/// pointer to vram bank D mapped as LCD
pub const VRAM_D: *mut u16 = 0x6860000 as *mut u16;

/// pointer to vram bank E mapped as LCD
pub const VRAM_E: *mut u16 = 0x6880000 as *mut u16;

/// Used for accessing vram E as an extended palette.
pub const VRAM_E_EXT_PALETTE: *const _ext_palette = VRAM_E as *const _ext_palette;

/// pointer to vram bank F mapped as LCD
pub const VRAM_F: *mut u16 = 0x6890000 as *mut u16;

/// Used for accessing vram F as an extended palette.
pub const VRAM_F_EXT_PALETTE: *const _ext_palette = VRAM_F as *const _ext_palette;

/// Used for accessing vram F as an extended sprite palette.
pub const VRAM_F_EXT_SPR_PALETTE: *const _palette = VRAM_F as *const _palette;

/// pointer to vram bank G mapped as LCD
pub const VRAM_G: *mut u16 = 0x6894000 as *mut u16;

/// Used for accessing vram G as an extended palette.
pub const VRAM_G_EXT_PALETTE: *const _ext_palette = VRAM_G as *const _ext_palette;

/// Used for accessing vram G as an extended sprite palette.
pub const VRAM_G_EXT_SPR_PALETTE: *const _palette = VRAM_G as *const _palette;

/// pointer to vram bank H mapped as LCD
pub const VRAM_H: *mut u16 = 0x6898000 as *mut u16;

/// Used for accessing vram H as an extended palette.
pub const VRAM_H_EXT_PALETTE: *const _ext_palette = VRAM_H as *const _ext_palette;

/// pointer to vram bank I mapped as LCD
pub const VRAM_I: *mut u16 = 0x68A0000 as *mut u16;

/// Used for accessing vram H as an extended palette.
pub const VRAM_I_EXT_PALETTE: *const _ext_palette = VRAM_I as *const _ext_palette;

#[macro_export]
macro_rules! argb16 {
    ($a:expr, $r:expr, $g:expr, $b:expr) => {
        ((($a) << 15) | ($r) | (($g) << 5) | (($b) << 10))
    };
}

#[macro_export]
macro_rules! rgb15 {
    ($r:expr, $g:expr, $b:expr) => {
        (($r) | (($g) << 5) | (($b) << 10))
    };
}
