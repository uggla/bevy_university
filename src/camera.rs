use bevy::prelude::*;

use crate::states::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_camera);

        #[cfg(debug_assertions)]
        app.add_systems(Update, debug_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

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
