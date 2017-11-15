use sdl2::sys::timer;
use sdl2;

pub struct Timer {
    start_mark: u32,
    stop_mark: u32,
    paused_mark: u32,
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
        unsafe {
            self.start_mark = sdl2::sys::timer::SDL_GetTicks();
        }
        self.stop_mark = 0;
        self.paused_mark = 0;
        self.running = true;
        self.paused = false;
    }

    pub fn stop(&mut self) {
        if !self.running {
            return;
        }
        unsafe {
            self.stop_mark = sdl2::sys::timer::SDL_GetTicks();
        }
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
        unsafe {
            self.paused_mark = sdl2::sys::timer::SDL_GetTicks() - self.start_mark;
        }
        self.running = false;
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        if self.running || !self.paused {
            return;
        }
        unsafe {
            self.start_mark = sdl2::sys::timer::SDL_GetTicks() - self.paused_mark;
        }
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

    pub fn delta(&mut self) -> u32 {
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

    pub fn delta_ms(&mut self) -> u32 {
        self.delta() % 1000
    }
        
    pub fn delta_s(&mut self) -> u32 {
        self.delta() / 1000
    }

    pub fn current_time(&mut self) -> u32 {
        unsafe {
            return sdl2::sys::timer::SDL_GetTicks() - self.start_mark;
        }
    }

}