//! StructureType constant resolution for Default impls and builders.

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::{EnumValue, StructDef, VkRegistry};

/// Return a `StructureType::from_raw(N)` token for a struct's sType member.
pub fn struct_stype(
    def: &StructDef,
    enum_variants: &HashSet<String>,
    stype_raw: &HashMap<String, i32>,
) -> Option<TokenStream> {
    def.members.iter().find_map(|m| {
        if m.name == "sType" {
            m.values
                .as_deref()
                .and_then(|v| stype_constant(v, enum_variants, stype_raw))
        } else {
            None
        }
    })
}

/// Build a map of C sType name → raw i32 value.
pub fn build_raw_map(registry: &VkRegistry) -> HashMap<String, i32> {
    let Some(stype_enum) = registry.enums.iter().find(|e| e.name == "StructureType") else {
        return HashMap::new();
    };
    stype_enum
        .variants
        .iter()
        .filter_map(|v| match &v.value {
            EnumValue::I32(val) => Some((v.name.clone(), *val)),
            _ => None,
        })
        .collect()
}

/// Build the set of StructureType variant names that exist as `pub const` in the
/// generated enum. Mirrors the enum emitter's deduplication logic.
pub fn build_variant_set(registry: &VkRegistry) -> HashSet<String> {
    use crate::emit_enums::{enum_variant_prefix, strip_variant_prefix};

    let Some(stype_enum) = registry.enums.iter().find(|e| e.name == "StructureType") else {
        return HashSet::new();
    };

    let prefix = enum_variant_prefix(&stype_enum.name);
    let mut seen = HashSet::new();
    stype_enum
        .variants
        .iter()
        .filter_map(|v| {
            let rust_name = strip_variant_prefix(&v.name, &prefix)?;
            if seen.insert(rust_name.clone()) {
                Some(rust_name)
            } else {
                None
            }
        })
        .collect()
}

fn stype_constant(
    values: &str,
    _enum_variants: &HashSet<String>,
    stype_raw: &HashMap<String, i32>,
) -> Option<TokenStream> {
    stype_raw
        .get(values)
        .map(|&raw| quote! { StructureType::from_raw(#raw) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::MemberDef;

    fn make_member(name: &str, type_name: &str) -> MemberDef {
        MemberDef {
            name: name.to_string(),
            type_name: type_name.to_string(),
            is_pointer: false,
            is_const: false,
            is_double_pointer: false,
            array_size: None,
            optional: false,
            values: None,
            len: None,
            extern_sync: None,
        }
    }

    fn make_pointer_member(name: &str, type_name: &str, is_const: bool) -> MemberDef {
        MemberDef {
            is_pointer: true,
            is_const,
            ..make_member(name, type_name)
        }
    }

    fn make_raw_map() -> HashMap<String, i32> {
        let mut m = HashMap::new();
        m.insert("VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO".to_string(), 12);
        m
    }

    #[test]
    fn stype_constant_uses_from_raw() {
        let raw = make_raw_map();
        let tokens = stype_constant(
            "VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO",
            &HashSet::new(),
            &raw,
        );
        assert!(tokens.unwrap().to_string().contains("from_raw"));
    }

    #[test]
    fn struct_stype_finds_value() {
        let def = StructDef {
            name: "BufferCreateInfo".to_string(),
            members: vec![
                MemberDef {
                    values: Some("VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO".to_string()),
                    ..make_member("sType", "VkStructureType")
                },
                make_pointer_member("pNext", "void", true),
            ],
            extends: vec![],
            returned_only: false,
            is_union: false,
            provided_by: None,
        };
        let raw = make_raw_map();
        let result = struct_stype(&def, &HashSet::new(), &raw);
        assert!(result.is_some());
        assert!(result.unwrap().to_string().contains("from_raw"));
    }

    #[test]
    fn struct_stype_returns_none_for_plain_struct() {
        let def = StructDef {
            name: "Extent2D".to_string(),
            members: vec![make_member("width", "uint32_t")],
            extends: vec![],
            returned_only: false,
            is_union: false,
            provided_by: None,
        };
        assert!(struct_stype(&def, &HashSet::new(), &HashMap::new()).is_none());
    }
}
