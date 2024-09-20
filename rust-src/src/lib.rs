mod addons;
mod ui;
mod network;

use godot::prelude::*;

struct RustSrc;

#[gdextension]
unsafe impl ExtensionLibrary for RustSrc {}