use level::Level;
use types::{SectorType, WadSector, LightLevel};

#[derive(PartialEq, Clone)]
pub struct LightInfo {
    pub level: f32,
    pub effect: Option<LightEffect>,
}

#[derive(PartialEq, Clone)]
pub struct LightEffect {
    pub alt_level: f32,
    pub speed: f32,
    pub duration: f32,
    pub sync: f32,
    pub kind: LightEffectKind,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum LightEffectKind {
    Glow,
    Random,
    Alternate,
}

pub fn new_light(level: &Level, sector: &WadSector, contrast: Option<Contrast>) -> LightInfo {
    use LightEffectKind::*;
    let base_level = light_to_f32(sector.light, contrast);
    let alt_level = match sector.sector_type {
        FLASH | FAST_STROBE_1 | FAST_STROBE_2 | FAST_STROBE_SYNC | SLOW_STROBE |
        SLOW_STROBE_SYNC | GLOW | FLICKER => {
            let alt_level = light_to_f32(level.sector_min_light(sector), contrast);
            if alt_level == base_level {
                return LightInfo { level: base_level, effect: None };
            } else {
                alt_level
            }
        },
        _ => return LightInfo { level: base_level, effect: None },
    };
    let sync = match sector.sector_type {
        SLOW_STROBE_SYNC | FAST_STROBE_SYNC | GLOW => 0.0,
        _ => id_to_sync(level.sector_id(sector)),
    };
    let (kind, speed, duration) = match sector.sector_type {
        FLASH =>
            (Random, FLASH_SPEED, FLASH_DURATION),
        FLICKER =>
            (Random, FLICKER_SPEED, FLICKER_DURATION),
        SLOW_STROBE | SLOW_STROBE_SYNC =>
            (Alternate, SLOW_STROBE_SPEED, SLOW_STROBE_DURATION),
        FAST_STROBE_1 | FAST_STROBE_2 | FAST_STROBE_SYNC =>
            (Alternate, FAST_STROBE_SPEED, FAST_STROBE_DURATION),
        GLOW =>
            (Glow, GLOW_SPEED, 0.0),
        _ => unreachable!(),
    };
    LightInfo {
        level: base_level,
        effect: Some(LightEffect {
                         alt_level: alt_level,
                         kind: kind,
                         speed: speed,
                         duration: duration,
                         sync: sync
                     }),
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Contrast {
    Darken,
    Brighten,
}

fn id_to_sync(id: u16) -> f32 {
    ((id as u64 * 1664525 + 1013904223) & 0xffff) as f32 / 15.0
}

fn light_to_f32(level: LightLevel, contrast: Option<Contrast>) -> f32 {
    let with_contrast = match contrast {
        Some(Contrast::Darken) => if level <= 16 { 0 } else { level - 16 },
        Some(Contrast::Brighten) => if level >= LightLevel::max_value() - 16 {
                                        LightLevel::max_value()
                                    } else {
                                        level + 16
                                    },
        None => level,
    };

    (with_contrast >> 3) as f32 / 31.0
}

const FLASH_SPEED: f32 = 20.0;
const FLASH_DURATION: f32 = 0.06;
const FLICKER_SPEED: f32 = 8.0;
const FLICKER_DURATION: f32 = 0.5;
const SLOW_STROBE_SPEED: f32 = 1.0;
const SLOW_STROBE_DURATION: f32 = 0.85;
const FAST_STROBE_SPEED: f32 = 2.0;
const FAST_STROBE_DURATION: f32 = 0.7;
const GLOW_SPEED: f32 = 0.5;

const FLASH: SectorType = 1;
const FAST_STROBE_1: SectorType = 2;
const FAST_STROBE_2: SectorType = 4;
const FAST_STROBE_SYNC: SectorType = 13;
const SLOW_STROBE: SectorType = 3;
const SLOW_STROBE_SYNC: SectorType = 12;
const GLOW: SectorType = 8;
const FLICKER: SectorType = 17;
