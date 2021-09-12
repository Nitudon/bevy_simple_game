const BLOCK_DEFAULT_DAMAGE : i32 = 1;

// ゲーム内で落ちてくるブロックを表すComponent
pub struct Block {
    pub damage: i32,
}

impl Default for Block {
    fn default() -> Self {
        Block::new(BLOCK_DEFAULT_DAMAGE)
    }
}

impl Block {
    pub fn new(damage: i32) -> Block{
        Block {
            damage
        }
    }
}