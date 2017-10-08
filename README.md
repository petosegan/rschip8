# rschip8
This is a CHIP-8 emulator written in Rust.

## TODO
* *asynchronous input to simulate hardware interrupts for keypresses*
  This is required for control of games to function correctly!
* use getopts to choose game and control debugging options
* add logging
* add sound to emulate sound timer
* key mappings
* better control over terminal
  * aspect ratio of grid
  * remove horizontal gutters
  * redraw without scrolling?
* refactor so that you can switch IO methods easily
* implement a piston or SDL2 display/keypad