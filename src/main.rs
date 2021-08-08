extern crate bevy;
extern crate apple_game as root;

use bevy::prelude::*;
use bevy::app::App;
use root::plugin::game::GamePlugin;
use root::resource::game::Game;
use root::system::setup::setup;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Apple Game".to_string(),
            width: 960.,
            height: 540.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(Game::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(setup.system())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .run();
}