# How to integrate SPECS Parallel ECS with Godot-Rust

There is a lot that goes into integrating a Rust based ECS with Godot. This is a compilation of patterns that I have found useful when attempting to work with [specs parallel ecs]() and the godot-rust project.

This project is intended is the results of a number of experiments regarding ECS and trying to develop patterns that may be useful to other developers that want to try integrating their ECS of choice with Godot and have no idea where to begin.

## Project Overview

This project was created to demonstrate the following
- How to structure workspaces to make working with ECS and Godot as painless as possible.
- Useful patterns with ECS
- How to write the synchonization glue with Godot.
- How to use make use of Godot's Servers to optimize specific systems of your game.
- A Top Town Shoot 'em up demonstrating all of the above



### Godot Project Structure

There's nothing incredibly special to this project structure, though I generally recommend keeping the project and the Cargo workspace in separate sibling directories such as with this repository where the workspace is located in "gdnative" and the godot files are located in "project" 

### GDNative Crate Structure

When working with GDNative, I like to try to ensure that I maintain very strict modularity and I accomplish that by making use of cargo workspaces.

In the workspace structure I have the following crates

| crate | Description |
| --- | --- |
| game | This acts as the entry point to the games. This contains anything that is unrelated to specs, such as UI, singletons and other managers. |
| gd-specs | This crate houses the "glue" layer that attempts to expose the specs specific functionality to godot. |
| specs-engine | This crate is used for logic itself. For portability it includes a feature called "godot" which is used for godot compatibility. It attempts to maintain a minimum number of dependencies and is where the main system logic is contained. |

It is also possible to include more crates, but for these examples, this would likely be unnecessary.

The architecture document will go into more depth about the specific modules and how they are intended to interact.