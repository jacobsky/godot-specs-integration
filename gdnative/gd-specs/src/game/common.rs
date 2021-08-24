use gdnative::prelude::*;
use specs::prelude::*;
use specs_engine::*;
use crate::ComponentInfo;
use crate::components::*;

use std::collections::HashMap;

/// Creates an entity in the Ecs World from a given `ComponentInfo` it consumes the component info via moves.
pub struct CreateEntityFromComponentInfo{}


impl WorldCommand for CreateEntityFromComponentInfo {
    type Args = ComponentInfo;
    type Output = Option<Entity>;
    fn execute(world: &mut World, info: Self::Args) -> Self::Output {
        let eb = world.create_entity()
            .maybe_with(info.position)
            .maybe_with(info.rotation)
            .maybe_with(info.scale)
            .maybe_with(info.velocity)
            .maybe_with(info.angular_velocity)
            .maybe_with(info.set_velocity_intent);
        Some(eb.build())
    }
}

