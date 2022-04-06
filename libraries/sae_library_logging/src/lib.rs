pub mod log4j;

pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Default)]
pub struct LoggingBuilder;

pub trait Logging {
    /// Wrapper logging
    fn wrapper(self: Box<Self>) -> Box<dyn Logging>;
    /// Is the log level enabled
    fn is_enabled(&self, level: Level) -> bool;
    /// Write log
    fn write(&self, msg: &str, level: Level, error: Option<String>);
    /// Record trace log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn trace(&self, msg: &str, error: Option<String>) {
        if self.is_enabled(Level::Trace) {
            self.write(msg, Level::Trace, error);
        }
    }
    /// Record debug log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn debug(&self, msg: &str) {
        if self.is_enabled(Level::Debug) {
            self.write(msg, Level::Debug, Option::None);
        }
    }
    /// Record info log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn info(&self, msg: &str) {
        if self.is_enabled(Level::Info) {
            self.write(msg, Level::Info, Option::None);
        }
    }
    /// Record warn log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn warn(&self, msg: &str) {
        if self.is_enabled(Level::Warn) {
            self.write(msg, Level::Warn, Option::None);
        }
    }
    /// Record error log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn error(&self, msg: &str, error: Option<String>) {
        if self.is_enabled(Level::Error) {
            self.write(msg, Level::Error, error);
        }
    }
    /// Record fatal log
    /// This function has a default implementation. Do not rewrite it unless necessary
    fn fatal(&self, msg: &str, error: Option<String>) {
        if self.is_enabled(Level::Fatal) {
            self.write(msg, Level::Fatal, error);
        }
    }
}

impl LoggingBuilder {
    pub fn add_logging<T: Logging + Default>(&self) -> Box<dyn Logging> {
        return Logging::wrapper(Box::new(T::default()));
    }
    pub fn add_log4j(&self) -> Box<dyn Logging> {
        return Logging::wrapper(Box::new(log4j::Log4jWrapper));
    }
}
