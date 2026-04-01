# Command Buffers

<!-- Phase 6.3.7 -->
<!-- Pedagogy: placed before Synchronization because sync doesn't make sense
     until you understand that commands are recorded, not executed immediately.
     This ordering follows the dependency graph. -->

## Motivation

In OpenGL, calling `glDrawArrays` immediately sends work to the GPU. In
Vulkan, you *record* commands into a buffer, then *submit* that buffer to
a queue. The GPU processes the queue asynchronously.

This separation exists for three reasons: batching (one submission of many
commands is cheaper than many individual calls), reuse (record once, submit
many times), and multi-threading (different threads can record into
different command buffers simultaneously).

## Intuition

A command buffer is a shopping list. You write down everything you need
("bind this pipeline", "draw 36 vertices", "copy this image"), then hand
the list to someone else (the GPU queue) who goes and does it all. You
don't stand in the store waiting for each item, you hand off the list
and do other work.

<!-- TODO: Diagram, recording vs submission timeline -->
<!-- TODO: Command pools, why they exist (memory allocation amortization) -->
<!-- TODO: Primary vs secondary command buffers -->

> *Before reading on: if command buffers can be recorded on different
> threads, what happens when two threads try to use the same command pool?*

## Worked example

<!-- TODO: Create pool, allocate buffer, record commands, submit -->
<!-- TODO: Sub-goal labels for each phase -->

## Formal reference

<!-- TODO: VkCommandPool, VkCommandBuffer, VkCommandBufferLevel -->
<!-- TODO: vkAllocateCommandBuffers, vkBeginCommandBuffer, vkEndCommandBuffer -->
<!-- TODO: vkQueueSubmit, submission ordering guarantees (or lack thereof) -->
<!-- TODO: Links to rustdoc -->
