use bevy::prelude::*;
use resource::game::Game;
use component::player::Player;
use GameState;

// Game UI識別用のstruct
pub struct GameScoreLabel;
pub struct GameTimeLabel;
pub struct GameLifeLabel;

pub fn setup_title_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TitleLabel
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(50.),
                    left: Val::Px(320.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Game Start",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus10-Regular.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Game Time Label
    commands
        // Text Component
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(20.0),
                    left: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus10-Regular.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        // Identifier
        .insert(GameTimeLabel);
    
    // Game Score Label
    commands
        // Text Component
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(70.0),
                    left: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus10-Regular.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        // Identifier
        .insert(GameScoreLabel);
    
    // Player Life Label
    commands
        // Text Component
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(120.0),
                    left: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus10-Regular.ttf"),
                    font_size: 64.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        // Identifier
        .insert(GameLifeLabel);
}

pub fn setup_game_over_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Game Over Label
    commands
        // Text Component
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(50.),
                    left: Val::Px(320.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "Game Over",
                TextStyle {
                    font: asset_server.load("fonts/PixelMplus10-Regular.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });
}

// クリックしてゲーム開始
pub fn wait_title_screen(mut state: ResMut<State<GameState>>, mut input: ResMut<Input<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        state.set(GameState::Playing).unwrap();
        // reset input
        input.reset(MouseButton::Left);
    }
}

// クリックしてタイトルへ
pub fn wait_game_over_screen(mut state: ResMut<State<GameState>>, mut input: ResMut<Input<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        state.set(GameState::Title).unwrap();
        // reset input
        input.reset(MouseButton::Left);
    }
}

// ゲームのスコア表示
pub fn game_score_display_system(game: Res<Game>, mut score_query: Query<&mut Text, With<GameScoreLabel>>) {
    if let Ok(mut score_label) = score_query.single_mut() {
        score_label.sections[0].value = format!("Score: {}", game.score); 
    }
}

// ゲームの時間表示
pub fn game_time_display_system(game: Res<Game>, mut score_query: Query<&mut Text, With<GameTimeLabel>>) {
    if let Ok(mut score_label) = score_query.single_mut() {
        let time = game.time as i32;
        score_label.sections[0].value = format!("Time: {}", time);
    }
}

// プレイヤーのライフ表示
pub fn player_life_display_system(mut life_query: Query<&mut Text, With<GameLifeLabel>>, player_query: Query<&Player>) {
    if let Ok(mut life_label) = life_query.single_mut() {
        if let Ok(player) = player_query.single() {
            life_label.sections[0].value = format!("Life: {}", player.life);
        }
    }
}