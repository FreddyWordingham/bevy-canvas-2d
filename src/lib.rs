mod components;
mod config;
mod messages;
mod plugin;
mod resources;
mod settings;
mod systems;
mod types;
mod utils;

pub mod prelude {
    pub use super::{
        config::CanvasConfig,
        messages::{ClearCanvas, DrawPixel, DrawPixels, DrawRect, DrawSpan},
        plugin::CanvasPlugin,
    };
}
