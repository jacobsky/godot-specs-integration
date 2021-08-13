//! This contains definitions for specific resources that can be used by the core specs engine.
#[derive(Default, Clone, Copy)]
pub struct Time {
    pub delta: f32,
    pub total: f32
}
impl Time {
    pub fn new() -> Self {
        Self { delta: 0f32, total: 0f32 }
    }
}


/// This is a thin wrapper around the `crossbeam::queue::SegQueue`. As each resource must be unique, using this allows differentiating between
/// queues by the message type. Message types must be defined by each system. 
#[derive(Default)]
pub struct WorldMsgQueue<T>(pub crossbeam::queue::SegQueue<T>);

impl <T> WorldMsgQueue <T> {
    pub fn new() -> Self {
        Self(crossbeam::queue::SegQueue::new())
    }
    pub fn push(&self, message: T) {
        self.0.push(message);
    }
    pub fn pop(&self) -> Option<T>{
        self.0.pop()
    }
}
