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
void suspend_power_down_user(void);
void suspend_wakeup_init_user(void);
void process_keypress(uint16_t);
void layer_changed(uint8_t);
void caps_lock_changed(bool);

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
