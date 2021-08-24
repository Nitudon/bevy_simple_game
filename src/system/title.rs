use bevy::ecs::system::Commands;
use bevy::prelude::*;
use resource::game::{Game, GameState};

pub struct TitleScreen;

pub fn setup_title_ui (mut commands: Commands, asset_server: Res<AssetServer>) {
    // Label
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
                    font_size:80.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(TitleScreen);
}

pub fn wait_title_screen (mut commands: Commands, mut game: ResMut<Game>, input: Res<Input<MouseButton>>, query: Query<Entity, With<TitleScreen>>) {
   if game.state != GameState::Title {
       return;
   }
    
   if input.just_pressed(MouseButton::Left) {
       if let Ok(entity) = query.single() {
           commands.entity(entity).despawn();
       }
       game.state = GameState::Game;
   } 
}