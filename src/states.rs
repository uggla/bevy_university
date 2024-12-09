use bevy::prelude::*;

#[derive(Component)]
pub struct Menu;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Menu), display_menu)
            .add_systems(Update, start_game.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), despawn_menu);
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
                Text::new("Press Space to start"),
                TextFont {
                    font: asset_server.load("fonts/kenvector_future.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.0, 0.0)),
            ));
        });
}

fn start_game(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keybord: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    if *state == GameState::Menu && keybord.just_pressed(KeyCode::Space) {
        next_state.set(GameState::InGame);
    }

    for (entity, gamepad) in gamepads.iter() {
        if *state == GameState::Menu && gamepad.just_pressed(GamepadButton::South) {
            debug!("Gamepad {} just pressed South", entity);
            next_state.set(GameState::InGame);
        }
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
