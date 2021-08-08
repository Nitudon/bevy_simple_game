use bevy::ecs::system::Commands;
use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let stage_texture_handle = asset_server.load("textures/stage.png");
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(stage_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(960., 540.)),
        ..Default::default()
    });
    
    let player_texture_handle = asset_server.load("textures/player.png");
    let position = Vec3::new(0., -200., 0.);
    let rotation = Quat::from_axis_angle(Vec3::Y, 0.);
    let scale = Vec3::splat(1.);
    commands
        .spawn_bundle(SpriteBundle {
        material: materials.add(player_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(90., 90.)),
        transform: Transform {
            translation: position,
            rotation: rotation,
            scale: scale,
        },
        ..Default::default() })
        .insert_bundle((
            Player::default(),
            Mover::default()
            ));
}