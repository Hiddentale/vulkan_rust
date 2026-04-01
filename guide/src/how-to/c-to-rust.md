# Map C Vulkan Calls to vulkan_rs

<!-- Phase 6.4.6 -->

> **Task:** You have C Vulkan code (or you're reading the spec) and want
> to find the equivalent `vulkan_rs` API.

## Naming convention

| C Vulkan | vulkan_rs |
|----------|-----------|
| `vkCreateBuffer` | `device.create_buffer(...)` |
| `VkBufferCreateInfo` | `vk::BufferCreateInfo` |
| `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` | `vk::BufferUsageFlags::VERTEX_BUFFER` |
| `VK_FORMAT_R8G8B8A8_SRGB` | `vk::Format::R8G8B8A8_SRGB` |
| `VK_SUCCESS` | `vk::Result::SUCCESS` |
| `VK_KHR_SWAPCHAIN_EXTENSION_NAME` | `vk::KHR_SWAPCHAIN_EXTENSION_NAME` |

## Rules

1. **Types:** Strip the `Vk` prefix → `VkBuffer` becomes `vk::Buffer`.
2. **Functions:** Strip `vk`, convert to `snake_case`, call as a method
   on the parent object → `vkCreateBuffer(device, ...)` becomes
   `device.create_buffer(...)`.
3. **Enum variants:** Strip the type prefix and `_BIT` suffix, keep
   `SCREAMING_CASE` → `VK_FORMAT_R8G8B8A8_SRGB` becomes
   `vk::Format::R8G8B8A8_SRGB`.
4. **Bitmask bits:** Same as enums, strip `_BIT` →
   `VK_BUFFER_USAGE_VERTEX_BUFFER_BIT` becomes
   `vk::BufferUsageFlags::VERTEX_BUFFER`.

## Search tip

All `vulkan_rs` types have `#[doc(alias = "VkOriginalName")]` attributes.
Searching for the C name in rustdoc will find the Rust equivalent.

<!-- TODO: Expanded reference table for the most common API calls -->
