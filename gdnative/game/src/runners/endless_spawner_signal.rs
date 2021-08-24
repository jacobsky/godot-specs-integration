use gdnative::prelude::*;
use gd_specs::*;

/// This example is intended to test the total performance of spawning, updating and despawning when using Signals with GDEntity and GDWorld 
#[derive(NativeClass)]
#[inherit(Node)]
pub struct EndlessSpawnerSignals {
    #[property(default = 2.0)]
    spawns_per_second: f64,
    #[property(default = 10000)]
    max_spawns: i32,
    #[property(default = 100)]
    despawn_rate: i32,
    #[property(default = true)]
    enable_velocity: bool,
    #[property(default = false)]
    enable_rotation: bool,
    #[property(default = false)]
    enable_scaling: bool,
    #[property]
    bounding_box: Rect2,
    #[property]
    world_path: NodePath,
    #[property]
    entity: Ref<PackedScene>,
    time: f64,
    spawn_timer: f64,
    // inverse of spawns_per_second for optimization reasons.
    seconds_per_spawns: f64,
    spawns: Vec<Ref<Node>>,
    world_instance: Option<Instance<GDWorld, Shared>>,
}

#[methods]
impl EndlessSpawnerSignals {
    fn new(_: &Node) -> Self {
        Self {
            spawns_per_second: 1.0,
            max_spawns: 10000,
            despawn_rate: 100,
            enable_velocity: true,
            enable_rotation: false,
            enable_scaling: false,
            bounding_box: Rect2::new(
                Point2::new(0f32,0f32),
                Size2::new(100f32, 100f32),
            ),
            world_path: NodePath::from_str("world"),
            entity: PackedScene::new().into_shared(),
            time: 0.0,
            spawn_timer: 0.0,
            seconds_per_spawns: 1.0,
            spawns: Vec::new(),
            world_instance: None,
        }
    }

    #[export]
    pub fn _ready(&mut self, owner: &Node) {
        log::info!("initializing world");
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
                world.set_dispatcher(gd_specs::signal_sync_dispatcher(self.enable_velocity, self.enable_rotation, self.enable_scaling));
            }).expect("this should work correctly");
            self.world_instance = Some(gd_world.claim());
        }
        self.seconds_per_spawns = 1.0 / self.spawns_per_second;
    }

    #[export]
    #[gdnative::profiled]
    pub fn _process(&mut self, owner: TRef<Node>, delta: f64) {
        self.time += delta;
        self.spawn_timer += delta;
        // Make the borrow checker happy
        if self.spawns.len() < self.max_spawns as usize &&
            self.world_instance.is_some() &&
            self.spawn_timer > self.seconds_per_spawns {
                let mut num_spawns = 0;
                while self.spawns.len() < self.max_spawns as usize && self.spawn_timer > self.seconds_per_spawns {
                    self.spawn_timer -= self.seconds_per_spawns;
                    // self.spawn_entity(owner);
                    num_spawns += 1;
                }
                self.spawn_entities(owner, num_spawns);
                log::debug!("total spawns: {}", self.spawns.len());
        }
        if let Some(instance) = &self.world_instance {
            let instance = unsafe { instance.assume_safe() };
            instance.map_mut(|world, owner| {
                world.run(owner, delta);
            }).expect("this should run successfully");
        }
    }

    #[export]
    #[gdnative::profiled]
    pub fn set_gd_entity_properties(&self, _: TRef<Node>, node: Ref<Node>) {
        let pos_x = self.bounding_box.width() / 2.0;
        let pos_y = 1.0;
        let vel_x = (self.time * 10.0 % 40.0) - 20.0;
        let vel_y = 300.0;
        let ang_vel = 1.0;

        let instance = unsafe { node.assume_unique() };
        let instance = instance.try_cast::<Node2D>().expect("root node type should be correct");
        let instance = unsafe { instance.into_shared().assume_safe() };
        let gd_entity = instance.cast_instance::<GDEntity>().expect("this should work");
        gd_entity.map_mut(|e, o| {

            e.set_component("Velocity", Vector2::new(vel_x as f32, vel_y));
            e.set_component("AngularVelocity", ang_vel.to_variant());
            e.world_path = NodePath::from_str(format!("../../{}", self.world_path.to_string()).as_str());
            // e.components; 
            o.set_position(Vector2::new(pos_x, pos_y));
        }).expect("this should work");
    }
    #[export]
    #[gdnative::profiled]
    pub fn instance_node(&self, _: TRef<Node>) -> Ref<Node> {
        let scene = unsafe { self.entity.assume_safe() };
        scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED).expect("should be able to instance scene")
    }
    #[export]
    #[gdnative::profiled]
    pub fn add_children_to_self(&self, owner: TRef<Node>, array: VariantArray) {
        const ENTITIES_PER_CHILD : i32 = 512;
        // This is the total number of spawns after `array` is added to the list
        let current_spawns = self.spawns.len() as i32;
        let total_spawns = current_spawns + array.len();
        let total_children_required = total_spawns / ENTITIES_PER_CHILD;
        while (owner.get_child_count() as i32) < total_children_required + 1 {
            let node = Node::new();
            owner.add_child(node, true);
        }
        let mut roots = Vec::new();
        for idx in 0..owner.get_child_count() as i32 {
            let root = owner.get_child(idx as i64).expect("this should exist");
            let root = unsafe { root.assume_safe() };
            roots.push(root);
        }

        for (i, child) in array.iter().enumerate() {
            let idx = (current_spawns as usize + i) / ENTITIES_PER_CHILD as usize;
            let instance = child.try_to_object::<Node>().expect("this should work");
            roots[idx].add_child(instance, true);
        }
    }
    #[export]
    #[gdnative::profiled]
    pub fn spawn_entities(&mut self, owner: TRef<Node>, num_spawns: i32) {
        log::trace!("spawn_entity");
        let mut spawned_nodes = Vec::new();
        let variant_array = VariantArray::new();

        for _ in 0 .. num_spawns {
            let node = self.instance_node(owner);
            let instance = unsafe { node.assume_unique() };
            let instance = instance.try_cast::<Node2D>().expect("root node type should be correct");
            self.set_gd_entity_properties(owner, node);
            variant_array.push(instance.owned_to_variant());
            spawned_nodes.push(node);
        }
        self.add_children_to_self(owner, variant_array.into_shared());
        for node in spawned_nodes {
            self.spawns.push(node);
        }
    }
    #[export]
    #[gdnative::profiled]
    pub fn spawn_entity(&mut self, owner: TRef<Node>) {
        log::trace!("spawn_entity");
        let scene = unsafe { self.entity.assume_safe() };

        let node = scene.instance(PackedScene::GEN_EDIT_STATE_DISABLED).expect("should be able to instance scene");
    
        let instance = unsafe { node.assume_unique() };
    
        let instance = instance.try_cast::<Node2D>().expect("root node type should be correct");
        let instance = unsafe { instance.into_shared().assume_safe() };
        
        let pos_x = self.bounding_box.width() / 2.0;
        let pos_y = 1.0;
        let vel_x = (self.time * 10.0 % 40.0) - 20.0;
        let vel_y = 300.0;
        let ang_vel = 1.0;

        let gd_entity = instance.cast_instance::<GDEntity>().expect("this should work");
        gd_entity.map_mut(|e, o| {

            e.set_component("Velocity", Vector2::new(vel_x as f32, vel_y));
            e.set_component("AngularVelocity", ang_vel.to_variant());
            e.world_path = NodePath::from_str(format!("../{}", self.world_path.to_string()).as_str());
            // e.components; 
            o.set_position(Vector2::new(pos_x, pos_y));
        }).expect("this should work");
        owner.add_child(instance, true);
        self.spawns.push(node.clone());
    }
}