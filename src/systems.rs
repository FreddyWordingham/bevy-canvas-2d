use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    math::U8Vec2,
    prelude::*,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo, TextureAspect, TextureDimension, TextureFormat,
            TextureUsages,
        },
        renderer::RenderQueue,
        texture::GpuImage,
    },
};

use super::{
    components::CanvasImage,
    config::CanvasConfig,
    resources::{CanvasCpuChunks, CanvasDirtyRects, CanvasImageHandles, CanvasUploadOps},
    utils,
};

/// Spawn chunk images/sprites, and initialise CPU resources.
pub fn spawn_canvas(mut commands: Commands, config: Res<CanvasConfig>, mut images: ResMut<Assets<Image>>) {
    let num_chunks = config.num_chunks();
    let chunk_size = config.chunk_size();
    let pixels_per_chunk = config.pixels_per_chunk();
    let clear_colour = config.clear_colour();

    // Initialise GPU images with the clear colour as raw RGBA8 bytes
    let clear_colour_bytes = utils::unpack_rgba8(clear_colour);
    let mut data = vec![0u8; pixels_per_chunk * 4];
    for px in data.chunks_exact_mut(4) {
        px.copy_from_slice(&clear_colour_bytes);
    }

    // Build chunk images and spawn sprites arranged in a grid
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

            // These textures will be updated during runtime
            image.texture_descriptor.usage |= TextureUsages::COPY_DST;

            // Use nearest-neighbour sampling to avoid blurring pixel art
            image.sampler = ImageSampler::nearest();

            // Store image handle for later access.
            let handle = images.add(image);
            image_handles.push(handle.clone());

            // Position the chunk sprites in a grid, centred around the origin
            commands.spawn((
                CanvasImage,
                Sprite::from_image(handle),
                Transform::from_translation(Vec3::new(
                    (x as f32 - ((num_chunks.x - 1) as f32 / 2.0)) * chunk_size.x as f32,
                    (y as f32 - ((num_chunks.y - 1) as f32 / 2.0)) * chunk_size.y as f32,
                    config.canvas_z_index(),
                ))
                // Flip Y so that canvas coords are bottom-left origin
                .with_scale(Vec3::new(1.0, -1.0, 1.0)),
            ));
        }
    }

    // Store image handles
    commands.insert_resource(CanvasImageHandles::new(num_chunks, image_handles));

    // CPU chunks store packed pixels
    commands.insert_resource(CanvasCpuChunks::new(num_chunks, chunk_size, clear_colour));

    // Dirty rect tracking and upload ops buffer
    commands.insert_resource(CanvasDirtyRects::new(num_chunks, chunk_size));
    commands.insert_resource(CanvasUploadOps::default());
}

/// Render-world system.
/// Apply pending upload ops to GPU textures.
pub fn apply_canvas_uploads(
    mut uploads: ResMut<CanvasUploadOps>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    render_queue: Res<RenderQueue>,
) {
    if uploads.ops.is_empty() {
        return;
    }

    for op in uploads.ops.drain(..) {
        // If GPU image is not ready yet, so skip
        let Some(gpu) = gpu_images.get(op.handle.id()) else {
            continue;
        };

        // Sanity checks
        debug_assert!(op.bytes_per_row % 256 == 0);
        debug_assert_eq!(op.bytes.len(), (op.bytes_per_row as usize) * (op.size.y as usize));
        debug_assert!(op.start.x + op.size.x <= gpu.size.width);
        debug_assert!(op.start.y + op.size.y <= gpu.size.height);

        // Write raw RGBA8 bytes into the GPU texture
        render_queue.write_texture(
            TexelCopyTextureInfo {
                texture: &gpu.texture,
                mip_level: 0,
                origin: Origin3d {
                    x: op.start.x,
                    y: op.start.y,
                    z: 0,
                },
                aspect: TextureAspect::All,
            },
            &op.bytes,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(op.bytes_per_row),
                rows_per_image: Some(op.size.y),
            },
            Extent3d {
                width: op.size.x,
                height: op.size.y,
                depth_or_array_layers: 1,
            },
        );
    }
}
