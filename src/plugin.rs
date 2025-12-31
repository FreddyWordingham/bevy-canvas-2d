use bevy::prelude::*;

use super::config::CanvasConfig;

/// Plugin for a chunked 2D canvas.
pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(self.config.clone());
    }
}
