use std::fmt::Display;

pub struct Logger {
    max_level: LogLevel,
}

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    // ERROR,
    // WARNING,
    INFO,
    VERBOSE,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match *self {
            // LogLevel::ERROR => "ERROR",
            // LogLevel::WARNING => "WARNING",
            LogLevel::INFO => "INFO",
            LogLevel::VERBOSE => "VERBOSE",
        };
        write!(f, "{}", to_print)
    }
}

impl Logger {
    pub fn new(max_level: LogLevel) -> Self {
        Self { max_level }
    }

    pub fn log<S: AsRef<str>>(&self, level: LogLevel, msg: S) {
        if self.max_level >= level {
            println!("[{level}]: {}", msg.as_ref())
        }
    }
}
