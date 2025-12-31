use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use super::{components::CanvasImage, config::CanvasConfig, utils};

/// Spawn chunk images/sprites, and initialise CPU resources.
pub fn spawn_canvas(mut commands: Commands, config: Res<CanvasConfig>, mut images: ResMut<Assets<Image>>) {
    let num_chunks = config.num_chunks();
    let chunk_size = config.chunk_size();
    let pixels_per_chunk = config.pixels_per_chunk();

    // Initialise GPU images with the clear colour as raw RGBA8 bytes.
    let clear_colour_bytes = utils::unpack_rgba8(config.clear_colour());
    let mut data = vec![0u8; pixels_per_chunk * 4];
    for px in data.chunks_exact_mut(4) {
        px.copy_from_slice(&clear_colour_bytes);
    }

    // Build chunk images and spawn sprites arranged in a grid.
    let mut image_handles = Vec::with_capacity(config.total_chunks());
    for y in 0..num_chunks.y {
        for x in 0..num_chunks.x {
            let mut image = Image::new(
                Extent3d {
                    width: chunk_size.x,
                    height: chunk_size.y,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                data.clone(),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );

            // These textures will be updated during runtime.
            image.texture_descriptor.usage |= TextureUsages::COPY_DST;

            // Use nearest-neighbour sampling to avoid blurring pixel art.
            image.sampler = ImageSampler::nearest();

            // Store image handle for later access.
            let handle = images.add(image);
            image_handles.push(handle.clone());

            // Position the chunk sprites in a grid, centred around the origin.
            commands.spawn((
                CanvasImage,
                Sprite::from_image(handle),
                Transform::from_translation(Vec3::new(
                    (x as f32 - ((num_chunks.x - 1) as f32 / 2.0)) * chunk_size.x as f32,
                    (y as f32 - ((num_chunks.y - 1) as f32 / 2.0)) * chunk_size.y as f32,
                    config.canvas_z_index(),
                ))
                // Flip Y so that canvas coords are bottom-left origin.
                .with_scale(Vec3::new(1.0, -1.0, 1.0)),
            ));
        }
    }
}
