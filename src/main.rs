extern crate bevy;
extern crate apple_game as game;

use bevy::prelude::*;
use bevy::app::App;
use game::plugin::game::GamePlugin;
use game::system::setup::setup;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Apple Game".to_string(),
            width: 960.,
            height: 540.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_startup_system(setup.system())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .run();
}