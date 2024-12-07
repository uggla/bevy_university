use bevy::{prelude::*, window::WindowResolution};

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy University".to_string(),
                resizable: false,
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, my_first_system)
        .add_systems(Update, my_second_system)
        .insert_resource(CurrentLevel(0))
        .run();
}

fn my_first_system(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    asset_server: Res<AssetServer>,
) {
    println!("Hello, bevy! I'm the first system!");
    current_level.0 = 1;
    commands.spawn(Player {
        name: "Bob".to_string(),
        lifes: 3,
    });

    commands.spawn(Camera2d);

    commands.spawn(SpriteBundle {
        texture: asset_server.load("sprites/player.png"),
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5)),
        ..default()
    });
}

fn my_second_system(mut players: Query<&mut Player>, current_level: Res<CurrentLevel>) {
    let mut player = players.single_mut();

    println!("Current level: {:?}", current_level.0);
    player.name = "Anakin".to_string();
    println!(
        "I'm the second system! Player {} has {} lifes!",
        player.name, player.lifes
    );

    if let Ok(mut player) = players.get_single_mut() {
        player.name = "Leia".to_string();
        println!(
            "I'm the second system! Player {} has {} lifes!",
            player.name, player.lifes
        );
    }

    for mut player in players.iter_mut() {
        player.name = "Luc".to_string();
        player.lifes = 2;
        println!(
            "I'm the second system, looping on Players! Player {} has {} lifes!",
            player.name, player.lifes
        );
    }
}
