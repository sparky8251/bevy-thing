use bevy::{color::Color, prelude::Resource};

#[derive(Resource, Default)]
pub(crate) struct BiomeConfig {
    thresholds: BiomeThresholds,
    colors: BiomeColors,
}

struct BiomeThresholds {
    snow: f64,
    rock: f64,
    grass: f64,
    sand: f64,
}

struct BiomeColors {
    snow: Color,
    rock: Color,
    grass: Color,
    sand: Color,
    water: Color,
}

impl BiomeConfig {
    pub(crate) fn get_color(&self, value: f64) -> Color {
        if value >= self.thresholds.snow {
            self.colors.snow
        } else if value > self.thresholds.rock {
            self.colors.rock
        } else if value > self.thresholds.grass {
            self.colors.grass
        } else if value > self.thresholds.sand {
            self.colors.sand
        } else {
            self.colors.water
        }
    }
}

impl Default for BiomeThresholds {
    fn default() -> Self {
        Self {
            snow: 0.75,
            rock: 0.5,
            grass: 0.0,
            sand: -0.1,
        }
    }
}

impl Default for BiomeColors {
    fn default() -> Self {
        Self {
            snow: Color::srgb_u8(192, 192, 192),
            rock: Color::srgb_u8(105, 105, 105),
            grass: Color::srgb_u8(0, 100, 0),
            sand: Color::srgb_u8(244, 164, 96),
            water: Color::srgb_u8(0, 0, 255),
        }
    }
}
