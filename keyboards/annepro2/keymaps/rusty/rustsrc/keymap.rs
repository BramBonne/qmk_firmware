use core::sync::atomic::{AtomicU8, Ordering};

use idle_state::IdleState;
use leds::{Color, Leds};

static BACKLIGHT_TIMEOUT_MINUTES: u32 = 1;
static DEFAULT_LED_PROFILE: i32 = 3;

static LEDS: Leds = Leds::new(DEFAULT_LED_PROFILE);
static IDLE_STATE: IdleState = IdleState::new();
static CURRENT_LAYER: AtomicU8 = AtomicU8::new(0);

// TODO: this will eventually contain the full keymap
enum Layer {
    Default,
    Movement,
    Emoji,
    Numpad,
    Unknown,
}

impl From<u8> for Layer {
    fn from(orig: u8) -> Self {
        match orig {
            0 => Layer::Default,
            1 => Layer::Movement,
            2 => Layer::Emoji,
            3 => Layer::Numpad,
            _ => Layer::Unknown,
        }
    }
}

#[no_mangle]
pub extern "C" fn matrix_init_user() {}

// Code to run after initializing the keyboard
#[no_mangle]
pub extern "C" fn keyboard_post_init_user() {}

#[no_mangle]
pub extern "C" fn matrix_scan_user() {}

#[no_mangle]
pub extern "C" fn process_keypress(keycode: i16) {
    if !LEDS.is_on() {
        LEDS.enable();
    }
    IDLE_STATE.reset();

    // TODO: Implement tmux navigation here
    /*match(keycode) {
        TMUX_DOWN => SEND_STRING(SS_LCTL("b") SS_TAP(X_DOWN))
        TMUX_UP => SEND_STRING(SS_LCTL("b") SS_TAP(X_UP))
        TMUX_LEFT => SEND_STRING(SS_LCTL("b") SS_TAP(X_LEFT))
        TMUX_RIGHT => SEND_STRING(SS_LCTL("b") SS_TAP(X_RIGHT))
    }*/
}

#[no_mangle]
pub extern "C" fn housekeeping_task_user() {
    if !LEDS.is_on() {
        return;
    }

    if IDLE_STATE.get_minutes_idle() >= BACKLIGHT_TIMEOUT_MINUTES {
        LEDS.disable();
    }
}

#[no_mangle]
pub extern "C" fn handle_led_keypress() {}

#[no_mangle]
pub extern "C" fn layer_changed(new_layer: u8) {
    CURRENT_LAYER.store(new_layer, Ordering::Relaxed);
    update_layer_color(new_layer.into());
}

#[no_mangle]
pub extern "C" fn caps_lock_changed(enabled: bool) {
    match enabled {
        true => LEDS.set_color(Color::Red),
        false => update_layer_color(CURRENT_LAYER.load(Ordering::Relaxed).into()),
    }
}

fn update_layer_color(layer: Layer) {
    match layer {
        Layer::Movement => LEDS.set_color(Color::Green),
        Layer::Emoji => LEDS.set_color(Color::Blue),
        Layer::Numpad => LEDS.set_color(Color::Orange),
        _ => LEDS.set_default_color(),
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
