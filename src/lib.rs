mod config;
mod messages;
mod plugin;
mod settings;

pub mod prelude {
    pub use super::{
        config::CanvasConfig,
        messages::{ClearCanvas, DrawPixel, DrawPixels, DrawRect, DrawSpan},
        plugin::CanvasPlugin,
    };
}
