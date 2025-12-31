//! Internal types and pure geometry helpers.

use bevy::prelude::*;

/// Canvas geometry helper; encapsulates common coordinate transforms:
/// - wrapping toroidally within the canvas
/// - locating which chunk a pixel belongs to
/// - computing chunk-local coordinates
#[derive(Clone, Copy)]
pub struct CanvasLayout {
    /// Full canvas size in pixels.
    pub canvas_size: UVec2,
    /// Chunk size in pixels.
    pub chunk_size: UVec2,
}

impl CanvasLayout {
    /// Create a new layout with validation.
    #[inline]
    pub fn new(canvas_size: UVec2, chunk_size: UVec2) -> Self {
        debug_assert!(canvas_size.x > 0 && canvas_size.y > 0);
        debug_assert!(chunk_size.x > 0 && chunk_size.y > 0);
        Self { canvas_size, chunk_size }
    }

    /// Toroidal wrap within canvas bounds.
    #[inline]
    pub fn wrap(self, pos: UVec2) -> UVec2 {
        pos % self.canvas_size
    }
}

/// Dirty rectangle for one chunk (chunk-local pixel space).
/// Stored as inclusive min/max.
/// `dirty=false` means ignore min/max.
#[derive(Clone, Copy)]
pub struct DirtyRect {
    pub min: UVec2,
    pub max: UVec2,
    pub dirty: bool,
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            min: UVec2::ZERO,
            max: UVec2::ZERO,
            dirty: false,
        }
    }
}

/// A single GPU upload operation for a chunk image.
///
/// `bytes` contains tightly packed rows with `bytes_per_row` stride (aligned).
#[derive(Clone)]
pub struct CanvasUploadOp {
    /// Handle to the chunk image to upload to.
    pub handle: Handle<Image>,
    /// Start (x, y) in the chunk texture (pixels).
    pub start: UVec2,
    /// Size (width, height) in pixels.
    pub size: UVec2,
    /// Row stride in bytes (must be aligned as required by wgpu).
    pub bytes_per_row: u32,
    /// Raw bytes sent to the GPU (RGBA8).
    pub bytes: Vec<u8>,
}
