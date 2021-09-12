#[derive(Default)]
pub struct Game {
    pub time: f32,
    pub spawn_time: f32,
    pub score: i32,
    pub level: i32,
}

const GAME_TIME: f32 = 120.;
const LEVEL_UP_SCORE: i32 = 500;

impl Game {
    pub fn new() -> Game {
        Game{
            time: GAME_TIME,
            ..Default::default()
        }
    }
    
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
        self.level = self.score / LEVEL_UP_SCORE;
    }
    
    pub fn reset(&mut self) {
        self.time = GAME_TIME;
        self.spawn_time = 0.;
        self.score = 0;
        self.level = 0;
    }
}