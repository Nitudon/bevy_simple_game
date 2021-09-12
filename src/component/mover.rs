use bevy::math::Vec2;

// ゲーム内の移動する要素を表すComponent、System側が参照して適切に位置更新に用いる
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