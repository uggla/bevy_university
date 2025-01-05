use bevy::prelude::*;

use crate::{asteroids::Asteroid, vessel::Player};

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub struct UiText;

#[derive(Resource)]
pub struct GameTime(f32);

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Menu), display_menu)
            .add_systems(
                Update,
                manage_inputs.run_if(in_state(GameState::Menu).or(in_state(GameState::GameOver))),
            )
            .add_systems(OnExit(GameState::Menu), despawn_menu)
            .add_systems(OnEnter(GameState::GameOver), display_gameover)
            .add_systems(OnExit(GameState::GameOver), despawn_menu)
            .add_systems(OnEnter(GameState::InGame), ui)
            .add_systems(Update, update_ui.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), despawn_ui)
            .insert_resource(GameTime(0.0));
    }
}

#[allow(dead_code)]
#[derive(Debug, States, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    Paused,
    GameOver,
}

fn display_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Asteroids\nPress Space to start"),
                TextFont {
                    font: asset_server.load("fonts/kenvector_future.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.0, 0.0)),
            ));
        });
}

fn display_gameover(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            Menu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Game Over\nPress Space to continue"),
                TextFont {
                    font: asset_server.load("fonts/kenvector_future.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.0, 0.0)),
            ));
        });
}

fn manage_inputs(
    mut app_exit_events: EventWriter<AppExit>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut keybord: ResMut<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    if *state == GameState::Menu && keybord.just_pressed(KeyCode::Space) {
        keybord.reset(KeyCode::Space);
        next_state.set(GameState::InGame);
    }

    if *state == GameState::Menu && keybord.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }

    if *state == GameState::GameOver && keybord.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Menu);
    }

    for (entity, gamepad) in gamepads.iter() {
        if *state == GameState::Menu && gamepad.just_pressed(GamepadButton::South) {
            debug!("Gamepad {} just pressed South", entity);
            next_state.set(GameState::InGame);
        }

        if *state == GameState::GameOver && gamepad.just_pressed(GamepadButton::South) {
            debug!("Gamepad {} just pressed South", entity);
            next_state.set(GameState::Menu);
        }
    }
}

fn ui(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut gametime: ResMut<GameTime>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Px(210.0),
                height: Val::Px(100.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            // BackgroundColor(Color::srgb(0.0, 0.5, 0.)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Life: 3\nAsteroids: 200\nTime: 00:00:00"),
                TextFont {
                    font: asset_server.load("fonts/kenvector_future.ttf"),
                    font_size: 20.0,
                    ..default()
                },
                TextLayout {
                    justify: JustifyText::Left,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.0, 0.0)),
                UiText,
                gametime.0 = time.elapsed_secs(),
            ));
        });
}

fn update_ui(
    time: Res<Time>,
    mut query: Query<&mut Text, With<UiText>>,
    asteroids: Query<Entity, With<Asteroid>>,
    player: Query<&Player, With<Player>>,
    gametime: Res<GameTime>,
) {
    let asteroids_count = asteroids.iter().count();
    let player = player.single();
    for mut text in query.iter_mut() {
        let game_time = time.elapsed_secs() - gametime.0;
        *text = Text::new(format!(
            "Life: {}\nAsteroids: {}\nTime: {:0>2}:{:0>2}:{:0>2}",
            player.lifes,
            asteroids_count,
            (game_time / 3600.0) as u32,
            (game_time / 60.0) as u32,
            (game_time % 60.0) as u32
        ));
    }
}
fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<UiText>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
