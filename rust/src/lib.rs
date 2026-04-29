mod ball;
mod brick;
mod bricks;
mod paddle;
mod scene_main;
mod state;
mod ui;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
