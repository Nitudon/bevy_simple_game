use bevy::prelude::*;
use component::apple::Apple;
use component::mover::Mover;
use component::block::Block;
use resource::game::Game;
use rand::Rng;
use std::cmp::min;

const SPEED_BASE : f32 = 2.0;
const SPEED_UP : f32 = 0.3;
const SPAWN_X_DISTANCE : f32 = 480.;
const SPAWN_INTERVAL_BASE : f32 = 2.5;
const SPAWN_INTERVAL_MIN : f32 = 0.5;
const SPAWN_INTERVAL_MINUS : f32 = 0.3;

pub enum ObjectType {
    Apple,
    Block,
}

pub fn check_spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let mut interval = SPAWN_INTERVAL_BASE - game.level as f32 * SPAWN_INTERVAL_MINUS;
    if interval < SPAWN_INTERVAL_MIN {
        interval = SPAWN_INTERVAL_MIN;
    }
    if game.spawn_time < interval {
        return;
    }
    
    game.spawn_time = 0.;
    let spawn_count = min(5, 1 + (game.level as f32 * 0.3) as i32);
    for _ in 0..spawn_count {
        let object_rand = rand::thread_rng().gen_range(1..10);
        let object_type = if object_rand < 4 {
            ObjectType::Block
        } else {
            ObjectType::Apple
        };
        let rand_x = rand::thread_rng().gen_range(-SPAWN_X_DISTANCE..SPAWN_X_DISTANCE);
        let position = Vec3::new(rand_x, 400., 0.);
        let rotation = Quat::from_axis_angle(Vec3::Y, 0.);
        let scale = Vec3::splat(1.);
        let texture = match object_type {
            ObjectType::Apple => asset_server.load("textures/apple.png"),
            ObjectType::Block => asset_server.load("textures/rock.png"),
        };
        let mut object_spawn = commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(texture.into()),
                sprite: Sprite::new(Vec2::new(90., 90.)),
                transform: Transform {
                    translation: position,
                    rotation: rotation,
                    scale: scale,
                },
                ..Default::default()
            });
        object_spawn.insert(Mover {
            velocity: Vec2::new(0., - (SPEED_BASE * rand::thread_rng().gen_range(0.8..1.2) + SPEED_UP * game.level as f32))
        });
        match object_type {
            ObjectType::Apple => object_spawn.insert(Apple{
                score: 100,
            }),
            ObjectType::Block => object_spawn.insert(Block{
                damage: 1,
            }),
        };
    }
}