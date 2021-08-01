use core::sync::atomic::{AtomicU8, Ordering};

use emoji::*;
use idle_state::IdleState;
use keycodes::*;
use leds::{Color, Leds};
use *;

const BACKLIGHT_TIMEOUT_MINUTES: u32 = 5;
const DEFAULT_LED_PROFILE: u8 = 4;
const MATRIX_ROWS: usize = 5;
const MATRIX_COLS: usize = 14;

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

keymaps!(
    rows => MATRIX_ROWS,
    cols => MATRIX_COLS,
    layer_cnt => 4,
    layer!(  // Default layer
        r!(ESC | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 0 | - | = | BSPC),
        r!( '↹' | Q | W  | E | R | T | Y | U | I | O | P | '[' |']' |[BSLS&LT{2}]),
  r!([CAPS&LT{1}]| A | S | D | F | G | H | J | K | L | ; |QUOTE| '⏎' |_),
        r!( LSFT | _ | Z | X | C | V | B | N | M | , | . | / |[UP&MT{RSFT}]|_),
        r!(LCTL|_|LGUI| LALT |_| _ | SPACE | _ | _ | [DEL&MT{RALT}] |'←'|'↓'|'→'|_),
    ),
    layer!(  // Movement
        r!('`' | F1| F2| F3| F4| F5| F6| F7| F8| F9|F10|F11|F12| BSPC),
        r!( [TT{3}] | _ | _ | _ | _ | _ | _ |PGDN|PGUP|PGDN|PSCR|HOME|END| _ ),
        r!( [  ]  | _ | _ | _ | _ | _ |'←'|'↓'|'↑'|'→'|PGUP|PGDN| _ |_),
        r!( [   ] | _ |'🔇'|'🔉'|'🔊'|"⏮️"|"⏯️"|"⏭️"| _ | _ | _ | _ | _ |_),
        r!( _ |_| _ | _ |_| _ | _ | _ | _ | _ | _ | _ | _ |_),
    ),
    layer!(  // System / Emoji
        r!( _ |BT1|BT2|BT3|BT4|USB|BTU|LEDON|LEDOFF|"🙁"|"🙂"|"😊"| _ | BSPC),
        r!( [  ] | _ | _ | _ | _ |"™"|"👍"| _ | _ |"😮"|"😛"| _ | _ | _ ),
        r!( [  ]  | _ | _ |"😄"| _ | _ | _ | _ |"😚"| _ |"😉"|"😅"| _ |_),
        r!( [   ] | _ | _ | _ | _ | _ |"😎"| _ | _ | _ | _ |"😕"| _ |_),
        r!( _ |_| _ | _ |_| _ | _ | _ | _ | _ | _ | _ | _ |_),
    ),
    layer!(  // NumPad
        r!( _ | _ | _ | _ | _ | _ | _ | _ | _ | _ | _ | - | = | _ ),
        r!(  _  | _ | _ | _ | _ | _ | 7 | 8 | 9 | + | _ | _ | _ | _ ),
        r!( [  ]  | _ | _ | _ | _ | _ | 4 | 5 | 6 | _ | _ | _ | _ |_),
        r!( _  | _ | _ | _ | _ | _ | _ | 1 | 2 | 3 | _ | _ | / | _ ),
        r!( _ |_| _ | _ |_| _ |    0    | _ | _ | . | _ | _ | _ |_),
    ),
);

#[no_mangle]
pub extern "C" fn matrix_init_user() {}

// Code to run after initializing the keyboard
#[no_mangle]
pub extern "C" fn keyboard_post_init_user() {
    LEDS.enable();
    IDLE_STATE.reset();
}

#[no_mangle]
pub extern "C" fn matrix_scan_user() {}

#[no_mangle]
pub extern "C" fn process_keypress(keycode: i16) {
    if !LEDS.is_on() {
        LEDS.enable();
    }
    IDLE_STATE.reset();

    if let Some(codepoint) = emoji_keycode_to_codepoint(keycode) {
        send_emoji(codepoint);
    }
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
pub extern "C" fn suspend_power_down_user() {
    LEDS.disable();
}

#[no_mangle]
pub extern "C" fn suspend_wakeup_init_user() {
    LEDS.enable();
    IDLE_STATE.reset();
}

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
