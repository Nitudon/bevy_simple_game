#[derive(Default)]
pub struct Game {
    pub time: f32,
    pub spawn_time: f32,
    pub score: i32,
    pub level: i32,
}

impl Game {
    pub fn new() -> Game {
        Game{
            time: 120.,
            ..Default::default()
        }
    }
    
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
        self.level = self.score / 500;
    }
}