#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[rustfmt::skip]
const PIXELS: [(usize, usize); 25] = [
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 0),
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
    (2, 4),
    (3, 0),
    (3, 1),
    (3, 2),
    (3, 3),
    (3, 4),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4)
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    #[rustfmt::skip]
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

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
        for current_led in PIXELS.iter() {
            //            leds[last_led.0][last_led.1] = 0;
            //            leds[current_led.0][current_led.1] = 1;
            display.show(&mut timer, h_led, 300);
            display.show(&mut timer, e_led, 200);
            display.show(&mut timer, l_led, 200);
            display.show(&mut timer, l_led, 200);
            display.show(&mut timer, o_led, 200);
            // last_led = *current_led;
        }
        nop();
    }
}
