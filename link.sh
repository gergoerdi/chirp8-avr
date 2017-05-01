BASE=target/avr-atmega328p/release
CRATE=hello-avr
OBJS="$BASE/deps/*.o $BASE/deps/*.rlib"

avr-gcc -Os -Wl,--gc-sections -mmcu=atmega328p -o $BASE/image.elf $OBJS
avr-objcopy -Oihex -R.eeprom $BASE/image.elf $BASE/image.hex
