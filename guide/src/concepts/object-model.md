# The Vulkan Object Model

<!-- Phase 6.3.1 -->
<!-- Pedagogy: this is the foundation, every other concept depends on it.
     Keep it concrete. Use the "what if this didn't exist" framing to
     motivate each design choice. -->

## Motivation

Every Vulkan API call operates on *handles*, opaque references to objects
that live on the GPU or in the driver. Before you can do anything useful
in Vulkan, you need to understand what these handles are, how they relate
to each other, and who is responsible for destroying them.

## Intuition

Think of Vulkan handles like file descriptors in Unix. You open them,
you use them, you close them. The OS (driver) manages the underlying
resource, you manage the lifetime of your reference to it.

<!-- TODO: Diagram, handle hierarchy: Instance → PhysicalDevice → Device → everything else -->
<!-- TODO: Dispatchable vs non-dispatchable handles, and why you should care -->
<!-- TODO: Parent-child relationships and destruction order -->

## Worked example

<!-- TODO: Creating, using, and destroying a Buffer, the simplest
     complete lifecycle -->
<!-- TODO: Sub-goal labels: "Create", "Use", "Destroy" -->

## Formal reference

<!-- TODO: Handle types in vulkan_rs (repr(transparent), Handle trait) -->
<!-- TODO: Links to vk::Instance, vk::Device, vk::Buffer rustdoc -->
<!-- TODO: Spec rules on object lifetime and valid usage -->

> *Before reading on: what do you think happens if you destroy a Device
> while one of its Buffers is still in use by the GPU?*
