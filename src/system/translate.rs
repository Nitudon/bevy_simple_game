use component::mover::Mover;
use bevy::prelude::*;

pub fn translate_mover_system(mut query: Query<(&Mover, &mut Transform)>) {
    for (mover, mut transform) in query.iter_mut(){
        let velocity = & mover.velocity;
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}