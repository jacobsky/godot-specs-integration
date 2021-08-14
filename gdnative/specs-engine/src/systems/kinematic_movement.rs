//! This module contains the systems used for kinematic movement. These should approximate physics behaviors without
//! "realistic" physics processing
use specs::prelude::*;
use crate::components::*;
use crate::resources::*;
pub struct UpdatePositionSystem {}
// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdatePositionSystem {
    type SystemData = (
        ReadExpect<'a, Time>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (time, velocities, mut positions) = data;
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x * time.delta;
            position.y += velocity.y * time.delta;
        }
    }
}

pub struct UpdateLocalRotationSystem {}
// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdateLocalRotationSystem {
    type SystemData = (
        ReadExpect<'a, Time>,
        ReadStorage<'a, TreeRelationship>,
        ReadStorage<'a, AngularVelocity>,
        WriteStorage<'a, Rotation>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (time, relationships, angular_velocities, mut rotations) = data;
        for (relationship, angular_velocity) in (&relationships, &angular_velocities).join() {
            for entity in relationship.children.iter() {
                if let Some(rotation) = rotations.get_mut(*entity) {
                    rotation.radians += angular_velocity.radians * time.delta;
                }
            }
        }
    }
}
pub struct UpdateLocalScaleSystem {}
// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdateLocalScaleSystem {
    type SystemData = (
        ReadExpect<'a, Time>,
        ReadStorage<'a, TreeRelationship>,
        WriteStorage<'a, Scale>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (time, relationships,  mut scales) = data;
        for relationship in (&relationships).join() {
            for entity in relationship.children.iter() {
                if let Some(scale) = scales.get_mut(*entity) {
                    scale.x = time.total.sin() + 0.5;
                    scale.y = time.total.cos() + 0.5;
                }
            }
        }
    }
}
