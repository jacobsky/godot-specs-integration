use specs::prelude::*;
use rapier2d::prelude::*;
use crate::rapier::*;

pub struct UpdateKinematicPositionSystem {

}

impl <'a> System <'a> for KinematicMovementSystem {
    type SystemData = (
        ReadExpect<'a, RapierPhysicsResource>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, data: Self::SystemData) {

    }
}