mod asteroids;
mod camera;
mod states;
mod vessel;

use asteroids::AsteroidsPlugin;
use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use camera::CameraPlugin;
use states::StatesPlugin;
use vessel::{VesselPlugin, VESSEL_WIDTH};

// 16/9 1280x720
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Resource, Debug)]
struct CurrentLevel(u8);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy University".to_string(),
                    resizable: false,
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            StatesPlugin,
            CameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(VESSEL_WIDTH * 0.5 / 5.0),
            RapierDebugRenderPlugin::default(),
            AsteroidsPlugin,
            VesselPlugin,
        ))
        .insert_resource(CurrentLevel(0))
        .run();
}
