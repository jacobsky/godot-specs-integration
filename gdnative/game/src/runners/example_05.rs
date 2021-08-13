//! This example builds on the previous message passing via queues by building an overengineered version of fizzbuzz

use gdnative::prelude::*;
use gd_specs::*;
use specs_engine::{BuzzQueue, FizzBuzzQueue,  FizzQueue, FizzbuzzInputMessage, FizzbuzzOutputMessage};

/// This demonstrates use of the message queue via an overengineered fizzbuzz solution.
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example05 {
    #[property]
    world_path: NodePath,
    world_instance: Option<Instance<GDWorld, Shared>>,
    time: f64,
    count: i32,
}

#[methods]
impl Example05 {
    fn new(_: &Node) -> Self {
        Self {
            world_path: NodePath::from_str("world"),
            world_instance: None,
            time: 0.0,
            count: 0,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        // Do the initialization here
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                use specs_engine::WorldMsgQueue;
                // POI #1
                world.insert_resource(WorldMsgQueue::<FizzbuzzInputMessage>::new());
                world.insert_resource(FizzQueue::new());
                world.insert_resource(BuzzQueue::new());
                world.insert_resource(FizzBuzzQueue::new());
                world.insert_resource(WorldMsgQueue::<FizzbuzzOutputMessage>::new());
                world.set_dispatcher(gd_specs::example_5_dispatcher())
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
        
        self.count += 1;
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                world.enqueue_message(FizzbuzzInputMessage(self.count));
                // POI #2 See what happens when you try to queue up one of the following
                // world.enqueue_message(specs_engine::FizzMessage{});
                // world.enqueue_message(specs_engine::BuzzMessage{});
                // world.enqueue_message(specs_engine::FizzBuzzMessage{});
                // let _msg = world.pop_message::<specs_engine::FizzBuzzMessage>();
                world.run(owner, delta);
                while let Some(output) = world.pop_message::<FizzbuzzOutputMessage>() {
                    log::info!("{}", output.0);
                }
            }).expect("this should run successfully");
        }
    }
}