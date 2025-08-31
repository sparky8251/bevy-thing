mod biome_config;
mod noise;
mod utils;

use bevy::prelude::*;
use rand::Rng;

use crate::{biome_config::BiomeConfig, noise::NoiseConfig, utils::generate_image};

#[derive(Component)]
struct Map;

#[derive(Event, Debug)]
struct RegenerateMapEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<BiomeConfig>()
        .init_resource::<NoiseConfig>()
        .add_event::<RegenerateMapEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_input,
                regenerate_map.run_if(on_event::<RegenerateMapEvent>).chain(),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    biome_config: Res<BiomeConfig>,
    noise_config: Res<NoiseConfig>,
) {
    commands.spawn(Camera2d);

    let image = generate_image(1280, 720, &biome_config, &noise_config);
    let image_handle = images.add(image);

    commands.spawn((
        Sprite {
            image: image_handle,
            ..default()
        },
        Map,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<NoiseConfig>,
    mut event_writer: EventWriter<RegenerateMapEvent>,
) {
    let mut regenerate_map = false;

    if keys.just_pressed(KeyCode::KeyR) {
        config.seed = rand::rng().random();
        info!("New seed: {:}", config.seed);
        regenerate_map = true;
    }

    if keys.just_pressed(KeyCode::KeyF) {
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            config.frequency -= 0.001;
        } else {
            config.frequency += 0.001;
        }
        info!("New frequency: {:.3}", config.frequency);
        regenerate_map = true;
    }

    if keys.just_pressed(KeyCode::KeyO) {
        if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
            config.octaves = (config.octaves - 1).max(1);
        } else {
            config.octaves = (config.octaves + 1).min(10);
        }
        info!("New octaves: {}", config.octaves);
        regenerate_map = true;
    }

    if regenerate_map {
        event_writer.write(RegenerateMapEvent);
    }
}

fn regenerate_map(
    noise_config: Res<NoiseConfig>,
    biome_config: Res<BiomeConfig>,
    mut images: ResMut<Assets<Image>>,
    q_map: Query<&Sprite, With<Map>>,
) {
    if let Ok(sprite) = q_map.single() {
        if let Some(image) = images.get_mut(&sprite.image) {
            let new_image = generate_image(
                image.texture_descriptor.size.width,
                image.texture_descriptor.size.height,
                &biome_config,
                &noise_config,
            );
            *image = new_image;
        }
    }
}
