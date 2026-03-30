use std::sync::Arc;

use crate::error::LoadError;
use crate::loader::Loader;
use crate::vk;
use vk::handles::Handle;

/// Entry point into the Vulkan API.
///
/// Loads the Vulkan shared library, resolves the bootstrap function pointers,
/// and provides access to entry-level commands (instance creation, version
/// query, layer/extension enumeration).
///
/// The `Entry` keeps the shared library alive via `Arc<dyn Loader>` for the
/// lifetime of all derived objects.
pub struct Entry {
    _loader: Arc<dyn Loader>,
    get_instance_proc_addr: vk::commands::PFN_vkGetInstanceProcAddr,
    get_device_proc_addr: vk::commands::PFN_vkGetDeviceProcAddr,
    commands: vk::commands::EntryCommands,
}

impl Entry {
    /// Create a new `Entry` from the given loader.
    ///
    /// Resolves `vkGetInstanceProcAddr` from the library, then uses it to
    /// bootstrap `vkGetDeviceProcAddr` and all entry-level commands.
    ///
    /// # Safety
    ///
    /// The loader must return valid Vulkan function pointers. The loaded
    /// shared library must remain valid for the lifetime of this `Entry`
    /// and any objects created from it.
    pub unsafe fn new(loader: impl Loader + 'static) -> Result<Self, LoadError> {
        let loader: Arc<dyn Loader> = Arc::new(loader);

        let get_instance_proc_addr: vk::commands::PFN_vkGetInstanceProcAddr = unsafe {
            let ptr = loader.load(c"vkGetInstanceProcAddr");
            if ptr.is_null() {
                return Err(LoadError::MissingEntryPoint);
            }
            std::mem::transmute(ptr)
        };

        let get_instance_proc_addr_fn = get_instance_proc_addr.unwrap();
        let null_instance = vk::handles::Instance::null();

        let get_device_proc_addr: vk::commands::PFN_vkGetDeviceProcAddr = unsafe {
            std::mem::transmute(loader.load(c"vkGetDeviceProcAddr"))
        };

        let commands = unsafe {
            vk::commands::EntryCommands::load(|name| {
                std::mem::transmute(get_instance_proc_addr_fn(null_instance, name.as_ptr()))
            })
        };

        Ok(Self {
            _loader: loader,
            get_instance_proc_addr,
            get_device_proc_addr,
            commands,
        })
    }

    /// Returns the raw `vkGetInstanceProcAddr` function pointer.
    ///
    /// Needed by OpenXR's `XR_KHR_vulkan_enable2` which requires the
    /// application to provide this function pointer.
    pub fn get_instance_proc_addr(&self) -> vk::commands::PFN_vkGetInstanceProcAddr {
        self.get_instance_proc_addr
    }

    /// Returns the raw `vkGetDeviceProcAddr` function pointer.
    pub fn get_device_proc_addr(&self) -> vk::commands::PFN_vkGetDeviceProcAddr {
        self.get_device_proc_addr
    }

    /// Returns a reference to the loaded entry-level commands.
    pub(crate) fn commands(&self) -> &vk::commands::EntryCommands {
        &self.commands
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_void, CStr};

    /// Mock loader that returns null for everything — simulates missing library.
    struct NullLoader;

    unsafe impl Loader for NullLoader {
        unsafe fn load(&self, _name: &CStr) -> *const c_void {
            std::ptr::null()
        }
    }

    #[test]
    fn new_returns_missing_entry_point_when_loader_returns_null() {
        let result = unsafe { Entry::new(NullLoader) };
        match result {
            Err(LoadError::MissingEntryPoint) => {}
            Err(other) => panic!("expected MissingEntryPoint, got {other}"),
            Ok(_) => panic!("expected error, got Ok"),
        }
    }

    #[test]
    #[ignore] // requires Vulkan runtime
    fn new_succeeds_with_real_loader() {
        let loader =
            crate::loader::LibloadingLoader::new().expect("failed to load Vulkan library");
        let entry = unsafe { Entry::new(loader) }.expect("failed to create Entry");
        assert!(entry.get_instance_proc_addr().is_some());
        assert!(entry.get_device_proc_addr().is_some());
    }
}
