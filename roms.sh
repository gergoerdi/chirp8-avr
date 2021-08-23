DEST="target/avr-unknown-gnu-atmega328/release/deps"
mkdir -p "$DEST"

avr-gcc -O3 -mmcu=atmega328p -fno-exceptions -ffunction-sections -fdata-sections -c src/rom.c -o "$DEST/rom.o"
