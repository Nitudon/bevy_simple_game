extern crate bevy;
extern crate core;
extern crate rand;

pub mod component;
pub mod system;
pub mod resource;

use bevy::ecs::system::Commands;
use bevy::prelude::*;
use component::player::Player;
use component::mover::Mover;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Title,
    Playing,
    GameOver,
}

pub struct GameScene;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Game Objects
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameScene);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(GameScene);

    // Stage
    let stage_texture_handle = asset_server.load("textures/stage.png");
    commands
        .spawn_bundle(SpriteBundle {
        material: materials.add(stage_texture_handle.into()),
        sprite: Sprite::new(Vec2::new(960., 540.)),
        ..Default::default()
    })
        .insert(GameScene);

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
            Mover::default(),
            GameScene
        ));
}