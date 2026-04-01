# Design Decisions & Safety Model

<!-- Phase 6.5.1 -->

## Why two crates?

`vulkan_rs` is split into `vk-sys` (generated FFI types) and `vk-engine`
(ergonomic wrappers) for a clear reason: generated code and hand-written
code have different quality bars, review processes, and change frequencies.

- **`vk-sys`** is regenerated from `vk.xml` whenever a new Vulkan version
  is released. It contains ~50,000 lines of `#[repr(C)]` types, enums,
  bitmasks, and function pointer typedefs. It is `#![no_std]`.
- **`vk-engine`** is hand-written (with some generated wrapper methods).
  It provides `Entry`, `Instance`, `Device`, builders, and the loading
  machinery.

Users depend on `vk-engine` and access raw types via `vk_engine::vk::*`.

## Why `#[repr(C)]`?

Vulkan structs must have the exact same memory layout as their C
counterparts. `#[repr(C)]` guarantees field ordering and alignment
match what the Vulkan driver expects. We verify this in CI by
cross-validating struct layouts between our Rust types and C headers
compiled from the same `vk.xml` version.

## Why builders use `Deref`?

<!-- TODO: The Deref<Target=Inner> pattern, justification and tradeoffs -->

## Why `unsafe` on everything?

<!-- TODO: Safety model, what the caller must guarantee -->
<!-- TODO: Why we don't add RAII / Drop, explicit is better for Vulkan -->

## Why inherent methods instead of traits?

<!-- TODO: The trait import problem and why one Device type is better -->
