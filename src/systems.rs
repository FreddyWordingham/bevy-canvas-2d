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
    messages::{ClearCanvas, DrawPixel, DrawPixels, DrawRect, DrawSpan},
    resources::{CanvasCpuChunks, CanvasDirtyRects, CanvasImageHandles, CanvasUploadOps},
    types::{CanvasLayout, CanvasUploadOp},
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

/// Consume messages, write CPU buffers, compute upload ops.
pub fn collect_ops(
    mut clear_canvas_msg: MessageReader<ClearCanvas>,
    mut draw_pixel_msg: MessageReader<DrawPixel>,
    mut draw_pixels_msg: MessageReader<DrawPixels>,
    mut draw_rect_msg: MessageReader<DrawRect>,
    mut draw_span_msg: MessageReader<DrawSpan>,
    config: Res<CanvasConfig>,
    canvas_image_handles: Res<CanvasImageHandles>,
    mut canvas_cpu_chunks: ResMut<CanvasCpuChunks>,
    mut canvas_dirty_rects: ResMut<CanvasDirtyRects>,
    mut canvas_upload_ops: ResMut<CanvasUploadOps>,
) {
    let layout = CanvasLayout::new(config.canvas_size(), config.chunk_size());

    // Clear whole canvas
    for ClearCanvas { rgba_u32 } in clear_canvas_msg.read() {
        clear_canvas(&mut canvas_cpu_chunks, &mut canvas_dirty_rects, layout, *rgba_u32);
    }

    // Single pixels
    for DrawPixel { pos, rgba_u32 } in draw_pixel_msg.read() {
        blit_pixel(&mut canvas_cpu_chunks, &mut canvas_dirty_rects, layout, *pos, *rgba_u32);
    }

    // Many independent pixels
    for DrawPixels { positions, rgba_u32 } in draw_pixels_msg.read() {
        if positions.len() != rgba_u32.len() {
            warn!(
                "DrawPixels length mismatch (positions {}, rgba_u32 {})",
                positions.len(),
                rgba_u32.len()
            );
            continue;
        }
        blit_pixels(&mut canvas_cpu_chunks, &mut canvas_dirty_rects, layout, positions, rgba_u32);
    }

    // Rect writes (row-major)
    for DrawRect { start, size, rgba_u32 } in draw_rect_msg.read() {
        if size.x == 0 || size.y == 0 {
            continue;
        }
        let expected = (size.x * size.y) as usize;
        if rgba_u32.len() != expected {
            warn!(
                "DrawRect rgba_u32 length mismatch (expected {}, got {})",
                expected,
                rgba_u32.len()
            );
            continue;
        }

        blit_rect_row_major(
            &mut canvas_cpu_chunks,
            &mut canvas_dirty_rects,
            layout,
            *start,
            *size,
            rgba_u32,
        );
    }

    // Span writes (row-major stream)
    for DrawSpan { start, rgba_u32 } in draw_span_msg.read() {
        if rgba_u32.is_empty() {
            continue;
        }
        blit_span_row_major(&mut canvas_cpu_chunks, &mut canvas_dirty_rects, layout, *start, rgba_u32);
    }

    // Convert dirty rects into upload ops for the render world.
    build_upload_ops(
        &canvas_image_handles,
        &canvas_cpu_chunks,
        &mut canvas_dirty_rects,
        &mut canvas_upload_ops,
        layout,
    );
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

// -- Helpers --

/// Clear the entire canvas to a single colour.
/// This fills every CPU chunk and each fully dirty.
#[inline]
fn clear_canvas(cpu: &mut CanvasCpuChunks, dirty: &mut CanvasDirtyRects, layout: CanvasLayout, rgba_u32: u32) {
    let num_chunks = layout.num_chunks();

    // Fill every CPU chunk
    for chunk in cpu.chunks_mut().iter_mut() {
        chunk.fill(rgba_u32);
    }

    // Mark every chunk as fully dirty
    for y in 0..num_chunks.y {
        for x in 0..num_chunks.x {
            let chunk_key = U8Vec2::new(x, y);
            dirty.mark_rect(&chunk_key, UVec2::ZERO, layout.chunk_size);
        }
    }
}

/// Draw a single pixel.
#[inline]
fn blit_pixel(cpu: &mut CanvasCpuChunks, dirty: &mut CanvasDirtyRects, layout: CanvasLayout, pos: UVec2, rgba_u32: u32) {
    write_run(cpu, dirty, layout, pos, core::slice::from_ref(&rgba_u32));
}

/// Draw many independent pixels.
#[inline]
fn blit_pixels(
    cpu: &mut CanvasCpuChunks,
    dirty: &mut CanvasDirtyRects,
    layout: CanvasLayout,
    positions: &[UVec2],
    rgba_u32: &[u32],
) {
    debug_assert_eq!(positions.len(), rgba_u32.len());

    for (pos, colour) in positions.iter().copied().zip(rgba_u32.iter().copied()) {
        write_run(cpu, dirty, layout, pos, core::slice::from_ref(&colour));
    }
}

/// Draw a row-major stream starting at `start`, wrapping at canvas edges.
#[inline]
fn blit_span_row_major(
    cpu: &mut CanvasCpuChunks,
    dirty: &mut CanvasDirtyRects,
    layout: CanvasLayout,
    start: UVec2,
    src_u32: &[u32],
) {
    if src_u32.is_empty() {
        return;
    }

    // Cursor is always a wrapped canvas coordinate
    let mut cursor = layout.wrap(start);
    let mut src_index = 0;
    let mut remaining = src_u32.len();

    while remaining > 0 {
        // Take the longest safe run at this cursor
        let max_run = layout.max_run_len(cursor) as usize;
        let run = remaining.min(max_run);
        debug_assert!(run > 0);

        write_run(cpu, dirty, layout, cursor, &src_u32[src_index..src_index + run]);

        src_index += run;
        remaining -= run;

        // Advance cursor in row-major order across the canvas
        cursor.x += run as u32;
        if cursor.x == layout.canvas_size.x {
            cursor.x = 0;
            cursor.y += 1;
            if cursor.y == layout.canvas_size.y {
                cursor.y = 0;
            }
        }
    }
}

/// Draw a row-major rectangle, with toroidal wrap.
#[inline]
fn blit_rect_row_major(
    cpu: &mut CanvasCpuChunks,
    dirty: &mut CanvasDirtyRects,
    layout: CanvasLayout,
    start: UVec2,
    size: UVec2,
    src_u32: &[u32],
) {
    if size.x == 0 || size.y == 0 {
        return;
    }
    debug_assert_eq!(src_u32.len(), (size.x * size.y) as usize);

    // Source row stride is the rect width
    let row_stride = size.x as usize;

    // For each source row, write across the canvas, wrapping X as needed
    for row in 0..size.y {
        let y = (start.y + row) % layout.canvas_size.y;
        let mut x = start.x % layout.canvas_size.x;

        let mut src_col = 0usize;
        let mut remaining = row_stride;

        while remaining > 0 {
            let pos = UVec2::new(x, y);
            let max_run = layout.max_run_len(pos) as usize;
            let run = remaining.min(max_run);
            debug_assert!(run > 0);

            let src_row_start = row as usize * row_stride;
            let src_start = src_row_start + src_col;

            write_run(cpu, dirty, layout, pos, &src_u32[src_start..src_start + run]);

            src_col += run;
            remaining -= run;

            x += run as u32;
            if x == layout.canvas_size.x {
                x = 0;
            }
        }
    }
}

/// Writes a contiguous run on a single scanline into chunk.
///
/// `src.len()` must not cross the end of the canvas row, or the end of the chunk row.
/// This should be enforced by using `layout.max_run_len(p)` at call sites.
#[inline]
fn write_run(cpu: &mut CanvasCpuChunks, dirty: &mut CanvasDirtyRects, layout: CanvasLayout, dst_start: UVec2, src: &[u32]) {
    if src.is_empty() {
        return;
    }

    // Toroidal wrap the starting coordinate
    let pos = layout.wrap(dst_start);

    // Validate that the run is boundary-safe
    let max_run = layout.max_run_len(pos) as usize;
    debug_assert!(
        src.len() <= max_run,
        "write_run called with a run that crosses a row boundary"
    );

    // Resolve destination chunk and chunk-local coords
    let chunk_xy = layout.chunk_xy(pos);
    let chunk_key = layout.chunk_key(chunk_xy);
    let local = layout.local_xy(pos);

    // CPU chunk stride (pixels per row)
    let stride = cpu.stride();

    // Write into chunk-local row-major storage
    let dst = cpu.chunk_mut(&chunk_key);
    let dst_index = local.y as usize * stride + local.x as usize;

    debug_assert!(dst_index + src.len() <= dst.len());
    dst[dst_index..dst_index + src.len()].copy_from_slice(src);

    // Dirty rect: mark the span (width = run, height = 1)
    dirty.mark_rect(&chunk_key, local, UVec2::new(src.len() as u32, 1));
}

/// Convert per-chunk dirty rects into GPU upload ops.
///
/// WGPU requires `bytes_per_row` to be aligned to 256 bytes.
/// For RGBA8, that's 4 bytes/px => 64 pixels alignment.
#[inline]
fn build_upload_ops(
    canvas_image_handles: &CanvasImageHandles,
    canvas_cpu_chunks: &CanvasCpuChunks,
    canvas_dirty_rects: &mut CanvasDirtyRects,
    canvas_upload_ops: &mut CanvasUploadOps,
    layout: CanvasLayout,
) {
    canvas_upload_ops.ops.clear();

    let chunk_w = layout.chunk_size.x;
    let chunk_h = layout.chunk_size.y;

    // 256-byte alignment / 4 bytes per pixel = 64 pixels
    const ROW_ALIGN_PX: u32 = 64;

    for chunk_index in 0..canvas_dirty_rects.len() {
        let Some((min, max)) = canvas_dirty_rects.take(chunk_index) else {
            continue;
        };

        // Convert inclusive max to exclusive range [min, max_ex)
        let min_ex = min;
        let max_ex = max + UVec2::ONE;

        // Clamp defensively to chunk bounds
        let min_ex = UVec2::new(
            min_ex.x.min(chunk_w.saturating_sub(1)),
            min_ex.y.min(chunk_h.saturating_sub(1)),
        );
        let max_ex = UVec2::new(max_ex.x.min(chunk_w), max_ex.y.min(chunk_h));

        // Pad X to satisfy bytes_per_row alignment
        let padding_min_x = (min_ex.x / ROW_ALIGN_PX) * ROW_ALIGN_PX;
        let padding_max_x = (max_ex.x.div_ceil(ROW_ALIGN_PX) * ROW_ALIGN_PX).min(chunk_w);

        let padded_width = padding_max_x.saturating_sub(padding_min_x);
        if padded_width == 0 {
            continue;
        }

        let height = max_ex.y.saturating_sub(min_ex.y);
        if height == 0 {
            continue;
        }

        let bytes_per_row = padded_width * 4;
        debug_assert_eq!(bytes_per_row % 256, 0);

        let handle = canvas_image_handles.handle(chunk_index).clone();
        let chunk = canvas_cpu_chunks.chunk(chunk_index);

        // Allocate full upload bytes once
        let mut bytes = Vec::with_capacity((bytes_per_row as usize) * (height as usize));
        let row_stride = chunk_w as usize;

        // Reuseable temp row buffer to avoid many small allocations
        let mut row_bytes = vec![0u8; (padded_width as usize) * 4];

        for y in min_ex.y..max_ex.y {
            let row_start = (y as usize) * row_stride;

            let x0 = padding_min_x as usize;
            let x1 = padding_max_x as usize;

            let src_u32 = &chunk[row_start + x0..row_start + x1];

            for (i, &px) in src_u32.iter().enumerate() {
                row_bytes[i * 4..i * 4 + 4].copy_from_slice(&utils::unpack_rgba8(px));
            }

            bytes.extend_from_slice(&row_bytes);
        }

        canvas_upload_ops.ops.push(CanvasUploadOp {
            handle,
            start: UVec2::new(padding_min_x, min_ex.y),
            size: UVec2::new(padded_width, height),
            bytes_per_row,
            bytes,
        });
    }
}
