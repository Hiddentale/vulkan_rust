/// Maps C type names from vk.xml to Rust type tokens.
pub fn c_type_to_rust(c: &str) -> Option<&'static str> {
    Some(match c {
        // Fixed-width integers
        "uint8_t" => "u8",
        "uint16_t" => "u16",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        "int32_t" => "i32",
        "int64_t" => "i64",
        "size_t" => "usize",

        // C primitives
        "float" => "f32",
        "double" => "f64",
        "char" => "std::ffi::c_char",
        "void" => "std::ffi::c_void",
        "int" => "std::ffi::c_int",

        // Vulkan typedefs
        "VkBool32" => "u32",
        "VkDeviceSize" => "u64",
        "VkDeviceAddress" => "u64",
        "VkFlags" => "u32",
        "VkFlags64" => "u64",
        "VkSampleMask" => "u32",

        // Platform types
        "HINSTANCE" => "isize",
        "HWND" => "isize",
        "HMONITOR" => "isize",
        "HANDLE" => "isize",
        "DWORD" => "u32",
        "LPCWSTR" => "*const u16",
        "SECURITY_ATTRIBUTES" => "std::ffi::c_void",

        // Vk-prefixed or unknown types are not primitives — caller handles them.
        _ => return None,
    })
}

/// True if this C type maps to a Rust primitive (not a generated Vk type).
pub fn is_primitive(c: &str) -> bool {
    !c.starts_with("Vk")
        || matches!(
            c,
            "VkBool32"
                | "VkDeviceSize"
                | "VkDeviceAddress"
                | "VkFlags"
                | "VkFlags64"
                | "VkSampleMask"
        )
}
