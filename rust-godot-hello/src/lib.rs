use godot::prelude::*;
mod player;

struct RustGotdotExampleExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustGotdotExampleExtension {}
