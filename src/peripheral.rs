#![no_main]
#![no_std]

#[macro_use]
mod keymap;
#[macro_use]
mod macros;

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::Input;
use embassy_rp::peripherals::{PIO0, USB};
use embassy_rp::usb::InterruptHandler;
use keymap::{COL_PER_PART, ROW, SIZE};
use rmk::debounce::fast_debouncer::FastDebouncer;
use rmk::direct_pin::DirectPinMatrix;
use rmk::futures::future::join;
use rmk::run_all;
use rmk::split::SPLIT_MESSAGE_MAX_SIZE;
use rmk::split::peripheral::run_rmk_split_peripheral;
use rmk::split::rp::uart::{BufferedUart, UartInterruptHandler};
use static_cell::StaticCell;
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

    // Pin config
    #[rustfmt::skip]
    let direct_pins = config_matrix_pins_rp! {
        peripherals: p,
        direct_pins: [
            [PIN_29, PIN_28, PIN_27, PIN_26, PIN_7],
            [PIN_0, PIN_21, PIN_23, PIN_20, PIN_22],
            [PIN_6, PIN_5, PIN_4, PIN_3, PIN_2],
            [PIN_9, PIN_8, _, _, _],
        ]
    };

    static RX_BUF: StaticCell<[u8; SPLIT_MESSAGE_MAX_SIZE]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; SPLIT_MESSAGE_MAX_SIZE])[..];
    let uart_instance = BufferedUart::new_half_duplex(p.PIO0, p.PIN_1, rx_buf, Irqs);

    // Define the matrix
    let debouncer = FastDebouncer::new();
    let mut matrix = DirectPinMatrix::<_, _, ROW, COL_PER_PART, SIZE>::new(direct_pins, debouncer, true);

    // Start
    join(run_all!(matrix), run_rmk_split_peripheral(uart_instance)).await;
}
