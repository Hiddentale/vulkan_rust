# Pipelines

<!-- Phase 6.3.5 -->
<!-- Pedagogy: THRESHOLD CONCEPT. "State is baked" is a fundamental shift
     from OpenGL's mutable state machine. -->

> **Threshold concept.** In OpenGL, you set rendering state one call at a
> time, blend mode here, depth test there, and the driver compiles the
> final state lazily. In Vulkan, *all* state is compiled into a pipeline
> object up front. This removes driver guesswork and hitching at the cost
> of more explicit setup.

## Motivation

A GPU is not a general-purpose processor. It is a configurable state
machine with fixed-function stages (vertex input, rasterization, blending)
and programmable stages (vertex shader, fragment shader). A pipeline object
captures the *full configuration* of this machine, every stage, every
setting, so the driver can compile it to hardware instructions once and
reuse it many times.

## Intuition

A pipeline is like a preset on a mixing console. Instead of adjusting
every knob during the live performance (and risking a pop or crackle),
you save the full board state as a preset and recall it instantly. You
can have many presets and switch between them, but you cannot twiddle
individual knobs mid-song.

(Vulkan 1.3 added *dynamic state* to relax this, certain knobs *can*
be adjusted at draw time. But the core idea holds: most state is baked.)

<!-- TODO: Diagram, graphics pipeline stages from vertex input to framebuffer -->
<!-- TODO: Graphics vs compute pipelines -->
<!-- TODO: Pipeline layout, the bridge to descriptor sets -->
<!-- TODO: Pipeline cache, avoid recompilation across runs -->

> *Before reading on: if you need to render some objects with blending
> and some without, how many pipeline objects do you need?*

## Worked example

<!-- TODO: Create a simple graphics pipeline -->
<!-- TODO: Annotate each pipeline stage configuration -->

## Formal reference

<!-- TODO: VkPipeline, VkPipelineLayout, VkPipelineCache -->
<!-- TODO: Graphics, compute, ray tracing pipeline types -->
<!-- TODO: Dynamic state in Vulkan 1.3 -->
<!-- TODO: Links to rustdoc -->
