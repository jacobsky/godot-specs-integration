use gdnative::prelude::*;
use specs::prelude::*;
use specs_engine::{Position, Scale, Rotation, Velocity, AngularVelocity, SetVelocityIntent};
use std::collections::HashMap;


mod node2d_entity;
mod sprite_entity;

pub use node2d_entity::*;
pub use sprite_entity::*;


/// This is a Variantable Entity reference that can be safely passed into GDScript for reference by other functions.
#[derive(NativeClass)]
#[inherit(Reference)]
#[no_constructor]
pub struct EntityRef {
    entity: Entity
}

#[methods]
impl EntityRef {}

pub struct ComponentInfo {
    pub (crate) position: Option<Position>,
    pub (crate) scale: Option<Scale>,
    pub (crate) rotation: Option<Rotation>,
    pub (crate) velocity: Option<Velocity>,
    pub (crate) angular_velocity: Option<AngularVelocity>,
    pub (crate) set_velocity_intent: Option<SetVelocityIntent>,
}

impl ComponentInfo {
    pub fn empty () -> Self {
        Self {
            // This should be set by the Node data directly
            position: None,
            // This should be set by the Node data directly
            scale: None,
            // This should be set by the Node data directly
            rotation: None,
            velocity: None,
            angular_velocity: None,
            set_velocity_intent: None,
        }
    }
}

impl From<&HashMap<String, Variant>> for ComponentInfo {
    fn from(hashmap: &HashMap<String, Variant>) -> Self {
        use std::convert::TryFrom;
        // This macro gets rid of a lot of the boiler-plate necessary to pull out the components.
        macro_rules! get_component {
            ($component_type:ty) => {
                hashmap.get(stringify!($component_type)).map(|variant| {
                    if let Ok(component) = <$component_type>::try_from(variant) {
                        Some(component)
                    } else {
                        None
                    }
                }).flatten();
            }
        }
        
        Self {
            // This should be set by the Node data directly
            position: None,
            // This should be set by the Node data directly
            scale: None,
            // This should be set by the Node data directly
            rotation: None,
            velocity: get_component!(Velocity),
            angular_velocity: get_component!(AngularVelocity),
            set_velocity_intent: get_component!(SetVelocityIntent),
        }
    }
}