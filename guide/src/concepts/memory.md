# Memory Management

<!-- Phase 6.3.2 -->
<!-- Pedagogy: THRESHOLD CONCEPT. This is where Vulkan diverges most sharply
     from OpenGL. Needs strong motivation and the "why not automatic" framing.
     Use visual decision tree as the key diagram (dual coding). -->

> **Threshold concept.** Vulkan memory management permanently changes how
> you think about GPU resources. In OpenGL, the driver decided where your
> data lived. In Vulkan, *you* decide, and that decision affects
> performance more than almost anything else.

## Motivation

A GPU has multiple memory pools with different properties: some are fast
for the GPU but invisible to the CPU, some are accessible to both but
slower, some are special-purpose. Vulkan exposes this hardware reality
directly because the "right" memory choice depends on your workload,
and only you know your workload.

## Intuition

Think of GPU memory like a warehouse with different storage areas:

- **Device-local memory** is the high-speed shelving right next to the
  assembly line (GPU cores). Fast to access, but the front office (CPU)
  can't reach it directly.
- **Host-visible memory** is the loading dock, both the warehouse
  workers (GPU) and the delivery trucks (CPU) can access it, but it's
  slower for the assembly line.
- **Host-coherent memory** is a special loading dock where changes are
  immediately visible to both sides, without needing to shout "new
  stuff here!" (flush/invalidate).

The job is: figure out what goes where.

<!-- TODO: Diagram, memory type decision tree
     "Is it read by GPU every frame?" → device-local
     "Is it written by CPU every frame?" → host-visible + coherent
     "Is it uploaded once and never touched?" → staging buffer → device-local -->

> *Before reading on: if you had a mesh that never changes after upload,
> which memory type would you pick and why?*

## Worked example

<!-- TODO: Allocate memory, create a buffer, bind them together -->
<!-- TODO: Staging buffer upload pattern (the canonical Vulkan memory pattern) -->
<!-- TODO: Annotate each step with "why this choice" -->

## Formal reference

<!-- TODO: VkMemoryType, VkMemoryHeap, VkMemoryPropertyFlags -->
<!-- TODO: vkAllocateMemory, vkBindBufferMemory, vkMapMemory -->
<!-- TODO: Memory type selection algorithm -->
<!-- TODO: Links to rustdoc -->
