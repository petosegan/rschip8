# rschip8
This is a CHIP-8 emulator written in Rust.

Two frontends are available: SDL2 (default), or terminal display using [Termion](https://github.com/ticki/termion).

## USAGE

```
cargo run -- -g path_to_game [-c clock_speed_in_hz]
```

The arrow keys work for most games. The CHIP-8 has a hex keypad, with 2, 4, 6, and 8 typically used for directions. I've mapped the other keys to 'qwerty' and 'asdfgh'. The default clock speed is 500 Hz.

To quit the emulator, press 'x'.

To display in the terminal, use the '-t' flag.

## Useful Resources
* http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* http://www.pong-story.com/chip8/
