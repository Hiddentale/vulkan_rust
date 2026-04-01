# Error Handling Philosophy

<!-- Phase 6.5.2 -->

## Vulkan's error model

Vulkan functions return `VkResult`, an integer that is either `VK_SUCCESS`
(or another success code like `VK_INCOMPLETE`) or a negative error code.
There is no exception system, no errno, no callbacks.

## How vulkan_rs handles errors

`vk-engine` wrapper methods return `Result<T, vk::Result>`:

```rust,ignore
// The wrapper checks the return code for you.
let buffer = unsafe { device.create_buffer(&info, None) }?;

// If you need to handle specific errors:
match unsafe { device.create_buffer(&info, None) } {
    Ok(buffer) => { /* use buffer */ }
    Err(vk::Result::ERROR_OUT_OF_DEVICE_MEMORY) => { /* handle OOM */ }
    Err(e) => panic!("Unexpected error: {:?}", e),
}
```

## When vulkan_rs panics

Panics are reserved for programming errors, not runtime failures:

- Calling an extension method that was not enabled at device creation
  (the function pointer stub panics with a descriptive message).
- Internal invariant violations (should never happen).

Vulkan runtime errors (out of memory, device lost, surface lost) are
always returned as `Err`, never panicked.

<!-- TODO: VulkanError enum and when it's used vs raw vk::Result -->
