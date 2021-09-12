use component::mover::Mover;
use bevy::prelude::*;

// Moverの速度に沿った位置更新
pub fn translate_mover_system(mut query: Query<(&Mover, &mut Transform)>) {
    // 速度に伴ってtransformの位置情報を更新
    for (mover, mut transform) in query.iter_mut(){
        let velocity = &mover.velocity;
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}