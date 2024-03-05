# mb2-embassy-rgb: blinky + RGB example for mb2
Bart Massey 2024

This crate demos Embassy on the MicroBit v2. It blinks an
LED on the MB2 with a one-second cycle time. It also blinks
an external RGB LED, cc

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd
