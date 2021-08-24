use bevy::prelude::*;
use resource::game:: Game;

pub fn update_time(mut game: ResMut<Game>, time: Res<Time>) {
    if game.is_game() == false {
        return;
    }
    
    game.time -= time.delta_seconds();
    game.spawn_time += time.delta_seconds();
}