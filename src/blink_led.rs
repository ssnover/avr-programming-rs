#![no_std]
#![no_main]

use avr_programming_rs::util;
use ruduino::cores::atmega328p as avr_core;
use ruduino::Register;

use avr_core::{DDRB, PORTB};

#[no_mangle]
pub extern "C" fn main() {
    // Data Direction Register B:
    // Writing a one to the bit enables output.
    DDRB::set_mask_raw(0x01u8);

    loop {
        PORTB::set_mask_raw(0x01u8); // Turn on the first LED bit/pin in PORTB
        util::delay_ms(1000); // Wait

        PORTB::unset_mask_raw(0x01u8); // Turn off first bit in PORTB
        util::delay_ms(1000); // Wait
    }
}
