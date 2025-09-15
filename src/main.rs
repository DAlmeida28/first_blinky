#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    #[rustfmt::skip]

    let h_led = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
    ];

    let e_led = [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 0, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
    ];

    let l_led = [
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
    ];

    let o_led = [
        [0, 0, 1, 0, 0],
        [0, 1, 0, 1, 0],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    // let mut last_led = (0, 0);

    loop {
        display.show(&mut timer, h_led, 500);
        display.show(&mut timer, e_led, 200);
        display.show(&mut timer, l_led, 200);
        display.show(&mut timer, l_led, 200);
        display.show(&mut timer, o_led, 200);
        nop();
    }
}
