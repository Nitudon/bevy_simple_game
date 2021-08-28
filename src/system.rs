pub mod collision;
pub mod object_spawn;
pub mod player;
pub mod display;
pub mod time;
pub mod translate;

use bevy::prelude::*;
use self::collision::collision;
use self::object_spawn::check_spawn_object;
use self::player::{player_input_system, player_life_system};
use self::translate::translate_mover_system;
use self::display::{setup_title_ui, set_game_ui, game_score_display_system, player_life_display_system, wait_title_screen};
use self::time::update_time;
use ::{GameState, GameScene};

// Title System
pub fn title_enter_system_set() -> SystemSet {
    SystemSet::on_enter(GameState::Title)
        .with_system(setup_title_ui.system())
}

pub fn title_update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::Title)
        .with_system(wait_title_screen.system())
}

pub fn title_exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::Title)
        .with_system(teardown.system())
}

// Playing System
pub fn game_enter_system_set() -> SystemSet {
    SystemSet::on_enter(GameState::Playing)
        .with_system(set_game_ui.system())
}

pub fn game_update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::Playing)
        .with_system(collision.system())
        .with_system(check_spawn_object.system())
        .with_system(player_input_system.system())
        .with_system(player_life_system.system())
        .with_system(translate_mover_system.system())
        .with_system(game_score_display_system.system())
        .with_system(player_life_display_system.system())
        .with_system(update_time.system())
}

pub fn game_exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::Playing)
        .with_system(teardown.system())
}

// Teardown State Entities
fn teardown(mut commands: Commands, entities: Query<Entity, Without<GameScene>>) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}