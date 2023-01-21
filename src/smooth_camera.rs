use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Reflect, Inspectable, Default)]
#[reflect(Component)]
pub struct SmoothCamera {
    pub max_distance: f32,
    pub leniency: f32,
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
        app.add_system(move_smooth_cameras);
    }
}

fn move_smooth_cameras(
    mut q_camera: Query<(&SmoothCamera, &mut Transform)>,
    q_target: Query<(&SmoothCameraTarget, &GlobalTransform)>,
) {
    let mut target_location = Vec3::ZERO;
    let mut target_count: f32 = 0.;

    for (target, transform) in q_target.iter() {
        target_location += transform.translation() * target.weight;
        target_count += target.weight;
    }

    target_location = Vec3::new(
        target_location.x / target_count,
        target_location.y / target_count,
        target_location.z / target_count,
    );

    for (cam, mut transform) in q_camera.iter_mut() {
        let difference = Vec3::new(
            target_location.x - transform.translation.x,
            target_location.y - transform.translation.y,
            // don't include z-axis
            0.,
        );

        // Move faster the further away, but have a buffer - string and spring
        if difference.length() <= cam.leniency {
            // Idea if it bugs you: use camera inertia to let it continue sliding a little if we're in
            // this if block
            return;
        }
        let step_proportion = ((difference.length() - cam.leniency).clamp(0., cam.max_distance)
            / cam.max_distance)
            .powi(2);

        transform.translation += difference * step_proportion;
    }
}
