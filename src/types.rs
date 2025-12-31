//! Internal types and pure geometry helpers.

use bevy::{math::U8Vec2, prelude::*};

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

    /// Number of chunks in each axis.
    #[inline]
    pub fn num_chunks(self) -> U8Vec2 {
        (self.canvas_size / self.chunk_size).as_u8vec2()
    }

    /// Toroidal wrap within canvas bounds.
    #[inline]
    pub fn wrap(self, pos: UVec2) -> UVec2 {
        pos % self.canvas_size
    }

    /// Convert a chunk coordinate into a compact `U8Vec2` key.
    /// Chunk count must fit within 8-bit per axis (<=255).
    #[inline]
    pub fn chunk_key(self, chunk_xy: UVec2) -> U8Vec2 {
        debug_assert!(chunk_xy.x < 256 && chunk_xy.y < 256);
        U8Vec2::new(chunk_xy.x as u8, chunk_xy.y as u8)
    }

    /// Chunk coordinate (in chunk grid space) for a wrapped pixel.
    #[inline]
    pub fn chunk_xy(self, wrapped_pos: UVec2) -> UVec2 {
        wrapped_pos / self.chunk_size
    }

    /// Convert a wrapped pixel to chunk-local pixel coordinates.
    #[inline]
    pub fn local_xy(self, wrapped_pos: UVec2) -> UVec2 {
        let chunk_xy = self.chunk_xy(wrapped_pos);
        wrapped_pos - self.chunk_min(chunk_xy)
    }

    /// Pixel-space origin (min corner) of a chunk in canvas coordinates.
    #[inline]
    pub fn chunk_min(self, chunk_xy: UVec2) -> UVec2 {
        chunk_xy * self.chunk_size
    }

    /// Maximum contiguous pixels you can write starting at `wrapped_pos`,
    /// without crossing the canvas row end or the chunk row end.
    ///
    /// This is the fundamental constraint enabling `write_run` to be safe.
    #[inline]
    pub fn max_run_len(self, wrapped_pos: UVec2) -> u32 {
        let local = self.local_xy(wrapped_pos);
        let until_canvas_row_end = self.canvas_size.x - wrapped_pos.x;
        let until_chunk_row_end = self.chunk_size.x - local.x;
        until_canvas_row_end.min(until_chunk_row_end)
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
