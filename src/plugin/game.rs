use crate::system::{
    collision::collision,
    object_spawn::check_spawn_object,
    player::player_input_system,
    translate::translate_mover_system,
    score::score_board_system,
    time::update_time,
    title::wait_title_screen,
};
use bevy::app::{Plugin, AppBuilder};
use bevy::prelude::*;

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collision.system());
        app.add_system(check_spawn_object.system());
        app.add_system(player_input_system.system());
        app.add_system(translate_mover_system.system());
        app.add_system(score_board_system.system());
        app.add_system(update_time.system());
        app.add_system(wait_title_screen.system());
    }
}