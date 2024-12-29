use crate::{states::GameState, CurrentLevel, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, ExternalImpulse, GravityScale, RigidBody, Velocity,
};
use std::f32::consts::PI;

pub const VESSEL_WIDTH: f32 = 112.0;
#[allow(dead_code)]
pub const VESSEL_HEIGHT: f32 = 75.0;
const VESSEL_THRUST_POWER: f32 = 10000.0;

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    name: String,
    lifes: u8,
}

#[derive(Component)]
struct Explosion;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Event)]
struct Restart {
    entity: Entity,
}

pub struct VesselPlugin;

impl Plugin for VesselPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_vessel)
            .add_systems(OnExit(GameState::InGame), despawn_vessel)
            .add_systems(
                Update,
                exit_to_menu
                    .run_if(input_just_pressed(KeyCode::Escape).and(in_state(GameState::InGame))),
            )
            .add_systems(
                Update,
                (
                    rotate_vessel,
                    move_vessel,
                    wrap_vessel,
                    vessel_collisions,
                    animate_player_explosion,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_observer(restart);
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
        ActiveEvents::COLLISION_EVENTS,
        Visibility::Visible,
        Velocity::default(),
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
    // Get the 2D rotation angle in radians around the Z axis
    let rotation = player_transform.rotation.to_euler(EulerRot::ZYX).0;

    // Compute the directional vector for the impulse. Since the vessel's nose points upward,
    // the impulse direction must be rotated 90 degrees counterclockwise. This is achieved
    // using trigonometry, where x = -sin(angle) and y = cos(angle) rotate the vector accordingly.
    let impulse_direction = Vec2::new(
        -rotation.sin() * VESSEL_THRUST_POWER,
        rotation.cos() * VESSEL_THRUST_POWER,
    );
    for mut ext_impulse in ext_impulses.iter_mut() {
        ext_impulse.impulse = impulse_direction
    }
}

fn wrap_vessel(
    mut objects_query: Query<&mut Transform, Without<Player>>,
    mut players_query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = players_query.single_mut();

    if player_transform.translation.x > WINDOW_WIDTH * 3.0 {
        player_transform.translation.x = -WINDOW_WIDTH * 3.0;
        translate_objects_horiz(&mut objects_query);
    } else if player_transform.translation.x < -WINDOW_WIDTH * 3.0 {
        player_transform.translation.x = WINDOW_WIDTH * 3.0;
        translate_objects_horiz(&mut objects_query);
    }

    if player_transform.translation.y > WINDOW_HEIGHT * 3.0 {
        player_transform.translation.y = -WINDOW_HEIGHT * 3.0;
        translate_objects_vert(&mut objects_query);
    } else if player_transform.translation.y < -WINDOW_HEIGHT * 3.0 {
        player_transform.translation.y = WINDOW_HEIGHT * 3.0;
        translate_objects_vert(&mut objects_query);
    }
}

fn translate_objects_horiz(objects_query: &mut Query<&mut Transform, Without<Player>>) {
    for mut asteroid_transform in objects_query.iter_mut() {
        if asteroid_transform.translation.x > WINDOW_WIDTH * 2.0 {
            asteroid_transform.translation.x -= WINDOW_WIDTH * 6.0;
        } else if asteroid_transform.translation.x < -WINDOW_WIDTH * 2.0 {
            asteroid_transform.translation.x += WINDOW_WIDTH * 6.0;
        }
    }
}

fn translate_objects_vert(objects_query: &mut Query<&mut Transform, Without<Player>>) {
    for mut asteroid_transform in objects_query.iter_mut() {
        if asteroid_transform.translation.y > WINDOW_HEIGHT * 2.0 {
            asteroid_transform.translation.y -= WINDOW_HEIGHT * 6.0;
        } else if asteroid_transform.translation.y < -WINDOW_HEIGHT * 2.0 {
            asteroid_transform.translation.y += WINDOW_HEIGHT * 6.0;
        }
    }
}

fn vessel_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &mut Visibility, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (player_entity, mut player_visibility, player_pos) = match player.get_single_mut() {
        Ok(player_entity) => player_entity,
        Err(_) => return,
    };
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _cf) => {
                // Warning, e1 and e2 can be swapped.
                if (player_entity == *e2) || (player_entity == *e1) {
                    debug!("Received collision event: {:?}", collision_event);
                    *player_visibility = Visibility::Hidden;
                    let texture = asset_server.load("sprites/explosion.png");
                    let layout =
                        TextureAtlasLayout::from_grid(UVec2::new(583, 536), 9, 1, None, None);
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    commands.spawn((
                        Sprite::from_atlas_image(
                            texture,
                            TextureAtlas {
                                layout: texture_atlas_layout,
                                index: 0,
                            },
                        ),
                        Transform {
                            translation: player_pos.translation,
                            scale: Vec3::splat(0.5),
                            ..default()
                        },
                        Explosion,
                        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                    ));
                }
            }
            CollisionEvent::Stopped(e1, e2, _cf) => {
                // Warning, e1 and e2 can be swapped.
                if (player_entity == *e2) || (player_entity == *e1) {
                    debug!("Received collision event: {:?}", collision_event);
                }
            }
        }
    }
}

fn animate_player_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut AnimationTimer, &mut Sprite), With<Explosion>>,
    mut player: Query<&Transform, (With<Player>, Without<Explosion>)>,
) {
    let player_pos = match player.get_single_mut() {
        Ok(player_entity) => player_entity,
        Err(_) => return,
    };

    for (explosion_entity, mut explosion_pos, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            explosion_pos.translation = player_pos.translation;
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < 8 {
                    atlas.index += 1;
                } else {
                    atlas.index = 0;
                    commands.trigger(Restart {
                        entity: explosion_entity,
                    });
                }
            }
        }
    }
}

fn restart(
    trigger: Trigger<Restart>,
    mut commands: Commands,
    mut player: Query<(&mut Player, &mut Transform, &mut Velocity, &mut Visibility), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (mut player, mut player_pos, mut player_velocity, mut player_visibility) =
        match player.get_single_mut() {
            Ok(player_entity) => player_entity,
            Err(_) => return,
        };
    commands.entity(trigger.entity).despawn_recursive();
    *player_visibility = Visibility::Visible;
    *player_velocity = Velocity::default();
    player_pos.translation = Vec3::new(0.0, 0.0, 0.0);
    player_pos.rotation = Quat::from_rotation_z(0.0);
    if player.lifes == 0 {
        next_state.set(GameState::GameOver);
    } else {
        player.lifes -= 1;
        debug!("Remaining lifes: {}", player.lifes);
    }
}

fn despawn_vessel(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit_to_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}
