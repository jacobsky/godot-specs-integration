use gdnative::prelude::*;
use gdnative::api::ShaderMaterial;
use specs::prelude::*;
use specs_engine::{Position, Scale, Rotation, Velocity, SetVelocityIntent, StayInsideBounds, Counter, TreeRelationship, AngularVelocity};

use crate::{GDEntityHybrid, Player, CanvasItemShader, ShaderParams, TextureOverride};

/// This class wraps the specs world and allows it to easily pass the world instance between Godot and Specs.
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct GDWorldHybrid {
    pub world: World,
    dispatcher: Option<Dispatcher<'static, 'static>>,
}

#[methods]
impl GDWorldHybrid {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "update_completed",
            args: &[],
        });
    }
    fn new(_: &Node) -> Self {
        let mut world = World::new();
        // At creation the GDWorldHybrid needs to create any required components, these resources can also be added later by the class that holds this.
        world.insert(specs_engine::Time::new());
        crate::components::register_components(&mut world);
        specs_engine::register_components(&mut world);
        Self {
            world,
            dispatcher: None,
        }
    }
    
    pub fn insert_resource<T>(&mut self, resource: T) 
        where T:
        std::any::Any + Send + Sync
    {
        self.world.insert::<T>(resource);
    }


    /// Clones the resource and returns a copy. This is useful if you need to inspect an internal resource
    pub fn clone_resource<T>(&self) -> Option<T> 
        where T: Clone + Copy + std::any::Any + Send + Sync {
        self.world.try_fetch::<T>().map(|fetch| *fetch.to_owned())
    }
    
    /// Enqueues a message onto a WorldMsgQueue<T> if a queue exists.
    pub fn enqueue_message<T>(&self, message: T) 
        where T: std::any::Any + Send + Sync {
        if let Some(resource) = self.world.try_fetch::<specs_engine::WorldMsgQueue<T>>() {
            resource.push(message);
        } else {
            log::error!("queue does not exist for {:?}", std::any::type_name::<T>());
        }
    }

    /// Pops a message off of a WorldMsgQueue<T> message queue.
    pub fn pop_message<T>(&self) -> Option<T> 
        where T: std::any::Any + Send + Sync {

        if let Some(resource) = self.world.try_fetch::<specs_engine::WorldMsgQueue<T>>() {
            resource.pop()
        } else {
            log::error!("queue does not exist for {:?}", std::any::type_name::<T>());
            None
        }
    }
    /// Lets the game itself determine which systems this world operates when it needs to run.
    pub fn set_dispatcher(&mut self, dispatcher: Dispatcher<'static, 'static>) {
        self.dispatcher = Some(dispatcher)
    }

    pub fn set_component_for_entity<C: Component>(&mut self, entity: Entity, component: C) {
        let mut storage = self.world.write_storage::<C>();
        if let Some(c) = storage.get_mut(entity) {
            *c = component;
        } else {
            log::error!("entity [{:?}] does not have component {}", entity, std::any::type_name::<C>());
        }
    }

    pub fn get_entities_with<C: Component>(&mut self) -> Vec<Entity> {
        let entities = self.world.entities();
        let storage = self.world.read_storage::<C>();
        let mut entities_with = Vec::new();
        for (e, _) in (&entities, &storage).join() {
            entities_with.push(e);
        }
        entities_with
    }

    // pub fn set_component_for_expr<C: Component>(&mut self, components: )
    pub fn create_entities_child(&mut self, parent_entity: Entity, child_node: TRef<Node2D>) -> Entity{
        let mut eb = self.world.create_entity();
        eb = eb.with(Position::from(&child_node.position()));
        log::trace!("with Position {:?}", &child_node.position());
        eb = eb.with(Rotation { radians: child_node.rotation() as f32 });
        log::trace!("with Rotation {:?}", &child_node.rotation());
        eb = eb.with(Scale::from(&child_node.scale()));
        log::trace!("with Scale {:?}", &child_node.scale());
        // TODO: Fix this name collision
        eb = eb.with(crate::components::CanvasItem {
            rid: child_node.get_canvas_item(),
            canvas_rid: Some(child_node.get_canvas()),
        });
        log::trace!("with canvas_item {:?} and parent {:?}", &child_node.get_canvas_item(), child_node.get_canvas());
        eb = eb.with(TreeRelationship {
            parent: Some(parent_entity),
            // TODO: Support recursion :)
            children: Vec::new(),
        });
        eb.build()
    }
    /// Creates and entity from a GDEntity if possible.
    /// This should generally only be called by new GDEntities during `_ready()`
    /// Returns Some(Entity) if creation is successful.
    pub fn create_entity_from(&mut self, entity: &GDEntityHybrid, entity_owner: TRef<Node2D>) -> Option<Entity> {
        log::trace!("create_entity_from {:?} at position [{:?}]", entity.components, entity_owner.position());
        let mut eb = self.world.create_entity();
        let mut fg = Color::from_rgb(1.0, 1.0, 1.0);
        let bg = Color::from_rgba(0.0, 0.0, 0.0, 0.0);
        // Get the initial starting position of the Node2D
        eb = eb.with(Position::from(&entity_owner.position()));
        log::trace!("with Position {:?}", &entity_owner.position());
        eb = eb.with(Rotation { radians: entity_owner.rotation() as f32 });
        log::trace!("with Rotation {:?}", &entity_owner.rotation());
        eb = eb.with(Scale::from(&entity_owner.scale()));
        log::trace!("with Scale {:?}", &entity_owner.scale());
        // TODO: Fix this name collision
        eb = eb.with(crate::components::CanvasItem {
            rid: entity_owner.get_canvas_item(),
            canvas_rid: Some(entity_owner.get_canvas()),
        });
        log::trace!("with canvas_item {:?} and parent {:?}", &entity_owner.get_canvas_item(), entity_owner.get_canvas());
        eb = eb.with(TreeRelationship {
            parent: None,
            children: Vec::new(),
        });
        log::trace!("with canvas_item relations");
        if let Some(velocity) = entity.inner_components.get("Velocity") {
            if let Some(velocity) = velocity.try_to_vector2() {
                log::trace!("with Velocity [{}, {}]", velocity.x, velocity.y);
                eb = eb.with(
                    Velocity {
                        x: velocity.x,
                        y: velocity.y,
                });
            } else {
                log::error!("velocity must be a vec2");
            }
        }
        if let Some(velocity) = entity.inner_components.get("AngularVelocity") {
            if let Some(radians) = velocity.try_to_f64() {
                log::trace!("with AngularVelocity [{}]", radians);
                eb = eb.with(AngularVelocity { radians: radians as f32 });
            } else {
                log::error!("velocity must be a vec2");
            }
        }
        if entity.inner_components.contains_key("Player") {
            log::trace!("with Player");
            eb = eb.with(Player {});
            // Make it unique if it is player controlled.
            fg.r = 0.789;
            fg.g = 0.832;
            fg.b = 0.0;
        }

        if entity.inner_components.contains_key("SetVelocityIntent") {
            log::trace!("with SetVelocityIntent");
            eb = eb.with(SetVelocityIntent { x: 0f32, y: 0f32 });
        }

        if entity.inner_components.contains_key("StayInsideBounds") {
            log::trace!("with StayInsideBounds");
            eb = eb.with(StayInsideBounds{});
        }
        if let Some(counter) = entity.inner_components.get("Counter") {
            log::trace!("with Counter");
            let value = counter.try_to_i64().unwrap_or(0);
            eb = eb.with(Counter(value as i32))
        }
        if let Some(texture_override) = entity.inner_components.get("TextureOverride") {
            // As the texture must come in as a variant, it will be necessary to convert it to an object and then to a `Texture`
            if let Some(texture) = texture_override.try_to_object::<Texture>() {
                log::trace!("with TextureOverride");
                eb = eb.with(TextureOverride { texture: texture.clone() });
            } else {
                log::error!("could not convert `TextureOverride` to texture");
            }
        }

        // If it has a sprite attached and the sprite has a material, set the shader parameters.
        if let Some(node) = entity_owner.get_node("sprite") {
            let node = unsafe { node.assume_unique() };
            if let Ok(canvas_item) = node.try_cast::<CanvasItem>() {
                if let Some(material) = canvas_item.material() {
                    if let Ok(material) = material.try_cast::<ShaderMaterial>() {
                        eb = eb.with(CanvasItemShader {
                            material: material.clone()
                        });
                        log::trace!("with CanvasItemMaterial");
                    } else {
                        log::error!("material is not compatible with ShaderMaterial")
                    }
                    log::trace!("with ShaderParams");
                    eb = eb.with(ShaderParams::new(fg, bg));
                }
            } else {
                log::error!("child named `sprite` cannot be cast to `Sprite`")
            }
        } else {
            log::trace!("without Color")
        }
        let parent_entity = eb.build();
        let mut children = Vec::new();
        // Now it will be necessary to create one canvas_item per child
        for child in entity_owner.get_children().iter() {
            if let Some(child) = child.try_to_object::<Node2D>() {
                let child = unsafe { child.assume_safe() };
                children.push(self.create_entities_child(parent_entity, child))
            }
        }
        let mut relationships = self.world.write_storage::<TreeRelationship>();
        if let Some(relationship) = relationships.get_mut(parent_entity) {
            relationship.children = children;
        } else {
            log::trace!("gd_entity_hybrid does not have ")
        }
        Some(parent_entity)
    }

    /// Runs the world with the current dispatcher or prints an error message if it does not have a dispatcher at the time of running.
    /// Note: this should probably return a `Result` but that is outside the scope of this project.
    #[export]
    #[gdnative::profiled]
    pub fn run(&mut self, owner: TRef<Node>, delta: f64) {
        
        if let Some(dispatcher) = &mut self.dispatcher {
            {
                // First we'll increment the time resource
                let mut time = self.world.write_resource::<specs_engine::Time>();
                // Note, as the godot FFI only gives us f64, it is necessary to translate this for specs.
                time.delta = delta as f32;
                time.total += delta as f32;
            }
            // Run the world.
            dispatcher.run_now(&self.world);
            // Ensure that the world commits all of the changes from the systems.
            self.world.maintain();
            // If everything can be assured to not attempt to access this until after the update is complete, such as by resolving during IDLE,
            // you can use emit signal
            owner.emit_signal("update_completed", &[]);
            // Otherwise use call deferred.
            // unsafe { owner.call_deferred("emit_signal", &["update_completed", &[]]); }
            // Another option is to use an internal mutability pattern by wrapping `self.world` in a `RefCell` to allow an update without exposing `self` as a mutable reference.
            // That is currently out of scope for this sample.
        } else {
            log::error!("GDWorldHybrid does not have a dispatcher set, please define a dispatcher");
        }
    }

    // This method is not exported because it uses internal rust types
   
}