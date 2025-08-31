use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::biome_config::BiomeConfig;
use noise::{Fbm, MultiFractal, NoiseFn, Simplex};

struct Noise {
    values: Vec<f64>,
    min: f64,
    max: f64,
}

impl Noise {
    fn normalize(&mut self) {
        let range = self.max - self.min;
        for value in self.values.iter_mut() {
            *value = -1.0 + 2.0 * (*value - self.min) / range;
        }
    }
}

pub(super) fn generate_image(width: u32, height: u32, config: &BiomeConfig) -> Image {
    let image_size = Extent3d {
        width,
        height,
        ..default()
    };

    let noise = {
        let mut n = generate_noise(image_size.width as usize, image_size.height as usize);
        n.normalize();
        n
    };

    let mut pixels = vec![0; (image_size.width * image_size.height * 4) as usize];

    for (i, &value) in noise.values.iter().enumerate() {
        let index = i * 4;

        let color = config.get_color(value);
        let color_bytes = srgba_as_bytes(&color.to_srgba());

        pixels[index..index + 4].copy_from_slice(&color_bytes);
    }

    Image::new(
        image_size,
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn generate_noise(width: usize, height: usize) -> Noise {
    let fbm = Fbm::<Simplex>::new(0);
    let fbm = fbm.set_frequency(0.01).set_octaves(4);

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

fn srgba_as_bytes(srgba: &Srgba) -> [u8; 4] {
    let r = (srgba.red * 255.0) as u8;
    let g = (srgba.green * 255.0) as u8;
    let b = (srgba.blue * 255.0) as u8;
    let a = (srgba.alpha * 255.0) as u8;

    [r, g, b, a]
}
