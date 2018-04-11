use sdl2::sys::timer;
use sdl2;
use time;

pub struct Timer {
    start_mark: u64,
    stop_mark: u64,
    paused_mark: u64,
    running: bool,
    paused: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            start_mark: 0,
            stop_mark: 0,
            paused_mark: 0,
            running: false,
            paused: false,
        }
    }

    pub fn start(&mut self) {
        if self.running {
            return;
        }
        self.start_mark = time::precise_time_ns();
        /*
        unsafe {
            self.start_mark = sdl2::sys::timer::SDL_GetTicks();
        }
        */
        self.stop_mark = 0;
        self.paused_mark = 0;
        self.running = true;
        self.paused = false;
    }

    pub fn stop(&mut self) {
        if !self.running {
            return;
        }
        self.stop_mark = time::precise_time_ns();
        /*
        unsafe {
            self.stop_mark = sdl2::sys::timer::SDL_GetTicks();
        }
        */
        self.running = false;
        self.paused = false;
    }

    pub fn restart(&mut self) {
        self.stop();
        self.start();
    }

    pub fn pause(&mut self) {
        if !self.running || self.paused {
            return;
        }
        self.paused_mark = time::precise_time_ns() - self.start_mark;
        /*
        unsafe {
            self.paused_mark = sdl2::sys::timer::SDL_GetTicks() - self.start_mark;
        }
        */
        self.running = false;
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        if self.running || !self.paused {
            return;
        }
        self.start_mark = time::precise_time_ns() - self.paused_mark;
        /*
        unsafe {
            self.start_mark = sdl2::sys::timer::SDL_GetTicks() - self.paused_mark;
        }
        */
        self.paused_mark = 0;
        self.running = true;
        self.paused = false;
    }
    
    pub fn is_running(&mut self) -> bool {
        self.running
    }

    pub fn is_paused(&mut self) -> bool {
        self.paused
    }

    pub fn delta(&mut self) -> u64 {
        if self.running {
            return self.current_time();
        }

        if self.paused {
            return self.paused_mark;
        }

        if self.start_mark == 0 {
            return 0;
        }

        return self.stop_mark - self.start_mark;
    }

    pub fn delta_ms(&mut self) -> f64 {
        self.delta() as f64 / 1_000_000.0
    }
        
    pub fn delta_s(&mut self) -> f64 {
        self.delta() as f64 / 1_000_000_000.0
    }

    pub fn current_time(&mut self) -> u64 {
        return time::precise_time_ns() - self.start_mark;
        /*
        unsafe {
            return sdl2::sys::timer::SDL_GetTicks() - self.start_mark;
        }
        */
    }

}