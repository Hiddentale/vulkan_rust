# Synchronization

<!-- Phase 6.3.3 -->
<!-- Pedagogy: THE #1 THRESHOLD CONCEPT in Vulkan. This chapter needs the
     most careful progressive formalization. Multiple worked examples with
     fading. Extra retrieval prompts. The "factory" analogy is the key
     intuition builder. -->

> **Threshold concept.** Synchronization is the single most confusing
> aspect of Vulkan for newcomers. Once you understand it, you understand
> Vulkan's execution model. If this chapter takes you three reads, that
> is normal.

## Motivation

The GPU does not execute your commands in the order you recorded them.
Not between queues, not between submissions, and not even between draw
calls within the same command buffer (the GPU pipelines them). Vulkan
gives you **zero implicit ordering guarantees**.

This is terrifying, and it is also why Vulkan is fast. The GPU can
overlap operations, reorder for efficiency, and saturate its hardware.
But it means *you* must tell the driver when ordering matters.

## Intuition: the factory

Imagine a factory with multiple assembly lines (queue families). Each
line has workers (pipeline stages) who process items in sequence.

- A **fence** is a sign on the factory door: "the factory is done with
  everything you asked for." You (the CPU) check the sign to know when
  GPU work has finished.
- A **semaphore** is a conveyor belt between two assembly lines: "line B
  should not start until line A puts something on the belt." Used for
  queue-to-queue synchronization.
- A **pipeline barrier** is a supervisor on a single line: "workers at
  stage X must finish before workers at stage Y start on the same items."
  Used for within-command-buffer ordering.
- An **event** is a sticky note: "I'll leave this here, check for it later."
  Split barriers for fine-grained control.

<!-- TODO: Timeline diagram showing overlapping GPU work without
     synchronization, then the same work with correct sync -->
<!-- TODO: Table, which primitive for which use case -->

> *Before reading on: you submit two command buffers to the same queue,
> one after the other. Does the second one wait for the first to finish?*

## Worked example 1: CPU waits for GPU (Fence)

<!-- TODO: Complete worked example with all annotations -->

## Worked example 2: Queue-to-queue sync (Semaphore)

<!-- TODO: Swapchain acquire → render → present pattern -->
<!-- TODO: This is the canonical semaphore use case -->

## Worked example 3: Image layout transition (Pipeline Barrier)

<!-- TODO: Faded example, more steps omitted than example 1 -->

## Formal reference

<!-- TODO: Execution dependencies, pipeline stages, access masks -->
<!-- TODO: VkFence, VkSemaphore, VkEvent, VkPipelineBarrier -->
<!-- TODO: The "happens-before" relationship -->
<!-- TODO: Links to rustdoc -->

## Common mistakes

<!-- TODO: "Debugging synchronization", common validation layer errors
     and what they mean -->
