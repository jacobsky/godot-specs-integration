use gdnative::prelude::*;
use gdnative::api::ShaderMaterial;

use specs::prelude::*;
use crate::GDWorldHybrid;
use std::collections::HashMap;
// This sample entity cannot be created by godot directly
#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct GDEntityHybrid {
    // This contains a list of the components that this entity must have.
    // Keys represent the component and the values represent the default properties for that component.
    // Keys the default values will then synchronize with the world from that point onwards.
    #[property]
    pub world_path: NodePath,
    // This is used to allow editor access from Godot as well as outputting the inforamtion
    #[property]
    pub components: Dictionary,
    // This inner components are visible to the rust layer only.
    pub (super) inner_components: HashMap<String, Variant>,
    // This is not visible to godot, but it will remain in Rust so that the entity can easily update it's properties
    // pub components_to_sync: HashSet<Components>
    pub entity: Option<Entity>,
}

#[methods]
impl GDEntityHybrid {
    fn new(_: &Node2D) -> Self {
        Self {
            // Parent is the default node path
            world_path: NodePath::from_str(".."),
            components: Dictionary::new().into_shared(),
            inner_components: HashMap::new(),
            entity: None,
        }
    }

    #[gdnative::profiled]
    pub fn set_component<T: ToVariant>(&mut self, key: &str, value: T) {
        self.inner_components.insert(key.to_owned(), value.to_variant());
    }
    
    #[export]
    #[gdnative::profiled]
    pub fn _ready(&mut self, owner: TRef<Node2D>) {
        // Synchronize the components from the Editor exposed dictionary
        for (k, v) in self.components.iter() {
            if let Some(key) = k.try_to_string() {
                // It is possible that when it was spawned it was spawned with inner_components of the same type.
                // As inner can only be set at runtime, but also before ready, they should always take precedence over editor.
                // It might be worth adding a warning message or something when this occurs.
                if !self.inner_components.contains_key(key.as_str()) {
                    self.inner_components.insert(key, v);
                }
            }
        }
        // This grabs the GDWorld based on the Scene itself, but likely the World will be one of many, it would be much cleaner to have another
        // Object (such as an Autoload singleton) that holds this information for us.
        let world = unsafe {
            owner
            .as_ref()
            .get_node_as_instance::<GDWorldHybrid>(self.world_path.to_string().as_str()) };
        if let Some(world) = world {
            let result = world.map_mut(|w, _|{
                w.create_entity_from(self, owner)
            }).expect("this should work");
            if let Some(entity) = result {
                self.entity = Some(entity);
                log::trace!("entity initialized to {:?}", self.entity);
            } else {
                log::error!("entity was not created");
            }
        } else {
            log::warn!("There is no GDWorld Node at world path {}", self.world_path.to_string());
        }
        
    }

    /// Synchronizes the internal components
    #[inline]
    #[gdnative::profiled]
    fn sync_internal_components(&mut self, world: &World) {
        use crate::{Player, ShaderParams};
        use specs_engine::{Position, Rotation, Scale, AngularVelocity, Velocity,  Counter, SetVelocityIntent, StayInsideBounds};
        macro_rules! update_components {
            ($entity:ident, $($type:ident),*) => {
                {
                    
                    $(
                        let storage = world.read_storage::<$type>();
                        if let Some(component) = storage.get($entity) {
                            self.inner_components.insert(stringify!($type).to_owned(), component.to_variant());
                        }
                    )*
                }
            }
        }
        let entity = self.entity.expect("this should work");
        update_components!(entity, Position, Rotation, Scale, AngularVelocity, Velocity, SetVelocityIntent, StayInsideBounds, Player, Counter, ShaderParams);
    }

    /// Synchronizes the Entity to it's state in the world. If the entity is deleted, it will free itself from the scene tree.
    /// As this is not exported, this cannot leverage signals.
    #[gdnative::profiled]
    pub fn synchronize(&mut self, owner: TRef<Node2D>, world: &World) {
        // TODO: Fix this part of the thingy up.
        if let Some(entity) = self.entity {
            if world.entities().is_alive(entity) {
                // self.sync_internal_components(world);
                
                // In addition, we can expose certain information directly to Godot by adding variant support.
            } else {
                log::info!("{:?} has been deleted, cleaning up", self);
                owner.queue_free();
            }
        } else {
            // If it doesn't exist after synchronization something went very wrong
            log::error!("{:?} does not have an entity registered", self);
            owner.queue_free();
        }
    }
    // // TODO: Finish this
    // #[export]
    // pub fn on_world_updated(&mut self, owner: TRef<Node2D>, world: Variant) {
    //     // This simplifies what kinds of information is supported for querying from the work.
        
    //     if let Some(world_owner) = world.try_to_object::<Node>() {
    //         let world_owner = unsafe { world_owner.assume_safe() };
    //         let world : RefInstance<GDWorldHybrid, Shared> = world_owner.cast_instance().expect("should successfully cast");
    //         world.map(|world, _| {
    //             self.synchronize(owner, &world.world);
    //         }).expect("this should succeed without errors");
    //         // Synchronizes the output dictionary after all of the synchronization has been completed.
    //         let updated_dict = Dictionary::new();
    //         for (key, v) in self.inner_components.iter() {
    //             updated_dict.insert(key, v);
    //         }
    //         self.components = updated_dict.into_shared();
    //     } else {
    //         log::error!("world is not a Ref<Node>");
    //     }
        
    //     // cast the world
    //     // Call synchronize internally
    //     // self.synchronize(owner, world)
    //     // In addition update the components from the GDWorld (which should know how to do it.)
    //     // let self.components = World.get_entity_components(self.entity);
    // }
}