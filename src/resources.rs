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

/// CPU backing store: row-major `u32` pixels per chunk.
#[derive(Resource)]
pub struct CanvasCpuChunks {
    num_chunks: U8Vec2,
    chunk_size: UVec2,
    chunk_data: Vec<Vec<u32>>,
}

impl CanvasCpuChunks {
    /// Create CPU chunks, filling all pixels with `default_colour`.
    pub fn new(num_chunks: U8Vec2, chunk_size: UVec2, default_colour: u32) -> Self {
        debug_assert!(num_chunks.x > 0);
        debug_assert!(num_chunks.y > 0);
        debug_assert!(chunk_size.x > 0);
        debug_assert!(chunk_size.y > 0);

        let total_chunks = (num_chunks.x as usize) * (num_chunks.y as usize);
        let pixels_per_chunk = (chunk_size.x as usize) * (chunk_size.y as usize);

        let mut chunk_data = Vec::with_capacity(total_chunks);
        for _ in 0..total_chunks {
            chunk_data.push(vec![default_colour; pixels_per_chunk]);
        }

        Self {
            num_chunks,
            chunk_size,
            chunk_data,
        }
    }

    /// Chunk row stride in pixels.
    #[inline]
    pub fn stride(&self) -> usize {
        self.chunk_size.x as usize
    }

    /// Convert chunk position key (x,y) to a linear index.
    #[inline]
    fn index(&self, chunk_key: &U8Vec2) -> usize {
        chunk_key.y as usize * self.num_chunks.x as usize + chunk_key.x as usize
    }

    /// Borrow a chunk by linear index.
    pub fn chunk(&self, index: usize) -> &[u32] {
        debug_assert!(index < self.chunk_data.len());
        &self.chunk_data[index]
    }

    /// Borrow a chunk mutably by position key (x,y).
    #[inline]
    pub fn chunk_mut(&mut self, chunk_key: &U8Vec2) -> &mut [u32] {
        debug_assert!(chunk_key.x < self.num_chunks.x);
        debug_assert!(chunk_key.y < self.num_chunks.y);
        let idx = self.index(chunk_key);
        &mut self.chunk_data[idx]
    }

    /// Mutably borrow all chunks.
    #[inline]
    pub fn chunks_mut(&mut self) -> &mut [Vec<u32>] {
        &mut self.chunk_data
    }
}
