use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;
use component::apple::Apple;
use component::block::Block;

const PLAYER_SPEED: f32 = 3.0;

pub fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Mover, &Transform), With<Player>>,
) {
    if let Ok((mut mover, transform)) = query.single_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            if transform.translation.x <= -435.0 {
                mover.stop();
            } else {
                mover.velocity.x = -PLAYER_SPEED; 
            }
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            if transform.translation.x >= 435.0 {
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

pub fn player_life_system(
    mut commands: Commands,
    query_player: Query<&Player>,
    mut query_mover: Query<&mut Mover>,
    mut query_apple: Query<Entity, With<Apple>>,
    mut query_block: Query<Entity, With<Block>>,
) {
    if let Ok(player) = query_player.single() {
        if player.life <= 0 {
            for mut mover in query_mover.iter_mut() {
                mover.stop();
            }
            
            for apple in query_apple.iter_mut() {
                commands.entity(apple).despawn();
            }

            for block in query_block.iter_mut() {
                commands.entity(block).despawn();
            }
        }
    }
}