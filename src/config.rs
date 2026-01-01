//! Canvas configuration resource.
//!
//! The canvas is split into a grid of chunks.
//! Each chunk is backed by a CPU `Vec<u32>` (packed RGBA8) and a GPU `Image`.
//! The GPU images are updated with partial texture uploads using dirty rectangles.

use bevy::{math::U8Vec2, prelude::*};

use super::settings::{DEFAULT_CANVAS_SIZE, DEFAULT_CANVAS_Z_INDEX, DEFAULT_CLEAR_COLOUR, DEFAULT_NUM_CHUNKS};

/// Runtime configuration for the canvas plugin.
#[derive(Resource, Clone)]
pub struct CanvasConfig {
    /// Clear colour used to initialise the GPU images and CPU buffers.
    pub clear_colour: u32,

    /// Z-index used for the chunk sprites.
    pub canvas_z_index: f32,

    /// Total canvas pixel resolution.
    pub canvas_size: UVec2,

    /// Number of chunks in (x, y). Must evenly divide `canvas_size`.
    pub num_chunks: U8Vec2,
}

impl CanvasConfig {
    /// Construct a new configuration with validation.
    ///
    /// # Panics / Debug asserts
    /// - `canvas_size` must be non-zero in both axes
    /// - `chunks` must be non-zero in both axes
    /// - Each axis must be exactly divisible by the corresponding chunk count
    pub fn new(clear_colour: u32, canvas_z_index: f32, canvas_size: UVec2, chunks: U8Vec2) -> Self {
        debug_assert!(canvas_size.x > 0);
        debug_assert!(canvas_size.y > 0);
        debug_assert!(chunks.x > 0);
        debug_assert!(chunks.y > 0);
        debug_assert!(canvas_size.x.is_multiple_of(chunks.x as u32));
        debug_assert!(canvas_size.y.is_multiple_of(chunks.y as u32));

        Self {
            clear_colour,
            canvas_z_index,
            canvas_size,
            num_chunks: chunks,
        }
    }

    /// Clear colour (packed RGBA8).
    #[inline]
    pub fn clear_colour(&self) -> u32 {
        self.clear_colour
    }

    /// Z-index used for the canvas sprites.
    #[inline]
    pub fn canvas_z_index(&self) -> f32 {
        self.canvas_z_index
    }

    /// Total canvas size in pixels.
    #[inline]
    pub fn canvas_size(&self) -> UVec2 {
        self.canvas_size
    }

    /// Chunk grid resolution.
    #[inline]
    pub fn num_chunks(&self) -> U8Vec2 {
        self.num_chunks
    }

    /// Size of one chunk in pixels.
    #[inline]
    pub fn chunk_size(&self) -> UVec2 {
        UVec2::new(
            self.canvas_size.x / self.num_chunks.x as u32,
            self.canvas_size.y / self.num_chunks.y as u32,
        )
    }

    /// Pixels per chunk.
    #[inline]
    pub fn pixels_per_chunk(&self) -> usize {
        let chunk_size = self.chunk_size();
        (chunk_size.x as usize) * (chunk_size.y as usize)
    }

    /// Total number of chunks.
    #[inline]
    pub fn total_chunks(&self) -> usize {
        (self.num_chunks.x as usize) * (self.num_chunks.y as usize)
    }
}

impl Default for CanvasConfig {
    fn default() -> Self {
        Self {
            clear_colour: DEFAULT_CLEAR_COLOUR,
            canvas_z_index: DEFAULT_CANVAS_Z_INDEX,
            canvas_size: DEFAULT_CANVAS_SIZE,
            num_chunks: DEFAULT_NUM_CHUNKS,
        }
    }
}
