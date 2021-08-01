use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

#[link(name = "board")]
extern "C" {
    fn annepro2LedEnable();
    fn annepro2LedDisable();
    fn annepro2LedSetProfile(profile: u8);
    fn annepro2LedSetForegroundColor(r: u8, g: u8, b: u8);
    fn annepro2LedResetForegroundColor();
    static annepro2LedStatus: LedStatus;
}

pub struct Leds {
    leds_on: AtomicBool,
    current_profile: AtomicU8,
}

pub enum Color {
    Green,
    Blue,
    Red,
    Orange,
}

#[repr(C)]
pub struct LedStatus {
    pub amount_of_profiles: u8,
    pub current_profile: u8,
    pub matrix_enabled: u8,
    pub is_reactive: u8,
    pub led_intensity: u8,
    pub errors: u8,
}

impl Leds {
    pub const fn new(default_profile: u8) -> Leds {
        Leds {
            leds_on: AtomicBool::new(false),
            current_profile: AtomicU8::new(default_profile),
        }
    }

    pub fn is_on(&self) -> bool {
        self.leds_on.load(Ordering::Relaxed)
    }

    pub fn enable(&self) {
        if self.is_on() {
            return;
        }
        unsafe {
            annepro2LedEnable();
            annepro2LedSetProfile(self.current_profile.load(Ordering::Relaxed));
        }
        self.leds_on.store(true, Ordering::Relaxed);
    }

    pub fn set_color(&self, color: Color) {
        let rgb = match color {
            Color::Green => (0, 0xff, 0),
            Color::Blue => (0, 0, 0xff),
            Color::Red => (0xff, 0, 0),
            Color::Orange => (0x88, 0x32, 0),
        };
        unsafe { annepro2LedSetForegroundColor(rgb.0, rgb.1, rgb.2) };
    }

    pub fn set_default_color(&self) {
        unsafe {
            annepro2LedResetForegroundColor();
        }
    }

    pub fn disable(&self) {
        // Store the current profile, as it might have changed outside of our purview
        let profile = unsafe { annepro2LedStatus.current_profile };
        unsafe {
            annepro2LedDisable();
        }
        self.current_profile.store(profile, Ordering::Relaxed);
        self.leds_on.store(false, Ordering::Relaxed);
    }
}
