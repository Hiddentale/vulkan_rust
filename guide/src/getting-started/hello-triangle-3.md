# Hello Triangle, Part 3: Render Pass & Pipeline

<!-- Phase 6.2.4 -->
<!-- Pedagogy: threshold concept, the pipeline is a major conceptual shift
     from immediate-mode rendering. Needs extra motivation and intuition. -->

> **What you will build:** A render pass that describes your rendering
> operations, and a graphics pipeline that compiles your shaders and
> fixed-function state into a GPU-executable program.

## Why does Vulkan need a pipeline object?

<!-- TODO: Motivation, GPU hardware is a configurable state machine,
     not a general-purpose processor -->
<!-- TODO: Intuition, factory assembly line analogy: you design the
     line (pipeline) before you run products (draw calls) through it -->
<!-- TODO: Diagram, pipeline stages from vertex input to color output -->
<!-- TODO: Code walkthrough, render pass, shader modules, pipeline -->
<!-- TODO: Warning box: common pipeline creation mistakes -->
