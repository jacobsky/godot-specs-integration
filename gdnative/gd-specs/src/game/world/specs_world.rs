use specs::prelude::*;
use specs_engine::*;



// pub struct WorldCommand<A, O>(Box<dyn FnMut(&mut World, A) -> O>);
// pub struct QueryClosure<A, O>(Box<dyn Fn(&World, A) -> O>);

/// This is a lightweight wrapper around the world class with some useful helper functions for use by the game runner.
/// This directly wraps the SpecsWorld
/// The goal for this is that the GameWorld
pub struct SpecsWorld {
    pub (crate) world: World,
}



impl SpecsWorld {
    fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
    /// Allows for passing a closure that can register
    pub fn register_components<CR: ComponentRegistry>(&mut self) {
        CR::register(&mut self.world)
    }
    pub fn insert_resource<R>(&mut self, resource: R) 
        where R:
        std::any::Any + Send + Sync
    {
        self.world.insert::<R>(resource);
    }


    /// Clones the resource and returns a copy. This is useful if you need to inspect an internal resource but don't want to do a query
    pub fn clone_resource<R>(&self) -> Option<R> 
        where R: Clone + Copy + std::any::Any + Send + Sync {
        self.world.try_fetch::<R>().map(|fetch| *fetch.to_owned())
    }
    
    /// Enqueues a message onto a WorldMsgQueue<R> if a queue exists.
    pub fn enqueue_message<R>(&self, message: R) 
        where R: std::any::Any + Send + Sync {
        if let Some(resource) = self.world.try_fetch::<specs_engine::WorldMsgQueue<R>>() {
            resource.push(message);
        } else {
            log::error!("queue does not exist for {:?}", std::any::type_name::<R>());
        }
    }

    /// Pops a message off of a WorldMsgQueue<R> message queue.
    pub fn pop_message<R>(&self) -> Option<R> 
        where R: std::any::Any + Send + Sync {

        if let Some(resource) = self.world.try_fetch::<specs_engine::WorldMsgQueue<R>>() {
            resource.pop()
        } else {
            log::error!("queue does not exist for {:?}", std::any::type_name::<R>());
            None
        }
    }

    pub fn set_component_for_entity<C: Component>(&mut self, entity: Entity, component: C) {
        let mut storage = self.world.write_storage::<C>();
        if let Some(c) = storage.get_mut(entity) {
            *c = component;
        } else {
            log::error!("entity [{:?}] does not have component {}", entity, std::any::type_name::<C>());
        }
    }

    pub fn query<Q: WorldQuery>(&self, args: Q::Args) -> Q::Output {
        Q::query(&self.world, args)
    }
    
    pub fn command<C: WorldCommand>(&mut self, args: C::Args) -> C::Output {
        C::execute(&mut self.world, args)
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

    /// Runs the world with the current dispatcher or prints an error message if it does not have a dispatcher at the time of running.
    /// Note: this should probably return a `Result` but that is outside the scope of this project.
    pub fn update_time(&mut self, delta: f64) {
        let mut time = self.world.write_resource::<specs_engine::Time>();
        // Note, as the godot FFI only gives us f64, it is necessary to translate this for specs.
        time.delta = delta as f32;
        time.total += delta as f32;
    }
    // #[export]
    // #[gdnative::profiled]
    pub fn run_with<'a, 'b>(&mut self, dispatcher: &mut Dispatcher<'a, 'b>, delta: f64) {
            // Run the world.
            dispatcher.run_now(&self.world);
            // Ensure that the world commits all of the changes from the systems.
            self.world.maintain();
            // If everything can be assured to not attempt to access this until after the update is complete, such as by resolving during IDLE,
            // you can use emit signal
            // owner.emit_signal("update_completed", &[]);
    }
}

mod test {
    use specs_engine::{WorldCommand, WorldQuery};

    use crate::SpecsWorld;
    use specs::prelude::*;
    use specs_engine::Position;

    #[test]
    pub fn test_query_struct() {
        struct GetPosition{}
        impl WorldQuery for GetPosition {
            type Args = Entity;
            type Output =  (f32, f32);
            fn query(world: &World, entity: Self::Args) -> Self::Output {
                let mut position = world.write_storage::<Position>();
                let pos = position.get_mut(entity).expect("this should work");
                (pos.x, pos.y)
            }
        }
        let mut world = SpecsWorld::new();
        world.world.register::<specs_engine::Position>();
        let entity = world.world.create_entity().with(Position { x: 1123.0, y: -312.0 }).build();
        let (x, y) = world.query::<GetPosition>(entity);
        assert_eq!(x, 1123.0);
        assert_eq!(y, -312.0); 
    }

    #[test]
    pub fn test_command_struct() {
        struct ChangePosition{}
        impl WorldCommand for ChangePosition {
            type Args = (Entity, (f32, f32));
            type Output = ();
            fn execute(world: &mut World, args: Self::Args) -> Self::Output {
                let (entity, (x, y)) = args;
                let mut position = world.write_storage::<Position>();
                let pos = position.get_mut(entity).expect("this should work");
                pos.x = x;
                pos.y = y;
                ()
            }
        }
        let mut world = SpecsWorld::new();
        world.world.register::<Position>();
        let entity = world.world.create_entity().with(Position { x: 1123.0, y: -312.0 }).build();

        world.command::<ChangePosition>((entity, (0.0, 0.0)));
        let positions = world.world.read_storage::<Position>();
        let position = positions.get(entity);
        assert!(position.is_some());
        assert_eq!(position.unwrap().x, 0.0);
        assert_eq!(position.unwrap().y, 0.0);
    }
}