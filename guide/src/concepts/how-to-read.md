# How to Read This Section

This section explains *how Vulkan works*, not as a tutorial to follow, but
as a set of mental models you can carry with you while writing any Vulkan code.

## Structure of each chapter

Every concept chapter follows the same four-part structure:

| Part | Purpose | How to use it |
|------|---------|---------------|
| **Motivation** | Why this concept exists | Read first, it tells you what problem you're solving |
| **Intuition** | Analogy, diagram, or informal explanation | Build a mental picture *before* touching code |
| **Worked example** | Annotated code showing the concept in practice | Read the annotations, not just the code |
| **Formal reference** | Spec terminology, edge cases, API links | Come back to this when you need precision |

You do not need to memorize the formal reference on first reading. The
intuition and worked example are enough to start writing code. The formal
section is there for when your intuition hits an edge case and you need
to know exactly what the spec says.

## Threshold concepts

Some ideas in Vulkan are **threshold concepts**, once they click, they
permanently change how you understand the API. These are flagged with a
marker:

> **Threshold concept.** This idea transforms how you think about Vulkan.
> If it feels confusing, that is normal, it means your mental model is
> being restructured. Stay with it.

The three biggest threshold concepts in Vulkan are:

1. **Explicit memory management**, you allocate GPU memory yourself and
   decide what goes where.
2. **Synchronization is your responsibility**, the GPU runs asynchronously
   and Vulkan gives you no implicit ordering guarantees.
3. **State is baked into pipeline objects**, you cannot change rendering
   state on the fly like in OpenGL.

## Reading order

The chapters are ordered by dependency, each builds on the ones before it.
If a concept doesn't make sense, check the [dependency map](../README.md)
to see which prerequisite you might need to revisit.

Two chapters are independent and can be read at any time:
- [The pNext Extension Chain](pnext.md)
- [Validation Layers & Debugging](validation.md)

## Active reading

Throughout each chapter, you will find questions like:

> *Before reading on: why do you think Vulkan requires explicit
> synchronization instead of handling it automatically?*

These are **retrieval prompts**. Pausing to answer, even briefly, even
wrong, significantly improves retention. You are not expected to know the
answer. The act of thinking about it before reading the explanation is
what matters.
