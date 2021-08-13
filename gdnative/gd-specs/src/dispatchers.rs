//! This file contains the dispatcher configurations used in various example projects.
use specs::prelude::*;
use specs_engine::*;
use crate::systems::*;

pub fn example_1_dispatcher<'a, 'b>(change_color:bool) -> Dispatcher<'a, 'b> {
    let mut builder = specs::DispatcherBuilder::new()
        .with(ChangeVelocityAtBounds{}, "update_velocity", &[])
        .with(UpdatePositionSystem{}, "update_position", &[]);
    
        if change_color {
            log::trace!("enable GradientShaderParamFGSystem");
            builder = builder.with(RainbowColorSystem{}, "change_color", &[]);
        }
        builder.build()
}

pub fn example_2_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
        .with(SetVelocitySystem{}, "update_velocity", &[])
        .with(UpdatePositionWithBoundsSystem{}, "update_position", &[])
        .build()
}

pub fn example_3_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
        .with(ChangeVelocityAtBounds{}, "change_vel_at_bound", &[])
        .with(SetVelocitySystem{}, "update_velocity", &["change_vel_at_bound"])
        // Option 1
        .with_barrier()
        .with(UpdateUnboundedPositionSystem{}, "update_unbounded_position", &[])
        .with(UpdateBoundedPositionSystem{}, "update_bounded_position", &[])
        // Option 2
        // .with(UpdateUnboundedPositionSystem{}, "update_unbounded_position", &["update_velocity", "change_vel_at_bound"])
        // .with(UpdateBoundedPositionSystem{}, "update_bounded_position", &["update_velocity", "change_vel_at_bound"])
        .build()
}

pub fn example_4_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
        .with(MessagePrintingSystem{}, "printer", &[])
        .build()
}

pub fn example_5_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
    .with(FizzBuzzDispatchSystem{}, "fizzbuzz_dispatcher", &[])
    .with(FizzSystem{}, "fizz", &["fizzbuzz_dispatcher"])
    .with(BuzzSystem{}, "buzz", &["fizzbuzz_dispatcher"])
    .with(FizzBuzzSystem{}, "fizzbuzz", &["fizzbuzz_dispatcher"])
    .build()
}

pub fn example_6_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    specs::DispatcherBuilder::new()
    .with(CountModifier1System{}, "mod_1", &[])
    .with(CountModifier2System{}, "mod_2", &[])
    .with(CountModifier3System{}, "mod_3", &[])
    .with(CountModifier4System{}, "mod_4", &[])
    .with_barrier()
    .with(CounterSideEffectsSystem {}, "side_effects", &[])
    .with(ColorBasedOnCountSystem{}, "color_change", &["side_effects"])
    .build()
}