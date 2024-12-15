mod states;

use std::f32::consts::PI;

use bevy::{prelude::*, window::WindowResolution};
use states::{GameState, StatesPlugin};

// 16/9 1280x720
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Component)]
struct Player {
    name: String,
    lifes: u8,
}

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
        ))
        .add_systems(OnEnter(GameState::Menu), setup_camera)
        .add_systems(OnEnter(GameState::InGame), setup_vessel)
        .add_systems(Update, rotate_vessel.run_if(in_state(GameState::InGame)))
        .insert_resource(CurrentLevel(0))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_vessel(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    current_level.0 = 1;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/player.png"),
            ..default()
        },
        Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
        Player {
            name: "Anakin".to_string(),
            lifes: 3,
        },
    ));
}

fn rotate_vessel(
    mut players: Query<&mut Transform, With<Player>>,

    keybord: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    let mut player = players.single_mut();

    if keybord.pressed(KeyCode::ArrowLeft) {
        player.rotate_z(PI / 24.0);
    }
    if keybord.pressed(KeyCode::ArrowRight) {
        player.rotate_z(-PI / 24.0);
    }

    for (entity, gamepad) in gamepads.iter() {
        if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX) {
            if left_stick_x > 0.6 {
                debug!("{:?} LeftStickX value is {}", entity, left_stick_x);
                player.rotate_z(-PI / 24.0);
            }
            if left_stick_x < -0.6 {
                debug!("{:?} LeftStickX value is {}", entity, left_stick_x);
                player.rotate_z(PI / 24.0);
            }
        }
    }
}
