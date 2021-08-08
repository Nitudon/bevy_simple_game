use bevy::prelude::*;
use component::player::Player;
use component::block::Block;
use bevy::sprite::collide_aabb::collide;

pub fn collision(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform, &Sprite)>,
    mut block_query: Query<(Entity, &mut Block, &Transform, &Sprite)>) {
    if let Ok((mut player, player_transform, player_sprite)) = player_query.single_mut() {
        for (block_entity, block, block_transform, block_sprite) in block_query.iter_mut() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                block_transform.translation,
                block_sprite.size,
            );
            if let Some(collision) = collision {
                commands.entity(block_entity).despawn();
            }
        }
    }
}