use gdnative::prelude::*;
use specs::prelude::*;
use std::{collections::HashMap};
use crate::WorldRegistry;

use super::EntityRef;
/// The Node2D version of the GodotAccessible entity that is made for use with the engine
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct EntityNode2D {
    #[property]
    owned_by_world: GodotString,
    pub (crate) components: HashMap<String, Variant>,
    entity: Option<Entity>,
}

#[methods]
impl EntityNode2D {
    fn new (_: &Node2D) -> Self {
        Self {
            owned_by_world: GodotString::from(""),
            components: HashMap::new(),
            entity: None,
        }
    }
    #[export]
    pub fn _ready(&mut self, owner: &Node2D) {
        if let Some(node) = unsafe { autoload::<Node>(crate::WORLD_REGISTRY_NAME) } {
            if let Some(registry) = node.cast_instance::<WorldRegistry>() {
                registry.map_mut(|w, o|{
                    // Register this entity with the world and create itself.]
                    // Needs to register each subnode in turn before registering itself with the references to the children
                }).expect("this should not fail"); 
            } else {
                log::error!("{} is found in the scene tree but it is not of type `WorldRegistry`", crate::WORLD_REGISTRY_NAME); 
            }
        } else {
            log::error!("WorldRegistry node cannot be found in the scene tree. Add it to the autoload singleton list as {}", crate::WORLD_REGISTRY_NAME);
        }
    }
    /// This is used in place of other types as there is noactual entities.
    #[export]
    pub fn has_entity(&self, _: &Node2D) -> bool {
        self.entity.is_some()
    }

    /// Copies the entity to be referencable by Godot
    #[export]
    pub fn entity(&self, _: &Node2D) -> Variant {
        if let Some(entity) = self.entity {
            EntityRef {
                entity
            }.emplace().owned_to_variant()
        } else {
            // This will be considered to be nil if this entity hasn't been initialized or was deleted already.
            Variant::new()
        }
    }
}