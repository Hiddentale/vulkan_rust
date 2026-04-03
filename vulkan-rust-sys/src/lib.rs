//! Raw Vulkan FFI types generated from `vk.xml`.
//!
//! Do not edit by hand,regenerate with the `generator` crate.
//!
//! Every type carries a spec link, and structs include metadata from
//! vk.xml: extension provenance, pNext chain relationships, member
//! annotations (optional, length-of, thread safety).

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod string_array;
pub use string_array::{
    StringArray, ExtensionName, LayerName, DeviceName,
    DescriptionName, DriverName, DriverInfo,
};

mod handles;
mod enums;
mod bitmasks;
mod constants;
pub mod extension_names;
mod structs;
mod builders;
mod clear_value;
pub mod commands;

#[doc(inline)]
pub use handles::*;
#[doc(inline)]
pub use enums::*;
#[doc(inline)]
pub use bitmasks::*;
#[doc(inline)]
pub use constants::*;
#[doc(inline)]
pub use structs::*;
#[doc(inline)]
pub use builders::*;

/// Vulkan device memory size, in bytes.
pub type DeviceSize = u64;
/// Vulkan device memory address.
pub type DeviceAddress = u64;
