use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, GravityScale, RigidBody, Velocity};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::states::GameState;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

const ASTEROIDS_COUNT: usize = 200;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup_asteroids);
        app.add_systems(OnExit(GameState::InGame), despawn_asteroids);
        app.add_systems(Update, wrap_asteroids);
    }
}

#[derive(Component, Debug, Clone)]
pub struct Asteroid {
    pub pos: Vec2,
    pub speed: Vec2,
    pub rot_speed: f32,
    pub size: AsteroidSize,
}

impl Asteroid {
    pub fn new(pos: Vec2, speed: Vec2, rot_speed: f32, size: AsteroidSize) -> Self {
        Self {
            pos,
            speed,
            rot_speed,
            size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AsteroidSize {
    Tiny,
    Small,
    Medium,
    Big,
}

impl AsteroidSize {
    fn size(&self) -> u32 {
        match self {
            AsteroidSize::Tiny => (18.0 * 1.5) as u32,
            AsteroidSize::Small => (28.0 * 1.5) as u32,
            AsteroidSize::Medium => (43.0 * 1.5) as u32,
            AsteroidSize::Big => (98.0 * 1.5) as u32,
        }
    }

    pub fn sprite(&self) -> &str {
        match self {
            AsteroidSize::Tiny => "sprites/meteorbrown_tiny1.png",
            AsteroidSize::Small => "sprites/meteorbrown_small1.png",
            AsteroidSize::Medium => "sprites/meteorbrown_med1.png",
            AsteroidSize::Big => "sprites/meteorbrown_big1.png",
        }
    }

    pub fn radius(&self) -> u32 {
        match self {
            AsteroidSize::Tiny => (12.0 * 1.5) as u32,
            AsteroidSize::Small => (18.0 * 1.5) as u32,
            AsteroidSize::Medium => (28.0 * 1.5) as u32,
            AsteroidSize::Big => (60.0 * 1.5) as u32,
        }
    }

    pub fn explosion_size(&self) -> f32 {
        match self {
            AsteroidSize::Tiny => 0.1,
            AsteroidSize::Small => 0.2,
            AsteroidSize::Medium => 0.3,
            AsteroidSize::Big => 0.5,
        }
    }
}

fn setup_asteroids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let asteroid_size = [
        AsteroidSize::Tiny,
        AsteroidSize::Small,
        AsteroidSize::Medium,
        AsteroidSize::Big,
    ];

    let mut rng = thread_rng();
    let mut asteroids: Vec<Asteroid> = Vec::with_capacity(ASTEROIDS_COUNT);

    for _ in 0..ASTEROIDS_COUNT {
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
    trace!("Asteroids: {:?}", asteroids);

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
