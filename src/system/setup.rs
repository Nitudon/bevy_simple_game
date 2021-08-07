use bevy::ecs::system::Commands;
use bevy::prelude::*;

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
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(player_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(90., 90.)),
        ..Default::default()
    });
}