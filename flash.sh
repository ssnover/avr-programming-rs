PORT=$1
TARGET_ELF=$2

avrdude -patmega328p -carduino -P$PORT -b115200 -D -Uflash:w:$TARGET_ELF:e
