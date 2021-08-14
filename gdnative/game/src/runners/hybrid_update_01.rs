use gdnative::prelude::*;
use gd_specs::*;

/// This example demonstrates entities that are moving within a bounded box
/// Each entity spawns at their position in the editor with the velocity specified in their components dictionary
/// If an entity has `StayInsideBounds` inside the defined component, they will bounce when they reach the bounding box  
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HybridUpdate01 {
    #[property(default = true)]
    enable_velocity: bool,
    #[property]
    enable_rotation: bool,
    #[property]
    enable_scaling: bool,
    #[property]
    bounding_box: Rect2,
    #[property]
    world_path: NodePath,
    world_instance: Option<Instance<GDWorldHybrid, Shared>>,
}

#[methods]
impl HybridUpdate01 {
    fn new(_: &Node) -> Self {
        Self {
            enable_velocity: true,
            enable_rotation: false,
            enable_scaling: false,
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
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorldHybrid>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                // Insert the necessary resources to the world to make this run
                world.insert_resource(specs_engine::BoundingBox::new(
                    self.bounding_box.origin.x,
                    self.bounding_box.origin.y,
                    self.bounding_box.size.width,
                    self.bounding_box.size.height,
                ));
                // Add a dispatcher that has all of the relevant systems
                world.set_dispatcher(gd_specs::hybrid_sync_dispatcher(self.enable_velocity, self.enable_rotation, self.enable_scaling));
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
    // Add an auto spawn code to validate spawning and deletion
    // #[export]
    // pub fn on_click_spawn(&self, owner: TRef<Node>) {
    //     let scene = unsafe { self.entity.assume_safe() };

    //     let instance = scene
    //         .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
    //         .expect("should be able to instance scene");
    
    //     let instance = unsafe { instance.assume_unique() };
    
    //     let instance = instance
    //         .try_cast::<Node2D>()
    //         .expect("root node type should be correct");
    //     let instance = unsafe { instance.into_shared().assume_safe() };
        
    //     let current_time = if let Some(world) = &self.world_instance {
    //         let world = unsafe { world.assume_safe() };
    //         world.map(|w, _|{
    //             if let Some(time) = w.clone_resource::<Time>() {
    //                 time.total
    //             } else {
    //                 0.0
    //             }
    //         }).expect("this should work.")
    //     } else { 0.0 };

    //     // It's not really random, but it does the job (unless you are reading this)
    //     let pos_x = (current_time * 10.0) % self.bounding_box.width();
    //     let pos_y = (current_time * 100.0) % self.bounding_box.width();
    //     const MAX_VEL: f32 = 100.0;
    //     const HALF_MAX_VEL: f32 = MAX_VEL / 2.0;
    //     let vel_x = (current_time.sin() * MAX_VEL) - HALF_MAX_VEL;
    //     let vel_y = (current_time.cos() * MAX_VEL) - HALF_MAX_VEL;

    //     let gd_entity = instance.cast_instance::<GDEntity>().expect("this should work");
    //     gd_entity.map_mut(|e, o| {

    //         e.set_component("Velocity", Vector2::new(vel_x, vel_y));
    //         e.world_path = NodePath::from_str(format!("../{}", self.world_path.to_string()).as_str());
    //         // e.components; 
    //         o.set_global_position(Vector2::new(pos_x, pos_y));
    //     }).expect("this should work");
    //     owner.add_child(instance, true);
    // }
}