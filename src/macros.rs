macro_rules! config_matrix_pins_rp {
    (peripherals: $p:ident, direct_pins: [$([$($pin:tt),+ $(,)?]),+ $(,)?]) => {
        {
            #[allow(unused_mut)]
            let mut pins = [
                $(
                    [
                        $(
                            config_matrix_pin_rp!(@pin $p, $pin)
                        ),+
                    ]
                ),+
            ];
            pins
        }
    };
}

macro_rules! config_matrix_pin_rp {
    (@pin $p:ident, _) => {
        None
    };

    (@pin $p:ident, $pin:ident) => {
        Some(Input::new($p.$pin, embassy_rp::gpio::Pull::Up))
    };
}

// action modifier tap
#[macro_export]
macro_rules! amt {
    ($a: expr, $m: expr) => {
        rmk::types::action::KeyAction::TapHold(
            $a,
            rmk::types::action::Action::Modifier($m),
            rmk::types::action::MorseProfile::const_default(),
        )
    };
}

// key code
#[macro_export]
macro_rules! kc {
    ($k: ident) => {
        rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::$k)
    };
}

// key with modifier
#[macro_export]
macro_rules! kwm {
    ($k: ident, $m: expr) => {
        Action::KeyWithModifier(
            rmk::types::keycode::KeyCode::Hid(rmk::types::keycode::HidKeyCode::$k), 
            $m
        )
    };
}