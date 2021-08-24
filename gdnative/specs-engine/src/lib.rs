//! The ECS crate that contains all of the ECS specific implementation details.

mod components;
mod resources;
mod systems;
mod util;

pub use components::*;
pub use resources::*;
pub use systems::*;
pub use util::*;

#[cfg(test)]
static TEST_LOGGER_INIT: std::sync::Once = std::sync::Once::new();
#[cfg(test)]
fn init_test_logger() {
    TEST_LOGGER_INIT.call_once(|| {
        flexi_logger::Logger::with_str("debug")
            .log_target(flexi_logger::LogTarget::StdOut)
            .start()
            .expect("the logger should start");
    });
}