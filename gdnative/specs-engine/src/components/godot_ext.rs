
use gdnative::core_types::{Variant, Vector2};
use specs::prelude::*;

use super::*;

impl From<&Position> for gdnative::core_types::Vector2 {
    fn from(pos: &Position) -> Self {
        Vector2::new(pos.x, pos.y)
    }
}
impl From<&gdnative::core_types::Vector2> for Position {
    fn from(vec: &gdnative::core_types::Vector2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl From<&Scale> for gdnative::core_types::Vector2 {
    fn from(scale: &Scale) -> Self {
        Vector2::new(scale.x, scale.y)
    }
}

impl From<&gdnative::core_types::Vector2> for Scale {
    fn from(vec: &gdnative::core_types::Vector2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl std::convert::From<&Velocity> for gdnative::core_types::Vector2 {
    fn from(velocity: &Velocity) -> Self {
        Vector2::new(velocity.x, velocity.y)
    }
}

impl std::convert::From<&gdnative::core_types::Vector2> for Velocity {
    fn from(vec: &gdnative::core_types::Vector2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl std::convert::TryFrom<&Variant> for Velocity {
    type Error = &'static str;
    fn try_from(v: &Variant) ->  Result<Self, Self::Error>  {
        if let Some(value) = v.try_to_vector2() {
            Ok(Self::from(&value))
        } else {
            Err("Variant cannot be converted to f64")
        }
    }
}


impl std::convert::From<f64> for AngularVelocity {
    fn from(v: f64) ->  Self  {
        Self::from(v as f32)
    }
}

impl std::convert::From<f32> for AngularVelocity {
    fn from(v: f32) ->  Self  {
        Self { radians: v }
    }
}


impl std::convert::TryFrom<&Variant> for AngularVelocity {
    type Error = &'static str;
    fn try_from(v: &Variant) ->  Result<Self, Self::Error>  {
        if let Some(value) = v.try_to_f64() {
            Ok(Self::from(value))
        } else {
            Err("Variant cannot be converted to f64")
        }
    }
}

impl std::convert::From<&gdnative::core_types::Vector2> for SetVelocityIntent {
    fn from(vec: &gdnative::core_types::Vector2) -> Self {
        Self { x: vec.x, y: vec.y }
    }
}

impl std::convert::TryFrom<&Variant> for SetVelocityIntent {
    type Error = &'static str;
    fn try_from(v: &Variant) ->  Result<Self, Self::Error>  {
        if let Some(value) = v.try_to_vector2() {
            Ok(Self::from(&value))
        } else {
            Err("Variant cannot be converted to f64")
        }
    }
}