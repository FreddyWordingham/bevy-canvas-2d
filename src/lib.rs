use bevy::prelude::*;

pub mod config;

use config::*;

/// Plugin for a chunked 2D canvas.
pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, _app: &mut App) {}
}

pub mod prelude {
    pub use super::{CanvasPlugin, config::CanvasConfig};
}
