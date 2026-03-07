#![no_main]
#![no_std]

#[macro_use]
mod keymap;
mod metadata;
#[macro_use]
mod macros;

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Input;
use embassy_rp::peripherals::{PIO0, USB};
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_time::Duration;
use keymap::{COL_PER_PART, ROW, SIZE};
use rmk::config::{BehaviorConfig, CombosConfig, DeviceConfig, ForksConfig, PositionalConfig, RmkConfig};
use rmk::debounce::fast_debouncer::FastDebouncer;
use rmk::direct_pin::DirectPinMatrix;
use rmk::futures::future::join4;
use rmk::input_device::Runnable;
use rmk::keyboard::Keyboard;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::central::run_peripheral_manager;
use rmk::split::rp::uart::{BufferedUart, UartInterruptHandler};
use rmk::types::action::{MorseMode, MorseProfile};
use rmk::{initialize_keymap, run_all, run_rmk};
use static_cell::StaticCell;
use crate::keymap::{COL, get_combos, get_forks};

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    PIO0_IRQ_0 => UartInterruptHandler<PIO0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("RMK start!");
    // Initialize peripherals
    let p = embassy_rp::init(Default::default());

    // Create the usb driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Pin config
    #[rustfmt::skip]
    let direct_pins = config_matrix_pins_rp! {
        peripherals: p,
        direct_pins: [
            [PIN_7, PIN_26, PIN_27, PIN_28, PIN_29],
            [PIN_22, PIN_20, PIN_23, PIN_21, PIN_0],
            [PIN_2, PIN_3, PIN_4, PIN_5, PIN_6],
            [_, _, _, PIN_8, PIN_9],
        ]
    };

    let keyboard_device_config = DeviceConfig {
        vid: metadata::VENDOR_ID,
        pid: metadata::PRODUCT_ID,
        manufacturer: metadata::MANUFACTURER,
        product_name: metadata::PRODUCT_NAME,
        serial_number: "vial:f64c2b3c:465301",
    };

    let rmk_config = RmkConfig {
        device_config: keyboard_device_config,
        ..Default::default()
    };

    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_receiver = BufferedUart::new_half_duplex(p.PIO0, p.PIN_1, rx_buf, Irqs);

    // Initialize the storage and keymap
    let mut default_keymap = keymap::get_default_keymap();
    let mut behavior_config = BehaviorConfig::default();

    behavior_config.morse.enable_flow_tap = true;
    behavior_config.morse.prior_idle_time = Duration::from_millis(50);
    behavior_config.morse.default_profile = MorseProfile::const_default().
        with_mode(Some(MorseMode::PermissiveHold))
        .with_hold_timeout_ms(Some(200));

    behavior_config.fork = ForksConfig {
        forks: get_forks()
    };
    behavior_config.combo = CombosConfig {
        combos: get_combos(),
        timeout: Duration::from_millis(100)
    };

    let mut per_key_config = PositionalConfig::default();
    let keymap = initialize_keymap(
        &mut default_keymap,
        &mut behavior_config,
        &mut per_key_config,
    )
    .await;

    // Initialize the matrix + keyboard
    let debouncer = FastDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL_PER_PART, SIZE>::new(direct_pins, debouncer, true);
    let mut keyboard = Keyboard::new(&keymap);

    // Start
    join4(
        run_all!(matrix),
        keyboard.run(),
        run_peripheral_manager::<ROW, COL, 0, COL_PER_PART, _>(0, uart_receiver),
        run_rmk(driver, rmk_config),
    )
    .await;
}
