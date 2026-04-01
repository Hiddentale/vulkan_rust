# Handle Window Resize

> **Task:** Detect window resize events and recreate the swapchain
> without crashing or leaking resources.

## Prerequisites

- [Hello Triangle, Part 2](../getting-started/hello-triangle-2.md) (swapchain creation)
- [Synchronization](../concepts/synchronization.md) (device idle)
- [Implement Double Buffering](double-buffering.md) (frames in flight)

## The problem

When the window is resized, the swapchain images no longer match the
window dimensions. Vulkan tells you this has happened through two
mechanisms:

1. `acquire_next_image` or `queue_present` returns `ERROR_OUT_OF_DATE_KHR`,
   meaning the swapchain is no longer compatible with the surface.
2. `queue_present` returns `SUBOPTIMAL_KHR`, meaning the swapchain still
   works but no longer matches the surface properties perfectly.

In either case, you must recreate the swapchain (and everything that
depends on its images) before rendering can continue.

## Step 1: Detect the resize

Track resize events from your windowing library and from Vulkan return
codes.

```rust,ignore
let mut framebuffer_resized = false;

// In your window event handler (winit example):
match event {
    WindowEvent::Resized(_) => {
        framebuffer_resized = true;
    }
    _ => {}
}
```

In the render loop, check both the flag and the Vulkan result codes:

```rust,ignore
let acquire_result = unsafe {
    device.acquire_next_image(
        swapchain, u64::MAX, image_available_semaphore, vk::Fence::null(),
    )
};

let (image_index, _suboptimal) = match acquire_result {
    Ok(result) => result,
    Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
        recreate_swapchain(/* ... */)?;
        continue; // restart this loop iteration
    }
    Err(e) => return Err(e.into()),
};

// ... record and submit ...

let present_result = unsafe { device.queue_present(present_queue, &present_info) };

match present_result {
    Ok(_) => {}
    Err(vk::Result::ERROR_OUT_OF_DATE_KHR | vk::Result::SUBOPTIMAL_KHR) => {
        framebuffer_resized = false;
        recreate_swapchain(/* ... */)?;
    }
    Err(e) => return Err(e.into()),
}

// Also check the manual flag (some platforms don't always return OUT_OF_DATE).
if framebuffer_resized {
    framebuffer_resized = false;
    recreate_swapchain(/* ... */)?;
}
```

> *Before reading on: why do we check `framebuffer_resized` separately
> from the Vulkan error codes? Why not rely on `OUT_OF_DATE_KHR` alone?*

Some window systems (notably X11) do not always report out-of-date when
the window is resized. The manual flag from the window event handler
catches those cases.

## Step 2: Wait for the GPU

Before destroying any swapchain-related resources, all in-flight work
must finish.

```rust,ignore
unsafe { device.device_wait_idle()?; }
```

This is simple and correct. For higher performance you could track
individual fences per swapchain image, but `device_wait_idle` is the
right choice for a resize path that runs infrequently.

## Step 3: Destroy old resources

Destroy everything that depends on the swapchain images, in reverse
creation order.

```rust,ignore
// Destroy framebuffers (one per swapchain image).
for &fb in &swapchain_framebuffers {
    unsafe { device.destroy_framebuffer(fb, None); }
}

// Destroy image views (one per swapchain image).
for &view in &swapchain_image_views {
    unsafe { device.destroy_image_view(view, None); }
}

// Do NOT destroy the old swapchain yet, we pass it to the new one.
```

You do not need to destroy the swapchain images themselves. They are
owned by the swapchain and will be cleaned up when the old swapchain is
destroyed.

## Step 4: Query new surface capabilities

The surface extent may have changed, so re-query it.

```rust,ignore
let surface_caps = unsafe {
    instance.get_physical_device_surface_capabilities(physical_device, surface)?
};

let new_extent = if surface_caps.current_extent.width != u32::MAX {
    // The surface has a defined size, use it.
    surface_caps.current_extent
} else {
    // The surface size is undefined (e.g. Wayland), clamp to limits.
    let window_size = window.inner_size();
    vk::Extent2D {
        width: window_size.width.clamp(
            surface_caps.min_image_extent.width,
            surface_caps.max_image_extent.width,
        ),
        height: window_size.height.clamp(
            surface_caps.min_image_extent.height,
            surface_caps.max_image_extent.height,
        ),
    }
};
```

## Step 5: Handle minimized windows

When a window is minimized, the surface extent can be `(0, 0)`. You
cannot create a swapchain with zero dimensions. Pause the render loop
until the window is restored.

```rust,ignore
if new_extent.width == 0 || new_extent.height == 0 {
    // Window is minimized. Wait for a resize event before continuing.
    // With winit, use Event::MainEventsCleared to avoid busy-waiting.
    return Ok(());
}
```

## Step 6: Create the new swapchain

Pass the old swapchain handle to `old_swapchain`. This lets the driver
reuse internal resources and can make the transition smoother.

```rust,ignore
let old_swapchain = swapchain; // save the handle

let swapchain_info = vk::SwapchainCreateInfoKHR::builder()
    .surface(surface)
    .min_image_count(desired_image_count)
    .image_format(surface_format.format)
    .image_color_space(surface_format.color_space)
    .image_extent(new_extent)
    .image_array_layers(1)
    .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
    .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
    .pre_transform(surface_caps.current_transform)
    .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
    .present_mode(present_mode)
    .clipped(true)
    .old_swapchain(old_swapchain); // <-- reuse hint

swapchain = unsafe { device.create_swapchain(&swapchain_info, None)? };

// Now destroy the old swapchain.
unsafe { device.destroy_swapchain(old_swapchain, None); }
```

## Step 7: Recreate image views and framebuffers

The new swapchain has new images, so create fresh image views and
framebuffers.

```rust,ignore
let swapchain_images = unsafe { device.get_swapchain_images(swapchain)? };

swapchain_image_views = swapchain_images
    .iter()
    .map(|&image| {
        let view_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(surface_format.format)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        unsafe { device.create_image_view(&view_info, None) }
    })
    .collect::<Result<Vec<_>, _>>()?;

swapchain_framebuffers = swapchain_image_views
    .iter()
    .map(|&view| {
        let attachments = [view];
        let fb_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(new_extent.width)
            .height(new_extent.height)
            .layers(1);
        unsafe { device.create_framebuffer(&fb_info, None) }
    })
    .collect::<Result<Vec<_>, _>>()?;
```

## Putting it all together

A helper function that bundles the recreation logic:

```rust,ignore
fn recreate_swapchain(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    window: &Window,
    render_pass: vk::RenderPass,
    swapchain: &mut vk::SwapchainKHR,
    swapchain_image_views: &mut Vec<vk::ImageView>,
    swapchain_framebuffers: &mut Vec<vk::Framebuffer>,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
) -> Result<vk::Extent2D, Box<dyn std::error::Error>> {
    unsafe { device.device_wait_idle()?; }

    // Destroy old framebuffers and image views.
    for &fb in swapchain_framebuffers.iter() {
        unsafe { device.destroy_framebuffer(fb, None); }
    }
    for &view in swapchain_image_views.iter() {
        unsafe { device.destroy_image_view(view, None); }
    }

    // Query new extent, create new swapchain, views, framebuffers.
    // ... (Steps 4 through 7 from above) ...

    Ok(new_extent)
}
```

## Common mistakes

**Forgetting to update the viewport and scissor.** If you use dynamic
viewport/scissor state (which you should), update them to the new extent
each frame. If you baked them into the pipeline, you need to recreate
the pipeline too.

**Leaking old image views.** Every `create_image_view` must have a
matching `destroy_image_view`. If you overwrite the `Vec` without
destroying the old views first, those handles leak.

**Not handling SUBOPTIMAL.** `SUBOPTIMAL_KHR` from `queue_present` is
not a fatal error, but ignoring it means you render at the wrong
resolution until something else triggers an `OUT_OF_DATE_KHR`.

## Notes

- **Depth buffers.** If your render pass uses a depth attachment, you
  must also recreate the depth image, its memory, and its image view
  when the swapchain extent changes.
- **Render pass compatibility.** The render pass itself does not depend
  on the swapchain extent, only on the image format. You do not need to
  recreate it unless the surface format changes (which is extremely rare).
- **Dynamic state.** Using `vk::DynamicState::VIEWPORT` and
  `vk::DynamicState::SCISSOR` avoids having to recreate the pipeline on
  resize. This is the recommended approach.
