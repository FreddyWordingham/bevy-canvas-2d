//! Message types for drawing to the canvas.
//!
//! These messages are consumed by the canvas systems each update,
//! translated into CPU writes and dirty-rect tracking,
//! and finally into GPU upload operations.

use bevy::prelude::*;

/// Set all canvas pixels to a single colour.
#[derive(Message)]
pub struct ClearCanvas {
    /// Colour to clear with.
    pub rgba_u32: u32,
}

/// Draw a single pixel to the canvas.
#[derive(Message)]
pub struct DrawPixel {
    /// Canvas coords, bottom-left origin.
    pub pos: UVec2,
    /// Pixel colour.
    pub rgba_u32: u32,
}

/// Draw many independent pixels to the canvas.
///
/// `positions.len()` must equal `rgba_u32.len()`.
#[derive(Message)]
pub struct DrawPixels {
    pub positions: Vec<UVec2>,
    pub rgba_u32: Vec<u32>,
}

/// Draw a rectangular region to the canvas.
///
/// The region will wrap toroidally if it exceeds canvas bounds.
/// `rgba_u32` is row-major: index = y*width + x.
#[derive(Message)]
pub struct DrawRect {
    pub start: UVec2,
    pub size: UVec2,
    pub rgba_u32: Vec<u32>,
}

/// Draw a contiguous row-major stream to the canvas.
///
/// It advances across X, then moves up a row, and wraps at edges.
#[derive(Message)]
pub struct DrawSpan {
    pub start: UVec2,
    pub rgba_u32: Vec<u32>,
}
