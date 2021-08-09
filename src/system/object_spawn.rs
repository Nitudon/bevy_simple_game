use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use component::mover::Mover;
use component::block::Block;
use resource::game::Game;
use rand::Rng;

const SPEED_BASE : f32 = 1.5;
const SPEED_UP : f32 = 0.5;
const SPAWN_X_DISTANCE : f32 = 480.;

#[derive(Default)]
pub struct ObjectSpawner {
    is_active: bool,
    level: i32,
}

pub enum ObjectType {
    Apple,
    Block,
}

const TICK_INTERVAL : i32 = 2;

pub fn check_spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let tick = game.time as i32 / TICK_INTERVAL;
    let object_type = ObjectType::Apple;
    if game.spawn_time == tick {
        return;
    }
    game.spawn_time = tick;
    
    let rand_x = rand::thread_rng().gen_range(-SPAWN_X_DISTANCE..SPAWN_X_DISTANCE);
    let position = Vec3::new(rand_x, 400., 0.);
    let rotation = Quat::from_axis_angle(Vec3::Y, 0.);
    let scale = Vec3::splat(1.);
    let texture = match object_type {
        ObjectType::Apple => asset_server.load("textures/apple.png"),
        ObjectType::Block => asset_server.load("textures/block.png"),
    };
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(texture.into()),
            sprite: Sprite::new(Vec2::new(90., 90.)),
            transform: Transform {
                translation: position,
                rotation: rotation,
                scale: scale,
            },
            ..Default::default()
        })
        .insert(Mover {
            velocity: Vec2::new(0., - (SPEED_BASE))
        });
}