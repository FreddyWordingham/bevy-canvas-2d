use bevy::prelude::*;

use super::systems::{pan_camera, spawn_camera, zoom_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Systems
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (zoom_camera, pan_camera));
    }
}
