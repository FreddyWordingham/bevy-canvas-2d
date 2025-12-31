use bevy::prelude::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, _app: &mut App) {}
}

pub mod prelude {
    pub use super::CanvasPlugin;
}
