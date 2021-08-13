use specs::prelude::*;
use specs_derive::Component;

#[cfg(feature = "godot")]
use gdnative::prelude::ToVariant;
/// Defines the position of an entity in 2D space

#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Position {
    pub x: f32,
    pub y: f32,
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
/// Defines the velocity (change in position) of an entity in 2D space
#[derive(Debug, Component)]
#[cfg_attr(feature = "godot", derive(ToVariant))]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
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

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Rotation>();
    world.register::<Scale>();
    world.register::<Velocity>();
    world.register::<SetVelocityIntent>();
    world.register::<StayInsideBounds>();
    world.register::<Counter>();
}