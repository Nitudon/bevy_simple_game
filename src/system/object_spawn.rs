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
const SPAWN_INTERVAL_BASE : f32 = 2.0;
const SPAWN_INTERVAL_MIN : f32 = 0.2;
const SPAWN_INTERVAL_MINUS : f32 = 0.3;
const SPAWN_COUNT_MIN : i32 = 2;
const SPAWN_COUNT_MAX : i32 = 6;

pub enum ObjectType {
    Apple,
    Block,
}

// 一定間隔でオブジェクト（林檎とブロック）を生成する
pub fn check_spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    // 生成間隔の計算
    let mut interval = SPAWN_INTERVAL_BASE - game.level as f32 * SPAWN_INTERVAL_MINUS;
    if interval < SPAWN_INTERVAL_MIN {
        interval = SPAWN_INTERVAL_MIN;
    }
    // 前回生成した時間からまだ次の生成時間に達してなかったら何もしない
    if game.spawn_time < interval {
        return;
    }
    
    // 生成するので生成時間のインターバルをリセット
    game.spawn_time = 0.;
    // 今のレベルに沿って適当に生成する個数を決定
    let spawn_count = min(SPAWN_COUNT_MAX, SPAWN_COUNT_MIN + (game.level as f32 * 0.3) as i32);
    // 個数分生成処理を回す
    for _ in 0..spawn_count {
        // 適当な生成物を決定する乱数
        let object_rand = rand::thread_rng().gen_range(1..10);
        let object_type = if object_rand < 4 {
            ObjectType::Block
        } else {
            ObjectType::Apple
        };
        // 適当な生成位置からtransform初期化し、Sprite用のTextureもロード
        let rand_x = rand::thread_rng().gen_range(-SPAWN_X_DISTANCE..SPAWN_X_DISTANCE);
        let position = Vec3::new(rand_x, 400., 0.);
        let rotation = Quat::from_axis_angle(Vec3::Y, 0.);
        let scale = Vec3::splat(1.);
        let texture = match object_type {
            ObjectType::Apple => asset_server.load("textures/apple.png"),
            ObjectType::Block => asset_server.load("textures/rock.png"),
        };
        // 生成処理
        // SpriteとMoverの設定
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
        object_spawn.insert(Mover::new(Vec2::new(0., - (SPEED_BASE * rand::thread_rng().gen_range(0.8..1.2) + SPEED_UP * game.level as f32))));
        // 乱数で決めたenumからComponentを分岐
        match object_type {
            ObjectType::Apple => object_spawn.insert(Apple::default()),
            ObjectType::Block => object_spawn.insert(Block::default()),
        };
    }
}