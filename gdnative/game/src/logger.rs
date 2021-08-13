use flexi_logger::writers::LogWriter;
use flexi_logger::{DeferredNow, Record};
use gdnative::prelude::*;
use log::{LevelFilter};

pub struct GodotLogWriter {}

impl LogWriter for GodotLogWriter {
    fn write(&self, _now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        match record.level() {
            flexi_logger::Level::Error => godot_error!(
                "{}: {} -- {}",
                record.level(),
                record.target(),
                record.args()
            ),
            flexi_logger::Level::Warn => godot_warn!(
                "{}: {} -- {}",
                record.level(),
                record.target(),
                record.args()
            ),
            _ => godot_print!(
                "{}: {} -- {}",
                record.level(),
                record.target(),
                record.args()
            ),
        };
        Ok(())
    }

    fn flush(&self) -> std::io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> LevelFilter {
        LevelFilter::Trace
    }
}

/// This is the global singleton that is used for logging
#[derive(NativeClass, Copy, Clone, Default)]
#[user_data(Aether<GlobalLogger>)]
#[inherit(Node)]
pub struct GlobalLogger;
#[methods]
impl GlobalLogger {
    fn new(_: &Node) -> Self {
        Self {}
    }
    #[export]
    fn error(&self, _: &Node, target: String, message: String) {
        log::error!(target: &target, "{}", message);
    }
    #[export]
    fn warn(&self, _: &Node, target: String, message: String) {
        log::warn!(target: &target, "{}", message);
    }
    #[export]
    fn info(&self, _: &Node, target: String, message: String) {
        log::info!(target: &target, "{}", message);
    }
    #[export]
    fn debug(&self, _: &Node, target: String, message: String) {
        log::debug!(target: &target, "{}", message);
    }
    #[export]
    fn trace(&self, _: &Node, target: String, message: String) {
        log::trace!(target: &target, "{}", message);
    }
}
