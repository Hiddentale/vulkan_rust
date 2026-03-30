use std::ffi::{c_void, CStr};

/// Abstraction over loading symbols from the Vulkan shared library.
///
/// # Safety
///
/// Implementations must return valid function pointers for the requested
/// symbol name, or null if the symbol is not found. Returning a pointer
/// to the wrong function causes undefined behavior.
pub unsafe trait Loader: Send + Sync {
    /// Load a function by name from the Vulkan library.
    ///
    /// Returns a raw function pointer, or null if the symbol is not found.
    unsafe fn load(&self, name: &CStr) -> *const c_void;
}
