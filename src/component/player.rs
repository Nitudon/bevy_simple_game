const PLAYER_LIFE: i32 = 3;

// プレイヤーを表すComponent、基本的にライフの値のModel
pub struct Player {
    pub life: i32,
}

impl Default for Player {
    fn default() -> Self {
        Player::new(PLAYER_LIFE)
    }
}

impl Player {
    pub fn new(life: i32) -> Self {
        Player {
            life
        }
    }
    
    pub fn reset(&mut self) {
        self.life = PLAYER_LIFE;
    }
}
