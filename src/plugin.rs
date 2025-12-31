use bevy::prelude::*;

use super::{
    config::CanvasConfig,
    messages::{ClearCanvas, DrawPixel, DrawPixels, DrawRect, DrawSpan},
    systems::spawn_canvas,
};

/// Plugin for a chunked 2D canvas.
pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        // Messages
        app.add_message::<ClearCanvas>()
            .add_message::<DrawPixel>()
            .add_message::<DrawPixels>()
            .add_message::<DrawRect>()
            .add_message::<DrawSpan>();

        // Resources
        app.insert_resource(self.config.clone());

        // Systems
        app.add_systems(Update, spawn_canvas);
    }
}
