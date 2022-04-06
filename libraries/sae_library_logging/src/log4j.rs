use crate::*;
use log::{
    debug, error, info, trace, warn
};

#[derive(Default)]
pub struct Log4jWrapper;

impl Logging for Log4jWrapper {
    fn wrapper(self: Box<Self>) -> Box<dyn Logging> {
        log4rs::init_file(
            "/home/pjb/Documents/rust/rust-sae/libraries/sae_library_logging/loggin.yaml",
            Default::default(),
        )
        .unwrap();
        self
    }

    fn is_enabled(&self, level: Level) -> bool {
        true
    }

    fn write(&self, msg: &str, level: Level, error: Option<String>) {
        match level {
            Level::Trace => trace!("{0} {1}", msg, error.unwrap_or_default()),
            Level::Debug => debug!("{0} {1}", msg, error.unwrap_or_default()),
            Level::Info => info!("{0} {1}", msg, error.unwrap_or_default()),
            Level::Warn => warn!("{0} {1}", msg, error.unwrap_or_default()),
            Level::Error => error!("{0} {1}", msg, error.unwrap_or_default()),
            _other => {}
        }
    }
}
