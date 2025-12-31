//! Utility helpers for pixel packing/unpacking.
//!
//! Note packing is standardised to little-endian RGBA8,
//! use the `pack_rgba8` function to create compatible messages.

/// Pack RGBA8 bytes into a `u32` using little-endian encoding.
///
/// The resulting `u32` should be treated as an opaque pixel token;
/// when uploading to the GPU it is converted back using `unpack_rgba8`.
#[inline(always)]
pub fn pack_rgba8(colour: [u8; 4]) -> u32 {
    u32::from_le_bytes(colour)
}

/// Unpack a `u32` pixel (little-endian RGBA8) into `[r,g,b,a]`.
#[inline(always)]
pub fn unpack_rgba8(colour: u32) -> [u8; 4] {
    colour.to_le_bytes()
}
