//! Emits `&CStr` constants for Vulkan extension names.
//!
//! Each extension in `vk.xml` gets a constant like:
//! ```ignore
//! pub const KHR_SWAPCHAIN_EXTENSION_NAME: &CStr = c"VK_KHR_swapchain";
//! ```

use std::ffi::CString;

use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::parse::VkRegistry;

/// Emit extension name constants for all Vulkan extensions.
pub fn emit_extension_names(registry: &VkRegistry) -> TokenStream {
    let constants: Vec<TokenStream> = registry
        .extensions
        .iter()
        .map(|ext| {
            let vk_name = &ext.name;
            let rust_name = to_constant_name(vk_name);
            let ident = format_ident!("{}", rust_name);
            let cstr_lit = Literal::c_string(&CString::new(vk_name.as_bytes()).unwrap());

            quote! {
                #[doc(alias = #vk_name)]
                pub const #ident: &::core::ffi::CStr = #cstr_lit;
            }
        })
        .collect();

    quote! { #(#constants)* }
}

/// Convert `"VK_KHR_swapchain"` to `"KHR_SWAPCHAIN_EXTENSION_NAME"`.
fn to_constant_name(vk_name: &str) -> String {
    let stripped = vk_name.strip_prefix("VK_").unwrap_or(vk_name);
    let mut result = String::with_capacity(stripped.len() + "_EXTENSION_NAME".len());
    result.push_str(&stripped.to_ascii_uppercase());
    result.push_str("_EXTENSION_NAME");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{ExtensionDef, VkRegistry};

    fn empty_registry() -> VkRegistry {
        VkRegistry {
            handles: vec![],
            structs: vec![],
            enums: vec![],
            bitmasks: vec![],
            commands: vec![],
            constants: vec![],
            func_pointers: vec![],
            extensions: vec![],
            platforms: vec![],
            aliases: vec![],
            base_types: Default::default(),
        }
    }

    fn make_extension(name: &str) -> ExtensionDef {
        ExtensionDef {
            name: name.to_string(),
            number: 1,
            ext_type: None,
            platform: None,
            depends: None,
            promoted_to: None,
            deprecated_by: None,
            supported: "vulkan".to_string(),
            items: vec![],
        }
    }

    #[test]
    fn to_constant_name_khr_swapchain() {
        assert_eq!(
            to_constant_name("VK_KHR_swapchain"),
            "KHR_SWAPCHAIN_EXTENSION_NAME"
        );
    }

    #[test]
    fn to_constant_name_ext_debug_utils() {
        assert_eq!(
            to_constant_name("VK_EXT_debug_utils"),
            "EXT_DEBUG_UTILS_EXTENSION_NAME"
        );
    }

    #[test]
    fn to_constant_name_no_prefix() {
        assert_eq!(to_constant_name("SOME_EXT"), "SOME_EXT_EXTENSION_NAME");
    }

    #[test]
    fn emit_empty_registry() {
        let registry = empty_registry();
        let tokens = emit_extension_names(&registry);
        assert_eq!(tokens.to_string(), "");
    }

    #[test]
    fn emit_single_extension() {
        let mut registry = empty_registry();
        registry.extensions.push(make_extension("VK_KHR_swapchain"));
        let tokens = emit_extension_names(&registry);
        let code = tokens.to_string();
        assert!(code.contains("KHR_SWAPCHAIN_EXTENSION_NAME"));
        assert!(code.contains("VK_KHR_swapchain"));
        assert!(code.contains("CStr"));
    }

    #[test]
    fn emit_multiple_extensions() {
        let mut registry = empty_registry();
        registry.extensions.push(make_extension("VK_KHR_surface"));
        registry.extensions.push(make_extension("VK_KHR_swapchain"));
        registry
            .extensions
            .push(make_extension("VK_EXT_debug_utils"));
        let tokens = emit_extension_names(&registry);
        let code = tokens.to_string();
        assert!(code.contains("KHR_SURFACE_EXTENSION_NAME"));
        assert!(code.contains("KHR_SWAPCHAIN_EXTENSION_NAME"));
        assert!(code.contains("EXT_DEBUG_UTILS_EXTENSION_NAME"));
    }

    #[test]
    fn emit_has_doc_alias() {
        let mut registry = empty_registry();
        registry.extensions.push(make_extension("VK_KHR_swapchain"));
        let code = emit_extension_names(&registry).to_string();
        assert!(code.contains("VK_KHR_swapchain"));
    }

    #[test]
    fn generated_code_is_valid_rust() {
        let mut registry = empty_registry();
        registry.extensions.push(make_extension("VK_KHR_swapchain"));
        registry
            .extensions
            .push(make_extension("VK_EXT_debug_utils"));

        let tokens = emit_extension_names(&registry);
        let wrapped = quote! {
            mod test_mod {
                #tokens
            }
        };
        syn::parse2::<syn::File>(wrapped)
            .expect("generated extension name constants should be valid Rust");
    }
}
