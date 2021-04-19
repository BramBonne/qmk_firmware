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

// Backlight timeout feature
#define BACKLIGHT_TIMEOUT 5 // in minutes

// Key symbols are based on QMK. Use them to remap your keyboard
/*
* Layer _BASE_LAYER
* ,-----------------------------------------------------------------------------------------.
* | esc |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  0  |  -  |  =  |    Bksp   |
* |-----------------------------------------------------------------------------------------+
* | Tab    |  q  |  w  |  e  |  r  |  t  |  y  |  u  |  i  |  o  |  p  |  [  |  ]  |  FN2/\ |
* |-----------------------------------------------------------------------------------------+
* | F1/Caps |  a  |  s  |  d  |  f  |  g  |  h  |  j  |  k  |  l  |  ;  |  '  |    Enter    |
* |-----------------------------------------------------------------------------------------+
* | Shift      |  z  |  x  |  c  |  v  |  b  |  n  |  m  |  ,  |  .  |  /  |    Shift/up    |
* |-----------------------------------------------------------------------------------------+
* | Ctrl  | Super |  Alt  |               space             |Alt/DEL| left  |  down | right |
* \-----------------------------------------------------------------------------------------/
*/
 const uint16_t keymaps[][MATRIX_ROWS][MATRIX_COLS] = {
 [_BASE_LAYER] = KEYMAP( /* Base */
    KC_ESC, KC_1, KC_2, KC_3, KC_4, KC_5, KC_6, KC_7, KC_8, KC_9, KC_0, KC_MINS, KC_EQL, KC_BSPC,
    KC_TAB, KC_Q, KC_W, KC_E, KC_R, KC_T, KC_Y, KC_U, KC_I, KC_O, KC_P, KC_LBRC, KC_RBRC, LT(_FN2_LAYER,KC_BSLS),
    LT(_MOVEMENT_LAYER,KC_CAPS), KC_A, KC_S, KC_D, KC_F, KC_G, KC_H, KC_J, KC_K, KC_L, KC_SCLN, KC_QUOT, KC_ENT,
    KC_LSFT, KC_Z, KC_X, KC_C, KC_V, KC_B, KC_N, KC_M, KC_COMM, KC_DOT, KC_SLSH, RSFT_T(KC_UP),
    KC_LCTL, KC_LGUI, KC_LALT, KC_SPC, RALT_T(KC_DEL), KC_LEFT, KC_DOWN, KC_RGHT
),
  /*
  * Layer _MOVEMENT_LAYER
  * ,-----------------------------------------------------------------------------------------.
  * |  `  |  F1 |  F2 |  F3 |  F4 |  F5 |  F6 |  F7 |  F8 |  F9 | F10 | F11 | F12 |  DELETE   |
  * |-----------------------------------------------------------------------------------------+
  * | numpad |  q  | UP  |  e  |  r  |  t  |  y  | PGDN| PGUP  | PGDN | PS | HOME | END |  \  |
  * |-----------------------------------------------------------------------------------------+
  * | Esc     |LEFT |DOWN |RIGHT|  f  |  g  | LEFT| DOWN| UP  |RIGHT| PGUP|PGDN |    Enter    |
  * |-----------------------------------------------------------------------------------------+
  * | Shift      |MUTE|V-DWN|V-UP | prev | pause | next | HOME| END |INSRT| DEL |    tmux_up  |
  * |-----------------------------------------------------------------------------------------+
  * | Ctrl  | Super |  Alt  |               space             |Alt/DEL|t_left | t_down|t_right|
  * \-----------------------------------------------------------------------------------------/
  *
  */
 [_MOVEMENT_LAYER] = KEYMAP( /* Base */
    KC_GRV, KC_F1, KC_F2, KC_F3, KC_F4, KC_F5, KC_F6, KC_F7, KC_F8, KC_F9, KC_F10, KC_F11, KC_F12, KC_DEL,
    // Allow quick numpad access while holding, or full numpad lock
    TT(_NUMPAD_LAYER), __, KC_UP, __, __, __, __, KC_PGDN, KC_PGUP, KC_PGDN, KC_PSCR, KC_HOME, KC_END, __,
    __, KC_LEFT, KC_DOWN, KC_RGHT, __, __, KC_LEFT, KC_DOWN, KC_UP, KC_RGHT, KC_PGUP, KC_PGDN, __,
    __, KC_MUTE, KC_VOLD, KC_VOLU, KC_MEDIA_PREV_TRACK, KC_MEDIA_PLAY_PAUSE, KC_MEDIA_NEXT_TRACK, KC_HOME, KC_END, KC_INS, KC_DEL,TMUX_UP,
    __, __, __, __, __, TMUX_LEFT, TMUX_DOWN, TMUX_RIGHT
),
/*
  * Layer _FN2_LAYER
  * ,-----------------------------------------------------------------------------------------.
  * |  _  | BT1 | BT2 | BT3 | BT4 | USB |UNPAIR|LEDON|LEDOFF|🙁| 🙂😊 | LEDSPEED | LEDINT | _ |
  * |-----------------------------------------------------------------------------------------+
  * | _      |  _  | _  |  _  |  _  |  _  |  _  |  👍  |  _   | _   |😮😳|  😛 |  _  | FN2    |
  * |-----------------------------------------------------------------------------------------+
  * | _       |  _  |  _  |  😄 |  _  |  _  | _  |  _  | 😚😘 |  _ | 😉😶 |  😅  |     _      |
  * |-----------------------------------------------------------------------------------------+
  * | _         | _  | _  |  _  |  _   |  😎  |  _ |  _  |  _  |   _  |  😕  |           _    |
  * |-----------------------------------------------------------------------------------------+
  * | Ctrl  | Super |  Alt  |               space             |Alt/DEL| left  |  down | right |
  * \-----------------------------------------------------------------------------------------/
  *
  */
 [_FN2_LAYER] = KEYMAP( /* Base */
    __, KC_AP2_BT1, KC_AP2_BT2, KC_AP2_BT3, KC_AP2_BT4, KC_AP2_USB, KC_AP2_BT_UNPAIR, KC_AP_LED_ON, KC_AP_LED_OFF, X(SAD), XP(SLIGHT_SMILE, SMILE_BLUSH), KC_AP_LED_SPEED, KC_AP_LED_NEXT_INTENSITY, __,
    __, __, __, __, __, __, X(THUMBS), UNICODE_MODE_FORWARD, __, XP(OPEN_MOUTH, FLUSHED), X(TONGUE), __, __, __,
    __, __, __, X(LAUGH), __, __, __, __, XP(KISS, KISS_HEART), __, XP(WINK, MOUTHLESS), X(SWEAT), __,
    __, __, __, __, __, X(SUNGLASSES), __, __, __, __, X(CONFUSED), __,
    __, __, __, __, __, __, __, __
 ),
 [_NUMPAD_LAYER] = KEYMAP(
   __, __, __, __, __, __, __, __, KC_ASTR, __, __, KC_MINS, KC_EQL, __,
   __, __, __, __, __, __, KC_7, KC_8, KC_9, KC_PLUS, __, __, __, __,
   __, __, __, __, __, __, KC_4, KC_5, KC_6, __, __, __, __,
   __, __, __, __, __, __, KC_1, KC_2, KC_3, __, KC_SLSH, __,
   __, __, __, KC_0, KC_DOT, __, __, __
 )
};
const uint16_t keymaps_size = sizeof(keymaps);

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



uint8_t led_animation_id = 0;
