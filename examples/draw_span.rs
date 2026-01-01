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
        .add_systems(Update, draw_span)
        .run();
}

fn draw_span(mut draw_span_msg: MessageWriter<DrawSpan>, mut seeded_rng: ResMut<shared::SeededRng>, mut counter: Local<usize>) {
    let rng = seeded_rng.rng();

    let [r, g, b, a] = shared::random_colour(rng);
    let colour = pack_rgba8([r, g, b, a]);

    let l = rng.random_range(1..=128) as usize;

    draw_span_msg.write(DrawSpan {
        start: counter_to_pos(*counter),
        rgba_u32: vec![colour; l],
    });

    *counter += l;
    *counter %= (CANVAS_SIZE.x * CANVAS_SIZE.y) as usize;
}

// -- Helpers --

pub fn counter_to_pos(count: usize) -> UVec2 {
    let x = (count as u32) % CANVAS_SIZE.x;
    let y = (count as u32) / CANVAS_SIZE.x;

    UVec2::new(x, y)
}
