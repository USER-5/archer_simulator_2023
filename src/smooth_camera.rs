use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Reflect, Inspectable, Default)]
#[reflect(Component)]
pub struct SmoothCamera {
    pub speed: f32,
    pub leniency: f32,
    current_focus: Vec3,
}

impl SmoothCamera {
    pub fn new(speed: f32, leniency: f32) -> SmoothCamera {
        return SmoothCamera {
            speed,
            leniency,
            current_focus: Vec3::ZERO,
        };
    }
}

#[derive(Component, Reflect, Inspectable)]
#[reflect(Component)]
pub struct SmoothCameraTarget {
    /// How much this particular target will "pull" the camera.
    /// Setting this negative will push the camera away from the target's location
    pub weight: f32,
}

impl Default for SmoothCameraTarget {
    fn default() -> SmoothCameraTarget {
        return SmoothCameraTarget { weight: 1. };
    }
}

pub struct SmoothCameraPlugin;
impl Plugin for SmoothCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_camera_focus)
            .add_system(move_smooth_cameras);
    }
}

fn update_camera_focus(
    mut q_camera: Query<&mut SmoothCamera>,
    q_target: Query<(&SmoothCameraTarget, &GlobalTransform)>,
) {
    let mut target_location = Vec3::ZERO;
    let mut target_count: f32 = 0.;

    // weighted average of target transforms
    for (target, transform) in q_target.iter() {
        target_location += transform.translation() * target.weight;
        target_count += target.weight;
    }
    target_location = target_location / target_count;

    if let Ok(mut cam) = q_camera.get_single_mut() {
        // calculate the delta for the current focus.
        let delta = Vec3::new(
            target_location.x - cam.current_focus.x,
            target_location.y - cam.current_focus.y,
            // don't include z-axis
            0.,
        );

        // Re-assign focus if outside of leniency range
        let overshoot = delta.length() - cam.leniency;
        if overshoot >= 0. {
            cam.current_focus += delta.normalize() * overshoot;
        }
    }
}

fn move_smooth_cameras(mut q_camera: Query<(&SmoothCamera, &mut Transform)>, time: Res<Time>) {
    for (cam, mut transform) in q_camera.iter_mut() {
        // determine how far we have to move in total to the target
        let move_diff = Vec3::new(
            cam.current_focus.x - transform.translation.x,
            cam.current_focus.y - transform.translation.y,
            0.,
        );
        // move in direction by proportional amount
        transform.translation += move_diff * move_diff.length() * cam.speed * time.delta_seconds();
    }
}
// let step_proportion = (move_diff.length().clamp(0., cam.max_distance) / cam.max_distance).powi(2) * time.delta_seconds();
