# Validation Layers & Debugging

<!-- Phase 6.3.9 -->
<!-- Independent chapter, can be read at any time -->
<!-- Pedagogy: this is the "debugging as pedagogy" concept from
     the interdisciplinary section of pedagogy.md -->

## Motivation

Vulkan does almost no error checking at runtime, calling a function
incorrectly is undefined behavior, not an error message. This is fast
but makes debugging brutal.

Validation layers are optional middleware that intercepts every Vulkan
call and checks it against the spec. They catch invalid usage, report
synchronization hazards, and point you to the spec section that explains
what went wrong. You should *always* enable them during development.

## Intuition

Validation layers are like a strict code reviewer sitting between your
application and the driver. Every API call goes through the reviewer first.
In development, the reviewer catches your mistakes before they reach the
driver. In production, you remove the reviewer and let calls go straight
through.

<!-- TODO: How to enable VK_LAYER_KHRONOS_validation -->
<!-- TODO: Debug messenger setup, getting callbacks in your code -->
<!-- TODO: Common validation errors and what they mean -->
<!-- TODO: Performance implications (why you disable in release) -->

> *Before reading on: why do you think Vulkan chose to make error
> checking optional instead of always-on?*

## Worked example

<!-- TODO: Enable validation, create debug messenger, trigger an
     intentional error, read the output -->

## Formal reference

<!-- TODO: VK_LAYER_KHRONOS_validation, VkDebugUtilsMessengerEXT -->
<!-- TODO: Message severity and type filtering -->
<!-- TODO: Best practices layer, synchronization validation -->
<!-- TODO: Links to rustdoc -->
