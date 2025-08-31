mod biome_config;
mod utils;

use bevy::prelude::*;

use crate::{biome_config::BiomeConfig, utils::generate_image};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<BiomeConfig>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>, config: Res<BiomeConfig>) {
    commands.spawn(Camera2d);

    let image = generate_image(1280, 720, &config);
    let image_handle = images.add(image);

    commands.spawn((
        Sprite {
            image: image_handle,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
