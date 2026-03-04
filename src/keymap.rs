use rmk::combo::{Combo, ComboConfig};
use rmk::fork::{Fork, StateBits};
use rmk::heapless::Vec;
use rmk::types::action::{Action, KeyAction, KeyboardAction};
use rmk::types::led_indicator::LedIndicator;
use rmk::types::modifier::ModifierCombination;
use rmk::types::mouse_button::MouseButtons;
use rmk::{a, df, k, layer, lt, mt, osm, shifted, wm};

use crate::{amt, kwm};

pub(crate) const COL: usize = 10;
pub(crate) const COL_PER_PART: usize = 5;
pub(crate) const ROW: usize = 4;
pub(crate) const SIZE: usize = 20;
pub(crate) const NUM_LAYER: usize = 17;
pub(crate) const COMBO_MAX_NUM: usize = 8;

const T: KeyAction = a!(Transparent);
const CAPS_WORD: KeyAction = KeyAction::Single(Action::KeyboardControl(KeyboardAction::CapsWordToggle));

const DE_CARET: KeyAction = k!(Grave);
const DE_STAR: KeyAction = shifted!(RightBracket);
const DE_AND: KeyAction = shifted!(Kc6);
const DE_HASHTAG: KeyAction = k!(Backslash);
const DE_GRAVE: KeyAction = wm!(RightBracket, ModifierCombination::RALT);
const DE_UNDERSCORE: KeyAction = shifted!(Slash);
const DE_PIPE: KeyAction = wm!(NonusBackslash, ModifierCombination::RALT);
const DE_APOSTROPHE: KeyAction = shifted!(Backslash);
const DE_SLASH: KeyAction = shifted!(Kc7);
const DE_QUOTATION: KeyAction = shifted!(Kc2);
const DE_MINUS: KeyAction = k!(Slash);
const DE_BACKSLASH: KeyAction = wm!(Minus, ModifierCombination::RALT);
const DE_BACKTICK: KeyAction = shifted!(Equal);
const DE_COLON: KeyAction = shifted!(Dot);
const DE_LESS_THAN: KeyAction = k!(NonusBackslash);
const DE_GREATHER_THAN: KeyAction = shifted!(NonusBackslash);
const DE_SEMICOLON: KeyAction = shifted!(Comma);
const DE_LEFT_BRACE: KeyAction = wm!(Kc7, ModifierCombination::RALT);
const DE_RIGHT_BRACE: KeyAction = wm!(Kc0, ModifierCombination::RALT);
const DE_LEFT_PARANTHESIS: KeyAction = shifted!(Kc8);
const DE_RIGHT_PARANTHESIS: KeyAction = shifted!(Kc9);
const DE_AT: KeyAction = wm!(Q, ModifierCombination::RALT);
const DE_EXCLAMATION: KeyAction = shifted!(Kc1);
const DE_LEFT_BRACKET: KeyAction = wm!(Kc8, ModifierCombination::RALT);
const DE_RIGHT_BRACKET: KeyAction = wm!(Kc9, ModifierCombination::RALT);
const DE_EQUAL: KeyAction = shifted!(Kc0);
const DE_PLUS: KeyAction = k!(RightBracket);
const DE_PERCENT: KeyAction = shifted!(Kc5);

const MAC_PIPE: KeyAction = wm!(Kc7, ModifierCombination::LALT);
const MAC_GRAVE: KeyAction = wm!(N, ModifierCombination::LALT);
const MAC_BACKSLASH: KeyAction = wm!(Kc7, ModifierCombination::new().with_left_alt(true).with_left_shift(true));
const MAC_LEFT_BRACE: KeyAction = wm!(Kc8, ModifierCombination::LALT);
const MAC_RIGHT_BRACE: KeyAction = wm!(Kc9, ModifierCombination::LALT);
const MAC_LEFT_BRACKET: KeyAction = wm!(Kc5, ModifierCombination::LALT);
const MAC_RIGHT_BRACKET: KeyAction = wm!(Kc6, ModifierCombination::LALT);
const MAC_AT: KeyAction = wm!(L, ModifierCombination::LALT);

const FORK_0: KeyAction = k!(F13); // Fork #0 -> ;/:
const FORK_1: KeyAction = k!(F14); // Fork #1 -> ,/<
const FORK_2: KeyAction = k!(F15); // Fork #2 -> ./>
const FORK_3: KeyAction = k!(F16); // Fork #3 -> //?
const FORK_4: KeyAction = k!(F17); // Fork #4 -> Mac - ./>

/*
Empty layer for copying

        layer!([
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, T, T, T, T, T, T, T],
            [T, T, T, T, T, T, T, T, T, T]
        ]),
*/

#[rustfmt::skip]
#[allow(unused)]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // 0: Base Layer
        layer!([
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Z), k!(U), k!(I), k!(O), k!(P)],
            [mt!(A, ModifierCombination::LSHIFT), lt!(5, S), lt!(1, D), lt!(3, F), k!(G), k!(H), lt!(4, J), lt!(2, K), lt!(6, L), FORK_0],
            [k!(Y), mt!(X, ModifierCombination::LCTRL), mt!(C, ModifierCombination::LALT), k!(V), k!(B), k!(N), k!(M), FORK_1, FORK_2, FORK_3],
            [a!(No), a!(No), a!(No), k!(Backspace), k!(Delete), lt!(7, Enter), k!(Space), a!(No), a!(No), a!(No)]
        ]),
        // 1: Mouse
        layer!([
            [T, T, T, T, T, T, k!(MouseBtn1), k!(MouseWheelUp), k!(MouseBtn2), T],
            [T, k!(MouseBtn2), T, k!(MouseBtn1), T, T, k!(MouseLeft), k!(MouseDown), k!(MouseUp), k!(MouseRight)],
            [T, T, T, T, T, T, k!(MouseWheelLeft), k!(MouseWheelDown), k!(MouseWheelRight), T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 2: Navigation
        layer!([
            [T, T, k!(PageUp), T, T, T, T, T, T, T],
            [k!(Left), k!(Up), k!(Down), k!(Right), T, T, k!(LGui), T, wm!(LCtrl, ModifierCombination::LALT), wm!(LShift, ModifierCombination::from_bits(5))],
            [T, k!(Home), k!(PageDown), k!(End), T, T, T, T, T, T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 3: Right Symbols
        layer!([
            [T, T, T, T, T, T, DE_UNDERSCORE, DE_PIPE, DE_APOSTROPHE, T],
            [DE_CARET, DE_STAR, DE_AND, T, T, DE_HASHTAG, DE_GRAVE, DE_SLASH, DE_QUOTATION, shifted!(Kc4)],
            [T, T, T, T, T, T, DE_MINUS, DE_BACKSLASH, DE_BACKTICK, T],
            [a!(No), a!(No), a!(No), k!(BrightnessDown), T, T, k!(BrightnessUp), a!(No), a!(No), a!(No)]
        ]),
        // 4: Left Symbols
        layer!([
            [T, DE_COLON, DE_LESS_THAN, DE_GREATHER_THAN, DE_SEMICOLON, T, T, T, T, T],
            [DE_LEFT_BRACE, DE_RIGHT_BRACE, DE_LEFT_PARANTHESIS, DE_RIGHT_PARANTHESIS, DE_AT, T, T, DE_EQUAL, DE_PLUS, DE_PERCENT],
            [T, DE_EXCLAMATION, DE_LEFT_BRACKET, DE_RIGHT_BRACKET, T, T, T, T, T, T],
            [a!(No), a!(No), a!(No), k!(AudioVolDown), T, T, k!(AudioVolUp), a!(No), a!(No), a!(No)]
        ]),
        // 5: Function Keys
        layer!([
            [T, T, T, T, T, T, k!(F7), k!(F8), k!(F9), k!(F10)],
            [T, T, wm!(LAlt, ModifierCombination::LCTRL), T, T, T, k!(F4), k!(F5), k!(F6), k!(F11)],
            [T, T, T, T, T, T, k!(F1), k!(F2), k!(F3), k!(F12)],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 6: Numbers
        layer!([
            [DE_SLASH, k!(Kc7), k!(Kc8), k!(Kc9), DE_PLUS, T, T, T, T, T],
            [k!(Kc0), k!(Kc4), k!(Kc5), k!(Kc6), DE_MINUS, T, T, T, T, T],
            [DE_STAR, k!(Kc1), k!(Kc2), k!(Kc3), DE_EQUAL, T, T, T, T, T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 7: Always accessible
        layer!([
            [T, T, DE_COLON, k!(Escape), T, df!(16), T, T, T, T],
            [T, DE_PERCENT, FORK_3, k!(Enter), T, df!(0), k!(LGui), osm!(ModifierCombination::LGUI), T, T],
            [T, T, CAPS_WORD, k!(CapsLock), T, df!(8), T, mt!(Comma, ModifierCombination::RALT), mt!(Dot, ModifierCombination::RCTRL), T],
            [a!(No), a!(No), a!(No), T, k!(Tab), T, T, a!(No), a!(No), a!(No)]
        ]),
        // 8: Mac - Base Layer
        layer!([
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Z), k!(U), k!(I), k!(O), k!(P)],
            [mt!(A, ModifierCombination::LSHIFT), lt!(13, S), lt!(9, D), lt!(11, F), k!(G), k!(H), lt!(12, J), lt!(10, K), lt!(14, L), FORK_0],
            [k!(Y), mt!(X, ModifierCombination::LGUI), mt!(C, ModifierCombination::LALT), k!(V), k!(B), k!(N), k!(M), FORK_1, FORK_4, FORK_3],
            [a!(No), a!(No), a!(No), k!(Backspace), k!(Delete), lt!(15, Enter), k!(Space), a!(No), a!(No), a!(No)]
        ]),
        // 9: Mac - Mouse
        layer!([
            [T, T, T, T, T, T, k!(MouseBtn1), k!(MouseWheelUp), k!(MouseBtn2), T],
            [T, k!(MouseBtn2), T, k!(MouseBtn1), T, T, k!(MouseLeft), k!(MouseDown), k!(MouseUp), k!(MouseRight)],
            [T, T, T, T, T, T, k!(MouseWheelLeft), k!(MouseWheelDown), k!(MouseWheelRight), T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 10: Mac - Navigation
        layer!([
            [T, T, k!(PageUp), T, T, T, T, T, T, T],
            [k!(Left), k!(Up), k!(Down), k!(Right), T, T, k!(LCtrl), T, wm!(LGui, ModifierCombination::LALT), wm!(LShift, ModifierCombination::from_bits(5))],
            [T, k!(Home), k!(PageDown), k!(End), T, T, T, T, T, T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 11: Mac - Right Symbols
        layer!([
            [T, T, T, T, T, T, DE_UNDERSCORE, MAC_PIPE, DE_APOSTROPHE, T],
            [DE_CARET, DE_STAR, DE_AND, T, T, DE_HASHTAG, MAC_GRAVE, DE_SLASH, DE_QUOTATION, shifted!(Kc4)],
            [T, T, T, T, T, T, DE_MINUS, MAC_BACKSLASH, DE_BACKTICK, T],
            [a!(No), a!(No), a!(No), k!(BrightnessDown), T, T, k!(BrightnessUp), a!(No), a!(No), a!(No)]
        ]),
        // 12: Mac - Left Symbols
        layer!([
            [T, DE_COLON, DE_LESS_THAN, DE_GREATHER_THAN, DE_SEMICOLON, T, T, T, T, T],
            [MAC_LEFT_BRACE, MAC_RIGHT_BRACE, DE_LEFT_PARANTHESIS, DE_RIGHT_PARANTHESIS, MAC_AT, T, T, DE_EQUAL, DE_PLUS, DE_PERCENT],
            [T, DE_EXCLAMATION, MAC_LEFT_BRACKET, MAC_RIGHT_BRACKET, T, T, T, T, T, T],
            [a!(No), a!(No), a!(No), k!(AudioVolDown), T, T, k!(AudioVolUp), a!(No), a!(No), a!(No)]
        ]),
        // 13: Mac - Function Keys
        layer!([
            [T, T, T, T, T, T, k!(F7), k!(F8), k!(F9), k!(F10)],
            [T, T, wm!(LAlt, ModifierCombination::LGUI), T, T, T, k!(F4), k!(F5), k!(F6), k!(F11)],
            [T, T, T, T, T, T, k!(F1), k!(F2), k!(F3), k!(F12)],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 14: Mac - Numbers
        layer!([
            [DE_SLASH, k!(Kc7), k!(Kc8), k!(Kc9), DE_PLUS, T, T, T, T, T],
            [k!(Kc0), k!(Kc4), k!(Kc5), k!(Kc6), DE_MINUS, T, T, T, T, T],
            [DE_STAR, k!(Kc1), k!(Kc2), k!(Kc3), DE_EQUAL, T, T, T, T, T],
            [a!(No), a!(No), a!(No), T, T, T, T, a!(No), a!(No), a!(No)]
        ]),
        // 15: Mac - Always accessible
        layer!([
            [T, T, DE_COLON, k!(Escape), T, df!(16), T, T, T, k!(Delete)],
            [T, DE_PERCENT, FORK_3, k!(Enter), T, df!(0), k!(LCtrl), osm!(ModifierCombination::LCTRL), T, T],
            [T, T, CAPS_WORD, k!(CapsLock), T, df!(8), T, mt!(Comma, ModifierCombination::RALT), mt!(Dot, ModifierCombination::RGUI), T],
            [a!(No), a!(No), a!(No), T, k!(Tab), T, T, a!(No), a!(No), a!(No)]
        ]),
        // 16: Gaming
        layer!([
            [k!(Tab), k!(Q), k!(E), k!(R), k!(T), df!(16), T, T, k!(AudioVolDown), k!(AudioVolUp)],
            [k!(LShift), k!(A), k!(W), k!(D), k!(F), df!(0), k!(Left), k!(Up), k!(Right), T],
            [k!(LCtrl), k!(Z), k!(X), k!(S), k!(V), df!(8), T, k!(Down), T, T],
            [a!(No), a!(No), a!(No), k!(LAlt), k!(Space), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
    ]
}

#[allow(unused)]
pub fn get_forks() -> Vec<Fork, 8> {
    let mut vec = Vec::new();
    
    vec.push(Fork::new(FORK_0, // ;/:
        amt!(kwm!(Comma, ModifierCombination::LSHIFT), ModifierCombination::RSHIFT), 
        amt!(kwm!(Dot, ModifierCombination::LSHIFT), ModifierCombination::RSHIFT), 
        StateBits::new_from(ModifierCombination::LSHIFT, LedIndicator::default(), MouseButtons::default()),
        StateBits::default(), ModifierCombination::default(), false)
    ).expect("too many items in vec");
    vec.push(Fork::new(FORK_1, // ,/<
        mt!(Comma, ModifierCombination::RALT), 
        mt!(NonusBackslash, ModifierCombination::RALT), 
        StateBits::new_from(ModifierCombination::LSHIFT, LedIndicator::default(), MouseButtons::default()),
        StateBits::default(), ModifierCombination::default(), false)
    ).expect("too many items in vec");
    vec.push(Fork::new(FORK_2, // ./>
        mt!(Dot, ModifierCombination::RCTRL), 
        amt!(kwm!(NonusBackslash, ModifierCombination::LSHIFT), ModifierCombination::RCTRL), 
        StateBits::new_from(ModifierCombination::LSHIFT, LedIndicator::default(), MouseButtons::default()),
        StateBits::default(), ModifierCombination::default(), false)
    ).expect("too many items in vec");
    vec.push(Fork::new(FORK_3, // //?
        shifted!(Kc7), 
        shifted!(Minus), 
        StateBits::new_from(ModifierCombination::LSHIFT, LedIndicator::default(), MouseButtons::default()),
        StateBits::default(), ModifierCombination::default(), false)
    ).expect("too many items in vec");
    vec.push(Fork::new(FORK_4, // Mac - ./>
        mt!(Dot, ModifierCombination::RGUI), 
        amt!(kwm!(NonusBackslash, ModifierCombination::LSHIFT), ModifierCombination::RGUI), 
        StateBits::new_from(ModifierCombination::LSHIFT, LedIndicator::default(), MouseButtons::default()),
        StateBits::default(), ModifierCombination::default(), false)
    ).expect("too many items in vec");

    vec
}

#[allow(unused)]
pub fn get_combos() -> [Option<Combo>; COMBO_MAX_NUM] {
    [
        Some(Combo::new(ComboConfig::new( // A+E -> Ä
            [mt!(A, ModifierCombination::LSHIFT), k!(E)], 
            k!(Quote), 
            Some(0)
        ))),
        Some(Combo::new(ComboConfig::new( // O+E -> Ö
            [k!(O), k!(E)], 
            k!(Semicolon),
            Some(0)
        ))),
        Some(Combo::new(ComboConfig::new( // U+E -> Ü
            [k!(U), k!(E)], 
            k!(LeftBracket), 
            Some(0)
        ))),
        Some(Combo::new(ComboConfig::new( // S+E -> ß
            [lt!(5, S), k!(E)], 
            k!(Minus), 
            Some(0)
        ))),
        Some(Combo::new(ComboConfig::new( // Mac - A+E -> Ä
            [mt!(A, ModifierCombination::LSHIFT), k!(E)], 
            k!(Quote), 
            Some(8)
        ))),
        Some(Combo::new(ComboConfig::new( // Mac - O+E -> Ö
            [k!(O), k!(E)], 
            k!(Semicolon), 
            Some(8)
        ))),
        Some(Combo::new(ComboConfig::new( // Mac - U+E -> Ü
            [k!(U), k!(E)], 
            k!(LeftBracket), 
            Some(8)
        ))),
        Some(Combo::new(ComboConfig::new( // Mac - S+E -> ß
            [lt!(13, S), k!(E)], 
            k!(Minus), 
            Some(8)
        )))
    ]
}