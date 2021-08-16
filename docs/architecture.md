# Architecture

This part of the guide pertains to the architectural patterns that I have found useful when working on interoperability between Godot and an ECS (in the case of this project [Specs Parallel ECS](https://github.com/amethyst/specs))

Please take a look at the individual code and associated comments for specific information.

## ECS <-> Godot Interoperability

The interoperability comes into play by having an `specs-engine` crate and `gd-specs` crate that help facilitate this. This helps to isolate as much of the interoperability as possible.

For this project the main interoperability layers come in the form of `GDWorld` and `GDEntity`. These two `NativeClass`es help to mitigate 

### GDWorld

The `GDWorld` class is a demonstration of how you can wrap a specs `World` resource in a way that it can be easily accessible from Rust based scripts without needing to import from specs directly in the core game crate.

The goal is to abstract the main tasks that will use an ECS and allow for other parts of the game to run it when it needs to run.

This resource can be used for both Editor-centric and Server-centric approaches. 

### GDEntity

GDEntity is a a general approach to including an entity and component data that is accessible and synchonized from within the scene tree.

In this case, the Entity must know the GDWorld that is should be associated with via the `world_path: NodePath` property. If you do not plan to have multiple `GDWorld`s for your game. An alternate approach is to encode this to a global singleton that serves as the `GDWorld` repository and can easily map between the relevant worlds. Then you just need to attach a GDWorld ID to each entity so that it knows where to register.

In this example, `GDWorld` uses a Dictionary to map the necessary values to Variants.

One option that may be useful is implementing `TryFrom<Dictionary>` and `using #[derive(ToVariant)]` to help smooth this transition between Godot and Rust and allow for some better type representations in the editor.

For this set of examples, I will just be using Manually implemented dictionaries in the editor.

### WorldMsgQueue\<T\>

WorldMsgQueue is a generic container resource that can be used to abstract away the under lying message queue. It exposes a `push()` and `pop()` which (in this example) mirrors the `crossbeam` crates SegQueue and ArrayQueue semantics. This queue is intended to be used as an externally accessible queue.

As specs resources can only have a single resource of a given T, this allows safe access in to the world based on message type exclusively.

A potential extension would be to create separate types or a use `PhantomData` to create `InboundOnly` and `OutboundOnly` queues that can be accessed via the `GDWorld`

### Direct Synchronization via Servers

This option 

As Godot has several Singletons that serve as the API Servers, one additional option is to synchronize the ECS by using the API Servers directly.

This involves manually creating the canvas items, materials, etc via the systems directly from the ECS while skipping any synchronization with the Scene tree.

All CanvasItems need to be manually managed.

This method has a major draw back of not using the `SceneTree` and, as a result, the SceneTree.
VisualServer

### Hybrid Synchronization

As demonstrated with the `GDEntityHybrid` and `GDWorldHybrid` classes, one method that may allow us access into the best of both worlds without too many trade-offs.

The GDEntityHybrid and GDWorldHybrid are responsible for the instantiation of the relevant components into the ECS and the instantiation of the `VisualServer` rids and other resources. In this implementation the GDEntityHybrid still synchronizes the data from the ECS, but it does not set the Node2D components directly.

Instead we let the systems interact with the `VisualServer` directly to handle all of the direct scene related updates.

Instead of implementing something like
```rust
#[inline]
    fn sync_scene_tree(&mut self, owner: TRef<Node2D>, world: &World) {
        use crate::{ShaderParams, TextureOverride};
        use specs_engine::{Position, Scale, Rotation};
        let entity = self.entity.expect("this should work");
        // TODO: These probably shouldn't be optional since they are in Node2D which this inherits.
        // It would be cleaner (and more correct) to just include an ".expects"
        if let Some(position) = world.read_storage::<Position>().get(entity) {
            // In this case, I only care about the global position, but it would be possible to use the local position
            // To base the scale off of the parent
            owner.set_global_position(Vector2::new(position.x, position.y));
        }
        if let Some(scale)  = world.read_storage::<Scale>().get(entity) {
            owner.set_global_scale(Vector2::new(scale.x, scale.y));
        }
        if let Some(rotation)  = world.read_storage::<Rotation>().get(entity) {
            owner.set_global_rotation(rotation.radians.into());
        }
        // We will give an option for the ECS to override the currently drawn texture in godot.
        if let Some(texture) = world.read_storage::<TextureOverride>().get(entity) {
            // First we attempt to find the "sprite" node which we assume is attached to this entity
            if let Some(child) = owner.get_node("sprite") {
                // For safe modification we have to assert that this is the ONLY reference to the sprite at this time.
                // As this is internal to the sprite, we can safely assert this.
                let child = unsafe { child.assume_unique() };
                // Next, we cast the `Node` into `Sprite` to get the correct interface
                if let Ok(sprite) = child.try_cast::<Sprite>() {
                    // Finally we can set the texture
                    sprite.set_texture(texture.texture.clone());
                } else {
                    log::error!("cannot cast `sprite` to CanvasItem");
                }
            }
        }
```

We use a system like the following that runs last. Be sure to brush up on your matrix math so that you understand what's going on.
```rust
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
            // This is the matrix math to handle rotation and scaling for a 2D object, this is more readable to you.
            // Note: you could also probably calculate this as
            // let mut tform = Vector2::new(position.x, position.y).to_transform()
            // tform.m11 = rotation.radians.cos() * scale.x;
            // tform.m12 = rotation.radians.sin();
            // tform.m21 = -rotation.radians.sin();
            // tform.m22 = rotation.radians.cos() * scale.y;

            let origin_x= position.x;
            let origin_y = position.y;
            // Put it in the transform
            let transform = Transform2D::new(x1, x2, y1, y2, origin_x, origin_y);
            // Let the VisualServer do the hard work.
            vs.canvas_item_set_transform(ci.rid, transform);
        }
    }
}
```
Oh, let's also make sure that we can update the ShaderParam as necessary with a system.

```rust
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
```

As it is also possible to use the `GDEntityHybrid` as an opaque ID (though a Reference class would probably be better for this) to allow for certain GDScript-based queries to exist. This makes synchronizing the components Dictionary optional depending upon how many signals you make use of.

From initial testing, I can see that generally speaking usijng individual callbacks is incredibly slow (even in rust) whereas using the hybrid approach is really nice for the performance budget.

    Important Side Note: Don't forget to turn on the thread safety if you go this route or things might deadlock. Found that bit out the hard way.
    Important Side Note#2 : In project settings increase the following settings appropriately
        In Memory -> Limits:
            Multithreaded Server -> Rid Pool Prelloc: Set this to 500
            Message Queue -> Max Size kb: While I was
            Command Queue -> Multithreading Queue Size Kb: Increase this as soon as you hit lag spikes. 

#### Final optimizations

There is one other point to consider with these system. As currently the systems run every frame and update the visual server for each.

Many ECS systems offer some form of mutation tracking for entities. In specs case, by using `FlaggedStorage` and `ComponentEvent` it is possible to be notified of changes. As there is overhead associated with pushing and reading the event, this is not always an optimization.

For example: in the top-down shooter game I am making for testing purposes most (>99%) things either move, rotation or scale every frame). In this case, using `FlaggedStorage` for `Position`, `Rotation` and `Scale` are unlikely to the amount of transforms that we will update every frame. In this case, the overhead caused by publishing and processing each storage's `ComponentEvent` will (probably) result in an overall performance drop.

This is the opposite of `ShaderParams` where maybe 1-10% of the entities will actually modify their ShaderParams each frame. In this case the overhead is well worth it and we can substantially reduce the number of calls to the `VisualServer`

Note: In a turn-based game where few entities change their position, rotation, or scale at a time, it would be helpful to use `FlaggedStorage` for managing updates to the transforms.


TL;DR: Don't just add `FlaggedStorage` without thinking about your data and when it will update.

By using an eventing system, such as Specs `FlaggedStorage` it is possible to ensure that we only update when necessary. In the examples in the source, I chose to implement `FlaggedStorage` only for `ShaderParams` and not `Position`, `Rotation`, or `Scale`. The reason is that I am working on creating a shoot-em up game where there are very few non-moving entities. As almost all entities can be assumed to need to update their tranform, adding the events to mark almost all of the events as dirty every frame isn't worth it.

In turn based games or games with a lot of interactive (but stationary) entities, using `FlaggedStorage` may turn into an optional.

This is the opposite of the `ShaderParams` which (in this case) rarely change so synchronization doesn't need to happen every frame. This means that the added overhead of changes by emitting events is a huge optimization.

TL;DR: Use the SceneTree for instantiation and freeing resources. Use the VisualServer directly to update everything else.

One question about this approach is "is the CanvasItem position pushed to the Server every frame? Or will ignoring (and not touching position) update everything in the background?

### Avoid Parallel Access to Godot Singletons

You might be tempted in your ECS to try to parallelize your access to the visual server, due to the way that the godot thread-safety is supported, this is almost always going to result in more complex code that is not significantly more performant.

Due to how godot is created, any singleton reference returned by a `godot_singleton()` method call does not implement `Sync` so it is not a thread safe reference. If you would like to access it from multiple threads, it is necessary to create a new reference to the singleton in every thread and requesting access vastely increases the overhead.

In this case, it is adviseable to access any given thread from a single place at a given time.

In the `endless_spawner_hybrid`, I there is an option that can be selected where the engine does access the  `VisualServer` until it returns from the ecs run. By using a crossbeam `ArrayQueue`, you can safely queue messages to be run in the main thread. As you can see from running the experiment, this does appear to sufficiently change the performance characteristics of the thread.

In some ways, this is really convenient that the simplest method yields the best performance. I also did the leg work in this repository so there's no need for you to research this on your own :).
## ECS Specific Patterns

### Side Effect Management via Message Passing

SideEffects is anything that modifies the state of the program. Rust is a language that cares deeply about tracking side effects via the mutability and immutability of data. For a better overview of Side Effects as they pertain to programming [this wikipedia article](https://en.wikipedia.org/wiki/Side_effect_(computer_science)) is a great starting place.

With games programming there can be many, seemlingly random side effects. For example: Movement, Spawning Mobs, Incrementing Score, using Items, handling achievements, etc.

While it can be possible (depending upon the style of the game) to ensure that each side effect occurs in one (and only one) system, it is unlikely that that will occur. In addition, due to Rust's mutability paradigm, each system can access components mutably and immutibly via the `WriteStorage<T>` and `ReadStorage<T>` respectively. While the specs `Dispatcher` will ensure that no two systems will ever have mutable access to the same storage at the same time. This often leads to complex sets of side effects resulting in a much less thread friendly set of systems.

Lazy

Something that ECS can often reflect is the idea of Micro Service Architectures where each system is responsible for one set of decisions and then can use message passing to dispatch behaviors to other services. With a series of message queues that can be used to send requests for changes that can be managed by separate systems.

This also allows for `example_06.rs` has a simple example of this kind of configuration.

While some ECS implementations already have this implemented, in Specs you will have to do this more manually.

### Side Effect Management with LazyUpdate

Lazy update is a very useful feature for any updates that need to touch a lot of systems simultaneously when the effect does not have other systems that depend upon the update. While it would be possible to use this as a message queue, it is best reserved for operations like spawning, where you need to add a new entity and add entries to a large number of storages. One note: As `LazyUpdate`s are dispatched in the order they are created, it is possible for race conditions to form when two systems attempt to lazily update the same data.

For more information about LazyUpdate:

[LazyUpdate Docs](https://docs.rs/specs/0.17.0/specs/struct.LazyUpdate.html)
`LazyUpdate` is a function of Specs that allows you to defer actions until the end of the current dispatching. This is good for anything that may need to touch a large number of systems.

While you could manage all side effects 