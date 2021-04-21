#include <stdint.h>
#include <stdint.h>
#include "annepro2.h"
#include "qmk_ap2_led.h"
#include "config.h"

#define __ KC_TRNS

// Implemented in keymap.rs
void matrix_init_user(void);
void keyboard_post_init_user(void);
void matrix_scan_user(void);
void housekeeping_task_user(void);
void process_keypress(uint16_t);
void layer_changed(uint8_t);
void caps_lock_changed(bool);

enum anne_pro_layers {
  _BASE_LAYER,
  _MOVEMENT_LAYER,
  _FN2_LAYER,
  _NUMPAD_LAYER,
};

enum custom_keycodes {
  TMUX_DOWN = AP2_SAFE_RANGE,
  TMUX_UP,
  TMUX_LEFT,
  TMUX_RIGHT
};

enum unicode_names {
    LAUGH,
    SWEAT,
    SLIGHT_SMILE,
    SMILE_BLUSH,
    CONFUSED,
    OPEN_MOUTH,
    FLUSHED,
    TONGUE,
    SAD,
    THUMBS,
    WINK,
    MOUTHLESS,
    KISS,
    KISS_HEART,
    SUNGLASSES
};

const uint32_t PROGMEM unicode_map[] = {
    [LAUGH]  = 0x1F604,
    [SWEAT] = 0x1F605,
    [SLIGHT_SMILE]  = 0x1F642,
    [SMILE_BLUSH] = 0x1F60A,
    [CONFUSED] = 0x1F615,
    [OPEN_MOUTH] = 0x1F62E,
    [FLUSHED] = 0x1F633,
    [TONGUE] = 0x1F61B,
    [SAD] = 0x1F641,
    [THUMBS] = 0x1F44D,
    [WINK] = 0x1F609,
    [MOUTHLESS] = 0x1F636,
    [KISS] = 0x1F61A,
    [KISS_HEART] = 0x1F618,
    [SUNGLASSES] = 0x1F60E,
};

bool process_record_user(uint16_t keycode, keyrecord_t *record) {
  if (record->event.pressed) {
      process_keypress(keycode);
  }

  return true;
}

layer_state_t layer_state_set_user(layer_state_t layer) {
    layer_changed(get_highest_layer(layer));

    return layer;
}

bool led_update_user(led_t leds) {
    caps_lock_changed(leds.caps_lock);

    return true;
}
