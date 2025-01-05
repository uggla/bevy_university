use crate::{
    asteroids::{Asteroid, AsteroidSize},
    states::GameState,
    CurrentLevel, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*, utils::HashSet};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, ExternalImpulse, GravityScale, RigidBody, Velocity,
};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

pub const VESSEL_WIDTH: f32 = 112.0;
#[allow(dead_code)]
pub const VESSEL_HEIGHT: f32 = 75.0;
const VESSEL_THRUST_POWER: f32 = 10000.0;
const LASER_WIDTH: f32 = 9.0;
const LASER_HEIGHT: f32 = 54.0;
const LASER_SPEED: f32 = 1000.0;
const LASER_SPAWN_DURATION: f32 = 0.2;
const LASER_FLY_TIME: f32 = 0.5;
const LASER_VESSEL_POSITION: [f32; 2] = [-22.0, 22.0];

#[allow(dead_code)]
#[derive(Component)]
pub struct Player {
    name: String,
    lifes: u8,
}

#[derive(Component)]
struct Laser {
    fly_time: Timer,
}

#[derive(Component)]
struct PlayerExplosion;

#[derive(Component)]
struct AsteroidExplosion {
    asteroid: Asteroid,
    laser_velocity: Vec2,
}

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
                    animate_player_explosion.after(vessel_collisions),
                    fire_lasers,
                    restrict_lasers_range.after(vessel_collisions),
                    animate_asteroid_explosion.after(vessel_collisions),
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

fn fire_lasers(
    mut commands: Commands,
    players: Query<(&mut Transform, &Velocity), With<Player>>,
    keybord: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
    asset_server: Res<AssetServer>,
    mut spawn_timer: Local<Timer>,
    time: Res<Time>,
) {
    spawn_timer.tick(time.delta());

    if keybord.pressed(KeyCode::Space) && spawn_timer.finished() {
        spawn_lasers(&mut spawn_timer, &players, &mut commands, &asset_server);
    }

    for (_entity, gamepad) in gamepads.iter() {
        if gamepad.pressed(GamepadButton::West) && spawn_timer.finished() {
            spawn_lasers(&mut spawn_timer, &players, &mut commands, &asset_server);
        }
    }
}

fn spawn_lasers(
    spawn_timer: &mut Local<Timer>,
    players: &Query<(&mut Transform, &Velocity), With<Player>>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    **spawn_timer = Timer::from_seconds(LASER_SPAWN_DURATION, TimerMode::Once);
    let (player_pos, player_velocity) = players.single();
    let rotation_angle = player_pos.rotation.to_euler(EulerRot::ZYX).0;
    let sprite_direction = Vec2::new(rotation_angle.cos(), rotation_angle.sin());
    let vessel_direction = Vec2::new(-rotation_angle.sin(), rotation_angle.cos());

    for laser_x_pos in LASER_VESSEL_POSITION {
        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/laser.png"),
                ..default()
            },
            Transform {
                translation: (player_pos.translation.xy()
                    + Vec2::new(laser_x_pos, LASER_HEIGHT / 2.0).rotate(sprite_direction))
                .extend(0.0),

                rotation: player_pos.rotation,
                ..default()
            },
            Laser {
                fly_time: Timer::from_seconds(LASER_FLY_TIME, TimerMode::Once),
            },
            RigidBody::Dynamic,
            Collider::cuboid(LASER_WIDTH / 2.5, LASER_HEIGHT / 2.0),
            GravityScale(0.0),
            ActiveEvents::COLLISION_EVENTS,
            Velocity {
                linvel: vessel_direction * LASER_SPEED + player_velocity.linvel,
                ..default()
            },
        ));
    }
}

fn restrict_lasers_range(
    mut commands: Commands,
    mut lasers: Query<(Entity, &mut Laser)>,
    time: Res<Time>,
) {
    for (entity, mut laser) in lasers.iter_mut() {
        if laser.fly_time.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
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

#[allow(clippy::too_many_arguments)]
fn vessel_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player: Query<(Entity, &mut Visibility, &Transform), With<Player>>,
    lasers: Query<Entity, With<Laser>>,
    asteroid_qry: Query<(&Transform, &Asteroid)>,
    velocity_qry: Query<&Velocity>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut avoid_duplicated: Local<HashSet<Entity>>,
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
                    let explosion = spawn_explosion(
                        &asset_server,
                        &mut texture_atlas_layouts,
                        &mut commands,
                        player_pos.translation,
                        0.5,
                    );
                    commands.entity(explosion).insert(PlayerExplosion);
                }

                // Determine which entity is the laser.
                let laser_colision = if lasers.contains(*e1) {
                    Some((e1, e2))
                } else if lasers.contains(*e2) {
                    Some((e2, e1))
                } else {
                    None
                };

                if let Some((laser_entity, object_entity)) = laser_colision {
                    if !avoid_duplicated.insert(*laser_entity) {
                        return;
                    }
                    if !avoid_duplicated.insert(*object_entity) {
                        return;
                    }

                    if *object_entity == player_entity {
                        debug!(
                            "Received collision event with player: {:?}",
                            collision_event
                        );
                        *player_visibility = Visibility::Hidden;
                        let explosion = spawn_explosion(
                            &asset_server,
                            &mut texture_atlas_layouts,
                            &mut commands,
                            player_pos.translation,
                            0.5,
                        );
                        commands.entity(explosion).insert(PlayerExplosion);
                        commands.entity(*laser_entity).despawn_recursive();
                    } else {
                        debug!(
                            "Received collision event with asteroid: {:?}",
                            collision_event
                        );
                        let (asteroid_pos, asteroid) = match asteroid_qry.get(*object_entity) {
                            Ok((asteroid_pos, asteroid)) => (asteroid_pos, asteroid),
                            Err(_) => return,
                        };
                        let laser_velocity = match velocity_qry.get(*laser_entity) {
                            Ok(laser_velocity) => laser_velocity,
                            Err(_) => return,
                        };
                        let explosion = spawn_explosion(
                            &asset_server,
                            &mut texture_atlas_layouts,
                            &mut commands,
                            asteroid_pos.translation,
                            asteroid.size.explosion_size(),
                        );
                        commands.entity(explosion).insert(AsteroidExplosion {
                            asteroid: asteroid.clone(),
                            laser_velocity: laser_velocity.linvel,
                        });
                        commands.entity(*laser_entity).despawn_recursive();
                        commands.entity(*object_entity).despawn_recursive();
                    }
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

fn spawn_explosion(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    commands: &mut Commands,
    explosion_position: Vec3,
    explosion_scale: f32,
) -> Entity {
    let texture = asset_server.load("sprites/explosion.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(583, 536), 9, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            Transform {
                translation: explosion_position,
                scale: Vec3::splat(explosion_scale),
                ..default()
            },
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .id()
}

fn animate_player_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (Entity, &mut Transform, &mut AnimationTimer, &mut Sprite),
        With<PlayerExplosion>,
    >,
    mut player: Query<&Transform, (With<Player>, Without<PlayerExplosion>)>,
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

fn animate_asteroid_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut Sprite,
            &Transform,
            &AsteroidExplosion,
        ),
        With<AsteroidExplosion>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (explosion_entity, mut timer, mut sprite, explosion_pos, explosion) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index < 8 {
                    atlas.index += 1;
                } else {
                    match explosion.asteroid.size {
                        AsteroidSize::Tiny => {}
                        AsteroidSize::Small => {}
                        AsteroidSize::Medium => {
                            let laser_direction = explosion.laser_velocity.normalize();
                            let mut asteroids: Vec<Asteroid> = Vec::with_capacity(2);

                            let asteroid_size = AsteroidSize::Tiny;
                            spawn_splited_asteroids(
                                &mut commands,
                                &asset_server,
                                &mut asteroids,
                                explosion_pos,
                                laser_direction,
                                asteroid_size,
                            );
                        }
                        AsteroidSize::Big => {
                            let laser_direction = explosion.laser_velocity.normalize();
                            let mut asteroids: Vec<Asteroid> = Vec::with_capacity(2);

                            let asteroid_size = AsteroidSize::Small;
                            spawn_splited_asteroids(
                                &mut commands,
                                &asset_server,
                                &mut asteroids,
                                explosion_pos,
                                laser_direction,
                                asteroid_size,
                            );
                        }
                    };
                    commands.entity(explosion_entity).despawn_recursive();
                }
            }
        }
    }
}

fn spawn_splited_asteroids(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    asteroids: &mut Vec<Asteroid>,
    explosion_pos: &Transform,
    laser_direction: Vec2,
    new_asteroid_size: AsteroidSize,
) {
    let mut rng = thread_rng();
    // Build first asteroid
    let pos = explosion_pos.translation.xy()
        + Vec2::new(0.0, new_asteroid_size.radius() as f32).rotate(laser_direction);

    let initial_speed =
        Vec2::new(-laser_direction.y, laser_direction.x) * (150.0 + rng.gen_range(0.0..100.0));

    let rot_speed = rng.gen_range(-5.0..5.0);

    asteroids.push(Asteroid::new(
        pos,
        initial_speed,
        rot_speed,
        new_asteroid_size,
    ));

    // Build second asteroid
    let pos = explosion_pos.translation.xy()
        + Vec2::new(0.0, -(new_asteroid_size.radius() as f32)).rotate(laser_direction);

    let initial_speed =
        Vec2::new(laser_direction.y, -laser_direction.x) * (150.0 + rng.gen_range(0.0..100.0));

    asteroids.push(Asteroid::new(
        pos,
        initial_speed,
        rot_speed,
        new_asteroid_size,
    ));

    // Spawn them
    for asteroid in asteroids.iter() {
        commands.spawn((
            Sprite {
                image: asset_server.load(asteroid.size.sprite()),
                ..default()
            },
            Transform {
                scale: Vec3::new(1.5, 1.5, 1.5),
                translation: asteroid.pos.extend(0.0),
                ..default()
            },
            asteroid.clone(),
            RigidBody::Dynamic,
            Collider::ball(asteroid.size.radius() as f32 / 2.0),
            GravityScale(0.0),
            Velocity {
                linvel: asteroid.speed,
                angvel: asteroid.rot_speed,
            },
        ));
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
