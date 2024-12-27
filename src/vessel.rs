use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, ExternalImpulse, GravityScale, RigidBody};

pub const VESSEL_WIDTH: f32 = 112.0;
#[allow(dead_code)]
pub const VESSEL_HEIGHT: f32 = 75.0;
const VESSEL_THRUST_POWER: f32 = 10000.0;

use std::f32::consts::PI;

use crate::{states::GameState, CurrentLevel};

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    name: String,
    lifes: u8,
}

pub struct VesselPlugin;

impl Plugin for VesselPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_vessel)
            .add_systems(
                Update,
                (rotate_vessel, move_vessel).run_if(in_state(GameState::InGame)),
            );
    }
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
        RigidBody::Dynamic,
        Collider::ball(VESSEL_WIDTH / 4.0),
        GravityScale(0.0),
        ExternalImpulse::default(),
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

fn move_vessel(
    players: Query<&mut Transform, With<Player>>,
    keybord: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<Player>>,
) {
    if keybord.pressed(KeyCode::ArrowUp) {
        activate_thrust(&players, &mut ext_impulses);
    }

    for (_entity, gamepad) in gamepads.iter() {
        if gamepad.pressed(GamepadButton::South) {
            activate_thrust(&players, &mut ext_impulses);
        }
    }
}

fn activate_thrust(
    players: &Query<&mut Transform, With<Player>>,
    ext_impulses: &mut Query<&mut ExternalImpulse, With<Player>>,
) {
    let player_transform = players.single();
    // Get the 2D rotation angle in radians
    let rotation = player_transform.rotation.to_euler(EulerRot::ZYX).0;
    // Z-axis rotation

    // Compute the directional vector using cos (x) and sin (y)
    let direction = Vec2::new(
        rotation.sin() * -VESSEL_THRUST_POWER,
        rotation.cos() * VESSEL_THRUST_POWER,
    );
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = direction
    }
}
        }
    }
}
