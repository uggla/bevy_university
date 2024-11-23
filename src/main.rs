use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, my_first_system)
        .add_systems(
            Update,
            (my_second_system, my_third_system, my_fourth_system),
            // (my_second_system, my_third_system, my_fourth_system).chain(),
        )
        .run();
}

fn my_first_system() {
    println!("Hello, bevy! I'm the first system!");
}

fn my_second_system() {
    println!("I'm the second system!");
}

fn my_third_system() {
    println!("I'm the third system!");
}

fn my_fourth_system() {
    println!("I'm the fourth system!");
}
