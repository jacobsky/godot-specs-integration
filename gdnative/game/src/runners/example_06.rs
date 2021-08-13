//! This example builds on the previous message passing via queues by building an overengineered version of fizzbuzz

use gdnative::prelude::*;
use gd_specs::*;

/// This demonstrates use of the message queue via an overengineered fizzbuzz solution.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example06 {
    #[property]
    world_path: NodePath,
    world_instance: Option<Instance<GDWorld, Shared>>,
    time: f64,
}

#[methods]
impl Example06 {
    fn new(_: &Node) -> Self {
        Self {
            world_path: NodePath::from_str("world"),
            world_instance: None,
            time: 0.0,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        // Do the initialization here
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                use specs_engine::SideEffectQueue;
                // POI #1
                world.insert_resource(SideEffectQueue::new());
                world.set_dispatcher(gd_specs::example_6_dispatcher())
            }).expect("this should work correctly");
            self.world_instance = Some(gd_world.claim());
        }
    }

    #[export]
    pub fn _process(&mut self, _: &Node, delta: f64) {
        // Naive timer implementation
        self.time += delta;
            
        if self.time < 0.25 {
            return;
        }
        self.time -= 0.25;
        log::info!("running!");
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                world.run(owner, delta);
            }).expect("this should run successfully");
        }
        // First one in the scene should be an ECS World
        // for (i, child) in owner.get_children().iter().skip(1).enumerate() {
        //     if let Some(node) = child.try_to_object::<Node2D>() {
        //         let node = unsafe { node.assume_unique() };
        //         if let Ok(gd_entity) = Instance::<GDEntity, Unique>::try_from_base(node) {
        //             gd_entity.map(|gde, _| {
        //                 // log::info!("entity {}: {:#?}", i, gde.components);
        //             }).expect("this should work");
        //         }
        //     }
        // }
    }
}