#[derive(Default)]
pub struct Game {
    pub time: f32,
    pub spawn_time: i32,
    pub score: i32,
    pub level: i32,
    pub is_start: bool,
}

pub struct ScoreText;

impl Game {
    pub fn new() -> Game {
        Game{
            time: 120.,
            is_start: true,
            ..Default::default()
        }
    }
}