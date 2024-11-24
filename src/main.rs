use bevy::prelude::*;

#[derive(Component)]
struct Player {
    name: String,
    lifes: u8,
}

fn main() {
    App::new()
        .add_systems(Startup, my_first_system)
        .add_systems(Update, my_second_system)
        .run();
}

fn my_first_system(mut commands: Commands) {
    println!("Hello, bevy! I'm the first system!");
    commands.spawn(Player {
        name: "Bob".to_string(),
        lifes: 3,
    });
    // commands.spawn(Player {
    //     name: "Alice".to_string(),
    //     lifes: 4,
    // });
}

fn my_second_system(players: Query<&Player>) {
    let player = players.single();
    println!(
        "I'm the second system! Player {} has {} lifes!",
        player.name, player.lifes
    );

    if let Ok(player) = players.get_single() {
        println!(
            "I'm the second system! Player {} has {} lifes!",
            player.name, player.lifes
        );
    }

    for player in players.iter() {
        println!(
            "I'm the second system, looping on Players! Player {} has {} lifes!",
            player.name, player.lifes
        );
    }
}
