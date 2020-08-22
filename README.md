# avr-programming-rs

This is a small project to replicate the code projects in Make: AVR Programming by Elliot Williams in Rust now that avr-rust is on the nightly toolchain! I have Arduino UNO's handy so this repo assumes you're using that.

In order to build the code, you'll need the dependencies as mentioned in https://github.com/avr-rust/blink

Then you can call:

```
make blink_led
```

To flash:

```
# ./flash.sh <PORT> <ELF_FILE>
./flash.sh /dev/ttyACM0 target/avr-atmega328p/release/blink_led.elf 
```