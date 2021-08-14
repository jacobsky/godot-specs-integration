//! This contains an example of creating entities and canvas items by hand.
//! This is unnecessary for most games, but can be useful if you need a lightweight solution for performance or for the rest of your solution
//!    OR if you want to use the Godot as a backend and do not need editor support.
//! Note: One major limitation of this approach is that the SceneTree that the editor is built upon is completely bypassed.
//! While you will still be able to make use of the output logs, much of the standard debugging and editing process will be unnavailable to you.

use gdnative::api::{Material, VisualServer};
use gdnative::prelude::*;

use specs::prelude::*;

use super::CanvasRoot;
use core_engine::components::*;

pub struct CreateCanvasItemMsg {
    entity: Entity,
    canvas: Rid,
    texture: Ref<Texture>,
    material: Ref<ShaderMaterial>
}

pub struct CreateCanvasItemSystem;

impl<'a> System<'a> for CreateCanvasItemSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, CanvasRoot>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        WriteStorage<'a, CanvasItem>,
    );
    fn run(&mut self, data: Self::SystemData) {
        // let (entities, canvas_root, positions, renderables, mut canvas_items) = data;
        // let vs = unsafe { VisualServer::godot_singleton() };
        // let parent_rid = canvas_root.0;

        // // TODO: Convert these to proper resources loaded at initialization.
        // let sprite_rsrc = ResourceLoader::godot_singleton()
        //     .load(crate::SPRITE_SHEET_RESOURCE, "", false)
        //     .expect("Resource not found")
        //     .cast::<Texture>()
        //     .expect("should be Texture");

        // let material_rsrc = ResourceLoader::godot_singleton()
        //     .load(crate::ENTITY_SHADER_MATERIAL_RESOURCE, "", false)
        //     .expect("resource not found")
        //     .cast::<Material>()
        //     .expect("should be able to cast to Shader");

        // let mut to_insert_canvas_items: Vec<(Entity, GD_CanvasItem)> = Vec::new();

        // // For every entity that has a position but not a canvas_item, we need to create it properly.
        // for (entity, renderable, position, _canvas_item) in
        //     (&entities, &renderables, &positions, !&canvas_items).join()
        // {
        //     // Create the canvas_item
        //     log::trace!("creating relevant rids");
        //     let rid = vs.canvas_item_create();
        //     vs.canvas_item_set_parent(rid, parent_rid);
            
        //     let transform = Vector2::new(
        //         position.x as f32 * crate::TILE_WIDTH,
        //         position.y as f32 * crate::TILE_HEIGHT,
        //     ).to_transform();

        //     let shader_material_rsrc = unsafe {
        //         material_rsrc
        //             .assume_safe()
        //             .duplicate(false)
        //             .expect("this should work")
        //             .cast::<Material>()
        //             .expect("should cast")
        //     };

        //     // Add the visibility mask
        //     // vs.canvas_item_set_light_mask(rid, crate::LIGHT_MASK_ENTITY_VISIBILITY_CULLING);

        //     // TODO: Move this into a lighting related thingy
        //     // Add the render order
        //     let z_index = renderable.render_order;
        //     vs.canvas_item_set_z_index(rid, z_index);
            
        //     // TODO: Move this into a canvas_item_z_index related thingy
        //     vs.canvas_item_set_light_mask(rid, crate::LIGHT_MASK_ENTITY_VISIBILITY_CULLING);

        //     // Put this information to the list to add to the entity.
        //     to_insert_canvas_items.push((
        //         entity,
        //         GD_CanvasItem {
        //             rid,
        //             parent_rid,
        //             material: Some(shader_material_rsrc.clone()),
        //             sprite: Some(sprite_rsrc.clone()),
        //             transform,
        //             z_index,
        //         },
        //     ));
        // }
        // // Add them in after the loop to make the borrow checker happy.
        // for (entity, canvas_item) in to_insert_canvas_items {
        //     canvas_items
        //         .insert(entity, canvas_item)
        //         .expect("Could not insert into the variable.");
        // }
    }
}

/// This message is used to tell the engine to free the various rids. This should include all the Rids of child nodes in the system.
pub struct FreeCanvasItemMessage{ pub rid: Vec<Rid> }

pub struct FreeCanvasItemSystem;
impl<'a> System<'a> for PruneCanvasItemSystem {
    type SystemData = WriteExpect<'a, WorldMsgQueue<FreeCanvasItemMessage>>;
    fn run(&mut self, data: Self::SystemData) {
        let queue = data;
        let vs = unsafe { VisualServer::godot_singleton() };
        while let Some(msg) = queue.pop() {
            vs.free_rid(msg.rid)
        }
    }
}