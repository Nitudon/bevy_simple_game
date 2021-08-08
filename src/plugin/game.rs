use crate::system::{
    collision::collision,
    player::player_input_system,
    translate::translate_mover_system,
    score::score_board_system,
};
use bevy::app::{Plugin, AppBuilder};
use bevy::prelude::*;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision.system());
        app.add_system(player_input_system.system());
        app.add_system(translate_mover_system.system());
        app.add_system(score_board_system.system());
    }
}