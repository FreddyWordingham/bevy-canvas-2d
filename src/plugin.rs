use bevy::prelude::*;

use super::config::CanvasConfig;

/// Plugin for a chunked 2D canvas.
pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, _app: &mut App) {}
}
