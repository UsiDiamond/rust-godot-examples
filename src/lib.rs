use godot::prelude::*;

struct RustGotdotExampleExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustGotdotExampleExtension {}
