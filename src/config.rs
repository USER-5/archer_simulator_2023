use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use crate::smooth_camera::SmoothCamera;

const ZOOM: f32 = 18.;
const CLEAR: Color = Color::DARK_GRAY;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR))
            .add_startup_system(setup_camera)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
    }
}

pub struct DebugConfigPlugin;
impl Plugin for DebugConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(WorldInspectorPlugin::new());
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.scale = Vec3::new(1. / ZOOM, 1. / ZOOM, 1.);
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 10.,
        min_height: 200.,
    };
    commands.spawn(camera).insert(SmoothCamera::new(5., 2.5));
}
