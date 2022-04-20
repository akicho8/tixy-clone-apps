// use bevy::{
//     core::FixedTimestep,
//     prelude::*,
//     sprite::collide_aabb::{collide, Collision},
// };

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 0.0),
                scale: Vec3::new(120.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });
        // .insert(Paddle { speed: 500.0 })
        // .insert(Collider::Paddle);
}
