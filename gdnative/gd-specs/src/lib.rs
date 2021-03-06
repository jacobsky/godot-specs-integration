//! This crate is used to contain all of the "glue" code between the specs based engine
//! and the game runner itself.
//! This crate may also add in some gdscript specific systems and components that can be used
use gdnative::prelude::*;

mod components;
mod dispatchers;
mod examples;
mod game;
mod systems;

pub use components::*;
pub use dispatchers::*;
pub use examples::*;
pub use game::*;
pub use systems::*;

// this is the world registry that is used to track the various game worlds.
const WORLD_REGISTRY_NAME : &'static str = "world_registry";

pub fn init(handle: InitHandle) {
    // Initialize the logger to make compatible with godot.
    // Each of the classes needs to be added in here.
    handle.add_class::<GDWorld>();
    handle.add_class::<GDEntity>();
    handle.add_class::<GDWorldHybrid>();
    handle.add_class::<GDEntityHybrid>();
}