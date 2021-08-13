macro_rules! declare_msg_queue {
    ($name:ident, $message_type:ty) => {
        #[derive(Debug, Default)]
        pub struct $name(crossbeam::queue::SegQueue<$message_type>);

        impl $name {
            pub fn new() -> Self {
                Self(crossbeam::queue::SegQueue::new())
            }
        }

        impl std::ops::Deref for $name {
            type Target = crossbeam::queue::SegQueue<$message_type>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
mod message_passing;
mod movement;
mod side_effects;

pub use message_passing::*;
pub use movement::*;
pub use side_effects::*;