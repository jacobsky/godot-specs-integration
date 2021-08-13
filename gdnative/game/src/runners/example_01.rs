use gdnative::prelude::*;
use gd_specs::*;

/// This example demonstrates entities that are moving within a bounded box
/// Each entity spawns at their position in the editor with the velocity specified in their components dictionary
/// If an entity has `StayInsideBounds` inside the defined component, they will bounce when they reach the bounding box  
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example01 {
    #[property]
    change_colors: bool,
    #[property]
    bounding_box: Rect2,
    #[property]
    world_path: NodePath,
    world_instance: Option<Instance<GDWorld, Shared>>,
}

#[methods]
impl Example01 {
    fn new(_: &Node) -> Self {
        Self {
            change_colors: false,
            bounding_box: Rect2::new(
                Point2::new(0f32,0f32),
                Size2::new(100f32, 100f32),
            ),
            world_path: NodePath::from_str("world"),
            world_instance: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        // Do the initialization here
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                // Insert the necessary resources to the world to make this run
                world.insert_resource(specs_engine::BoundingBox::new(
                    self.bounding_box.origin.x,
                    self.bounding_box.origin.y,
                    self.bounding_box.size.width,
                    self.bounding_box.size.height,
                ));
                // Add a dispatcher that has all of the relevant systems
                world.set_dispatcher(gd_specs::example_1_dispatcher(self.change_colors))
            }).expect("this should work correctly");
            self.world_instance = Some(gd_world.claim());
        }
    }

    #[export]
    pub fn _physics_process(&mut self, _: &Node, delta: f64) {
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                world.run(owner, delta);
            }).expect("this should run successfully");
        }
    }
}