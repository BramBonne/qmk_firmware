use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};

#[link(name = "board")]
extern "C" {
    fn timer_read() -> u16;
    fn timer_elapsed(since: u16) -> u16;
}

pub struct IdleState {
    half_minutes_since_last_keypress: AtomicU32,
    idle_timer: AtomicU16,
}

impl IdleState {
    pub const fn new() -> IdleState {
        IdleState {
            half_minutes_since_last_keypress: AtomicU32::new(0),
            idle_timer: AtomicU16::new(0),
        }
    }

    /// Needs to be called at least once every 30 seconds to return something useful.
    pub fn get_minutes_idle(&self) -> u32 {
        self.poll_idle();
        self.half_minutes_since_last_keypress
            .load(Ordering::Relaxed)
            / 2
    }

    pub fn reset(&self) {
        self.idle_timer
            .store(IdleState::get_time(), Ordering::Relaxed);
        self.half_minutes_since_last_keypress
            .store(0, Ordering::Relaxed);
    }

    fn poll_idle(&self) {
        if self.get_elapsed_since_timer() > 30000 {
            // No compare_exchange on cortex0, but that's OK because we run single-threaded.
            let half_minutes_since_last_keypress = self
                .half_minutes_since_last_keypress
                .load(Ordering::Relaxed)
                + 1;
            self.half_minutes_since_last_keypress
                .store(half_minutes_since_last_keypress, Ordering::Relaxed);
            self.idle_timer
                .store(IdleState::get_time(), Ordering::Relaxed);
        }
    }

    fn get_time() -> u16 {
        unsafe { timer_read() }
    }

    fn get_elapsed_since_timer(&self) -> u16 {
        unsafe { timer_elapsed(self.idle_timer.load(Ordering::Relaxed)) }
    }
}
