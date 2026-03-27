#![allow(dead_code)]

mod parse;
mod type_map;

use std::path::Path;

fn main() {
    let vk_xml = Path::new(env!("CARGO_MANIFEST_DIR")).join("vk.xml");
    let registry = parse::parse_registry(&vk_xml);

    println!("=== vk.xml parse summary ===");
    println!("  handles:   {}", registry.handles.len());
    println!("  structs:   {}", registry.structs.len());
    println!("  enums:     {}", registry.enums.len());
    println!("  bitmasks:  {}", registry.bitmasks.len());
    println!("  commands:  {}", registry.commands.len());
    println!("  constants: {}", registry.constants.len());
    println!("  aliases:   {}", registry.aliases.len());

    // Spot-check a few known types.
    let has_instance = registry.handles.iter().any(|h| h.name == "Instance");
    let has_buffer_create = registry
        .structs
        .iter()
        .any(|s| s.name == "BufferCreateInfo");
    let has_format = registry.enums.iter().any(|e| e.name == "Format");
    let has_buffer_usage = registry
        .bitmasks
        .iter()
        .any(|b| b.name == "BufferUsageFlagBits");
    let has_create_instance = registry
        .commands
        .iter()
        .any(|c| c.name == "vkCreateInstance");

    println!();
    println!("=== spot checks ===");
    println!("  Instance handle:      {has_instance}");
    println!("  BufferCreateInfo:     {has_buffer_create}");
    println!("  Format enum:          {has_format}");
    println!("  BufferUsageFlagBits:  {has_buffer_usage}");
    println!("  vkCreateInstance cmd: {has_create_instance}");
}
