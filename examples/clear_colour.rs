use bevy::prelude::*;
use bevy_canvas_2d::prelude::*;
use rand::{Rng, SeedableRng, rng};
use rand_chacha::ChaCha8Rng;

// -- Resources --

#[derive(Resource)]
pub struct SeededRng {
    seed: u64,
    rng: ChaCha8Rng,
}

impl Default for SeededRng {
    fn default() -> Self {
        let seed = rng().random();
        Self::new(seed)
    }
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        SeededRng {
            seed,
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    // -- Getters --

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn rng(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }
}

// -- Main --

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CanvasPlugin {
            config: CanvasConfig::default(),
        })
        .init_resource::<SeededRng>()
        .insert_resource(Time::<Fixed>::from_hz(1.0))
        .add_systems(Startup, spawn_camera)
        .add_systems(FixedUpdate, clear_colour)
        .run();
}

// -- Systems --

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
    ));
}

fn clear_colour(mut seeded_rng: ResMut<SeededRng>, mut clear_canvas_msg: MessageWriter<ClearCanvas>) {
    let rng = seeded_rng.rng();

    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    let a: u8 = 255;
    let colour = pack_rgba8([r, g, b, a]);

    clear_canvas_msg.write(ClearCanvas { rgba_u32: colour });
}
