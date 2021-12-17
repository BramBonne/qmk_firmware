// Largely copied from https://github.com/houqp/qmk_firmware/blob/massdrop_houqp_rust/rust/src/macros.rs

#[macro_export]
macro_rules! pad {
    ( $x:ident ) => {
        kb!(NO)
    };
}

#[macro_export]
macro_rules! keycode {
    (,) => {
        KC_COMMA
    };
    (.) => {
        KC_DOT
    };
    (-) => {
        KC_MINUS
    };
    (=) => {
        KC_EQUAL
    };
    (;) => {
        KC_SCLN
    };
    (/) => {
        KC_SLSH
    };
    (!) => {
        KC_EXLM
    };
    (@) => {
        KC_AT
    };
    (#) => {
        KC_HASH
    };
    ($) => {
        KC_DLR
    };
    (%) => {
        KC_PERC
    };
    (^) => {
        KC_CIRC
    };
    (&) => {
        KC_AMPR
    };
    (*) => {
        KC_ASTR
    };
    (+) => {
        KC_PLUS
    };
    ('[') => {
        KC_LBRC
    };
    (']') => {
        KC_RBRC
    };
    ('(') => {
        KC_LPRN
    };
    (')') => {
        KC_RPRN
    };
    (?) => {
        KC_QUEST
    };
    ('⏎') => {
        KC_ENTER
    };
    ('↑') => {
        KC_UP
    };
    ('→') => {
        KC_RIGHT
    };
    ('↓') => {
        KC_DOWN
    };
    ('←') => {
        KC_LEFT
    };
    ('⤒') => {
        KC_MS_WH_UP
    };
    ('⤞') => {
        KC_MS_WH_RIGHT
    };
    ('⤓') => {
        KC_MS_WH_DOWN
    };
    ('⤝') => {
        KC_MS_WH_LEFT
    };
    ('↹') => {
        KC_TAB
    };
    ('`') => {
        KC_GRV
    };
    ('🔊') => {
        KC_VOLU
    };
    ('🔉') => {
        KC_VOLD
    };
    ('🔇') => {
        KC_MUTE
    };
    ("⏭️") => {
        KC_NXTRK
    };
    ("⏯️") => {
        KC_PAUS
    };
    ("⏮️") => {
        KC_PRVTRK
    };
    ("🙂") => {
        EmojiKeyCodes::SMILE as u16
    };
    ("😊") => {EmojiKeyCodes::SMILE_BLUSH as u16};
    ("🙁") => {EmojiKeyCodes::SAD as u16};
    ("😮") => {EmojiKeyCodes::SURPRISED as u16};
    ("😄") => {EmojiKeyCodes::LAUGH as u16};
    ("😅") => {EmojiKeyCodes::SWEAT as u16};
    ("😚") => {EmojiKeyCodes::KISS as u16};
    ("😉") => {EmojiKeyCodes::WINK as u16};
    ("😎") => {EmojiKeyCodes::SUNGLASSES as u16};
    ("😕") => {EmojiKeyCodes::CONFUSED as u16};
    ("😛") => {EmojiKeyCodes::TONGUE as u16};
    ("👍") => {EmojiKeyCodes::THUMB as u16};
    ("™") => {EmojiKeyCodes::TM as u16};
    ([ ]) => {
        KC__TODO_
    };
    (_) => {
        KC__TODO_
    };

    ([$key:ident&LT{$layernu:literal}]) => {
        LT!($layernu, keycode!($key))
    };
    ([$key:literal&LT{$layernu:literal}]) => {
        LT!($layernu, keycode!($key))
    };
    ([MO{$layernu:literal}]) => {
        MO!($layernu)
    };
    ([TT{$layernu:literal}]) => {
        TT!($layernu)
    };
    ([$key:ident&MT{$mod:expr}]) => {
        MT!(paste::expr! { [<MOD_ $mod>] }, keycode!($key))
    };
    ($key:literal) => {
        paste::expr! { [<KC_ $key>] }
    };
    ($key:ident) => {
        paste::expr! { [<KC_ $key>] }
    };
    ($key:expr) => {
        $key
    };
    ($key:ident&LT($layernu:literal)) => {
        LT!($layernu, paste::expr! { [<KC_ $key>] })
    };
}

#[macro_export]
macro_rules! r {
    ( $( $key:tt )|+ ) => {
        [ $( keycode!($key), )* ]
    }
}

#[macro_export]
macro_rules! layer {
    ( $( $row:expr ),+ $(,)?) => {
        [ $( $row, )* ]
    }
}

#[macro_export]
macro_rules! keymaps {
    (
        rows => $rows: ident,
        cols => $cols: ident,
        layer_cnt => $layer_cnt: literal,
        $( $layer:expr ),+ $(,)?
    ) => {
        #[no_mangle]
        static keymaps: [[[u16; $cols]; $rows]; $layer_cnt] = [
            $( $layer, )*
        ];
    }
}
