use bevy::{
    prelude::*,
    render::{Render, RenderApp, RenderSystems, extract_resource::ExtractResourcePlugin},
};

use super::{
    config::CanvasConfig,
    messages::{ClearCanvas, DrawPixel, DrawPixels, DrawRect, DrawSpan},
    resources::{CanvasImageHandles, CanvasUploadOps},
    systems::{apply_canvas_uploads, collect_ops, spawn_canvas},
};

/// Plugin for a chunked 2D canvas.
pub struct CanvasPlugin {
    pub config: CanvasConfig,
}

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        // Plugins.
        app.add_plugins(ExtractResourcePlugin::<CanvasUploadOps>::default());

        // Messages
        app.add_message::<ClearCanvas>()
            .add_message::<DrawPixel>()
            .add_message::<DrawPixels>()
            .add_message::<DrawRect>()
            .add_message::<DrawSpan>();

        // Resources
        app.insert_resource(self.config.clone());

        // Systems
        app.add_systems(Startup, spawn_canvas)
            .add_systems(Update, collect_ops.run_if(resource_exists::<CanvasImageHandles>));

        // Render-world system
        app.sub_app_mut(RenderApp)
            .add_systems(Render, apply_canvas_uploads.in_set(RenderSystems::Queue));
    }
}
