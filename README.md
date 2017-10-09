# rschip8
This is a CHIP-8 emulator written in Rust.

Unlike most of the Rust CHIP-8 emulators out there, this one displays directly in the terminal, using [Termion](https://github.com/ticki/termion).

## USAGE

```
cargo run -- -g games/BRIX
```

The arrow keys work for most games. The CHIP-8 has a hex keypad, with 2, 4, 6, and 8 typically used for directions. I've mapped the other keys to 'qwerty' and 'asdfgh'.

To quit the emulator, press 'x'.

## TODO
* add sound to emulate sound timer
* key mappings
* better control over terminal
  * aspect ratio of grid
  * remove horizontal gutters
* implement a piston or SDL2 display/keypad

## Useful Resources
* http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* http://www.pong-story.com/chip8/