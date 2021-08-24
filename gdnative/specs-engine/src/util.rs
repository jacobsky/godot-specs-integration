use specs::prelude::*;

/// This trait can be defined to register only the components that a world requires.
pub trait ComponentRegistry {
    // (Box<dyn FnMut(&mut World)>)
    fn register(world: &mut World);
}

/// WorldCommand is used to help implement generic commands
/// `args` are of type `A`
/// ouput is of type `O`
pub trait WorldCommand {
    type Args;
    type Output;
    fn execute(world: &mut World, args: Self::Args) -> Self::Output;
}
/// WorldQuery allows for non-mutable access to be defined as QueryObjects that can be dynamically passed to any specs Engine.
pub trait WorldQuery {
    type Args;
    type Output;
    fn query(world: &World, args: Self::Args) -> Self::Output;
}

mod test {
    use super::*;
    use specs::prelude::*;
    use crate::Position;
    #[test]
    pub fn test_query_struct() {
        struct GetPosition{}
        impl WorldQuery for GetPosition {
            type Args = Entity;
            type Output = (f32, f32);
            fn query(world: &World, entity: Self::Args) -> Self::Output {
                let mut position = world.write_storage::<Position>();
                let pos = position.get_mut(entity).expect("this should work");
                (pos.x, pos.y)
            }
        }
        let mut world = World::new();
        world.register::<crate::Position>();
        let entity = world.create_entity().with(Position { x: 1123.0, y: -312.0 }).build();
        let (x, y) = GetPosition::query(&world, entity);
        assert_eq!(x, 1123.0);
        assert_eq!(y, -312.0); 
    }

    #[test]
    pub fn test_command_struct() {
        struct ChangePosition{}
        impl WorldCommand for ChangePosition {
            type Args = (Entity, (f32, f32));
            type Output = (f32, f32);
            fn execute(world: &mut World, args: Self::Args) -> Self::Output {
                let (entity, (x, y)) = args;
                let mut position = world.write_storage::<Position>();
                let pos = position.get_mut(entity).expect("this should work");
                pos.x = x;
                pos.y = y;
                (pos.x, pos.y)
            }
        }
        let mut world = World::new();
        world.register::<Position>();
        let entity = world.create_entity().with(Position { x: 1123.0, y: -312.0 }).build();

        ChangePosition::execute(&mut world, (entity, (0.0, 0.0)));
        let positions = world.read_storage::<Position>();
        let position = positions.get(entity);
        assert!(position.is_some());
        assert_eq!(position.unwrap().x, 0.0);
        assert_eq!(position.unwrap().y, 0.0);
    }
}