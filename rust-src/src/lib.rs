mod addons;
mod gameframework;
mod ui;

use godot::prelude::*;

struct RustSrc;

#[gdextension]
unsafe impl ExtensionLibrary for RustSrc {}
