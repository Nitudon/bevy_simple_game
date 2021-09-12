const APPLE_DEFAULT_SCORE: i32 = 100;

pub struct Apple {
    pub score: i32
}

impl Default for Apple {
    fn default() -> Self {
        Apple::new(APPLE_DEFAULT_SCORE)
    }
}

impl Apple {
    pub fn new(score: i32) -> Apple {
        Apple {
            score
        }
    }
}