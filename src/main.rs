#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Duration;
use microbit_bsp::{
    display::{Brightness, Frame},
    LedMatrix, Microbit,
};
use panic_probe as _;

struct Mb2Blinker {
    display: LedMatrix,
    state: bool,
}

impl Mb2Blinker {
    fn new(mut display: LedMatrix) -> Self {
        display.clear();
        display.set_brightness(Brightness::MAX);
        Self{ display, state: false }
    }

    async fn step(&mut self) {
        let mut frame = Frame::default();
        
        if self.state {
            frame.set(0, 0);
        }
        
        self.display.display(frame, Duration::from_millis(500)).await;
        self.state = !self.state;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let board = Microbit::default();
    let display = board.display;

    let mut mb2_blinker = Mb2Blinker::new(display);
    
    println!("开始 LED 闪烁");
    loop {
        mb2_blinker.step().await;
    }
}