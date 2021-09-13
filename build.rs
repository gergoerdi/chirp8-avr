fn main() {
    cc::Build::new()
        .compiler("avr-gcc").pic(false)
        .flag("-mmcu=atmega328p")
        .flag("-fno-exceptions").flag("-ffunction-sections").flag("-fdata-sections")
        .file("src/rom.c")
        .compile("rom");
}
