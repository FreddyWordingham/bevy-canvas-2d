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
        .add_systems(Update, draw_pixels)
        .run();
}

fn draw_pixels(mut draw_pixels_msg: MessageWriter<DrawPixels>, mut seeded_rng: ResMut<shared::SeededRng>) {
    let rng = seeded_rng.rng();

    let l = rng.random_range(0..1000) as usize;
    let mut positions = Vec::with_capacity(l);
    let mut rgba_u32 = Vec::with_capacity(l);

    for _ in 0..l {
        let x = rng.random_range(0..CANVAS_SIZE.x);
        let y = rng.random_range(0..CANVAS_SIZE.y);

        let [r, g, b, a] = shared::random_colour(rng);
        let colour = pack_rgba8([r, g, b, a]);

        positions.push(UVec2::new(x, y));
        rgba_u32.push(colour);
    }

    draw_pixels_msg.write(DrawPixels { positions, rgba_u32 });
}
