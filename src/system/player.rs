use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;

const PLAYER_SPEED: f32 = 3.;
const PLAYER_MAX_MOVE_X: f32 = 435.;

pub fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Mover, &Transform), With<Player>>,
) {
    if let Ok((mut mover, transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if transform.translation.x <= -PLAYER_MAX_MOVE_X {
                mover.stop();
            } else {
                mover.velocity.x = -PLAYER_SPEED; 
            }
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            if transform.translation.x >= PLAYER_MAX_MOVE_X {
                mover.stop();
            } else {
                mover.velocity.x = PLAYER_SPEED;
            }
        }
        else {
            mover.stop();
        }
    }
}