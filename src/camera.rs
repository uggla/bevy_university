use bevy::prelude::*;

use crate::{states::GameState, vessel::Player};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(
            Update,
            stick_camera_on_vessel.run_if(in_state(GameState::InGame)),
        );

        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[cfg(debug_assertions)]
fn debug_camera(
    mut camera: Query<&mut OrthographicProjection, With<Camera2d>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut camera_ortho = camera.single_mut();

    if keyboard_input.pressed(KeyCode::KeyZ) {
        camera_ortho.scale += 0.1;
    }

    if keyboard_input.pressed(KeyCode::KeyX) {
        camera_ortho.scale -= 0.1;
    }
}

fn stick_camera_on_vessel(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let player_transform = player.single();
    let mut camera_transform = camera.single_mut();
    camera_transform.translation = player_transform.translation;
}
