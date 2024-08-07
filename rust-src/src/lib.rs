mod example;

use godot::prelude::*;

struct RustSrc;

#[gdextension]
unsafe impl ExtensionLibrary for RustSrc {}