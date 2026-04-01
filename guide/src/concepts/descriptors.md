# Descriptor Sets & Resource Binding

<!-- Phase 6.3.6 -->

## Motivation

Shaders need access to resources: buffers containing vertex data, images
to sample, uniform buffers with transformation matrices. Descriptors are
Vulkan's mechanism for connecting shader bindings (`layout(binding = 0)`)
to actual GPU resources.

The descriptor system is more complex than OpenGL's `glBindTexture`, but
it exists because binding resources one at a time is a performance
bottleneck. Vulkan lets you bind *sets* of resources at once, and reuse
those sets across multiple draw calls.

## Intuition

Think of a descriptor set as a tray of tools laid out for a surgeon.
The descriptor set *layout* is the diagram showing which tool goes in
which slot. The descriptor *pool* is the sterilization room where trays
are prepared. The actual descriptor *set* is a prepared tray, ready to
be slid under the surgeon's hands at the right moment.

You define the layout once (what slots exist), allocate trays from the
pool, fill each tray with specific tools (write descriptors), and then
bind the tray during command recording.

<!-- TODO: Diagram, layout → pool → set → binding flow -->
<!-- TODO: Descriptor types (uniform buffer, sampled image, storage buffer) -->
<!-- TODO: Push descriptors as the simpler alternative for small bindings -->

> *Before reading on: why do you think Vulkan uses descriptor "pools"
> instead of allocating descriptors individually?*

## Worked example

<!-- TODO: Create layout, pool, allocate set, write, bind -->
<!-- TODO: Sub-goal labels for each step -->

## Formal reference

<!-- TODO: VkDescriptorSetLayout, VkDescriptorPool, VkDescriptorSet -->
<!-- TODO: vkUpdateDescriptorSets, vkCmdBindDescriptorSets -->
<!-- TODO: Push descriptors (VK_KHR_push_descriptor) -->
<!-- TODO: Links to rustdoc -->
