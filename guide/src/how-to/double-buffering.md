# Implement Double Buffering

<!-- Phase 6.4.2 -->

> **Task:** Set up frames-in-flight so the CPU records frame N+1 while
> the GPU renders frame N.

## Prerequisites

- [Synchronization](../concepts/synchronization.md) (fences, semaphores)
- [Command Buffers](../concepts/command-buffers.md)

## Steps

<!-- TODO: 1. Allocate per-frame resources (command buffer, fence, semaphores) -->
<!-- TODO: 2. The frame loop: wait for fence → acquire image → record → submit → present -->
<!-- TODO: 3. Why MAX_FRAMES_IN_FLIGHT = 2 is the common choice -->
