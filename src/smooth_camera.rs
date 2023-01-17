use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Reflect, Inspectable, Default)]
#[reflect(Component)]
pub struct SmoothCamera {}

#[derive(Component, Reflect, Inspectable, Default)]
#[reflect(Component)]
pub struct SmoothCameraTarget {}

pub struct SmoothCameraPlugin;
impl Plugin for SmoothCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_smooth_cameras);
    }
}

fn move_smooth_cameras(
    mut q_camera: Query<(&SmoothCamera, &mut Transform)>,
    q_target: Query<(&SmoothCameraTarget, &GlobalTransform)>,
    time: Res<Time>,
) {
    let mut avg_target_location = Vec3::ZERO;
    let mut target_count: f32 = 0.;

    for (_, transform) in q_target.iter() {
        avg_target_location += transform.translation();
        target_count += 1.;
    }

    avg_target_location = Vec3::new(
        avg_target_location.x / target_count,
        avg_target_location.y / target_count,
        avg_target_location.z / target_count,
    );

    for (_, mut transform) in q_camera.iter_mut() {
        transform.translation = avg_target_location;
    }
}
