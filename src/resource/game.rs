#[derive(Default)]
pub struct Game {
    pub time: f32,
    pub spawn_time: f32,
    pub score: i32,
    pub level: i32,
    pub state: GameState,
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    Title,
    Game,
    Result,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Title
    }
}

pub struct GameScreen;
pub struct ScoreText;

impl Game {
    pub fn new() -> Game {
        Game{
            time: 120.,
            ..Default::default()
        }
    }
    
    pub fn is_game(&self) -> bool {
        self.state == GameState::Game
    }
    
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
        self.level = self.score / 500;
    }
}