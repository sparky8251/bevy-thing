use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    biome_config::BiomeConfig,
    noise::{Noise, NoiseConfig},
};

pub(crate) fn generate_image(width: u32, height: u32, biome_config: &BiomeConfig, noise_config: &NoiseConfig) -> Image {
    let image_size = Extent3d {
        width,
        height,
        ..default()
    };

    let noise = {
        let mut n = Noise::new(image_size.width, image_size.height, noise_config);
        n.normalize();
        n
    };

    let mut pixels = vec![0; (image_size.width * image_size.height * 4) as usize];

    for (i, &value) in noise.values.iter().enumerate() {
        let index = i * 4;

        let color = biome_config.get_color(value);
        let color_bytes = srgba_as_bytes(&color.to_srgba());

        pixels[index..index + 4].copy_from_slice(&color_bytes);
    }

    Image::new(
        image_size,
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::all(),
    )
}

fn srgba_as_bytes(srgba: &Srgba) -> [u8; 4] {
    let r = (srgba.red * 255.0) as u8;
    let g = (srgba.green * 255.0) as u8;
    let b = (srgba.blue * 255.0) as u8;
    let a = (srgba.alpha * 255.0) as u8;

    [r, g, b, a]
}
