//! Resources used by the canvas.

use bevy::{math::U8Vec2, prelude::*};

/// Stores the `Image` handles for each chunk.
///
/// This allows upload ops to reference the correct GPU `Image`.
#[derive(Resource, Default)]
pub struct CanvasImageHandles {
    handles: Vec<Handle<Image>>,
}

impl CanvasImageHandles {
    /// Create chunk handle storage.
    ///
    /// # Debug asserts
    /// - `handles.len()` must match `num_chunks.x * num_chunks.y`
    pub fn new(num_chunks: U8Vec2, handles: Vec<Handle<Image>>) -> Self {
        debug_assert!(num_chunks.x > 0);
        debug_assert!(num_chunks.y > 0);

        let total_chunks = (num_chunks.x as usize) * (num_chunks.y as usize);
        debug_assert_eq!(handles.len(), total_chunks);

        Self { handles }
    }

    /// Get handle by linear chunk index (row-major chunk ordering).
    #[inline(always)]
    pub fn handle(&self, index: usize) -> &Handle<Image> {
        &self.handles[index]
    }
}
