use bevy::prelude::*;
use crate::{AppState, PlayerState};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, read_build_input
                .run_if(in_state(AppState::InGame)));
    }
}

fn read_build_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    current_state: Res<State<PlayerState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        if *current_state.get() != PlayerState::Build {
            player_state.set(PlayerState::Build);
        } else {
            player_state.set(PlayerState::None);
        }
    }
}