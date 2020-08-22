#![no_std]
#![no_main]

use avr_programming_rs::util;
use ruduino::cores::atmega328p as avr_core;
use ruduino::Register;

use avr_core::{DDRD, PORTD};

#[no_mangle]
pub extern "C" fn main() {
    DDRD::set_mask_raw(0xff);

    loop {
        pov_display(0b0000_1110);
        pov_display(0b0001_1000);
        pov_display(0b1011_1101);
        pov_display(0b0111_0110);
        pov_display(0b0011_1100);
        pov_display(0b0011_1100);
        pov_display(0b0011_1100);
        pov_display(0b0111_0110);
        pov_display(0b1011_1101);
        pov_display(0b0001_1000);
        pov_display(0b0000_1110);

        PORTD::unset_mask_raw(!0x00);
        util::delay_ms(10);
    }
}

fn pov_display(one_byte: u8) {
    PORTD::write(one_byte);
    util::delay_ms(2);
}
