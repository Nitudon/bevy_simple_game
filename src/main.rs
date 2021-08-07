extern crate bevy;
extern crate apple_game as game;

use bevy::prelude::*;
use bevy::app::App;
use game::plugin::game::GamePlugin;
use game::system::setup::setup;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .run();
}