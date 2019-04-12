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
                println!("{}", msg);
            },
            _ => {}
        }
    }

    pub fn info(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                println!("{}", msg);
            },
            LogLevel::Info => {
                println!("{}", msg);
            },
            _ => {}
        }
    }

    pub fn warning(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                println!("{}", msg);
            },
            LogLevel::Info => {
                println!("{}", msg);
            },
            LogLevel::Warning => {
                println!("{}", msg);
            },
            _ => {}
        }
    }

    pub fn error(msg: &str) {
        match LOGLEVEL {
            LogLevel::Debug => {
                println!("{}", msg);
            },
            LogLevel::Info => {
                println!("{}", msg);
            },
            LogLevel::Warning => {
                println!("{}", msg);
            },
            LogLevel::Error => {
                println!("{}", msg);
            },
        }
    }
}