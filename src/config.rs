use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

const ZOOM: f32 = 15.;

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
    camera.transform.scale = Vec3::new(1./ZOOM,1./ZOOM,1.);
    camera.projection.scaling_mode = ScalingMode::Auto {
        min_width: 10.,
        min_height: 200.,
    };
    commands.spawn(camera);
}

const CLEAR: Color = Color::DARK_GRAY;