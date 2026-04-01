# Port from ash to vulkan_rs

<!-- Phase 6.4.5 -->

> **Task:** Migrate an existing `ash`-based project to `vulkan_rs`.

## Key differences

| Aspect | ash | vulkan_rs |
|--------|-----|-----------|
| Command style | Trait methods (`DeviceV1_0`, `KhrSwapchainExtension`) | Inherent methods on `Device` |
| Imports | One trait per API version + extension | None, just `use vk_engine::*` |
| Raw handle access | `.handle()` or `*device` | `.handle()` |
| Interop | Limited `from_raw` | `from_raw_parts(handle, get_proc_addr)` |
| Builders | `Builder<'a>` with `build()` | `Builder<'a>` with `Deref` |
| Error type | `vk::Result` split into error/success codes | `vk::Result` directly |

## Migration steps

<!-- TODO: 1. Replace Cargo dependency -->
<!-- TODO: 2. Remove trait imports -->
<!-- TODO: 3. Replace Entry/Instance/Device creation -->
<!-- TODO: 4. Replace builder .build() with Deref -->
<!-- TODO: 5. Update error handling -->
<!-- TODO: Side-by-side code comparisons for each step -->
