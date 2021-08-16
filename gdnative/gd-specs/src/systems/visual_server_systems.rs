use specs::prelude::*;
use crate::components::*;
use specs_engine::{Position, Rotation, Scale, WorldMsgQueue};
use gdnative::prelude::*;
use gdnative::api::VisualServer;

use crate::ShaderParams;

pub struct VSUpdateTransforms {}


impl<'a> System<'a> for VSUpdateTransforms {
    type SystemData = (
        ReadStorage<'a, crate::components::CanvasItem>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, Scale>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // Start fresh
        let (canvas_items, positions, rotations, scales) = data;
        // Synchronize the position of any canvas items that exist
        let vs = unsafe { VisualServer::godot_singleton() };
        for (ci, position, scale, rotation) in (&canvas_items, &positions, &scales, &rotations).join() {
            // This is the matrix math to handle rotation and scaling for a 2D object
            let x1 = rotation.radians.cos() * scale.x;
            let x2 = rotation.radians.sin();
            let y1 = -rotation.radians.sin();
            let y2 = rotation.radians.cos() * scale.y;
            // This is the offset
            let origin_x= position.x;
            let origin_y = position.y;
            // Put it in the transform
            let transform = Transform2D::new(
                x1, x2, y1, y2, origin_x, origin_y
            );
            // log::trace!("update transform to {:?}", transform);
            // Let the VisualServer do the hard work.
            vs.canvas_item_set_transform(ci.rid, transform);
        }
    }
}

use once_cell::sync::Lazy;
use crossbeam::queue::ArrayQueue;
/// This is the globally accessible queue that is used to dispatch the VisualServer::canvas_item_set_transform(Rid, Transform2D) command from the main thread.
pub static VS_TRANSFORM_QUEUE : Lazy<ArrayQueue<VSTransformSetMessage>> = Lazy::new(|| {
    ArrayQueue::new(200000)
});

#[derive(Debug, Default)]
pub struct VSTransformSetMessage(pub Rid, pub Transform2D);
pub struct VSUpdateTransformsParallel {}


impl<'a> System<'a> for VSUpdateTransformsParallel {
    type SystemData = (
        // WriteExpect<'a, WorldMsgQueue<VSTransformSetMessage>>,
        ReadStorage<'a, crate::components::CanvasItem>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        ReadStorage<'a, Scale>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // Start fresh
        let (canvas_items, positions, rotations, scales) = data;
        
        for (ci, position, scale, rotation) in (&canvas_items, &positions, &scales, &rotations)
            .join() {
            // .par_join()
            // // Is ther some way to get the visual server for each parallel thread instead of having to grab it for each item
            // .for_each(|(ci, position, scale, rotation)| {
                // This is the matrix math to handle rotation and scaling for a 2D object
                let x1 = rotation.radians.cos() * scale.x;
                let x2 = rotation.radians.sin();
                let y1 = -rotation.radians.sin();
                let y2 = rotation.radians.cos() * scale.y;
                // This is the offset
                let origin_x= position.x;
                let origin_y = position.y;
                // Put it in the transform
                let tform = Transform2D::new(
                    x1, x2, y1, y2, origin_x, origin_y
                );
                VS_TRANSFORM_QUEUE.push(VSTransformSetMessage(ci.rid, tform)).expect("this should work");
            }
        // );
        
    }
}

pub struct VSUpdateTransformsThreadLocal {}
impl <'a> System <'a> for VSUpdateTransformsThreadLocal {
    type SystemData = WriteExpect<'a, WorldMsgQueue<VSTransformSetMessage>>;
    fn run(&mut self, data: Self::SystemData) {
        let queue = data;
        let vs = unsafe { VisualServer::godot_singleton() };
        while let Some(msg) = queue.pop() {
            vs.canvas_item_set_transform(msg.0, msg.1);
        }
    }
}
pub struct VSUpdateShaderParams {}

impl <'a> System <'a> for VSUpdateShaderParams {
    type SystemData = (
        ReadStorage<'a, CanvasItemShader>,
        ReadStorage<'a, ShaderParams>);
    fn run(&mut self, data: Self::SystemData) {
        let (shader_materials, shader_params) = data;
        let vs = unsafe {VisualServer::godot_singleton()};
        for (material, shader_param) in (&shader_materials, &shader_params).join() {
            let material = unsafe { material.material.assume_safe() };
            let rid = material.get_rid();
            vs.material_set_param(
                rid,
                "fg",
                shader_param.fg,
            );
            vs.material_set_param(
                rid,
                "bg",
                shader_param.bg,
            );
        }
    }
}