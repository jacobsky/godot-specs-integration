use gdnative::prelude::*;
use gdnative::api::ShaderMaterial;

use specs::prelude::*;
use crate::GDWorld;
use std::collections::HashMap;
// This sample entity cannot be created by godot directly
#[derive(Debug, NativeClass)]
#[inherit(Node2D)]
pub struct GDEntity {
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
impl GDEntity {
    fn new(_: &Node2D) -> Self {
        Self {
            // Parent is the default node path
            world_path: NodePath::from_str(".."),
            components: Dictionary::new().into_shared(),
            inner_components: HashMap::new(),
            entity: None,
        }
    }

    pub fn set_component<T: ToVariant>(&mut self, key: &str, value: T) {
        self.inner_components.insert(key.to_owned(), value.to_variant());
    }
    
    #[export]
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
            .get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) };
        if let Some(world) = world {
            let result = world.map_mut(|w, o|{
                let e = w.create_entity_from(self, owner);
                if e.is_some() {
                    let binds = VariantArray::new();
                    binds.push(o.to_variant());
                    o.connect("update_completed", owner, "on_world_updated", binds.into_shared(), Object::CONNECT_DEFERRED).expect("this should work");
                }
                e
            }).expect("this should work");
            if let Some(entity) = result {
                self.entity = Some(entity);
                log::info!("entity initialized to {:?}", self.entity);
            } else {
                log::error!("entity was not created");
            }
        } else {
            log::warn!("There is no GDWorld Node at world path {}", self.world_path.to_string());
        }
        
    }
    // pub fn register_with_world(&mut self, owner: &Node2D, gd_world: &mut World) {
    //     let world = unsafe { gd_world.assume_safe() };
    //     self.entity = world.map_mut(|w, _| {
    //         gd_world.create_entity(self.components)
    //     }).expect("this should not have any errors");
    // }
    

    /// Synchronizes the Entity and SceneTree objects with the data from the ECS World
    #[inline]
    fn sync_scene_tree(&mut self, owner: TRef<Node2D>, world: &World) {
        use crate::{ShaderParams, TextureOverride};
        use specs_engine::{Position, Scale, Rotation};
        let entity = self.entity.expect("this should work");
        // TODO: These probably shouldn't be optional since they are in Node2D which this inherits.
        // It would be cleaner (and more correct) to just include an ".expects"
        if let Some(position) = world.read_storage::<Position>().get(entity) {
            // In this case, I only care about the global position, but it would be possible to use the local position
            // To base the scale off of the parent
            owner.set_global_position(Vector2::new(position.x, position.y));
        }
        if let Some(scale)  = world.read_storage::<Scale>().get(entity) {
            owner.set_global_scale(Vector2::new(scale.x, scale.y));
        }
        if let Some(rotation)  = world.read_storage::<Rotation>().get(entity) {
            owner.set_global_rotation(rotation.radians.into());
        }
        // We will give an option for the ECS to override the currently drawn texture in godot.
        if let Some(texture) = world.read_storage::<TextureOverride>().get(entity) {
            // First we attempt to find the "sprite" node which we assume is attached to this entity
            if let Some(child) = owner.get_node("sprite") {
                // For safe modification we have to assert that this is the ONLY reference to the sprite at this time.
                // As this is internal to the sprite, we can safely assert this.
                let child = unsafe { child.assume_unique() };
                // Next, we cast the `Node` into `Sprite` to get the correct interface
                if let Ok(sprite) = child.try_cast::<Sprite>() {
                    // Finally we can set the texture
                    sprite.set_texture(texture.texture.clone());
                } else {
                    log::error!("cannot cast `sprite` to CanvasItem");
                }
            }
        }
        // Now we may also want to update the shader params if those have been updated by the engine.
        // This one is a bit messier due to how deep into the resource we need to dig, but the process is the same as the texture.
        if let Some(shader_params) = world.read_storage::<ShaderParams>().get(entity) {
            // First, try to get the "sprite" node
            if let Some(child) = owner.get_node("sprite") {
                // Assert that this is unique so that we can safetly modify it.
                let child = unsafe { child.assume_unique() };
                // Cast it to a CanvasItem for the interface to get the material properties.
                if let Ok(canvas_item) = child.try_cast::<CanvasItem>() {
                    // Check that we have a material on the canvas_item
                    if let Some(material)  = canvas_item.material() {
                        // Cast the `Ref<Material>` to `Ref<ShaderMaterial>` to use the correct interface. 
                        if let Ok(material) = material.try_cast::<ShaderMaterial>() {
                            let material = unsafe { material.assume_safe() };
                            // Note: This could also be done using a HashMap if more dynamic properties are required.
                            // This example is specifically trying to be simple.
                            // for (k, v) in shader_params.params {
                            //      material.set_shader_param(k, v)
                            //}
                            material.set_shader_param("fg", shader_params.fg);
                            material.set_shader_param("bg", shader_params.bg);
                        } else {
                            log::error!("`sprite`'s Material is not a `ShaderMaterial`");
                        }
                    }
                    else {
                        log::error!("`sprite`'s CanvasItem has no material");
                    }
                } else {
                    log::error!("cannot cast `sprite` to CanvasItem");
                }
            }
        }
    }
    /// Synchronizes the internal components
    #[inline]
    fn sync_internal_components(&mut self, world: &World) {
        use crate::{Player, ShaderParams};
        use specs_engine::{Velocity,  Counter, SetVelocityIntent, StayInsideBounds};
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
        update_components!(entity, Velocity, SetVelocityIntent, StayInsideBounds, Player, Counter, ShaderParams);
    }

    /// Synchronizes the Entity to it's state in the world. If the entity is deleted, it will free itself from the scene tree.
    /// As this is not exported, this cannot leverage signals.
    pub fn synchronize(&mut self, owner: TRef<Node2D>, world: &World) {
        // TODO: Fix this part of the thingy up.
        if let Some(entity) = self.entity {
            if world.entities().is_alive(entity) {
                self.sync_scene_tree(owner, world);
                self.sync_internal_components(world);
                
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
    // TODO: Finish this
    #[export]
    pub fn on_world_updated(&mut self, owner: TRef<Node2D>, world: Variant) {
        // This simplifies what kinds of information is supported for querying from the work.
        
        if let Some(world_owner) = world.try_to_object::<Node>() {
            let world_owner = unsafe { world_owner.assume_safe() };
            let world : RefInstance<GDWorld, Shared> = world_owner.cast_instance().expect("should successfully cast");
            world.map(|world, _| {
                self.synchronize(owner, &world.world);
            }).expect("this should succeed without errors");
            // Synchronizes the output dictionary after all of the synchronization has been completed.
            let updated_dict = Dictionary::new();
            for (key, v) in self.inner_components.iter() {
                updated_dict.insert(key, v);
            }
            self.components = updated_dict.into_shared();
        } else {
            log::error!("world is not a Ref<Node>");
        }
        
        // cast the world
        // Call synchronize internally
        // self.synchronize(owner, world)
        // In addition update the components from the GDWorld (which should know how to do it.)
        // let self.components = World.get_entity_components(self.entity);
    }
}