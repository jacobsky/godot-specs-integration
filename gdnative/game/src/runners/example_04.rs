//! This example builds on the previous message passing via queues by building an overengineered version of fizzbuzz

use gdnative::prelude::*;
use gd_specs::*;
use specs_engine::{StringMessage};

/// This demonstrates use of the message queue via an overengineered fizzbuzz solution.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example04 {
    #[property]
    message: GodotString,
    #[property]
    world_path: NodePath,
    world_instance: Option<Instance<GDWorld, Shared>>,
    time: f64,
    times_run: i32,
    times_clicked: i32
}

#[methods]
impl Example04 {
    fn new(_: &Node) -> Self {
        Self {
            message: GodotString::from_str("Hello World!"),
            world_path: NodePath::from_str("world"),
            world_instance: None,
            time: 0.0,
            times_run: 0,
            times_clicked: 0
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        // Do the initialization here
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                use specs_engine::WorldMsgQueue;
                world.insert_resource(WorldMsgQueue::<StringMessage>::new());
                world.set_dispatcher(gd_specs::example_4_dispatcher());
            }).expect("this should work correctly");
            self.world_instance = Some(gd_world.claim());
        }
    }

    #[export]
    pub fn _process(&mut self, _: &Node, delta: f64) {
        // Naive timer implementation
        self.time += delta;
            
        if self.time < 1.0 {
            return;
        }
        self.time -= 1.0;
        
        self.times_run += 1;
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                // POI #1
                world.enqueue_message(
                    StringMessage {
                        message: format!("{} #{}", self.message.to_string(), self.times_run)
                    }
                );
                world.run(owner, delta);
            }).expect("this should run successfully");
        }
    }


    // POI #2
    /// This allows for buttons to insert events into the `GDWorld`
    #[export]
    pub fn on_click(&mut self, _: &Node) {
        if let Some(instance) = &self.world_instance {
            self.times_clicked += 1;
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, _| {
                world.enqueue_message(
                    StringMessage {
                        message: format!("button clicked {} times", self.times_clicked)
                    }
                );
            }).expect("this should run successfully");
        }
    }
}