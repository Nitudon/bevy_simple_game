use bevy::prelude::*;
use resource::game::{ScoreText, Game};

pub fn update_time(mut game: ResMut<Game>, time: Res<Time>) {
    if !game.is_start {
        return;
    }
    
    game.time -= time.delta_seconds();
}