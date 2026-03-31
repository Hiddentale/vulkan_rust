Potential Bugs / Areas to Verify Before Documentation
Here are the areas I'd recommend auditing first:

1. Two-call enumeration pattern correctness
The generated wrappers use enumerate_two_call() and fill_two_call() for commands that return arrays. These are subtle — the first call gets the count, the second fills. If any generated wrapper incorrectly classifies a command as two-call vs single-output, you get runtime UB or panics. Worth spot-checking a few commands (e.g., enumerate_physical_devices, get_swapchain_images_khr).

2. 64-bit bitmask types
Some Vulkan flags are 64-bit (VkPipelineStageFlags2, VkAccessFlags2). The generator handles both, but these are rarer and less tested than the 32-bit path. A layout test for a 64-bit bitmask struct would be cheap insurance.

3. Union types
Vulkan has unions (VkClearColorValue, VkClearValue, VkPerformanceValueDataINTEL). These need #[repr(C)] and correct field sizes. If any union member is a struct with padding, the layout could silently diverge from C.

4. Platform #[cfg] gate coverage
Platform-specific types (Win32 handles like HWND, HINSTANCE, Wayland wl_display, X11 Display, etc.) are behind cfg(target_os) gates. If any struct references a platform type without matching its gate, you get a cross-compilation failure that CI might not catch (CI only builds on each platform, not for every target).

5. pNext chain safety
The push_next builder method chains structs via raw pointers. If a user calls .build() and the push_next'd struct goes out of scope, the pNext pointer dangles. This is documented-unsafe, but worth verifying the lifetime 'a on builders actually prevents this at compile time.

6. Aliased types / promoted extensions
When extensions get promoted to core (e.g., VK_KHR_get_physical_device_properties2 → Vulkan 1.1), the types get aliases. If the generator doesn't resolve these consistently, you could end up with duplicate types or broken references.

7. Command dispatch level classification
Commands are classified as entry-level, instance-level, or device-level based on their first parameter. If the generator misclassifies a command (e.g., puts a device command into InstanceCommands), it loads via the wrong proc addr and may silently return null.


Current tests cover the structural correctness (layouts, handle round-trips, determinism, compilation) — this is good.

What's missing is runtime behavioral testing (actually calling Vulkan through the generated wrappers on a real driver). The #[ignore] integration tests exist but only cover a handful of commands.

The realistic bar is: "confident enough that users hitting bugs is rare and fixable." ash has been around for years and still gets bug reports. The key is that the generator is the single source of truth — fixing a class of bugs there fixes all affected commands at once.

