use bevy::math::Vec2;

#[derive(Default)]
pub struct Mover {
    pub velocity: Vec2,
}

impl Mover {
    pub fn stop(&mut self) {
        self.velocity = Vec2::ZERO;
    }
}