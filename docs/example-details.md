# Integration Examples

This is an explanation of all the examples that are being offered with insight into the intent behind how they work.

In the source files for the examples

## Example 01 - Moving Objects

This example scene is located at `res::/basics/example03.tscn`

Source code for the example is located in `gdnative\game\src\runners\example_01.rs`

This show cases that objects can move and bounce around the level. The entity scenes can also have their component dictionaries tweaked to change their initial velocity.

Due to how godot works, all Node2D objects have a position, scale and rotation already, so it's possible to update this information directly in the entity.

All other components are coerced into the correct objects by `GDWorld` during the `synchronize()` method call. In this example the node is responsible for listening for the directly.

For more information on how the `GDEntity` and `GDWorld` work, please refer to the [architecture](architecture.md).

As a bonus, I always added a color change system to demonstrate how to manipulate shader material properties from the ECS

## Example 02 - Movement by Input

This example scene is located at `res::/basics/example03.tscn`

Source code for the example is located in `gdnative\game\src\runners\example_02.rs`

As an extension of the previous example, this demonstrates adding player movement.

In addition, this sprite includes a `TextureOverride` which is used to convert the smiley face to the traditional `@` of the player character in traditional ASCII Roguelike games.

## Example 03 - Movement with Moving Objects
Source code for the example is located in `gdnative/game/src/runners/example_03.rs`

This example scene is located at `res::/basics/example03.tscn`

This show cases handling entities that need to respect two different control schemes.

The main point of interest for this example is located in `gdnative/gd-specs/src/dispachers.rs` at line 19. I'll just inline it below to make it easier to explain.

```rust
pub fn example_3_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
        .with(ChangeVelocityAtBounds{}, "change_vel_at_bound", &[])
        .with(SetVelocitySystem{}, "update_velocity", &["change_vel_at_bound"])
        // Option 1 - Barrier
        .with_barrier()
        .with(UpdateUnboundedPositionSystem{}, "update_unbounded_position", &[])
        .with(UpdateBoundedPositionSystem{}, "update_bounded_position", &[])
        // Option 2 - Explicit dependencies
        // .with(UpdateUnboundedPositionSystem{}, "update_unbounded_position", &["update_velocity", "change_vel_at_bound"])
        // .with(UpdateBoundedPositionSystem{}, "update_bounded_position", &["update_velocity", "change_vel_at_bound"])
        // .build()
}
```

The point of interest is in how we choose to schedule our nodes. Due to how the logic works, we want to ensure that some entities bounce, while other entities done. So it is imperative, that we let the `ChangeVelocityAtBounds` system is run before any other system. Then we can run the `SetVelocitySystem` to update based on any velocity override intents.

Finally, we can either add an explicity barrier which will guarantee that the velocity changes are completed before updating the positions or we can add explicit dependencies to allow specs to determine that for us.

The reason this works is that the specs `Dispatcher` is able to determine how to schedule the tasks based on their `Storage` dependencies. And this is good for most tasks where order does not matter.

In example 1 and example 2, as you borrow a storage immutably (as `ReadStorage<T>`) while it is borrowed mutably (as `WriteStorage<T>`), it will fallback to registration order automatically. Even in the first two examples, there is a very clear requirement that you must mutate the `Velocity` before you can update the `Position`.

## Example 4 - Hello Message Queue

This example scene is located in `res::/patterns/example04.tscn`

This demonstrates how to send a custom message to a system with a message queue. This is essentially the Hello World! of message queues.

### POI #1

This demonstrates how you can queue messages.

### POI #2

It's also possible to inject messages based on user input.

## Example 5 - Overengineered FizzBuzz

This example scene is located in `res::/patterns/example05.tscn`

Source code for the example is located in 
`gdnative\game\src\runners\example_05.rs`

This demonstrates how to create public and private queues for sending messages into and out of the `GDWorld` to create a heavily over engineered implementation of the fizzbuzz game [Fizz buzz](https://en.wikipedia.org/wiki/Fizz_buzz)

### POI #1

The astute viewer may note that there is a distinct difference between some of the message queue resources.

With `GDWorld` any queue that is of type `WorldMsgQueue<T>` can easily be resolved by querying for the message type. While anything that should be handled between systems and not interfered with from specs should not implement this.

This feature allows for message queues to be freely available to systems, without fear of accidentally overwriting or popping incorrect messages an an inopportune time.

### POI #2

It is possible to verify the assertion made about POI #1 by attempting to interfere and try uncommenting some of the lines. You will get a nicely logged godot error anytime that you attempt to do that. As it uses `try_fetch` under the hood, we can avoid a `panic` and just ignore the input.

## Example 6 puts together the 