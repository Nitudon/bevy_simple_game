use bevy::ecs::system::Commands;
use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;
use resource::game::{ScoreText, Game};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Game
    commands.insert_resource(Game::new());
    
    // Stage
    let stage_texture_handle = asset_server.load("textures/stage.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(stage_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(960., 540.)),
        ..Default::default()
    });
    
    // Player
    let player_texture_handle = asset_server.load("textures/player.png");
    let player_position = Vec3::new(0., -200., 0.);
    let player_rotation = Quat::from_axis_angle(Vec3::Y, 0.);
    let player_scale = Vec3::splat(1.);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(player_texture_handle.into()),
            sprite: Sprite::new(Vec2::new(90., 90.)),
            transform: Transform {
                translation: player_position,
                rotation: player_rotation,
                scale: player_scale,
            },
            ..Default::default()
        })
        .insert_bundle((
            Player::default(),
            Mover::default()
            ));
    
    // Score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(80.0),
                    left : Val::Percent(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "---",
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
        .insert(ScoreText);
}