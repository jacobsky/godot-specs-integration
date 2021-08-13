use specs::prelude::*;
use specs_engine::{Time, Counter};
use crate::ShaderParams;

/// This system uses system time and converts the FG shader param.
/// Note: This would be much faster to implement in a shader, but this is made to demonstrate how you can easily feed
/// shader parameters from an ECS into Godot directly
pub struct RainbowColorSystem {}

impl <'a> System <'a> for RainbowColorSystem {
    type SystemData = (ReadExpect<'a, Time>, WriteStorage<'a, ShaderParams>);
    fn run(&mut self, data: Self::SystemData) {

        let (time, mut shader_params) = data;
        for params in (&mut shader_params).join() {
            // These are pretty arbitrary, they just allow the color to change while still being visible on a black background
            params.fg.r = (time.total * 0.5).sin().max(0.2);
            params.fg.g = ((time.total * 2.0) + 0.5).sin().max(0.2);
            params.fg.b = (time.total + 1.0).sin().max(0.2);
        }
    }
}

pub struct ColorBasedOnCountSystem {}

impl <'a> System <'a> for ColorBasedOnCountSystem {
    type SystemData = (ReadStorage<'a, Counter>, WriteStorage<'a, ShaderParams>);
    fn run(&mut self, data: Self::SystemData) {

        let (counters, mut shader_params) = data;
        for (counter, params) in (&counters, &mut shader_params).join() {
            // These are pretty arbitrary, they just allow the color to change while still being visible on a black background
            let value = counter.0;
            params.fg.r = (value as f32 * 0.5).sin().max(0.2);
            params.fg.g = (value as f32 + 0.5).sin().max(0.2);
            params.fg.b = ((value % 1000) as f32).sin().max(0.2);
        }
    }
}