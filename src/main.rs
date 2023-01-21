use bevy::prelude::*;

mod config;
mod player;
mod smooth_camera;

use config::*;
use smooth_camera::SmoothCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1920.,
                height: 1080.,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ConfigPlugin)
        // comment / uncomment as needed
        // .add_plugin(DebugConfigPlugin)
        .add_plugin(SmoothCameraPlugin)
        .add_startup_system(floor_texture)
        .add_startup_system(player::spawn_archer)
        .add_system(player::forward_archer)
        .add_system(player::update_archer_rotation_input)
        .add_system(player::update_archer_rotation)
        .add_system(player::rotate_archer)
        .add_system(player::rotate_archer_rear)
        .register_type::<player::Player>()
        .run();
}

fn floor_texture(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("checkers.png"),
        ..default()
    });
}
