#![feature(llvm_asm)]
#![no_std]

pub mod util {
    pub fn delay_ms(count: u16) {
        for _ in 0..2280 {
            for _ in 0..count {
                unsafe { llvm_asm!("" :::: "volatile") }
            }
        }
    }
}
