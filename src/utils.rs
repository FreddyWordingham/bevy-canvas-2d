//! Utility helpers for pixel packing/unpacking.
//!
//! Note packing is standardised to little-endian RGBA8,
//! use the `pack_rgba8` function to create compatible messages.

/// Pack RGBA8 bytes into a `u32` using little-endian encoding.
///
/// The resulting `u32` should be treated as an opaque pixel token;
/// when uploading to the GPU it is converted back using `unpack_rgba8`.
#[inline(always)]
pub fn pack_rgba8(r: u8, g: u8, b: u8, a: u8) -> u32 {
    u32::from_le_bytes([r, g, b, a])
}

/// Unpack a `u32` pixel (little-endian RGBA8) into `[r,g,b,a]`.
#[inline(always)]
pub fn unpack_rgba8(px: u32) -> [u8; 4] {
    px.to_le_bytes()
}
