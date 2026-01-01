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
        .add_systems(Update, draw_pixel)
        .run();
}

fn draw_pixel(mut seeded_rng: ResMut<shared::SeededRng>, mut draw_pixel_msg: MessageWriter<DrawPixel>) {
    let rng = seeded_rng.rng();

    let x = rng.random_range(0..CANVAS_SIZE.x);
    let y = rng.random_range(0..CANVAS_SIZE.y);

    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    let a: u8 = 255;
    let colour = pack_rgba8([r, g, b, a]);

    draw_pixel_msg.write(DrawPixel {
        pos: UVec2::new(x, y),
        rgba_u32: colour,
    });
}
