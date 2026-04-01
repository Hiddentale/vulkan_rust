# Hello Triangle, Part 1: Instance & Device

<!-- Phase 6.2.2 -->
<!-- Pedagogy: progressive formalization, intuition, then practical, then code -->

> **What you will build:** By the end of this page, you will have working code
> that loads Vulkan, creates an Instance, picks a physical GPU, and creates a
> logical Device. This is the foundation every Vulkan program needs.

## Why so much setup?

OpenGL gave you a context and you could start drawing. Vulkan does not.

Vulkan is explicit: you tell the driver exactly which GPU to use, which
queue families you need, which extensions to enable, and which features
to turn on. This is more work up front, but it means no hidden state,
no driver heuristics guessing what you meant, and no surprises in
production.

Think of it this way: OpenGL is a taxi, you say where to go and
the driver picks the route. Vulkan is a self-driving car you program
yourself, more control, more responsibility.

## The initialization sequence

Every Vulkan program follows the same four steps:

```text
1. Entry       , Load the Vulkan library from the OS
2. Instance    , Create a Vulkan instance (your connection to the driver)
3. PhysicalDevice, Pick which GPU to use
4. Device      , Create a logical device (your interface to that GPU)
```

Let's walk through each one.

<!-- TODO: Annotated code for each step -->
<!-- TODO: Full working example at the end -->
<!-- TODO: "What could go wrong" section (common errors) -->
<!-- TODO: Retrieval prompt: "Before reading on, can you explain why
     Vulkan separates Instance from Device?" -->
