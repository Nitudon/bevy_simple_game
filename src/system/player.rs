use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;
use resource::game::Game;

pub fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    game: Res<Game>,
    mut query: Query<(&mut Mover, &Player)>,
) {
    if game.is_game() == false {
        return;
    }
    
    if let Ok((mut mover, player)) = query.single_mut() {
        let velocity = &mut mover.velocity;
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -2.0;
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = 2.0;
        }
        else {
            *velocity = Vec2::ZERO;
        }
    }
}