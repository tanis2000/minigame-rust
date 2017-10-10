use sdl2::log::log;

pub struct Log {
}

impl Log {
    pub fn debug(msg: &str) {
        log(msg);
    }

    pub fn info(msg: &str) {
        log(msg);
    }

    pub fn warning(msg: &str) {
        log(msg);
    }

    pub fn error(msg: &str) {
        log(msg);
    }
}