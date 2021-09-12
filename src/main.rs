extern crate bevy;
extern crate apple_game as root;

use bevy::prelude::*;
use bevy::app::App;
use root::resource::game::Game;
use root::{
    setup,
    GameState
};
use root::system::{
    title_enter_system_set, 
    title_update_system_set, 
    game_enter_system_set, 
    game_update_system_set, 
    title_exit_system_set, 
    game_exit_system_set, 
    game_over_enter_system_set, 
    game_over_update_system_set,
    game_over_exit_system_set
};

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Apple Game".to_string(),
            width: 960.,
            height: 540.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(Game::new())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Title)
        .add_startup_system(setup.system())
        .add_system_set(title_enter_system_set())
        .add_system_set(title_update_system_set())
        .add_system_set(title_exit_system_set())
        .add_system_set(game_enter_system_set())
        .add_system_set(game_update_system_set())
        .add_system_set(game_exit_system_set())
        .add_system_set(game_over_enter_system_set())
        .add_system_set(game_over_update_system_set())
        .add_system_set(game_over_exit_system_set())
        .run();
}