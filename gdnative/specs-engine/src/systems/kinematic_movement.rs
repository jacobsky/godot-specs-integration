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
        (&mut positions, &velocities).par_join().for_each(|(position, velocity)| {
               position.x += velocity.x * time.delta;
               position.y += velocity.y * time.delta;
           }
        );
    }
}

pub struct UpdateRotationSystem {}
impl <'a> System <'a> for UpdateRotationSystem {
    type SystemData = (
        ReadExpect<'a, Time>,
        ReadStorage<'a, AngularVelocity>,
        WriteStorage<'a, Rotation>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (time, angular_velocities, mut rotations) = data;
        (&mut rotations, &angular_velocities)
            .par_join()
            .for_each(|(rotation, vel)|{
                rotation.radians += vel.radians + time.delta;
            });
    }
}
