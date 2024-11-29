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

fn my_second_system(mut players: Query<&mut Player>) {
    let mut player = players.single_mut();
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
