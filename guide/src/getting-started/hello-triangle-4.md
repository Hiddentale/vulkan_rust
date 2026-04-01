# Hello Triangle, Part 4: Command Buffers & Drawing

<!-- Phase 6.2.5 -->
<!-- Pedagogy: this ties all previous parts together, the "integrative"
     moment. Use worked example with sub-goal labels. -->

> **What you will build:** The final piece, recording draw commands,
> submitting them to the GPU, and presenting the result. A triangle
> on screen.

## The render loop

<!-- TODO: Motivation, why record commands into buffers instead of
     calling the GPU directly? (Batching, reuse, multi-threading) -->
<!-- TODO: Intuition, command buffer as a shopping list you hand to
     someone else to execute -->
<!-- TODO: Sub-goal labeled code walkthrough:
     1. "Acquire the next swapchain image"
     2. "Begin the command buffer and render pass"
     3. "Bind the pipeline and draw"
     4. "End the render pass and submit"
     5. "Present the image" -->
<!-- TODO: Full working example combining all four parts -->
<!-- TODO: "What's next" section pointing to Concepts -->
