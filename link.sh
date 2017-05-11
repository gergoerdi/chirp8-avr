BASE=target/avr-atmega328p/release
CRATE=hello-avr
OBJS="$BASE/deps/*.o $BASE/deps/*.rlib"
LDSCRIPT=lookup-text.ld

avr-gcc -Os -Wl,--gc-sections -mmcu=atmega328p -T $LDSCRIPT -o $BASE/image.elf $OBJS
avr-strip $BASE/image.elf
avr-objcopy -Oihex -R.eeprom $BASE/image.elf $BASE/image.hex
