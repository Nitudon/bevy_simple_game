use bevy::math::Vec2;

#[derive(Default)]
pub struct Mover {
    pub velocity: Vec2,
}

impl Mover {
    pub fn new(velocity: Vec2) -> Mover {
        Mover {
            velocity
        }
    }
    
    pub fn stop(&mut self) {
        self.velocity = Vec2::ZERO;
    }
}