use bevy::prelude::*;
use bevy_canvas_2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(CanvasPlugin {
            config: CanvasConfig::default(),
        })
        .run();
}
