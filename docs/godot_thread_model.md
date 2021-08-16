# Godot's Thread Model

If you want to try to work with Godot and Specs, it's very important to understand the threading model.

For some background, here's what the official godot documation has to say about it. I would consider these required reading if you want to use any of the more advanced features of Godot Rust.

- [Using Multiple Threads](https://docs.godotengine.org/en/stable/tutorials/threads/using_multiple_threads.html)
- [Thread safe APIs](https://docs.godotengine.org/en/stable/tutorials/threads/thread_safe_apis.html)
- [Custom Godot Servers and Threading](https://docs.godotengine.org/en/stable/development/cpp/custom_godot_servers.html)

To summarize, 

- Global Singletons (Servers) are thread-safe (if Multi-threaded is selected in project settings).
- SceneTree manipulation is singlethreaded not thread-safe
- `call_deferred` can be used to safely make modifications to the main thread.
- Modifying Unique Resources is not thread-safe (if you are using Rust, the compiler will tell you about it.)

An addendum not covered in the documentation above is how the message queue and the command queue can impact the performance of your game. Note: this is important if you wish to optimize your game by using the Servers to directly synchronize your game and visual states.

## The Message Queue

The message queue is just used internally by godot for things like signalling, if it runs out of space, it will emit an error and you can increase the size.

## The Command Queue

This is the per server thread-safe queue.

When you have thread safety enabled, accessing any of the Godot Singletons (Servers) from outside the main thread, will instead push a command to the command queue to be executed asynchronously at a later time. If you do not have a sufficiently sized queue you will see major performance spikes as the entity count increases. This is either due to the `lock()` on the queue blocking until the Server is able to run or until it flushes all of the commands to make room on the queue. Either way, this leads to piece-wise performance dips that will appear to spike after certain breakpoints in the count of your nodes.

In order to avoid this issue, you will want to ensure that you have a sufficiently large queue that will allow you to queue all of the commands