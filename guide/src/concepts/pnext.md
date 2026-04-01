# The pNext Extension Chain

<!-- Phase 6.3.8 -->
<!-- Independent chapter, can be read at any time -->

## Motivation

Vulkan evolves through extensions, and extensions often need to add fields
to existing structs. But Vulkan structs are `#[repr(C)]`, you cannot just
add fields. The solution is `pNext`: a linked list pointer in every
extensible struct that lets you chain additional data structures.

This is Vulkan's most powerful extensibility mechanism and one of its most
confusing features for newcomers.

## Intuition

Every Vulkan struct with a `pNext` field is an envelope. The `pNext` chain
is additional pages stuffed into the same envelope. The driver opens the
envelope, reads the main page, then checks if there are more pages. Each
extra page has a header (`sType`) that says what it is, so the driver
knows how to interpret it.

`vulkan_rs` makes this safe with typed builders:

```rust,ignore
let mut features_12 = vk::PhysicalDeviceVulkan12Features::builder();
let mut features_13 = vk::PhysicalDeviceVulkan13Features::builder();

let device_info = vk::DeviceCreateInfo::builder()
    .push_next(&mut features_12)
    .push_next(&mut features_13);
// features_13 → features_12 → DeviceCreateInfo
```

The `push_next` method only accepts types that the spec says are valid
extensions for that struct. The compiler rejects invalid chains.

<!-- TODO: Diagram, linked list of pNext structs with sType headers -->
<!-- TODO: How push_next works under the hood (marker traits from structextends) -->
<!-- TODO: Common pNext patterns (feature queries, creation info extensions) -->

> *Before reading on: what do you think happens if you chain a struct
> that the driver doesn't recognize?*

## Formal reference

<!-- TODO: BaseInStructure, BaseOutStructure -->
<!-- TODO: sType values and how they're generated -->
<!-- TODO: Lifetime safety in vulkan_rs builders -->
<!-- TODO: Links to rustdoc -->
