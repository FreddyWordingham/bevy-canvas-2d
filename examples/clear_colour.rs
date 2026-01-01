use bevy::{math::U8Vec2, prelude::*};
use bevy_canvas_2d::prelude::*;
use rand::Rng;

mod shared;

const CANVAS_SIZE: UVec2 = UVec2::splat(512);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((shared::CameraPlugin, shared::RandomPlugin))
        .add_plugins(CanvasPlugin {
            config: CanvasConfig {
                canvas_size: CANVAS_SIZE,
                num_chunks: U8Vec2::splat(4),
                ..default()
            },
        })
        .insert_resource(Time::<Fixed>::from_hz(1.0))
        .add_systems(FixedUpdate, clear_colour)
        .run();
}

fn clear_colour(mut seeded_rng: ResMut<shared::SeededRng>, mut clear_canvas_msg: MessageWriter<ClearCanvas>) {
    let rng = seeded_rng.rng();

    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    let a: u8 = 255;
    let colour = pack_rgba8([r, g, b, a]);

    clear_canvas_msg.write(ClearCanvas { rgba_u32: colour });
}
