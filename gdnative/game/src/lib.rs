use gdnative::prelude::*;
// Each file in these modules is used to run a specific godot-specs example.
mod runners;
use runners::*;

mod logger;



fn init(handle: InitHandle) {
    // Initialize the logger to make compatible with godot.
    flexi_logger::Logger::with_str("trace")
        .log_target(flexi_logger::LogTarget::Writer(Box::new(
            logger::GodotLogWriter {},
        )))
        .start()
        .unwrap();
    gd_specs::init(handle);
    // Each of the classes needs to be added in here.
    handle.add_class::<Example01>();
    handle.add_class::<Example02>();
    handle.add_class::<Example03>();
    handle.add_class::<Example04>();
    handle.add_class::<Example05>();
    handle.add_class::<Example06>();
    handle.add_class::<Example07>();
    handle.add_class::<HybridUpdate01>();
    handle.add_class::<EndlessSpawnerSignals>();
    handle.add_class::<EndlessSpawnerHybrid>();
}

godot_init!(init);
