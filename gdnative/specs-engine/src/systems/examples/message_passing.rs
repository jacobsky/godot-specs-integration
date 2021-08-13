use specs::prelude::*;
use crate::resources::WorldMsgQueue;

pub struct StringMessage {
    pub message: String
}

// Note: This resource MUST be added to the simulation for this system to work.
pub struct MessagePrintingSystem {}

impl <'a> System <'a> for MessagePrintingSystem {
    type SystemData = WriteExpect<'a, WorldMsgQueue<StringMessage>>;
    fn run(&mut self, data: Self::SystemData) {
        let queue = data;
        while let Some(msg) = queue.pop() {
            log::info!("Received message: {}", msg.message);
        }
    }
}

pub struct FizzbuzzInputMessage(pub i32);
pub struct FizzbuzzOutputMessage(pub String);

/// This is where FizzBuzz dispatches originate.
pub struct FizzBuzzDispatchSystem;

impl <'a> System <'a> for FizzBuzzDispatchSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, WorldMsgQueue<FizzbuzzInputMessage>>,
        ReadExpect<'a, FizzQueue>,
        ReadExpect<'a, BuzzQueue>,
        ReadExpect<'a, FizzBuzzQueue>,
        ReadExpect<'a, WorldMsgQueue<FizzbuzzOutputMessage>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            dispatch,
            fizz,
            buzz,
            fizzbuzz,
            output,
        ) = data;
        while let Some(msg) = dispatch.pop() {
            let value = msg.0;
            if value % 15 == 0 {
                fizzbuzz.push(FizzBuzzMessage{});
            } else if value % 5 == 0 {
                buzz.push(BuzzMessage{});
            } else if value % 3 == 0 {
                fizz.push(FizzMessage{});
            } else {
                // Our dispatch knows how to handle non-special fizzing and buzzing cases.
                output.push(FizzbuzzOutputMessage(format!("{}", value)));
            }
        }
    }
}

pub struct FizzMessage;
declare_msg_queue!(FizzQueue, FizzMessage);
pub struct FizzSystem;

impl <'a> System <'a> for FizzSystem {
    type SystemData = (
        WriteExpect<'a, FizzQueue>,
        ReadExpect<'a, WorldMsgQueue<FizzbuzzOutputMessage>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (queue, output) = data;
        while queue.pop().is_some() {
            output.push(FizzbuzzOutputMessage("Fizz!".to_owned()));
        }
    }
}

pub struct BuzzMessage;
declare_msg_queue!(BuzzQueue, BuzzMessage);
pub struct BuzzSystem;
impl <'a> System <'a> for BuzzSystem {
    type SystemData = (
        WriteExpect<'a, BuzzQueue>,
        ReadExpect<'a, WorldMsgQueue<FizzbuzzOutputMessage>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (queue, output) = data;
        while queue.pop().is_some() {
            output.push(FizzbuzzOutputMessage("Buzz!".to_owned()));
        }
    }
}
pub struct FizzBuzzMessage;
declare_msg_queue!(FizzBuzzQueue, FizzBuzzMessage);
pub struct FizzBuzzSystem;
impl <'a> System <'a> for FizzBuzzSystem {
    type SystemData = (
        WriteExpect<'a, FizzBuzzQueue>,
        ReadExpect<'a, WorldMsgQueue<FizzbuzzOutputMessage>>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (queue, output) = data;
        while queue.pop().is_some() {
            output.push(FizzbuzzOutputMessage("Fizzbuzz!".to_owned()));
        }
    }
}
