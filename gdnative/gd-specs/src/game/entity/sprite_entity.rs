use gdnative::prelude::*;
use specs::prelude::*;
use std::{collections::HashMap};
use super::{EntityRef, ComponentInfo};

/// The Sprite version of the GodotAccessible entity that is made for use with the engine
#[derive(NativeClass)]
#[inherit(Sprite)]
pub struct EntitySprite {
    #[property]
    component_settings: Dictionary,
    pub (crate) components: ComponentInfo,
    entity: Option<Entity>,
}

#[methods]
impl EntitySprite {
    fn new (_: &Sprite) -> Self {
        Self {
            component_settings: Dictionary::new().into_shared(),
            components: ComponentInfo::empty(),
            entity: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Sprite) {
        let registry = unsafe { autoload::<Node>(crate::WORLD_REGISTRY_NAME) };
    }
    /// This is used in place of other types as there is noactual entities.
    #[export]
    pub fn has_entity(&self, _: &Sprite) -> bool {
        self.entity.is_some()
    }

    /// Copies the entity to be referencable by Godot
    #[export]
    pub fn entity(&self, _: &Sprite) -> Variant {
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