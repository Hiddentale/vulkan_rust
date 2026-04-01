# Render Passes & Framebuffers

<!-- Phase 6.3.4 -->

## Motivation

A render pass tells Vulkan the *structure* of your rendering: what
attachments you use (color, depth), how they are loaded and stored, and
how subpasses depend on each other. This information lets the driver
make hardware-specific optimizations, especially on tile-based GPUs
(mobile) where the render pass boundaries determine what fits in on-chip
memory.

## Intuition

A render pass is a *blueprint* for a painting session. It says: "I will
paint on this size canvas, using these colors of paint, and I will do it
in these stages." The actual painting happens later, inside a command
buffer, using this blueprint.

A framebuffer is the *specific canvas*, the actual images that match the
blueprint's description.

<!-- TODO: Diagram, render pass structure with subpasses and dependencies -->
<!-- TODO: Attachments: load/store ops and why they matter -->
<!-- TODO: Dynamic rendering (VK_KHR_dynamic_rendering) as the modern alternative -->

> *Before reading on: why might a mobile GPU care about the difference
> between LOAD_OP_LOAD and LOAD_OP_CLEAR?*

## Worked example

<!-- TODO: Create render pass with one color attachment -->
<!-- TODO: Create framebuffer from swapchain image views -->

## Formal reference

<!-- TODO: VkRenderPass, VkFramebuffer, VkAttachmentDescription -->
<!-- TODO: Subpass dependencies -->
<!-- TODO: Links to rustdoc -->
