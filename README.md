# mb2-embassy-blinky: modified embassy blinky example for mb2
Bart Massey 2024

This crate is a light rehacking of the Embassy "basic"
example to work on the MicroBit v2. It blinks an LED on the
MB2 with a one-second cycle time.

This currently depends on the version of Embassy on their
Git. Will fix as needed.

`defmt` currently runs into linker issues and is thus
disabled. Will put it back when this can be fixed.
