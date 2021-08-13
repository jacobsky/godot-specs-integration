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

## ECS Specific Patterns

### Side Effect Management via Message Passing

SideEffects is anything that modifies the state of the program. Rust is a language that cares deeply about tracking side effects via the mutability and immutability of data. For a better overview of Side Effects as they pertain to programming [this wikipedia article](https://en.wikipedia.org/wiki/Side_effect_(computer_science)) is a great starting place.

With games programming there can be many, seemlingly random side effects. For example: Movement, Spawning Mobs, Incrementing Score, using Items, handling achievements, etc.

While it can be possible (depending upon the style of the game) to ensure that each side effect occurs in one (and only one) system, it is unlikely that that will occur. In addition, due to Rust's mutability paradigm, each system can access components mutably and immutibly via the `WriteStorage<T>` and `ReadStorage<T>` respectively. While the specs `Dispatcher` will ensure that no two systems will ever have mutable access to the same storage at the same time. This often leads to complex sets of side effects resulting in a much less thread friendly set of systems.

Something that ECS can often reflect is the idea of Micro Service Architectures where each system is responsible for one set of decisions and then can use message passing to dispatch behaviors to other services. With a series of message queues that can be used to send requests for changes that can be managed by separate systems.

This also allows for `example_06.rs` has a simple example of this kind of configuration.


While some ECS implementations already have this implemented, in Specs you will have to do this more manually.
