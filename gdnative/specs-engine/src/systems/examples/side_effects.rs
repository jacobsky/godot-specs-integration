use specs::prelude::*;
use crate::components::Counter;

pub enum SideEffect {
    SetZero,
    Increment,
    Decrement,
    Add { amount: i32 },
    Subtract { amount: i32 },
}

declare_msg_queue!(SideEffectQueue, (Entity, SideEffect));

pub struct CountModifier1System {}
impl <'a> System <'a> for CountModifier1System {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SideEffectQueue>,
        ReadStorage<'a, Counter>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, queue, counters) = data;
        for (entity, counter) in (&entities, &counters).join() {
            if counter.0 > 0 {
                queue.push((entity, SideEffect::Add{ amount: counter.0}))
            }
        }
    }
}

pub struct CountModifier2System {}
impl <'a> System <'a> for CountModifier2System {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SideEffectQueue>,
        ReadStorage<'a, Counter>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, queue, counters) = data;
        for (entity, counter) in (&entities, &counters).join() {
            if counter.0 > 500 {
                queue.push((entity, SideEffect::SetZero{}))
            } else {
                queue.push((entity, SideEffect::Add { amount: 5 }));
            }
        }
    }
}
pub struct CountModifier3System {}
impl <'a> System <'a> for CountModifier3System {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SideEffectQueue>,
        ReadStorage<'a, Counter>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, queue, counters) = data;
        for (entity, counter) in (&entities, &counters).join() {
            if counter.0 % 10 == 0 {
                queue.push((entity, SideEffect::Subtract{ amount: 1}))
            } else {
                queue.push((entity, SideEffect::Increment{}));
            }
        }
    }
}
pub struct CountModifier4System {}
impl <'a> System <'a> for CountModifier4System {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, SideEffectQueue>,
        ReadStorage<'a, Counter>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, queue, counters) = data;
        for (entity, counter) in (&entities, &counters).join() {
            if counter.0 > 100 {
                queue.push((entity, SideEffect::Increment{}))
            } else {
                queue.push((entity, SideEffect::Add { amount: 10 }));
            }
        }
    }
}


pub struct CounterSideEffectsSystem {}

impl <'a> System <'a> for CounterSideEffectsSystem {
    type SystemData = (
        WriteExpect<'a, SideEffectQueue>,
        WriteStorage<'a, Counter>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            queue,
            mut counters
        ) = data;
        while let Some((entity, side_effect)) = queue.pop() {
            if let Some(counter) = counters.get_mut(entity) {
                match side_effect {
                    SideEffect::SetZero => { counter.0 = 0; },
                    SideEffect::Increment => { counter.0 += 1;},
                    SideEffect::Decrement => { counter.0 -= 1; },
                    SideEffect::Add { amount } => { counter.0 += amount },
                    SideEffect::Subtract { amount } => { counter.0 -= amount },
                }
            }
        }
    }
}