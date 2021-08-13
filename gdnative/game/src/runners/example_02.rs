use gdnative::prelude::*;
use gdnative::api::Texture;
use gd_specs::*;
use specs_engine::SetVelocityIntent;

/// This example demonstrates handling player input within a bounding box based on Example01.
/// In addition this demonstrates tagging of multiple entities 
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Example02 {
    #[property]
    bounding_box: Rect2,
    #[property]
    move_speed: f32,
    #[property]
    world_path: NodePath,
    #[property]
    player_texture: Option<Ref<Texture>>,
    world_instance: Option<Instance<GDWorld, Shared>>,
}

#[methods]
impl Example02 {
    fn new(_: &Node) -> Self {
        Self {
            bounding_box: Rect2::new(
                Point2::new(0f32,0f32),
                Size2::new(100f32, 100f32),
            ),
            move_speed: 100.0,
            world_path: NodePath::from_str("world"),
            player_texture: None,
            world_instance: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        // Do the initialization here
        if let Some(gd_world) = unsafe { owner.get_node_as_instance::<GDWorld>(self.world_path.to_string().as_str()) } {
            gd_world.map_mut(|world, _| {
                world.insert_resource(specs_engine::BoundingBox::new(
                    self.bounding_box.origin.x,
                    self.bounding_box.origin.y,
                    self.bounding_box.size.width,
                    self.bounding_box.size.height,
                ));
                if let Some(texture) = self.player_texture.clone() {
                    world.insert_resource(texture.clone());
                }
                world.set_dispatcher(gd_specs::example_2_dispatcher());
            }).expect("this should work correctly");
            self.world_instance = Some(gd_world.claim());
        }
    }
    
    #[export]
    pub fn _physics_process(&mut self, _: &Node, delta: f64) {
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                let input = Input::godot_singleton();
                // Get the intent directly from the player.
                let intent_direction = 
                    Vector2::new(
                        (input.get_action_strength("ui_right") - input.get_action_strength("ui_left"))as f32,
                        (input.get_action_strength("ui_down") - input.get_action_strength("ui_up")) as f32
                    ) * self.move_speed;
                // POI #1
                let entities = world.get_entities_with::<Player>();
                for entity in entities {
                    world.set_component_for_entity(
                        entity,
                        SetVelocityIntent { x: intent_direction.x, y: intent_direction.y }
                    );
                }
                world.run(owner, delta);
            }).expect("this should run successfully");
        }
    }
}