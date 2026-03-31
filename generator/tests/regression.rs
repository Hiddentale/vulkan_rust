//! Regression tests that run against the real vk.xml registry and generated output.
//!
//! These tests catch classes of bugs that would otherwise only surface when
//! vk.xml is updated to a newer Vulkan version. Each test guards a specific
//! invariant that the generator must uphold across all types and commands.

use std::path::Path;

/// Load the real vk.xml registry for testing.
fn load_registry() -> generator::parse::VkRegistry {
    let vk_xml = Path::new(env!("CARGO_MANIFEST_DIR")).join("vk.xml");
    generator::parse::parse_registry(&vk_xml)
}

/// Read generated file content as a string.
fn read_generated(relative: &str) -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    std::fs::read_to_string(root.join(relative))
        .unwrap_or_else(|e| panic!("failed to read {relative}: {e}"))
}

// ═══════════════════════════════════════════════════════════════════
// Test 1: No raw u64/u32 fallback for aliased Flags types
// ═══════════════════════════════════════════════════════════════════

/// If vk.xml defines a bitmask alias (e.g. VkPipelineCreateFlags2KHR →
/// VkPipelineCreateFlags2), the generated output must emit a proper type
/// alias, not a raw `u64` or `u32` fallback.
#[test]
fn no_raw_fallback_for_aliased_flags() {
    let registry = load_registry();
    let structs_rs = read_generated("vk-sys/src/structs.rs");

    // Collect all bitmask alias names from the registry.
    let bitmask_alias_names: Vec<&str> = registry
        .aliases
        .iter()
        .filter(|a| a.name.contains("Flags") && a.target.contains("Flags"))
        .map(|a| {
            a.name
                .strip_prefix("Vk")
                .unwrap_or(&a.name)
        })
        .collect();

    let mut bad = Vec::new();
    for name in &bitmask_alias_names {
        // Check if this alias appears as `pub type XxxFlags = u32;` or `= u64;`
        let pattern_u32 = format!("pub type {name} = u32;");
        let pattern_u64 = format!("pub type {name} = u64;");
        if structs_rs.contains(&pattern_u32) || structs_rs.contains(&pattern_u64) {
            bad.push(*name);
        }
    }

    assert!(
        bad.is_empty(),
        "These aliased Flags types fell back to raw u32/u64 instead of a proper type alias:\n  {}",
        bad.join("\n  ")
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 4: Two-call method count regression
// ═══════════════════════════════════════════════════════════════════

/// The number of generated two-call methods (enumerate_two_call + fill_two_call)
/// must not decrease. A decrease means a command was misclassified or dropped.
#[test]
fn two_call_method_count_does_not_regress() {
    let instance_wrappers = read_generated("vk-engine/src/generated/instance_wrappers.rs");
    let device_wrappers = read_generated("vk-engine/src/generated/device_wrappers.rs");
    let all = format!("{instance_wrappers}\n{device_wrappers}");

    let enumerate_count = all.matches("enumerate_two_call").count();
    let fill_count = all.matches("fill_two_call").count();
    let total = enumerate_count + fill_count;

    // Current baseline: 51 two-call methods. This must only grow.
    assert!(
        total >= 51,
        "Two-call method count regressed: found {total} (enumerate={enumerate_count}, fill={fill_count}), expected >= 51"
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 5: No two-call for single-output commands
// ═══════════════════════════════════════════════════════════════════

/// Every command classified as Enumerate or Fill must actually have the
/// output count/array pair in its parameters (validated via vk.xml metadata).
#[test]
fn two_call_commands_have_output_pairs() {
    let registry = load_registry();
    let pnext = generator::wrapper_utils::build_pnext_struct_set(&registry);

    let mut false_positives = Vec::new();

    for cmd in &registry.commands {
        let roles = generator::wrapper_utils::classify_params(cmd, &pnext);
        let pattern = generator::wrapper_utils::classify_command(cmd, &roles);

        if matches!(
            pattern,
            generator::wrapper_utils::CommandPattern::Enumerate
                | generator::wrapper_utils::CommandPattern::Fill
        ) {
            // Must have at least one OutputArray role.
            let has_output_array = roles
                .iter()
                .any(|r| matches!(r, generator::wrapper_utils::ParamRole::OutputArray { .. }));
            if !has_output_array {
                false_positives.push(cmd.name.as_str());
            }
        }
    }

    assert!(
        false_positives.is_empty(),
        "Commands classified as Enumerate/Fill but missing OutputArray:\n  {}",
        false_positives.join("\n  ")
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 6: Return type consistency for two-call commands
// ═══════════════════════════════════════════════════════════════════

/// Enumerate commands must return VkResult. Fill commands must return void.
#[test]
fn two_call_return_type_consistency() {
    let registry = load_registry();
    let pnext = generator::wrapper_utils::build_pnext_struct_set(&registry);

    let mut mismatches = Vec::new();

    for cmd in &registry.commands {
        let roles = generator::wrapper_utils::classify_params(cmd, &pnext);
        let pattern = generator::wrapper_utils::classify_command(cmd, &roles);

        match pattern {
            generator::wrapper_utils::CommandPattern::Enumerate => {
                if cmd.return_type != "VkResult" {
                    mismatches.push(format!(
                        "{}: Enumerate but returns '{}'",
                        cmd.name, cmd.return_type
                    ));
                }
            }
            generator::wrapper_utils::CommandPattern::Fill => {
                if cmd.return_type != "void" {
                    mismatches.push(format!(
                        "{}: Fill but returns '{}'",
                        cmd.name, cmd.return_type
                    ));
                }
            }
            _ => {}
        }
    }

    assert!(
        mismatches.is_empty(),
        "Two-call return type mismatches:\n  {}",
        mismatches.join("\n  ")
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 7: No duplicate type definitions
// ═══════════════════════════════════════════════════════════════════

/// Scan generated output for duplicate `pub type`, `pub struct`, or
/// `pub union` definitions. Promoted extensions can cause duplication.
#[test]
fn no_duplicate_type_definitions() {
    let files = [
        "vk-sys/src/handles.rs",
        "vk-sys/src/enums.rs",
        "vk-sys/src/bitmasks.rs",
        "vk-sys/src/structs.rs",
        "vk-sys/src/constants.rs",
        "vk-sys/src/commands.rs",
    ];

    let mut all_duplicates = Vec::new();

    for file in &files {
        let content = read_generated(file);
        let mut type_names: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();

        for line in content.lines() {
            let trimmed = line.trim();
            let name = if let Some(rest) = trimmed.strip_prefix("pub struct ") {
                rest.split(|c: char| c == '(' || c == '{' || c == '<' || c.is_whitespace())
                    .next()
            } else if let Some(rest) = trimmed.strip_prefix("pub union ") {
                rest.split(|c: char| c == '{' || c == '<' || c.is_whitespace())
                    .next()
            } else if let Some(rest) = trimmed.strip_prefix("pub type ") {
                rest.split(|c: char| c == '=' || c.is_whitespace())
                    .next()
            } else {
                None
            };

            if let Some(name) = name
                && !name.is_empty()
            {
                *type_names.entry(name.to_string()).or_default() += 1;
            }
        }

        for (name, count) in &type_names {
            if *count > 1 {
                all_duplicates.push(format!("{file}: '{name}' defined {count} times"));
            }
        }
    }

    assert!(
        all_duplicates.is_empty(),
        "Duplicate type definitions found:\n  {}",
        all_duplicates.join("\n  ")
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 8: All struct members resolve to known types
// ═══════════════════════════════════════════════════════════════════

/// Every struct member type in the registry must resolve to either a
/// primitive, a generated type, or a valid alias. This catches broken
/// type references when new types are added to vk.xml.
#[test]
fn all_struct_members_resolve() {
    let registry = load_registry();

    // Collect all known type names (handles, structs, enums, bitmasks, aliases).
    let mut known: std::collections::HashSet<String> = std::collections::HashSet::new();

    for h in &registry.handles {
        known.insert(h.name.clone());
    }
    for s in &registry.structs {
        known.insert(s.name.clone());
    }
    for e in &registry.enums {
        known.insert(e.name.clone());
    }
    for b in &registry.bitmasks {
        known.insert(b.name.clone());
        known.insert(b.flags_name.clone());
    }
    for a in &registry.aliases {
        known.insert(a.name.strip_prefix("Vk").unwrap_or(&a.name).to_string());
        known.insert(
            a.target
                .strip_prefix("Vk")
                .unwrap_or(&a.target)
                .to_string(),
        );
    }
    for fp in &registry.func_pointers {
        known.insert(fp.name.clone());
    }
    for name in registry.base_types.keys() {
        known.insert(name.clone());
    }

    // Primitives and known C types.
    let primitives = [
        "void",
        "char",
        "float",
        "double",
        "int",
        "uint8_t",
        "uint16_t",
        "uint32_t",
        "uint64_t",
        "int32_t",
        "int64_t",
        "size_t",
    ];
    for p in &primitives {
        known.insert(p.to_string());
    }

    // Reserved Flags types (empty bitmasks with no FlagBits) are resolved as
    // u32/u64 aliases in the generated output. They're valid types even though
    // they don't appear in the bitmask list. Recognize any VkXxxFlags pattern.
    let is_known_flags = |name: &str| -> bool {
        let stripped = name.strip_prefix("Vk").unwrap_or(name);
        stripped.contains("Flags") && !stripped.contains("FlagBits")
    };

    let mut unresolved = Vec::new();

    for s in &registry.structs {
        for m in &s.members {
            let stripped = m
                .type_name
                .strip_prefix("Vk")
                .unwrap_or(&m.type_name);
            if !known.contains(stripped)
                && !known.contains(&m.type_name)
                && !m.type_name.starts_with("StdVideo")
                && !is_known_flags(&m.type_name)
                && generator::type_map::c_type_to_rust(&m.type_name).is_none()
            {
                unresolved.push(format!("{}.{}: type '{}'", s.name, m.name, m.type_name));
            }
        }
    }

    assert!(
        unresolved.is_empty(),
        "Unresolved struct member types:\n  {}",
        unresolved.join("\n  ")
    );
}

// ═══════════════════════════════════════════════════════════════════
// Test 9: Command dispatch level classification
// ═══════════════════════════════════════════════════════════════════

/// For every command in InstanceCommands, its first parameter must be
/// VkInstance or VkPhysicalDevice. For DeviceCommands, it must be
/// VkDevice, VkQueue, or VkCommandBuffer. Misclassification means
/// loading via the wrong proc addr.
#[test]
fn command_dispatch_level_is_correct() {
    let registry = load_registry();

    let instance_handles = ["VkInstance", "VkPhysicalDevice"];
    let device_handles = ["VkDevice", "VkQueue", "VkCommandBuffer"];

    let mut misclassified = Vec::new();

    for cmd in &registry.commands {
        let first_param_type = cmd
            .params
            .first()
            .map(|p| p.type_name.as_str())
            .unwrap_or("");

        match cmd.dispatch_level {
            generator::parse::DispatchLevel::Instance => {
                if !instance_handles.contains(&first_param_type) {
                    misclassified.push(format!(
                        "{}: Instance-level but first param is '{first_param_type}'",
                        cmd.name
                    ));
                }
            }
            generator::parse::DispatchLevel::Device => {
                if !device_handles.contains(&first_param_type) {
                    misclassified.push(format!(
                        "{}: Device-level but first param is '{first_param_type}'",
                        cmd.name
                    ));
                }
            }
            generator::parse::DispatchLevel::Entry => {
                // Entry commands can have any first param (or none).
            }
        }
    }

    assert!(
        misclassified.is_empty(),
        "Commands with incorrect dispatch level:\n  {}",
        misclassified.join("\n  ")
    );
}
