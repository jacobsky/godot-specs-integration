#[cfg(feature = "godot")]
use gdnative::prelude::*;

use specs::prelude::*;
use specs_derive::Component;

#[cfg(feature = "godot")]
mod godot_ext;
#[cfg(feature = "godot")]
pub use godot_ext::*;
/// Defines the position of an entity in 2D space

#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Defines the velocity (change in position) of an entity in 2D space
#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct AngularVelocity {
    pub radians: f32,
}


#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Rotation {
    pub radians: f32,
}

#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Scale {
    pub x: f32,
    pub y: f32
}


/// Indicates that an entity wants to instantaneously change it's velocity to the current value
#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct SetVelocityIntent {
    pub x: f32,
    pub y: f32,
}

/// This identifies which entities must respect the bounding box when moving
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct StayInsideBounds;

#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Counter(pub i32);

/// This represents a "tree-like" relationship between entities. The current entity may index a parent and a list of children
/// This is used to mimic the scene-tree relationship that allows for objects to rotate in place.
#[derive(Debug, Default, Component)]
pub struct TreeRelationship {
    // If there is no parent, this should be treated as the root
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
}

/// This represents a "tree-like" relationship between entities. The current entity may index a parent and a list of children
/// This is used to mimic the scene-tree relationship that allows for objects to rotate in place.
#[derive(Debug, Default, Component)]
pub struct StringContainer {
    pub message: String,
}

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Scale>();
    world.register::<Velocity>();
    world.register::<AngularVelocity>();
    world.register::<SetVelocityIntent>();
    world.register::<StayInsideBounds>();
    world.register::<Counter>();
    world.register::<TreeRelationship>();
    world.register::<StringContainer>();
}