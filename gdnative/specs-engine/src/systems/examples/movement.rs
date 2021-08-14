use specs::prelude::*;
use crate::components::*;
use crate::resources::*;
pub struct SetVelocitySystem {}

// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for SetVelocitySystem {
    type SystemData = (
        WriteStorage<'a, SetVelocityIntent>,
        WriteStorage<'a, Velocity>);
    fn run(&mut self, data: Self::SystemData) {
        let (set_velocity_intents, mut velocities) = data;
        for (velocity, intent) in (&mut velocities, &set_velocity_intents).join() {
            velocity.x = intent.x;
            velocity.y = intent.y;
        }
    }
}

/// Changes the velocity when an object would otherwise leave the bounding box
pub struct ChangeVelocityAtBounds {}

// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for ChangeVelocityAtBounds {
    type SystemData = (
        ReadExpect<'a, BoundingBox>,
        ReadExpect<'a, Time>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Position>,);
    fn run(&mut self, data: Self::SystemData) {
        let (bounding_box, time, mut velocities, positions) = data;
        let x_min = bounding_box.x;
        let y_min = bounding_box.y;
        let x_max = bounding_box.x + bounding_box.width;
        let y_max = bounding_box.y + bounding_box.height;
        for (velocity, position) in (&mut velocities, &positions).join() {
            let future_x = position.x + velocity.x * time.delta;
            let future_y = position.y + velocity.y * time.delta;
            // If the future does not reside in the bounding box
            if future_x < x_min || future_x > x_max {
                velocity.x = -velocity.x;
            }
            if future_y < y_min || future_y > y_max {
                velocity.y = -velocity.y;
            }
        }
    }
}

/// Simple bounding box starting at the coorindate `x`, `y` and extending out `width` and `height`. `width` and `height` must be non-zero.
pub struct BoundingBox { x: f32, y: f32, width: f32, height: f32 }
impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        assert!(width > 0f32);
        assert!(height > 0f32);
        Self { x, y, width, height }
    }
}

pub struct UpdatePositionWithBoundsSystem {}
// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdatePositionWithBoundsSystem {
    type SystemData = (
        ReadExpect<'a, BoundingBox>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (bounding_box, velocities, mut positions) = data;
        
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x = f32::clamp(position.x + velocity.x, bounding_box.x, bounding_box.x + bounding_box.width);
            position.y = f32::clamp(position.y + velocity.y, bounding_box.y, bounding_box.y + bounding_box.height);
        }
    }
}

pub struct UpdateBoundedPositionSystem {}

// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdateBoundedPositionSystem {
    type SystemData = (
        ReadExpect<'a, BoundingBox>,
        ReadStorage<'a, StayInsideBounds>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (bounding_box, stay_inside, velocities, mut positions) = data;
        
        for (position, velocity, _) in (&mut positions, &velocities, &stay_inside).join() {
            position.x = f32::clamp(position.x + velocity.x, bounding_box.x, bounding_box.x + bounding_box.width);
            position.y = f32::clamp(position.y+ velocity.y, bounding_box.y, bounding_box.y + bounding_box.height);
        }
    }
}

pub struct UpdateUnboundedPositionSystem {}

// Note: If you have a more physicy game, you may wish to base Velocity off of Acceleration.
impl <'a> System <'a> for UpdateUnboundedPositionSystem {
    type SystemData = (
        ReadStorage<'a, StayInsideBounds>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (stay_inside, velocities, mut positions) = data;
        
        for (position, velocity, _) in (&mut positions, &velocities, !&stay_inside).join() {
            position.x = velocity.x;
            position.y = velocity.y;
        }
    }
}