use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_rapier2d::prelude::*;

use crate::smooth_camera::SmoothCameraTarget;

#[derive(Reflect, Inspectable, Default)]
pub enum Direction {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Component, Reflect, Inspectable, Default)]
#[reflect(Component)]
pub struct Player {
    move_speed: f32,
    rotate_speed: f32,
    target_rotation: Direction,
    current_rotation: f32,
    max_rotation_angle: f32,
    wiggle_magnitude: f32,
}

pub fn spawn_archer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Name::new("Player"))
        .insert(SpriteBundle {
            texture: asset_server.load("archer_head.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(1.)),
                ..default()
            },
            ..default()
        })
        .insert(TransformBundle::from_transform(
            // archer's head is 0.5x0.5 square
            Transform::default().with_scale(Vec3::splat(0.5)),
        ))
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(Player {
            move_speed: 7.0,
            rotate_speed: 200.0,
            target_rotation: Direction::None,
            current_rotation: 0.,
            max_rotation_angle: 8.,
            wiggle_magnitude: 0.7,
        })
        .insert(SmoothCameraTarget::default())
        .with_children(|child_builder| spawn_archer_rear(child_builder, asset_server));
}

fn spawn_archer_rear(builder: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    builder
        .spawn(SpatialBundle::from_transform(
            Transform::default().with_translation(Vec3::new(0., -0.5, 0.)),
        ))
        .with_children(|builder| {
            builder
                .spawn(Name::new("Player Trail"))
                .insert(SpriteBundle {
                    texture: asset_server.load("archer_rear.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1.)),
                        ..default()
                    },
                    ..default()
                })
                .insert(TransformBundle::from_transform(
                    Transform::default().with_translation(Vec3::new(0., -0.5, 0.)),
                ))
                .insert(Collider::cuboid(0.5, 0.5));
        });
}

pub fn forward_archer(mut query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
    for (player, mut transform) in query.iter_mut() {
        let forward_direciton = transform.up();
        transform.translation += forward_direciton * time.delta_seconds() * player.move_speed;
    }
}

pub fn update_archer_rotation_input(mut query: Query<&mut Player>, input: Res<Input<KeyCode>>) {
    for mut player in query.iter_mut() {
        // notably, once you've set this direciton, you can't unset it
        if input.pressed(KeyCode::Left) {
            player.target_rotation = Direction::Left;
        } else if input.pressed(KeyCode::Right) {
            player.target_rotation = Direction::Right;
        }
    }
}

pub fn update_archer_rotation(mut query: Query<&mut Player>, time: Res<Time>) {
    for mut player in query.iter_mut() {
        // increase current rotation angle toward target angle, but never beyond max rotation angle
        player.current_rotation += (match player.target_rotation {
            Direction::Left => player.rotate_speed,
            Direction::Right => -player.rotate_speed,
            _ => 0.,
        } * time.delta_seconds());
        player.current_rotation = player
            .current_rotation
            .clamp(-player.max_rotation_angle, player.max_rotation_angle);
    }
}

pub fn rotate_archer(mut query: Query<(&Player, &mut Transform)>, time: Res<Time>) {
    for (player, mut transform) in query.iter_mut() {
        transform.rotate_local_z(player.current_rotation * time.delta_seconds());
    }
}

pub fn rotate_archer_rear(
    q_archer: Query<(&Children, &Player)>,
    mut q_archer_butt: Query<&mut Transform>,
) {
    for (children, player) in q_archer.iter() {
        for &child in children.iter() {
            if let Ok(mut rear_transform) = q_archer_butt.get_mut(child) {
                let rear_rotation_amount = -player.current_rotation / player.max_rotation_angle;
                rear_transform.rotation =
                    Quat::from_rotation_z(player.wiggle_magnitude * rear_rotation_amount);
            }
        }
    }
}
