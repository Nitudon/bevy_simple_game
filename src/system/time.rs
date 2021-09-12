use bevy::prelude::*;
use resource::game::Game;
use component::player::Player;
use GameState;

pub fn update_game(mut game: ResMut<Game>, mut state: ResMut<State<GameState>>, time: Res<Time>, query_player: Query<&Player>) {
    game.time -= time.delta_seconds();
    game.spawn_time += time.delta_seconds();

    if let Ok(player) = query_player.single() {
        if player.life <= 0 {
            state.set(GameState::GameOver).unwrap();
            return;
        }
    }
    
    if game.time <= 0. {
        state.set(GameState::GameOver).unwrap();
        return;
    }
}