use crate::{SpecsWorld};
use specs_engine::ComponentRegistry;
use std::collections::HashMap;
use gdnative::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(NativeClass)]
#[inherit(Node)]
pub struct WorldRegistry {
    worlds: HashMap<String, SpecsWorld>,
}

#[methods]
impl WorldRegistry {
    fn new(_: &Node) -> Self {
        Self {
            worlds: HashMap::new()
        }
    }
    
    /// Registers an instance of a WorldRef so that it can be globally accessible to the game.
    pub fn register_world(&mut self, name: &str, world: SpecsWorld) {
        if self.worlds.insert(name.to_owned(), world).is_some() {
            log::warn!("World {} has been replaced", name.to_string().as_str());
        } else {
            log::info!("World {} has been registered", name.to_string().as_str());
        }
    }
    pub fn borrow_world(&mut self, name: &str) -> Option<&SpecsWorld> {
        self.worlds.get(name)
    }

    pub fn register<CR: ComponentRegistry>(&mut self, name: &str, registration: CR) {
        if let Some(world) = self.worlds.get_mut(name) {
            world.register_components::<CR>();
            // registration.register(&mut world.world);
        }
    }
}