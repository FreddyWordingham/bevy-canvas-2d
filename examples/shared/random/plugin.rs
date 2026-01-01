use bevy::prelude::*;

use super::resources::SeededRng;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.init_resource::<SeededRng>();
    }
}
