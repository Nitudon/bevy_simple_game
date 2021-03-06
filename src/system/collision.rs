use bevy::prelude::*;
use component::player::Player;
use component::block::Block;
use bevy::sprite::collide_aabb::collide;
use component::apple::Apple;
use resource::game::Game;

const OBJECT_DESPAWN_Y : f32 = -480.;

pub fn collision(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    mut apple_query: Query<(Entity, &mut Apple, &Transform, &Sprite)>,
    mut block_query: Query<(Entity, &mut Block, &Transform, &Sprite)>) {
    
    if let Ok((mut player, player_transform, player_sprite)) = player_query.single_mut() {
        for (block_entity, block, block_transform, block_sprite) in block_query.iter_mut() {
            if block_transform.translation.y < OBJECT_DESPAWN_Y {
                commands.entity(block_entity).despawn(); 
                break;
            }
            
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                block_transform.translation,
                block_sprite.size,
            );
            if let Some(_) = collision {
                commands.entity(block_entity).despawn();
                player.life -= block.damage;
            }
        }

        for (apple_entity, apple, apple_transform, apple_sprite) in apple_query.iter_mut() {
            if apple_transform.translation.y < OBJECT_DESPAWN_Y {
                commands.entity(apple_entity).despawn();
                break;
            }
            
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                apple_transform.translation,
                apple_sprite.size,
            );
            if let Some(_) = collision {
                game.add_score(apple.score);
                commands.entity(apple_entity).despawn();
            }
        }
    }
}