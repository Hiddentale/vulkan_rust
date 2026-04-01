# Load and Sample Textures

<!-- Phase 6.4.1 -->
<!-- Recipe format: problem statement → prerequisites → steps → full code -->

> **Task:** Load an image from disk, upload it to GPU memory, and sample
> it in a fragment shader.

## Prerequisites

You should be comfortable with:
- [Memory Management](../concepts/memory.md) (staging buffers)
- [Command Buffers](../concepts/command-buffers.md) (one-shot transfers)
- [Descriptor Sets](../concepts/descriptors.md) (binding samplers)
- [Synchronization](../concepts/synchronization.md) (image layout transitions)

## Steps

<!-- TODO: 1. Load image pixels (use the `image` crate) -->
<!-- TODO: 2. Create staging buffer, copy pixels in -->
<!-- TODO: 3. Create VkImage with optimal tiling -->
<!-- TODO: 4. Transition image to TRANSFER_DST layout -->
<!-- TODO: 5. Copy staging buffer → image -->
<!-- TODO: 6. Transition image to SHADER_READ_ONLY layout -->
<!-- TODO: 7. Create image view and sampler -->
<!-- TODO: 8. Write descriptor set with combined image sampler -->
