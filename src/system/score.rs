use bevy::prelude::*;
use resource::game::{ScoreText, Game};

pub fn score_board_system(mut game: Res<Game>, mut query: Query<&mut Text, With<ScoreText>>) {
    if game.is_game() == false {
        return;
    }
    
    if let Ok(mut text) = query.single_mut() {
        text.sections[0].value = game.score.to_string(); 
    }
}