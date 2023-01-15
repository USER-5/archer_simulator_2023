use bevy::prelude::*;

mod config;
mod player;

use config::*;

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
        .add_startup_system(player::spawn_archer)
        .add_system(player::forward_archer)
        .add_system(player::update_archer_rotation_input)
        .add_system(player::update_archer_rotation)
        .add_system(player::rotate_archer)
        .add_system(player::rotate_archer_rear)
        .register_type::<player::Player>()
        .run();
}
