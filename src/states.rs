use bevy::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, start_game.run_if(in_state(GameState::Menu)));
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

fn start_game(state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    // TODO: right now we enter the InGame state directly,
    // but we should display a "menu" and add a condition when player press a button.
    if *state == GameState::Menu {
        next_state.set(GameState::InGame);
    }
}
