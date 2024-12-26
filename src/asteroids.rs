use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, GravityScale, RigidBody, Velocity};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::states::GameState;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_asteroids);
        app.add_systems(OnExit(GameState::InGame), despawn_asteroids);
        app.add_systems(Update, wrap_asteroids);
    }
}

#[derive(Component, Debug, Clone)]
struct Asteroid {
    pos: Vec2,
    speed: Vec2,
    rot_speed: f32,
    size: AsteriodSize,
}

impl Asteroid {
    fn new(pos: Vec2, speed: Vec2, rot_speed: f32, size: AsteriodSize) -> Self {
        Self {
            pos,
            speed,
            rot_speed,
            size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AsteriodSize {
    Tiny,
    Small,
    Medium,
    Big,
}

impl AsteriodSize {
    fn size(&self) -> u32 {
        match self {
            AsteriodSize::Tiny => (18.0 * 1.5) as u32,
            AsteriodSize::Small => (28.0 * 1.5) as u32,
            AsteriodSize::Medium => (43.0 * 1.5) as u32,
            AsteriodSize::Big => (98.0 * 1.5) as u32,
        }
    }

    fn sprite(&self) -> &str {
        match self {
            AsteriodSize::Tiny => "sprites/meteorbrown_tiny1.png",
            AsteriodSize::Small => "sprites/meteorbrown_small1.png",
            AsteriodSize::Medium => "sprites/meteorbrown_med1.png",
            AsteriodSize::Big => "sprites/meteorbrown_big1.png",
        }
    }

    fn radius(&self) -> u32 {
        match self {
            AsteriodSize::Tiny => (12.0 * 1.5) as u32,
            AsteriodSize::Small => (18.0 * 1.5) as u32,
            AsteriodSize::Medium => (28.0 * 1.5) as u32,
            AsteriodSize::Big => (60.0 * 1.5) as u32,
        }
    }
}

fn setup_asteroids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asteroid_size = [
        AsteriodSize::Tiny,
        AsteriodSize::Small,
        AsteriodSize::Medium,
        AsteriodSize::Big,
    ];

    let mut rng = thread_rng();
    let mut asteroids: Vec<Asteroid> = Vec::with_capacity(200);

    for _ in 0..=200 {
        let asteroid_size = *asteroid_size.choose(&mut rng).unwrap();

        let mut pos = Vec2::new(
            rng.gen_range(-WINDOW_WIDTH * 3.0..WINDOW_WIDTH * 3.0),
            rng.gen_range(-WINDOW_HEIGHT * 3.0..WINDOW_HEIGHT * 3.0),
        );
        while !pos_is_valid(&asteroids, pos, asteroid_size.size()) {
            pos = Vec2::new(
                rng.gen_range(-WINDOW_WIDTH * 3.0..WINDOW_WIDTH * 3.0),
                rng.gen_range(-WINDOW_HEIGHT * 3.0..WINDOW_HEIGHT * 3.0),
            );
        }

        let initial_speed = Vec2::new(rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let rot_speed = rng.gen_range(-5.0..5.0);

        asteroids.push(Asteroid::new(pos, initial_speed, rot_speed, asteroid_size));
    }
    debug!("Asteroids: {:?}", asteroids);

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

fn pos_is_valid(asteroids: &[Asteroid], pos: Vec2, size: u32) -> bool {
    let size = size as f32 * 2.0f32.sqrt();
    if pos.x.abs() < 100.0 || pos.y.abs() < 100.0 {
        return false;
    }

    asteroids.iter().all(|a| {
        let distance = a.pos.distance(pos);
        distance > size
    })
}

fn wrap_asteroids(mut query: Query<&mut Transform, With<Asteroid>>) {
    for mut transform in query.iter_mut() {
        if transform.translation.x > WINDOW_WIDTH * 4.0 {
            transform.translation.x = -WINDOW_WIDTH * 4.0;
        }
        if transform.translation.x < -WINDOW_WIDTH * 4.0 {
            transform.translation.x = WINDOW_WIDTH * 4.0;
        }

        if transform.translation.y > WINDOW_HEIGHT * 4.0 {
            transform.translation.y = -WINDOW_HEIGHT * 4.0;
        }
        if transform.translation.y < -WINDOW_HEIGHT * 4.0 {
            transform.translation.y = WINDOW_HEIGHT * 4.0;
        }
    }
}

fn despawn_asteroids(mut commands: Commands, query: Query<Entity, With<Asteroid>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
