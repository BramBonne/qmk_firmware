use keycodes::*;

#[link(name = "board")]
extern "C" {
    fn send_string(string: *const u8);
    fn send_unicode_string(string: *const u8);
    fn register_code(keycode: u16);
    fn unregister_code(keycode: u16);
    fn tap_code(keycode: u16);
}

pub fn emoji_keycode_to_codepoint(keycode: i16) -> Option<&'static [u8]> {
    match keycode {
        keycode if keycode == EmojiKeyCodes::SMILE as i16 => Some(b"1f642\0"),
        keycode if keycode == EmojiKeyCodes::SAD as i16 => Some(b"1f641\0"),
        keycode if keycode == EmojiKeyCodes::THUMB as i16 => Some(b"1f44d\0"),
        _ => None
    }
}

pub fn send_emoji(codepoint: &[u8]) {
    unsafe {
        register_code(KC_LCTL);
        register_code(KC_LSFT);
        send_string(b"u\0".as_ptr() as *const u8);
        unregister_code(KC_LCTL);
        unregister_code(KC_LSFT);
        send_string(codepoint.as_ptr() as *const u8);
        tap_code(KC_ENTER);
    }
}
