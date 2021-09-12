use bevy::prelude::*;
use resource::game::Game;
use component::player::Player;
use GameState;

// ゲーム中の時間処理
pub fn update_game(mut game: ResMut<Game>, mut state: ResMut<State<GameState>>, time: Res<Time>, query_player: Query<&Player>) {
    // ゲーム内時間とオブジェクト生成時間を更新
    game.time -= time.delta_seconds();
    game.spawn_time += time.delta_seconds();

    // プレイヤーのライフがゼロになったら終わり
    if let Ok(player) = query_player.single() {
        if player.life <= 0 {
            state.set(GameState::GameOver).unwrap();
            return;
        }
    }
    
    // ゲームの時間がつきたら終わり
    if game.time <= 0. {
        state.set(GameState::GameOver).unwrap();
        return;
    }
}