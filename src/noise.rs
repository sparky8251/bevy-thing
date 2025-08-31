use bevy::prelude::*;
use noise_lib::{Fbm, MultiFractal, NoiseFn, Simplex};

pub(crate) struct Noise {
    pub(crate) values: Vec<f64>,
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Noise {
    pub(crate) fn new(width: u32, height: u32, config: &NoiseConfig) -> Self {
        let fbm = Fbm::<Simplex>::new(config.seed);
        let fbm = fbm.set_frequency(config.frequency).set_octaves(config.octaves);

        let mut values = vec![0.0; (width * height) as usize];
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let value = fbm.get([x as f64, y as f64]);
                values[index] = value;

                if value < min {
                    min = value;
                }

                if value > max {
                    max = value;
                }
            }
        }

        Noise { values, min, max }
    }

    pub(crate) fn normalize(&mut self) {
        let range = self.max - self.min;
        for value in self.values.iter_mut() {
            *value = -1.0 + 2.0 * (*value - self.min) / range;
        }
    }
}

#[derive(Resource)]
pub(crate) struct NoiseConfig {
    pub(crate) seed: u32,
    pub(crate) frequency: f64,
    pub(crate) octaves: usize,
}

impl Default for NoiseConfig {
    fn default() -> Self {
        Self {
            seed: 0,
            frequency: 0.01,
            octaves: 4,
        }
    }
}
