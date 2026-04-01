# Load and Sample Textures

> **Task:** Load an image from disk, upload it to GPU memory, and sample
> it in a fragment shader.

## Prerequisites

You should be comfortable with:
- [Memory Management](../concepts/memory.md) (staging buffers, memory types)
- [Command Buffers](../concepts/command-buffers.md) (one-shot transfers)
- [Descriptor Sets](../concepts/descriptors.md) (binding samplers)
- [Synchronization](../concepts/synchronization.md) (image layout transitions)

## Overview

Sampling a texture in Vulkan requires several steps that OpenGL handled
behind the scenes: creating a staging buffer, allocating a device-local
image, transitioning layouts with pipeline barriers, copying data, and
finally binding the image through a descriptor set. This recipe walks
through each step.

## Step 1: Load pixels from disk

Use the `image` crate to decode an image file into raw RGBA pixels.

```rust,ignore
let img = image::open("assets/texture.png")
    .expect("Failed to open image")
    .to_rgba8();

let (width, height) = img.dimensions();
let pixels = img.as_raw();
let image_size = (width * height * 4) as vk::DeviceSize; // 4 bytes per RGBA pixel
```

## Step 2: Create a staging buffer

The CPU cannot write directly to device-local memory on most hardware.
Upload the pixels into a host-visible staging buffer first.

```rust,ignore
let staging_info = vk::BufferCreateInfo::builder()
    .size(image_size)
    .usage(vk::BufferUsageFlags::TRANSFER_SRC)
    .sharing_mode(vk::SharingMode::EXCLUSIVE);

let staging_buffer = unsafe { device.create_buffer(&staging_info, None)? };
let staging_reqs = unsafe { device.get_buffer_memory_requirements(staging_buffer) };

let staging_memory = allocate_and_bind_buffer(
    device,
    staging_buffer,
    &staging_reqs,
    &mem_properties,
    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
)?;

// Map, copy pixels, unmap.
unsafe {
    let mut ptr: *mut core::ffi::c_void = core::ptr::null_mut();
    device.map_memory(
        staging_memory, 0, image_size,
        vk::MemoryMapFlags::empty(), &mut ptr,
    )?;
    core::ptr::copy_nonoverlapping(
        pixels.as_ptr(), ptr as *mut u8, image_size as usize,
    );
    device.unmap_memory(staging_memory);
}
```

> See [Memory Management](../concepts/memory.md) for the
> `allocate_and_bind_buffer` helper and the `find_memory_type` algorithm.

## Step 3: Create the device-local image

The image needs `TRANSFER_DST` (we will copy into it) and `SAMPLED`
(the fragment shader will sample it).

```rust,ignore
let image_info = vk::ImageCreateInfo::builder()
    .image_type(vk::ImageType::TYPE_2D)
    .format(vk::Format::R8G8B8A8_SRGB)
    .extent(vk::Extent3D { width, height, depth: 1 })
    .mip_levels(1)
    .array_layers(1)
    .samples(vk::SampleCountFlags::TYPE_1)
    .tiling(vk::ImageTiling::OPTIMAL)
    .usage(
        vk::ImageUsageFlags::TRANSFER_DST
        | vk::ImageUsageFlags::SAMPLED
    )
    .sharing_mode(vk::SharingMode::EXCLUSIVE)
    .initial_layout(vk::ImageLayout::UNDEFINED);

let texture_image = unsafe { device.create_image(&image_info, None)? };

// Allocate DEVICE_LOCAL memory and bind it to the image.
let img_reqs = unsafe { device.get_image_memory_requirements(texture_image) };
let texture_memory = allocate_and_bind_image(
    device, texture_image, &img_reqs, &mem_properties,
    vk::MemoryPropertyFlags::DEVICE_LOCAL,
)?;
```

## Step 4: Transition layout UNDEFINED to TRANSFER_DST_OPTIMAL

Before copying into the image, transition it to a layout the transfer
engine can write to. This requires a pipeline barrier.

> *Before reading on: why can't we just copy into an image that is in
> UNDEFINED layout? What does the layout tell the driver?*

```rust,ignore
let barrier_to_transfer = vk::ImageMemoryBarrier::builder()
    .old_layout(vk::ImageLayout::UNDEFINED)
    .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .image(texture_image)
    .subresource_range(vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    })
    // No prior access to wait for (image was UNDEFINED).
    .src_access_mask(vk::AccessFlags::empty())
    // The transfer write must wait until the transition completes.
    .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE);

unsafe {
    device.cmd_pipeline_barrier(
        cmd,
        vk::PipelineStageFlags::TOP_OF_PIPE,   // src stage: nothing before
        vk::PipelineStageFlags::TRANSFER,       // dst stage: transfer write
        vk::DependencyFlags::empty(),
        &[],             // memory barriers
        &[],             // buffer memory barriers
        &[barrier_to_transfer],
    );
}
```

> See [Synchronization](../concepts/synchronization.md) for a deeper
> explanation of pipeline barriers and access masks.

## Step 5: Copy staging buffer to image

```rust,ignore
let region = vk::BufferImageCopy {
    buffer_offset: 0,
    // 0 means tightly packed (no padding between rows).
    buffer_row_length: 0,
    buffer_image_height: 0,
    image_subresource: vk::ImageSubresourceLayers {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        mip_level: 0,
        base_array_layer: 0,
        layer_count: 1,
    },
    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
    image_extent: vk::Extent3D { width, height, depth: 1 },
};

unsafe {
    device.cmd_copy_buffer_to_image(
        cmd,
        staging_buffer,
        texture_image,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        &[region],
    );
}
```

## Step 6: Transition layout TRANSFER_DST to SHADER_READ_ONLY

After the copy, transition the image to a layout the shader can read.

```rust,ignore
let barrier_to_shader = vk::ImageMemoryBarrier::builder()
    .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
    .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    .image(texture_image)
    .subresource_range(vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    })
    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
    .dst_access_mask(vk::AccessFlags::SHADER_READ);

unsafe {
    device.cmd_pipeline_barrier(
        cmd,
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::FRAGMENT_SHADER,
        vk::DependencyFlags::empty(),
        &[], &[],
        &[barrier_to_shader],
    );
}
```

## Step 7: Create image view and sampler

The shader does not access images directly. It reads through an
**image view** (which selects format, mip levels, and array layers) and a
**sampler** (which controls filtering and addressing).

```rust,ignore
let view_info = vk::ImageViewCreateInfo::builder()
    .image(texture_image)
    .view_type(vk::ImageViewType::TYPE_2D)
    .format(vk::Format::R8G8B8A8_SRGB)
    .subresource_range(vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlags::COLOR,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    });

let texture_view = unsafe { device.create_image_view(&view_info, None)? };

let sampler_info = vk::SamplerCreateInfo::builder()
    .mag_filter(vk::Filter::LINEAR)
    .min_filter(vk::Filter::LINEAR)
    .address_mode_u(vk::SamplerAddressMode::REPEAT)
    .address_mode_v(vk::SamplerAddressMode::REPEAT)
    .address_mode_w(vk::SamplerAddressMode::REPEAT)
    .anisotropy_enable(true)
    .max_anisotropy(16.0)
    .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
    .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
    .min_lod(0.0)
    .max_lod(0.0);

let sampler = unsafe { device.create_sampler(&sampler_info, None)? };
```

## Step 8: Bind via descriptor set

Update a descriptor set so the shader can access the combined
image/sampler pair at a binding point.

```rust,ignore
let image_descriptor = vk::DescriptorImageInfo {
    sampler,
    image_view: texture_view,
    image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
};

let write = vk::WriteDescriptorSet::builder()
    .dst_set(descriptor_set)
    .dst_binding(1) // must match the binding in the shader
    .dst_array_element(0)
    .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
    .image_info(&[image_descriptor]);

unsafe { device.update_descriptor_sets(&[write], &[]) };
```

In the fragment shader (GLSL):

```glsl
layout(set = 0, binding = 1) uniform sampler2D texSampler;

void main() {
    outColor = texture(texSampler, fragTexCoord);
}
```

> See [Descriptor Sets](../concepts/descriptors.md) for descriptor pool
> creation and layout setup.

## Cleanup

Because `vulkan_rs` handles do not implement `Drop`, you must destroy
resources manually when they are no longer needed.

```rust,ignore
// Wait for the GPU to finish using these resources first.
unsafe {
    device.device_wait_idle()?;
    device.destroy_sampler(sampler, None);
    device.destroy_image_view(texture_view, None);
    device.destroy_image(texture_image, None);
    device.free_memory(texture_memory, None);
    // Staging buffer should already be destroyed after the upload.
}
```

## Notes

- **Format choice.** `R8G8B8A8_SRGB` is correct for most color textures.
  Use `R8G8B8A8_UNORM` for data textures (normal maps, roughness) where
  sRGB gamma correction would be wrong.
- **Mipmaps.** This recipe creates a single mip level. For proper texture
  filtering at a distance, generate a full mip chain using
  `cmd_blit_image` in a loop, with a barrier between each level.
- **One-shot command buffer.** Steps 4 through 6 are typically recorded
  into a short-lived command buffer that is submitted and waited on
  immediately. Reuse command buffers from a transient pool for this.
