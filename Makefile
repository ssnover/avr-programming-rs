blink_led:
	cargo build -Z build-std=core --target avr-atmega328p.json --release --bin blink_led

pov_toy:
	cargo build -Z build-std=core --target avr-atmega328p.json --release --bin pov_toy

