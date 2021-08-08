#[derive(Default)]
pub struct Game {
    time: i32,
    score: i32,
}

pub struct ScoreText;

impl Game {
    pub fn score(&self) -> &i32 {
        &self.score
    } 
    
    pub fn decrement_time(&mut self) {
        self.time = &self.time - 1;
    }
    
    pub fn add_score(&mut self, add: i32) {
        self.score += add; 
    }
}