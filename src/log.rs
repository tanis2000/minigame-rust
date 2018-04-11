use sdl2::log::log;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

static LOGLEVEL: LogLevel = LogLevel::Info;

pub struct Log {
}

impl Log {
    pub fn debug(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                log(msg);
            },
            _ => {}
        }
    }

    pub fn info(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                log(msg);
            },
            LogLevel::Info => {
                log(msg);
            },
            _ => {}
        }
    }

    pub fn warning(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                log(msg);
            },
            LogLevel::Info => {
                log(msg);
            },
            LogLevel::Warning => {
                log(msg);
            },
            _ => {}
        }
    }

    pub fn error(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                log(msg);
            },
            LogLevel::Info => {
                log(msg);
            },
            LogLevel::Warning => {
                log(msg);
            },
            LogLevel::Error => {
                log(msg);
            },
        }
    }
}